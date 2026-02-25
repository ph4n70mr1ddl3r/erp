use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LeaseType {
    Operating,
    Finance,
    ShortTerm,
    LowValue,
    Land,
    Building,
    Equipment,
    Vehicle,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LeaseStatus {
    Draft,
    PendingApproval,
    Active,
    Modified,
    UnderReview,
    Expired,
    Terminated,
    Renewed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentFrequency {
    Monthly,
    Quarterly,
    SemiAnnually,
    Annually,
    Weekly,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentTiming {
    InAdvance,
    InArrears,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EscalationType {
    Fixed,
    Percentage,
    CPI,
    MarketRate,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lease {
    pub base: BaseEntity,
    pub lease_number: String,
    pub name: String,
    pub description: Option<String>,
    pub lease_type: LeaseType,
    pub lessor_id: Uuid,
    pub lessee_id: Uuid,
    pub asset_id: Option<Uuid>,
    pub commencement_date: NaiveDate,
    pub end_date: NaiveDate,
    pub lease_term_months: i32,
    pub renewal_option: bool,
    pub renewal_term_months: Option<i32>,
    pub termination_option: bool,
    pub termination_notice_days: Option<i32>,
    pub purchase_option: bool,
    pub purchase_option_price: Option<i64>,
    pub fair_value_at_commencement: i64,
    pub residual_value_guarantee: Option<i64>,
    pub currency: String,
    pub discount_rate: f64,
    pub implicit_rate: Option<f64>,
    pub incremental_borrowing_rate: Option<f64>,
    pub initial_direct_costs: i64,
    pub lease_incentives: i64,
    pub decommissioning_provision: Option<i64>,
    pub status: LeaseStatus,
    pub classification_date: Option<NaiveDate>,
    pub classification_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeasePayment {
    pub base: BaseEntity,
    pub lease_id: Uuid,
    pub payment_number: String,
    pub payment_date: NaiveDate,
    pub due_date: NaiveDate,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub period_number: i32,
    pub fixed_payment: i64,
    pub variable_payment: i64,
    pub escalation_amount: i64,
    pub total_payment: i64,
    pub currency: String,
    pub payment_status: PaymentStatus,
    pub paid_date: Option<NaiveDate>,
    pub paid_amount: Option<i64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentStatus {
    Scheduled,
    Due,
    Paid,
    Overdue,
    Waived,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeasePaymentSchedule {
    pub base: BaseEntity,
    pub lease_id: Uuid,
    pub schedule_number: String,
    pub effective_date: NaiveDate,
    pub payment_frequency: PaymentFrequency,
    pub payment_timing: PaymentTiming,
    pub payment_day: Option<i32>,
    pub base_payment: i64,
    pub escalation_type: EscalationType,
    pub escalation_rate: Option<f64>,
    pub escalation_frequency_months: Option<i32>,
    pub first_escalation_date: Option<NaiveDate>,
    pub cap_amount: Option<i64>,
    pub floor_amount: Option<i64>,
    pub currency: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RightOfUseAsset {
    pub base: BaseEntity,
    pub lease_id: Uuid,
    pub asset_number: String,
    pub name: String,
    pub initial_cost: i64,
    pub accumulated_depreciation: i64,
    pub impairment_loss: i64,
    pub net_book_value: i64,
    pub depreciation_method: DepreciationMethod,
    pub useful_life_months: i32,
    pub residual_value: i64,
    pub depreciation_start_date: NaiveDate,
    pub depreciation_end_date: NaiveDate,
    pub currency: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DepreciationMethod {
    StraightLine,
    DecliningBalance,
    SumOfYearsDigits,
    UnitsOfProduction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseLiability {
    pub base: BaseEntity,
    pub lease_id: Uuid,
    pub liability_number: String,
    pub initial_liability: i64,
    pub outstanding_balance: i64,
    pub interest_accrued: i64,
    pub principal_paid: i64,
    pub currency: String,
    pub calculation_date: NaiveDate,
    pub amortization_method: AmortizationMethod,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AmortizationMethod {
    EffectiveInterest,
    StraightLine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseAmortizationSchedule {
    pub id: Uuid,
    pub lease_id: Uuid,
    pub period_number: i32,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub opening_liability: i64,
    pub payment: i64,
    pub interest_expense: i64,
    pub principal_reduction: i64,
    pub closing_liability: i64,
    pub opening_rou_asset: i64,
    pub depreciation_expense: i64,
    pub closing_rou_asset: i64,
    pub total_expense: i64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseModification {
    pub base: BaseEntity,
    pub modification_number: String,
    pub lease_id: Uuid,
    pub modification_date: NaiveDate,
    pub effective_date: NaiveDate,
    pub modification_type: ModificationType,
    pub reason: String,
    pub original_term_months: i32,
    pub new_term_months: i32,
    pub original_payment: i64,
    pub new_payment: i64,
    pub original_discount_rate: f64,
    pub new_discount_rate: Option<f64>,
    pub remeasurement_gain_loss: i64,
    pub rou_adjustment: i64,
    pub liability_adjustment: i64,
    pub currency: String,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub status: ModificationStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ModificationType {
    TermExtension,
    TermReduction,
    ScopeChange,
    PaymentChange,
    RateChange,
    PartialTermination,
    FullTermination,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ModificationStatus {
    Draft,
    PendingApproval,
    Approved,
    Rejected,
    Processed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseExpense {
    pub base: BaseEntity,
    pub lease_id: Uuid,
    pub expense_date: NaiveDate,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub depreciation_expense: i64,
    pub interest_expense: i64,
    pub variable_lease_expense: i64,
    pub short_term_lease_expense: i64,
    pub low_value_lease_expense: i64,
    pub total_expense: i64,
    pub currency: String,
    pub journal_entry_id: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseDisclosure {
    pub base: BaseEntity,
    pub reporting_period: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_finance_leases: i32,
    pub total_operating_leases: i32,
    pub total_rou_assets: i64,
    pub total_lease_liabilities: i64,
    pub total_depreciation: i64,
    pub total_interest: i64,
    pub total_lease_payments: i64,
    pub maturities_within_1_year: i64,
    pub maturities_1_to_5_years: i64,
    pub maturities_after_5_years: i64,
    pub total_undiscounted_payments: i64,
    pub weighted_avg_lease_term: f64,
    pub weighted_avg_discount_rate: f64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseAssetClass {
    pub base: BaseEntity,
    pub class_code: String,
    pub name: String,
    pub description: Option<String>,
    pub default_useful_life_months: i32,
    pub depreciation_method: DepreciationMethod,
    pub residual_value_percent: f64,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
