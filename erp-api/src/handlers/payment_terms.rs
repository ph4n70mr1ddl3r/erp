use axum::{extract::{Path, Query, State}, Json};
use chrono::Utc;
use erp_core::{BaseEntity, Pagination, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::AppState;
use crate::error::ApiResult;
use erp_payment_terms::{PaymentTerm, PaymentTermService};

#[derive(Deserialize)]
pub struct CreatePaymentTermRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub due_days: i32,
    pub discount_days: Option<i32>,
    pub discount_percent: Option<f64>,
    pub is_default: Option<bool>,
}

#[derive(Deserialize)]
pub struct UpdatePaymentTermRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub due_days: i32,
    pub discount_days: Option<i32>,
    pub discount_percent: Option<f64>,
    pub is_default: Option<bool>,
    pub status: Option<String>,
}

#[derive(Deserialize)]
pub struct CalculateRequest {
    pub invoice_date: String,
    pub amount: i64,
}

#[derive(Serialize)]
pub struct PaymentTermResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub due_days: i32,
    pub discount_days: Option<i32>,
    pub discount_percent: Option<f64>,
    pub is_default: bool,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct CalculationResponse {
    pub term_id: Uuid,
    pub invoice_date: String,
    pub due_date: String,
    pub discount_date: Option<String>,
    pub discount_amount: Option<i64>,
}

impl From<PaymentTerm> for PaymentTermResponse {
    fn from(t: PaymentTerm) -> Self {
        Self {
            id: t.base.id,
            code: t.code,
            name: t.name,
            description: t.description,
            due_days: t.due_days,
            discount_days: t.discount_days,
            discount_percent: t.discount_percent,
            is_default: t.is_default,
            status: format!("{:?}", t.status),
            created_at: t.base.created_at.to_rfc3339(),
            updated_at: t.base.updated_at.to_rfc3339(),
        }
    }
}

pub async fn list_payment_terms(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<PaymentTermResponse>>> {
    let svc = PaymentTermService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(
        res.items.into_iter().map(PaymentTermResponse::from).collect(),
        res.total,
        Pagination {
            page: res.page,
            per_page: res.per_page,
        },
    )))
}

pub async fn get_payment_term(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<PaymentTermResponse>> {
    let svc = PaymentTermService::new();
    Ok(Json(PaymentTermResponse::from(svc.get(&state.pool, id).await?)))
}

pub async fn get_payment_term_by_code(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> ApiResult<Json<PaymentTermResponse>> {
    let svc = PaymentTermService::new();
    Ok(Json(PaymentTermResponse::from(
        svc.get_by_code(&state.pool, &code).await?,
    )))
}

pub async fn get_default_payment_term(
    State(state): State<AppState>,
) -> ApiResult<Json<Option<PaymentTermResponse>>> {
    let svc = PaymentTermService::new();
    let term = svc.get_default(&state.pool).await?;
    Ok(Json(term.map(PaymentTermResponse::from)))
}

pub async fn create_payment_term(
    State(state): State<AppState>,
    Json(req): Json<CreatePaymentTermRequest>,
) -> ApiResult<Json<PaymentTermResponse>> {
    let svc = PaymentTermService::new();
    let term = PaymentTerm {
        base: BaseEntity::new(),
        code: req.code,
        name: req.name,
        description: req.description,
        due_days: req.due_days,
        discount_days: req.discount_days,
        discount_percent: req.discount_percent,
        is_default: req.is_default.unwrap_or(false),
        status: Status::Active,
    };
    Ok(Json(PaymentTermResponse::from(
        svc.create(&state.pool, term).await?,
    )))
}

pub async fn update_payment_term(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdatePaymentTermRequest>,
) -> ApiResult<Json<PaymentTermResponse>> {
    let svc = PaymentTermService::new();
    let existing = svc.get(&state.pool, id).await?;
    let term = PaymentTerm {
        base: existing.base,
        code: req.code,
        name: req.name,
        description: req.description,
        due_days: req.due_days,
        discount_days: req.discount_days,
        discount_percent: req.discount_percent,
        is_default: req.is_default.unwrap_or(false),
        status: match req.status.as_deref() {
            Some("Inactive") => Status::Inactive,
            _ => Status::Active,
        },
    };
    Ok(Json(PaymentTermResponse::from(
        svc.update(&state.pool, term).await?,
    )))
}

pub async fn delete_payment_term(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let svc = PaymentTermService::new();
    svc.delete(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "deleted" })))
}

pub async fn set_default_payment_term(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let svc = PaymentTermService::new();
    svc.set_default(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "set_as_default" })))
}

pub async fn calculate_payment_term(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CalculateRequest>,
) -> ApiResult<Json<CalculationResponse>> {
    let svc = PaymentTermService::new();
    let term = svc.get(&state.pool, id).await?;
    
    let invoice_date = chrono::DateTime::parse_from_rfc3339(&req.invoice_date)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());
    
    let calc = svc.calculate_dates(&term, invoice_date, req.amount);
    
    Ok(Json(CalculationResponse {
        term_id: calc.term_id,
        invoice_date: calc.invoice_date.to_rfc3339(),
        due_date: calc.due_date.to_rfc3339(),
        discount_date: calc.discount_date.map(|d| d.to_rfc3339()),
        discount_amount: calc.discount_amount,
    }))
}

pub fn routes() -> axum::Router<crate::db::AppState> {
    axum::Router::new()
        .route("/", axum::routing::get(list_payment_terms).post(create_payment_term))
        .route("/default", axum::routing::get(get_default_payment_term))
        .route("/code/:code", axum::routing::get(get_payment_term_by_code))
        .route(
            "/:id",
            axum::routing::get(get_payment_term)
                .put(update_payment_term)
                .delete(delete_payment_term),
        )
        .route("/:id/set-default", axum::routing::post(set_default_payment_term))
        .route("/:id/calculate", axum::routing::post(calculate_payment_term))
}
