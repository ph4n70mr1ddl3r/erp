use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Account {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub account_type: AccountType,
    pub parent_id: Option<Uuid>,
    pub status: Status,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AccountType {
    Asset,
    Liability,
    Equity,
    Revenue,
    Expense,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalEntry {
    pub base: BaseEntity,
    pub entry_number: String,
    pub date: DateTime<Utc>,
    pub description: String,
    pub reference: Option<String>,
    pub lines: Vec<JournalLine>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JournalLine {
    pub id: Uuid,
    pub account_id: Uuid,
    pub debit: Money,
    pub credit: Money,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FiscalYear {
    pub base: BaseEntity,
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxRate {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub rate: f64,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub base: BaseEntity,
    pub name: String,
    pub fiscal_year_id: Uuid,
    pub lines: Vec<BudgetLine>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetLine {
    pub id: Uuid,
    pub account_id: Uuid,
    pub period: u32,
    pub amount: Money,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountBalance {
    pub account_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub account_type: AccountType,
    pub balance: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalanceSheet {
    pub as_of_date: DateTime<Utc>,
    pub assets: Vec<AccountBalance>,
    pub total_assets: i64,
    pub liabilities: Vec<AccountBalance>,
    pub total_liabilities: i64,
    pub equity: Vec<AccountBalance>,
    pub total_equity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfitAndLoss {
    pub from_date: DateTime<Utc>,
    pub to_date: DateTime<Utc>,
    pub revenue: Vec<AccountBalance>,
    pub total_revenue: i64,
    pub expenses: Vec<AccountBalance>,
    pub total_expenses: i64,
    pub net_income: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialBalance {
    pub as_of_date: DateTime<Utc>,
    pub accounts: Vec<TrialBalanceLine>,
    pub total_debits: i64,
    pub total_credits: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrialBalanceLine {
    pub account_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub debit: i64,
    pub credit: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyDef {
    pub code: String,
    pub name: String,
    pub symbol: String,
    pub is_base: bool,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub id: Uuid,
    pub from_currency: String,
    pub to_currency: String,
    pub rate: f64,
    pub effective_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetWithVariance {
    pub base: BaseEntity,
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub total_amount: i64,
    pub total_actual: i64,
    pub total_variance: i64,
    pub variance_percent: f64,
    pub status: Status,
    pub lines: Vec<BudgetLineWithVariance>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BudgetLineWithVariance {
    pub id: Uuid,
    pub account_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub period: u32,
    pub budget_amount: i64,
    pub actual_amount: i64,
    pub variance: i64,
    pub variance_percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FixedAsset {
    pub id: Uuid,
    pub asset_code: String,
    pub name: String,
    pub description: Option<String>,
    pub category: String,
    pub location: Option<String>,
    pub cost: i64,
    pub salvage_value: i64,
    pub useful_life_years: i32,
    pub depreciation_method: DepreciationMethod,
    pub acquisition_date: DateTime<Utc>,
    pub depreciation_start_date: Option<DateTime<Utc>>,
    pub accumulated_depreciation: i64,
    pub net_book_value: i64,
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
pub struct AssetDepreciation {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub period: String,
    pub depreciation_amount: i64,
    pub accumulated_depreciation: i64,
    pub posted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankAccount {
    pub id: Uuid,
    pub account_id: Uuid,
    pub bank_name: String,
    pub account_number: String,
    pub account_type: BankAccountType,
    pub currency: String,
    pub gl_code: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BankAccountType {
    Checking,
    Savings,
    MoneyMarket,
    CreditCard,
    Loan,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankStatement {
    pub id: Uuid,
    pub bank_account_id: Uuid,
    pub statement_date: DateTime<Utc>,
    pub opening_balance: i64,
    pub closing_balance: i64,
    pub status: Status,
    pub reconciled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BankTransaction {
    pub id: Uuid,
    pub bank_account_id: Uuid,
    pub statement_id: Option<Uuid>,
    pub transaction_date: DateTime<Utc>,
    pub value_date: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub debit: i64,
    pub credit: i64,
    pub balance: i64,
    pub reconciled: bool,
    pub journal_entry_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReconciliationRule {
    pub id: Uuid,
    pub bank_account_id: Uuid,
    pub rule_type: ReconciliationRuleType,
    pub match_field: String,
    pub match_pattern: Option<String>,
    pub tolerance_days: i32,
    pub tolerance_amount: i64,
    pub auto_match: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReconciliationRuleType {
    ExactMatch,
    FuzzyMatch,
    AmountRange,
    DateRange,
    PatternMatch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashFlowForecast {
    pub id: Uuid,
    pub forecast_date: DateTime<Utc>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub opening_balance: i64,
    pub expected_inflows: i64,
    pub expected_outflows: i64,
    pub closing_balance: i64,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashFlowCategory {
    pub id: Uuid,
    pub name: String,
    pub category_type: CashFlowCategoryType,
    pub parent_id: Option<Uuid>,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CashFlowCategoryType {
    Operating,
    Investing,
    Financing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashFlowItem {
    pub id: Uuid,
    pub forecast_id: Uuid,
    pub category_id: Uuid,
    pub description: String,
    pub expected_date: Option<DateTime<Utc>>,
    pub amount: i64,
    pub probability: i32,
    pub actual_amount: Option<i64>,
    pub actual_date: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostCenter {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub department_id: Option<Uuid>,
    pub manager_id: Option<Uuid>,
    pub cost_center_type: CostCenterType,
    pub allocation_method: AllocationMethod,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CostCenterType {
    Production,
    Service,
    Administrative,
    Sales,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AllocationMethod {
    Direct,
    StepDown,
    Reciprocal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostElement {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub element_type: CostElementType,
    pub account_id: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CostElementType {
    Material,
    Labor,
    Overhead,
    Service,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostPool {
    pub id: Uuid,
    pub name: String,
    pub cost_center_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_cost: i64,
    pub allocation_base: String,
    pub allocation_rate: f64,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostAllocation {
    pub id: Uuid,
    pub pool_id: Uuid,
    pub from_cost_center_id: Uuid,
    pub to_cost_center_id: Uuid,
    pub allocation_base_value: f64,
    pub allocated_amount: i64,
    pub allocated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityType {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub cost_driver: String,
    pub unit_of_measure: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityCost {
    pub id: Uuid,
    pub activity_type_id: Uuid,
    pub cost_pool_id: Uuid,
    pub total_activities: i64,
    pub cost_per_activity: i64,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub legal_name: Option<String>,
    pub tax_id: Option<String>,
    pub registration_number: Option<String>,
    pub currency: String,
    pub address: Option<String>,
    pub city: Option<String>,
    pub country: Option<String>,
    pub is_consolidation_entity: bool,
    pub parent_company_id: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntercompanyTransaction {
    pub id: Uuid,
    pub transaction_number: String,
    pub from_company_id: Uuid,
    pub to_company_id: Uuid,
    pub transaction_date: DateTime<Utc>,
    pub amount: i64,
    pub currency: String,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub from_journal_entry_id: Option<Uuid>,
    pub to_journal_entry_id: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntercompanyAccount {
    pub id: Uuid,
    pub company_id: Uuid,
    pub partner_company_id: Uuid,
    pub account_id: Uuid,
    pub due_to_account_id: Uuid,
    pub due_from_account_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueSchedule {
    pub id: Uuid,
    pub schedule_number: String,
    pub name: String,
    pub recognition_method: RecognitionMethod,
    pub total_amount: i64,
    pub recognized_amount: i64,
    pub deferred_amount: i64,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RecognitionMethod {
    StraightLine,
    PercentageOfCompletion,
    CompletedContract,
    Installment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueScheduleLine {
    pub id: Uuid,
    pub schedule_id: Uuid,
    pub line_number: i32,
    pub recognition_date: DateTime<Utc>,
    pub amount: i64,
    pub recognized: bool,
    pub journal_entry_id: Option<Uuid>,
    pub recognized_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RevenueRecognitionTemplate {
    pub id: Uuid,
    pub name: String,
    pub recognition_type: RecognitionType,
    pub periods: i32,
    pub recognition_rule: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RecognitionType {
    Monthly,
    Quarterly,
    Yearly,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationSchedule {
    pub id: Uuid,
    pub name: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub parent_company_id: Uuid,
    pub status: Status,
    pub elimination_entries: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationCompany {
    pub id: Uuid,
    pub consolidation_id: Uuid,
    pub company_id: Uuid,
    pub ownership_percent: f64,
    pub consolidation_method: ConsolidationMethod,
    pub exchange_rate: f64,
    pub translation_method: TranslationMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ConsolidationMethod {
    Full,
    Equity,
    Proportional,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TranslationMethod {
    Current,
    Temporal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EliminationRule {
    pub id: Uuid,
    pub name: String,
    pub from_account_pattern: String,
    pub to_account_pattern: String,
    pub elimination_account_id: Uuid,
    pub description: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EliminationEntry {
    pub id: Uuid,
    pub consolidation_id: Uuid,
    pub elimination_rule_id: Option<Uuid>,
    pub description: String,
    pub debit_account_id: Uuid,
    pub credit_account_id: Uuid,
    pub amount: i64,
    pub journal_entry_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DunningLevel {
    Reminder,
    FirstNotice,
    SecondNotice,
    FinalNotice,
    Collection,
    Legal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DunningPolicy {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub levels: Vec<DunningLevelConfig>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DunningLevelConfig {
    pub id: Uuid,
    pub policy_id: Uuid,
    pub level: DunningLevel,
    pub days_overdue: i32,
    pub fee_percent: f64,
    pub fee_fixed: i64,
    pub template_id: Option<Uuid>,
    pub stop_services: bool,
    pub send_email: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DunningRun {
    pub id: Uuid,
    pub run_number: String,
    pub policy_id: Uuid,
    pub run_date: DateTime<Utc>,
    pub status: DunningRunStatus,
    pub customers_processed: i32,
    pub total_amount: i64,
    pub total_fees: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DunningRunStatus {
    Draft,
    Running,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DunningLetter {
    pub id: Uuid,
    pub run_id: Uuid,
    pub customer_id: Uuid,
    pub level: DunningLevel,
    pub letter_date: DateTime<Utc>,
    pub invoice_ids: Vec<Uuid>,
    pub invoice_amount: i64,
    pub fee_amount: i64,
    pub total_amount: i64,
    pub sent_at: Option<DateTime<Utc>>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub status: DunningLetterStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DunningLetterStatus {
    Generated,
    Sent,
    Acknowledged,
    Paid,
    Escalated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionCase {
    pub id: Uuid,
    pub case_number: String,
    pub customer_id: Uuid,
    pub dunning_letter_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub open_date: DateTime<Utc>,
    pub close_date: Option<DateTime<Utc>>,
    pub total_amount: i64,
    pub collected_amount: i64,
    pub status: CollectionCaseStatus,
    pub priority: CollectionPriority,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CollectionCaseStatus {
    Open,
    InProgress,
    Negotiating,
    PartialPayment,
    Settled,
    WrittenOff,
    LegalAction,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CollectionPriority {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollectionActivity {
    pub id: Uuid,
    pub case_id: Uuid,
    pub activity_type: CollectionActivityType,
    pub description: String,
    pub performed_by: Option<Uuid>,
    pub performed_at: DateTime<Utc>,
    pub result: Option<String>,
    pub next_action: Option<String>,
    pub next_action_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CollectionActivityType {
    Phone,
    Email,
    Letter,
    Meeting,
    PaymentPlan,
    Settlement,
    Legal,
    Note,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountingPeriod {
    pub id: Uuid,
    pub fiscal_year_id: Uuid,
    pub period_number: i32,
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub lock_type: PeriodLockType,
    pub locked_at: Option<DateTime<Utc>>,
    pub locked_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PeriodLockType {
    Open,
    SoftClose,
    HardClose,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeriodCloseChecklist {
    pub id: Uuid,
    pub period_id: Uuid,
    pub task_name: String,
    pub description: Option<String>,
    pub task_order: i32,
    pub is_required: bool,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringJournal {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub frequency: RecurringFrequency,
    pub interval_value: i32,
    pub day_of_month: Option<i32>,
    pub day_of_week: Option<i32>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub next_run_date: Option<DateTime<Utc>>,
    pub last_run_date: Option<DateTime<Utc>>,
    pub lines: Vec<RecurringJournalLine>,
    pub auto_post: bool,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RecurringFrequency {
    Daily,
    Weekly,
    Biweekly,
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringJournalLine {
    pub id: Uuid,
    pub recurring_journal_id: Uuid,
    pub account_id: Uuid,
    pub debit: i64,
    pub credit: i64,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringJournalRun {
    pub id: Uuid,
    pub recurring_journal_id: Uuid,
    pub run_date: DateTime<Utc>,
    pub journal_entry_id: Uuid,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashPool {
    pub id: Uuid,
    pub pool_code: String,
    pub name: String,
    pub description: Option<String>,
    pub header_account_id: Uuid,
    pub pooling_type: PoolingType,
    pub pooling_frequency: PoolingFrequency,
    pub target_balance: i64,
    pub min_balance: i64,
    pub max_balance: i64,
    pub interest_calculation_method: InterestCalcMethod,
    pub interest_rate: f64,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PoolingType {
    ZeroBalance,
    TargetBalance,
    Notional,
    Physical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PoolingFrequency {
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum InterestCalcMethod {
    Simple,
    Compound,
    Tiered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashPoolMember {
    pub id: Uuid,
    pub pool_id: Uuid,
    pub bank_account_id: Uuid,
    pub company_id: Uuid,
    pub member_type: PoolMemberType,
    pub participation_percent: f64,
    pub target_balance: i64,
    pub min_balance: i64,
    pub max_balance: i64,
    pub interest_rate_override: Option<f64>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PoolMemberType {
    Header,
    Participant,
    SubAccount,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashSweep {
    pub id: Uuid,
    pub sweep_number: String,
    pub pool_id: Uuid,
    pub sweep_date: DateTime<Utc>,
    pub sweep_type: SweepType,
    pub total_amount: i64,
    pub status: SweepStatus,
    pub processed_at: Option<DateTime<Utc>>,
    pub journal_entry_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SweepType {
    ZeroBalance,
    TargetBalance,
    Threshold,
    Scheduled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SweepStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashSweepLine {
    pub id: Uuid,
    pub sweep_id: Uuid,
    pub member_id: Uuid,
    pub bank_account_id: Uuid,
    pub opening_balance: i64,
    pub target_balance: i64,
    pub sweep_amount: i64,
    pub closing_balance: i64,
    pub direction: SweepDirection,
    pub status: SweepLineStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SweepDirection {
    ToHeader,
    FromHeader,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SweepLineStatus {
    Pending,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashPosition {
    pub id: Uuid,
    pub position_date: DateTime<Utc>,
    pub company_id: Option<Uuid>,
    pub bank_account_id: Option<Uuid>,
    pub opening_balance: i64,
    pub receipts: i64,
    pub disbursements: i64,
    pub transfers_in: i64,
    pub transfers_out: i64,
    pub closing_balance: i64,
    pub currency: String,
    pub exchange_rate: f64,
    pub base_currency_balance: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashPositionDetail {
    pub id: Uuid,
    pub position_id: Uuid,
    pub transaction_type: CashTransactionType,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub description: Option<String>,
    pub amount: i64,
    pub value_date: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CashTransactionType {
    Receipt,
    Disbursement,
    TransferIn,
    TransferOut,
    Adjustment,
    BankFee,
    Interest,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntercompanyLoan {
    pub id: Uuid,
    pub loan_number: String,
    pub from_company_id: Uuid,
    pub to_company_id: Uuid,
    pub from_account_id: Uuid,
    pub to_account_id: Uuid,
    pub principal_amount: i64,
    pub currency: String,
    pub interest_rate: f64,
    pub interest_type: InterestType,
    pub start_date: DateTime<Utc>,
    pub maturity_date: Option<DateTime<Utc>>,
    pub repayment_schedule: Option<String>,
    pub status: LoanStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum InterestType {
    Fixed,
    Floating,
    LIBOR,
    SOFR,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LoanStatus {
    Active,
    Repaid,
    Defaulted,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntercompanyLoanPayment {
    pub id: Uuid,
    pub loan_id: Uuid,
    pub payment_date: DateTime<Utc>,
    pub principal_amount: i64,
    pub interest_amount: i64,
    pub total_amount: i64,
    pub from_journal_entry_id: Option<Uuid>,
    pub to_journal_entry_id: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierQualification {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub qualification_type: QualificationType,
    pub status: QualificationStatus,
    pub submitted_at: Option<DateTime<Utc>>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub score: Option<i32>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum QualificationType {
    Financial,
    Quality,
    Environmental,
    Safety,
    Technical,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum QualificationStatus {
    Pending,
    UnderReview,
    Approved,
    Rejected,
    Expired,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierQualificationDocument {
    pub id: Uuid,
    pub qualification_id: Uuid,
    pub document_type: String,
    pub document_name: String,
    pub file_path: String,
    pub expiry_date: Option<DateTime<Utc>>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierEvaluation {
    pub id: Uuid,
    pub evaluation_number: String,
    pub vendor_id: Uuid,
    pub evaluation_period_start: DateTime<Utc>,
    pub evaluation_period_end: DateTime<Utc>,
    pub quality_score: i32,
    pub delivery_score: i32,
    pub price_score: i32,
    pub service_score: i32,
    pub overall_score: f64,
    pub grade: SupplierGrade,
    pub evaluator_id: Option<Uuid>,
    pub evaluated_at: DateTime<Utc>,
    pub comments: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SupplierGrade {
    A,
    B,
    C,
    D,
    F,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierEvaluationCriteria {
    pub id: Uuid,
    pub criteria_code: String,
    pub name: String,
    pub description: Option<String>,
    pub category: EvaluationCategory,
    pub weight: i32,
    pub max_score: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EvaluationCategory {
    Quality,
    Delivery,
    Price,
    Service,
    Compliance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierEvaluationLine {
    pub id: Uuid,
    pub evaluation_id: Uuid,
    pub criteria_id: Uuid,
    pub score: i32,
    pub weighted_score: f64,
    pub comments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyRevaluation {
    pub id: Uuid,
    pub revaluation_number: String,
    pub revaluation_date: DateTime<Utc>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub base_currency: String,
    pub status: CurrencyRevaluationStatus,
    pub total_unrealized_gain: i64,
    pub total_unrealized_loss: i64,
    pub net_unrealized: i64,
    pub journal_entry_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CurrencyRevaluationStatus {
    Draft,
    Pending,
    Completed,
    Reversed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyRevaluationLine {
    pub id: Uuid,
    pub revaluation_id: Uuid,
    pub account_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub currency: String,
    pub original_balance: i64,
    pub original_rate: f64,
    pub revaluation_rate: f64,
    pub base_currency_balance: i64,
    pub revalued_balance: i64,
    pub unrealized_gain: i64,
    pub unrealized_loss: i64,
    pub gain_account_id: Option<Uuid>,
    pub loss_account_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyRevaluationSummary {
    pub currency: String,
    pub total_accounts: i32,
    pub total_original_balance: i64,
    pub total_revalued_balance: i64,
    pub total_unrealized_gain: i64,
    pub total_unrealized_loss: i64,
    pub net_change: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CurrencyRevaluationPreview {
    pub revaluation_date: DateTime<Utc>,
    pub base_currency: String,
    pub lines: Vec<CurrencyRevaluationLine>,
    pub total_unrealized_gain: i64,
    pub total_unrealized_loss: i64,
    pub net_unrealized: i64,
    pub summaries: Vec<CurrencyRevaluationSummary>,
}
