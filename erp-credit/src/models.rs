use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CreditHoldStatus {
    Active,
    Released,
    Overridden,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CreditCheckResult {
    Approved,
    Warning,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CreditTransactionType {
    InvoiceCreated,
    InvoicePaid,
    OrderPlaced,
    OrderCancelled,
    CreditLimitChanged,
    ManualAdjustment,
    CreditHoldPlaced,
    CreditHoldReleased,
    Overpayment,
    WriteOff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerCreditProfile {
    pub base: BaseEntity,
    pub customer_id: Uuid,
    pub credit_limit: i64,
    pub credit_used: i64,
    pub available_credit: i64,
    pub outstanding_invoices: i64,
    pub pending_orders: i64,
    pub overdue_amount: i64,
    pub overdue_days_avg: i32,
    pub credit_score: Option<i32>,
    pub risk_level: RiskLevel,
    pub payment_history_score: Option<f64>,
    pub last_credit_review: Option<DateTime<Utc>>,
    pub next_review_date: Option<DateTime<Utc>>,
    pub auto_hold_enabled: bool,
    pub hold_threshold_percent: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditTransaction {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub customer_id: Uuid,
    pub transaction_type: CreditTransactionType,
    pub amount: i64,
    pub previous_credit_used: i64,
    pub new_credit_used: i64,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub reference_number: Option<String>,
    pub description: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditHold {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub customer_id: Uuid,
    pub hold_type: CreditHoldType,
    pub reason: String,
    pub amount_over_limit: i64,
    pub related_order_id: Option<Uuid>,
    pub related_invoice_id: Option<Uuid>,
    pub status: CreditHoldStatus,
    pub placed_by: Option<Uuid>,
    pub placed_at: DateTime<Utc>,
    pub released_by: Option<Uuid>,
    pub released_at: Option<DateTime<Utc>>,
    pub override_reason: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CreditHoldType {
    CreditLimitExceeded,
    OverdueInvoices,
    PaymentDefault,
    ManualHold,
    RiskAssessment,
    CreditReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCheckRequest {
    pub customer_id: Uuid,
    pub order_id: Option<Uuid>,
    pub order_amount: i64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditCheckResponse {
    pub customer_id: Uuid,
    pub result: CreditCheckResult,
    pub credit_limit: i64,
    pub credit_used: i64,
    pub available_credit: i64,
    pub requested_amount: i64,
    pub projected_available: i64,
    pub hold_id: Option<Uuid>,
    pub reason: Option<String>,
    pub warnings: Vec<String>,
    pub checked_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditLimitChange {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub customer_id: Uuid,
    pub previous_limit: i64,
    pub new_limit: i64,
    pub change_reason: String,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub effective_date: DateTime<Utc>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditAlert {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub customer_id: Uuid,
    pub alert_type: CreditAlertType,
    pub severity: AlertSeverity,
    pub message: String,
    pub threshold_value: i64,
    pub actual_value: i64,
    pub is_read: bool,
    pub acknowledged_by: Option<Uuid>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CreditAlertType {
    ApproachingLimit,
    LimitExceeded,
    OverduePayment,
    HighRisk,
    CreditReviewDue,
    PaymentReceived,
    HoldPlaced,
    HoldReleased,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AlertSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditSummary {
    pub total_customers: i64,
    pub total_credit_limit: i64,
    pub total_credit_used: i64,
    pub total_available_credit: i64,
    pub total_overdue: i64,
    pub customers_on_hold: i64,
    pub high_risk_customers: i64,
    pub avg_utilization_percent: f64,
}
