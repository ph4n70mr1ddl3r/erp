use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status};
use crate::models::*;

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct BomRow { id: String, product_id: String, name: String, version: String, quantity: i64, status: String, created_at: String, updated_at: String }

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct BomComponentRow { id: String, bom_id: String, product_id: String, quantity: i64, unit: String, scrap_percent: f64 }

pub struct SqliteBillOfMaterialRepository;

#[async_trait]
impl BillOfMaterialRepository for SqliteBillOfMaterialRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<BillOfMaterial> {
        let row = sqlx::query_as::<_, BomRow>("SELECT id, product_id, name, version, quantity, status, created_at, updated_at FROM bills_of_material WHERE id = ?")
            .bind(id.to_string()).fetch_optional(pool).await?.ok_or_else(|| Error::not_found("BillOfMaterial", &id.to_string()))?;
        
        let components = sqlx::query_as::<_, BomComponentRow>("SELECT id, bom_id, product_id, quantity, unit, scrap_percent FROM bom_components WHERE bom_id = ?")
            .bind(id.to_string()).fetch_all(pool).await?;
        
        Ok(BillOfMaterial {
            base: BaseEntity { id: Uuid::parse_str(&row.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None, updated_by: None },
            product_id: Uuid::parse_str(&row.product_id).unwrap_or_default(), name: row.name, version: row.version, quantity: row.quantity,
            components: components.into_iter().map(|c| BomComponent {
                id: Uuid::parse_str(&c.id).unwrap_or_default(), product_id: Uuid::parse_str(&c.product_id).unwrap_or_default(),
                quantity: c.quantity, unit: c.unit, scrap_percent: c.scrap_percent,
            }).collect(),
            operations: vec![],
            status: match row.status.as_str() { "Inactive" => Status::Inactive, _ => Status::Active },
        })
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<BillOfMaterial>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM bills_of_material").fetch_one(pool).await?;
        let rows = sqlx::query_as::<_, BomRow>("SELECT id, product_id, name, version, quantity, status, created_at, updated_at FROM bills_of_material ORDER BY name LIMIT ? OFFSET ?")
            .bind(pagination.limit() as i64).bind(pagination.offset() as i64).fetch_all(pool).await?;
        
        let mut boms = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id).unwrap_or_default();
            let components = sqlx::query_as::<_, BomComponentRow>("SELECT id, bom_id, product_id, quantity, unit, scrap_percent FROM bom_components WHERE bom_id = ?")
                .bind(id.to_string()).fetch_all(pool).await?;
            boms.push(BillOfMaterial {
                base: BaseEntity { id, created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()), created_by: None, updated_by: None },
                product_id: Uuid::parse_str(&row.product_id).unwrap_or_default(), name: row.name, version: row.version, quantity: row.quantity,
                components: components.into_iter().map(|c| BomComponent {
                    id: Uuid::parse_str(&c.id).unwrap_or_default(), product_id: Uuid::parse_str(&c.product_id).unwrap_or_default(),
                    quantity: c.quantity, unit: c.unit, scrap_percent: c.scrap_percent,
                }).collect(),
                operations: vec![],
                status: match row.status.as_str() { "Inactive" => Status::Inactive, _ => Status::Active },
            });
        }
        Ok(Paginated::new(boms, count.0 as u64, pagination))
    }

    async fn create(&self, pool: &SqlitePool, bom: BillOfMaterial) -> Result<BillOfMaterial> {
        let now = Utc::now();
        let mut tx = pool.begin().await?;
        
        sqlx::query("INSERT INTO bills_of_material (id, product_id, name, version, quantity, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(bom.base.id.to_string()).bind(bom.product_id.to_string()).bind(&bom.name).bind(&bom.version)
            .bind(bom.quantity).bind(format!("{:?}", bom.status)).bind(bom.base.created_at.to_rfc3339()).bind(now.to_rfc3339())
            .execute(&mut *tx).await?;
        
        for c in &bom.components {
            sqlx::query("INSERT INTO bom_components (id, bom_id, product_id, quantity, unit, scrap_percent) VALUES (?, ?, ?, ?, ?, ?)")
                .bind(c.id.to_string()).bind(bom.base.id.to_string()).bind(c.product_id.to_string())
                .bind(c.quantity).bind(&c.unit).bind(c.scrap_percent).execute(&mut *tx).await?;
        }
        tx.commit().await?;
        Ok(bom)
    }
}

#[derive(sqlx::FromRow)]
struct WorkOrderRow { id: String, order_number: String, product_id: String, bom_id: String, quantity: i64,
    planned_start: String, planned_end: String, actual_start: Option<String>, actual_end: Option<String>, status: String, created_at: String, updated_at: String }

pub struct SqliteWorkOrderRepository;

#[async_trait]
impl WorkOrderRepository for SqliteWorkOrderRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<WorkOrder> {
        let row = sqlx::query_as::<_, WorkOrderRow>(
            "SELECT id, order_number, product_id, bom_id, quantity, planned_start, planned_end, actual_start, actual_end, status, created_at, updated_at FROM work_orders WHERE id = ?")
            .bind(id.to_string()).fetch_optional(pool).await?.ok_or_else(|| Error::not_found("WorkOrder", &id.to_string()))?;
        
        Ok(WorkOrder {
            base: BaseEntity { id: Uuid::parse_str(&row.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None, updated_by: None },
            order_number: row.order_number, product_id: Uuid::parse_str(&row.product_id).unwrap_or_default(),
            bom_id: Uuid::parse_str(&row.bom_id).unwrap_or_default(), quantity: row.quantity,
            planned_start: chrono::DateTime::parse_from_rfc3339(&row.planned_start).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            planned_end: chrono::DateTime::parse_from_rfc3339(&row.planned_end).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            actual_start: row.actual_start.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            actual_end: row.actual_end.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            status: match row.status.as_str() { "InProgress" => Status::Pending, "Completed" => Status::Completed, _ => Status::Draft },
        })
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<WorkOrder>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM work_orders").fetch_one(pool).await?;
        let rows = sqlx::query_as::<_, WorkOrderRow>(
            "SELECT id, order_number, product_id, bom_id, quantity, planned_start, planned_end, actual_start, actual_end, status, created_at, updated_at FROM work_orders ORDER BY planned_start DESC LIMIT ? OFFSET ?")
            .bind(pagination.limit() as i64).bind(pagination.offset() as i64).fetch_all(pool).await?;
        
        Ok(Paginated::new(rows.into_iter().map(|row| WorkOrder {
            base: BaseEntity { id: Uuid::parse_str(&row.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None, updated_by: None },
            order_number: row.order_number, product_id: Uuid::parse_str(&row.product_id).unwrap_or_default(),
            bom_id: Uuid::parse_str(&row.bom_id).unwrap_or_default(), quantity: row.quantity,
            planned_start: chrono::DateTime::parse_from_rfc3339(&row.planned_start).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            planned_end: chrono::DateTime::parse_from_rfc3339(&row.planned_end).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            actual_start: row.actual_start.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            actual_end: row.actual_end.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            status: match row.status.as_str() { "InProgress" => Status::Pending, "Completed" => Status::Completed, _ => Status::Draft },
        }).collect(), count.0 as u64, pagination))
    }

    async fn create(&self, pool: &SqlitePool, order: WorkOrder) -> Result<WorkOrder> {
        let now = Utc::now();
        sqlx::query("INSERT INTO work_orders (id, order_number, product_id, bom_id, quantity, planned_start, planned_end, actual_start, actual_end, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(order.base.id.to_string()).bind(&order.order_number).bind(order.product_id.to_string())
            .bind(order.bom_id.to_string()).bind(order.quantity).bind(order.planned_start.to_rfc3339())
            .bind(order.planned_end.to_rfc3339()).bind(order.actual_start.map(|d| d.to_rfc3339()))
            .bind(order.actual_end.map(|d| d.to_rfc3339())).bind(format!("{:?}", order.status))
            .bind(order.base.created_at.to_rfc3339()).bind(now.to_rfc3339()).execute(pool).await?;
        Ok(order)
    }

    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: Status, actual_start: Option<String>, actual_end: Option<String>) -> Result<()> {
        let rows = sqlx::query("UPDATE work_orders SET status = ?, actual_start = COALESCE(?, actual_start), actual_end = COALESCE(?, actual_end), updated_at = ? WHERE id = ?")
            .bind(format!("{:?}", status)).bind(actual_start).bind(actual_end).bind(Utc::now().to_rfc3339()).bind(id.to_string()).execute(pool).await?;
        if rows.rows_affected() == 0 { return Err(Error::not_found("WorkOrder", &id.to_string())); }
        Ok(())
    }
}

#[async_trait]
pub trait BillOfMaterialRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<BillOfMaterial>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<BillOfMaterial>>;
    async fn create(&self, pool: &SqlitePool, bom: BillOfMaterial) -> Result<BillOfMaterial>;
}

#[async_trait]
pub trait WorkOrderRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<WorkOrder>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<WorkOrder>>;
    async fn create(&self, pool: &SqlitePool, order: WorkOrder) -> Result<WorkOrder>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: Status, actual_start: Option<String>, actual_end: Option<String>) -> Result<()>;
}
