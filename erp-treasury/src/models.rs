use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CashPoolType {
    ZeroBalance,
    NotionalPooling,
    PhysicalPooling,
    Sweep,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashPool {
    pub base: BaseEntity,
    pub pool_number: String,
    pub name: String,
    pub description: Option<String>,
    pub pool_type: CashPoolType,
    pub header_account_id: Uuid,
    pub currency: String,
    pub target_balance: i64,
    pub min_balance: i64,
    pub max_balance: Option<i64>,
    pub interest_allocation_method: Option<String>,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashPoolMember {
    pub id: Uuid,
    pub pool_id: Uuid,
    pub bank_account_id: Uuid,
    pub company_id: Uuid,
    pub participation_percent: f64,
    pub contribution_limit: Option<i64>,
    pub is_header: bool,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashPosition {
    pub base: BaseEntity,
    pub position_date: NaiveDate,
    pub currency: String,
    pub opening_balance: i64,
    pub receipts: i64,
    pub disbursements: i64,
    pub transfers_in: i64,
    pub transfers_out: i64,
    pub fx_gains: i64,
    pub fx_losses: i64,
    pub closing_balance: i64,
    pub available_balance: i64,
    pub invested_balance: i64,
    pub borrowed_balance: i64,
    pub net_position: i64,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashPositionDetail {
    pub id: Uuid,
    pub position_id: Uuid,
    pub bank_account_id: Uuid,
    pub company_id: Uuid,
    pub opening_balance: i64,
    pub closing_balance: i64,
    pub available_balance: i64,
    pub float_balance: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashForecast {
    pub base: BaseEntity,
    pub forecast_number: String,
    pub forecast_date: NaiveDate,
    pub horizon_days: i32,
    pub currency: String,
    pub company_id: Option<Uuid>,
    pub scenario: ForecastScenario,
    pub status: ForecastStatus,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ForecastScenario {
    Baseline,
    Optimistic,
    Pessimistic,
    WorstCase,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ForecastStatus {
    Draft,
    Approved,
    Final,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashForecastLine {
    pub id: Uuid,
    pub forecast_id: Uuid,
    pub forecast_date: NaiveDate,
    pub cash_flow_type: CashFlowType,
    pub category: String,
    pub subcategory: Option<String>,
    pub amount: i64,
    pub probability: i32,
    pub expected_amount: i64,
    pub source_type: Option<String>,
    pub source_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CashFlowType {
    Inflow,
    Outflow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentPolicy {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub max_single_investment: i64,
    pub max_counterparty_exposure: i64,
    pub min_credit_rating: String,
    pub allowed_instrument_types: String,
    pub max_duration_days: i32,
    pub min_duration_days: i32,
    pub liquidity_requirement_percent: i32,
    pub max_foreign_exposure_percent: i32,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum InvestmentType {
    MoneyMarket,
    TreasuryBill,
    CommercialPaper,
    CertificateOfDeposit,
    GovernmentBond,
    CorporateBond,
    RepurchaseAgreement,
    BankDeposit,
    MutualFund,
    ETF,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum InvestmentStatus {
    Pending,
    Active,
    Matured,
    Called,
    Sold,
    Defaulted,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Investment {
    pub base: BaseEntity,
    pub investment_number: String,
    pub name: String,
    pub investment_type: InvestmentType,
    pub issuer: String,
    pub currency: String,
    pub principal_amount: i64,
    pub current_value: i64,
    pub purchase_date: NaiveDate,
    pub maturity_date: NaiveDate,
    pub purchase_price: i64,
    pub coupon_rate: Option<f64>,
    pub yield_to_maturity: Option<f64>,
    pub yield_current: Option<f64>,
    pub credit_rating: Option<String>,
    pub cusip: Option<String>,
    pub isin: Option<String>,
    pub counterparty_id: Option<Uuid>,
    pub custodian: Option<String>,
    pub account_id: Option<Uuid>,
    pub accrued_interest: i64,
    pub unrealized_gain_loss: i64,
    pub realized_gain_loss: i64,
    pub day_count_convention: String,
    pub payment_frequency: String,
    pub next_payment_date: Option<NaiveDate>,
    pub call_date: Option<NaiveDate>,
    pub call_price: Option<i64>,
    pub put_date: Option<NaiveDate>,
    pub put_price: Option<i64>,
    pub status: InvestmentStatus,
    pub policy_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InvestmentTransaction {
    pub base: BaseEntity,
    pub investment_id: Uuid,
    pub transaction_type: InvestmentTransactionType,
    pub transaction_date: NaiveDate,
    pub settlement_date: Option<NaiveDate>,
    pub quantity: i64,
    pub unit_price: i64,
    pub principal_amount: i64,
    pub accrued_interest: i64,
    pub fees: i64,
    pub total_amount: i64,
    pub realized_gain_loss: i64,
    pub counterparty_id: Option<Uuid>,
    pub reference: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum InvestmentTransactionType {
    Purchase,
    Sale,
    Maturity,
    InterestPayment,
    PrincipalPayment,
    Call,
    Put,
    Amortization,
    ValuationAdjustment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditFacility {
    pub base: BaseEntity,
    pub facility_number: String,
    pub name: String,
    pub facility_type: CreditFacilityType,
    pub lender_id: Uuid,
    pub currency: String,
    pub committed_amount: i64,
    pub available_amount: i64,
    pub drawn_amount: i64,
    pub undrawn_amount: i64,
    pub interest_rate_type: InterestRateType,
    pub base_rate: Option<String>,
    pub margin_rate: f64,
    pub all_in_rate: Option<f64>,
    pub commitment_fee: f64,
    pub facility_fee: i64,
    pub utilization_fee: Option<f64>,
    pub effective_date: NaiveDate,
    pub maturity_date: NaiveDate,
    pub renewal_date: Option<NaiveDate>,
    pub financial_covenants: Option<String>,
    pub borrowing_base_formula: Option<String>,
    pub status: CreditFacilityStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CreditFacilityType {
    RevolvingCredit,
    TermLoan,
    LineOfCredit,
    AssetBased,
    CashPool,
    Syndicated,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum InterestRateType {
    Fixed,
    Floating,
    Variable,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CreditFacilityStatus {
    Pending,
    Active,
    Suspended,
    Terminated,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditDrawdown {
    pub base: BaseEntity,
    pub drawdown_number: String,
    pub facility_id: Uuid,
    pub drawdown_date: NaiveDate,
    pub value_date: NaiveDate,
    pub amount: i64,
    pub currency: String,
    pub interest_rate: f64,
    pub interest_start_date: NaiveDate,
    pub maturity_date: Option<NaiveDate>,
    pub purpose: Option<String>,
    pub status: CreditDrawdownStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CreditDrawdownStatus {
    Requested,
    Approved,
    Funded,
    Repaid,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditRepayment {
    pub base: BaseEntity,
    pub repayment_number: String,
    pub drawdown_id: Uuid,
    pub facility_id: Uuid,
    pub repayment_date: NaiveDate,
    pub value_date: NaiveDate,
    pub principal_amount: i64,
    pub interest_amount: i64,
    pub total_amount: i64,
    pub currency: String,
    pub status: CreditRepaymentStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CreditRepaymentStatus {
    Scheduled,
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum HedgeType {
    Forward,
    Future,
    Option,
    Swap,
    Collar,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum HedgePurpose {
    FxExposure,
    InterestRate,
    Commodity,
    Equity,
    Credit,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum HedgeStatus {
    Proposed,
    Approved,
    Active,
    Matured,
    Terminated,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum HedgeEffectiveness {
    HighlyEffective,
    Effective,
    PartiallyEffective,
    Ineffective,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HedgeInstrument {
    pub base: BaseEntity,
    pub hedge_number: String,
    pub name: String,
    pub hedge_type: HedgeType,
    pub hedge_purpose: HedgePurpose,
    pub underlying: String,
    pub notional_amount: i64,
    pub notional_currency: String,
    pub counter_currency: Option<String>,
    pub strike_price: Option<f64>,
    pub forward_rate: Option<f64>,
    pub spot_rate: Option<f64>,
    pub premium: Option<i64>,
    pub premium_currency: Option<String>,
    pub trade_date: NaiveDate,
    pub effective_date: NaiveDate,
    pub maturity_date: NaiveDate,
    pub settlement_date: Option<NaiveDate>,
    pub counterparty_id: Option<Uuid>,
    pub hedge_accounting: bool,
    pub effectiveness_method: Option<String>,
    pub effectiveness_status: Option<HedgeEffectiveness>,
    pub designation_document: Option<String>,
    pub fair_value: Option<i64>,
    pub unrealized_gain_loss: Option<i64>,
    pub realized_gain_loss: Option<i64>,
    pub status: HedgeStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HedgeRelationship {
    pub base: BaseEntity,
    pub relationship_number: String,
    pub name: String,
    pub hedge_instrument_id: Uuid,
    pub hedged_item_type: String,
    pub hedged_item_id: Option<Uuid>,
    pub hedged_description: String,
    pub hedged_amount: i64,
    pub hedge_ratio: f64,
    pub risk_type: String,
    pub effectiveness_test_frequency: String,
    pub effectiveness_threshold_min: f64,
    pub effectiveness_threshold_max: f64,
    pub status: HedgeStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HedgeTest {
    pub base: BaseEntity,
    pub relationship_id: Uuid,
    pub test_date: NaiveDate,
    pub test_method: String,
    pub hedge_value_change: i64,
    pub hedged_value_change: i64,
    pub effectiveness_ratio: f64,
    pub is_effective: bool,
    pub cumulative_effectiveness: f64,
    valtest_notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentMethod {
    WireTransfer,
    ACH,
    Check,
    Draft,
    Card,
    RTP,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentStatus {
    Draft,
    Submitted,
    Approved,
    Processing,
    Completed,
    Failed,
    Cancelled,
    Returned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentFile {
    pub base: BaseEntity,
    pub file_number: String,
    pub file_type: String,
    pub bank_id: Uuid,
    pub file_date: NaiveDate,
    pub value_date: NaiveDate,
    pub currency: String,
    pub total_amount: i64,
    pub payment_count: i32,
    pub file_name: Option<String>,
    pub file_content: Option<String>,
    pub status: PaymentFileStatus,
    pub submitted_at: Option<DateTime<Utc>>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentFileStatus {
    Created,
    Validated,
    Submitted,
    Acknowledged,
    Processed,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccountBalance {
    pub base: BaseEntity,
    pub bank_account_id: Uuid,
    pub balance_date: NaiveDate,
    pub opening_balance: i64,
    pub closing_balance: i64,
    pub available_balance: i64,
    pub current_balance: i64,
    pub float_balance: i64,
    pub hold_balance: i64,
    pub average_balance: i64,
    pub currency: String,
    pub last_statement_balance: Option<i64>,
    pub last_statement_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryLimit {
    pub base: BaseEntity,
    pub limit_type: TreasuryLimitType,
    pub name: String,
    pub description: Option<String>,
    pub currency: String,
    pub limit_amount: i64,
    pub utilized_amount: i64,
    pub available_amount: i64,
    pub counterparty_id: Option<Uuid>,
    pub instrument_type: Option<String>,
    pub effective_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub approval_required: bool,
    pub alert_threshold_percent: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TreasuryLimitType {
    CounterpartyExposure,
    SingleTransaction,
    DailyLimit,
    CurrencyExposure,
    InvestmentConcentration,
    BorrowingLimit,
}
