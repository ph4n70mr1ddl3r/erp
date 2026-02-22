use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{BaseEntity, Status, Currency, Money, Pagination};
use erp_finance::{Account, AccountType, JournalEntry, JournalLine, FiscalYear,
                 AccountService, JournalEntryService, FiscalYearService};

#[derive(Debug, Deserialize)]
pub struct CreateAccountRequest {
    pub code: String,
    pub name: String,
    pub account_type: Option<String>,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AccountResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub account_type: String,
    pub parent_id: Option<Uuid>,
    pub status: String,
    pub description: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<Account> for AccountResponse {
    fn from(acc: Account) -> Self {
        Self {
            id: acc.base.id,
            code: acc.code,
            name: acc.name,
            account_type: format!("{:?}", acc.account_type),
            parent_id: acc.parent_id,
            status: format!("{:?}", acc.status),
            description: acc.description,
            created_at: acc.base.created_at.to_rfc3339(),
            updated_at: acc.base.updated_at.to_rfc3339(),
        }
    }
}

pub async fn list_accounts(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<AccountResponse>>> {
    let service = AccountService::new();
    let result = service.list_accounts(&state.pool, pagination).await?;
    
    Ok(Json(erp_core::Paginated::new(
        result.items.into_iter().map(AccountResponse::from).collect(),
        result.total,
        erp_core::Pagination { page: result.page, per_page: result.per_page }
    )))
}

pub async fn get_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<AccountResponse>> {
    let service = AccountService::new();
    let account = service.get_account(&state.pool, id).await?;
    Ok(Json(AccountResponse::from(account)))
}

pub async fn create_account(
    State(state): State<AppState>,
    Json(req): Json<CreateAccountRequest>,
) -> ApiResult<Json<AccountResponse>> {
    let service = AccountService::new();
    
    let account = Account {
        base: BaseEntity::new(),
        code: req.code,
        name: req.name,
        account_type: match req.account_type.as_deref() {
            Some("Liability") => AccountType::Liability,
            Some("Equity") => AccountType::Equity,
            Some("Revenue") => AccountType::Revenue,
            Some("Expense") => AccountType::Expense,
            _ => AccountType::Asset,
        },
        parent_id: req.parent_id,
        status: Status::Active,
        description: req.description,
    };
    
    let created = service.create_account(&state.pool, account).await?;
    Ok(Json(AccountResponse::from(created)))
}

pub async fn update_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateAccountRequest>,
) -> ApiResult<Json<AccountResponse>> {
    let service = AccountService::new();
    
    let mut account = service.get_account(&state.pool, id).await?;
    account.code = req.code;
    account.name = req.name;
    account.account_type = match req.account_type.as_deref() {
        Some("Liability") => AccountType::Liability,
        Some("Equity") => AccountType::Equity,
        Some("Revenue") => AccountType::Revenue,
        Some("Expense") => AccountType::Expense,
        _ => AccountType::Asset,
    };
    account.parent_id = req.parent_id;
    account.description = req.description;
    
    let updated = service.update_account(&state.pool, account).await?;
    Ok(Json(AccountResponse::from(updated)))
}

pub async fn delete_account(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<()>> {
    let service = AccountService::new();
    service.delete_account(&state.pool, id).await?;
    Ok(Json(()))
}

#[derive(Debug, Deserialize)]
pub struct CreateJournalEntryRequest {
    pub description: String,
    pub reference: Option<String>,
    pub lines: Vec<JournalLineRequest>,
}

#[derive(Debug, Deserialize)]
pub struct JournalLineRequest {
    pub account_id: Uuid,
    pub debit: i64,
    pub credit: i64,
    pub description: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct JournalEntryResponse {
    pub id: Uuid,
    pub entry_number: String,
    pub date: String,
    pub description: String,
    pub reference: Option<String>,
    pub lines: Vec<JournalLineResponse>,
    pub status: String,
    pub total_debit: f64,
    pub total_credit: f64,
}

#[derive(Debug, Serialize)]
pub struct JournalLineResponse {
    pub id: Uuid,
    pub account_id: Uuid,
    pub debit: f64,
    pub credit: f64,
    pub description: Option<String>,
}

impl From<JournalEntry> for JournalEntryResponse {
    fn from(entry: JournalEntry) -> Self {
        let total_debit: i64 = entry.lines.iter().map(|l| l.debit.amount).sum();
        let total_credit: i64 = entry.lines.iter().map(|l| l.credit.amount).sum();
        
        Self {
            id: entry.base.id,
            entry_number: entry.entry_number,
            date: entry.date.to_rfc3339(),
            description: entry.description,
            reference: entry.reference,
            lines: entry.lines.into_iter().map(|l| JournalLineResponse {
                id: l.id,
                account_id: l.account_id,
                debit: l.debit.to_decimal(),
                credit: l.credit.to_decimal(),
                description: l.description,
            }).collect(),
            status: format!("{:?}", entry.status),
            total_debit: total_debit as f64 / 100.0,
            total_credit: total_credit as f64 / 100.0,
        }
    }
}

pub async fn list_journal_entries(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<JournalEntryResponse>>> {
    let service = JournalEntryService::new();
    let result = service.list_entries(&state.pool, pagination).await?;
    
    Ok(Json(erp_core::Paginated::new(
        result.items.into_iter().map(JournalEntryResponse::from).collect(),
        result.total,
        erp_core::Pagination { page: result.page, per_page: result.per_page }
    )))
}

pub async fn get_journal_entry(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<JournalEntryResponse>> {
    let service = JournalEntryService::new();
    let entry = service.get_entry(&state.pool, id).await?;
    Ok(Json(JournalEntryResponse::from(entry)))
}

pub async fn create_journal_entry(
    State(state): State<AppState>,
    Json(req): Json<CreateJournalEntryRequest>,
) -> ApiResult<Json<JournalEntryResponse>> {
    let service = JournalEntryService::new();
    
    let entry = JournalEntry {
        base: BaseEntity::new(),
        entry_number: String::new(),
        date: Utc::now(),
        description: req.description,
        reference: req.reference,
        lines: req.lines.into_iter().map(|l| JournalLine {
            id: Uuid::nil(),
            account_id: l.account_id,
            debit: Money::new(l.debit, Currency::USD),
            credit: Money::new(l.credit, Currency::USD),
            description: l.description,
        }).collect(),
        status: Status::Draft,
    };
    
    let created = service.create_entry(&state.pool, entry).await?;
    Ok(Json(JournalEntryResponse::from(created)))
}

pub async fn post_journal_entry(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = JournalEntryService::new();
    service.post_entry(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "posted" })))
}

#[derive(Debug, Deserialize)]
pub struct CreateFiscalYearRequest {
    pub name: String,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize)]
pub struct FiscalYearResponse {
    pub id: Uuid,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub status: String,
}

impl From<FiscalYear> for FiscalYearResponse {
    fn from(fy: FiscalYear) -> Self {
        Self {
            id: fy.base.id,
            name: fy.name,
            start_date: fy.start_date.to_rfc3339(),
            end_date: fy.end_date.to_rfc3339(),
            status: format!("{:?}", fy.status),
        }
    }
}

pub async fn list_fiscal_years(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<FiscalYearResponse>>> {
    let service = FiscalYearService::new();
    let years = service.list_fiscal_years(&state.pool).await?;
    Ok(Json(years.into_iter().map(FiscalYearResponse::from).collect()))
}

pub async fn create_fiscal_year(
    State(state): State<AppState>,
    Json(req): Json<CreateFiscalYearRequest>,
) -> ApiResult<Json<FiscalYearResponse>> {
    let service = FiscalYearService::new();
    
    let start_date = chrono::DateTime::parse_from_rfc3339(&req.start_date)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|_| erp_core::Error::validation("Invalid start date format"))?;
    
    let end_date = chrono::DateTime::parse_from_rfc3339(&req.end_date)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|_| erp_core::Error::validation("Invalid end date format"))?;
    
    let year = FiscalYear {
        base: BaseEntity::new(),
        name: req.name,
        start_date,
        end_date,
        status: Status::Active,
    };
    
    let created = service.create_fiscal_year(&state.pool, year).await?;
    Ok(Json(FiscalYearResponse::from(created)))
}
