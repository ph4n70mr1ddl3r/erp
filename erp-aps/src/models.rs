use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ScheduleStatus {
    Draft,
    Released,
    Firmed,
    InProgress,
    Completed,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ResourceType {
    Machine,
    Labor,
    Tool,
    WorkCenter,
    ProductionLine,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ScheduleMethod {
    Forward,
    Backward,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningCalendar {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub working_days: String,
    pub shift_pattern: String,
    pub holidays: Option<String>,
    pub capacity_per_day: i64,
    pub effective_date: DateTime<Utc>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningShift {
    pub id: Uuid,
    pub calendar_id: Uuid,
    pub shift_name: String,
    pub start_time: String,
    pub end_time: String,
    pub break_start: Option<String>,
    pub break_end: Option<String>,
    pub capacity_percent: f64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceCapacity {
    pub id: Uuid,
    pub resource_id: Uuid,
    pub resource_type: ResourceType,
    pub resource_name: String,
    pub work_center_id: Option<Uuid>,
    pub calendar_id: Option<Uuid>,
    pub daily_capacity: i64,
    pub unit_of_measure: String,
    pub efficiency_percent: f64,
    pub utilization_percent: f64,
    pub available_from: DateTime<Utc>,
    pub available_to: Option<DateTime<Utc>>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceAvailability {
    pub id: Uuid,
    pub resource_id: Uuid,
    pub date: DateTime<Utc>,
    pub shift_id: Option<Uuid>,
    pub available_capacity: i64,
    pub planned_capacity: i64,
    pub actual_capacity: i64,
    pub downtime_minutes: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MasterProductionSchedule {
    pub id: Uuid,
    pub schedule_number: String,
    pub name: String,
    pub description: Option<String>,
    pub planning_horizon_days: i32,
    pub time_bucket: String,
    pub status: ScheduleStatus,
    pub schedule_method: ScheduleMethod,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub released_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpsItem {
    pub id: Uuid,
    pub mps_id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub planning_start: DateTime<Utc>,
    pub planning_end: DateTime<Utc>,
    pub time_buckets: Vec<MpsTimeBucket>,
    pub total_planned: i64,
    pub total_demand: i64,
    pub total_supply: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpsTimeBucket {
    pub id: Uuid,
    pub mps_item_id: Uuid,
    pub bucket_start: DateTime<Utc>,
    pub bucket_end: DateTime<Utc>,
    pub gross_requirement: i64,
    pub scheduled_receipts: i64,
    pub projected_on_hand: i64,
    pub net_requirement: i64,
    pub planned_order_receipt: i64,
    pub planned_order_release: i64,
    pub available_to_promise: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialRequirementsPlan {
    pub id: Uuid,
    pub mrp_number: String,
    pub mps_id: Option<Uuid>,
    pub planning_date: DateTime<Utc>,
    pub planning_horizon_days: i32,
    pub regenerate: bool,
    pub status: ScheduleStatus,
    pub run_started_at: Option<DateTime<Utc>>,
    pub run_completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MrpItem {
    pub id: Uuid,
    pub mrp_id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub bom_id: Option<Uuid>,
    pub lead_time_days: i32,
    pub safety_stock: i64,
    pub lot_size: i64,
    pub lot_size_method: String,
    pub on_hand: i64,
    pub allocated: i64,
    pub on_order: i64,
    pub net_requirement: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MrpSuggestion {
    pub id: Uuid,
    pub mrp_item_id: Uuid,
    pub suggestion_type: String,
    pub product_id: Uuid,
    pub quantity: i64,
    pub due_date: DateTime<Utc>,
    pub release_date: DateTime<Utc>,
    pub source_type: Option<String>,
    pub source_id: Option<Uuid>,
    pub priority: i32,
    pub status: String,
    pub processed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetailedSchedule {
    pub id: Uuid,
    pub schedule_number: String,
    pub name: String,
    pub schedule_type: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: ScheduleStatus,
    pub optimization_method: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleOperation {
    pub id: Uuid,
    pub schedule_id: Uuid,
    pub work_order_id: Uuid,
    pub routing_operation_id: Option<Uuid>,
    pub resource_id: Uuid,
    pub resource_type: ResourceType,
    pub operation_name: String,
    pub scheduled_start: DateTime<Utc>,
    pub scheduled_end: DateTime<Utc>,
    pub setup_time: i64,
    pub run_time: i64,
    pub quantity: i64,
    pub status: String,
    pub priority: i32,
    pub sequence: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleConstraint {
    pub id: Uuid,
    pub schedule_id: Uuid,
    pub constraint_type: String,
    pub operation_id: Option<Uuid>,
    pub related_operation_id: Option<Uuid>,
    pub offset_minutes: i64,
    pub is_hard: bool,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityPlan {
    pub id: Uuid,
    pub plan_number: String,
    pub name: String,
    pub planning_horizon_days: i32,
    pub bucket_size: String,
    pub status: ScheduleStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityPlanItem {
    pub id: Uuid,
    pub capacity_plan_id: Uuid,
    pub resource_id: Uuid,
    pub bucket_start: DateTime<Utc>,
    pub bucket_end: DateTime<Utc>,
    pub available_capacity: i64,
    pub required_capacity: i64,
    pub overload_capacity: i64,
    pub utilization_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionLine {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub work_center_id: Option<Uuid>,
    pub capacity_per_hour: i64,
    pub efficiency: f64,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionLineStation {
    pub id: Uuid,
    pub production_line_id: Uuid,
    pub station_number: i32,
    pub station_name: String,
    pub work_center_id: Option<Uuid>,
    pub cycle_time: i64,
    pub buffer_capacity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiniteSchedule {
    pub id: Uuid,
    pub schedule_number: String,
    pub name: String,
    pub resource_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub optimization_goal: String,
    pub status: ScheduleStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiniteScheduleBlock {
    pub id: Uuid,
    pub finite_schedule_id: Uuid,
    pub work_order_id: Uuid,
    pub operation_id: Option<Uuid>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub quantity: i64,
    pub setup_time: i64,
    pub run_time: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhatIfScenario {
    pub id: Uuid,
    pub scenario_number: String,
    pub name: String,
    pub description: Option<String>,
    pub base_date: DateTime<Utc>,
    pub changes: String,
    pub results: Option<String>,
    pub comparison_baseline_id: Option<Uuid>,
    pub status: String,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningException {
    pub id: Uuid,
    pub exception_type: String,
    pub severity: String,
    pub product_id: Option<Uuid>,
    pub resource_id: Option<Uuid>,
    pub work_order_id: Option<Uuid>,
    pub message: String,
    pub suggested_action: Option<String>,
    pub is_resolved: bool,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderPriority {
    pub id: Uuid,
    pub order_type: String,
    pub order_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub priority_score: i32,
    pub due_date: DateTime<Utc>,
    pub value: i64,
    pub customer_priority: i32,
    pub strategic_value: i32,
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMaintenanceWindow {
    pub id: Uuid,
    pub resource_id: Uuid,
    pub window_start: DateTime<Utc>,
    pub window_end: DateTime<Utc>,
    pub maintenance_type: String,
    pub description: Option<String>,
    pub capacity_reduction_percent: f64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SequencingRule {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub rule_type: String,
    pub priority_criteria: String,
    pub constraints: String,
    pub is_default: bool,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulePerformance {
    pub id: Uuid,
    pub schedule_id: Uuid,
    pub metric_date: DateTime<Utc>,
    pub on_time_percent: f64,
    pub utilization_percent: f64,
    pub efficiency_percent: f64,
    pub throughput: i64,
    pub wip_value: i64,
    pub tardy_orders: i32,
    pub calculated_at: DateTime<Utc>,
}
