use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueContract {
    pub id: Uuid,
    pub contract_number: String,
    pub customer_id: Uuid,
    pub contract_date: NaiveDate,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_value: i64,
    pub currency: String,
    pub status: ContractStatus,
    pub performance_obligations: Vec<PerformanceObligation>,
    pub transaction_price: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractStatus {
    Draft,
    Active,
    Completed,
    Cancelled,
    Modified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceObligation {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub standalone_price: i64,
    pub allocated_price: i64,
    pub recognition_type: RecognitionType,
    pub recognition_method: RecognitionMethod,
    pub total_periods: i32,
    pub status: ObligationStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecognitionType {
    PointInTime,
    OverTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecognitionMethod {
    StraightLine,
    PercentageComplete,
    OutputMethod,
    InputMethod,
    Milestones,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObligationStatus {
    NotStarted,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueSchedule {
    pub id: Uuid,
    pub obligation_id: Uuid,
    pub period: NaiveDate,
    pub planned_revenue: i64,
    pub recognized_revenue: i64,
    pub deferred_revenue: i64,
    pub status: ScheduleStatus,
    pub recognized_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ScheduleStatus {
    Pending,
    Recognized,
    PartiallyRecognized,
    Reversed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractModification {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub modification_date: NaiveDate,
    pub modification_type: ModificationType,
    pub description: String,
    pub price_change: i64,
    pub new_total_value: i64,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModificationType {
    Addition,
    Deletion,
    PriceChange,
    TermExtension,
    ScopeChange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeferredRevenue {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub obligation_id: Uuid,
    pub account_id: Uuid,
    pub original_amount: i64,
    pub recognized_amount: i64,
    pub remaining_amount: i64,
    pub currency: String,
    pub recognition_start: NaiveDate,
    pub recognition_end: NaiveDate,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueEvent {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub obligation_id: Option<Uuid>,
    pub event_type: RevenueEventType,
    pub amount: i64,
    pub currency: String,
    pub event_date: NaiveDate,
    pub period: NaiveDate,
    pub description: Option<String>,
    pub journal_entry_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RevenueEventType {
    ContractSigned,
    MilestoneComplete,
    ServiceDelivered,
    PeriodRecognition,
    Modification,
    Refund,
    Adjustment,
    Reversal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllocationRule {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub method: AllocationMethod,
    pub basis: AllocationBasis,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllocationMethod {
    StandalonePrice,
    Residual,
    AdjustedMarketAssessment,
    ExpectedCostPlusMargin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AllocationBasis {
    RelativeStandalonePrice,
    RelativeSellingPrice,
    Equal,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vatb {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub calculation_date: NaiveDate,
    pub total_complete_percent: f64,
    pub costs_incurred: i64,
    pub total_estimated_costs: i64,
    pub revenue_to_date: i64,
    pub revenue_to_recognize: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateContractRequest {
    pub customer_id: Uuid,
    pub contract_date: NaiveDate,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_value: i64,
    pub currency: String,
    pub obligations: Vec<CreateObligationRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateObligationRequest {
    pub name: String,
    pub description: Option<String>,
    pub standalone_price: i64,
    pub recognition_type: RecognitionType,
    pub recognition_method: RecognitionMethod,
    pub total_periods: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecognizeRevenueRequest {
    pub contract_id: Uuid,
    pub obligation_id: Option<Uuid>,
    pub amount: i64,
    pub event_type: RevenueEventType,
    pub event_date: NaiveDate,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueWaterfall {
    pub contract_id: Uuid,
    pub periods: Vec<WaterfallPeriod>,
    pub total_contract_value: i64,
    pub total_recognized: i64,
    pub total_deferred: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterfallPeriod {
    pub period: NaiveDate,
    pub beginning_deferred: i64,
    pub new_revenue: i64,
    pub recognized: i64,
    pub ending_deferred: i64,
}
