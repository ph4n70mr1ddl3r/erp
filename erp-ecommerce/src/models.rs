use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PlatformType {
    Shopify,
    WooCommerce,
    Magento,
    BigCommerce,
    Amazon,
    EBay,
    Etsy,
    Walmart,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SyncStatus {
    Pending,
    Syncing,
    Synced,
    Failed,
    Conflict,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SyncDirection {
    Import,
    Export,
    Bidirectional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcommercePlatform {
    pub base: BaseEntity,
    pub name: String,
    pub platform_type: PlatformType,
    pub base_url: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub access_token: Option<String>,
    pub webhook_secret: Option<String>,
    pub store_id: Option<String>,
    pub status: Status,
    pub sync_direction: SyncDirection,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub sync_interval_minutes: i32,
    pub auto_sync: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductListing {
    pub base: BaseEntity,
    pub platform_id: Uuid,
    pub product_id: Uuid,
    pub external_product_id: String,
    pub external_variant_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub price: Money,
    pub compare_at_price: Option<Money>,
    pub quantity: i64,
    pub sku: Option<String>,
    pub barcode: Option<String>,
    pub status: ProductListingStatus,
    pub visibility: ProductVisibility,
    pub seo_title: Option<String>,
    pub seo_description: Option<String>,
    pub tags: Option<String>,
    pub category: Option<String>,
    pub images: Option<String>,
    pub sync_status: SyncStatus,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub sync_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ProductListingStatus {
    Active,
    Draft,
    Archived,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ProductVisibility {
    Visible,
    Hidden,
    SearchOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcommerceOrder {
    pub base: BaseEntity,
    pub platform_id: Uuid,
    pub external_order_id: String,
    pub order_number: String,
    pub customer_id: Option<Uuid>,
    pub external_customer_id: Option<String>,
    pub sales_order_id: Option<Uuid>,
    pub order_date: DateTime<Utc>,
    pub status: EcommerceOrderStatus,
    pub fulfillment_status: FulfillmentStatus,
    pub payment_status: PaymentStatus,
    pub subtotal: Money,
    pub shipping_amount: Money,
    pub tax_amount: Money,
    pub discount_amount: Money,
    pub total: Money,
    pub currency: String,
    pub billing_name: String,
    pub billing_address: String,
    pub billing_city: String,
    pub billing_state: String,
    pub billing_postal_code: String,
    pub billing_country: String,
    pub shipping_name: String,
    pub shipping_address: String,
    pub shipping_city: String,
    pub shipping_state: String,
    pub shipping_postal_code: String,
    pub shipping_country: String,
    pub shipping_method: Option<String>,
    pub tracking_number: Option<String>,
    pub customer_email: Option<String>,
    pub customer_phone: Option<String>,
    pub notes: Option<String>,
    pub lines: Vec<EcommerceOrderLine>,
    pub sync_status: SyncStatus,
    pub imported_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EcommerceOrderStatus {
    Pending,
    Confirmed,
    Processing,
    OnHold,
    Cancelled,
    Refunded,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum FulfillmentStatus {
    Unfulfilled,
    PartiallyFulfilled,
    Fulfilled,
    Shipped,
    Delivered,
    Returned,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentStatus {
    Pending,
    Authorized,
    Paid,
    PartiallyRefunded,
    Refunded,
    Voided,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcommerceOrderLine {
    pub id: Uuid,
    pub order_id: Uuid,
    pub external_line_id: Option<String>,
    pub product_id: Option<Uuid>,
    pub listing_id: Option<Uuid>,
    pub sku: Option<String>,
    pub title: String,
    pub variant_title: Option<String>,
    pub quantity: i64,
    pub unit_price: Money,
    pub tax_amount: Money,
    pub discount_amount: Money,
    pub line_total: Money,
    pub fulfillment_status: FulfillmentStatus,
    pub quantity_fulfilled: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcommerceCustomer {
    pub base: BaseEntity,
    pub platform_id: Uuid,
    pub external_customer_id: String,
    pub internal_customer_id: Option<Uuid>,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: Option<String>,
    pub company: Option<String>,
    pub total_orders: i64,
    pub total_spent: Money,
    pub first_order_at: Option<DateTime<Utc>>,
    pub last_order_at: Option<DateTime<Utc>>,
    pub tags: Option<String>,
    pub notes: Option<String>,
    pub sync_status: SyncStatus,
    pub last_sync_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventorySync {
    pub base: BaseEntity,
    pub platform_id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub external_product_id: String,
    pub external_variant_id: Option<String>,
    pub local_quantity: i64,
    pub remote_quantity: i64,
    pub reserved_quantity: i64,
    pub sync_status: SyncStatus,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub sync_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub base: BaseEntity,
    pub platform_id: Uuid,
    pub event_type: String,
    pub external_id: String,
    pub payload: String,
    pub processed: bool,
    pub processed_at: Option<DateTime<Utc>>,
    pub error: Option<String>,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceSync {
    pub base: BaseEntity,
    pub platform_id: Uuid,
    pub product_id: Uuid,
    pub price_list_id: Option<Uuid>,
    pub external_product_id: String,
    pub local_price: Money,
    pub remote_price: Money,
    pub sync_status: SyncStatus,
    pub last_sync_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncJob {
    pub base: BaseEntity,
    pub platform_id: Uuid,
    pub job_type: SyncJobType,
    pub status: SyncStatus,
    pub records_total: i64,
    pub records_processed: i64,
    pub records_failed: i64,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SyncJobType {
    FullProductSync,
    IncrementalProductSync,
    OrderImport,
    InventorySync,
    CustomerSync,
    PriceSync,
    FulfillmentUpdate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FulfillmentSync {
    pub base: BaseEntity,
    pub platform_id: Uuid,
    pub order_id: Uuid,
    pub external_order_id: String,
    pub tracking_number: Option<String>,
    pub carrier: Option<String>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub status: FulfillmentStatus,
    pub sync_status: SyncStatus,
    pub last_sync_at: Option<DateTime<Utc>>,
}
