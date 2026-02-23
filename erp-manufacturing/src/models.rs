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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MRPRun {
    pub base: BaseEntity,
    pub run_number: String,
    pub run_date: DateTime<Utc>,
    pub planning_horizon_days: i32,
    pub status: MRPRunStatus,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MRPRunStatus {
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MRPPlannedOrder {
    pub id: Uuid,
    pub mrp_run_id: Uuid,
    pub product_id: Uuid,
    pub order_type: MRPOrderType,
    pub quantity: i64,
    pub due_date: DateTime<Utc>,
    pub release_date: Option<DateTime<Utc>>,
    pub source_type: Option<String>,
    pub source_id: Option<Uuid>,
    pub status: MRPPlannedOrderStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MRPOrderType {
    Production,
    Purchase,
    Transfer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MRPPlannedOrderStatus {
    Planned,
    Released,
    Firmed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityPlan {
    pub id: Uuid,
    pub plan_number: String,
    pub work_center_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub available_hours: i64,
    pub planned_hours: i64,
    pub actual_hours: i64,
    pub utilization_percent: f64,
    pub status: CapacityPlanStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CapacityPlanStatus {
    Planned,
    Active,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityRequirement {
    pub id: Uuid,
    pub capacity_plan_id: Uuid,
    pub work_order_id: Uuid,
    pub operation_sequence: i32,
    pub required_hours: i64,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub status: CapacityRequirementStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CapacityRequirementStatus {
    Planned,
    Scheduled,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopFloorOperation {
    pub id: Uuid,
    pub operation_number: String,
    pub work_order_id: Uuid,
    pub work_center_id: Uuid,
    pub operation_code: String,
    pub description: Option<String>,
    pub setup_time: i64,
    pub run_time: i64,
    pub quantity: i64,
    pub completed_qty: i64,
    pub scrapped_qty: i64,
    pub status: ShopFloorOperationStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub operator_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ShopFloorOperationStatus {
    Pending,
    InProgress,
    Completed,
    OnHold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShopFloorLog {
    pub id: Uuid,
    pub operation_id: Uuid,
    pub log_type: ShopFloorLogType,
    pub operator_id: Option<Uuid>,
    pub quantity: Option<i64>,
    pub reason: Option<String>,
    pub notes: Option<String>,
    pub logged_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ShopFloorLogType {
    Start,
    Complete,
    Scrap,
    Rework,
    Hold,
    Resume,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DowntimeEvent {
    pub id: Uuid,
    pub work_center_id: Uuid,
    pub event_type: DowntimeEventType,
    pub reason_code: Option<String>,
    pub description: Option<String>,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DowntimeEventType {
    Planned,
    Unplanned,
    Maintenance,
    Changeover,
    Break,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    pub id: Uuid,
    pub equipment_code: String,
    pub name: String,
    pub description: Option<String>,
    pub equipment_type: String,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub installation_date: Option<DateTime<Utc>>,
    pub warranty_expiry: Option<DateTime<Utc>>,
    pub location: Option<String>,
    pub work_center_id: Option<Uuid>,
    pub parent_equipment_id: Option<Uuid>,
    pub status: EquipmentStatus,
    pub criticality: EquipmentCriticality,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EquipmentStatus {
    Active,
    Inactive,
    Maintenance,
    Retired,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EquipmentCriticality {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceSchedule {
    pub id: Uuid,
    pub schedule_number: String,
    pub equipment_id: Uuid,
    pub maintenance_type: MaintenanceType,
    pub frequency_type: FrequencyType,
    pub frequency_value: i32,
    pub last_maintenance: Option<DateTime<Utc>>,
    pub next_maintenance: DateTime<Utc>,
    pub estimated_duration: Option<i32>,
    pub assigned_to: Option<Uuid>,
    pub status: MaintenanceScheduleStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MaintenanceType {
    Preventive,
    Corrective,
    Predictive,
    Inspection,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum FrequencyType {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
    RuntimeHours,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MaintenanceScheduleStatus {
    Active,
    Inactive,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceWorkOrder {
    pub id: Uuid,
    pub work_order_number: String,
    pub equipment_id: Uuid,
    pub schedule_id: Option<Uuid>,
    pub maintenance_type: MaintenanceType,
    pub priority: MaintenancePriority,
    pub description: String,
    pub requested_by: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub scheduled_date: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub downtime_hours: f64,
    pub labor_hours: f64,
    pub parts_cost: i64,
    pub labor_cost: i64,
    pub status: MaintenanceWorkOrderStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MaintenancePriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MaintenanceWorkOrderStatus {
    Requested,
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenancePart {
    pub id: Uuid,
    pub work_order_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i64,
    pub unit_cost: Option<i64>,
    pub total_cost: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineeringChangeRequest {
    pub id: Uuid,
    pub ecr_number: String,
    pub title: String,
    pub description: String,
    pub change_type: ChangeType,
    pub reason: String,
    pub impact_assessment: Option<String>,
    pub requested_by: Uuid,
    pub priority: ECRPriority,
    pub status: ECRStatus,
    pub submitted_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ChangeType {
    Design,
    Process,
    Material,
    Documentation,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ECRPriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ECRStatus {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Rejected,
    Implemented,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ECRItem {
    pub id: Uuid,
    pub ecr_id: Uuid,
    pub item_type: ECRItemType,
    pub item_id: Uuid,
    pub current_state: Option<String>,
    pub proposed_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ECRItemType {
    Product,
    BOM,
    Routing,
    Document,
    Specification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineeringChangeOrder {
    pub id: Uuid,
    pub eco_number: String,
    pub ecr_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub effective_date: DateTime<Utc>,
    pub approved_by: Option<Uuid>,
    pub approval_date: Option<DateTime<Utc>>,
    pub status: ECOStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ECOStatus {
    Draft,
    Approved,
    Implemented,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocumentRevision {
    pub id: Uuid,
    pub document_number: String,
    pub revision: String,
    pub title: String,
    pub document_type: DocumentType,
    pub file_path: Option<String>,
    pub product_id: Option<Uuid>,
    pub status: DocumentRevisionStatus,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DocumentType {
    Drawing,
    Specification,
    Procedure,
    WorkInstruction,
    Manual,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DocumentRevisionStatus {
    Draft,
    UnderReview,
    Approved,
    Superseded,
    Obsolete,
}
