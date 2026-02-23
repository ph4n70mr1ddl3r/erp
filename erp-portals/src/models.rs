use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PortalType {
    Customer,
    Supplier,
    Partner,
    Employee,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PortalAccessLevel {
    ReadOnly,
    Standard,
    Premium,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalUser {
    pub base: BaseEntity,
    pub portal_type: PortalType,
    pub external_id: Option<Uuid>,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub display_name: Option<String>,
    pub company_name: Option<String>,
    pub phone: Option<String>,
    pub access_level: PortalAccessLevel,
    pub permissions: Option<String>,
    pub preferences: Option<String>,
    pub language: String,
    pub timezone: String,
    pub avatar_url: Option<String>,
    pub last_login_at: Option<DateTime<Utc>>,
    pub login_count: i32,
    pub failed_login_count: i32,
    pub locked_until: Option<DateTime<Utc>>,
    pub password_changed_at: Option<DateTime<Utc>>,
    pub must_change_password: bool,
    pub two_factor_enabled: bool,
    pub two_factor_secret: Option<String>,
    pub api_key: Option<String>,
    pub api_key_expires_at: Option<DateTime<Utc>>,
    pub session_timeout_minutes: i32,
    pub status: PortalUserStatus,
    pub invited_by: Option<Uuid>,
    pub invited_at: Option<DateTime<Utc>>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub notification_preferences: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PortalUserStatus {
    Pending,
    Active,
    Suspended,
    Locked,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerPortalAccess {
    pub id: Uuid,
    pub portal_user_id: Uuid,
    pub customer_id: Uuid,
    pub role: CustomerPortalRole,
    pub permissions: Option<String>,
    pub can_view_orders: bool,
    pub can_create_orders: bool,
    pub can_view_invoices: bool,
    pub can_pay_invoices: bool,
    pub can_view_quotes: bool,
    pub can_request_quotes: bool,
    pub can_view_shipments: bool,
    pub can_view_returns: bool,
    pub can_create_returns: bool,
    pub can_view_contracts: bool,
    pub can_view_statements: bool,
    pub can_download_documents: bool,
    pub can_view_pricing: bool,
    pub can_view_inventory: bool,
    pub default_warehouse_id: Option<Uuid>,
    pub default_price_list_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum CustomerPortalRole {
    Viewer,
    Buyer,
    Approver,
    Administrator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierPortalAccess {
    pub id: Uuid,
    pub portal_user_id: Uuid,
    pub vendor_id: Uuid,
    pub role: SupplierPortalRole,
    pub permissions: Option<String>,
    pub can_view_pos: bool,
    pub can_submit_quotes: bool,
    pub can_update_orders: bool,
    pub can_create_invoices: bool,
    pub can_view_payments: bool,
    pub can_view_forecasts: bool,
    pub can_update_inventory: bool,
    pub can_view_performance: bool,
    pub can_upload_documents: bool,
    pub can_manage_catalog: bool,
    pub default_currency: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum SupplierPortalRole {
    Viewer,
    Contributor,
    Administrator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalSession {
    pub id: Uuid,
    pub portal_user_id: Uuid,
    pub session_token: String,
    pub refresh_token: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub device_type: Option<String>,
    pub login_at: DateTime<Utc>,
    pub last_activity_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub logout_at: Option<DateTime<Utc>>,
    pub logout_reason: Option<String>,
    pub status: SessionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum SessionStatus {
    Active,
    Expired,
    LoggedOut,
    Revoked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalOrder {
    pub base: BaseEntity,
    pub portal_user_id: Uuid,
    pub portal_order_number: String,
    pub erp_order_id: Option<Uuid>,
    pub erp_order_number: Option<String>,
    pub customer_id: Uuid,
    pub order_type: PortalOrderType,
    pub status: PortalOrderStatus,
    pub billing_address: Option<String>,
    pub shipping_address: Option<String>,
    pub requested_delivery_date: Option<DateTime<Utc>>,
    pub shipping_method: Option<String>,
    pub payment_method: Option<String>,
    pub notes: Option<String>,
    pub internal_notes: Option<String>,
    pub subtotal_cents: i64,
    pub tax_cents: i64,
    pub shipping_cents: i64,
    pub discount_cents: i64,
    pub total_cents: i64,
    pub currency: String,
    pub submitted_at: Option<DateTime<Utc>>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub lines: Vec<PortalOrderLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PortalOrderType {
    Standard,
    Rush,
    Replenishment,
    Sample,
    Consignment,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PortalOrderStatus {
    Draft,
    PendingApproval,
    Submitted,
    Confirmed,
    Processing,
    PartiallyShipped,
    Shipped,
    Delivered,
    Cancelled,
    OnHold,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalOrderLine {
    pub id: Uuid,
    pub portal_order_id: Uuid,
    pub line_number: i32,
    pub product_id: Uuid,
    pub product_code: String,
    pub product_name: String,
    pub description: Option<String>,
    pub quantity: i64,
    pub unit_of_measure: String,
    pub unit_price_cents: i64,
    pub discount_percent: f64,
    pub discount_cents: i64,
    pub tax_percent: f64,
    pub tax_cents: i64,
    pub line_total_cents: i64,
    pub notes: Option<String>,
    pub erp_line_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalInvoice {
    pub base: BaseEntity,
    pub portal_user_id: Uuid,
    pub erp_invoice_id: Uuid,
    pub erp_invoice_number: String,
    pub customer_id: Uuid,
    pub invoice_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub status: PortalInvoiceStatus,
    pub subtotal_cents: i64,
    pub tax_cents: i64,
    pub total_cents: i64,
    pub amount_paid_cents: i64,
    pub amount_due_cents: i64,
    pub currency: String,
    pub payment_url: Option<String>,
    pub pdf_url: Option<String>,
    pub payments: Option<String>,
    pub viewed_at: Option<DateTime<Utc>>,
    pub disputed: bool,
    pub dispute_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PortalInvoiceStatus {
    Draft,
    Sent,
    Viewed,
    PartiallyPaid,
    Paid,
    Overdue,
    Disputed,
    Cancelled,
    WriteOff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalPayment {
    pub base: BaseEntity,
    pub portal_user_id: Uuid,
    pub payment_reference: String,
    pub erp_payment_id: Option<Uuid>,
    pub customer_id: Uuid,
    pub invoice_ids: String,
    pub payment_method: PaymentMethodType,
    pub amount_cents: i64,
    pub currency: String,
    pub status: PaymentStatus,
    pub payment_provider: Option<String>,
    pub provider_transaction_id: Option<String>,
    pub provider_response: Option<String>,
    pub card_last_four: Option<String>,
    pub card_brand: Option<String>,
    pub bank_name: Option<String>,
    pub check_number: Option<String>,
    pub processed_at: Option<DateTime<Utc>>,
    pub failed_at: Option<DateTime<Utc>>,
    pub failure_reason: Option<String>,
    pub refunded_at: Option<DateTime<Utc>>,
    pub refund_amount_cents: Option<i64>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentMethodType {
    CreditCard,
    DebitCard,
    ACH,
    WireTransfer,
    Check,
    PayPal,
    Stripe,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
    Refunded,
    PartiallyRefunded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierQuoteSubmission {
    pub base: BaseEntity,
    pub portal_user_id: Uuid,
    pub vendor_id: Uuid,
    pub rfq_id: Uuid,
    pub quote_number: String,
    pub erp_quote_id: Option<Uuid>,
    pub status: SupplierQuoteStatus,
    pub valid_until: Option<DateTime<Utc>>,
    pub delivery_lead_time_days: i32,
    pub payment_terms: Option<String>,
    pub incoterms: Option<String>,
    pub notes: Option<String>,
    pub internal_notes: Option<String>,
    pub subtotal_cents: i64,
    pub tax_cents: i64,
    pub total_cents: i64,
    pub currency: String,
    pub submitted_at: Option<DateTime<Utc>>,
    pub lines: Vec<SupplierQuoteLine>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum SupplierQuoteStatus {
    Draft,
    Submitted,
    UnderReview,
    Accepted,
    Rejected,
    Expired,
    Withdrawn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierQuoteLine {
    pub id: Uuid,
    pub submission_id: Uuid,
    pub rfq_line_id: Uuid,
    pub product_id: Option<Uuid>,
    pub product_code: String,
    pub description: String,
    pub quantity: i64,
    pub unit_of_measure: String,
    pub unit_price_cents: i64,
    pub discount_percent: f64,
    pub tax_percent: f64,
    pub line_total_cents: i64,
    pub lead_time_days: i32,
    pub minimum_order_quantity: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalNotification {
    pub id: Uuid,
    pub portal_user_id: Uuid,
    pub notification_type: PortalNotificationType,
    pub title: String,
    pub message: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub action_url: Option<String>,
    pub priority: i32,
    pub read_at: Option<DateTime<Utc>>,
    pub emailed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PortalNotificationType {
    OrderConfirmation,
    OrderShipped,
    OrderDelivered,
    InvoiceCreated,
    PaymentReceived,
    PaymentFailed,
    QuoteReady,
    ReturnProcessed,
    CreditMemo,
    AccountUpdate,
    SecurityAlert,
    SystemMaintenance,
    ApprovalRequired,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalDocument {
    pub base: BaseEntity,
    pub portal_user_id: Uuid,
    pub document_type: PortalDocumentType,
    pub document_number: String,
    pub erp_entity_type: String,
    pub erp_entity_id: Uuid,
    pub file_name: String,
    pub file_path: String,
    pub file_size_bytes: i64,
    pub mime_type: String,
    pub download_count: i32,
    pub last_downloaded_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PortalDocumentType {
    Invoice,
    CreditMemo,
    Statement,
    PackingSlip,
    BillOfLading,
    CertificateOfAnalysis,
    MSDS,
    Warranty,
    Contract,
    Quote,
    OrderConfirmation,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalActivityLog {
    pub id: Uuid,
    pub portal_user_id: Uuid,
    pub session_id: Option<Uuid>,
    pub activity_type: String,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub description: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub request_path: Option<String>,
    pub request_method: Option<String>,
    pub response_status: Option<i32>,
    pub duration_ms: Option<i64>,
    pub additional_data: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalSettings {
    pub id: Uuid,
    pub portal_type: PortalType,
    pub setting_key: String,
    pub setting_value: String,
    pub description: Option<String>,
    pub is_encrypted: bool,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalTheme {
    pub id: Uuid,
    pub name: String,
    pub portal_type: PortalType,
    pub primary_color: String,
    pub secondary_color: String,
    pub accent_color: String,
    pub logo_url: Option<String>,
    pub favicon_url: Option<String>,
    pub custom_css: Option<String>,
    pub custom_header: Option<String>,
    pub custom_footer: Option<String>,
    pub is_default: bool,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
