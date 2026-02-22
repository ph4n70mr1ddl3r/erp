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

use erp_finance::{FixedAsset, AssetDepreciation, FixedAssetService, DepreciationMethod};
use erp_inventory::{
    QualityInspection, InspectionType, InspectionStatus, InspectionResult, QualityInspectionService,
    NonConformanceReport, NCRSeverity, NCRStatus, NonConformanceService,
    DemandForecast, ForecastMethod, DemandForecastService,
    SafetyStock, SafetyStockService,
    ReplenishmentOrder, ReplenishmentType, ReplenishmentOrderService,
};
use erp_sales::{
    Lead, LeadStatus, LeadService,
    Opportunity, OpportunityStage, OpportunityStatus, ActivityType, OpportunityService,
};
use erp_manufacturing::{ProductionSchedule, ScheduleStatus, ProductionScheduleService};
use erp_purchasing::{SupplierScorecard, VendorPerformance, SupplierScorecardService};
use erp_core::{CustomFieldService, CustomFieldDefinition, CustomFieldType, CustomFieldValue};

#[derive(Debug, Serialize)]
pub struct FixedAssetResponse {
    pub id: Uuid,
    pub asset_code: String,
    pub name: String,
    pub category: String,
    pub cost: f64,
    pub net_book_value: f64,
    pub accumulated_depreciation: f64,
    pub status: String,
}

impl From<FixedAsset> for FixedAssetResponse {
    fn from(a: FixedAsset) -> Self {
        Self {
            id: a.id,
            asset_code: a.asset_code,
            name: a.name,
            category: a.category,
            cost: a.cost as f64 / 100.0,
            net_book_value: a.net_book_value as f64 / 100.0,
            accumulated_depreciation: a.accumulated_depreciation as f64 / 100.0,
            status: format!("{:?}", a.status),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateAssetRequest {
    pub asset_code: String,
    pub name: String,
    pub category: String,
    pub cost: i64,
    pub salvage_value: i64,
    pub useful_life_years: i32,
    pub depreciation_method: String,
    pub acquisition_date: String,
    pub location: Option<String>,
    pub description: Option<String>,
}

pub async fn create_fixed_asset(
    State(state): State<AppState>,
    Json(req): Json<CreateAssetRequest>,
) -> ApiResult<Json<FixedAssetResponse>> {
    let method = match req.depreciation_method.as_str() {
        "DecliningBalance" => DepreciationMethod::DecliningBalance,
        "SumOfYearsDigits" => DepreciationMethod::SumOfYearsDigits,
        "UnitsOfProduction" => DepreciationMethod::UnitsOfProduction,
        _ => DepreciationMethod::StraightLine,
    };
    let asset = FixedAssetService::create_asset(
        &state.pool,
        &req.asset_code,
        &req.name,
        &req.category,
        req.cost,
        req.salvage_value,
        req.useful_life_years,
        method,
        &req.acquisition_date,
        req.location.as_deref(),
        req.description.as_deref(),
    ).await?;
    Ok(Json(FixedAssetResponse::from(asset)))
}

pub async fn list_fixed_assets(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<FixedAssetResponse>>> {
    let assets = FixedAssetService::list_assets(&state.pool).await?;
    Ok(Json(assets.into_iter().map(FixedAssetResponse::from).collect()))
}

pub async fn depreciate_asset(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<FixedAssetResponse>> {
    FixedAssetService::calculate_depreciation(&state.pool, id).await?;
    let asset = FixedAssetService::get_asset(&state.pool, id).await?;
    Ok(Json(FixedAssetResponse::from(asset)))
}

#[derive(Debug, Serialize)]
pub struct InspectionResponse {
    pub id: Uuid,
    pub inspection_number: String,
    pub inspection_type: String,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub status: String,
    pub result: Option<String>,
}

impl From<QualityInspection> for InspectionResponse {
    fn from(i: QualityInspection) -> Self {
        Self {
            id: i.id,
            inspection_number: i.inspection_number,
            inspection_type: format!("{:?}", i.inspection_type),
            entity_type: i.entity_type,
            entity_id: i.entity_id,
            status: format!("{:?}", i.status),
            result: i.result.map(|r| format!("{:?}", r)),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateInspectionRequest {
    pub inspection_type: String,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub inspector_id: Option<Uuid>,
}

pub async fn create_inspection(
    State(state): State<AppState>,
    Json(req): Json<CreateInspectionRequest>,
) -> ApiResult<Json<InspectionResponse>> {
    let itype = match req.inspection_type.as_str() {
        "Incoming" => InspectionType::Incoming,
        "InProcess" => InspectionType::InProcess,
        "Final" => InspectionType::Final,
        "Outgoing" => InspectionType::Outgoing,
        _ => InspectionType::Incoming,
    };
    let inspection = QualityInspectionService::create_inspection(
        &state.pool,
        itype,
        &req.entity_type,
        req.entity_id,
        req.inspector_id,
    ).await?;
    Ok(Json(InspectionResponse::from(inspection)))
}

#[derive(Debug, Deserialize)]
pub struct CompleteInspectionRequest {
    pub result: String,
    pub notes: Option<String>,
}

pub async fn complete_inspection(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CompleteInspectionRequest>,
) -> ApiResult<Json<InspectionResponse>> {
    let result = match req.result.as_str() {
        "Pass" => InspectionResult::Pass,
        "Fail" => InspectionResult::Fail,
        _ => InspectionResult::Conditional,
    };
    let inspection = QualityInspectionService::complete_inspection(&state.pool, id, result, req.notes.as_deref()).await?;
    Ok(Json(InspectionResponse::from(inspection)))
}

#[derive(Debug, Serialize)]
pub struct NCRResponse {
    pub id: Uuid,
    pub ncr_number: String,
    pub source_type: String,
    pub description: String,
    pub severity: String,
    pub status: String,
}

impl From<NonConformanceReport> for NCRResponse {
    fn from(n: NonConformanceReport) -> Self {
        Self {
            id: n.id,
            ncr_number: n.ncr_number,
            source_type: n.source_type,
            description: n.description,
            severity: format!("{:?}", n.severity),
            status: format!("{:?}", n.status),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateNCRRequest {
    pub source_type: String,
    pub source_id: Uuid,
    pub description: String,
    pub severity: String,
}

pub async fn create_ncr(
    State(state): State<AppState>,
    Json(req): Json<CreateNCRRequest>,
) -> ApiResult<Json<NCRResponse>> {
    let severity = match req.severity.as_str() {
        "Major" => NCRSeverity::Major,
        "Critical" => NCRSeverity::Critical,
        _ => NCRSeverity::Minor,
    };
    let ncr = NonConformanceService::create_ncr(&state.pool, &req.source_type, req.source_id, &req.description, severity).await?;
    Ok(Json(NCRResponse::from(ncr)))
}

#[derive(Debug, Serialize)]
pub struct LeadResponse {
    pub id: Uuid,
    pub lead_number: String,
    pub company_name: String,
    pub contact_name: Option<String>,
    pub email: Option<String>,
    pub estimated_value: f64,
    pub status: String,
}

impl From<Lead> for LeadResponse {
    fn from(l: Lead) -> Self {
        Self {
            id: l.id,
            lead_number: l.lead_number,
            company_name: l.company_name,
            contact_name: l.contact_name,
            email: l.email,
            estimated_value: l.estimated_value as f64 / 100.0,
            status: format!("{:?}", l.status),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateLeadRequest {
    pub company_name: String,
    pub contact_name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub source: Option<String>,
    pub industry: Option<String>,
    pub estimated_value: i64,
    pub assigned_to: Option<Uuid>,
}

pub async fn create_lead(
    State(state): State<AppState>,
    Json(req): Json<CreateLeadRequest>,
) -> ApiResult<Json<LeadResponse>> {
    let lead = LeadService::create(
        &state.pool,
        &req.company_name,
        req.contact_name.as_deref(),
        req.email.as_deref(),
        req.phone.as_deref(),
        req.source.as_deref(),
        req.industry.as_deref(),
        req.estimated_value,
        req.assigned_to,
    ).await?;
    Ok(Json(LeadResponse::from(lead)))
}

pub async fn list_leads(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<LeadResponse>>> {
    let leads = LeadService::list(&state.pool, None).await?;
    Ok(Json(leads.into_iter().map(LeadResponse::from).collect()))
}

#[derive(Debug, Serialize)]
pub struct OpportunityResponse {
    pub id: Uuid,
    pub opportunity_number: String,
    pub name: String,
    pub stage: String,
    pub probability: i32,
    pub amount: f64,
    pub status: String,
}

impl From<Opportunity> for OpportunityResponse {
    fn from(o: Opportunity) -> Self {
        Self {
            id: o.id,
            opportunity_number: o.opportunity_number,
            name: o.name,
            stage: format!("{:?}", o.stage),
            probability: o.probability,
            amount: o.amount as f64 / 100.0,
            status: format!("{:?}", o.status),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateOpportunityRequest {
    pub name: String,
    pub customer_id: Option<Uuid>,
    pub lead_id: Option<Uuid>,
    pub amount: i64,
    pub expected_close_date: Option<String>,
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
}

pub async fn create_opportunity(
    State(state): State<AppState>,
    Json(req): Json<CreateOpportunityRequest>,
) -> ApiResult<Json<OpportunityResponse>> {
    let opp = OpportunityService::create(
        &state.pool,
        &req.name,
        req.customer_id,
        req.lead_id,
        req.amount,
        req.expected_close_date.as_deref(),
        req.description.as_deref(),
        req.assigned_to,
    ).await?;
    Ok(Json(OpportunityResponse::from(opp)))
}

pub async fn list_opportunities(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<OpportunityResponse>>> {
    let opps = OpportunityService::list(&state.pool, None).await?;
    Ok(Json(opps.into_iter().map(OpportunityResponse::from).collect()))
}

#[derive(Debug, Deserialize)]
pub struct UpdateStageRequest {
    pub stage: String,
}

pub async fn update_opportunity_stage(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateStageRequest>,
) -> ApiResult<Json<OpportunityResponse>> {
    let stage = match req.stage.as_str() {
        "Qualification" => OpportunityStage::Qualification,
        "Proposal" => OpportunityStage::Proposal,
        "Negotiation" => OpportunityStage::Negotiation,
        "ClosedWon" => OpportunityStage::ClosedWon,
        "ClosedLost" => OpportunityStage::ClosedLost,
        _ => OpportunityStage::Prospecting,
    };
    let opp = OpportunityService::update_stage(&state.pool, id, stage).await?;
    Ok(Json(OpportunityResponse::from(opp)))
}

#[derive(Debug, Serialize)]
pub struct ScheduleResponse {
    pub id: Uuid,
    pub schedule_number: String,
    pub work_order_id: Uuid,
    pub work_center_id: Uuid,
    pub start_time: String,
    pub end_time: String,
    pub status: String,
}

impl From<ProductionSchedule> for ScheduleResponse {
    fn from(s: ProductionSchedule) -> Self {
        Self {
            id: s.id,
            schedule_number: s.schedule_number,
            work_order_id: s.work_order_id,
            work_center_id: s.work_center_id,
            start_time: s.start_time.to_rfc3339(),
            end_time: s.end_time.to_rfc3339(),
            status: format!("{:?}", s.status),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateScheduleRequest {
    pub work_order_id: Uuid,
    pub work_center_id: Uuid,
    pub start_time: String,
    pub end_time: String,
    pub notes: Option<String>,
}

pub async fn create_production_schedule(
    State(state): State<AppState>,
    Json(req): Json<CreateScheduleRequest>,
) -> ApiResult<Json<ScheduleResponse>> {
    let schedule = ProductionScheduleService::create_schedule(
        &state.pool,
        req.work_order_id,
        req.work_center_id,
        &req.start_time,
        &req.end_time,
        req.notes.as_deref(),
    ).await?;
    Ok(Json(ScheduleResponse::from(schedule)))
}

pub async fn list_schedules(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<ScheduleResponse>>> {
    let schedules = ProductionScheduleService::list(&state.pool, None).await?;
    Ok(Json(schedules.into_iter().map(ScheduleResponse::from).collect()))
}

#[derive(Debug, Serialize)]
pub struct ScorecardResponse {
    pub id: Uuid,
    pub vendor_id: Uuid,
    pub period: String,
    pub on_time_delivery: i32,
    pub quality_score: i32,
    pub overall_score: i32,
    pub total_orders: i32,
}

impl From<SupplierScorecard> for ScorecardResponse {
    fn from(s: SupplierScorecard) -> Self {
        Self {
            id: s.id,
            vendor_id: s.vendor_id,
            period: s.period,
            on_time_delivery: s.on_time_delivery,
            quality_score: s.quality_score,
            overall_score: s.overall_score,
            total_orders: s.total_orders,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateScorecardRequest {
    pub vendor_id: Uuid,
    pub period: String,
}

pub async fn create_scorecard(
    State(state): State<AppState>,
    Json(req): Json<CreateScorecardRequest>,
) -> ApiResult<Json<ScorecardResponse>> {
    let scorecard = SupplierScorecardService::create_scorecard(&state.pool, req.vendor_id, &req.period).await?;
    Ok(Json(ScorecardResponse::from(scorecard)))
}

pub async fn list_scorecards(
    State(state): State<AppState>,
    Path(vendor_id): Path<Uuid>,
) -> ApiResult<Json<Vec<ScorecardResponse>>> {
    let scorecards = SupplierScorecardService::get_for_vendor(&state.pool, vendor_id).await?;
    Ok(Json(scorecards.into_iter().map(ScorecardResponse::from).collect()))
}

#[derive(Debug, Serialize)]
pub struct CustomFieldDefResponse {
    pub id: Uuid,
    pub entity_type: String,
    pub field_name: String,
    pub field_label: String,
    pub field_type: String,
    pub required: bool,
}

impl From<CustomFieldDefinition> for CustomFieldDefResponse {
    fn from(d: CustomFieldDefinition) -> Self {
        Self {
            id: d.id,
            entity_type: d.entity_type,
            field_name: d.field_name,
            field_label: d.field_label,
            field_type: format!("{:?}", d.field_type),
            required: d.required,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateCustomFieldRequest {
    pub entity_type: String,
    pub field_name: String,
    pub field_label: String,
    pub field_type: String,
    pub required: bool,
    pub options: Option<String>,
    pub sort_order: i32,
}

pub async fn create_custom_field(
    State(state): State<AppState>,
    Json(req): Json<CreateCustomFieldRequest>,
) -> ApiResult<Json<CustomFieldDefResponse>> {
    let ftype = match req.field_type.as_str() {
        "Number" => CustomFieldType::Number,
        "Date" => CustomFieldType::Date,
        "Boolean" => CustomFieldType::Boolean,
        "Select" => CustomFieldType::Select,
        "MultiSelect" => CustomFieldType::MultiSelect,
        _ => CustomFieldType::Text,
    };
    let def = CustomFieldService::create_definition(
        &state.pool,
        &req.entity_type,
        &req.field_name,
        &req.field_label,
        ftype,
        req.required,
        req.options.as_deref(),
        req.sort_order,
    ).await?;
    Ok(Json(CustomFieldDefResponse::from(def)))
}

pub async fn list_custom_fields(
    State(state): State<AppState>,
    Path(entity_type): Path<String>,
) -> ApiResult<Json<Vec<CustomFieldDefResponse>>> {
    let defs = CustomFieldService::get_definitions_for_entity(&state.pool, &entity_type).await?;
    Ok(Json(defs.into_iter().map(CustomFieldDefResponse::from).collect()))
}

#[derive(Debug, Deserialize)]
pub struct SetCustomValueRequest {
    pub definition_id: Uuid,
    pub entity_id: Uuid,
    pub value: String,
}

pub async fn set_custom_value(
    State(state): State<AppState>,
    Json(req): Json<SetCustomValueRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    CustomFieldService::set_value(&state.pool, req.definition_id, req.entity_id, &req.value).await?;
    Ok(Json(serde_json::json!({"success": true})))
}
