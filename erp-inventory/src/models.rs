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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseZone {
    pub id: Uuid,
    pub warehouse_id: Uuid,
    pub zone_code: String,
    pub name: String,
    pub zone_type: ZoneType,
    pub temperature_controlled: bool,
    pub max_capacity: Option<i64>,
    pub current_utilization: i64,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ZoneType {
    Receiving,
    Storage,
    Picking,
    Shipping,
    Quarantine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WarehouseBin {
    pub id: Uuid,
    pub zone_id: Uuid,
    pub bin_code: String,
    pub bin_type: BinType,
    pub aisle: Option<String>,
    pub row_number: Option<i32>,
    pub level_number: Option<i32>,
    pub capacity: Option<i64>,
    pub current_quantity: i64,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BinType {
    Standard,
    Bulk,
    FlowRack,
    Mezzanine,
    ColdStorage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PickList {
    pub id: Uuid,
    pub pick_number: String,
    pub warehouse_id: Uuid,
    pub order_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub priority: i32,
    pub status: PickListStatus,
    pub total_items: i32,
    pub picked_items: i32,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PickListStatus {
    Pending,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PickListItem {
    pub id: Uuid,
    pub pick_list_id: Uuid,
    pub product_id: Uuid,
    pub bin_id: Uuid,
    pub lot_id: Option<Uuid>,
    pub requested_qty: i64,
    pub picked_qty: i64,
    pub status: PickItemStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PickItemStatus {
    Pending,
    Picked,
    Short,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackList {
    pub id: Uuid,
    pub pack_number: String,
    pub pick_list_id: Uuid,
    pub warehouse_id: Uuid,
    pub packed_by: Option<Uuid>,
    pub status: PackListStatus,
    pub total_weight: Option<i64>,
    pub tracking_number: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PackListStatus {
    Pending,
    InProgress,
    Completed,
    Shipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackListItem {
    pub id: Uuid,
    pub pack_list_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i64,
    pub box_number: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentOrder {
    pub id: Uuid,
    pub shipment_number: String,
    pub warehouse_id: Uuid,
    pub carrier_id: Option<Uuid>,
    pub service_type: Option<String>,
    pub ship_to_name: String,
    pub ship_to_address: String,
    pub ship_to_city: String,
    pub ship_to_state: Option<String>,
    pub ship_to_postal: String,
    pub ship_to_country: String,
    pub total_weight: Option<i64>,
    pub tracking_number: Option<String>,
    pub ship_date: Option<DateTime<Utc>>,
    pub estimated_delivery: Option<DateTime<Utc>>,
    pub actual_delivery: Option<DateTime<Utc>>,
    pub status: ShipmentStatus,
    pub freight_charge: i64,
    pub insurance_charge: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ShipmentStatus {
    Draft,
    Pending,
    Shipped,
    InTransit,
    Delivered,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentItem {
    pub id: Uuid,
    pub shipment_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i64,
    pub weight: Option<i64>,
    pub foreign_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingCarrier {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub api_endpoint: Option<String>,
    pub api_key: Option<String>,
    pub account_number: Option<String>,
    pub supports_tracking: bool,
    pub supports_label_generation: bool,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarrierService {
    pub id: Uuid,
    pub carrier_id: Uuid,
    pub service_code: String,
    pub service_name: String,
    pub delivery_days: Option<i32>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingRateCard {
    pub id: Uuid,
    pub carrier_id: Uuid,
    pub service_id: Uuid,
    pub zone_from: String,
    pub zone_to: String,
    pub weight_from: i64,
    pub weight_to: i64,
    pub base_rate: i64,
    pub per_kg_rate: i64,
    pub effective_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EDIPartner {
    pub id: Uuid,
    pub partner_code: String,
    pub partner_name: String,
    pub partner_type: EDIPartnerType,
    pub edi_standard: EDIStandard,
    pub communication_method: CommunicationMethod,
    pub ftp_host: Option<String>,
    pub ftp_username: Option<String>,
    pub ftp_password: Option<String>,
    pub api_endpoint: Option<String>,
    pub api_key: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EDIPartnerType {
    Customer,
    Supplier,
    Carrier,
    Warehouse,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EDIStandard {
    X12,
    EDIFACT,
    XML,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CommunicationMethod {
    FTP,
    SFTP,
    AS2,
    API,
    Email,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EDIDocument {
    pub id: Uuid,
    pub document_number: String,
    pub partner_id: Uuid,
    pub document_type: EDIDocumentType,
    pub direction: EDIDirection,
    pub reference_number: Option<String>,
    pub raw_content: Option<String>,
    pub parsed_data: Option<String>,
    pub status: EDIDocumentStatus,
    pub processed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EDIDocumentType {
    PO,
    Invoice,
    ASN,
    POChange,
    POAck,
    InvoiceResponse,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EDIDirection {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EDIDocumentStatus {
    Pending,
    Processing,
    Processed,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EDIMapping {
    pub id: Uuid,
    pub partner_id: Uuid,
    pub document_type: String,
    pub segment_id: String,
    pub element_position: i32,
    pub internal_field: String,
    pub transformation_rule: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierUser {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub role: SupplierUserRole,
    pub last_login: Option<DateTime<Utc>>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SupplierUserRole {
    Supplier,
    SupplierAdmin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierInvitation {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub email: String,
    pub invitation_token: String,
    pub expires_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub status: InvitationStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierDocument {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub document_type: SupplierDocumentType,
    pub document_name: String,
    pub file_path: String,
    pub uploaded_by: Option<Uuid>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SupplierDocumentType {
    Certificate,
    Insurance,
    TaxForm,
    Contract,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RFQ {
    pub id: Uuid,
    pub rfq_number: String,
    pub title: String,
    pub description: Option<String>,
    pub buyer_id: Uuid,
    pub currency: String,
    pub submission_deadline: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,
    pub status: RFQStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RFQStatus {
    Draft,
    Published,
    Closed,
    Awarded,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RFQLine {
    pub id: Uuid,
    pub rfq_id: Uuid,
    pub line_number: i32,
    pub product_id: Option<Uuid>,
    pub description: String,
    pub quantity: i64,
    pub unit: String,
    pub delivery_date: Option<DateTime<Utc>>,
    pub specifications: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RFQVendor {
    pub id: Uuid,
    pub rfq_id: Uuid,
    pub vendor_id: Uuid,
    pub invited_at: Option<DateTime<Utc>>,
    pub responded_at: Option<DateTime<Utc>>,
    pub status: RFQVendorStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RFQVendorStatus {
    Invited,
    Responded,
    Declined,
    Awarded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RFQResponse {
    pub id: Uuid,
    pub rfq_id: Uuid,
    pub vendor_id: Uuid,
    pub response_number: String,
    pub response_date: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,
    pub payment_terms: Option<i32>,
    pub delivery_terms: Option<String>,
    pub notes: Option<String>,
    pub status: RFQResponseStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RFQResponseStatus {
    Submitted,
    UnderReview,
    Accepted,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RFQResponseLine {
    pub id: Uuid,
    pub response_id: Uuid,
    pub rfq_line_id: Uuid,
    pub unit_price: i64,
    pub lead_time_days: Option<i32>,
    pub minimum_order_qty: Option<i64>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ValuationMethod {
    FIFO,
    LIFO,
    WeightedAverage,
    StandardCost,
    MovingAverage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductValuation {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub valuation_method: ValuationMethod,
    pub standard_cost: i64,
    pub current_unit_cost: i64,
    pub total_quantity: i64,
    pub total_value: i64,
    pub last_receipt_cost: i64,
    pub last_receipt_date: Option<DateTime<Utc>>,
    pub last_issue_cost: i64,
    pub last_issue_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryCostLayer {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub layer_date: DateTime<Utc>,
    pub receipt_reference: String,
    pub receipt_id: Option<Uuid>,
    pub quantity: i64,
    pub unit_cost: i64,
    pub remaining_quantity: i64,
    pub total_value: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryValuationSummary {
    pub id: Uuid,
    pub valuation_date: DateTime<Utc>,
    pub warehouse_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub total_products: i32,
    pub total_quantity: i64,
    pub total_value: i64,
    pub currency: String,
    pub status: ValuationStatus,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ValuationStatus {
    Draft,
    Posted,
    Reversed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryValuationLine {
    pub id: Uuid,
    pub summary_id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub quantity_on_hand: i64,
    pub unit_cost: i64,
    pub total_value: i64,
    pub valuation_method: ValuationMethod,
    pub previous_value: i64,
    pub value_change: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostAdjustment {
    pub id: Uuid,
    pub adjustment_number: String,
    pub adjustment_type: CostAdjustmentType,
    pub adjustment_date: DateTime<Utc>,
    pub reason: String,
    pub status: CostAdjustmentStatus,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub journal_entry_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CostAdjustmentType {
    StandardCostChange,
    Revaluation,
    WriteDown,
    WriteUp,
    CurrencyRevaluation,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CostAdjustmentStatus {
    Draft,
    Pending,
    Approved,
    Posted,
    Reversed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostAdjustmentLine {
    pub id: Uuid,
    pub adjustment_id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub quantity: i64,
    pub old_unit_cost: i64,
    pub new_unit_cost: i64,
    pub old_total_value: i64,
    pub new_total_value: i64,
    pub value_change: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ABCClassification {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub classification: ABCClass,
    pub annual_value: i64,
    pub annual_quantity: i64,
    pub cumulative_value_percent: f64,
    pub classification_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ABCClass {
    A,
    B,
    C,
    D,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryTurnover {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub beginning_inventory: i64,
    pub ending_inventory: i64,
    pub average_inventory: i64,
    pub cost_of_goods_sold: i64,
    pub turnover_ratio: f64,
    pub days_of_inventory: f64,
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsignmentStock {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub vendor_id: Uuid,
    pub agreement_id: Option<Uuid>,
    pub quantity: i64,
    pub unit_cost: i64,
    pub total_value: i64,
    pub ownership_status: ConsignmentOwnership,
    pub received_date: DateTime<Utc>,
    pub consumption_start_date: Option<DateTime<Utc>>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ConsignmentOwnership {
    VendorOwned,
    Transferred,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsignmentConsumption {
    pub id: Uuid,
    pub consignment_id: Uuid,
    pub consumption_date: DateTime<Utc>,
    pub quantity: i64,
    pub unit_cost: i64,
    pub total_cost: i64,
    pub purchase_order_id: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlanketPurchaseOrder {
    pub base: BaseEntity,
    pub bpo_number: String,
    pub vendor_id: Uuid,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub total_amount_limit: i64,
    pub total_quantity_limit: Option<i64>,
    pub total_released: i64,
    pub total_invoiced: i64,
    pub payment_terms: i32,
    pub terms_conditions: Option<String>,
    pub status: BlanketOrderStatus,
    pub created_by: Option<Uuid>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BlanketOrderStatus {
    Draft,
    Pending,
    Active,
    Expired,
    Closed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlanketPurchaseOrderLine {
    pub id: Uuid,
    pub bpo_id: Uuid,
    pub product_id: Uuid,
    pub unit_price: i64,
    pub currency: String,
    pub min_release_qty: i64,
    pub max_quantity: i64,
    pub released_quantity: i64,
    pub remaining_quantity: i64,
    pub uom: String,
    pub lead_time_days: i32,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseOrder {
    pub base: BaseEntity,
    pub release_number: String,
    pub bpo_id: Uuid,
    pub bpo_line_id: Uuid,
    pub quantity: i64,
    pub unit_price: i64,
    pub total_amount: i64,
    pub required_date: DateTime<Utc>,
    pub ship_to_warehouse_id: Uuid,
    pub status: ReleaseOrderStatus,
    pub purchase_order_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReleaseOrderStatus {
    Draft,
    Submitted,
    Confirmed,
    PartiallyReceived,
    Received,
    Cancelled,
}
