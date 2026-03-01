use chrono::{DateTime, Utc};
use erp_core::Status;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CompensationType {
    BaseSalary,
    Hourly,
    Commission,
    Bonus,
    Equity,
    Benefits,
    Allowance,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReviewCycle {
    Annual,
    SemiAnnual,
    Quarterly,
    Monthly,
    OffCycle,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AdjustmentType {
    Merit,
    Promotion,
    CostOfLiving,
    MarketAdjustment,
    EquityAdjustment,
    CounterOffer,
    Retention,
    SignOnBonus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationPlan {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub plan_year: i32,
    pub effective_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub budget_amount: i64,
    pub allocated_amount: i64,
    pub spent_amount: i64,
    pub review_cycle: ReviewCycle,
    pub default_merit_budget_percent: f64,
    pub max_merit_percent: f64,
    pub promotion_budget_percent: f64,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationBudget {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub department_id: Option<Uuid>,
    pub cost_center_id: Option<Uuid>,
    pub budget_type: String,
    pub allocated_amount: i64,
    pub committed_amount: i64,
    pub spent_amount: i64,
    pub remaining_amount: i64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryRange {
    pub id: Uuid,
    pub grade_id: Option<Uuid>,
    pub job_family_id: Option<Uuid>,
    pub name: String,
    pub code: String,
    pub min_salary: i64,
    pub mid_salary: i64,
    pub max_salary: i64,
    pub currency: String,
    pub effective_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobGrade {
    pub id: Uuid,
    pub grade_code: String,
    pub grade_name: String,
    pub description: Option<String>,
    pub level: i32,
    pub job_family_id: Option<Uuid>,
    pub salary_range_id: Option<Uuid>,
    pub min_experience_years: i32,
    pub typical_title: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobFamily {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub department_id: Option<Uuid>,
    pub parent_family_id: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeCompensation {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub effective_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub base_salary: i64,
    pub hourly_rate: Option<i64>,
    pub pay_frequency: String,
    pub currency: String,
    pub grade_id: Option<Uuid>,
    pub salary_range_id: Option<Uuid>,
    pub compa_ratio: f64,
    pub position_in_range: f64,
    pub is_current: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationComponent {
    pub id: Uuid,
    pub employee_compensation_id: Uuid,
    pub component_type: CompensationType,
    pub name: String,
    pub amount: i64,
    pub frequency: String,
    pub is_recurring: bool,
    pub effective_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationAdjustment {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub plan_id: Option<Uuid>,
    pub adjustment_type: AdjustmentType,
    pub current_base: i64,
    pub new_base: i64,
    pub adjustment_amount: i64,
    pub adjustment_percent: f64,
    pub effective_date: DateTime<Utc>,
    pub reason: String,
    pub justification: Option<String>,
    pub old_grade_id: Option<Uuid>,
    pub new_grade_id: Option<Uuid>,
    pub old_position_id: Option<Uuid>,
    pub new_position_id: Option<Uuid>,
    pub status: String,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeritMatrix {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub performance_levels: i32,
    pub compa_ratio_buckets: i32,
    pub matrix_data: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeritGuideline {
    pub id: Uuid,
    pub matrix_id: Uuid,
    pub performance_rating: i32,
    pub compa_ratio_min: f64,
    pub compa_ratio_max: f64,
    pub recommended_increase_min: f64,
    pub recommended_increase_mid: f64,
    pub recommended_increase_max: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BonusPlan {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub plan_type: String,
    pub target_percent: f64,
    pub max_percent: f64,
    pub funding_formula: String,
    pub performance_weights: String,
    pub fiscal_year: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeBonus {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub bonus_plan_id: Uuid,
    pub fiscal_year: i32,
    pub target_amount: i64,
    pub company_performance_factor: f64,
    pub individual_performance_factor: f64,
    pub calculated_amount: i64,
    pub recommended_amount: i64,
    pub approved_amount: Option<i64>,
    pub status: String,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityGrant {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub grant_number: String,
    pub grant_type: String,
    pub shares: i64,
    pub strike_price: i64,
    pub grant_date: DateTime<Utc>,
    pub vest_start_date: DateTime<Utc>,
    pub vest_schedule: String,
    pub vesting_years: i32,
    pub cliff_months: i32,
    pub vested_shares: i64,
    pub unvested_shares: i64,
    pub forfeited_shares: i64,
    pub expiration_date: DateTime<Utc>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EquityVestingEvent {
    pub id: Uuid,
    pub grant_id: Uuid,
    pub vest_date: DateTime<Utc>,
    pub shares: i64,
    pub cumulative_shares: i64,
    pub status: String,
    pub processed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketData {
    pub id: Uuid,
    pub source: String,
    pub survey_date: DateTime<Utc>,
    pub job_code: String,
    pub job_title: String,
    pub industry: Option<String>,
    pub region: Option<String>,
    pub company_size: Option<String>,
    pub p10: i64,
    pub p25: i64,
    pub p50: i64,
    pub p75: i64,
    pub p90: i64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MarketDataMapping {
    pub id: Uuid,
    pub position_id: Uuid,
    pub market_data_id: Uuid,
    pub match_quality: f64,
    pub effective_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationReview {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub employee_id: Uuid,
    pub reviewer_id: Uuid,
    pub review_date: DateTime<Utc>,
    pub current_salary: i64,
    pub proposed_salary: i64,
    pub proposed_increase_percent: f64,
    pub performance_rating: Option<i32>,
    pub potential_rating: Option<i32>,
    pub compa_ratio_current: f64,
    pub compa_ratio_proposed: f64,
    pub merit_recommendation: String,
    pub promotion_recommendation: bool,
    pub retention_risk: String,
    pub comments: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationApproval {
    pub id: Uuid,
    pub review_id: Uuid,
    pub approver_id: Uuid,
    pub approval_level: i32,
    pub original_amount: i64,
    pub approved_amount: i64,
    pub status: String,
    pub comments: Option<String>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationHistory {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub effective_date: DateTime<Utc>,
    pub change_type: String,
    pub previous_value: i64,
    pub new_value: i64,
    pub change_amount: i64,
    pub change_percent: f64,
    pub reason: String,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TotalRewardsStatement {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub statement_year: i32,
    pub base_salary: i64,
    pub variable_pay: i64,
    pub benefits_value: i64,
    pub equity_value: i64,
    pub other_compensation: i64,
    pub total_compensation: i64,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayEquityAnalysis {
    pub id: Uuid,
    pub analysis_date: DateTime<Utc>,
    pub analysis_type: String,
    pub group_type: String,
    pub group_id: Option<Uuid>,
    pub employee_count: i32,
    pub avg_salary_male: i64,
    pub avg_salary_female: i64,
    pub pay_gap_percent: f64,
    pub adjusted_gap_percent: f64,
    pub statistical_significance: f64,
    pub findings: Option<String>,
    pub recommendations: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationBenchmark {
    pub id: Uuid,
    pub employee_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub current_salary: i64,
    pub market_p50: i64,
    pub market_p75: i64,
    pub market_p90: i64,
    pub compa_ratio_p50: f64,
    pub percentile: f64,
    pub benchmark_date: DateTime<Utc>,
    pub market_data_source: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetentionAllowance {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub allowance_type: String,
    pub amount: i64,
    pub currency: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub clawback_period_months: i32,
    pub payment_schedule: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeographicDifferential {
    pub id: Uuid,
    pub location_id: Uuid,
    pub location_name: String,
    pub differential_percent: f64,
    pub effective_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}
