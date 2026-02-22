use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status};
use crate::models::*;
use crate::repository::*;

pub struct BillOfMaterialService { repo: SqliteBillOfMaterialRepository }
impl BillOfMaterialService {
    pub fn new() -> Self { Self { repo: SqliteBillOfMaterialRepository } }
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<BillOfMaterial> { self.repo.find_by_id(pool, id).await }
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<BillOfMaterial>> { self.repo.find_all(pool, pagination).await }
    
    pub async fn create(&self, pool: &SqlitePool, mut bom: BillOfMaterial) -> Result<BillOfMaterial> {
        if bom.components.is_empty() { return Err(Error::validation("BOM must have at least one component")); }
        if bom.name.is_empty() { return Err(Error::validation("BOM name is required")); }
        bom.base = BaseEntity::new();
        bom.status = Status::Draft;
        for c in &mut bom.components { c.id = Uuid::new_v4(); }
        self.repo.create(pool, bom).await
    }
}

pub struct WorkOrderService { repo: SqliteWorkOrderRepository }
impl WorkOrderService {
    pub fn new() -> Self { Self { repo: SqliteWorkOrderRepository } }
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<WorkOrder> { self.repo.find_by_id(pool, id).await }
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<WorkOrder>> { self.repo.find_all(pool, pagination).await }
    
    pub async fn create(&self, pool: &SqlitePool, mut order: WorkOrder) -> Result<WorkOrder> {
        if order.quantity <= 0 { return Err(Error::validation("Quantity must be positive")); }
        order.order_number = format!("WO-{}", Utc::now().format("%Y%m%d%H%M%S"));
        order.base = BaseEntity::new();
        order.status = Status::Draft;
        self.repo.create(pool, order).await
    }
    
    pub async fn start(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, Status::Pending, Some(Utc::now().to_rfc3339()), None).await
    }
    
    pub async fn complete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, Status::Completed, None, Some(Utc::now().to_rfc3339())).await
    }
}

pub struct ProductionScheduleService;

impl ProductionScheduleService {
    pub fn new() -> Self { Self }

    pub async fn create_schedule(
        pool: &SqlitePool,
        work_order_id: Uuid,
        work_center_id: Uuid,
        start_time: &str,
        end_time: &str,
        notes: Option<&str>,
    ) -> Result<ProductionSchedule> {
        let now = chrono::Utc::now();
        let schedule_number = format!("PS-{}", now.format("%Y%m%d%H%M%S"));
        let schedule = ProductionSchedule {
            id: Uuid::new_v4(),
            schedule_number: schedule_number.clone(),
            work_order_id,
            work_center_id,
            start_time: chrono::DateTime::parse_from_rfc3339(start_time)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or(now),
            end_time: chrono::DateTime::parse_from_rfc3339(end_time)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or(now),
            status: ScheduleStatus::Planned,
            actual_start: None,
            actual_end: None,
            notes: notes.map(|s| s.to_string()),
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO production_schedules (id, schedule_number, work_order_id, work_center_id, start_time, end_time, status, actual_start, actual_end, notes, created_at)
             VALUES (?, ?, ?, ?, ?, ?, 'Planned', NULL, NULL, ?, ?)"
        )
        .bind(schedule.id.to_string())
        .bind(&schedule.schedule_number)
        .bind(schedule.work_order_id.to_string())
        .bind(schedule.work_center_id.to_string())
        .bind(schedule.start_time.to_rfc3339())
        .bind(schedule.end_time.to_rfc3339())
        .bind(&schedule.notes)
        .bind(schedule.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(schedule)
    }

    pub async fn get(pool: &SqlitePool, id: Uuid) -> Result<ProductionSchedule> {
        let row = sqlx::query_as::<_, ScheduleRow>(
            "SELECT id, schedule_number, work_order_id, work_center_id, start_time, end_time, status, actual_start, actual_end, notes, created_at
             FROM production_schedules WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("ProductionSchedule", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn list(pool: &SqlitePool, work_center_id: Option<Uuid>) -> Result<Vec<ProductionSchedule>> {
        let rows = match work_center_id {
            Some(wc_id) => {
                sqlx::query_as::<_, ScheduleRow>(
                    "SELECT id, schedule_number, work_order_id, work_center_id, start_time, end_time, status, actual_start, actual_end, notes, created_at
                     FROM production_schedules WHERE work_center_id = ? ORDER BY start_time"
                )
                .bind(wc_id.to_string())
                .fetch_all(pool)
                .await
            }
            None => {
                sqlx::query_as::<_, ScheduleRow>(
                    "SELECT id, schedule_number, work_order_id, work_center_id, start_time, end_time, status, actual_start, actual_end, notes, created_at
                     FROM production_schedules ORDER BY start_time"
                )
                .fetch_all(pool)
                .await
            }
        }.map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn start(pool: &SqlitePool, id: Uuid) -> Result<ProductionSchedule> {
        let now = chrono::Utc::now();
        sqlx::query(
            "UPDATE production_schedules SET status = 'InProgress', actual_start = ? WHERE id = ?"
        )
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get(pool, id).await
    }

    pub async fn complete(pool: &SqlitePool, id: Uuid) -> Result<ProductionSchedule> {
        let now = chrono::Utc::now();
        sqlx::query(
            "UPDATE production_schedules SET status = 'Completed', actual_end = ? WHERE id = ?"
        )
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get(pool, id).await
    }
}

#[derive(sqlx::FromRow)]
struct ScheduleRow {
    id: String,
    schedule_number: String,
    work_order_id: String,
    work_center_id: String,
    start_time: String,
    end_time: String,
    status: String,
    actual_start: Option<String>,
    actual_end: Option<String>,
    notes: Option<String>,
    created_at: String,
}

impl From<ScheduleRow> for ProductionSchedule {
    fn from(r: ScheduleRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            schedule_number: r.schedule_number,
            work_order_id: Uuid::parse_str(&r.work_order_id).unwrap_or_default(),
            work_center_id: Uuid::parse_str(&r.work_center_id).unwrap_or_default(),
            start_time: chrono::DateTime::parse_from_rfc3339(&r.start_time)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            end_time: chrono::DateTime::parse_from_rfc3339(&r.end_time)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            status: match r.status.as_str() {
                "InProgress" => ScheduleStatus::InProgress,
                "Completed" => ScheduleStatus::Completed,
                "Cancelled" => ScheduleStatus::Cancelled,
                _ => ScheduleStatus::Planned,
            },
            actual_start: r.actual_start.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            actual_end: r.actual_end.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            notes: r.notes,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}
