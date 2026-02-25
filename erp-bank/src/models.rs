use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BankConnectionStatus {
    Active,
    Inactive,
    Pending,
    Error,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum StatementFormat {
    BAI2,
    OFX,
    MT940,
    CAMT053,
    CSV,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TransactionType {
    Credit,
    Debit,
    Check,
    WireTransfer,
    ACH,
    CardPayment,
    ATM,
    Fee,
    Interest,
    Adjustment,
    Return,
    Reversal,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReconciliationStatus {
    Unmatched,
    Matched,
    PartiallyMatched,
    Reconciled,
    Exception,
    Review,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankConnection {
    pub base: BaseEntity,
    pub connection_number: String,
    pub bank_name: String,
    pub bank_code: Option<String>,
    pub swift_code: Option<String>,
    pub api_endpoint: Option<String>,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub certificate_path: Option<String>,
    pub authentication_type: AuthenticationType,
    pub statement_format: StatementFormat,
    pub polling_enabled: bool,
    pub polling_interval_minutes: i32,
    pub last_poll_at: Option<DateTime<Utc>>,
    pub last_successful_at: Option<DateTime<Utc>>,
    pub last_error: Option<String>,
    pub status: BankConnectionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AuthenticationType {
    Basic,
    OAuth2,
    APIKey,
    Certificate,
    MTLS,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    pub base: BaseEntity,
    pub connection_id: Uuid,
    pub account_number: String,
    pub masked_account_number: String,
    pub account_name: String,
    pub account_type: BankAccountType,
    pub currency: String,
    pub gl_account_id: Option<Uuid>,
    pub company_id: Uuid,
    pub bank_branch: Option<String>,
    pub iban: Option<String>,
    pub routing_number: Option<String>,
    pub auto_reconcile: bool,
    pub reconciliation_rules: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BankAccountType {
    Checking,
    Savings,
    MoneyMarket,
    LineOfCredit,
    Escrow,
    Payroll,
    Operating,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankStatement {
    pub base: BaseEntity,
    pub statement_number: String,
    pub bank_account_id: Uuid,
    pub statement_date: NaiveDate,
    pub currency: String,
    pub opening_balance: i64,
    pub closing_balance: i64,
    pub total_credits: i64,
    pub total_debits: i64,
    pub credit_count: i32,
    pub debit_count: i32,
    pub statement_format: StatementFormat,
    pub raw_file_path: Option<String>,
    pub imported_at: DateTime<Utc>,
    pub imported_by: Option<Uuid>,
    pub status: StatementStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum StatementStatus {
    Imported,
    Validated,
    Processed,
    Reconciled,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransaction {
    pub base: BaseEntity,
    pub statement_id: Uuid,
    pub bank_account_id: Uuid,
    pub transaction_date: NaiveDate,
    pub value_date: Option<NaiveDate>,
    pub transaction_type: TransactionType,
    pub amount: i64,
    pub currency: String,
    pub reference_number: Option<String>,
    pub bank_reference: Option<String>,
    pub customer_reference: Option<String>,
    pub description: String,
    pub payee_name: Option<String>,
    pub payee_account: Option<String>,
    pub check_number: Option<String>,
    pub additional_info: Option<String>,
    pub reconciliation_status: ReconciliationStatus,
    pub matched_entity_type: Option<String>,
    pub matched_entity_id: Option<Uuid>,
    pub matched_amount: Option<i64>,
    pub match_confidence: Option<f64>,
    pub match_rule: Option<String>,
    pub journal_entry_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationRule {
    pub base: BaseEntity,
    pub rule_name: String,
    pub description: Option<String>,
    pub bank_account_id: Option<Uuid>,
    pub match_criteria: String,
    pub tolerance_type: ToleranceType,
    pub tolerance_value: f64,
    pub date_tolerance_days: i32,
    pub reference_patterns: Option<String>,
    pub auto_match: bool,
    pub priority: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ToleranceType {
    Fixed,
    Percentage,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationSession {
    pub base: BaseEntity,
    pub session_number: String,
    pub bank_account_id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_transactions: i32,
    pub matched_count: i32,
    pub unmatched_count: i32,
    pub exception_count: i32,
    pub auto_matched_count: i32,
    pub manual_matched_count: i32,
    pub opening_balance: i64,
    pub closing_balance: i64,
    pub calculated_balance: i64,
    pub variance: i64,
    pub status: ReconciliationSessionStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReconciliationSessionStatus {
    InProgress,
    Completed,
    Exception,
    Abandoned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationMatch {
    pub id: Uuid,
    pub session_id: Uuid,
    pub bank_transaction_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub entity_reference: String,
    pub transaction_amount: i64,
    pub entity_amount: i64,
    pub match_difference: i64,
    pub match_type: MatchType,
    pub match_rule: Option<String>,
    pub match_confidence: f64,
    pub matched_at: DateTime<Utc>,
    pub matched_by: Option<Uuid>,
    pub status: MatchStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MatchType {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MatchStatus {
    Proposed,
    Confirmed,
    Rejected,
    Unmatched,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentFileGeneration {
    pub base: BaseEntity,
    pub file_number: String,
    pub bank_account_id: Uuid,
    pub file_type: String,
    pub file_date: NaiveDate,
    pub value_date: NaiveDate,
    pub currency: String,
    pub total_amount: i64,
    pub payment_count: i32,
    pub file_content: Option<String>,
    pub file_path: Option<String>,
    pub status: PaymentFileStatus,
    pub generated_at: DateTime<Utc>,
    pub transmitted_at: Option<DateTime<Utc>>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentFileStatus {
    Generated,
    Validated,
    Transmitted,
    Acknowledged,
    Processed,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankFee {
    pub base: BaseEntity,
    pub bank_account_id: Uuid,
    pub fee_date: NaiveDate,
    pub fee_type: BankFeeType,
    pub description: String,
    pub amount: i64,
    pub currency: String,
    pub transaction_id: Option<Uuid>,
    pub gl_account_id: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BankFeeType {
    MonthlyFee,
    TransactionFee,
    WireFee,
    ACHFee,
    CheckFee,
    OverdraftFee,
    StopPaymentFee,
    ReturnFee,
    Other,
}
