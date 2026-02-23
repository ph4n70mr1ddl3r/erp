use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SOPCycleStatus {
    Draft,
    DemandReview,
    SupplyReview,
    PreSOPMeeting,
    ExecutiveSOPMeeting,
    Finalized,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOPCycle {
    pub base: BaseEntity,
    pub cycle_number: String,
    pub name: String,
    pub fiscal_year: i32,
    pub planning_horizon_months: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub current_status: SOPCycleStatus,
    pub demand_review_date: Option<NaiveDate>,
    pub supply_review_date: Option<NaiveDate>,
    pub pre_sop_date: Option<NaiveDate>,
    pub executive_sop_date: Option<NaiveDate>,
    pub total_demand: i64,
    pub total_supply: i64,
    pub gap: i64,
    pub currency: String,
    pub owner_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandPlan {
    pub base: BaseEntity,
    pub plan_number: String,
    pub name: String,
    pub sop_cycle_id: Uuid,
    pub plan_type: DemandPlanType,
    pub planning_horizon_months: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub currency: String,
    pub status: PlanStatus,
    pub created_by: Option<Uuid>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DemandPlanType {
    Statistical,
    SalesInput,
    MarketingInput,
    Consensus,
    Final,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PlanStatus {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Finalized,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandPlanLine {
    pub id: Uuid,
    pub demand_plan_id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub region_id: Option<Uuid>,
    pub customer_group_id: Option<Uuid>,
    pub period_type: PeriodType,
    pub periods: Vec<DemandPlanPeriod>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandPlanPeriod {
    pub period_number: i32,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub baseline_forecast: i64,
    pub sales_adjustment: i64,
    pub marketing_adjustment: i64,
    pub promotion_lift: i64,
    pub event_adjustment: i64,
    pub final_forecast: i64,
    pub unit: String,
    pub confidence_level: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PeriodType {
    Weekly,
    Monthly,
    Quarterly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyPlan {
    pub base: BaseEntity,
    pub plan_number: String,
    pub name: String,
    pub sop_cycle_id: Uuid,
    pub planning_horizon_months: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub currency: String,
    pub total_production_capacity: i64,
    pub total_external_supply: i64,
    pub total_available: i64,
    pub status: PlanStatus,
    pub created_by: Option<Uuid>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyPlanLine {
    pub id: Uuid,
    pub supply_plan_id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub period_type: PeriodType,
    pub periods: Vec<SupplyPlanPeriod>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplyPlanPeriod {
    pub period_number: i32,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub opening_inventory: i64,
    pub demand: i64,
    pub production: i64,
    pub purchases: i64,
    pub transfers_in: i64,
    pub transfers_out: i64,
    pub closing_inventory: i64,
    pub days_of_supply: f64,
    pub unit: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SOPScenario {
    pub base: BaseEntity,
    pub scenario_number: String,
    pub name: String,
    pub sop_cycle_id: Uuid,
    pub scenario_type: ScenarioType,
    pub description: Option<String>,
    pub demand_plan_id: Uuid,
    pub supply_plan_id: Uuid,
    pub assumptions: Option<String>,
    pub revenue: i64,
    pub cost: i64,
    pub margin: i64,
    pub service_level_percent: f64,
    pub inventory_days: f64,
    pub is_baseline: bool,
    pub status: ScenarioStatus,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ScenarioType {
    Baseline,
    Optimistic,
    Pessimistic,
    BestCase,
    WorstCase,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ScenarioStatus {
    Draft,
    Simulated,
    Approved,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DRPPlan {
    pub base: BaseEntity,
    pub plan_number: String,
    pub name: String,
    pub planning_horizon_days: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub currency: String,
    pub status: PlanStatus,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DRPPlanLine {
    pub id: Uuid,
    pub drp_plan_id: Uuid,
    pub product_id: Uuid,
    pub source_warehouse_id: Option<Uuid>,
    pub destination_warehouse_id: Uuid,
    pub periods: Vec<DRPPeriod>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DRPPeriod {
    pub period_number: i32,
    pub period_date: NaiveDate,
    pub gross_requirements: i64,
    pub scheduled_receipts: i64,
    pub on_hand: i64,
    pub safety_stock: i64,
    pub net_requirements: i64,
    pub planned_order_receipt: i64,
    pub planned_order_release: i64,
    pub in_transit: i64,
    pub available_to_promise: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionNetwork {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub source_warehouse_id: Uuid,
    pub destination_warehouse_id: Uuid,
    pub lead_time_days: i32,
    pub transportation_mode: TransportationMode,
    pub shipping_cost_per_unit: i64,
    pub min_order_quantity: i64,
    pub max_order_quantity: Option<i64>,
    pub lot_size_multiple: Option<i64>,
    pub priority: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TransportationMode {
    Truck,
    Rail,
    Air,
    Sea,
    Intermodal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningParameter {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub parameter_type: PlanningParameterType,
    pub value: f64,
    pub unit: String,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub source: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PlanningParameterType {
    SafetyStock,
    ReorderPoint,
    EconomicOrderQuantity,
    MinOrderQuantity,
    MaxOrderQuantity,
    LotSizeMultiple,
    LeadTime,
    ServiceLevel,
    ForecastAccuracy,
    CarryingCostPercent,
    OrderingCost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionEvent {
    pub base: BaseEntity,
    pub event_number: String,
    pub name: String,
    pub description: Option<String>,
    pub event_type: PromotionType,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub products: Vec<PromotionProduct>,
    pub status: PromotionStatus,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PromotionType {
    Discount,
    BuyOneGetOne,
    Bundling,
    VolumeDiscount,
    Seasonal,
    Clearance,
    Launch,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PromotionStatus {
    Planned,
    Active,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionProduct {
    pub id: Uuid,
    pub promotion_id: Uuid,
    pub product_id: Uuid,
    pub baseline_units: i64,
    pub lift_percent: f64,
    pub forecasted_units: i64,
    pub cannibalization_products: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastAccuracyMetric {
    pub base: BaseEntity,
    pub metric_date: NaiveDate,
    pub product_id: Option<Uuid>,
    pub product_category_id: Option<Uuid>,
    pub warehouse_id: Option<Uuid>,
    pub forecast_type: String,
    pub horizon_days: i32,
    pub mape: f64,
    pub bias: f64,
    pub mad: f64,
    pub mape_target: f64,
    pub accuracy_percent: f64,
    pub sample_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanningException {
    pub base: BaseEntity,
    pub exception_number: String,
    pub exception_type: PlanningExceptionType,
    pub severity: ExceptionSeverity,
    pub product_id: Option<Uuid>,
    pub warehouse_id: Option<Uuid>,
    pub period_date: NaiveDate,
    pub current_value: f64,
    pub threshold_value: f64,
    pub deviation_percent: f64,
    pub description: String,
    pub recommended_action: Option<String>,
    pub status: ExceptionStatus,
    pub assigned_to: Option<Uuid>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution_notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PlanningExceptionType {
    StockoutRisk,
    ExcessInventory,
    LateOrder,
    DemandSpike,
    CapacityConstraint,
    SupplierDelay,
    QualityHold,
    ForecastVariance,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ExceptionSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ExceptionStatus {
    Open,
    Acknowledged,
    InProgress,
    Resolved,
    Closed,
    Ignored,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityPlanHeader {
    pub base: BaseEntity,
    pub plan_number: String,
    pub name: String,
    pub planning_horizon_weeks: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: PlanStatus,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityPlanLine {
    pub id: Uuid,
    pub capacity_plan_id: Uuid,
    pub work_center_id: Uuid,
    pub periods: Vec<CapacityPeriod>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapacityPeriod {
    pub period_number: i32,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub available_hours: i64,
    pub required_hours: i64,
    pub overload_hours: i64,
    pub utilization_percent: f64,
    pub efficiency_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryOptimization {
    pub base: BaseEntity,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub optimization_date: NaiveDate,
    pub current_safety_stock: i64,
    pub optimal_safety_stock: i64,
    pub current_reorder_point: i64,
    pub optimal_reorder_point: i64,
    pub current_eoq: i64,
    pub optimal_eoq: i64,
    pub service_level_target: f64,
    pub current_service_level: Option<f64>,
    pub carrying_cost: i64,
    pub ordering_cost: i64,
    pub annual_demand: i64,
    pub lead_time_variability: Option<f64>,
    pub demand_variability: Option<f64>,
    pub potential_savings: Option<i64>,
    pub currency: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPIReport {
    pub base: BaseEntity,
    pub report_date: NaiveDate,
    pub report_type: KPIReportType,
    pub forecast_accuracy: f64,
    pub plan_adherence: f64,
    pub inventory_turnover: f64,
    pub days_of_supply: f64,
    pub service_level: f64,
    pub perfect_order_rate: f64,
    pub stockout_incidents: i32,
    pub excess_inventory_value: i64,
    pub obsolete_inventory_value: i64,
    pub supplier_on_time_percent: f64,
    pub production_efficiency: f64,
    pub currency: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum KPIReportType {
    Weekly,
    Monthly,
    Quarterly,
}
