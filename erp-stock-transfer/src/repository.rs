use crate::models::*;
use async_trait::async_trait;
use chrono::Utc;
use erp_core::BaseEntity;
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

#[async_trait]
pub trait TransferRepository: Send + Sync {
    async fn create(&self, transfer: &StockTransfer) -> anyhow::Result<StockTransfer>;
    async fn get(&self, id: Uuid) -> anyhow::Result<Option<StockTransfer>>;
    async fn list(&self, from_warehouse_id: Option<Uuid>, to_warehouse_id: Option<Uuid>, status: Option<TransferStatus>) -> anyhow::Result<Vec<StockTransfer>>;
    async fn update(&self, transfer: &StockTransfer) -> anyhow::Result<StockTransfer>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn add_line(&self, line: &StockTransferLine) -> anyhow::Result<StockTransferLine>;
    async fn get_lines(&self, transfer_id: Uuid) -> anyhow::Result<Vec<StockTransferLine>>;
    async fn update_line(&self, line: &StockTransferLine) -> anyhow::Result<StockTransferLine>;
    async fn delete_lines(&self, transfer_id: Uuid) -> anyhow::Result<()>;
    async fn get_next_number(&self) -> anyhow::Result<String>;
}

pub struct SqliteTransferRepository {
    pool: SqlitePool,
}

impl SqliteTransferRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
#[async_trait]
impl TransferRepository for SqliteTransferRepository {
    async fn create(&self, transfer: &StockTransfer) -> anyhow::Result<StockTransfer> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            r#"INSERT INTO stock_transfers 
               (id, transfer_number, from_warehouse_id, to_warehouse_id, status, priority,
                requested_date, expected_date, shipped_date, received_date, approved_by, 
                approved_at, shipped_by, received_by, notes, created_at, updated_at, created_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(transfer.base.id.to_string())
        .bind(&transfer.transfer_number)
        .bind(transfer.from_warehouse_id.to_string())
        .bind(transfer.to_warehouse_id.to_string())
        .bind(format!("{:?}", transfer.status))
        .bind(format!("{:?}", transfer.priority))
        .bind(transfer.requested_date.map(|d| d.to_rfc3339()))
        .bind(transfer.expected_date.map(|d| d.to_rfc3339()))
        .bind(transfer.shipped_date.map(|d| d.to_rfc3339()))
        .bind(transfer.received_date.map(|d| d.to_rfc3339()))
        .bind(transfer.approved_by.map(|id| id.to_string()))
        .bind(transfer.approved_at.map(|d| d.to_rfc3339()))
        .bind(transfer.shipped_by.map(|id| id.to_string()))
        .bind(transfer.received_by.map(|id| id.to_string()))
        .bind(&transfer.notes)
        .bind(&now)
        .bind(&now)
        .bind(transfer.base.created_by.map(|id| id.to_string()))
        .execute(&self.pool)
        .await?;
        Ok(transfer.clone())
    }
    async fn get(&self, id: Uuid) -> anyhow::Result<Option<StockTransfer>> {
        let row = sqlx::query(
            "SELECT id, transfer_number, from_warehouse_id, to_warehouse_id, status, priority,
                    requested_date, expected_date, shipped_date, received_date, approved_by,
                    approved_at, shipped_by, received_by, notes, created_at, updated_at, created_by 
             FROM stock_transfers WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.as_ref().map(row_to_transfer))
    }
    async fn list(&self, from_warehouse_id: Option<Uuid>, to_warehouse_id: Option<Uuid>, status: Option<TransferStatus>) -> anyhow::Result<Vec<StockTransfer>> {
        let mut query = "SELECT id, transfer_number, from_warehouse_id, to_warehouse_id, status, priority,
                        requested_date, expected_date, shipped_date, received_date, approved_by,
                        approved_at, shipped_by, received_by, notes, created_at, updated_at, created_by 
                        FROM stock_transfers WHERE 1=1".to_string();
        let mut binds: Vec<String> = Vec::new();
        if let Some(wh_id) = from_warehouse_id {
            query.push_str(" AND from_warehouse_id = ?");
            binds.push(wh_id.to_string());
        }
        if let Some(wh_id) = to_warehouse_id {
            query.push_str(" AND to_warehouse_id = ?");
            binds.push(wh_id.to_string());
        }
        if let Some(s) = status {
            query.push_str(" AND status = ?");
            binds.push(format!("{:?}", s));
        }
        query.push_str(" ORDER BY created_at DESC");
        let mut sql_query = sqlx::query(&query);
        for bind in binds {
            sql_query = sql_query.bind(bind);
        }
        let rows = sql_query.fetch_all(&self.pool).await?;
        Ok(rows.iter().map(row_to_transfer).collect())
    }
    async fn update(&self, transfer: &StockTransfer) -> anyhow::Result<StockTransfer> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE stock_transfers SET from_warehouse_id=?, to_warehouse_id=?, status=?, priority=?,
             requested_date=?, expected_date=?, shipped_date=?, received_date=?, approved_by=?,
             approved_at=?, shipped_by=?, received_by=?, notes=?, updated_at=? WHERE id=?"
        )
        .bind(transfer.from_warehouse_id.to_string())
        .bind(transfer.to_warehouse_id.to_string())
        .bind(format!("{:?}", transfer.status))
        .bind(format!("{:?}", transfer.priority))
        .bind(transfer.requested_date.map(|d| d.to_rfc3339()))
        .bind(transfer.expected_date.map(|d| d.to_rfc3339()))
        .bind(transfer.shipped_date.map(|d| d.to_rfc3339()))
        .bind(transfer.received_date.map(|d| d.to_rfc3339()))
        .bind(transfer.approved_by.map(|id| id.to_string()))
        .bind(transfer.approved_at.map(|d| d.to_rfc3339()))
        .bind(transfer.shipped_by.map(|id| id.to_string()))
        .bind(transfer.received_by.map(|id| id.to_string()))
        .bind(&transfer.notes)
        .bind(&now)
        .bind(transfer.base.id.to_string())
        .execute(&self.pool)
        .await?;
        Ok(transfer.clone())
    }
    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM stock_transfers WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn add_line(&self, line: &StockTransferLine) -> anyhow::Result<StockTransferLine> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            r#"INSERT INTO stock_transfer_lines 
               (id, transfer_id, product_id, requested_quantity, shipped_quantity, 
                received_quantity, unit_cost, notes, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(line.id.to_string())
        .bind(line.transfer_id.to_string())
        .bind(line.product_id.to_string())
        .bind(line.requested_quantity)
        .bind(line.shipped_quantity)
        .bind(line.received_quantity)
        .bind(line.unit_cost)
        .bind(&line.notes)
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(line.clone())
    }
    async fn get_lines(&self, transfer_id: Uuid) -> anyhow::Result<Vec<StockTransferLine>> {
        let rows = sqlx::query(
            "SELECT id, transfer_id, product_id, requested_quantity, shipped_quantity, 
                    received_quantity, unit_cost, notes, created_at 
             FROM stock_transfer_lines WHERE transfer_id = ?"
        )
        .bind(transfer_id.to_string())
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.iter().map(row_to_line).collect())
    }
    async fn update_line(&self, line: &StockTransferLine) -> anyhow::Result<StockTransferLine> {
        sqlx::query(
            "UPDATE stock_transfer_lines SET requested_quantity=?, shipped_quantity=?, 
             received_quantity=?, notes=? WHERE id=?"
        )
        .bind(line.requested_quantity)
        .bind(line.shipped_quantity)
        .bind(line.received_quantity)
        .bind(&line.notes)
        .bind(line.id.to_string())
        .execute(&self.pool)
        .await?;
        Ok(line.clone())
    }
    async fn delete_lines(&self, transfer_id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM stock_transfer_lines WHERE transfer_id = ?")
            .bind(transfer_id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }
    async fn get_next_number(&self) -> anyhow::Result<String> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM stock_transfers")
            .fetch_one(&self.pool)
            .await?;
        Ok(format!("ST-{:06}", count.0 + 1))
    }
}

fn row_to_transfer(row: &sqlx::sqlite::SqliteRow) -> StockTransfer {
    let id: String = row.get(0);
    let transfer_number: String = row.get(1);
    let from_warehouse_id: String = row.get(2);
    let to_warehouse_id: String = row.get(3);
    let status: String = row.get(4);
    let priority: String = row.get(5);
    let requested_date: Option<String> = row.get(6);
    let expected_date: Option<String> = row.get(7);
    let shipped_date: Option<String> = row.get(8);
    let received_date: Option<String> = row.get(9);
    let approved_by: Option<String> = row.get(10);
    let approved_at: Option<String> = row.get(11);
    let shipped_by: Option<String> = row.get(12);
    let received_by: Option<String> = row.get(13);
    let notes: Option<String> = row.get(14);
    let created_at: String = row.get(15);
    let updated_at: String = row.get(16);
    let created_by: Option<String> = row.get(17);
    StockTransfer {
        base: BaseEntity {
            id: Uuid::parse_str(&id).unwrap_or_default(),
            created_at: chrono::DateTime::parse_from_rfc3339(&created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            created_by: created_by.and_then(|s| Uuid::parse_str(&s).ok()),
            updated_by: None,
        },
        transfer_number,
        from_warehouse_id: Uuid::parse_str(&from_warehouse_id).unwrap_or_default(),
        to_warehouse_id: Uuid::parse_str(&to_warehouse_id).unwrap_or_default(),
        status: parse_status(&status),
        priority: parse_priority(&priority),
        requested_date: requested_date.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok()).map(|d| d.with_timezone(&Utc)),
        expected_date: expected_date.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok()).map(|d| d.with_timezone(&Utc)),
        shipped_date: shipped_date.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok()).map(|d| d.with_timezone(&Utc)),
        received_date: received_date.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok()).map(|d| d.with_timezone(&Utc)),
        approved_by: approved_by.and_then(|s| Uuid::parse_str(&s).ok()),
        approved_at: approved_at.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok()).map(|d| d.with_timezone(&Utc)),
        shipped_by: shipped_by.and_then(|s| Uuid::parse_str(&s).ok()),
        received_by: received_by.and_then(|s| Uuid::parse_str(&s).ok()),
        notes,
    }
}

fn row_to_line(row: &sqlx::sqlite::SqliteRow) -> StockTransferLine {
    let id: String = row.get(0);
    let transfer_id: String = row.get(1);
    let product_id: String = row.get(2);
    StockTransferLine {
        id: Uuid::parse_str(&id).unwrap_or_default(),
        transfer_id: Uuid::parse_str(&transfer_id).unwrap_or_default(),
        product_id: Uuid::parse_str(&product_id).unwrap_or_default(),
        requested_quantity: row.get(3),
        shipped_quantity: row.get(4),
        received_quantity: row.get(5),
        unit_cost: row.get(6),
        notes: row.get(7),
        created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>(8)).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
    }
}

fn parse_status(s: &str) -> TransferStatus {
    match s {
        "Draft" => TransferStatus::Draft,
        "Pending" => TransferStatus::Pending,
        "Approved" => TransferStatus::Approved,
        "InTransit" => TransferStatus::InTransit,
        "Received" => TransferStatus::Received,
        "PartiallyReceived" => TransferStatus::PartiallyReceived,
        _ => TransferStatus::Cancelled,
    }
}

fn parse_priority(s: &str) -> TransferPriority {
    match s {
        "Low" => TransferPriority::Low,
        "High" => TransferPriority::High,
        "Urgent" => TransferPriority::Urgent,
        _ => TransferPriority::Normal,
    }
}
