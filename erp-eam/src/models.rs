use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AssetType {
    Production,
    Facility,
    Fleet,
    IT,
    Office,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AssetCriticality {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquipmentAsset {
    pub base: BaseEntity,
    pub asset_number: String,
    pub name: String,
    pub description: Option<String>,
    pub asset_type: AssetType,
    pub category: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub location_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub parent_asset_id: Option<Uuid>,
    pub installation_date: Option<NaiveDate>,
    pub warranty_end_date: Option<NaiveDate>,
    pub criticality: AssetCriticality,
    pub status: AssetOperationalStatus,
    pub acquisition_cost: i64,
    pub depreciation_method: Option<String>,
    pub useful_life_years: Option<i32>,
    pub current_book_value: i64,
    pub meter_type: Option<MeterType>,
    pub meter_unit: Option<String>,
    pub current_meter_reading: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AssetOperationalStatus {
    Running,
    Idle,
    Down,
    Maintenance,
    Retired,
    Disposed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MeterType {
    HourMeter,
    Odometer,
    CycleCounter,
    Production,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetMeterReading {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub reading_date: NaiveDate,
    pub reading_value: i64,
    pub reading_type: MeterType,
    pub entered_by: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WorkOrderType {
    Corrective,
    Preventive,
    Predictive,
    Inspection,
    Emergency,
    Modification,
    Project,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WorkOrderPriority {
    Emergency,
    High,
    Medium,
    Low,
    Scheduled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WorkOrderStatus {
    Requested,
    Approved,
    Planned,
    Scheduled,
    InProgress,
    OnHold,
    Completed,
    Closed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrder {
    pub base: BaseEntity,
    pub wo_number: String,
    pub description: String,
    pub work_order_type: WorkOrderType,
    pub priority: WorkOrderPriority,
    pub asset_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub failure_code_id: Option<Uuid>,
    pub problem_description: Option<String>,
    pub cause_description: Option<String>,
    pub remedy_description: Option<String>,
    pub requested_by: Option<Uuid>,
    pub requested_date: NaiveDate,
    pub required_date: Option<NaiveDate>,
    pub scheduled_start: Option<DateTime<Utc>>,
    pub scheduled_end: Option<DateTime<Utc>>,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub assigned_to: Option<Uuid>,
    pub assigned_team_id: Option<Uuid>,
    pub status: WorkOrderStatus,
    pub estimated_labor_hours: f64,
    pub actual_labor_hours: f64,
    pub estimated_cost: i64,
    pub actual_cost: i64,
    pub downtime_hours: f64,
    pub completion_notes: Option<String>,
    pub closed_by: Option<Uuid>,
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrderTask {
    pub id: Uuid,
    pub work_order_id: Uuid,
    pub task_number: i32,
    pub description: String,
    pub estimated_hours: f64,
    pub actual_hours: f64,
    pub assigned_to: Option<Uuid>,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrderLabor {
    pub id: Uuid,
    pub work_order_id: Uuid,
    pub employee_id: Uuid,
    pub labor_type: LaborType,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub hours: f64,
    pub hourly_rate: i64,
    pub total_cost: i64,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LaborType {
    Mechanic,
    Electrician,
    Technician,
    Engineer,
    Contractor,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkOrderPart {
    pub id: Uuid,
    pub work_order_id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub quantity_required: i64,
    pub quantity_issued: i64,
    pub unit_cost: i64,
    pub total_cost: i64,
    pub issued_at: Option<DateTime<Utc>>,
    pub issued_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MaintenanceStrategy {
    TimeBased,
    UsageBased,
    ConditionBased,
    Predictive,
    ReliabilityCentered,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum FrequencyType {
    Days,
    Weeks,
    Months,
    Years,
    Hours,
    Cycles,
    Miles,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreventiveMaintenanceSchedule {
    pub base: BaseEntity,
    pub pm_number: String,
    pub name: String,
    pub description: Option<String>,
    pub asset_id: Uuid,
    pub maintenance_strategy: MaintenanceStrategy,
    pub frequency_type: FrequencyType,
    pub frequency_value: i32,
    pub last_performed_date: Option<NaiveDate>,
    pub next_due_date: NaiveDate,
    pub meter_based: bool,
    pub last_meter_reading: Option<i64>,
    pub next_meter_due: Option<i64>,
    pub estimated_duration_hours: f64,
    pub estimated_cost: i64,
    pub auto_generate_wo: bool,
    pub lead_time_days: i32,
    pub checklist_id: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PMTask {
    pub id: Uuid,
    pub pm_schedule_id: Uuid,
    pub task_number: i32,
    pub description: String,
    pub estimated_minutes: i32,
    pub required_skills: Option<String>,
    pub safety_notes: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailureCode {
    pub base: BaseEntity,
    pub code: String,
    pub description: String,
    pub problem_type: String,
    pub cause_type: Option<String>,
    pub remedy_type: Option<String>,
    pub parent_id: Option<Uuid>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetFailureHistory {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub failure_date: NaiveDate,
    pub failure_code_id: Option<Uuid>,
    pub problem_description: String,
    pub cause_description: Option<String>,
    pub remedy_description: Option<String>,
    pub downtime_hours: f64,
    pub repair_cost: i64,
    pub work_order_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceCalendar {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub working_days: String,
    pub shift_start: String,
    pub shift_end: String,
    pub holidays: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceShift {
    pub id: Uuid,
    pub calendar_id: Uuid,
    pub shift_name: String,
    pub start_time: String,
    pub end_time: String,
    pub days_of_week: String,
    pub crew_size: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SparePart {
    pub base: BaseEntity,
    pub part_number: String,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub manufacturer: Option<String>,
    pub unit_of_measure: String,
    pub unit_cost: i64,
    pub min_stock_level: i64,
    pub max_stock_level: i64,
    pub reorder_point: i64,
    pub current_stock: i64,
    pub warehouse_id: Uuid,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetSparePart {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub spare_part_id: Uuid,
    pub quantity_required: i64,
    pub installation_date: Option<NaiveDate>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceBudget {
    pub base: BaseEntity,
    pub name: String,
    pub fiscal_year: i32,
    pub department_id: Option<Uuid>,
    pub total_budget: i64,
    pub labor_budget: i64,
    pub parts_budget: i64,
    pub contractor_budget: i64,
    pub spent_to_date: i64,
    pub committed_amount: i64,
    pub remaining_budget: i64,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceKPI {
    pub id: Uuid,
    pub kpi_type: MaintenanceKPIType,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub asset_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub value: f64,
    pub target: f64,
    pub variance: f64,
    pub trend: Option<String>,
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MaintenanceKPIType {
    MTBF,
    MTTR,
    Availability,
    OEE,
    PMCompliance,
    BacklogHours,
    WorkOrderCompletionRate,
    PlannedVsUnplanned,
    MaintenanceCostPerUnit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceContract {
    pub base: BaseEntity,
    pub contract_number: String,
    pub vendor_id: Uuid,
    pub contract_type: ServiceContractType,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub annual_cost: i64,
    pub response_time_hours: i32,
    pub coverage_type: String,
    pub terms: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ServiceContractType {
    FullService,
    PreventiveOnly,
    BreakFix,
    TimeAndMaterials,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetLocation {
    pub base: BaseEntity,
    pub location_code: String,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub site_id: Option<Uuid>,
    pub building: Option<String>,
    pub floor: Option<String>,
    pub room: Option<String>,
    pub area: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetDownEvent {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub down_start: DateTime<Utc>,
    pub down_end: Option<DateTime<Utc>>,
    pub downtime_hours: Option<f64>,
    pub reason: String,
    pub work_order_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}
