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
