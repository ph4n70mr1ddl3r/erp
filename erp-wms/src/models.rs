use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageLocation {
    pub id: Uuid,
    pub warehouse_id: Uuid,
    pub zone: String,
    pub aisle: String,
    pub rack: String,
    pub shelf: String,
    pub bin: String,
    pub location_type: LocationType,
    pub capacity: i64,
    pub occupied: i64,
    pub status: LocationStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationType {
    Receiving,
    Bulk,
    Pick,
    Packing,
    Shipping,
    CrossDock,
    Quarantine,
    Returns,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LocationStatus {
    Active,
    Inactive,
    Blocked,
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PutAwayTask {
    pub id: Uuid,
    pub receipt_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i64,
    pub source_location: Option<Uuid>,
    pub suggested_location: Option<Uuid>,
    pub actual_location: Option<Uuid>,
    pub status: PutAwayStatus,
    pub priority: i32,
    pub assigned_to: Option<Uuid>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PutAwayStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PickTask {
    pub id: Uuid,
    pub wave_id: Option<Uuid>,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub location_id: Uuid,
    pub quantity: i64,
    pub picked_quantity: i64,
    pub status: PickStatus,
    pub pick_type: PickType,
    pub priority: i32,
    pub assigned_to: Option<Uuid>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PickStatus {
    Released,
    InProgress,
    Picked,
    Short,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PickType {
    Single,
    Batch,
    Zone,
    Wave,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wave {
    pub id: Uuid,
    pub wave_number: String,
    pub warehouse_id: Uuid,
    pub status: WaveStatus,
    pub planned_date: NaiveDate,
    pub released_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub total_picks: i32,
    pub completed_picks: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WaveStatus {
    Planning,
    Released,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossDockOrder {
    pub id: Uuid,
    pub inbound_shipment_id: Uuid,
    pub outbound_order_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i64,
    pub status: CrossDockStatus,
    pub dock_location: Option<Uuid>,
    pub processed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CrossDockStatus {
    Planned,
    InTransit,
    Arrived,
    Processing,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceivingReceipt {
    pub id: Uuid,
    pub receipt_number: String,
    pub warehouse_id: Uuid,
    pub po_id: Option<Uuid>,
    pub carrier: Option<String>,
    pub tracking_number: Option<String>,
    pub status: ReceivingStatus,
    pub received_by: Option<Uuid>,
    pub received_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReceivingStatus {
    Expected,
    InReceiving,
    Received,
    PutAway,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReceiptLine {
    pub id: Uuid,
    pub receipt_id: Uuid,
    pub product_id: Uuid,
    pub expected_qty: i64,
    pub received_qty: i64,
    pub damaged_qty: i64,
    pub lot_number: Option<String>,
    pub expiry_date: Option<NaiveDate>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingManifest {
    pub id: Uuid,
    pub manifest_number: String,
    pub warehouse_id: Uuid,
    pub carrier: String,
    pub service_level: String,
    pub status: ManifestStatus,
    pub total_packages: i32,
    pub total_weight: f64,
    pub shipped_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ManifestStatus {
    Open,
    Closed,
    Shipped,
    InTransit,
    Delivered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CycleCount {
    pub id: Uuid,
    pub count_number: String,
    pub warehouse_id: Uuid,
    pub count_type: CountType,
    pub status: CountStatus,
    pub scheduled_date: NaiveDate,
    pub completed_date: Option<NaiveDate>,
    pub variance_count: i32,
    pub accuracy_rate: Option<f64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CountType {
    ABC,
    Random,
    Location,
    Product,
    Annual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CountStatus {
    Scheduled,
    InProgress,
    Completed,
    Adjusted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CountLine {
    pub id: Uuid,
    pub cycle_count_id: Uuid,
    pub location_id: Uuid,
    pub product_id: Uuid,
    pub system_qty: i64,
    pub counted_qty: i64,
    pub variance: i64,
    pub counted_by: Option<Uuid>,
    pub recounted: bool,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseZone {
    pub id: Uuid,
    pub warehouse_id: Uuid,
    pub zone_code: String,
    pub zone_name: String,
    pub zone_type: ZoneType,
    pub temperature_controlled: bool,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ZoneType {
    Receiving,
    BulkStorage,
    PickArea,
    Packing,
    Shipping,
    CrossDock,
    ColdStorage,
    Hazmat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLocationRequest {
    pub warehouse_id: Uuid,
    pub zone: String,
    pub aisle: String,
    pub rack: String,
    pub shelf: String,
    pub bin: String,
    pub location_type: LocationType,
    pub capacity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWaveRequest {
    pub warehouse_id: Uuid,
    pub planned_date: NaiveDate,
    pub order_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePickTaskRequest {
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i64,
    pub pick_type: PickType,
    pub priority: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCycleCountRequest {
    pub warehouse_id: Uuid,
    pub count_type: CountType,
    pub scheduled_date: NaiveDate,
    pub location_ids: Option<Vec<Uuid>>,
    pub product_ids: Option<Vec<Uuid>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizeWaveRequest {
    pub warehouse_id: Uuid,
    pub order_ids: Vec<Uuid>,
    pub strategy: WaveStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WaveStrategy {
    MinimizeTravel,
    MaximizeThroughput,
    PrioritizeShipDate,
    BalanceWorkload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaveOptimizationResult {
    pub waves: Vec<WavePlan>,
    pub estimated_picks: i32,
    pub estimated_travel_time: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WavePlan {
    pub wave_number: i32,
    pub order_ids: Vec<Uuid>,
    pub zone_assignments: Vec<ZoneAssignment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZoneAssignment {
    pub zone: String,
    pub pick_count: i32,
    pub estimated_time: i32,
}
