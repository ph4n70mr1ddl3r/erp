use axum::{extract::{Query, State}, Json, routing::{get, post}};
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{Pagination, BaseEntity, Status};
use erp_tax::{
    TaxJurisdiction, TaxRate, TaxType, TaxCalculationMethod,
    TaxExemption, ExemptionType,
    TaxJurisdictionService, TaxRateService, TaxCalculationService, TaxExemptionService,
};

#[derive(Serialize)]
pub struct TaxJurisdictionResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub country_code: String,
    pub status: String,
}

impl From<TaxJurisdiction> for TaxJurisdictionResponse {
    fn from(j: TaxJurisdiction) -> Self {
        Self {
            id: j.base.id,
            code: j.code,
            name: j.name,
            country_code: j.country_code,
            status: format!("{:?}", j.status),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateJurisdictionRequest {
    pub code: String,
    pub name: String,
    pub country_code: String,
    pub state_code: Option<String>,
    pub county: Option<String>,
    pub city: Option<String>,
}

pub async fn list_jurisdictions(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<TaxJurisdictionResponse>>> {
    let svc = TaxJurisdictionService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(
        res.items.into_iter().map(TaxJurisdictionResponse::from).collect(),
        res.total,
        Pagination { page: res.page, per_page: res.per_page },
    )))
}

pub async fn create_jurisdiction(
    State(state): State<AppState>,
    Json(req): Json<CreateJurisdictionRequest>,
) -> ApiResult<Json<TaxJurisdictionResponse>> {
    let svc = TaxJurisdictionService::new();
    let jurisdiction = TaxJurisdiction {
        base: BaseEntity::new(),
        code: req.code,
        name: req.name,
        country_code: req.country_code,
        state_code: req.state_code,
        county: req.county,
        city: req.city,
        postal_code_from: None,
        postal_code_to: None,
        parent_jurisdiction_id: None,
        status: Status::Active,
        effective_from: Utc::now(),
        effective_to: None,
    };
    Ok(Json(TaxJurisdictionResponse::from(svc.create(&state.pool, jurisdiction).await?)))
}

#[derive(Serialize)]
pub struct TaxRateResponse {
    pub id: Uuid,
    pub jurisdiction_id: Uuid,
    pub name: String,
    pub code: String,
    pub rate: f64,
    pub tax_type: String,
    pub status: String,
}

impl From<TaxRate> for TaxRateResponse {
    fn from(r: TaxRate) -> Self {
        Self {
            id: r.base.id,
            jurisdiction_id: r.jurisdiction_id,
            name: r.name,
            code: r.code,
            rate: r.rate,
            tax_type: format!("{:?}", r.tax_type),
            status: format!("{:?}", r.status),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateTaxRateRequest {
    pub jurisdiction_id: Uuid,
    pub name: String,
    pub code: String,
    pub rate: f64,
    pub tax_type: Option<String>,
    pub is_compound: Option<bool>,
    pub is_recoverable: Option<bool>,
}

pub async fn list_tax_rates(
    State(state): State<AppState>,
    Query(_jurisdiction_id): Query<Option<Uuid>>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<TaxRateResponse>>> {
    let svc = TaxRateService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(
        res.items.into_iter().map(TaxRateResponse::from).collect(),
        res.total,
        Pagination { page: res.page, per_page: res.per_page },
    )))
}

pub async fn create_tax_rate(
    State(state): State<AppState>,
    Json(req): Json<CreateTaxRateRequest>,
) -> ApiResult<Json<TaxRateResponse>> {
    let svc = TaxRateService::new();
    let rate = TaxRate {
        base: BaseEntity::new(),
        jurisdiction_id: req.jurisdiction_id,
        tax_type: match req.tax_type.as_deref() {
            Some("VAT") => TaxType::VAT,
            Some("GST") => TaxType::GST,
            Some("PST") => TaxType::PST,
            Some("HST") => TaxType::HST,
            Some("Withholding") => TaxType::Withholding,
            Some("Excise") => TaxType::Excise,
            Some("Custom") => TaxType::Custom,
            _ => TaxType::SalesTax,
        },
        name: req.name,
        code: req.code,
        rate: req.rate,
        is_compound: req.is_compound.unwrap_or(false),
        is_recoverable: req.is_recoverable.unwrap_or(false),
        calculation_method: TaxCalculationMethod::Exclusive,
        status: Status::Active,
        effective_from: Utc::now(),
        effective_to: None,
        priority: 1,
        min_amount: None,
        max_amount: None,
    };
    Ok(Json(TaxRateResponse::from(svc.create(&state.pool, rate).await?)))
}

#[derive(Deserialize)]
pub struct CalculateTaxRequest {
    pub jurisdiction_id: Uuid,
    pub amount: i64,
    pub customer_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct CalculateTaxResponse {
    pub taxable_amount: f64,
    pub total_tax: f64,
}

pub async fn calculate_tax(
    State(state): State<AppState>,
    Json(req): Json<CalculateTaxRequest>,
) -> ApiResult<Json<CalculateTaxResponse>> {
    let result = TaxCalculationService::calculate(
        &state.pool,
        req.jurisdiction_id,
        req.amount,
        req.customer_id,
        None,
    ).await?;
    
    Ok(Json(CalculateTaxResponse {
        taxable_amount: result.taxable_amount.to_decimal(),
        total_tax: result.total_tax.to_decimal(),
    }))
}

#[derive(Serialize)]
pub struct TaxExemptionResponse {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub exemption_type: String,
    pub certificate_number: String,
    pub status: String,
}

impl From<TaxExemption> for TaxExemptionResponse {
    fn from(e: TaxExemption) -> Self {
        Self {
            id: e.base.id,
            customer_id: e.customer_id,
            exemption_type: format!("{:?}", e.exemption_type),
            certificate_number: e.certificate_number,
            status: format!("{:?}", e.status),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateExemptionRequest {
    pub customer_id: Uuid,
    pub exemption_type: String,
    pub certificate_number: String,
    pub jurisdiction_id: Option<Uuid>,
    pub issue_date: String,
    pub expiry_date: Option<String>,
}

pub async fn create_exemption(
    State(state): State<AppState>,
    Json(req): Json<CreateExemptionRequest>,
) -> ApiResult<Json<TaxExemptionResponse>> {
    let svc = TaxExemptionService::new();
    let exemption = TaxExemption {
        base: BaseEntity::new(),
        customer_id: req.customer_id,
        exemption_type: match req.exemption_type.as_str() {
            "Manufacturing" => ExemptionType::Manufacturing,
            "Agricultural" => ExemptionType::Agricultural,
            "Government" => ExemptionType::Government,
            "NonProfit" => ExemptionType::NonProfit,
            "Educational" => ExemptionType::Educational,
            "DirectPay" => ExemptionType::DirectPay,
            "Other" => ExemptionType::Other,
            _ => ExemptionType::Resale,
        },
        certificate_number: req.certificate_number,
        jurisdiction_id: req.jurisdiction_id,
        issue_date: chrono::DateTime::parse_from_rfc3339(&req.issue_date)
            .map(|d| d.with_timezone(&chrono::Utc))
            .unwrap_or_else(|_| chrono::Utc::now()),
        expiry_date: req.expiry_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
            .map(|d| d.with_timezone(&chrono::Utc)),
        status: Status::Active,
        document_url: None,
        notes: None,
    };
    Ok(Json(TaxExemptionResponse::from(svc.create(&state.pool, exemption).await?)))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/jurisdictions", get(list_jurisdictions).post(create_jurisdiction))
        .route("/rates", get(list_tax_rates).post(create_tax_rate))
        .route("/calculate", post(calculate_tax))
        .route("/exemptions", post(create_exemption))
}
