use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MRPStatus {
    Draft,
    Running,
    Completed,
    Error,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MRPActionType {
    Purchase,
    Manufacture,
    Transfer,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DemandType {
    SalesOrder,
    Forecast,
    Manual,
    Dependent,
    SafetyStock,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MRPRun {
    pub base: BaseEntity,
    pub run_number: String,
    pub name: String,
    pub planning_horizon_days: i32,
    pub run_date: NaiveDate,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub include_forecasts: bool,
    include_sales_orders: bool,
    include_work_orders: bool,
    pub safety_stock_method: SafetyStockMethod,
    pub status: MRPStatus,
    pub total_items_planned: i32,
    pub total_suggestions: i32,
    pub error_message: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SafetyStockMethod {
    Fixed,
    DaysOfSupply,
    Dynamic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MRPItemPlan {
    pub id: Uuid,
    pub run_id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub starting_inventory: i64,
    pub safety_stock: i64,
    pub total_demand: i64,
    pub total_supply: i64,
    pub ending_inventory: i64,
    pub shortage_quantity: i64,
    pub suggested_actions: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MRPDemand {
    pub id: Uuid,
    pub run_id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub demand_type: DemandType,
    pub source_type: String,
    pub source_id: Uuid,
    pub source_line_id: Option<Uuid>,
    pub required_date: NaiveDate,
    pub quantity: i64,
    pub allocated_quantity: i64,
    pub remaining_quantity: i64,
    pub priority: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MRPSupply {
    pub id: Uuid,
    pub run_id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub supply_type: SupplyType,
    pub source_type: String,
    pub source_id: Uuid,
    pub available_date: NaiveDate,
    pub quantity: i64,
    pub allocated_quantity: i64,
    pub remaining_quantity: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SupplyType {
    OnHand,
    PurchaseOrder,
    WorkOrder,
    TransferOrder,
    PlannedOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MRPSuggestion {
    pub id: Uuid,
    pub run_id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub action_type: MRPActionType,
    pub quantity: i64,
    pub required_date: NaiveDate,
    pub suggested_date: NaiveDate,
    pub lead_time_days: i32,
    pub priority: i32,
    pub reason: String,
    pub source_demand_ids: Vec<Uuid>,
    pub status: SuggestionStatus,
    pub converted_type: Option<String>,
    pub converted_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SuggestionStatus {
    Suggested,
    Approved,
    Converted,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MRPParameter {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub planning_method: PlanningMethod,
    pub lot_size_method: LotSizeMethod,
    pub fixed_lot_size: i64,
    pub min_lot_size: i64,
    pub max_lot_size: i64,
    pub multiple_lot_size: i64,
    pub safety_stock: i64,
    pub safety_time_days: i32,
    pub lead_time_days: i32,
    pub planning_time_fence_days: i32,
    pub order_policy: OrderPolicy,
    pub min_order_days: i32,
    pub max_order_days: i32,
    pub days_of_supply: i32,
    pub service_level_percent: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PlanningMethod {
    MRP,
    ReorderPoint,
    Kanban,
    TimePhased,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LotSizeMethod {
    Fixed,
    LotForLot,
    PeriodOrderQuantity,
    EconomicOrderQuantity,
    MinMax,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum OrderPolicy {
    Standard,
    Forward,
    Backward,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandForecast {
    pub base: BaseEntity,
    pub forecast_number: String,
    pub name: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub forecast_method: ForecastMethod,
    pub status: Status,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ForecastMethod {
    MovingAverage,
    WeightedMovingAverage,
    ExponentialSmoothing,
    Seasonal,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandForecastLine {
    pub id: Uuid,
    pub forecast_id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub forecast_quantity: i64,
    pub actual_quantity: Option<i64>,
    pub variance: Option<i64>,
    pub confidence_level: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedOrder {
    pub base: BaseEntity,
    pub order_number: String,
    pub run_id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub order_type: MRPActionType,
    pub quantity: i64,
    pub start_date: NaiveDate,
    pub due_date: NaiveDate,
    pub bom_id: Option<Uuid>,
    pub routing_id: Option<Uuid>,
    pub source_demand_ids: String,
    pub status: PlannedOrderStatus,
    pub firmed: bool,
    pub converted_type: Option<String>,
    pub converted_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PlannedOrderStatus {
    Open,
    Firmed,
    Converted,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlannedOrderComponent {
    pub id: Uuid,
    pub planned_order_id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub required_quantity: i64,
    pub issued_quantity: i64,
    pub required_date: NaiveDate,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionRequirement {
    pub id: Uuid,
    pub run_id: Uuid,
    pub product_id: Uuid,
    pub source_warehouse_id: Uuid,
    pub dest_warehouse_id: Uuid,
    pub required_date: NaiveDate,
    pub quantity: i64,
    pub priority: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MRPException {
    pub id: Uuid,
    pub run_id: Uuid,
    pub product_id: Uuid,
    pub exception_type: MRPExceptionType,
    pub severity: ExceptionSeverity,
    pub message: String,
    pub details: Option<String>,
    pub suggested_action: Option<String>,
    pub acknowledged: bool,
    pub acknowledged_by: Option<Uuid>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MRPExceptionType {
    PastDueOrder,
    LeadTimeViolation,
    Shortage,
    ExcessInventory,
    BelowSafetyStock,
    InvalidParameter,
    MissingBOM,
    MissingRouting,
    SupplierLeadTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ExceptionSeverity {
    Info,
    Warning,
    Error,
    Critical,
}
