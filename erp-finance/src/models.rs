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
