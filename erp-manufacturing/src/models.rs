use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillOfMaterial {
    pub base: BaseEntity,
    pub product_id: Uuid,
    pub name: String,
    pub version: String,
    pub quantity: i64,
    pub components: Vec<BomComponent>,
    pub operations: Vec<BomOperation>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomComponent {
    pub id: Uuid,
    pub product_id: Uuid,
    pub quantity: i64,
    pub unit: String,
    pub scrap_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BomOperation {
    pub id: Uuid,
    pub sequence: u32,
    pub name: String,
    pub work_center_id: Uuid,
    pub setup_time: i64,
    pub run_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCenter {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub capacity: i64,
    pub cost_per_hour: Money,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrder {
    pub base: BaseEntity,
    pub order_number: String,
    pub product_id: Uuid,
    pub bom_id: Uuid,
    pub quantity: i64,
    pub planned_start: DateTime<Utc>,
    pub planned_end: DateTime<Utc>,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionOrder {
    pub base: BaseEntity,
    pub order_number: String,
    pub work_orders: Vec<Uuid>,
    pub planned_quantity: i64,
    pub produced_quantity: i64,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Routing {
    pub base: BaseEntity,
    pub name: String,
    pub product_id: Uuid,
    pub operations: Vec<RoutingOperation>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoutingOperation {
    pub id: Uuid,
    pub sequence: u32,
    pub work_center_id: Uuid,
    pub operation: String,
    pub setup_time: i64,
    pub run_time: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkCenterResource {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub capacity: i64,
    pub efficiency: i32,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionSchedule {
    pub id: Uuid,
    pub schedule_number: String,
    pub work_order_id: Uuid,
    pub work_center_id: Uuid,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: ScheduleStatus,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ScheduleStatus {
    Planned,
    InProgress,
    Completed,
    Cancelled,
}
