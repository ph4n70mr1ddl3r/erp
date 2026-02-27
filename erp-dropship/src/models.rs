use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DropShipOrderStatus {
    Pending,
    SentToVendor,
    Confirmed,
    PartiallyShipped,
    Shipped,
    Delivered,
    Cancelled,
    OnHold,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DropShipPOStatus {
    Draft,
    Submitted,
    Acknowledged,
    PartiallyShipped,
    Received,
    Invoiced,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ShipmentStatus {
    Pending,
    LabelCreated,
    PickedUp,
    InTransit,
    OutForDelivery,
    Delivered,
    FailedDelivery,
    Returned,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum VendorDropShipTier {
    Standard,
    Preferred,
    Premium,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorDropShipSettings {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub enabled: bool,
    pub tier: VendorDropShipTier,
    pub auto_confirm: bool,
    pub require_approval: bool,
    pub min_order_value: i64,
    pub max_order_value: Option<i64>,
    pub processing_time_days: i32,
    pub shipping_carrier: Option<String>,
    pub shipping_method: Option<String>,
    pub free_shipping_threshold: Option<i64>,
    pub handling_fee: i64,
    pub allow_partial_shipment: bool,
    pub return_policy_days: i32,
    pub notification_email: Option<String>,
    pub api_endpoint: Option<String>,
    pub api_key: Option<String>,
    pub sync_inventory: bool,
    pub inventory_sync_hours: i32,
    pub product_feed_url: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropShipOrder {
    pub base: BaseEntity,
    pub order_number: String,
    pub sales_order_id: Uuid,
    pub customer_id: Uuid,
    pub vendor_id: Uuid,
    pub purchase_order_id: Option<Uuid>,
    pub ship_to_name: String,
    pub ship_to_company: Option<String>,
    pub ship_to_address: String,
    pub ship_to_city: String,
    pub ship_to_state: Option<String>,
    pub ship_to_postal_code: String,
    pub ship_to_country: String,
    pub ship_to_phone: Option<String>,
    pub ship_to_email: Option<String>,
    pub lines: Vec<DropShipOrderLine>,
    pub subtotal: i64,
    pub shipping_cost: i64,
    pub tax_amount: i64,
    pub total: i64,
    pub currency: String,
    pub status: DropShipOrderStatus,
    pub vendor_confirmation_number: Option<String>,
    pub expected_ship_date: Option<DateTime<Utc>>,
    pub actual_ship_date: Option<DateTime<Utc>>,
    pub expected_delivery_date: Option<DateTime<Utc>>,
    pub actual_delivery_date: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub internal_notes: Option<String>,
    pub priority: i32,
    pub created_by: Option<Uuid>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub sent_to_vendor_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropShipOrderLine {
    pub id: Uuid,
    pub drop_ship_order_id: Uuid,
    pub sales_order_line_id: Option<Uuid>,
    pub product_id: Uuid,
    pub vendor_sku: Option<String>,
    pub description: String,
    pub quantity: i64,
    pub quantity_shipped: i64,
    pub quantity_cancelled: i64,
    pub unit_price: i64,
    pub line_total: i64,
    pub tax_amount: i64,
    pub status: DropShipOrderStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropShipPurchaseOrder {
    pub base: BaseEntity,
    pub po_number: String,
    pub vendor_id: Uuid,
    pub drop_ship_order_id: Uuid,
    pub customer_id: Uuid,
    pub ship_to_name: String,
    pub ship_to_address: String,
    pub ship_to_city: String,
    pub ship_to_state: Option<String>,
    pub ship_to_postal_code: String,
    pub ship_to_country: String,
    pub lines: Vec<DropShipPOLine>,
    pub subtotal: i64,
    pub shipping_cost: i64,
    pub tax_amount: i64,
    pub total: i64,
    pub currency: String,
    pub status: DropShipPOStatus,
    pub vendor_acknowledgment: Option<String>,
    pub vendor_acknowledged_at: Option<DateTime<Utc>>,
    pub expected_delivery: Option<DateTime<Utc>>,
    pub terms: Option<String>,
    pub notes: Option<String>,
    pub created_by: Option<Uuid>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropShipPOLine {
    pub id: Uuid,
    pub po_id: Uuid,
    pub product_id: Uuid,
    pub vendor_sku: Option<String>,
    pub description: String,
    pub quantity: i64,
    pub quantity_received: i64,
    pub unit_cost: i64,
    pub line_total: i64,
    pub status: DropShipPOStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropShipShipment {
    pub id: Uuid,
    pub shipment_number: String,
    pub drop_ship_order_id: Uuid,
    pub vendor_id: Uuid,
    pub carrier: String,
    pub carrier_service: Option<String>,
    pub tracking_number: Option<String>,
    pub tracking_url: Option<String>,
    pub shipping_label_url: Option<String>,
    pub status: ShipmentStatus,
    pub ship_date: Option<DateTime<Utc>>,
    pub estimated_delivery: Option<DateTime<Utc>>,
    pub actual_delivery: Option<DateTime<Utc>>,
    pub weight: Option<f64>,
    pub weight_unit: Option<String>,
    pub dimensions: Option<String>,
    pub shipped_from_address: Option<String>,
    pub shipped_from_city: Option<String>,
    pub shipped_from_state: Option<String>,
    pub shipped_from_postal: Option<String>,
    pub shipped_from_country: Option<String>,
    pub signature_required: bool,
    pub signed_by: Option<String>,
    pub signed_at: Option<DateTime<Utc>>,
    pub delivery_notes: Option<String>,
    pub lines: Vec<DropShipShipmentLine>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropShipShipmentLine {
    pub id: Uuid,
    pub shipment_id: Uuid,
    pub drop_ship_order_line_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i64,
    pub condition: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropShipTrackingEvent {
    pub id: Uuid,
    pub shipment_id: Uuid,
    pub event_timestamp: DateTime<Utc>,
    pub status_code: String,
    pub status_description: String,
    pub location_city: Option<String>,
    pub location_state: Option<String>,
    pub location_country: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropShipInventoryFeed {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub feed_url: Option<String>,
    pub feed_format: FeedFormat,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub last_sync_status: Option<String>,
    pub products_count: i32,
    pub in_stock_count: i32,
    pub out_of_stock_count: i32,
    pub sync_enabled: bool,
    pub sync_interval_hours: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum FeedFormat {
    CSV,
    XML,
    JSON,
    API,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropShipInventoryItem {
    pub id: Uuid,
    pub feed_id: Uuid,
    pub vendor_id: Uuid,
    pub product_id: Uuid,
    pub vendor_sku: String,
    pub quantity_available: i64,
    pub quantity_reserved: i64,
    pub reorder_point: Option<i64>,
    pub expected_restock_date: Option<DateTime<Utc>>,
    pub cost: i64,
    pub msrp: Option<i64>,
    pub map_price: Option<i64>,
    pub weight: Option<f64>,
    pub status: Status,
    pub last_updated: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropShipInvoice {
    pub id: Uuid,
    pub invoice_number: String,
    pub vendor_id: Uuid,
    pub drop_ship_order_id: Uuid,
    pub purchase_order_id: Option<Uuid>,
    pub invoice_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub subtotal: i64,
    pub shipping_cost: i64,
    pub tax_amount: i64,
    pub total: i64,
    pub currency: String,
    pub status: DropShipInvoiceStatus,
    pub vendor_invoice_number: Option<String>,
    pub lines: Vec<DropShipInvoiceLine>,
    pub paid_amount: i64,
    pub paid_at: Option<DateTime<Utc>>,
    pub payment_reference: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DropShipInvoiceStatus {
    Draft,
    Received,
    Approved,
    Paid,
    PartiallyPaid,
    Disputed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropShipInvoiceLine {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
    pub unit_cost: i64,
    pub line_total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropShipReturn {
    pub id: Uuid,
    pub return_number: String,
    pub drop_ship_order_id: Uuid,
    pub customer_id: Uuid,
    pub vendor_id: Uuid,
    pub return_type: DropShipReturnType,
    pub reason: DropShipReturnReason,
    pub request_date: DateTime<Utc>,
    pub authorized_date: Option<DateTime<Utc>>,
    pub received_date: Option<DateTime<Utc>>,
    pub processed_date: Option<DateTime<Utc>>,
    pub lines: Vec<DropShipReturnLine>,
    pub total_amount: i64,
    pub currency: String,
    pub status: DropShipReturnStatus,
    pub vendor_rma_number: Option<String>,
    pub refund_method: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DropShipReturnType {
    Defective,
    WrongItem,
    Damaged,
    NotAsDescribed,
    ChangedMind,
    Warranty,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DropShipReturnReason {
    ProductDefect,
    ShippingDamage,
    WrongProductSent,
    CustomerError,
    QualityIssue,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DropShipReturnStatus {
    Requested,
    Authorized,
    Received,
    Inspected,
    Refunded,
    Replaced,
    Rejected,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropShipReturnLine {
    pub id: Uuid,
    pub return_id: Uuid,
    pub drop_ship_order_line_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i64,
    pub unit_price: i64,
    pub condition: Option<String>,
    pub disposition: Option<String>,
    pub refund_amount: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropShipPerformance {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_orders: i32,
    pub orders_on_time: i32,
    pub orders_late: i32,
    pub orders_cancelled: i32,
    pub total_items: i32,
    pub items_shipped: i32,
    pub items_backordered: i32,
    pub avg_processing_time_hours: f64,
    pub avg_shipping_time_hours: f64,
    pub on_time_delivery_rate: f64,
    pub fill_rate: f64,
    pub cancellation_rate: f64,
    pub return_rate: f64,
    pub customer_complaints: i32,
    pub quality_score: f64,
    pub overall_score: f64,
    pub tier: VendorDropShipTier,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropShipProductMapping {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub product_id: Uuid,
    pub vendor_sku: String,
    pub vendor_product_name: Option<String>,
    pub vendor_cost: i64,
    pub vendor_msrp: Option<i64>,
    pub vendor_map: Option<i64>,
    pub min_order_qty: i64,
    pub lead_time_days: i32,
    pub enabled: bool,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
