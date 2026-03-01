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
                 AccountService, JournalEntryService, FiscalYearService, FinancialReportingService,
                 BalanceSheet, ProfitAndLoss, TrialBalance,
                 DunningService, PeriodManagementService, RecurringJournalService,
                 DunningLevel,
                 CollectionPriority, CollectionActivityType,
                 PeriodLockType, RecurringFrequency,
                 CurrencyRevaluationService, CurrencyRevaluation,
                 CurrencyRevaluationPreview};

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

#[derive(Debug, Serialize)]
pub struct BalanceSheetResponse {
    pub as_of_date: String,
    pub assets: Vec<AccountBalanceResponse>,
    pub total_assets: f64,
    pub liabilities: Vec<AccountBalanceResponse>,
    pub total_liabilities: f64,
    pub equity: Vec<AccountBalanceResponse>,
    pub total_equity: f64,
}

#[derive(Debug, Serialize)]
pub struct AccountBalanceResponse {
    pub account_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub account_type: String,
    pub balance: f64,
}

impl From<BalanceSheet> for BalanceSheetResponse {
    fn from(bs: BalanceSheet) -> Self {
        Self {
            as_of_date: bs.as_of_date.to_rfc3339(),
            assets: bs.assets.into_iter().map(|a| AccountBalanceResponse {
                account_id: a.account_id,
                account_code: a.account_code,
                account_name: a.account_name,
                account_type: format!("{:?}", a.account_type),
                balance: a.balance as f64 / 100.0,
            }).collect(),
            total_assets: bs.total_assets as f64 / 100.0,
            liabilities: bs.liabilities.into_iter().map(|a| AccountBalanceResponse {
                account_id: a.account_id,
                account_code: a.account_code,
                account_name: a.account_name,
                account_type: format!("{:?}", a.account_type),
                balance: a.balance as f64 / 100.0,
            }).collect(),
            total_liabilities: bs.total_liabilities as f64 / 100.0,
            equity: bs.equity.into_iter().map(|a| AccountBalanceResponse {
                account_id: a.account_id,
                account_code: a.account_code,
                account_name: a.account_name,
                account_type: format!("{:?}", a.account_type),
                balance: a.balance as f64 / 100.0,
            }).collect(),
            total_equity: bs.total_equity as f64 / 100.0,
        }
    }
}

pub async fn get_balance_sheet(
    State(state): State<AppState>,
) -> ApiResult<Json<BalanceSheetResponse>> {
    let service = FinancialReportingService::new();
    let bs = service.get_balance_sheet(&state.pool).await?;
    Ok(Json(BalanceSheetResponse::from(bs)))
}

#[derive(Debug, Serialize)]
pub struct ProfitAndLossResponse {
    pub from_date: String,
    pub to_date: String,
    pub revenue: Vec<AccountBalanceResponse>,
    pub total_revenue: f64,
    pub expenses: Vec<AccountBalanceResponse>,
    pub total_expenses: f64,
    pub net_income: f64,
}

impl From<ProfitAndLoss> for ProfitAndLossResponse {
    fn from(pl: ProfitAndLoss) -> Self {
        Self {
            from_date: pl.from_date.to_rfc3339(),
            to_date: pl.to_date.to_rfc3339(),
            revenue: pl.revenue.into_iter().map(|a| AccountBalanceResponse {
                account_id: a.account_id,
                account_code: a.account_code,
                account_name: a.account_name,
                account_type: format!("{:?}", a.account_type),
                balance: a.balance as f64 / 100.0,
            }).collect(),
            total_revenue: pl.total_revenue as f64 / 100.0,
            expenses: pl.expenses.into_iter().map(|a| AccountBalanceResponse {
                account_id: a.account_id,
                account_code: a.account_code,
                account_name: a.account_name,
                account_type: format!("{:?}", a.account_type),
                balance: a.balance as f64 / 100.0,
            }).collect(),
            total_expenses: pl.total_expenses as f64 / 100.0,
            net_income: pl.net_income as f64 / 100.0,
        }
    }
}

pub async fn get_profit_and_loss(
    State(state): State<AppState>,
) -> ApiResult<Json<ProfitAndLossResponse>> {
    let service = FinancialReportingService::new();
    let pl = service.get_profit_and_loss(&state.pool, None, None).await?;
    Ok(Json(ProfitAndLossResponse::from(pl)))
}

#[derive(Debug, Serialize)]
pub struct TrialBalanceResponse {
    pub as_of_date: String,
    pub accounts: Vec<TrialBalanceLineResponse>,
    pub total_debits: f64,
    pub total_credits: f64,
}

#[derive(Debug, Serialize)]
pub struct TrialBalanceLineResponse {
    pub account_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub debit: f64,
    pub credit: f64,
}

impl From<TrialBalance> for TrialBalanceResponse {
    fn from(tb: TrialBalance) -> Self {
        Self {
            as_of_date: tb.as_of_date.to_rfc3339(),
            accounts: tb.accounts.into_iter().map(|a| TrialBalanceLineResponse {
                account_id: a.account_id,
                account_code: a.account_code,
                account_name: a.account_name,
                debit: a.debit as f64 / 100.0,
                credit: a.credit as f64 / 100.0,
            }).collect(),
            total_debits: tb.total_debits as f64 / 100.0,
            total_credits: tb.total_credits as f64 / 100.0,
        }
    }
}

pub async fn get_trial_balance(
    State(state): State<AppState>,
) -> ApiResult<Json<TrialBalanceResponse>> {
    let service = FinancialReportingService::new();
    let tb = service.get_trial_balance(&state.pool).await?;
    Ok(Json(TrialBalanceResponse::from(tb)))
}

#[derive(Deserialize)]
pub struct CreateDunningPolicyRequest {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct DunningPolicyResponse {
    pub id: Uuid,
    pub name: String,
    pub status: String,
}

pub async fn create_dunning_policy(
    State(state): State<AppState>,
    Json(req): Json<CreateDunningPolicyRequest>,
) -> ApiResult<Json<DunningPolicyResponse>> {
    let policy = DunningService::create_policy(&state.pool, &req.name, req.description.as_deref()).await?;
    Ok(Json(DunningPolicyResponse {
        id: policy.id,
        name: policy.name,
        status: format!("{:?}", policy.status),
    }))
}

#[derive(Deserialize)]
pub struct AddDunningLevelRequest {
    pub level: String,
    pub days_overdue: i32,
    pub fee_percent: f64,
    pub fee_fixed: i64,
    pub stop_services: bool,
    pub send_email: bool,
}

pub async fn add_dunning_level(
    State(state): State<AppState>,
    Path(policy_id): Path<Uuid>,
    Json(req): Json<AddDunningLevelRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let level = match req.level.as_str() {
        "FirstNotice" => DunningLevel::FirstNotice,
        "SecondNotice" => DunningLevel::SecondNotice,
        "FinalNotice" => DunningLevel::FinalNotice,
        "Collection" => DunningLevel::Collection,
        "Legal" => DunningLevel::Legal,
        _ => DunningLevel::Reminder,
    };
    
    DunningService::add_level(
        &state.pool,
        policy_id,
        level,
        req.days_overdue,
        req.fee_percent,
        req.fee_fixed,
        req.stop_services,
        req.send_email,
    ).await?;
    
    Ok(Json(serde_json::json!({ "status": "added" })))
}

#[derive(Deserialize)]
pub struct CreateDunningRunRequest {
    pub policy_id: Uuid,
}

#[derive(Serialize)]
pub struct DunningRunResponse {
    pub id: Uuid,
    pub run_number: String,
    pub status: String,
}

pub async fn create_dunning_run(
    State(state): State<AppState>,
    Json(req): Json<CreateDunningRunRequest>,
) -> ApiResult<Json<DunningRunResponse>> {
    let run = DunningService::create_run(&state.pool, req.policy_id).await?;
    Ok(Json(DunningRunResponse {
        id: run.id,
        run_number: run.run_number,
        status: format!("{:?}", run.status),
    }))
}

pub async fn execute_dunning_run(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let letters = DunningService::execute_run(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "letters_generated": letters.len() })))
}

#[derive(Serialize)]
pub struct AgingReportResponse {
    pub as_of_date: String,
    pub customers: Vec<AgingLineResponse>,
}

#[derive(Serialize)]
pub struct AgingLineResponse {
    pub customer_id: Uuid,
    pub customer_name: String,
    pub current: f64,
    pub days_31_60: f64,
    pub days_61_90: f64,
    pub over_90: f64,
    pub total: f64,
}

pub async fn get_aging_report(
    State(state): State<AppState>,
) -> ApiResult<Json<AgingReportResponse>> {
    let report = DunningService::get_aging_report(&state.pool).await?;
    Ok(Json(AgingReportResponse {
        as_of_date: report.as_of_date.to_rfc3339(),
        customers: report.customers.into_iter().map(|c| AgingLineResponse {
            customer_id: c.customer_id,
            customer_name: c.customer_name,
            current: c.current as f64 / 100.0,
            days_31_60: c.days_31_60 as f64 / 100.0,
            days_61_90: c.days_61_90 as f64 / 100.0,
            over_90: c.over_90 as f64 / 100.0,
            total: c.total as f64 / 100.0,
        }).collect(),
    }))
}

#[derive(Deserialize)]
pub struct CreateCollectionCaseRequest {
    pub customer_id: Uuid,
    pub dunning_letter_id: Option<Uuid>,
    pub total_amount: i64,
    pub priority: Option<String>,
}

#[derive(Serialize)]
pub struct CollectionCaseResponse {
    pub id: Uuid,
    pub case_number: String,
    pub status: String,
}

pub async fn create_collection_case(
    State(state): State<AppState>,
    Json(req): Json<CreateCollectionCaseRequest>,
) -> ApiResult<Json<CollectionCaseResponse>> {
    let priority = match req.priority.as_deref() {
        Some("Low") => CollectionPriority::Low,
        Some("High") => CollectionPriority::High,
        Some("Critical") => CollectionPriority::Critical,
        _ => CollectionPriority::Medium,
    };
    
    let case = DunningService::create_collection_case(
        &state.pool,
        req.customer_id,
        req.dunning_letter_id,
        req.total_amount,
        priority,
    ).await?;
    
    Ok(Json(CollectionCaseResponse {
        id: case.id,
        case_number: case.case_number,
        status: format!("{:?}", case.status),
    }))
}

#[derive(Deserialize)]
pub struct AddCollectionActivityRequest {
    pub activity_type: String,
    pub description: String,
    pub result: Option<String>,
    pub next_action: Option<String>,
}

pub async fn add_collection_activity(
    State(state): State<AppState>,
    Path(case_id): Path<Uuid>,
    Json(req): Json<AddCollectionActivityRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let activity_type = match req.activity_type.as_str() {
        "Phone" => CollectionActivityType::Phone,
        "Email" => CollectionActivityType::Email,
        "Letter" => CollectionActivityType::Letter,
        "Meeting" => CollectionActivityType::Meeting,
        "PaymentPlan" => CollectionActivityType::PaymentPlan,
        "Settlement" => CollectionActivityType::Settlement,
        "Legal" => CollectionActivityType::Legal,
        _ => CollectionActivityType::Note,
    };
    
    DunningService::add_collection_activity(
        &state.pool,
        case_id,
        activity_type,
        &req.description,
        None,
        req.result.as_deref(),
        req.next_action.as_deref(),
        None,
    ).await?;
    
    Ok(Json(serde_json::json!({ "status": "added" })))
}

#[derive(Deserialize)]
pub struct ListPeriodsQuery {
    pub fiscal_year_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct PeriodResponse {
    pub id: Uuid,
    pub period_number: i32,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub lock_type: String,
}

pub async fn list_periods(
    State(state): State<AppState>,
    Query(query): Query<ListPeriodsQuery>,
) -> ApiResult<Json<Vec<PeriodResponse>>> {
    let fiscal_year_id = query.fiscal_year_id
        .ok_or_else(|| erp_core::Error::validation("fiscal_year_id is required"))?;
    
    let periods = PeriodManagementService::list_periods(&state.pool, fiscal_year_id).await?;
    Ok(Json(periods.into_iter().map(|p| PeriodResponse {
        id: p.id,
        period_number: p.period_number,
        name: p.name,
        start_date: p.start_date.to_rfc3339(),
        end_date: p.end_date.to_rfc3339(),
        lock_type: format!("{:?}", p.lock_type),
    }).collect()))
}

pub async fn create_periods(
    State(state): State<AppState>,
    Path(fiscal_year_id): Path<Uuid>,
) -> ApiResult<Json<Vec<PeriodResponse>>> {
    let periods = PeriodManagementService::create_periods_for_fiscal_year(&state.pool, fiscal_year_id).await?;
    Ok(Json(periods.into_iter().map(|p| PeriodResponse {
        id: p.id,
        period_number: p.period_number,
        name: p.name,
        start_date: p.start_date.to_rfc3339(),
        end_date: p.end_date.to_rfc3339(),
        lock_type: format!("{:?}", p.lock_type),
    }).collect()))
}

#[derive(Deserialize)]
pub struct LockPeriodRequest {
    pub lock_type: String,
}

pub async fn lock_period(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<LockPeriodRequest>,
) -> ApiResult<Json<PeriodResponse>> {
    let lock_type = match req.lock_type.as_str() {
        "HardClose" => PeriodLockType::HardClose,
        _ => PeriodLockType::SoftClose,
    };
    
    let period = PeriodManagementService::lock_period(&state.pool, id, lock_type, None).await?;
    Ok(Json(PeriodResponse {
        id: period.id,
        period_number: period.period_number,
        name: period.name,
        start_date: period.start_date.to_rfc3339(),
        end_date: period.end_date.to_rfc3339(),
        lock_type: format!("{:?}", period.lock_type),
    }))
}

pub async fn unlock_period(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PeriodResponse>> {
    let period = PeriodManagementService::unlock_period(&state.pool, id).await?;
    Ok(Json(PeriodResponse {
        id: period.id,
        period_number: period.period_number,
        name: period.name,
        start_date: period.start_date.to_rfc3339(),
        end_date: period.end_date.to_rfc3339(),
        lock_type: format!("{:?}", period.lock_type),
    }))
}

#[derive(Deserialize)]
pub struct CreateChecklistRequest {
    pub tasks: Vec<ChecklistTaskRequest>,
}

#[derive(Deserialize)]
pub struct ChecklistTaskRequest {
    pub task_name: String,
    pub description: Option<String>,
    pub is_required: bool,
}

pub async fn create_close_checklist(
    State(state): State<AppState>,
    Path(period_id): Path<Uuid>,
    Json(req): Json<CreateChecklistRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let tasks: Vec<(String, Option<String>, bool)> = req.tasks.into_iter()
        .map(|t| (t.task_name, t.description, t.is_required))
        .collect();
    
    let items = PeriodManagementService::create_close_checklist(&state.pool, period_id, tasks).await?;
    Ok(Json(serde_json::json!({ "tasks_created": items.len() })))
}

pub async fn complete_checklist_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    PeriodManagementService::complete_checklist_task(&state.pool, task_id, None).await?;
    Ok(Json(serde_json::json!({ "status": "completed" })))
}

#[derive(Deserialize)]
pub struct CreateRecurringJournalRequest {
    pub name: String,
    pub description: Option<String>,
    pub frequency: String,
    pub interval_value: i32,
    pub start_date: String,
    pub end_date: Option<String>,
    pub auto_post: bool,
    pub lines: Vec<RecurringLineRequest>,
}

#[derive(Deserialize)]
pub struct RecurringLineRequest {
    pub account_id: Uuid,
    pub debit: i64,
    pub credit: i64,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct RecurringJournalResponse {
    pub id: Uuid,
    pub name: String,
    pub frequency: String,
    pub next_run_date: Option<String>,
    pub status: String,
}

pub async fn list_recurring_journals(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<RecurringJournalResponse>>> {
    let journals = RecurringJournalService::list(&state.pool).await?;
    Ok(Json(journals.into_iter().map(|j| RecurringJournalResponse {
        id: j.id,
        name: j.name,
        frequency: format!("{:?}", j.frequency),
        next_run_date: j.next_run_date.map(|d| d.to_rfc3339()),
        status: format!("{:?}", j.status),
    }).collect()))
}

pub async fn create_recurring_journal(
    State(state): State<AppState>,
    Json(req): Json<CreateRecurringJournalRequest>,
) -> ApiResult<Json<RecurringJournalResponse>> {
    let frequency = match req.frequency.as_str() {
        "Daily" => RecurringFrequency::Daily,
        "Weekly" => RecurringFrequency::Weekly,
        "Biweekly" => RecurringFrequency::Biweekly,
        "Quarterly" => RecurringFrequency::Quarterly,
        "Yearly" => RecurringFrequency::Yearly,
        _ => RecurringFrequency::Monthly,
    };
    
    let start_date = chrono::DateTime::parse_from_rfc3339(&req.start_date)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|_| erp_core::Error::validation("Invalid start date format"))?;
    
    let end_date = req.end_date.and_then(|d| {
        chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&Utc))
    });
    
    let lines: Vec<(Uuid, i64, i64, Option<&str>)> = req.lines.into_iter()
        .map(|l| (l.account_id, l.debit, l.credit, None))
        .collect();
    
    let journal = RecurringJournalService::create(
        &state.pool,
        &req.name,
        req.description.as_deref(),
        frequency,
        req.interval_value,
        start_date,
        end_date,
        req.auto_post,
        lines,
    ).await?;
    
    Ok(Json(RecurringJournalResponse {
        id: journal.id,
        name: journal.name,
        frequency: format!("{:?}", journal.frequency),
        next_run_date: journal.next_run_date.map(|d| d.to_rfc3339()),
        status: format!("{:?}", journal.status),
    }))
}

pub async fn process_recurring_journals(
    State(state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let processed = RecurringJournalService::process_due(&state.pool).await?;
    Ok(Json(serde_json::json!({ "processed": processed.len() })))
}

pub async fn deactivate_recurring_journal(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    RecurringJournalService::deactivate(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "deactivated" })))
}

#[derive(Deserialize)]
pub struct PreviewCurrencyRevaluationRequest {
    pub revaluation_date: String,
    pub base_currency: Option<String>,
}

#[derive(Serialize)]
pub struct CurrencyRevaluationPreviewResponse {
    pub revaluation_date: String,
    pub base_currency: String,
    pub lines: Vec<CurrencyRevaluationLineResponse>,
    pub total_unrealized_gain: f64,
    pub total_unrealized_loss: f64,
    pub net_unrealized: f64,
    pub summaries: Vec<CurrencyRevaluationSummaryResponse>,
}

#[derive(Serialize)]
pub struct CurrencyRevaluationLineResponse {
    pub account_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub currency: String,
    pub original_balance: f64,
    pub original_rate: f64,
    pub revaluation_rate: f64,
    pub base_currency_balance: f64,
    pub revalued_balance: f64,
    pub unrealized_gain: f64,
    pub unrealized_loss: f64,
}

#[derive(Serialize)]
pub struct CurrencyRevaluationSummaryResponse {
    pub currency: String,
    pub total_accounts: i32,
    pub total_original_balance: f64,
    pub total_revalued_balance: f64,
    pub total_unrealized_gain: f64,
    pub total_unrealized_loss: f64,
    pub net_change: f64,
}

impl From<CurrencyRevaluationPreview> for CurrencyRevaluationPreviewResponse {
    fn from(p: CurrencyRevaluationPreview) -> Self {
        Self {
            revaluation_date: p.revaluation_date.to_rfc3339(),
            base_currency: p.base_currency,
            lines: p.lines.into_iter().map(|l| CurrencyRevaluationLineResponse {
                account_id: l.account_id,
                account_code: l.account_code,
                account_name: l.account_name,
                currency: l.currency,
                original_balance: l.original_balance as f64 / 100.0,
                original_rate: l.original_rate,
                revaluation_rate: l.revaluation_rate,
                base_currency_balance: l.base_currency_balance as f64 / 100.0,
                revalued_balance: l.revalued_balance as f64 / 100.0,
                unrealized_gain: l.unrealized_gain as f64 / 100.0,
                unrealized_loss: l.unrealized_loss as f64 / 100.0,
            }).collect(),
            total_unrealized_gain: p.total_unrealized_gain as f64 / 100.0,
            total_unrealized_loss: p.total_unrealized_loss as f64 / 100.0,
            net_unrealized: p.net_unrealized as f64 / 100.0,
            summaries: p.summaries.into_iter().map(|s| CurrencyRevaluationSummaryResponse {
                currency: s.currency,
                total_accounts: s.total_accounts,
                total_original_balance: s.total_original_balance as f64 / 100.0,
                total_revalued_balance: s.total_revalued_balance as f64 / 100.0,
                total_unrealized_gain: s.total_unrealized_gain as f64 / 100.0,
                total_unrealized_loss: s.total_unrealized_loss as f64 / 100.0,
                net_change: s.net_change as f64 / 100.0,
            }).collect(),
        }
    }
}

pub async fn preview_currency_revaluation(
    State(state): State<AppState>,
    Json(req): Json<PreviewCurrencyRevaluationRequest>,
) -> ApiResult<Json<CurrencyRevaluationPreviewResponse>> {
    let revaluation_date = chrono::DateTime::parse_from_rfc3339(&req.revaluation_date)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|_| erp_core::Error::validation("Invalid revaluation date format"))?;
    
    let base_currency = req.base_currency.unwrap_or_else(|| "USD".to_string());
    
    let preview = CurrencyRevaluationService::preview_revaluation(&state.pool, revaluation_date, &base_currency).await?;
    Ok(Json(CurrencyRevaluationPreviewResponse::from(preview)))
}

#[derive(Deserialize)]
pub struct CreateCurrencyRevaluationRequest {
    pub revaluation_date: String,
    pub period_start: String,
    pub period_end: String,
    pub base_currency: Option<String>,
    pub gain_account_id: Option<Uuid>,
    pub loss_account_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct CurrencyRevaluationResponse {
    pub id: Uuid,
    pub revaluation_number: String,
    pub revaluation_date: String,
    pub period_start: String,
    pub period_end: String,
    pub base_currency: String,
    pub status: String,
    pub total_unrealized_gain: f64,
    pub total_unrealized_loss: f64,
    pub net_unrealized: f64,
    pub journal_entry_id: Option<Uuid>,
}

impl From<CurrencyRevaluation> for CurrencyRevaluationResponse {
    fn from(r: CurrencyRevaluation) -> Self {
        Self {
            id: r.id,
            revaluation_number: r.revaluation_number,
            revaluation_date: r.revaluation_date.to_rfc3339(),
            period_start: r.period_start.to_rfc3339(),
            period_end: r.period_end.to_rfc3339(),
            base_currency: r.base_currency,
            status: format!("{:?}", r.status),
            total_unrealized_gain: r.total_unrealized_gain as f64 / 100.0,
            total_unrealized_loss: r.total_unrealized_loss as f64 / 100.0,
            net_unrealized: r.net_unrealized as f64 / 100.0,
            journal_entry_id: r.journal_entry_id,
        }
    }
}

pub async fn list_currency_revaluations(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<CurrencyRevaluationResponse>>> {
    let revaluations = CurrencyRevaluationService::list_revaluations(&state.pool).await?;
    Ok(Json(revaluations.into_iter().map(CurrencyRevaluationResponse::from).collect()))
}

pub async fn get_currency_revaluation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<CurrencyRevaluationResponse>> {
    let revaluation = CurrencyRevaluationService::get_revaluation(&state.pool, id).await?;
    Ok(Json(CurrencyRevaluationResponse::from(revaluation)))
}

pub async fn create_currency_revaluation(
    State(state): State<AppState>,
    Json(req): Json<CreateCurrencyRevaluationRequest>,
) -> ApiResult<Json<CurrencyRevaluationResponse>> {
    let revaluation_date = chrono::DateTime::parse_from_rfc3339(&req.revaluation_date)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|_| erp_core::Error::validation("Invalid revaluation date format"))?;
    
    let period_start = chrono::DateTime::parse_from_rfc3339(&req.period_start)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|_| erp_core::Error::validation("Invalid period start format"))?;
    
    let period_end = chrono::DateTime::parse_from_rfc3339(&req.period_end)
        .map(|d| d.with_timezone(&Utc))
        .map_err(|_| erp_core::Error::validation("Invalid period end format"))?;
    
    let base_currency = req.base_currency.unwrap_or_else(|| "USD".to_string());
    
    let revaluation = CurrencyRevaluationService::create_revaluation(
        &state.pool,
        revaluation_date,
        period_start,
        period_end,
        &base_currency,
        req.gain_account_id,
        req.loss_account_id,
        None,
    ).await?;
    
    Ok(Json(CurrencyRevaluationResponse::from(revaluation)))
}

pub async fn get_currency_revaluation_lines(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<Vec<CurrencyRevaluationLineResponse>>> {
    let lines = CurrencyRevaluationService::get_revaluation_lines(&state.pool, id).await?;
    Ok(Json(lines.into_iter().map(|l| CurrencyRevaluationLineResponse {
        account_id: l.account_id,
        account_code: l.account_code,
        account_name: l.account_name,
        currency: l.currency,
        original_balance: l.original_balance as f64 / 100.0,
        original_rate: l.original_rate,
        revaluation_rate: l.revaluation_rate,
        base_currency_balance: l.base_currency_balance as f64 / 100.0,
        revalued_balance: l.revalued_balance as f64 / 100.0,
        unrealized_gain: l.unrealized_gain as f64 / 100.0,
        unrealized_loss: l.unrealized_loss as f64 / 100.0,
    }).collect()))
}

pub async fn post_currency_revaluation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<CurrencyRevaluationResponse>> {
    let revaluation = CurrencyRevaluationService::post_revaluation(&state.pool, id).await?;
    Ok(Json(CurrencyRevaluationResponse::from(revaluation)))
}

pub async fn reverse_currency_revaluation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<CurrencyRevaluationResponse>> {
    let revaluation = CurrencyRevaluationService::reverse_revaluation(&state.pool, id).await?;
    Ok(Json(CurrencyRevaluationResponse::from(revaluation)))
}
