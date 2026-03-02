use crate::models::*;
use crate::repository::{SqliteTransferRepository, TransferRepository};
use anyhow::Result;
use chrono::Utc;
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use tracing::info;
use uuid::Uuid;

pub struct TransferService<R: TransferRepository> {
    repo: R,
}

impl TransferService<SqliteTransferRepository> {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            repo: SqliteTransferRepository::new(pool),
        }
    }
}

impl<R: TransferRepository> TransferService<R> {
    pub fn with_repo(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create(&self, req: CreateTransferRequest, created_by: Option<Uuid>) -> Result<StockTransferWithLines> {
        if req.from_warehouse_id == req.to_warehouse_id {
            anyhow::bail!("Source and destination warehouses must be different");
        }

        if req.lines.is_empty() {
            anyhow::bail!("At least one transfer line is required");
        }

        let transfer_number = self.repo.get_next_number().await?;
        let now = Utc::now();
        let transfer_id = Uuid::new_v4();

        let mut lines = Vec::new();
        for line_req in &req.lines {
            if line_req.requested_quantity <= 0 {
                anyhow::bail!("Requested quantity must be positive");
            }
            
            let line = StockTransferLine {
                id: Uuid::new_v4(),
                transfer_id,
                product_id: line_req.product_id,
                requested_quantity: line_req.requested_quantity,
                shipped_quantity: 0,
                received_quantity: 0,
                unit_cost: line_req.unit_cost,
                notes: line_req.notes.clone(),
                created_at: now,
            };
            lines.push(line);
        }

        let transfer = StockTransfer {
            base: BaseEntity {
                id: transfer_id,
                created_at: now,
                updated_at: now,
                created_by,
                updated_by: None,
            },
            transfer_number,
            from_warehouse_id: req.from_warehouse_id,
            to_warehouse_id: req.to_warehouse_id,
            status: TransferStatus::Draft,
            priority: req.priority,
            requested_date: req.requested_date,
            expected_date: req.expected_date,
            shipped_date: None,
            received_date: None,
            approved_by: None,
            approved_at: None,
            shipped_by: None,
            received_by: None,
            notes: req.notes,
        };

        let transfer = self.repo.create(&transfer).await?;
        
        for line in &lines {
            self.repo.add_line(line).await?;
        }

        info!("Created stock transfer {} with {} lines", transfer.transfer_number, lines.len());
        Ok(StockTransferWithLines { transfer, lines })
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<StockTransferWithLines>> {
        let transfer = match self.repo.get(id).await? {
            Some(t) => t,
            None => return Ok(None),
        };
        let lines = self.repo.get_lines(id).await?;
        Ok(Some(StockTransferWithLines { transfer, lines }))
    }

    pub async fn list(&self, from_warehouse_id: Option<Uuid>, to_warehouse_id: Option<Uuid>, status: Option<TransferStatus>) -> Result<Vec<StockTransfer>> {
        self.repo.list(from_warehouse_id, to_warehouse_id, status).await
    }

    pub async fn submit(&self, id: Uuid) -> Result<StockTransfer> {
        let mut transfer = self.repo.get(id).await?
            .ok_or_else(|| anyhow::anyhow!("Transfer not found"))?;

        if transfer.status != TransferStatus::Draft {
            anyhow::bail!("Only draft transfers can be submitted");
        }

        transfer.status = TransferStatus::Pending;
        transfer.base.updated_at = Utc::now();
        self.repo.update(&transfer).await
    }

    pub async fn approve(&self, id: Uuid, approved_by: Uuid) -> Result<StockTransfer> {
        let mut transfer = self.repo.get(id).await?
            .ok_or_else(|| anyhow::anyhow!("Transfer not found"))?;

        if transfer.status != TransferStatus::Pending {
            anyhow::bail!("Only pending transfers can be approved");
        }

        transfer.status = TransferStatus::Approved;
        transfer.approved_by = Some(approved_by);
        transfer.approved_at = Some(Utc::now());
        transfer.base.updated_at = Utc::now();
        
        info!("Approved stock transfer {}", transfer.transfer_number);
        self.repo.update(&transfer).await
    }

    pub async fn reject(&self, id: Uuid, reason: String) -> Result<StockTransfer> {
        let mut transfer = self.repo.get(id).await?
            .ok_or_else(|| anyhow::anyhow!("Transfer not found"))?;

        if transfer.status != TransferStatus::Pending {
            anyhow::bail!("Only pending transfers can be rejected");
        }

        transfer.status = TransferStatus::Cancelled;
        transfer.notes = Some(reason);
        transfer.base.updated_at = Utc::now();
        
        info!("Rejected stock transfer {}", transfer.transfer_number);
        self.repo.update(&transfer).await
    }

    pub async fn ship(&self, id: Uuid, shipped_by: Uuid, req: ShipTransferRequest) -> Result<StockTransfer> {
        let mut transfer = self.repo.get(id).await?
            .ok_or_else(|| anyhow::anyhow!("Transfer not found"))?;

        if transfer.status != TransferStatus::Approved {
            anyhow::bail!("Only approved transfers can be shipped");
        }

        let mut lines = self.repo.get_lines(id).await?;
        for ship_line in &req.lines {
            if let Some(line) = lines.iter_mut().find(|l| l.product_id == ship_line.product_id) {
                if ship_line.shipped_quantity > line.requested_quantity {
                    anyhow::bail!("Shipped quantity cannot exceed requested quantity");
                }
                line.shipped_quantity = ship_line.shipped_quantity;
                self.repo.update_line(line).await?;
            }
        }

        transfer.status = TransferStatus::InTransit;
        transfer.shipped_date = Some(Utc::now());
        transfer.shipped_by = Some(shipped_by);
        transfer.base.updated_at = Utc::now();
        
        info!("Shipped stock transfer {}", transfer.transfer_number);
        self.repo.update(&transfer).await
    }

    pub async fn receive(&self, id: Uuid, received_by: Uuid, req: ReceiveTransferRequest) -> Result<StockTransfer> {
        let mut transfer = self.repo.get(id).await?
            .ok_or_else(|| anyhow::anyhow!("Transfer not found"))?;

        if transfer.status != TransferStatus::InTransit {
            anyhow::bail!("Only in-transit transfers can be received");
        }

        let mut lines = self.repo.get_lines(id).await?;
        let mut all_received = true;
        
        for recv_line in &req.lines {
            if let Some(line) = lines.iter_mut().find(|l| l.product_id == recv_line.product_id) {
                if recv_line.received_quantity > line.shipped_quantity {
                    anyhow::bail!("Received quantity cannot exceed shipped quantity");
                }
                line.received_quantity = recv_line.received_quantity;
                self.repo.update_line(line).await?;
                
                if line.received_quantity < line.shipped_quantity {
                    all_received = false;
                }
            }
        }

        transfer.received_date = Some(Utc::now());
        transfer.received_by = Some(received_by);
        transfer.status = if all_received {
            TransferStatus::Received
        } else {
            TransferStatus::PartiallyReceived
        };
        transfer.base.updated_at = Utc::now();
        
        info!("Received stock transfer {} with status {:?}", transfer.transfer_number, transfer.status);
        self.repo.update(&transfer).await
    }

    pub async fn cancel(&self, id: Uuid) -> Result<StockTransfer> {
        let mut transfer = self.repo.get(id).await?
            .ok_or_else(|| anyhow::anyhow!("Transfer not found"))?;

        if matches!(transfer.status, TransferStatus::Received | TransferStatus::PartiallyReceived | TransferStatus::InTransit) {
            anyhow::bail!("Cannot cancel transfers that are in transit or received");
        }

        transfer.status = TransferStatus::Cancelled;
        transfer.base.updated_at = Utc::now();
        
        info!("Cancelled stock transfer {}", transfer.transfer_number);
        self.repo.update(&transfer).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        let transfer = self.repo.get(id).await?
            .ok_or_else(|| anyhow::anyhow!("Transfer not found"))?;

        if transfer.status != TransferStatus::Draft {
            anyhow::bail!("Only draft transfers can be deleted");
        }

        self.repo.delete_lines(id).await?;
        self.repo.delete(id).await?;
        
        info!("Deleted stock transfer {}", transfer.transfer_number);
        Ok(())
    }

    pub async fn add_line(&self, transfer_id: Uuid, req: CreateTransferLineRequest) -> Result<StockTransferLine> {
        let transfer = self.repo.get(transfer_id).await?
            .ok_or_else(|| anyhow::anyhow!("Transfer not found"))?;

        if transfer.status != TransferStatus::Draft {
            anyhow::bail!("Can only add lines to draft transfers");
        }

        if req.requested_quantity <= 0 {
            anyhow::bail!("Requested quantity must be positive");
        }

        let line = StockTransferLine {
            id: Uuid::new_v4(),
            transfer_id,
            product_id: req.product_id,
            requested_quantity: req.requested_quantity,
            shipped_quantity: 0,
            received_quantity: 0,
            unit_cost: req.unit_cost,
            notes: req.notes,
            created_at: Utc::now(),
        };

        let line = self.repo.add_line(&line).await?;
        info!("Added line to stock transfer {}", transfer.transfer_number);
        Ok(line)
    }

    pub async fn get_lines(&self, transfer_id: Uuid) -> Result<Vec<StockTransferLine>> {
        self.repo.get_lines(transfer_id).await
    }

    pub async fn get_analytics(&self) -> Result<TransferAnalytics> {
        let transfers = self.repo.list(None, None, None).await?;

        let total_transfers = transfers.len() as i64;
        let pending_transfers = transfers.iter().filter(|t| matches!(t.status, TransferStatus::Pending | TransferStatus::Draft)).count() as i64;
        let in_transit = transfers.iter().filter(|t| t.status == TransferStatus::InTransit).count() as i64;
        let completed_transfers = transfers.iter().filter(|t| matches!(t.status, TransferStatus::Received | TransferStatus::PartiallyReceived)).count() as i64;

        let mut by_status: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        for t in &transfers {
            let key = format!("{:?}", t.status);
            *by_status.entry(key).or_insert(0) += 1;
        }

        let mut total_time_hours = 0.0;
        let mut completed_count = 0;
        for t in &transfers {
            if let (Some(shipped), Some(received)) = (t.shipped_date, t.received_date) {
                let duration = received.signed_duration_since(shipped);
                total_time_hours += duration.num_minutes() as f64 / 60.0;
                completed_count += 1;
            }
        }
        let average_transfer_time_hours = if completed_count > 0 {
            total_time_hours / completed_count as f64
        } else {
            0.0
        };

        Ok(TransferAnalytics {
            total_transfers,
            pending_transfers,
            in_transit,
            completed_transfers,
            total_value_in_transit: 0,
            transfers_by_status: serde_json::to_value(by_status).unwrap_or(serde_json::json!({})),
            average_transfer_time_hours,
        })
    }
}
