use chrono::{DateTime, Utc};
use erp_core::models::{BaseEntity, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CostObjectType {
    Product,
    Service,
    Customer,
    Channel,
    Process,
    Project,
    Department,
    Location,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ActivityType {
    UnitLevel,
    BatchLevel,
    ProductLevel,
    CustomerLevel,
    FacilityLevel,
    OrganizationLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CostDriverType {
    Transaction,
    Duration,
    Intensity,
    DirectCharge,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostPool {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub pool_type: CostPoolType,
    pub total_cost: Money,
    pub currency: String,
    pub fiscal_year: i32,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CostPoolType {
    DirectLabor,
    DirectMaterial,
    Manufacturing,
    Administrative,
    Sales,
    Distribution,
    CustomerService,
    Research,
    IT,
    HumanResources,
    Facilities,
    Quality,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Activity {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub activity_type: ActivityType,
    pub cost_pool_id: Uuid,
    pub total_cost: Money,
    pub cost_driver_id: Option<Uuid>,
    pub driver_quantity: f64,
    pub cost_per_driver: Money,
    pub department_id: Option<Uuid>,
    pub process_id: Option<Uuid>,
    pub is_value_added: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostDriver {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub driver_type: CostDriverType,
    pub unit_of_measure: String,
    pub total_capacity: f64,
    pub used_capacity: f64,
    pub unused_capacity: f64,
    pub utilization_percent: f64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostObject {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub object_type: CostObjectType,
    pub parent_id: Option<Uuid>,
    pub direct_cost: Money,
    pub indirect_cost: Money,
    pub total_cost: Money,
    pub revenue: Money,
    pub profit_margin: Money,
    pub profit_margin_percent: f64,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityAllocation {
    pub base: BaseEntity,
    pub activity_id: Uuid,
    pub cost_object_id: Uuid,
    pub cost_pool_id: Uuid,
    pub driver_quantity: f64,
    pub allocation_rate: Money,
    pub allocated_amount: Money,
    pub allocation_date: DateTime<Utc>,
    pub fiscal_period: String,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceConsumption {
    pub base: BaseEntity,
    pub resource_id: Uuid,
    pub resource_name: String,
    pub activity_id: Uuid,
    pub consumption_date: DateTime<Utc>,
    pub quantity: f64,
    pub unit_cost: Money,
    pub total_cost: Money,
    pub source_module: String,
    pub source_reference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Process {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub owner_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub total_activities: i32,
    pub total_cost: Money,
    pub cycle_time_hours: f64,
    pub is_core_process: bool,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessStep {
    pub base: BaseEntity,
    pub process_id: Uuid,
    pub step_number: i32,
    pub name: String,
    pub description: Option<String>,
    pub activity_id: Uuid,
    pub estimated_duration_hours: f64,
    pub actual_duration_hours: Option<f64>,
    pub cost: Money,
    pub is_bottleneck: bool,
    pub next_step_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostSimulation {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub simulation_type: SimulationType,
    pub base_scenario_id: Option<Uuid>,
    pub status: SimulationStatus,
    pub created_by: Uuid,
    pub results: serde_json::Value,
    pub variance_analysis: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SimulationType {
    WhatIf,
    Sensitivity,
    Scenario,
    Optimization,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SimulationStatus {
    Draft,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostRate {
    pub base: BaseEntity,
    pub activity_id: Uuid,
    pub cost_driver_id: Uuid,
    pub fiscal_period: String,
    pub rate_per_unit: Money,
    pub fixed_component: Money,
    pub variable_component: Money,
    pub is_blended: bool,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostAnalysis {
    pub base: BaseEntity,
    pub name: String,
    pub analysis_type: AnalysisType,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_direct_costs: Money,
    pub total_indirect_costs: Money,
    pub total_costs: Money,
    pub cost_breakdown: serde_json::Value,
    pub insights: Vec<CostInsight>,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AnalysisType {
    ProductProfitability,
    CustomerProfitability,
    ChannelProfitability,
    ProcessEfficiency,
    CostReduction,
    CapacityUtilization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostInsight {
    pub insight_type: String,
    pub description: String,
    pub impact_amount: Money,
    pub recommendation: String,
    pub priority: InsightPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum InsightPriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillOfActivities {
    pub base: BaseEntity,
    pub cost_object_id: Uuid,
    pub name: String,
    pub version: i32,
    pub activities: Vec<BillOfActivityLine>,
    pub total_cost: Money,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillOfActivityLine {
    pub activity_id: Uuid,
    pub activity_name: String,
    pub driver_quantity: f64,
    pub unit_cost: Money,
    pub total_cost: Money,
    pub sequence: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityPlan {
    pub base: BaseEntity,
    pub cost_driver_id: Uuid,
    pub fiscal_period: String,
    pub planned_capacity: f64,
    pub available_capacity: f64,
    pub committed_capacity: f64,
    pub remaining_capacity: f64,
    pub utilization_target: f64,
}
