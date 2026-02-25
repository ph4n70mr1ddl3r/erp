use chrono::{DateTime, Utc};
use erp_core::models::{BaseEntity, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BudgetType {
    Operating,
    Capital,
    Project,
    Department,
    CashFlow,
    Revenue,
    Expense,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[derive(PartialEq)]
pub enum BudgetStatus {
    Draft,
    Submitted,
    UnderReview,
    Approved,
    Rejected,
    Active,
    Closed,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ForecastMethod {
    Linear,
    Exponential,
    MovingAverage,
    Seasonal,
    ARIMA,
    MachineLearning,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub budget_type: BudgetType,
    pub status: BudgetStatus,
    pub fiscal_year: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub total_amount: Money,
    pub currency: String,
    pub department_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub owner_id: Uuid,
    pub approval_workflow_id: Option<Uuid>,
    pub version: i32,
    pub parent_budget_id: Option<Uuid>,
    pub is_template: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetLine {
    pub base: BaseEntity,
    pub budget_id: Uuid,
    pub account_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub description: Option<String>,
    pub planned_amount: Money,
    pub committed_amount: Money,
    pub actual_amount: Money,
    pub variance_amount: Money,
    pub variance_percent: f64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub cost_center_id: Option<Uuid>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetPeriod {
    pub base: BaseEntity,
    pub budget_id: Uuid,
    pub period_type: PeriodType,
    pub period_number: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub planned_amount: Money,
    pub actual_amount: Money,
    pub is_locked: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PeriodType {
    Monthly,
    Quarterly,
    SemiAnnual,
    Annual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Forecast {
    pub base: BaseEntity,
    pub name: String,
    pub forecast_type: BudgetType,
    pub method: ForecastMethod,
    pub fiscal_year: i32,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub total_forecast: Money,
    pub confidence_level: f64,
    pub created_by: Uuid,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastLine {
    pub base: BaseEntity,
    pub forecast_id: Uuid,
    pub account_id: Uuid,
    pub period_date: DateTime<Utc>,
    pub forecasted_amount: Money,
    pub actual_amount: Option<Money>,
    pub accuracy_score: Option<f64>,
    pub factors: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetTransfer {
    pub base: BaseEntity,
    pub from_budget_line_id: Uuid,
    pub to_budget_line_id: Uuid,
    pub amount: Money,
    pub reason: String,
    pub requested_by: Uuid,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetScenario {
    pub base: BaseEntity,
    pub budget_id: Uuid,
    pub name: String,
    pub scenario_type: ScenarioType,
    pub adjustment_factor: f64,
    pub description: Option<String>,
    pub total_amount: Money,
    pub is_baseline: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ScenarioType {
    BestCase,
    WorstCase,
    MostLikely,
    Optimistic,
    Pessimistic,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetAlert {
    pub base: BaseEntity,
    pub budget_id: Uuid,
    pub budget_line_id: Option<Uuid>,
    pub alert_type: AlertType,
    pub threshold_percent: f64,
    pub is_active: bool,
    pub last_triggered: Option<DateTime<Utc>>,
    pub notify_users: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AlertType {
    OverBudget,
    NearLimit,
    UnderBudget,
    VarianceExceeded,
    ForecastDeviation,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetApproval {
    pub base: BaseEntity,
    pub budget_id: Uuid,
    pub approver_id: Uuid,
    pub approval_level: i32,
    pub status: BudgetStatus,
    pub comments: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetActual {
    pub base: BaseEntity,
    pub budget_line_id: Uuid,
    pub transaction_date: DateTime<Utc>,
    pub transaction_type: String,
    pub reference_id: Option<Uuid>,
    pub amount: Money,
    pub description: Option<String>,
    pub source_module: String,
}
