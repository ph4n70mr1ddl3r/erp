use axum::{
    extract::{Path, Query, State},
    Json,
    Router,
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::AppState;
use crate::error::ApiResult;
use erp_credit_notes::{
    CreditNoteService, CreditNote, CreditNoteLine, CreditNoteReason,
    CreateCreditNoteRequest, CreateCreditNoteLineRequest, ApplyCreditNoteRequest,
    CreditNoteApplication,
};

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Deserialize)]
pub struct CreateCreditNoteBody {
    pub customer_id: Uuid,
    pub invoice_id: Option<Uuid>,
    pub credit_note_date: Option<String>,
    pub lines: Vec<CreateCreditNoteLineBody>,
    pub reason: String,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateCreditNoteLineBody {
    pub product_id: Option<Uuid>,
    pub description: String,
    pub quantity: i64,
    pub unit_price: i64,
}

#[derive(Deserialize)]
pub struct ApplyCreditNoteBody {
    pub invoice_id: Uuid,
    pub amount: i64,
}

#[derive(Serialize)]
pub struct CreditNoteResponse {
    pub id: String,
    pub credit_note_number: String,
    pub customer_id: String,
    pub customer_name: Option<String>,
    pub invoice_id: Option<String>,
    pub invoice_number: Option<String>,
    pub credit_note_date: String,
    pub lines: Vec<CreditNoteLineResponse>,
    pub subtotal: i64,
    pub tax_amount: i64,
    pub total: i64,
    pub currency: String,
    pub reason: String,
    pub notes: Option<String>,
    pub status: String,
    pub applied_amount: i64,
    pub available_amount: i64,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct CreditNoteLineResponse {
    pub id: String,
    pub product_id: Option<String>,
    pub description: String,
    pub quantity: i64,
    pub unit_price: i64,
    pub line_total: i64,
}

#[derive(Serialize)]
pub struct CreditNoteListResponse {
    pub items: Vec<CreditNoteResponse>,
    pub total: u64,
    pub page: u32,
    pub per_page: u32,
}

#[derive(Serialize)]
pub struct CreditNoteApplicationResponse {
    pub id: String,
    pub credit_note_id: String,
    pub invoice_id: String,
    pub amount: i64,
    pub currency: String,
    pub applied_at: String,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_credit_notes).post(create_credit_note))
        .route("/:id", get(get_credit_note))
        .route("/:id/issue", post(issue_credit_note))
        .route("/:id/void", post(void_credit_note))
        .route("/:id/apply", post(apply_credit_note))
        .route("/customer/:customer_id", get(list_customer_credit_notes))
}

impl From<CreditNote> for CreditNoteResponse {
    fn from(cn: CreditNote) -> Self {
        let available = cn.total.amount - cn.applied_amount.amount;
        Self {
            id: cn.base.id.to_string(),
            credit_note_number: cn.credit_note_number,
            customer_id: cn.customer_id.to_string(),
            customer_name: None,
            invoice_id: cn.invoice_id.map(|id: Uuid| id.to_string()),
            invoice_number: None,
            credit_note_date: cn.credit_note_date.to_rfc3339(),
            lines: cn.lines.into_iter().map(|l: CreditNoteLine| l.into()).collect(),
            subtotal: cn.subtotal.amount,
            tax_amount: cn.tax_amount.amount,
            total: cn.total.amount,
            currency: cn.total.currency.to_string(),
            reason: format!("{:?}", cn.reason),
            notes: cn.notes,
            status: format!("{:?}", cn.status),
            applied_amount: cn.applied_amount.amount,
            available_amount: available,
            created_at: cn.base.created_at.to_rfc3339(),
            updated_at: cn.base.updated_at.to_rfc3339(),
        }
    }
}

impl From<CreditNoteLine> for CreditNoteLineResponse {
    fn from(line: CreditNoteLine) -> Self {
        Self {
            id: line.id.to_string(),
            product_id: line.product_id.map(|id: Uuid| id.to_string()),
            description: line.description,
            quantity: line.quantity,
            unit_price: line.unit_price.amount,
            line_total: line.line_total.amount,
        }
    }
}

impl From<CreditNoteApplication> for CreditNoteApplicationResponse {
    fn from(app: CreditNoteApplication) -> Self {
        Self {
            id: app.id.to_string(),
            credit_note_id: app.credit_note_id.to_string(),
            invoice_id: app.invoice_id.to_string(),
            amount: app.amount.amount,
            currency: app.amount.currency.to_string(),
            applied_at: app.applied_at.to_rfc3339(),
        }
    }
}

fn parse_reason(s: &str) -> CreditNoteReason {
    match s.to_lowercase().as_str() {
        "return" => CreditNoteReason::Return,
        "damaged" => CreditNoteReason::Damaged,
        "wrongitem" => CreditNoteReason::WrongItem,
        "pricingerror" => CreditNoteReason::PricingError,
        "qualityissue" => CreditNoteReason::QualityIssue,
        _ => CreditNoteReason::Other,
    }
}

pub async fn list_credit_notes(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> ApiResult<Json<CreditNoteListResponse>> {
    let page = query.page.unwrap_or(1);
    let per_page = query.per_page.unwrap_or(20);

    let result = CreditNoteService::list(&state.pool, page, per_page).await?;

    Ok(Json(CreditNoteListResponse {
        items: result.items.into_iter().map(CreditNoteResponse::from).collect(),
        total: result.total,
        page: result.page,
        per_page: result.per_page,
    }))
}

pub async fn get_credit_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<CreditNoteResponse>> {
    let cn = CreditNoteService::get(&state.pool, id).await?;
    Ok(Json(CreditNoteResponse::from(cn)))
}

pub async fn create_credit_note(
    State(state): State<AppState>,
    Json(body): Json<CreateCreditNoteBody>,
) -> ApiResult<Json<CreditNoteResponse>> {
    let req = CreateCreditNoteRequest {
        customer_id: body.customer_id,
        invoice_id: body.invoice_id,
        credit_note_date: body.credit_note_date.and_then(|d| {
            DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&Utc))
        }),
        lines: body.lines.into_iter().map(|l| CreateCreditNoteLineRequest {
            product_id: l.product_id,
            description: l.description,
            quantity: l.quantity,
            unit_price: l.unit_price,
        }).collect(),
        reason: parse_reason(&body.reason),
        notes: body.notes,
    };

    let cn = CreditNoteService::create(&state.pool, req).await?;

    Ok(Json(CreditNoteResponse::from(cn)))
}

pub async fn issue_credit_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<CreditNoteResponse>> {
    let cn = CreditNoteService::issue(&state.pool, id).await?;
    Ok(Json(CreditNoteResponse::from(cn)))
}

pub async fn void_credit_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<CreditNoteResponse>> {
    let cn = CreditNoteService::void(&state.pool, id).await?;
    Ok(Json(CreditNoteResponse::from(cn)))
}

pub async fn apply_credit_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<ApplyCreditNoteBody>,
) -> ApiResult<Json<CreditNoteApplicationResponse>> {
    let req = ApplyCreditNoteRequest {
        invoice_id: body.invoice_id,
        amount: body.amount,
    };

    let app = CreditNoteService::apply_to_invoice(&state.pool, id, req).await?;

    Ok(Json(CreditNoteApplicationResponse::from(app)))
}

pub async fn list_customer_credit_notes(
    State(state): State<AppState>,
    Path(customer_id): Path<Uuid>,
) -> ApiResult<Json<Vec<CreditNoteResponse>>> {
    let notes: Vec<CreditNote> = CreditNoteService::list_by_customer(&state.pool, customer_id).await?;

    Ok(Json(notes.into_iter().map(CreditNoteResponse::from).collect()))
}
