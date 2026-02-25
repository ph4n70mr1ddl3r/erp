use chrono::{DateTime, Utc};
use erp_core::models::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudAlert {
    pub base: BaseEntity,
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub status: AlertStatus,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub rule_id: Option<Uuid>,
    pub score: f64,
    pub risk_factors: Vec<RiskFactor>,
    pub description: String,
    pub detected_at: DateTime<Utc>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub resolution: Option<AlertResolution>,
    pub assigned_to: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum AlertType {
    TransactionAnomaly,
    DuplicatePayment,
    VendorFraud,
    EmployeeFraud,
    InvoiceFraud,
    ExpenseFraud,
    PayrollFraud,
    InventoryShrinkage,
    DataTampering,
    UnauthorizedAccess,
    PolicyViolation,
    SuspiciousPattern,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum AlertSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum AlertStatus {
    New,
    UnderReview,
    Confirmed,
    FalsePositive,
    Escalated,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskFactor {
    pub factor_type: String,
    pub description: String,
    pub weight: f64,
    pub value: serde_json::Value,
    pub contribution: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertResolution {
    pub resolution_type: ResolutionType,
    pub notes: String,
    pub actions_taken: Vec<String>,
    pub resolved_by: Uuid,
    pub resolved_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResolutionType {
    ConfirmedFraud,
    FalsePositive,
    RequiresMonitoring,
    PolicyException,
    UnderInvestigation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudRule {
    pub base: BaseEntity,
    pub name: String,
    pub description: String,
    pub category: RuleCategory,
    pub rule_type: RuleType,
    pub conditions: Vec<RuleCondition>,
    pub actions: Vec<RuleAction>,
    pub enabled: bool,
    pub priority: i32,
    pub false_positive_rate: f64,
    pub true_positive_rate: f64,
    pub last_triggered: Option<DateTime<Utc>>,
    pub trigger_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleCategory {
    Transaction,
    Vendor,
    Employee,
    Invoice,
    Expense,
    Payroll,
    Inventory,
    Access,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleType {
    Threshold,
    Statistical,
    Pattern,
    MachineLearning,
    Behavioral,
    Composite,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub field: String,
    pub operator: ConditionOperator,
    pub value: serde_json::Value,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Contains,
    NotContains,
    InList,
    NotInList,
    RegexMatch,
    DeviationFromMean(f64),
    PercentileAbove(f64),
    PercentileBelow(f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleAction {
    pub action_type: ActionType,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    CreateAlert,
    BlockTransaction,
    RequireApproval,
    NotifyUser,
    NotifyAdmin,
    LogEvent,
    TagEntity,
    UpdateRiskScore,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskScore {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub overall_score: f64,
    pub score_components: Vec<ScoreComponent>,
    pub risk_level: RiskLevel,
    pub last_updated: DateTime<Utc>,
    pub factors: Vec<RiskFactor>,
    pub historical_trend: Vec<HistoricalScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoreComponent {
    pub component_name: String,
    pub score: f64,
    pub weight: f64,
    pub contribution: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoricalScore {
    pub date: DateTime<Utc>,
    pub score: f64,
    pub trigger: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudCase {
    pub base: BaseEntity,
    pub case_number: String,
    pub title: String,
    pub description: String,
    pub status: CaseStatus,
    pub priority: CasePriority,
    pub assigned_investigator: Option<Uuid>,
    pub alert_ids: Vec<Uuid>,
    pub related_entities: Vec<RelatedEntity>,
    pub timeline: Vec<CaseEvent>,
    pub evidence: Vec<Evidence>,
    pub estimated_loss: i64,
    pub actual_loss: Option<i64>,
    pub recovery_amount: Option<i64>,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub resolution: Option<CaseResolution>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum CaseStatus {
    Open,
    UnderInvestigation,
    PendingReview,
    AwaitingAction,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum CasePriority {
    Low,
    Medium,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RelatedEntity {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub relationship: String,
    pub relevance_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: String,
    pub description: String,
    pub user_id: Option<Uuid>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Evidence {
    pub id: Uuid,
    pub evidence_type: EvidenceType,
    pub description: String,
    pub file_path: Option<String>,
    pub collected_at: DateTime<Utc>,
    pub collected_by: Uuid,
    pub verified: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType {
    Document,
    Screenshot,
    LogEntry,
    TransactionRecord,
    WitnessStatement,
    Communication,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseResolution {
    pub outcome: CaseOutcome,
    pub summary: String,
    pub actions_taken: Vec<String>,
    pub recommendations: Vec<String>,
    pub resolved_by: Uuid,
    pub resolved_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaseOutcome {
    Confirmed,
    PartiallyConfirmed,
    NotConfirmed,
    InsufficientEvidence,
    Referred,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FraudAnalytics {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_alerts: i64,
    pub alerts_by_type: serde_json::Value,
    pub alerts_by_severity: serde_json::Value,
    pub false_positive_rate: f64,
    pub average_resolution_time_hours: f64,
    pub total_estimated_loss: i64,
    pub total_actual_loss: i64,
    pub total_recovery: i64,
    pub top_risk_entities: Vec<EntityRiskSummary>,
    pub trend_data: Vec<DailyFraudStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityRiskSummary {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub entity_name: String,
    pub risk_score: f64,
    pub alert_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DailyFraudStats {
    pub date: DateTime<Utc>,
    pub alert_count: i64,
    pub confirmed_fraud_count: i64,
    pub false_positive_count: i64,
    pub estimated_loss: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorRiskProfile {
    pub vendor_id: Uuid,
    pub risk_score: f64,
    pub risk_level: RiskLevel,
    pub risk_factors: Vec<RiskFactor>,
    pub historical_alerts: i64,
    pub payment_anomalies: i64,
    pub days_since_first_transaction: i64,
    pub total_transaction_value: i64,
    pub average_transaction_value: i64,
    pub transaction_count: i64,
    pub duplicate_invoice_attempts: i64,
    pub address_changes: i64,
    pub bank_account_changes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeRiskProfile {
    pub employee_id: Uuid,
    pub risk_score: f64,
    pub risk_level: RiskLevel,
    pub risk_factors: Vec<RiskFactor>,
    pub expense_anomalies: i64,
    pub access_violations: i64,
    pub policy_violations: i64,
    pub total_expense_value: i64,
    pub average_expense_value: i64,
    pub expense_count: i64,
    pub after_hours_access: i64,
    pub data_export_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateAlertRequest {
    pub alert_type: AlertType,
    pub severity: AlertSeverity,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub rule_id: Option<Uuid>,
    pub score: f64,
    pub risk_factors: Vec<RiskFactor>,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewAlertRequest {
    pub status: AlertStatus,
    pub resolution: Option<AlertResolution>,
    pub notes: Option<String>,
}
