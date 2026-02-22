use chrono::{DateTime, Utc};
use erp_core::{Address, BaseEntity, ContactInfo, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Customer {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub contact: ContactInfo,
    pub billing_address: Address,
    pub shipping_address: Option<Address>,
    pub credit_limit: Option<Money>,
    pub payment_terms: u32,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesOrder {
    pub base: BaseEntity,
    pub order_number: String,
    pub customer_id: Uuid,
    pub order_date: DateTime<Utc>,
    pub required_date: Option<DateTime<Utc>>,
    pub lines: Vec<SalesOrderLine>,
    pub subtotal: Money,
    pub tax_amount: Money,
    pub total: Money,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesOrderLine {
    pub id: Uuid,
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
    pub unit_price: Money,
    pub discount_percent: f64,
    pub tax_rate: f64,
    pub line_total: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesQuote {
    pub base: BaseEntity,
    pub quote_number: String,
    pub customer_id: Uuid,
    pub quote_date: DateTime<Utc>,
    pub valid_until: DateTime<Utc>,
    pub lines: Vec<SalesQuoteLine>,
    pub subtotal: Money,
    pub tax_amount: Money,
    pub total: Money,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesQuoteLine {
    pub id: Uuid,
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
    pub unit_price: Money,
    pub discount_percent: f64,
    pub line_total: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub base: BaseEntity,
    pub invoice_number: String,
    pub customer_id: Uuid,
    pub sales_order_id: Option<Uuid>,
    pub invoice_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub lines: Vec<InvoiceLine>,
    pub subtotal: Money,
    pub tax_amount: Money,
    pub total: Money,
    pub amount_paid: Money,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvoiceLine {
    pub id: Uuid,
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
    pub unit_price: Money,
    pub line_total: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payment {
    pub base: BaseEntity,
    pub payment_number: String,
    pub customer_id: Uuid,
    pub invoice_id: Option<Uuid>,
    pub payment_date: DateTime<Utc>,
    pub amount: Money,
    pub payment_method: PaymentMethod,
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentMethod {
    Cash,
    Check,
    CreditCard,
    BankTransfer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lead {
    pub id: Uuid,
    pub lead_number: String,
    pub company_name: String,
    pub contact_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub source: Option<String>,
    pub industry: Option<String>,
    pub estimated_value: i64,
    pub status: LeadStatus,
    pub assigned_to: Option<Uuid>,
    pub notes: Option<String>,
    pub converted_to_customer: Option<Uuid>,
    pub converted_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LeadStatus {
    New,
    Contacted,
    Qualified,
    Unqualified,
    Converted,
    Lost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opportunity {
    pub id: Uuid,
    pub opportunity_number: String,
    pub name: String,
    pub customer_id: Option<Uuid>,
    pub lead_id: Option<Uuid>,
    pub stage: OpportunityStage,
    pub probability: i32,
    pub expected_close_date: Option<DateTime<Utc>>,
    pub amount: i64,
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub status: OpportunityStatus,
    pub activities: Vec<OpportunityActivity>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum OpportunityStage {
    Prospecting,
    Qualification,
    Proposal,
    Negotiation,
    ClosedWon,
    ClosedLost,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum OpportunityStatus {
    Open,
    Won,
    Lost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpportunityActivity {
    pub id: Uuid,
    pub opportunity_id: Uuid,
    pub activity_type: ActivityType,
    pub subject: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ActivityType {
    Call,
    Meeting,
    Email,
    Task,
    Note,
}
