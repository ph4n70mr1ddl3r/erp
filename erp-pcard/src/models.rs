use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CardType {
    Corporate,
    Purchasing,
    Travel,
    Fleet,
    OneCard,
    Virtual,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CardStatus {
    Active,
    Suspended,
    Cancelled,
    Lost,
    Stolen,
    Expired,
    PendingActivation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateCard {
    pub base: BaseEntity,
    pub card_number: String,
    pub masked_number: String,
    pub card_type: CardType,
    pub cardholder_id: Uuid,
    pub department_id: Option<Uuid>,
    pub issuer: String,
    pub card_program: Option<String>,
    pub credit_limit: i64,
    pub available_credit: i64,
    pub currency: String,
    pub issue_date: NaiveDate,
    pub expiry_date: NaiveDate,
    pub last_four: String,
    pub embossed_name: Option<String>,
    pub pin_set: bool,
    pub contactless_enabled: bool,
    pub international_enabled: bool,
    pub atm_enabled: bool,
    pub online_enabled: bool,
    pub mcc_restrictions: Option<String>,
    pub merchant_restrictions: Option<String>,
    pub daily_limit: Option<i64>,
    pub transaction_limit: Option<i64>,
    pub status: CardStatus,
    pub activated_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancellation_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardTransaction {
    pub base: BaseEntity,
    pub transaction_number: String,
    pub card_id: Uuid,
    pub transaction_date: NaiveDate,
    pub posting_date: Option<NaiveDate>,
    pub merchant_name: String,
    pub merchant_category: Option<String>,
    pub mcc_code: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub billing_amount: i64,
    pub billing_currency: String,
    pub transaction_type: CardTransactionType,
    pub status: CardTransactionStatus,
    pub reference_number: Option<String>,
    pub authorization_code: Option<String>,
    pub description: Option<String>,
    pub receipt_available: bool,
    pub receipt_path: Option<String>,
    pub expense_report_id: Option<Uuid>,
    pub expense_line_id: Option<Uuid>,
    pub reconciled: bool,
    pub reconciled_at: Option<DateTime<Utc>>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub tax_amount: Option<i64>,
    pub tip_amount: Option<i64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CardTransactionType {
    Purchase,
    Refund,
    CashAdvance,
    Fee,
    Payment,
    Adjustment,
    Authorization,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CardTransactionStatus {
    Pending,
    Posted,
    Disputed,
    Reconciled,
    Declined,
    Reversed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardStatement {
    pub base: BaseEntity,
    pub statement_number: String,
    pub card_id: Uuid,
    pub statement_date: NaiveDate,
    pub due_date: NaiveDate,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub opening_balance: i64,
    pub payments: i64,
    pub credits: i64,
    pub purchases: i64,
    pub fees: i64,
    pub interest: i64,
    pub closing_balance: i64,
    pub minimum_payment: i64,
    pub currency: String,
    pub paid_amount: Option<i64>,
    pub paid_date: Option<NaiveDate>,
    pub status: StatementPaymentStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum StatementPaymentStatus {
    Open,
    PartiallyPaid,
    Paid,
    Overdue,
    Disputed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PCardOrder {
    pub base: BaseEntity,
    pub order_number: String,
    pub card_id: Uuid,
    pub vendor_id: Uuid,
    pub vendor_name: String,
    pub order_date: NaiveDate,
    pub required_date: Option<NaiveDate>,
    pub delivery_date: Option<NaiveDate>,
    pub subtotal: i64,
    pub tax: i64,
    pub shipping: i64,
    pub total: i64,
    pub currency: String,
    pub po_number: Option<String>,
    pub transaction_id: Option<Uuid>,
    pub status: PCardOrderStatus,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PCardOrderStatus {
    Draft,
    PendingApproval,
    Approved,
    Ordered,
    Received,
    Invoiced,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PCardOrderLine {
    pub id: Uuid,
    pub order_id: Uuid,
    pub line_number: i32,
    pub product_id: Option<Uuid>,
    pub description: String,
    pub quantity: i64,
    pub unit_price: i64,
    pub tax: i64,
    pub total: i64,
    pub gl_account_id: Option<Uuid>,
    pub cost_center_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub received_quantity: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardPolicy {
    pub base: BaseEntity,
    pub policy_number: String,
    pub name: String,
    pub description: Option<String>,
    pub card_type: CardType,
    pub default_limit: i64,
    pub daily_limit: Option<i64>,
    pub transaction_limit: Option<i64>,
    pub mcc_allowed: Option<String>,
    pub mcc_blocked: Option<String>,
    pub merchant_blocked: Option<String>,
    pub requires_approval_over: Option<i64>,
    pub approval_workflow_id: Option<Uuid>,
    pub requires_receipt_over: Option<i64>,
    pub auto_reconcile: bool,
    pub international_allowed: bool,
    pub atm_allowed: bool,
    pub online_allowed: bool,
    pub contactless_allowed: bool,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardLimitOverride {
    pub base: BaseEntity,
    pub override_number: String,
    pub card_id: Uuid,
    pub original_limit: i64,
    pub new_limit: i64,
    pub limit_type: LimitType,
    pub reason: String,
    pub effective_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub status: OverrideStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LimitType {
    Credit,
    Daily,
    Transaction,
    ATM,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum OverrideStatus {
    Pending,
    Approved,
    Rejected,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardDispute {
    pub base: BaseEntity,
    pub dispute_number: String,
    pub card_id: Uuid,
    pub transaction_id: Uuid,
    pub dispute_type: DisputeType,
    pub dispute_reason: String,
    pub disputed_amount: i64,
    pub currency: String,
    pub filed_date: NaiveDate,
    pub resolution_date: Option<NaiveDate>,
    pub resolution: Option<String>,
    pub provisional_credit: Option<i64>,
    pub provisional_credit_date: Option<NaiveDate>,
    pub status: DisputeStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DisputeType {
    Unauthorized,
    Fraud,
    NotReceived,
    Defective,
    NotAsDescribed,
    Duplicate,
    IncorrectAmount,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DisputeStatus {
    Filed,
    UnderReview,
    ProvisionalCredit,
    Resolved,
    Closed,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualCard {
    pub base: BaseEntity,
    pub parent_card_id: Option<Uuid>,
    pub cardholder_id: Uuid,
    pub masked_number: String,
    pub credit_limit: i64,
    pub available_credit: i64,
    pub currency: String,
    pub valid_from: NaiveDate,
    pub valid_until: NaiveDate,
    pub single_use: bool,
    pub merchant_lock: Option<String>,
    pub usage_limit: Option<i32>,
    pub usage_count: i32,
    pub status: CardStatus,
    pub created_at: DateTime<Utc>,
}
