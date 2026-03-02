use crate::models::*;
use crate::repository::{AdjustmentRepository, SqliteAdjustmentRepository};
use anyhow::Result;
use chrono::Utc;
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use tracing::info;
use uuid::Uuid;

pub struct AdjustmentService<R: AdjustmentRepository> {
    repo: R,
}

impl AdjustmentService<SqliteAdjustmentRepository> {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            repo: SqliteAdjustmentRepository::new(pool),
        }
    }
}

impl<R: AdjustmentRepository> AdjustmentService<R> {
    pub fn with_repo(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create(&self, req: CreateAdjustmentRequest, created_by: Option<Uuid>) -> Result<InventoryAdjustmentWithLines> {
        if req.lines.is_empty() {
            anyhow::bail!("At least one adjustment line is required");
        }

        let adjustment_number = self.repo.get_next_number().await?;
        let now = Utc::now();
        let adjustment_id = Uuid::new_v4();

        let mut total_value_change = 0i64;
        let mut lines = Vec::new();

        for line_req in &req.lines {
            let adjustment_quantity = line_req.counted_quantity - line_req.system_quantity;
            let total_value = adjustment_quantity * line_req.unit_cost;
            total_value_change += total_value;

            let line = InventoryAdjustmentLine {
                id: Uuid::new_v4(),
                adjustment_id,
                product_id: line_req.product_id,
                location_id: line_req.location_id,
                system_quantity: line_req.system_quantity,
                counted_quantity: line_req.counted_quantity,
                adjustment_quantity,
                unit_cost: line_req.unit_cost,
                total_value_change: total_value,
                lot_number: line_req.lot_number.clone(),
                serial_number: line_req.serial_number.clone(),
                reason_code: line_req.reason_code.clone(),
                notes: line_req.notes.clone(),
                created_at: now,
            };
            lines.push(line);
        }

        let adjustment = InventoryAdjustment {
            base: BaseEntity {
                id: adjustment_id,
                created_at: now,
                updated_at: now,
                created_by,
                updated_by: None,
            },
            adjustment_number,
            warehouse_id: req.warehouse_id,
            adjustment_type: req.adjustment_type,
            reason: req.reason,
            status: AdjustmentStatus::Draft,
            total_value_change,
            approved_by: None,
            approved_at: None,
            notes: req.notes,
        };

        let adjustment = self.repo.create(&adjustment).await?;
        
        for line in &lines {
            self.repo.add_line(line).await?;
        }

        info!("Created inventory adjustment {} with {} lines", adjustment.adjustment_number, lines.len());
        Ok(InventoryAdjustmentWithLines { adjustment, lines })
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<InventoryAdjustmentWithLines>> {
        let adjustment = match self.repo.get(id).await? {
            Some(a) => a,
            None => return Ok(None),
        };
        let lines = self.repo.get_lines(id).await?;
        Ok(Some(InventoryAdjustmentWithLines { adjustment, lines }))
    }

    pub async fn list(&self, warehouse_id: Option<Uuid>, status: Option<AdjustmentStatus>) -> Result<Vec<InventoryAdjustment>> {
        self.repo.list(warehouse_id, status).await
    }

    pub async fn submit(&self, id: Uuid) -> Result<InventoryAdjustment> {
        let mut adjustment = self.repo.get(id).await?
            .ok_or_else(|| anyhow::anyhow!("Adjustment not found"))?;

        if adjustment.status != AdjustmentStatus::Draft {
            anyhow::bail!("Only draft adjustments can be submitted");
        }

        adjustment.status = AdjustmentStatus::Pending;
        adjustment.base.updated_at = Utc::now();
        self.repo.update(&adjustment).await
    }

    pub async fn approve(&self, id: Uuid, approved_by: Uuid) -> Result<InventoryAdjustment> {
        let mut adjustment = self.repo.get(id).await?
            .ok_or_else(|| anyhow::anyhow!("Adjustment not found"))?;

        if adjustment.status != AdjustmentStatus::Pending {
            anyhow::bail!("Only pending adjustments can be approved");
        }

        adjustment.status = AdjustmentStatus::Approved;
        adjustment.approved_by = Some(approved_by);
        adjustment.approved_at = Some(Utc::now());
        adjustment.base.updated_at = Utc::now();
        
        info!("Approved inventory adjustment {}", adjustment.adjustment_number);
        self.repo.update(&adjustment).await
    }

    pub async fn reject(&self, id: Uuid, reason: String) -> Result<InventoryAdjustment> {
        let mut adjustment = self.repo.get(id).await?
            .ok_or_else(|| anyhow::anyhow!("Adjustment not found"))?;

        if adjustment.status != AdjustmentStatus::Pending {
            anyhow::bail!("Only pending adjustments can be rejected");
        }

        adjustment.status = AdjustmentStatus::Rejected;
        adjustment.notes = Some(reason);
        adjustment.base.updated_at = Utc::now();
        
        info!("Rejected inventory adjustment {}", adjustment.adjustment_number);
        self.repo.update(&adjustment).await
    }

    pub async fn complete(&self, id: Uuid) -> Result<InventoryAdjustment> {
        let mut adjustment = self.repo.get(id).await?
            .ok_or_else(|| anyhow::anyhow!("Adjustment not found"))?;

        if adjustment.status != AdjustmentStatus::Approved {
            anyhow::bail!("Only approved adjustments can be completed");
        }

        adjustment.status = AdjustmentStatus::Completed;
        adjustment.base.updated_at = Utc::now();
        
        info!("Completed inventory adjustment {}", adjustment.adjustment_number);
        self.repo.update(&adjustment).await
    }

    pub async fn cancel(&self, id: Uuid) -> Result<InventoryAdjustment> {
        let mut adjustment = self.repo.get(id).await?
            .ok_or_else(|| anyhow::anyhow!("Adjustment not found"))?;

        if adjustment.status == AdjustmentStatus::Completed {
            anyhow::bail!("Completed adjustments cannot be cancelled");
        }

        adjustment.status = AdjustmentStatus::Cancelled;
        adjustment.base.updated_at = Utc::now();
        
        info!("Cancelled inventory adjustment {}", adjustment.adjustment_number);
        self.repo.update(&adjustment).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<()> {
        let adjustment = self.repo.get(id).await?
            .ok_or_else(|| anyhow::anyhow!("Adjustment not found"))?;

        if adjustment.status != AdjustmentStatus::Draft {
            anyhow::bail!("Only draft adjustments can be deleted");
        }

        self.repo.delete_lines(id).await?;
        self.repo.delete(id).await?;
        
        info!("Deleted inventory adjustment {}", adjustment.adjustment_number);
        Ok(())
    }

    pub async fn add_line(&self, adjustment_id: Uuid, req: CreateAdjustmentLineRequest) -> Result<InventoryAdjustmentLine> {
        let adjustment = self.repo.get(adjustment_id).await?
            .ok_or_else(|| anyhow::anyhow!("Adjustment not found"))?;

        if adjustment.status != AdjustmentStatus::Draft {
            anyhow::bail!("Can only add lines to draft adjustments");
        }

        let adjustment_quantity = req.counted_quantity - req.system_quantity;
        let total_value = adjustment_quantity * req.unit_cost;

        let line = InventoryAdjustmentLine {
            id: Uuid::new_v4(),
            adjustment_id,
            product_id: req.product_id,
            location_id: req.location_id,
            system_quantity: req.system_quantity,
            counted_quantity: req.counted_quantity,
            adjustment_quantity,
            unit_cost: req.unit_cost,
            total_value_change: total_value,
            lot_number: req.lot_number,
            serial_number: req.serial_number,
            reason_code: req.reason_code,
            notes: req.notes,
            created_at: Utc::now(),
        };

        let line = self.repo.add_line(&line).await?;

        let mut adj = adjustment.clone();
        adj.total_value_change += total_value;
        adj.base.updated_at = Utc::now();
        self.repo.update(&adj).await?;

        Ok(line)
    }

    pub async fn get_lines(&self, adjustment_id: Uuid) -> Result<Vec<InventoryAdjustmentLine>> {
        self.repo.get_lines(adjustment_id).await
    }

    pub async fn get_analytics(&self) -> Result<AdjustmentAnalytics> {
        let adjustments = self.repo.list(None, None).await?;

        let total_adjustments = adjustments.len() as i64;
        let pending_adjustments = adjustments.iter().filter(|a| a.status == AdjustmentStatus::Pending).count() as i64;
        let completed_adjustments = adjustments.iter().filter(|a| a.status == AdjustmentStatus::Completed).count() as i64;
        let total_value_increase = adjustments.iter().filter(|a| a.total_value_change > 0).map(|a| a.total_value_change).sum();
        let total_value_decrease = adjustments.iter().filter(|a| a.total_value_change < 0).map(|a| a.total_value_change.abs()).sum();

        let mut by_type: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        for adj in &adjustments {
            let key = format!("{:?}", adj.adjustment_type);
            *by_type.entry(key).or_insert(0) += 1;
        }

        Ok(AdjustmentAnalytics {
            total_adjustments,
            pending_adjustments,
            completed_adjustments,
            total_value_increase,
            total_value_decrease,
            adjustments_by_type: serde_json::to_value(by_type).unwrap_or(serde_json::json!({})),
            adjustments_by_month: serde_json::json!({}),
        })
    }
}
