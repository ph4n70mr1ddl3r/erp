use crate::models::*;
use async_trait::async_trait;
use chrono::Utc;
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait AdjustmentRepository: Send + Sync {
    async fn create(&self, adjustment: &InventoryAdjustment) -> anyhow::Result<InventoryAdjustment>;
    async fn get(&self, id: Uuid) -> anyhow::Result<Option<InventoryAdjustment>>;
    async fn list(&self, warehouse_id: Option<Uuid>, status: Option<AdjustmentStatus>) -> anyhow::Result<Vec<InventoryAdjustment>>;
    async fn update(&self, adjustment: &InventoryAdjustment) -> anyhow::Result<InventoryAdjustment>;
    async fn delete(&self, id: Uuid) -> anyhow::Result<()>;
    async fn add_line(&self, line: &InventoryAdjustmentLine) -> anyhow::Result<InventoryAdjustmentLine>;
    async fn get_lines(&self, adjustment_id: Uuid) -> anyhow::Result<Vec<InventoryAdjustmentLine>>;
    async fn delete_lines(&self, adjustment_id: Uuid) -> anyhow::Result<()>;
    async fn get_next_number(&self) -> anyhow::Result<String>;
}

pub struct SqliteAdjustmentRepository {
    pool: SqlitePool,
}

impl SqliteAdjustmentRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl AdjustmentRepository for SqliteAdjustmentRepository {
    async fn create(&self, adjustment: &InventoryAdjustment) -> anyhow::Result<InventoryAdjustment> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            r#"INSERT INTO inventory_adjustments 
               (id, adjustment_number, warehouse_id, adjustment_type, reason, status, 
                total_value_change, approved_by, approved_at, notes, created_at, updated_at, created_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(adjustment.base.id.to_string())
        .bind(&adjustment.adjustment_number)
        .bind(adjustment.warehouse_id.to_string())
        .bind(format!("{:?}", adjustment.adjustment_type))
        .bind(&adjustment.reason)
        .bind(format!("{:?}", adjustment.status))
        .bind(adjustment.total_value_change)
        .bind(adjustment.approved_by.map(|id| id.to_string()))
        .bind(adjustment.approved_at.map(|d| d.to_rfc3339()))
        .bind(&adjustment.notes)
        .bind(&now)
        .bind(&now)
        .bind(adjustment.base.created_by.map(|id| id.to_string()))
        .execute(&self.pool)
        .await?;
        Ok(adjustment.clone())
    }

    async fn get(&self, id: Uuid) -> anyhow::Result<Option<InventoryAdjustment>> {
        let row = sqlx::query_as::<_, (String, String, String, String, String, String, i64, Option<String>, Option<String>, Option<String>, String, String, Option<String>)>(
            "SELECT id, adjustment_number, warehouse_id, adjustment_type, reason, status, 
                    total_value_change, approved_by, approved_at, notes, created_at, updated_at, created_by 
             FROM inventory_adjustments WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| {
            let created_by = r.12.as_ref().and_then(|s| Uuid::parse_str(s).ok());
            InventoryAdjustment {
                base: BaseEntity {
                    id: Uuid::parse_str(&r.0).unwrap_or_default(),
                    created_at: chrono::DateTime::parse_from_rfc3339(&r.10).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&r.11).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                    created_by,
                    updated_by: None,
                },
                adjustment_number: r.1,
                warehouse_id: Uuid::parse_str(&r.2).unwrap_or_default(),
                adjustment_type: parse_adjustment_type(&r.3),
                reason: r.4,
                status: parse_status(&r.5),
                total_value_change: r.6,
                approved_by: r.7.and_then(|s| Uuid::parse_str(&s).ok()),
                approved_at: r.8.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok()).map(|d| d.with_timezone(&Utc)),
                notes: r.9,
            }
        }))
    }

    async fn list(&self, warehouse_id: Option<Uuid>, status: Option<AdjustmentStatus>) -> anyhow::Result<Vec<InventoryAdjustment>> {
        let mut query = "SELECT id, adjustment_number, warehouse_id, adjustment_type, reason, status, 
                        total_value_change, approved_by, approved_at, notes, created_at, updated_at, created_by 
                        FROM inventory_adjustments WHERE 1=1".to_string();
        let mut binds: Vec<String> = Vec::new();

        if warehouse_id.is_some() {
            query.push_str(" AND warehouse_id = ?");
            binds.push(warehouse_id.unwrap().to_string());
        }
        if status.is_some() {
            query.push_str(" AND status = ?");
            binds.push(format!("{:?}", status.unwrap()));
        }
        query.push_str(" ORDER BY created_at DESC");

        let mut sql_query = sqlx::query_as::<_, (String, String, String, String, String, String, i64, Option<String>, Option<String>, Option<String>, String, String, Option<String>)>(&query);
        for bind in binds {
            sql_query = sql_query.bind(bind);
        }

        let rows = sql_query.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| {
            let created_by = r.12.as_ref().and_then(|s| Uuid::parse_str(s).ok());
            InventoryAdjustment {
                base: BaseEntity {
                    id: Uuid::parse_str(&r.0).unwrap_or_default(),
                    created_at: chrono::DateTime::parse_from_rfc3339(&r.10).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&r.11).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                    created_by,
                    updated_by: None,
                },
                adjustment_number: r.1,
                warehouse_id: Uuid::parse_str(&r.2).unwrap_or_default(),
                adjustment_type: parse_adjustment_type(&r.3),
                reason: r.4,
                status: parse_status(&r.5),
                total_value_change: r.6,
                approved_by: r.7.and_then(|s| Uuid::parse_str(&s).ok()),
                approved_at: r.8.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok()).map(|d| d.with_timezone(&Utc)),
                notes: r.9,
            }
        }).collect())
    }

    async fn update(&self, adjustment: &InventoryAdjustment) -> anyhow::Result<InventoryAdjustment> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE inventory_adjustments SET warehouse_id=?, adjustment_type=?, reason=?, status=?, 
             total_value_change=?, approved_by=?, approved_at=?, notes=?, updated_at=? WHERE id=?"
        )
        .bind(adjustment.warehouse_id.to_string())
        .bind(format!("{:?}", adjustment.adjustment_type))
        .bind(&adjustment.reason)
        .bind(format!("{:?}", adjustment.status))
        .bind(adjustment.total_value_change)
        .bind(adjustment.approved_by.map(|id| id.to_string()))
        .bind(adjustment.approved_at.map(|d| d.to_rfc3339()))
        .bind(&adjustment.notes)
        .bind(&now)
        .bind(adjustment.base.id.to_string())
        .execute(&self.pool)
        .await?;
        Ok(adjustment.clone())
    }

    async fn delete(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM inventory_adjustments WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn add_line(&self, line: &InventoryAdjustmentLine) -> anyhow::Result<InventoryAdjustmentLine> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            r#"INSERT INTO inventory_adjustment_lines 
               (id, adjustment_id, product_id, location_id, system_quantity, counted_quantity, 
                adjustment_quantity, unit_cost, total_value_change, lot_number, serial_number, 
                reason_code, notes, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(line.id.to_string())
        .bind(line.adjustment_id.to_string())
        .bind(line.product_id.to_string())
        .bind(line.location_id.to_string())
        .bind(line.system_quantity)
        .bind(line.counted_quantity)
        .bind(line.adjustment_quantity)
        .bind(line.unit_cost)
        .bind(line.total_value_change)
        .bind(&line.lot_number)
        .bind(&line.serial_number)
        .bind(&line.reason_code)
        .bind(&line.notes)
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(line.clone())
    }

    async fn get_lines(&self, adjustment_id: Uuid) -> anyhow::Result<Vec<InventoryAdjustmentLine>> {
        let rows = sqlx::query_as::<_, (String, String, String, String, i64, i64, i64, i64, i64, Option<String>, Option<String>, Option<String>, Option<String>, String)>(
            "SELECT id, adjustment_id, product_id, location_id, system_quantity, counted_quantity, 
                    adjustment_quantity, unit_cost, total_value_change, lot_number, serial_number, 
                    reason_code, notes, created_at 
             FROM inventory_adjustment_lines WHERE adjustment_id = ?"
        )
        .bind(adjustment_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| InventoryAdjustmentLine {
            id: Uuid::parse_str(&r.0).unwrap_or_default(),
            adjustment_id: Uuid::parse_str(&r.1).unwrap_or_default(),
            product_id: Uuid::parse_str(&r.2).unwrap_or_default(),
            location_id: Uuid::parse_str(&r.3).unwrap_or_default(),
            system_quantity: r.4,
            counted_quantity: r.5,
            adjustment_quantity: r.6,
            unit_cost: r.7,
            total_value_change: r.8,
            lot_number: r.9,
            serial_number: r.10,
            reason_code: r.11,
            notes: r.12,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.13).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }).collect())
    }

    async fn delete_lines(&self, adjustment_id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM inventory_adjustment_lines WHERE adjustment_id = ?")
            .bind(adjustment_id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_next_number(&self) -> anyhow::Result<String> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM inventory_adjustments")
            .fetch_one(&self.pool)
            .await?;
        Ok(format!("ADJ-{:06}", count.0 + 1))
    }
}

fn parse_adjustment_type(s: &str) -> AdjustmentType {
    match s {
        "CountVariance" => AdjustmentType::CountVariance,
        "Damage" => AdjustmentType::Damage,
        "Theft" => AdjustmentType::Theft,
        "Expired" => AdjustmentType::Expired,
        "Obsolete" => AdjustmentType::Obsolete,
        "Found" => AdjustmentType::Found,
        "TransferCorrection" => AdjustmentType::TransferCorrection,
        _ => AdjustmentType::Other,
    }
}

fn parse_status(s: &str) -> AdjustmentStatus {
    match s {
        "Draft" => AdjustmentStatus::Draft,
        "Pending" => AdjustmentStatus::Pending,
        "Approved" => AdjustmentStatus::Approved,
        "Rejected" => AdjustmentStatus::Rejected,
        "Completed" => AdjustmentStatus::Completed,
        _ => AdjustmentStatus::Cancelled,
    }
}
