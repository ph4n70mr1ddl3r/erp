use chrono::{DateTime, Utc};
use erp_core::{Address, BaseEntity, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub base: BaseEntity,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub product_type: ProductType,
    pub category_id: Option<Uuid>,
    pub unit_of_measure: String,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ProductType {
    Goods,
    Service,
    Digital,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCategory {
    pub base: BaseEntity,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warehouse {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub address: Address,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockLocation {
    pub base: BaseEntity,
    pub warehouse_id: Uuid,
    pub code: String,
    pub name: String,
    pub location_type: LocationType,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LocationType {
    Receiving,
    Storage,
    Picking,
    Shipping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockLevel {
    pub id: Uuid,
    pub product_id: Uuid,
    pub location_id: Uuid,
    pub quantity: i64,
    pub reserved_quantity: i64,
    pub available_quantity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockMovement {
    pub base: BaseEntity,
    pub movement_number: String,
    pub movement_type: MovementType,
    pub product_id: Uuid,
    pub from_location_id: Option<Uuid>,
    pub to_location_id: Uuid,
    pub quantity: i64,
    pub reference: Option<String>,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MovementType {
    Receipt,
    Issue,
    Transfer,
    Adjustment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceList {
    pub base: BaseEntity,
    pub name: String,
    pub currency: String,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceListItem {
    pub id: Uuid,
    pub price_list_id: Uuid,
    pub product_id: Uuid,
    pub price: Money,
    pub min_quantity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lot {
    pub id: Uuid,
    pub lot_number: String,
    pub product_id: Uuid,
    pub serial_number: Option<String>,
    pub manufacture_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub quantity: i64,
    pub status: LotStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LotStatus {
    Active,
    Expired,
    Quarantined,
    Depleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LotTransaction {
    pub id: Uuid,
    pub lot_id: Uuid,
    pub transaction_type: LotTransactionType,
    pub quantity: i64,
    pub reference_type: Option<String>,
    pub reference_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LotTransactionType {
    Receipt,
    Issue,
    Transfer,
    Adjustment,
    Expiry,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QualityInspection {
    pub id: Uuid,
    pub inspection_number: String,
    pub inspection_type: InspectionType,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub inspector_id: Option<Uuid>,
    pub inspection_date: DateTime<Utc>,
    pub status: InspectionStatus,
    pub result: Option<InspectionResult>,
    pub notes: Option<String>,
    pub items: Vec<InspectionItem>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum InspectionType {
    Incoming,
    InProcess,
    Final,
    Outgoing,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum InspectionStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum InspectionResult {
    Pass,
    Fail,
    Conditional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionItem {
    pub id: Uuid,
    pub inspection_id: Uuid,
    pub criterion: String,
    pub expected_value: Option<String>,
    pub actual_value: Option<String>,
    pub pass_fail: Option<PassFail>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PassFail {
    Pass,
    Fail,
    NotApplicable,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NonConformanceReport {
    pub id: Uuid,
    pub ncr_number: String,
    pub source_type: String,
    pub source_id: Uuid,
    pub description: String,
    pub severity: NCRSeverity,
    pub status: NCRStatus,
    pub assigned_to: Option<Uuid>,
    pub root_cause: Option<String>,
    pub corrective_action: Option<String>,
    pub preventive_action: Option<String>,
    pub resolution_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum NCRSeverity {
    Minor,
    Major,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum NCRStatus {
    Open,
    InProgress,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandForecast {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub forecast_quantity: i64,
    pub confidence_level: i32,
    pub method: ForecastMethod,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ForecastMethod {
    MovingAverage,
    WeightedAverage,
    ExponentialSmoothing,
    Seasonal,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyStock {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub safety_stock: i64,
    pub reorder_point: i64,
    pub reorder_quantity: i64,
    pub lead_time_days: i32,
    pub service_level: i32,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplenishmentOrder {
    pub id: Uuid,
    pub order_number: String,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub order_type: ReplenishmentType,
    pub quantity: i64,
    pub status: ReplenishmentStatus,
    pub source: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReplenishmentType {
    Purchase,
    Transfer,
    Production,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReplenishmentStatus {
    Draft,
    Submitted,
    Completed,
    Cancelled,
}
