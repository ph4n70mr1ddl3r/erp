use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TaxType {
    SalesTax,
    VAT,
    GST,
    PST,
    HST,
    Withholding,
    Excise,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TaxCalculationMethod {
    Exclusive,
    Inclusive,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxJurisdiction {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub country_code: String,
    pub state_code: Option<String>,
    pub county: Option<String>,
    pub city: Option<String>,
    pub postal_code_from: Option<String>,
    pub postal_code_to: Option<String>,
    pub parent_jurisdiction_id: Option<Uuid>,
    pub status: Status,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRate {
    pub base: BaseEntity,
    pub jurisdiction_id: Uuid,
    pub tax_type: TaxType,
    pub name: String,
    pub code: String,
    pub rate: f64,
    pub is_compound: bool,
    pub is_recoverable: bool,
    pub calculation_method: TaxCalculationMethod,
    pub status: Status,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
    pub priority: i32,
    pub min_amount: Option<Money>,
    pub max_amount: Option<Money>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRule {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub jurisdiction_id: Option<Uuid>,
    pub tax_rate_ids: Vec<Uuid>,
    pub customer_tax_class_id: Option<Uuid>,
    pub product_tax_class_id: Option<Uuid>,
    pub priority: i32,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxClass {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub class_type: TaxClassType,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TaxClassType {
    Product,
    Customer,
    Both,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxExemption {
    pub base: BaseEntity,
    pub customer_id: Uuid,
    pub exemption_type: ExemptionType,
    pub certificate_number: String,
    pub jurisdiction_id: Option<Uuid>,
    pub issue_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub status: Status,
    pub document_url: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ExemptionType {
    Resale,
    Manufacturing,
    Agricultural,
    Government,
    NonProfit,
    Educational,
    DirectPay,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxTransaction {
    pub base: BaseEntity,
    pub transaction_type: String,
    pub transaction_id: Uuid,
    pub transaction_date: DateTime<Utc>,
    pub customer_id: Option<Uuid>,
    pub jurisdiction_id: Uuid,
    pub tax_rate_id: Uuid,
    pub tax_class_id: Option<Uuid>,
    pub tax_type: TaxType,
    pub taxable_amount: Money,
    pub tax_rate: f64,
    pub tax_amount: Money,
    pub exemption_id: Option<Uuid>,
    pub exempt_amount: Money,
    pub source: TaxTransactionSource,
    pub external_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TaxTransactionSource {
    Manual,
    SalesOrder,
    Invoice,
    PurchaseOrder,
    POS,
    Ecommerce,
    Import,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxReport {
    pub base: BaseEntity,
    pub name: String,
    pub jurisdiction_id: Uuid,
    pub report_period: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_sales: Money,
    pub taxable_sales: Money,
    pub exempt_sales: Money,
    pub tax_collected: Money,
    pub tax_paid: Money,
    pub tax_due: Money,
    pub status: TaxReportStatus,
    pub filed_at: Option<DateTime<Utc>>,
    pub filed_by: Option<Uuid>,
    pub filing_reference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TaxReportStatus {
    Draft,
    Generated,
    Filed,
    Paid,
    Amended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Nexus {
    pub base: BaseEntity,
    pub jurisdiction_id: Uuid,
    pub nexus_type: NexusType,
    pub established_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: Status,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum NexusType {
    Physical,
    Economic,
    Affiliate,
    Marketplace,
    ClickThrough,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxZone {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub countries: String,
    pub states: Option<String>,
    pub postal_codes: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxConfiguration {
    pub base: BaseEntity,
    pub name: String,
    pub calculation_engine: TaxEngine,
    pub default_tax_class_id: Option<Uuid>,
    pub shipping_taxable: bool,
    pub discount_affects_tax: bool,
    pub tax_based_on: TaxBasedOn,
    pub price_includes_tax: bool,
    pub apply_tax_after_discount: bool,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TaxEngine {
    Internal,
    Avalara,
    TaxJar,
    Vertex,
    Sovos,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TaxBasedOn {
    ShippingOrigin,
    ShippingDestination,
    BillingAddress,
    StoreAddress,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxAuditLog {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub action: String,
    pub old_values: Option<String>,
    pub new_values: Option<String>,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRateHistory {
    pub id: Uuid,
    pub tax_rate_id: Uuid,
    pub old_rate: f64,
    pub new_rate: f64,
    pub effective_from: DateTime<Utc>,
    pub effective_to: Option<DateTime<Utc>>,
    pub changed_at: DateTime<Utc>,
    pub changed_by: Uuid,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxInvoice {
    pub base: BaseEntity,
    pub invoice_number: String,
    pub transaction_id: Uuid,
    pub customer_id: Uuid,
    pub invoice_date: DateTime<Utc>,
    pub subtotal: Money,
    pub total_tax: Money,
    pub total: Money,
    pub currency: String,
    pub lines: Vec<TaxInvoiceLine>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxInvoiceLine {
    pub id: Uuid,
    pub tax_invoice_id: Uuid,
    pub description: String,
    pub tax_rate_id: Uuid,
    pub tax_rate: f64,
    pub taxable_amount: Money,
    pub tax_amount: Money,
}
