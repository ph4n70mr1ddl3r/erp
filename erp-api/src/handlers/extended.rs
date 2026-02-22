use axum::{extract::{Path, Query, State}, Json};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::Pagination;
use erp_finance::{CurrencyDef, ExchangeRate, BudgetWithVariance, CurrencyService, BudgetService};
use erp_inventory::{Lot, LotService};
use erp_hr::{LeaveTypeDef, LeaveBalance, LeaveRequestExtended, ExpenseReport, ExpenseCategory, LeaveService, ExpenseService, LeaveRequestStatus, ExpenseReportStatus};

#[derive(Debug, Deserialize)]
pub struct ExchangeRateRequest {
    pub from: String,
    pub to: String,
    pub rate: f64,
}

#[derive(Debug, Serialize)]
pub struct ExchangeRateResponse {
    pub id: Uuid,
    pub from_currency: String,
    pub to_currency: String,
    pub rate: f64,
    pub effective_date: String,
}

impl From<ExchangeRate> for ExchangeRateResponse {
    fn from(r: ExchangeRate) -> Self {
        Self {
            id: r.id,
            from_currency: r.from_currency,
            to_currency: r.to_currency,
            rate: r.rate,
            effective_date: r.effective_date.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct CurrencyResponse {
    pub code: String,
    pub name: String,
    pub symbol: String,
    pub is_base: bool,
    pub status: String,
}

impl From<CurrencyDef> for CurrencyResponse {
    fn from(c: CurrencyDef) -> Self {
        Self {
            code: c.code,
            name: c.name,
            symbol: c.symbol,
            is_base: c.is_base,
            status: format!("{:?}", c.status),
        }
    }
}

pub async fn list_currencies(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<CurrencyResponse>>> {
    let currencies = CurrencyService::list_currencies(&state.pool).await?;
    Ok(Json(currencies.into_iter().map(CurrencyResponse::from).collect()))
}

pub async fn set_exchange_rate(
    State(state): State<AppState>,
    Json(req): Json<ExchangeRateRequest>,
) -> ApiResult<Json<ExchangeRateResponse>> {
    let rate = CurrencyService::set_exchange_rate(&state.pool, &req.from, &req.to, req.rate).await?;
    Ok(Json(ExchangeRateResponse::from(rate)))
}

pub async fn convert_currency(
    State(state): State<AppState>,
    Query(query): Query<ConvertQuery>,
) -> ApiResult<Json<serde_json::Value>> {
    let converted = CurrencyService::convert_amount(&state.pool, query.amount, &query.from, &query.to).await?;
    Ok(Json(serde_json::json!({
        "from": query.from,
        "to": query.to,
        "original_amount": query.amount,
        "converted_amount": converted
    })))
}

#[derive(Debug, Deserialize)]
pub struct ConvertQuery {
    pub from: String,
    pub to: String,
    pub amount: i64,
}

#[derive(Debug, Serialize)]
pub struct BudgetResponse {
    pub id: Uuid,
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub total_amount: f64,
    pub total_actual: f64,
    pub total_variance: f64,
    pub variance_percent: f64,
    pub status: String,
    pub lines: Vec<BudgetLineResponse>,
}

#[derive(Debug, Serialize)]
pub struct BudgetLineResponse {
    pub account_id: Uuid,
    pub account_code: String,
    pub account_name: String,
    pub period: u32,
    pub budget_amount: f64,
    pub actual_amount: f64,
    pub variance: f64,
    pub variance_percent: f64,
}

impl From<BudgetWithVariance> for BudgetResponse {
    fn from(b: BudgetWithVariance) -> Self {
        Self {
            id: b.base.id,
            name: b.name,
            start_date: b.start_date.to_rfc3339(),
            end_date: b.end_date.to_rfc3339(),
            total_amount: b.total_amount as f64 / 100.0,
            total_actual: b.total_actual as f64 / 100.0,
            total_variance: b.total_variance as f64 / 100.0,
            variance_percent: b.variance_percent,
            status: format!("{:?}", b.status),
            lines: b.lines.into_iter().map(|l| BudgetLineResponse {
                account_id: l.account_id,
                account_code: l.account_code,
                account_name: l.account_name,
                period: l.period,
                budget_amount: l.budget_amount as f64 / 100.0,
                actual_amount: l.actual_amount as f64 / 100.0,
                variance: l.variance as f64 / 100.0,
                variance_percent: l.variance_percent,
            }).collect(),
        }
    }
}

pub async fn list_budgets(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<BudgetResponse>>> {
    let budgets = BudgetService::list_budgets(&state.pool).await?;
    Ok(Json(budgets.into_iter().map(BudgetResponse::from).collect()))
}

#[derive(Debug, Deserialize)]
pub struct CreateBudgetRequest {
    pub name: String,
    pub start_date: String,
    pub end_date: String,
    pub lines: Vec<BudgetLineRequest>,
}

#[derive(Debug, Deserialize)]
pub struct BudgetLineRequest {
    pub account_id: String,
    pub period: u32,
    pub amount: i64,
}

pub async fn create_budget(
    State(state): State<AppState>,
    Json(req): Json<CreateBudgetRequest>,
) -> ApiResult<Json<BudgetResponse>> {
    let lines: Vec<(String, u32, i64)> = req.lines.into_iter()
        .map(|l| (l.account_id, l.period, l.amount))
        .collect();
    
    let budget = BudgetService::create_budget(&state.pool, &req.name, &req.start_date, &req.end_date, lines).await?;
    Ok(Json(BudgetResponse::from(budget)))
}

#[derive(Debug, Serialize)]
pub struct LotResponse {
    pub id: Uuid,
    pub lot_number: String,
    pub product_id: Uuid,
    pub serial_number: Option<String>,
    pub manufacture_date: Option<String>,
    pub expiry_date: Option<String>,
    pub quantity: i64,
    pub status: String,
}

impl From<Lot> for LotResponse {
    fn from(l: Lot) -> Self {
        Self {
            id: l.id,
            lot_number: l.lot_number,
            product_id: l.product_id,
            serial_number: l.serial_number,
            manufacture_date: l.manufacture_date.map(|d| d.to_rfc3339()),
            expiry_date: l.expiry_date.map(|d| d.to_rfc3339()),
            quantity: l.quantity,
            status: format!("{:?}", l.status),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateLotRequest {
    pub lot_number: String,
    pub product_id: Uuid,
    pub serial_number: Option<String>,
    pub manufacture_date: Option<String>,
    pub expiry_date: Option<String>,
    pub quantity: i64,
    pub notes: Option<String>,
}

pub async fn create_lot(
    State(state): State<AppState>,
    Json(req): Json<CreateLotRequest>,
) -> ApiResult<Json<LotResponse>> {
    let lot = LotService::create_lot(
        &state.pool,
        &req.lot_number,
        req.product_id,
        req.serial_number.as_deref(),
        req.manufacture_date,
        req.expiry_date,
        req.quantity,
        req.notes.as_deref(),
    ).await?;
    Ok(Json(LotResponse::from(lot)))
}

pub async fn list_lots(
    State(state): State<AppState>,
    Query(query): Query<LotsQuery>,
) -> ApiResult<Json<Vec<LotResponse>>> {
    let lots = LotService::list_lots_for_product(&state.pool, query.product_id).await?;
    Ok(Json(lots.into_iter().map(LotResponse::from).collect()))
}

#[derive(Debug, Deserialize)]
pub struct LotsQuery {
    pub product_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct LeaveTypeResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub days_per_year: i64,
    pub carry_over: bool,
    pub status: String,
}

impl From<LeaveTypeDef> for LeaveTypeResponse {
    fn from(lt: LeaveTypeDef) -> Self {
        Self {
            id: lt.id,
            name: lt.name,
            code: lt.code,
            days_per_year: lt.days_per_year,
            carry_over: lt.carry_over,
            status: format!("{:?}", lt.status),
        }
    }
}

pub async fn list_leave_types(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<LeaveTypeResponse>>> {
    let types = LeaveService::list_leave_types(&state.pool).await?;
    Ok(Json(types.into_iter().map(LeaveTypeResponse::from).collect()))
}

#[derive(Debug, Serialize)]
pub struct LeaveRequestResponse {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub leave_type_id: Uuid,
    pub start_date: String,
    pub end_date: String,
    pub days: i64,
    pub reason: Option<String>,
    pub status: String,
    pub created_at: String,
}

impl From<LeaveRequestExtended> for LeaveRequestResponse {
    fn from(r: LeaveRequestExtended) -> Self {
        Self {
            id: r.id,
            employee_id: r.employee_id,
            leave_type_id: r.leave_type_id,
            start_date: r.start_date.to_string(),
            end_date: r.end_date.to_string(),
            days: r.days,
            reason: r.reason,
            status: format!("{:?}", r.status),
            created_at: r.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateLeaveRequestReq {
    pub employee_id: Uuid,
    pub leave_type_id: Uuid,
    pub start_date: String,
    pub end_date: String,
    pub days: i64,
    pub reason: Option<String>,
}

pub async fn create_leave_request(
    State(state): State<AppState>,
    Json(req): Json<CreateLeaveRequestReq>,
) -> ApiResult<Json<LeaveRequestResponse>> {
    let lr = LeaveService::create_leave_request(
        &state.pool,
        req.employee_id,
        req.leave_type_id,
        &req.start_date,
        &req.end_date,
        req.days,
        req.reason.as_deref(),
    ).await?;
    Ok(Json(LeaveRequestResponse::from(lr)))
}

pub async fn list_pending_leave(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<LeaveRequestResponse>>> {
    let requests = LeaveService::list_pending_leave_requests(&state.pool).await?;
    Ok(Json(requests.into_iter().map(LeaveRequestResponse::from).collect()))
}

pub async fn approve_leave(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<LeaveRequestResponse>> {
    let approver_id = Uuid::nil();
    let lr = LeaveService::approve_leave_request(&state.pool, id, approver_id).await?;
    Ok(Json(LeaveRequestResponse::from(lr)))
}

#[derive(Debug, Deserialize)]
pub struct RejectLeaveRequest {
    pub reason: String,
}

pub async fn reject_leave(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<RejectLeaveRequest>,
) -> ApiResult<Json<LeaveRequestResponse>> {
    let lr = LeaveService::reject_leave_request(&state.pool, id, &req.reason).await?;
    Ok(Json(LeaveRequestResponse::from(lr)))
}

#[derive(Debug, Serialize)]
pub struct ExpenseReportResponse {
    pub id: Uuid,
    pub report_number: String,
    pub employee_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub total_amount: f64,
    pub status: String,
    pub created_at: String,
}

impl From<ExpenseReport> for ExpenseReportResponse {
    fn from(r: ExpenseReport) -> Self {
        Self {
            id: r.id,
            report_number: r.report_number,
            employee_id: r.employee_id,
            title: r.title,
            description: r.description,
            total_amount: r.total_amount as f64 / 100.0,
            status: format!("{:?}", r.status),
            created_at: r.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ExpenseCategoryResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub status: String,
}

impl From<ExpenseCategory> for ExpenseCategoryResponse {
    fn from(c: ExpenseCategory) -> Self {
        Self {
            id: c.id,
            name: c.name,
            code: c.code,
            description: c.description,
            status: format!("{:?}", c.status),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateExpenseReportReq {
    pub employee_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub lines: Vec<ExpenseLineReq>,
}

#[derive(Debug, Deserialize)]
pub struct ExpenseLineReq {
    pub category_id: Uuid,
    pub expense_date: String,
    pub description: String,
    pub amount: i64,
}

pub async fn create_expense_report(
    State(state): State<AppState>,
    Json(req): Json<CreateExpenseReportReq>,
) -> ApiResult<Json<ExpenseReportResponse>> {
    let lines: Vec<(Uuid, &str, &str, i64)> = req.lines.iter()
        .map(|l| (l.category_id, l.expense_date.as_str(), l.description.as_str(), l.amount))
        .collect();
    
    let report = ExpenseService::create_expense_report(
        &state.pool,
        req.employee_id,
        &req.title,
        req.description.as_deref(),
        lines,
    ).await?;
    Ok(Json(ExpenseReportResponse::from(report)))
}

pub async fn list_expense_reports(
    State(state): State<AppState>,
    Query(query): Query<ExpenseReportsQuery>,
) -> ApiResult<Json<Vec<ExpenseReportResponse>>> {
    let reports = ExpenseService::list_expense_reports(&state.pool, query.employee_id).await?;
    Ok(Json(reports.into_iter().map(ExpenseReportResponse::from).collect()))
}

#[derive(Debug, Deserialize)]
pub struct ExpenseReportsQuery {
    pub employee_id: Option<Uuid>,
}

pub async fn submit_expense(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ExpenseReportResponse>> {
    let report = ExpenseService::submit_expense_report(&state.pool, id).await?;
    Ok(Json(ExpenseReportResponse::from(report)))
}

pub async fn approve_expense(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ExpenseReportResponse>> {
    let approver_id = Uuid::nil();
    let report = ExpenseService::approve_expense_report(&state.pool, id, approver_id).await?;
    Ok(Json(ExpenseReportResponse::from(report)))
}

pub async fn reject_expense(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<RejectLeaveRequest>,
) -> ApiResult<Json<ExpenseReportResponse>> {
    let report = ExpenseService::reject_expense_report(&state.pool, id, &req.reason).await?;
    Ok(Json(ExpenseReportResponse::from(report)))
}

pub async fn list_expense_categories(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<ExpenseCategoryResponse>>> {
    let categories = ExpenseService::list_expense_categories(&state.pool).await?;
    Ok(Json(categories.into_iter().map(ExpenseCategoryResponse::from).collect()))
}
