use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CompanyType {
    Parent,
    Subsidiary,
    Division,
    Branch,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ConsolidationMethod {
    Full,
    Equity,
    Proportional,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Company {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub legal_name: String,
    pub company_type: CompanyType,
    pub parent_id: Option<Uuid>,
    pub tax_id: Option<String>,
    pub registration_number: Option<String>,
    pub currency: String,
    pub fiscal_year_start: i32,
    pub consolidation_method: ConsolidationMethod,
    pub ownership_percentage: f64,
    pub street: String,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntercompanyTransaction {
    pub id: Uuid,
    pub transaction_number: String,
    pub from_company_id: Uuid,
    pub to_company_id: Uuid,
    pub transaction_type: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub amount: i64,
    pub currency: String,
    pub exchange_rate: f64,
    pub base_amount: i64,
    pub description: String,
    pub due_date: Option<DateTime<Utc>>,
    pub status: String,
    pub elimination_entry_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationEntry {
    pub id: Uuid,
    pub consolidation_id: Uuid,
    pub company_id: Uuid,
    pub account_code: String,
    pub debit: i64,
    pub credit: i64,
    pub elimination_type: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consolidation {
    pub id: Uuid,
    pub name: String,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub status: String,
    pub total_eliminations: i64,
    pub created_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessUnit {
    pub id: Uuid,
    pub company_id: Uuid,
    pub code: String,
    pub name: String,
    pub manager_id: Option<Uuid>,
    pub budget: Option<i64>,
    pub currency: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCompanyRequest {
    pub code: String,
    pub name: String,
    pub legal_name: String,
    pub company_type: CompanyType,
    pub parent_id: Option<Uuid>,
    pub tax_id: Option<String>,
    pub registration_number: Option<String>,
    pub currency: String,
    pub fiscal_year_start: i32,
    pub consolidation_method: ConsolidationMethod,
    pub ownership_percentage: f64,
    pub street: String,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateIntercompanyRequest {
    pub from_company_id: Uuid,
    pub to_company_id: Uuid,
    pub transaction_type: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub amount: i64,
    pub currency: String,
    pub exchange_rate: f64,
    pub description: String,
    pub due_date: Option<DateTime<Utc>>,
}
