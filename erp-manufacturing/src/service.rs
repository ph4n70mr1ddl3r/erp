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
        .map_err(Error::Database)?;
        
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
        .map_err(Error::Database)?
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
        }.map_err(Error::Database)?;
        
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
        .map_err(Error::Database)?;
        
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
        .map_err(Error::Database)?;
        
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

pub struct MRPService;

impl MRPService {
    pub fn new() -> Self { Self }

    pub async fn run_mrp(pool: &SqlitePool, planning_horizon_days: i32) -> Result<MRPRun> {
        if planning_horizon_days <= 0 {
            return Err(Error::validation("Planning horizon must be positive"));
        }
        let now = Utc::now();
        let run_number = format!("MRP-{}", now.format("%Y%m%d%H%M%S"));
        let run_id = Uuid::new_v4();
        
        sqlx::query(
            "INSERT INTO mrp_runs (id, run_number, run_date, planning_horizon_days, status, created_at)
             VALUES (?, ?, ?, ?, 'Running', ?)"
        )
        .bind(run_id.to_string())
        .bind(&run_number)
        .bind(now.to_rfc3339())
        .bind(planning_horizon_days)
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        let planned_orders = Self::generate_planned_orders(pool, run_id, planning_horizon_days).await?;
        for order in planned_orders {
            sqlx::query(
                "INSERT INTO mrp_planned_orders (id, mrp_run_id, product_id, order_type, quantity, due_date, release_date, source_type, source_id, status)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'Planned')"
            )
            .bind(order.id.to_string())
            .bind(order.mrp_run_id.to_string())
            .bind(order.product_id.to_string())
            .bind(Self::order_type_to_str(&order.order_type))
            .bind(order.quantity)
            .bind(order.due_date.to_rfc3339())
            .bind(order.release_date.map(|d| d.to_rfc3339()))
            .bind(&order.source_type)
            .bind(order.source_id.map(|id| id.to_string()))
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        }
        
        sqlx::query("UPDATE mrp_runs SET status = 'Completed', completed_at = ? WHERE id = ?")
            .bind(Utc::now().to_rfc3339())
            .bind(run_id.to_string())
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        
        Self::get_mrp_run(pool, run_id).await
    }

    async fn generate_planned_orders(pool: &SqlitePool, run_id: Uuid, horizon_days: i32) -> Result<Vec<MRPPlannedOrder>> {
        let _ = (pool, run_id, horizon_days);
        Ok(vec![])
    }

    pub async fn get_mrp_run(pool: &SqlitePool, id: Uuid) -> Result<MRPRun> {
        let row = sqlx::query_as::<_, MRPRunRow>(
            "SELECT id, run_number, run_date, planning_horizon_days, status, completed_at, created_at FROM mrp_runs WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("MRPRun", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn get_planned_orders(pool: &SqlitePool, mrp_run_id: Uuid) -> Result<Vec<MRPPlannedOrder>> {
        let rows = sqlx::query_as::<_, MRPPlannedOrderRow>(
            "SELECT id, mrp_run_id, product_id, order_type, quantity, due_date, release_date, source_type, source_id, status
             FROM mrp_planned_orders WHERE mrp_run_id = ?"
        )
        .bind(mrp_run_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn release_order(pool: &SqlitePool, planned_order_id: Uuid) -> Result<MRPPlannedOrder> {
        let order = Self::get_planned_order_by_id(pool, planned_order_id).await?;
        if order.status != MRPPlannedOrderStatus::Planned {
            return Err(Error::validation("Only planned orders can be released"));
        }
        
        let now = Utc::now();
        match order.order_type {
            MRPOrderType::Production => {
                let wo_number = format!("WO-{}", now.format("%Y%m%d%H%M%S"));
                sqlx::query(
                    "INSERT INTO work_orders (id, order_number, product_id, bom_id, quantity, planned_start, planned_end, actual_start, actual_end, status, created_at, updated_at)
                     VALUES (?, ?, ?, ?, ?, ?, ?, NULL, NULL, 'Released', ?, ?)"
                )
                .bind(Uuid::new_v4().to_string())
                .bind(&wo_number)
                .bind(order.product_id.to_string())
                .bind(Uuid::nil().to_string())
                .bind(order.quantity)
                .bind(order.due_date.to_rfc3339())
                .bind(order.due_date.to_rfc3339())
                .bind(now.to_rfc3339())
                .bind(now.to_rfc3339())
                .execute(pool)
                .await
                .map_err(Error::Database)?;
            }
            MRPOrderType::Purchase => {}
            MRPOrderType::Transfer => {}
        }
        
        sqlx::query("UPDATE mrp_planned_orders SET status = 'Released' WHERE id = ?")
            .bind(planned_order_id.to_string())
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        
        Self::get_planned_order_by_id(pool, planned_order_id).await
    }

    async fn get_planned_order_by_id(pool: &SqlitePool, id: Uuid) -> Result<MRPPlannedOrder> {
        let row = sqlx::query_as::<_, MRPPlannedOrderRow>(
            "SELECT id, mrp_run_id, product_id, order_type, quantity, due_date, release_date, source_type, source_id, status
             FROM mrp_planned_orders WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("MRPPlannedOrder", &id.to_string()))?;
        
        Ok(row.into())
    }

    fn order_type_to_str(t: &MRPOrderType) -> &'static str {
        match t {
            MRPOrderType::Production => "Production",
            MRPOrderType::Purchase => "Purchase",
            MRPOrderType::Transfer => "Transfer",
        }
    }
}

#[derive(sqlx::FromRow)]
struct MRPRunRow {
    id: String,
    run_number: String,
    run_date: String,
    planning_horizon_days: i32,
    status: String,
    completed_at: Option<String>,
    created_at: String,
}

impl From<MRPRunRow> for MRPRun {
    fn from(r: MRPRunRow) -> Self {
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: Utc::now(),
                created_by: None,
                updated_by: None,
            },
            run_number: r.run_number,
            run_date: chrono::DateTime::parse_from_rfc3339(&r.run_date).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            planning_horizon_days: r.planning_horizon_days,
            status: match r.status.as_str() { "Completed" => MRPRunStatus::Completed, "Failed" => MRPRunStatus::Failed, _ => MRPRunStatus::Running },
            completed_at: r.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
        }
    }
}

#[derive(sqlx::FromRow)]
struct MRPPlannedOrderRow {
    id: String,
    mrp_run_id: String,
    product_id: String,
    order_type: String,
    quantity: i64,
    due_date: String,
    release_date: Option<String>,
    source_type: Option<String>,
    source_id: Option<String>,
    status: String,
}

impl From<MRPPlannedOrderRow> for MRPPlannedOrder {
    fn from(r: MRPPlannedOrderRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            mrp_run_id: Uuid::parse_str(&r.mrp_run_id).unwrap_or_default(),
            product_id: Uuid::parse_str(&r.product_id).unwrap_or_default(),
            order_type: match r.order_type.as_str() { "Purchase" => MRPOrderType::Purchase, "Transfer" => MRPOrderType::Transfer, _ => MRPOrderType::Production },
            quantity: r.quantity,
            due_date: chrono::DateTime::parse_from_rfc3339(&r.due_date).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            release_date: r.release_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            source_type: r.source_type,
            source_id: r.source_id.and_then(|s| Uuid::parse_str(&s).ok()),
            status: match r.status.as_str() { "Released" => MRPPlannedOrderStatus::Released, "Firmed" => MRPPlannedOrderStatus::Firmed, "Cancelled" => MRPPlannedOrderStatus::Cancelled, _ => MRPPlannedOrderStatus::Planned },
        }
    }
}

pub struct CapacityPlanningService;

impl CapacityPlanningService {
    pub fn new() -> Self { Self }

    pub async fn create_capacity_plan(
        pool: &SqlitePool,
        work_center_id: Uuid,
        period_start: &str,
        period_end: &str,
        available_hours: i64,
    ) -> Result<CapacityPlan> {
        if available_hours <= 0 {
            return Err(Error::validation("Available hours must be positive"));
        }
        let now = Utc::now();
        let plan_id = Uuid::new_v4();
        let plan_number = format!("CP-{}", now.format("%Y%m%d%H%M%S"));
        let start = chrono::DateTime::parse_from_rfc3339(period_start).map(|d| d.with_timezone(&Utc)).unwrap_or(now);
        let end = chrono::DateTime::parse_from_rfc3339(period_end).map(|d| d.with_timezone(&Utc)).unwrap_or(now);
        
        sqlx::query(
            "INSERT INTO capacity_plans (id, plan_number, work_center_id, period_start, period_end, available_hours, planned_hours, actual_hours, utilization_percent, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, 0, 0, 0, 'Planned', ?)"
        )
        .bind(plan_id.to_string())
        .bind(&plan_number)
        .bind(work_center_id.to_string())
        .bind(start.to_rfc3339())
        .bind(end.to_rfc3339())
        .bind(available_hours)
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Self::get_capacity_plan(pool, plan_id).await
    }

    pub async fn get_capacity_plan(pool: &SqlitePool, id: Uuid) -> Result<CapacityPlan> {
        let row = sqlx::query_as::<_, CapacityPlanRow>(
            "SELECT id, plan_number, work_center_id, period_start, period_end, available_hours, planned_hours, actual_hours, utilization_percent, status, created_at FROM capacity_plans WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("CapacityPlan", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn calculate_requirements(pool: &SqlitePool, capacity_plan_id: Uuid) -> Result<Vec<CapacityRequirement>> {
        let plan = Self::get_capacity_plan(pool, capacity_plan_id).await?;
        let rows = sqlx::query_as::<_, CapacityRequirementRow>(
            "SELECT id, capacity_plan_id, work_order_id, operation_sequence, required_hours, scheduled_start, scheduled_end, status
             FROM capacity_requirements WHERE capacity_plan_id = ?"
        )
        .bind(capacity_plan_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        let total_hours: i64 = rows.iter().map(|r| r.required_hours).sum();
        let utilization = if plan.available_hours > 0 { (total_hours as f64 / plan.available_hours as f64) * 100.0 } else { 0.0 };
        
        sqlx::query("UPDATE capacity_plans SET planned_hours = ?, utilization_percent = ? WHERE id = ?")
            .bind(total_hours)
            .bind(utilization)
            .bind(capacity_plan_id.to_string())
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn check_capacity(pool: &SqlitePool, work_center_id: Uuid, required_hours: i64) -> Result<bool> {
        let row = sqlx::query_as::<_, (i64,)>(
            "SELECT COALESCE(SUM(available_hours), 0) FROM capacity_plans WHERE work_center_id = ? AND status = 'Active'"
        )
        .bind(work_center_id.to_string())
        .fetch_one(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(row.0 >= required_hours)
    }
}

#[derive(sqlx::FromRow)]
struct CapacityPlanRow {
    id: String,
    plan_number: String,
    work_center_id: String,
    period_start: String,
    period_end: String,
    available_hours: i64,
    planned_hours: i64,
    actual_hours: i64,
    utilization_percent: f64,
    status: String,
    created_at: String,
}

impl From<CapacityPlanRow> for CapacityPlan {
    fn from(r: CapacityPlanRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            plan_number: r.plan_number,
            work_center_id: Uuid::parse_str(&r.work_center_id).unwrap_or_default(),
            period_start: chrono::DateTime::parse_from_rfc3339(&r.period_start).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            period_end: chrono::DateTime::parse_from_rfc3339(&r.period_end).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            available_hours: r.available_hours,
            planned_hours: r.planned_hours,
            actual_hours: r.actual_hours,
            utilization_percent: r.utilization_percent,
            status: match r.status.as_str() { "Active" => CapacityPlanStatus::Active, "Closed" => CapacityPlanStatus::Closed, _ => CapacityPlanStatus::Planned },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct CapacityRequirementRow {
    id: String,
    capacity_plan_id: String,
    work_order_id: String,
    operation_sequence: i32,
    required_hours: i64,
    scheduled_start: Option<String>,
    scheduled_end: Option<String>,
    status: String,
}

impl From<CapacityRequirementRow> for CapacityRequirement {
    fn from(r: CapacityRequirementRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            capacity_plan_id: Uuid::parse_str(&r.capacity_plan_id).unwrap_or_default(),
            work_order_id: Uuid::parse_str(&r.work_order_id).unwrap_or_default(),
            operation_sequence: r.operation_sequence,
            required_hours: r.required_hours,
            scheduled_start: r.scheduled_start.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            scheduled_end: r.scheduled_end.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            status: match r.status.as_str() { "Scheduled" => CapacityRequirementStatus::Scheduled, "InProgress" => CapacityRequirementStatus::InProgress, "Completed" => CapacityRequirementStatus::Completed, _ => CapacityRequirementStatus::Planned },
        }
    }
}

pub struct MESService;

impl MESService {
    pub fn new() -> Self { Self }

    pub async fn start_operation(pool: &SqlitePool, work_order_id: Uuid, work_center_id: Uuid, operation_code: &str, quantity: i64, operator_id: Option<Uuid>) -> Result<ShopFloorOperation> {
        if quantity <= 0 { return Err(Error::validation("Quantity must be positive")); }
        let now = Utc::now();
        let op_id = Uuid::new_v4();
        let op_number = format!("OP-{}", now.format("%Y%m%d%H%M%S"));
        
        sqlx::query(
            "INSERT INTO shop_floor_operations (id, operation_number, work_order_id, work_center_id, operation_code, description, setup_time, run_time, quantity, completed_qty, scrapped_qty, status, started_at, completed_at, operator_id)
             VALUES (?, ?, ?, ?, ?, NULL, 0, 0, ?, 0, 0, 'InProgress', ?, NULL, ?)"
        )
        .bind(op_id.to_string())
        .bind(&op_number)
        .bind(work_order_id.to_string())
        .bind(work_center_id.to_string())
        .bind(operation_code)
        .bind(quantity)
        .bind(now.to_rfc3339())
        .bind(operator_id.map(|id| id.to_string()))
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Self::log_event(pool, op_id, ShopFloorLogType::Start, operator_id, None, None).await?;
        Self::get_operation(pool, op_id).await
    }

    pub async fn log_progress(pool: &SqlitePool, operation_id: Uuid, completed_qty: i64, operator_id: Option<Uuid>) -> Result<ShopFloorOperation> {
        let op = Self::get_operation(pool, operation_id).await?;
        if op.status != ShopFloorOperationStatus::InProgress {
            return Err(Error::validation("Operation is not in progress"));
        }
        if completed_qty > op.quantity {
            return Err(Error::validation("Completed quantity cannot exceed planned quantity"));
        }
        
        sqlx::query("UPDATE shop_floor_operations SET completed_qty = ? WHERE id = ?")
            .bind(completed_qty)
            .bind(operation_id.to_string())
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        
        Self::log_event(pool, operation_id, ShopFloorLogType::Resume, operator_id, Some(completed_qty), None).await?;
        Self::get_operation(pool, operation_id).await
    }

    pub async fn complete_operation(pool: &SqlitePool, operation_id: Uuid, operator_id: Option<Uuid>) -> Result<ShopFloorOperation> {
        let now = Utc::now();
        sqlx::query("UPDATE shop_floor_operations SET status = 'Completed', completed_at = ? WHERE id = ?")
            .bind(now.to_rfc3339())
            .bind(operation_id.to_string())
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        
        Self::log_event(pool, operation_id, ShopFloorLogType::Complete, operator_id, None, None).await?;
        Self::get_operation(pool, operation_id).await
    }

    pub async fn log_downtime(pool: &SqlitePool, work_center_id: Uuid, event_type: DowntimeEventType, reason_code: Option<&str>, description: Option<&str>, duration_minutes: i32) -> Result<DowntimeEvent> {
        let now = Utc::now();
        let event_id = Uuid::new_v4();
        
        sqlx::query(
            "INSERT INTO downtime_events (id, work_center_id, event_type, reason_code, description, started_at, ended_at, duration_minutes)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(event_id.to_string())
        .bind(work_center_id.to_string())
        .bind(Self::downtime_type_to_str(&event_type))
        .bind(reason_code)
        .bind(description)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(duration_minutes)
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Self::get_downtime_event(pool, event_id).await
    }

    async fn log_event(pool: &SqlitePool, operation_id: Uuid, log_type: ShopFloorLogType, operator_id: Option<Uuid>, quantity: Option<i64>, notes: Option<&str>) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO shop_floor_logs (id, operation_id, log_type, operator_id, quantity, reason, notes, logged_at) VALUES (?, ?, ?, ?, ?, NULL, ?, ?)"
        )
        .bind(Uuid::new_v4().to_string())
        .bind(operation_id.to_string())
        .bind(Self::log_type_to_str(&log_type))
        .bind(operator_id.map(|id| id.to_string()))
        .bind(quantity)
        .bind(notes)
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        Ok(())
    }

    fn log_type_to_str(t: &ShopFloorLogType) -> &'static str {
        match t { ShopFloorLogType::Start => "Start", ShopFloorLogType::Complete => "Complete", ShopFloorLogType::Scrap => "Scrap", ShopFloorLogType::Rework => "Rework", ShopFloorLogType::Hold => "Hold", ShopFloorLogType::Resume => "Resume" }
    }

    fn downtime_type_to_str(t: &DowntimeEventType) -> &'static str {
        match t { DowntimeEventType::Planned => "Planned", DowntimeEventType::Unplanned => "Unplanned", DowntimeEventType::Maintenance => "Maintenance", DowntimeEventType::Changeover => "Changeover", DowntimeEventType::Break => "Break" }
    }

    pub async fn get_operation(pool: &SqlitePool, id: Uuid) -> Result<ShopFloorOperation> {
        let row = sqlx::query_as::<_, ShopFloorOperationRow>(
            "SELECT id, operation_number, work_order_id, work_center_id, operation_code, description, setup_time, run_time, quantity, completed_qty, scrapped_qty, status, started_at, completed_at, operator_id FROM shop_floor_operations WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("ShopFloorOperation", &id.to_string()))?;
        
        Ok(row.into())
    }

    async fn get_downtime_event(pool: &SqlitePool, id: Uuid) -> Result<DowntimeEvent> {
        let row = sqlx::query_as::<_, DowntimeEventRow>(
            "SELECT id, work_center_id, event_type, reason_code, description, started_at, ended_at, duration_minutes FROM downtime_events WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("DowntimeEvent", &id.to_string()))?;
        
        Ok(row.into())
    }
}

#[derive(sqlx::FromRow)]
struct ShopFloorOperationRow {
    id: String,
    operation_number: String,
    work_order_id: String,
    work_center_id: String,
    operation_code: String,
    description: Option<String>,
    setup_time: i64,
    run_time: i64,
    quantity: i64,
    completed_qty: i64,
    scrapped_qty: i64,
    status: String,
    started_at: Option<String>,
    completed_at: Option<String>,
    operator_id: Option<String>,
}

impl From<ShopFloorOperationRow> for ShopFloorOperation {
    fn from(r: ShopFloorOperationRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            operation_number: r.operation_number,
            work_order_id: Uuid::parse_str(&r.work_order_id).unwrap_or_default(),
            work_center_id: Uuid::parse_str(&r.work_center_id).unwrap_or_default(),
            operation_code: r.operation_code,
            description: r.description,
            setup_time: r.setup_time,
            run_time: r.run_time,
            quantity: r.quantity,
            completed_qty: r.completed_qty,
            scrapped_qty: r.scrapped_qty,
            status: match r.status.as_str() { "InProgress" => ShopFloorOperationStatus::InProgress, "Completed" => ShopFloorOperationStatus::Completed, "OnHold" => ShopFloorOperationStatus::OnHold, _ => ShopFloorOperationStatus::Pending },
            started_at: r.started_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            completed_at: r.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            operator_id: r.operator_id.and_then(|s| Uuid::parse_str(&s).ok()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct DowntimeEventRow {
    id: String,
    work_center_id: String,
    event_type: String,
    reason_code: Option<String>,
    description: Option<String>,
    started_at: String,
    ended_at: Option<String>,
    duration_minutes: Option<i32>,
}

impl From<DowntimeEventRow> for DowntimeEvent {
    fn from(r: DowntimeEventRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            work_center_id: Uuid::parse_str(&r.work_center_id).unwrap_or_default(),
            event_type: match r.event_type.as_str() { "Unplanned" => DowntimeEventType::Unplanned, "Maintenance" => DowntimeEventType::Maintenance, "Changeover" => DowntimeEventType::Changeover, "Break" => DowntimeEventType::Break, _ => DowntimeEventType::Planned },
            reason_code: r.reason_code,
            description: r.description,
            started_at: chrono::DateTime::parse_from_rfc3339(&r.started_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            ended_at: r.ended_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            duration_minutes: r.duration_minutes,
        }
    }
}

pub struct MaintenanceService;

impl MaintenanceService {
    pub fn new() -> Self { Self }

    pub async fn create_equipment(
        pool: &SqlitePool,
        equipment_code: &str,
        name: &str,
        equipment_type: &str,
        work_center_id: Option<Uuid>,
        criticality: EquipmentCriticality,
    ) -> Result<Equipment> {
        if equipment_code.is_empty() || name.is_empty() {
            return Err(Error::validation("Equipment code and name are required"));
        }
        let now = Utc::now();
        let id = Uuid::new_v4();
        
        sqlx::query(
            "INSERT INTO equipment (id, equipment_code, name, description, equipment_type, manufacturer, model, serial_number, installation_date, warranty_expiry, location, work_center_id, parent_equipment_id, status, criticality, created_at)
             VALUES (?, ?, ?, NULL, ?, NULL, NULL, NULL, NULL, NULL, NULL, ?, NULL, 'Active', ?, ?)"
        )
        .bind(id.to_string())
        .bind(equipment_code)
        .bind(name)
        .bind(equipment_type)
        .bind(work_center_id.map(|id| id.to_string()))
        .bind(Self::criticality_to_str(&criticality))
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Self::get_equipment(pool, id).await
    }

    pub async fn get_equipment(pool: &SqlitePool, id: Uuid) -> Result<Equipment> {
        let row = sqlx::query_as::<_, EquipmentRow>(
            "SELECT id, equipment_code, name, description, equipment_type, manufacturer, model, serial_number, installation_date, warranty_expiry, location, work_center_id, parent_equipment_id, status, criticality, created_at FROM equipment WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("Equipment", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn create_maintenance_schedule(
        pool: &SqlitePool,
        equipment_id: Uuid,
        maintenance_type: MaintenanceType,
        frequency_type: FrequencyType,
        frequency_value: i32,
        next_maintenance: &str,
    ) -> Result<MaintenanceSchedule> {
        if frequency_value <= 0 { return Err(Error::validation("Frequency value must be positive")); }
        let now = Utc::now();
        let id = Uuid::new_v4();
        let schedule_number = format!("MS-{}", now.format("%Y%m%d%H%M%S"));
        let next = chrono::DateTime::parse_from_rfc3339(next_maintenance).map(|d| d.with_timezone(&Utc)).unwrap_or(now);
        
        sqlx::query(
            "INSERT INTO maintenance_schedules (id, schedule_number, equipment_id, maintenance_type, frequency_type, frequency_value, last_maintenance, next_maintenance, estimated_duration, assigned_to, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, NULL, ?, NULL, NULL, 'Active', ?)"
        )
        .bind(id.to_string())
        .bind(&schedule_number)
        .bind(equipment_id.to_string())
        .bind(Self::maintenance_type_to_str(&maintenance_type))
        .bind(Self::frequency_type_to_str(&frequency_type))
        .bind(frequency_value)
        .bind(next.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Self::get_schedule(pool, id).await
    }

    pub async fn create_work_order(
        pool: &SqlitePool,
        equipment_id: Uuid,
        maintenance_type: MaintenanceType,
        priority: MaintenancePriority,
        description: &str,
        requested_by: Option<Uuid>,
    ) -> Result<MaintenanceWorkOrder> {
        if description.is_empty() { return Err(Error::validation("Description is required")); }
        let now = Utc::now();
        let id = Uuid::new_v4();
        let work_order_number = format!("MWO-{}", now.format("%Y%m%d%H%M%S"));
        
        sqlx::query(
            "INSERT INTO maintenance_work_orders (id, work_order_number, equipment_id, schedule_id, maintenance_type, priority, description, requested_by, assigned_to, scheduled_date, started_at, completed_at, downtime_hours, labor_hours, parts_cost, labor_cost, status, created_at)
             VALUES (?, ?, ?, NULL, ?, ?, ?, ?, NULL, NULL, NULL, NULL, 0, 0, 0, 0, 'Requested', ?)"
        )
        .bind(id.to_string())
        .bind(&work_order_number)
        .bind(equipment_id.to_string())
        .bind(Self::maintenance_type_to_str(&maintenance_type))
        .bind(Self::priority_to_str(&priority))
        .bind(description)
        .bind(requested_by.map(|id| id.to_string()))
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Self::get_work_order(pool, id).await
    }

    pub async fn complete_work_order(pool: &SqlitePool, id: Uuid, downtime_hours: f64, labor_hours: f64) -> Result<MaintenanceWorkOrder> {
        let now = Utc::now();
        sqlx::query("UPDATE maintenance_work_orders SET status = 'Completed', completed_at = ?, downtime_hours = ?, labor_hours = ? WHERE id = ?")
            .bind(now.to_rfc3339())
            .bind(downtime_hours)
            .bind(labor_hours)
            .bind(id.to_string())
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        
        Self::get_work_order(pool, id).await
    }

    pub async fn get_due_maintenance(pool: &SqlitePool) -> Result<Vec<MaintenanceSchedule>> {
        let now = Utc::now();
        let rows = sqlx::query_as::<_, MaintenanceScheduleRow>(
            "SELECT id, schedule_number, equipment_id, maintenance_type, frequency_type, frequency_value, last_maintenance, next_maintenance, estimated_duration, assigned_to, status, created_at FROM maintenance_schedules WHERE next_maintenance <= ? AND status = 'Active'"
        )
        .bind(now.to_rfc3339())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn get_schedule(pool: &SqlitePool, id: Uuid) -> Result<MaintenanceSchedule> {
        let row = sqlx::query_as::<_, MaintenanceScheduleRow>(
            "SELECT id, schedule_number, equipment_id, maintenance_type, frequency_type, frequency_value, last_maintenance, next_maintenance, estimated_duration, assigned_to, status, created_at FROM maintenance_schedules WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("MaintenanceSchedule", &id.to_string()))?;
        Ok(row.into())
    }

    async fn get_work_order(pool: &SqlitePool, id: Uuid) -> Result<MaintenanceWorkOrder> {
        let row = sqlx::query_as::<_, MaintenanceWorkOrderRow>(
            "SELECT id, work_order_number, equipment_id, schedule_id, maintenance_type, priority, description, requested_by, assigned_to, scheduled_date, started_at, completed_at, downtime_hours, labor_hours, parts_cost, labor_cost, status, created_at FROM maintenance_work_orders WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("MaintenanceWorkOrder", &id.to_string()))?;
        Ok(row.into())
    }

    fn criticality_to_str(c: &EquipmentCriticality) -> &'static str {
        match c { EquipmentCriticality::Low => "Low", EquipmentCriticality::Medium => "Medium", EquipmentCriticality::High => "High", EquipmentCriticality::Critical => "Critical" }
    }

    fn maintenance_type_to_str(t: &MaintenanceType) -> &'static str {
        match t { MaintenanceType::Preventive => "Preventive", MaintenanceType::Corrective => "Corrective", MaintenanceType::Predictive => "Predictive", MaintenanceType::Inspection => "Inspection" }
    }

    fn frequency_type_to_str(t: &FrequencyType) -> &'static str {
        match t { FrequencyType::Daily => "Daily", FrequencyType::Weekly => "Weekly", FrequencyType::Monthly => "Monthly", FrequencyType::Quarterly => "Quarterly", FrequencyType::Yearly => "Yearly", FrequencyType::RuntimeHours => "RuntimeHours" }
    }

    fn priority_to_str(p: &MaintenancePriority) -> &'static str {
        match p { MaintenancePriority::Low => "Low", MaintenancePriority::Medium => "Medium", MaintenancePriority::High => "High", MaintenancePriority::Critical => "Critical" }
    }
}

#[derive(sqlx::FromRow)]
struct EquipmentRow {
    id: String,
    equipment_code: String,
    name: String,
    description: Option<String>,
    equipment_type: String,
    manufacturer: Option<String>,
    model: Option<String>,
    serial_number: Option<String>,
    installation_date: Option<String>,
    warranty_expiry: Option<String>,
    location: Option<String>,
    work_center_id: Option<String>,
    parent_equipment_id: Option<String>,
    status: String,
    criticality: String,
    created_at: String,
}

impl From<EquipmentRow> for Equipment {
    fn from(r: EquipmentRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            equipment_code: r.equipment_code,
            name: r.name,
            description: r.description,
            equipment_type: r.equipment_type,
            manufacturer: r.manufacturer,
            model: r.model,
            serial_number: r.serial_number,
            installation_date: r.installation_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            warranty_expiry: r.warranty_expiry.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            location: r.location,
            work_center_id: r.work_center_id.and_then(|s| Uuid::parse_str(&s).ok()),
            parent_equipment_id: r.parent_equipment_id.and_then(|s| Uuid::parse_str(&s).ok()),
            status: match r.status.as_str() { "Inactive" => EquipmentStatus::Inactive, "Maintenance" => EquipmentStatus::Maintenance, "Retired" => EquipmentStatus::Retired, _ => EquipmentStatus::Active },
            criticality: match r.criticality.as_str() { "Low" => EquipmentCriticality::Low, "Medium" => EquipmentCriticality::Medium, "High" => EquipmentCriticality::High, _ => EquipmentCriticality::Critical },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct MaintenanceScheduleRow {
    id: String,
    schedule_number: String,
    equipment_id: String,
    maintenance_type: String,
    frequency_type: String,
    frequency_value: i32,
    last_maintenance: Option<String>,
    next_maintenance: String,
    estimated_duration: Option<i32>,
    assigned_to: Option<String>,
    status: String,
    created_at: String,
}

impl From<MaintenanceScheduleRow> for MaintenanceSchedule {
    fn from(r: MaintenanceScheduleRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            schedule_number: r.schedule_number,
            equipment_id: Uuid::parse_str(&r.equipment_id).unwrap_or_default(),
            maintenance_type: match r.maintenance_type.as_str() { "Corrective" => MaintenanceType::Corrective, "Predictive" => MaintenanceType::Predictive, "Inspection" => MaintenanceType::Inspection, _ => MaintenanceType::Preventive },
            frequency_type: match r.frequency_type.as_str() { "Weekly" => FrequencyType::Weekly, "Monthly" => FrequencyType::Monthly, "Quarterly" => FrequencyType::Quarterly, "Yearly" => FrequencyType::Yearly, "RuntimeHours" => FrequencyType::RuntimeHours, _ => FrequencyType::Daily },
            frequency_value: r.frequency_value,
            last_maintenance: r.last_maintenance.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            next_maintenance: chrono::DateTime::parse_from_rfc3339(&r.next_maintenance).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            estimated_duration: r.estimated_duration,
            assigned_to: r.assigned_to.and_then(|s| Uuid::parse_str(&s).ok()),
            status: match r.status.as_str() { "Inactive" => MaintenanceScheduleStatus::Inactive, "Suspended" => MaintenanceScheduleStatus::Suspended, _ => MaintenanceScheduleStatus::Active },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct MaintenanceWorkOrderRow {
    id: String,
    work_order_number: String,
    equipment_id: String,
    schedule_id: Option<String>,
    maintenance_type: String,
    priority: String,
    description: String,
    requested_by: Option<String>,
    assigned_to: Option<String>,
    scheduled_date: Option<String>,
    started_at: Option<String>,
    completed_at: Option<String>,
    downtime_hours: f64,
    labor_hours: f64,
    parts_cost: i64,
    labor_cost: i64,
    status: String,
    created_at: String,
}

impl From<MaintenanceWorkOrderRow> for MaintenanceWorkOrder {
    fn from(r: MaintenanceWorkOrderRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            work_order_number: r.work_order_number,
            equipment_id: Uuid::parse_str(&r.equipment_id).unwrap_or_default(),
            schedule_id: r.schedule_id.and_then(|s| Uuid::parse_str(&s).ok()),
            maintenance_type: match r.maintenance_type.as_str() { "Corrective" => MaintenanceType::Corrective, "Predictive" => MaintenanceType::Predictive, "Inspection" => MaintenanceType::Inspection, _ => MaintenanceType::Preventive },
            priority: match r.priority.as_str() { "Low" => MaintenancePriority::Low, "Medium" => MaintenancePriority::Medium, "High" => MaintenancePriority::High, _ => MaintenancePriority::Critical },
            description: r.description,
            requested_by: r.requested_by.and_then(|s| Uuid::parse_str(&s).ok()),
            assigned_to: r.assigned_to.and_then(|s| Uuid::parse_str(&s).ok()),
            scheduled_date: r.scheduled_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            started_at: r.started_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            completed_at: r.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            downtime_hours: r.downtime_hours,
            labor_hours: r.labor_hours,
            parts_cost: r.parts_cost,
            labor_cost: r.labor_cost,
            status: match r.status.as_str() { "Scheduled" => MaintenanceWorkOrderStatus::Scheduled, "InProgress" => MaintenanceWorkOrderStatus::InProgress, "Completed" => MaintenanceWorkOrderStatus::Completed, "Cancelled" => MaintenanceWorkOrderStatus::Cancelled, _ => MaintenanceWorkOrderStatus::Requested },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }
    }
}

pub struct PLMService;

impl PLMService {
    pub fn new() -> Self { Self }

    pub async fn create_ecr(
        pool: &SqlitePool,
        title: &str,
        description: &str,
        change_type: ChangeType,
        reason: &str,
        requested_by: Uuid,
        priority: ECRPriority,
    ) -> Result<EngineeringChangeRequest> {
        if title.is_empty() || description.is_empty() || reason.is_empty() {
            return Err(Error::validation("Title, description, and reason are required"));
        }
        let now = Utc::now();
        let id = Uuid::new_v4();
        let ecr_number = format!("ECR-{}", now.format("%Y%m%d%H%M%S"));
        
        sqlx::query(
            "INSERT INTO engineering_change_requests (id, ecr_number, title, description, change_type, reason, impact_assessment, requested_by, priority, status, submitted_at, approved_at, created_at)
             VALUES (?, ?, ?, ?, ?, ?, NULL, ?, ?, 'Draft', NULL, NULL, ?)"
        )
        .bind(id.to_string())
        .bind(&ecr_number)
        .bind(title)
        .bind(description)
        .bind(Self::change_type_to_str(&change_type))
        .bind(reason)
        .bind(requested_by.to_string())
        .bind(Self::ecr_priority_to_str(&priority))
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Self::get_ecr(pool, id).await
    }

    pub async fn submit_ecr(pool: &SqlitePool, id: Uuid) -> Result<EngineeringChangeRequest> {
        let ecr = Self::get_ecr(pool, id).await?;
        if ecr.status != ECRStatus::Draft {
            return Err(Error::validation("Only draft ECRs can be submitted"));
        }
        let now = Utc::now();
        sqlx::query("UPDATE engineering_change_requests SET status = 'Submitted', submitted_at = ? WHERE id = ?")
            .bind(now.to_rfc3339())
            .bind(id.to_string())
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        Self::get_ecr(pool, id).await
    }

    pub async fn approve_ecr(pool: &SqlitePool, id: Uuid) -> Result<EngineeringChangeRequest> {
        let ecr = Self::get_ecr(pool, id).await?;
        if ecr.status != ECRStatus::Submitted && ecr.status != ECRStatus::UnderReview {
            return Err(Error::validation("ECR must be submitted or under review to approve"));
        }
        let now = Utc::now();
        sqlx::query("UPDATE engineering_change_requests SET status = 'Approved', approved_at = ? WHERE id = ?")
            .bind(now.to_rfc3339())
            .bind(id.to_string())
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        Self::get_ecr(pool, id).await
    }

    pub async fn create_eco(pool: &SqlitePool, ecr_id: Uuid, title: &str, effective_date: &str) -> Result<EngineeringChangeOrder> {
        let ecr = Self::get_ecr(pool, ecr_id).await?;
        if ecr.status != ECRStatus::Approved {
            return Err(Error::validation("ECR must be approved before creating ECO"));
        }
        let now = Utc::now();
        let id = Uuid::new_v4();
        let eco_number = format!("ECO-{}", now.format("%Y%m%d%H%M%S"));
        let eff_date = chrono::DateTime::parse_from_rfc3339(effective_date).map(|d| d.with_timezone(&Utc)).unwrap_or(now);
        
        sqlx::query(
            "INSERT INTO engineering_change_orders (id, eco_number, ecr_id, title, description, effective_date, approved_by, approval_date, status, created_at)
             VALUES (?, ?, ?, ?, NULL, ?, NULL, NULL, 'Draft', ?)"
        )
        .bind(id.to_string())
        .bind(&eco_number)
        .bind(ecr_id.to_string())
        .bind(title)
        .bind(eff_date.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Self::get_eco(pool, id).await
    }

    pub async fn approve_eco(pool: &SqlitePool, id: Uuid, approved_by: Uuid) -> Result<EngineeringChangeOrder> {
        let eco = Self::get_eco(pool, id).await?;
        if eco.status != ECOStatus::Draft {
            return Err(Error::validation("Only draft ECOs can be approved"));
        }
        let now = Utc::now();
        sqlx::query("UPDATE engineering_change_orders SET status = 'Approved', approved_by = ?, approval_date = ? WHERE id = ?")
            .bind(approved_by.to_string())
            .bind(now.to_rfc3339())
            .bind(id.to_string())
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        Self::get_eco(pool, id).await
    }

    pub async fn create_document_revision(
        pool: &SqlitePool,
        document_number: &str,
        revision: &str,
        title: &str,
        document_type: DocumentType,
        product_id: Option<Uuid>,
        created_by: Option<Uuid>,
    ) -> Result<DocumentRevision> {
        if document_number.is_empty() || revision.is_empty() || title.is_empty() {
            return Err(Error::validation("Document number, revision, and title are required"));
        }
        let now = Utc::now();
        let id = Uuid::new_v4();
        
        sqlx::query(
            "INSERT INTO document_revisions (id, document_number, revision, title, document_type, file_path, product_id, status, approved_by, approved_at, created_by, created_at)
             VALUES (?, ?, ?, ?, ?, NULL, ?, 'Draft', NULL, NULL, ?, ?)"
        )
        .bind(id.to_string())
        .bind(document_number)
        .bind(revision)
        .bind(title)
        .bind(Self::doc_type_to_str(&document_type))
        .bind(product_id.map(|id| id.to_string()))
        .bind(created_by.map(|id| id.to_string()))
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Self::get_document_revision(pool, id).await
    }

    pub async fn approve_revision(pool: &SqlitePool, id: Uuid, approved_by: Uuid) -> Result<DocumentRevision> {
        let doc = Self::get_document_revision(pool, id).await?;
        if doc.status != DocumentRevisionStatus::Draft && doc.status != DocumentRevisionStatus::UnderReview {
            return Err(Error::validation("Document must be draft or under review to approve"));
        }
        let now = Utc::now();
        sqlx::query("UPDATE document_revisions SET status = 'Approved', approved_by = ?, approved_at = ? WHERE id = ?")
            .bind(approved_by.to_string())
            .bind(now.to_rfc3339())
            .bind(id.to_string())
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        Self::get_document_revision(pool, id).await
    }

    async fn get_ecr(pool: &SqlitePool, id: Uuid) -> Result<EngineeringChangeRequest> {
        let row = sqlx::query_as::<_, ECRRow>(
            "SELECT id, ecr_number, title, description, change_type, reason, impact_assessment, requested_by, priority, status, submitted_at, approved_at, created_at FROM engineering_change_requests WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("EngineeringChangeRequest", &id.to_string()))?;
        Ok(row.into())
    }

    async fn get_eco(pool: &SqlitePool, id: Uuid) -> Result<EngineeringChangeOrder> {
        let row = sqlx::query_as::<_, ECORow>(
            "SELECT id, eco_number, ecr_id, title, description, effective_date, approved_by, approval_date, status, created_at FROM engineering_change_orders WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("EngineeringChangeOrder", &id.to_string()))?;
        Ok(row.into())
    }

    async fn get_document_revision(pool: &SqlitePool, id: Uuid) -> Result<DocumentRevision> {
        let row = sqlx::query_as::<_, DocumentRevisionRow>(
            "SELECT id, document_number, revision, title, document_type, file_path, product_id, status, approved_by, approved_at, created_by, created_at FROM document_revisions WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("DocumentRevision", &id.to_string()))?;
        Ok(row.into())
    }

    fn change_type_to_str(t: &ChangeType) -> &'static str {
        match t { ChangeType::Design => "Design", ChangeType::Process => "Process", ChangeType::Material => "Material", ChangeType::Documentation => "Documentation", ChangeType::Other => "Other" }
    }

    fn ecr_priority_to_str(p: &ECRPriority) -> &'static str {
        match p { ECRPriority::Low => "Low", ECRPriority::Medium => "Medium", ECRPriority::High => "High", ECRPriority::Urgent => "Urgent" }
    }

    fn doc_type_to_str(t: &DocumentType) -> &'static str {
        match t { DocumentType::Drawing => "Drawing", DocumentType::Specification => "Specification", DocumentType::Procedure => "Procedure", DocumentType::WorkInstruction => "WorkInstruction", DocumentType::Manual => "Manual", DocumentType::Other => "Other" }
    }
}

#[derive(sqlx::FromRow)]
struct ECRRow {
    id: String,
    ecr_number: String,
    title: String,
    description: String,
    change_type: String,
    reason: String,
    impact_assessment: Option<String>,
    requested_by: String,
    priority: String,
    status: String,
    submitted_at: Option<String>,
    approved_at: Option<String>,
    created_at: String,
}

impl From<ECRRow> for EngineeringChangeRequest {
    fn from(r: ECRRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            ecr_number: r.ecr_number,
            title: r.title,
            description: r.description,
            change_type: match r.change_type.as_str() { "Process" => ChangeType::Process, "Material" => ChangeType::Material, "Documentation" => ChangeType::Documentation, "Other" => ChangeType::Other, _ => ChangeType::Design },
            reason: r.reason,
            impact_assessment: r.impact_assessment,
            requested_by: Uuid::parse_str(&r.requested_by).unwrap_or_default(),
            priority: match r.priority.as_str() { "Low" => ECRPriority::Low, "Medium" => ECRPriority::Medium, "High" => ECRPriority::High, _ => ECRPriority::Urgent },
            status: match r.status.as_str() { "Submitted" => ECRStatus::Submitted, "UnderReview" => ECRStatus::UnderReview, "Approved" => ECRStatus::Approved, "Rejected" => ECRStatus::Rejected, "Implemented" => ECRStatus::Implemented, _ => ECRStatus::Draft },
            submitted_at: r.submitted_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            approved_at: r.approved_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct ECORow {
    id: String,
    eco_number: String,
    ecr_id: String,
    title: String,
    description: Option<String>,
    effective_date: String,
    approved_by: Option<String>,
    approval_date: Option<String>,
    status: String,
    created_at: String,
}

impl From<ECORow> for EngineeringChangeOrder {
    fn from(r: ECORow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            eco_number: r.eco_number,
            ecr_id: Uuid::parse_str(&r.ecr_id).unwrap_or_default(),
            title: r.title,
            description: r.description,
            effective_date: chrono::DateTime::parse_from_rfc3339(&r.effective_date).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            approved_by: r.approved_by.and_then(|s| Uuid::parse_str(&s).ok()),
            approval_date: r.approval_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            status: match r.status.as_str() { "Approved" => ECOStatus::Approved, "Implemented" => ECOStatus::Implemented, "Cancelled" => ECOStatus::Cancelled, _ => ECOStatus::Draft },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct DocumentRevisionRow {
    id: String,
    document_number: String,
    revision: String,
    title: String,
    document_type: String,
    file_path: Option<String>,
    product_id: Option<String>,
    status: String,
    approved_by: Option<String>,
    approved_at: Option<String>,
    created_by: Option<String>,
    created_at: String,
}

impl From<DocumentRevisionRow> for DocumentRevision {
    fn from(r: DocumentRevisionRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            document_number: r.document_number,
            revision: r.revision,
            title: r.title,
            document_type: match r.document_type.as_str() { "Specification" => DocumentType::Specification, "Procedure" => DocumentType::Procedure, "WorkInstruction" => DocumentType::WorkInstruction, "Manual" => DocumentType::Manual, "Other" => DocumentType::Other, _ => DocumentType::Drawing },
            file_path: r.file_path,
            product_id: r.product_id.and_then(|s| Uuid::parse_str(&s).ok()),
            status: match r.status.as_str() { "UnderReview" => DocumentRevisionStatus::UnderReview, "Approved" => DocumentRevisionStatus::Approved, "Superseded" => DocumentRevisionStatus::Superseded, "Obsolete" => DocumentRevisionStatus::Obsolete, _ => DocumentRevisionStatus::Draft },
            approved_by: r.approved_by.and_then(|s| Uuid::parse_str(&s).ok()),
            approved_at: r.approved_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            created_by: r.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }
    }
}
