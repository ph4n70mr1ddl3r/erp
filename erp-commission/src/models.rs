use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CommissionBasis {
    Revenue,
    GrossMargin,
    NetMargin,
    Quantity,
    MRR,
    ARR,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CommissionFrequency {
    Monthly,
    Quarterly,
    Annually,
    PerDeal,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CommissionType {
    Percentage,
    Fixed,
    Tiered,
    Split,
    Draw,
    Bonus,
    Override,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionPlan {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub commission_type: CommissionType,
    pub basis: CommissionBasis,
    pub frequency: CommissionFrequency,
    pub default_rate: f64,
    pub min_rate: Option<f64>,
    pub max_rate: Option<f64>,
    pub cap_amount: Option<i64>,
    pub clawback_period_days: i32,
    pub effective_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionTier {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub tier_name: String,
    pub min_amount: i64,
    pub max_amount: Option<i64>,
    pub rate: f64,
    pub is_accelerator: bool,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionSplit {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub name: String,
    pub split_type: String,
    pub participants: Vec<CommissionSplitParticipant>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionSplitParticipant {
    pub id: Uuid,
    pub split_id: Uuid,
    pub sales_rep_id: Uuid,
    pub split_percent: f64,
    pub role: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesRepAssignment {
    pub id: Uuid,
    pub sales_rep_id: Uuid,
    pub plan_id: Uuid,
    pub territory_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub effective_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub is_primary: bool,
    pub split_percent: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CalculationStatus {
    Pending,
    Calculating,
    Completed,
    Approved,
    Paid,
    Adjusted,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionCalculation {
    pub id: Uuid,
    pub calculation_number: String,
    pub sales_rep_id: Uuid,
    pub plan_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub gross_revenue: i64,
    pub returns: i64,
    pub net_revenue: i64,
    pub cost_of_goods: i64,
    pub gross_margin: i64,
    pub base_commission: i64,
    pub tier_bonus: i64,
    pub override_commission: i64,
    pub adjustments: i64,
    pub clawbacks: i64,
    pub total_commission: i64,
    pub status: CalculationStatus,
    pub calculated_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub approved_by: Option<Uuid>,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionLine {
    pub id: Uuid,
    pub calculation_id: Uuid,
    pub source_type: String,
    pub source_id: Uuid,
    pub transaction_date: DateTime<Utc>,
    pub customer_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub quantity: i64,
    pub revenue: i64,
    pub cost: i64,
    pub margin: i64,
    pub rate_applied: f64,
    pub commission_amount: i64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AdjustmentType {
    Manual,
    Dispute,
    Correction,
    Bonus,
    Penalty,
    Clawback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionAdjustment {
    pub id: Uuid,
    pub calculation_id: Uuid,
    pub adjustment_type: AdjustmentType,
    pub amount: i64,
    pub reason: String,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionPayment {
    pub id: Uuid,
    pub payment_number: String,
    pub calculation_id: Uuid,
    pub sales_rep_id: Uuid,
    pub payment_date: DateTime<Utc>,
    pub amount: i64,
    pub payment_method: String,
    pub reference: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionDraw {
    pub id: Uuid,
    pub sales_rep_id: Uuid,
    pub plan_id: Uuid,
    pub draw_type: String,
    pub amount: i64,
    pub frequency: String,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub balance: i64,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawTransaction {
    pub id: Uuid,
    pub draw_id: Uuid,
    pub transaction_type: String,
    pub amount: i64,
    pub balance_after: i64,
    pub period: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesQuota {
    pub id: Uuid,
    pub sales_rep_id: Uuid,
    pub quota_type: String,
    pub period: String,
    pub year: i32,
    pub quarter: Option<i32>,
    pub month: Option<i32>,
    pub target_amount: i64,
    pub stretch_amount: Option<i64>,
    pub territory_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuotaProgress {
    pub id: Uuid,
    pub quota_id: Uuid,
    pub as_of_date: DateTime<Utc>,
    pub achieved_amount: i64,
    pub percent_achieved: f64,
    pub forecast_amount: i64,
    pub gap_to_quota: i64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionDispute {
    pub id: Uuid,
    pub calculation_id: Uuid,
    pub sales_rep_id: Uuid,
    pub dispute_type: String,
    pub description: String,
    pub requested_amount: i64,
    pub status: String,
    pub resolution: Option<String>,
    pub resolved_amount: Option<i64>,
    pub resolved_by: Option<Uuid>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesTeam {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub manager_id: Option<Uuid>,
    pub parent_team_id: Option<Uuid>,
    pub description: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalesTeamMember {
    pub id: Uuid,
    pub team_id: Uuid,
    pub sales_rep_id: Uuid,
    pub role: String,
    pub effective_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverrideRule {
    pub id: Uuid,
    pub manager_id: Uuid,
    pub plan_id: Option<Uuid>,
    pub override_type: String,
    pub rate: f64,
    pub applies_to_team: bool,
    pub team_id: Option<Uuid>,
    pub effective_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionReport {
    pub id: Uuid,
    pub report_type: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub sales_rep_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
    pub total_revenue: i64,
    pub total_commission: i64,
    pub avg_commission_rate: f64,
    pub top_performer_id: Option<Uuid>,
    pub generated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommissionForecast {
    pub id: Uuid,
    pub sales_rep_id: Uuid,
    pub period: String,
    pub pipeline_revenue: i64,
    pub weighted_revenue: i64,
    pub forecast_commission: i64,
    pub confidence_level: f64,
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Spiff {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub product_id: Option<Uuid>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub reward_type: String,
    pub reward_amount: i64,
    pub qualification_criteria: String,
    pub max_payouts: Option<i32>,
    pub current_payouts: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpiffAchievement {
    pub id: Uuid,
    pub spiff_id: Uuid,
    pub sales_rep_id: Uuid,
    pub achieved_date: DateTime<Utc>,
    pub achievement_value: i64,
    pub payout_amount: i64,
    pub paid: bool,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}
