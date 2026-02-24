use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SpendCategory {
    DirectMaterials,
    IndirectMaterials,
    Services,
    Capital,
    Labor,
    Logistics,
    IT,
    Marketing,
    Facilities,
    Travel,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AnalysisPeriod {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Annually,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendCategoryTree {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub level: i32,
    pub path: String,
    pub is_leaf: bool,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendTransaction {
    pub id: Uuid,
    pub transaction_number: String,
    pub transaction_date: DateTime<Utc>,
    pub vendor_id: Uuid,
    pub category_id: Uuid,
    pub cost_center_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub gl_account_id: Option<Uuid>,
    pub amount: i64,
    pub currency: String,
    pub amount_base: i64,
    pub quantity: Option<i64>,
    pub unit_of_measure: Option<String>,
    pub source_type: String,
    pub source_id: Option<Uuid>,
    pub description: Option<String>,
    pub is_contracted: bool,
    pub contract_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendSummary {
    pub id: Uuid,
    pub period_type: AnalysisPeriod,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub category_id: Option<Uuid>,
    pub vendor_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub cost_center_id: Option<Uuid>,
    pub total_spend: i64,
    pub transaction_count: i64,
    pub avg_transaction: i64,
    pub min_transaction: i64,
    pub max_transaction: i64,
    pub contracted_spend: i64,
    pub uncontracted_spend: i64,
    pub maverick_spend: i64,
    pub savings_identified: i64,
    pub savings_realized: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorSpendAnalysis {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub analysis_period: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_spend: i64,
    pub invoice_count: i64,
    pub po_count: i64,
    pub contract_count: i64,
    pub spend_under_contract: i64,
    pub spend_off_contract: i64,
    pub avg_invoice_value: i64,
    pub payment_terms_avg: i32,
    pub early_payment_savings: i64,
    pub duplicate_spend: i64,
    pub category_breakdown: String,
    pub trend_percent: f64,
    pub market_share: f64,
    pub risk_score: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CategorySpendAnalysis {
    pub id: Uuid,
    pub category_id: Uuid,
    pub analysis_period: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_spend: i64,
    pub vendor_count: i64,
    pub top_vendor_spend: i64,
    pub top_vendor_share: f64,
    pub avg_contract_value: i64,
    pub contract_coverage: f64,
    pub savings_opportunity: i64,
    pub price_variance: f64,
    pub volume_trend: f64,
    pub supplier_concentration: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepartmentSpendAnalysis {
    pub id: Uuid,
    pub department_id: Uuid,
    pub analysis_period: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_spend: i64,
    pub budget: i64,
    pub variance: i64,
    pub variance_percent: f64,
    pub category_breakdown: String,
    pub top_vendors: String,
    pub maverick_spend: i64,
    pub maverick_percent: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendTrend {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub metric_type: String,
    pub period: String,
    pub value: i64,
    pub previous_value: i64,
    pub change_percent: f64,
    pub moving_avg_3m: i64,
    pub moving_avg_12m: i64,
    pub trend_direction: String,
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendForecast {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub forecast_date: DateTime<Utc>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub forecast_amount: i64,
    pub confidence_low: i64,
    pub confidence_high: i64,
    pub confidence_level: f64,
    pub method: String,
    pub assumptions: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaverickSpend {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub vendor_id: Uuid,
    pub category_id: Uuid,
    pub amount: i64,
    pub maverick_type: String,
    pub reason: String,
    pub preferred_vendor_id: Option<Uuid>,
    pub preferred_contract_id: Option<Uuid>,
    pub potential_savings: i64,
    pub status: String,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateSpend {
    pub id: Uuid,
    pub transaction_ids: String,
    pub vendor_id: Uuid,
    pub category_id: Option<Uuid>,
    pub total_amount: i64,
    pub duplicate_amount: i64,
    pub match_type: String,
    pub confidence: f64,
    pub status: String,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceVariance {
    pub id: Uuid,
    pub product_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub vendor_id: Uuid,
    pub standard_price: i64,
    pub actual_price: i64,
    pub variance_amount: i64,
    pub variance_percent: f64,
    pub quantity: i64,
    pub total_variance: i64,
    pub period: String,
    pub analysis_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContractCompliance {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub vendor_id: Uuid,
    pub category_id: Option<Uuid>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_spend: i64,
    pub contracted_spend: i64,
    pub compliance_rate: f64,
    pub off_contract_spend: i64,
    pub off_contract_transactions: i64,
    pub contract_utilization: f64,
    pub savings_achieved: i64,
    pub missed_savings: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavingsOpportunity {
    pub id: Uuid,
    pub opportunity_number: String,
    pub category_id: Uuid,
    pub vendor_id: Option<Uuid>,
    pub opportunity_type: String,
    pub description: String,
    pub current_spend: i64,
    pub potential_savings: i64,
    pub savings_percent: f64,
    pub effort_level: String,
    pub implementation_timeframe: String,
    pub status: String,
    pub owner_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub realized_at: Option<DateTime<Utc>>,
    pub realized_savings: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendKpi {
    pub id: Uuid,
    pub kpi_name: String,
    pub kpi_type: String,
    pub period: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub target_value: f64,
    pub actual_value: f64,
    pub variance: f64,
    pub variance_percent: f64,
    pub trend: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierRiskScore {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub score_date: DateTime<Utc>,
    pub financial_risk: f64,
    pub operational_risk: f64,
    pub compliance_risk: f64,
    pub geographic_risk: f64,
    pub concentration_risk: f64,
    pub overall_risk: f64,
    pub risk_category: String,
    pub mitigation_recommendations: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendBudget {
    pub id: Uuid,
    pub name: String,
    pub fiscal_year: i32,
    pub category_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub cost_center_id: Option<Uuid>,
    pub annual_budget: i64,
    pub q1_budget: i64,
    pub q2_budget: i64,
    pub q3_budget: i64,
    pub q4_budget: i64,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendBudgetActual {
    pub id: Uuid,
    pub budget_id: Uuid,
    pub period: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub budget_amount: i64,
    pub actual_amount: i64,
    pub variance: i64,
    pub variance_percent: f64,
    pub forecast_amount: i64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TailSpendAnalysis {
    pub id: Uuid,
    pub analysis_date: DateTime<Utc>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_spend: i64,
    pub tail_spend: i64,
    pub tail_percent: f64,
    pub tail_vendor_count: i64,
    pub tail_transaction_count: i64,
    pub avg_tail_transaction: i64,
    pub consolidation_opportunity: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TailVendor {
    pub id: Uuid,
    pub analysis_id: Uuid,
    pub vendor_id: Uuid,
    pub vendor_name: String,
    pub total_spend: i64,
    pub transaction_count: i64,
    pub is_tail: bool,
    pub consolidation_candidate: bool,
    pub recommended_action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendDashboard {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub owner_id: Option<Uuid>,
    pub filters: String,
    pub widgets: String,
    pub refresh_interval: i32,
    pub last_refreshed: Option<DateTime<Utc>>,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpendReport {
    pub id: Uuid,
    pub report_name: String,
    pub report_type: String,
    pub parameters: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub generated_by: Option<Uuid>,
    pub generated_at: DateTime<Utc>,
    pub file_path: Option<String>,
    pub file_format: String,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentTermAnalysis {
    pub id: Uuid,
    pub vendor_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_spend: i64,
    pub avg_payment_terms: f64,
    pub early_payment_count: i64,
    pub on_time_count: i64,
    pub late_payment_count: i64,
    pub early_payment_discounts: i64,
    pub late_payment_penalties: i64,
    pub working_capital_impact: i64,
    pub created_at: DateTime<Utc>,
}
