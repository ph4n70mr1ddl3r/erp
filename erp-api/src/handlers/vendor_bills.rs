use axum::{extract::{Path, Query, State}, Json};
use chrono::Utc;
use erp_core::Pagination;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::AppState;
use crate::error::ApiResult;
use erp_vendor_bills::{ThreeWayMatchResult, VendorBill, VendorBillLineCreateRequest, VendorBillService};

#[derive(Deserialize)]
pub struct CreateVendorBillRequest {
    pub vendor_invoice_number: String,
    pub vendor_id: Uuid,
    pub purchase_order_id: Option<Uuid>,
    pub bill_date: String,
    pub due_date: String,
    pub lines: Vec<CreateVendorBillLineRequest>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateVendorBillLineRequest {
    pub po_line_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub description: String,
    pub quantity: i64,
    pub unit_price: i64,
    pub tax_rate: f64,
}

#[derive(Deserialize)]
pub struct RecordPaymentRequest {
    pub payment_id: Uuid,
    pub amount: i64,
}

#[derive(Serialize)]
pub struct VendorBillResponse {
    pub id: Uuid,
    pub bill_number: String,
    pub vendor_invoice_number: String,
    pub vendor_id: Uuid,
    pub purchase_order_id: Option<Uuid>,
    pub bill_date: String,
    pub due_date: String,
    pub lines: Vec<VendorBillLineResponse>,
    pub subtotal: i64,
    pub tax_amount: i64,
    pub total: i64,
    pub amount_paid: i64,
    pub status: String,
    pub match_status: String,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize)]
pub struct VendorBillLineResponse {
    pub id: Uuid,
    pub bill_id: Uuid,
    pub po_line_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub description: String,
    pub quantity: i64,
    pub unit_price: i64,
    pub tax_rate: f64,
    pub line_total: i64,
    pub match_quantity: i64,
    pub match_status: String,
}

#[derive(Serialize)]
pub struct ThreeWayMatchResponse {
    pub bill_id: Uuid,
    pub po_id: Option<Uuid>,
    pub total_matched_lines: i32,
    pub total_unmatched_lines: i32,
    pub total_exceptions: i32,
    pub match_status: String,
    pub exceptions: Vec<MatchExceptionResponse>,
}

#[derive(Serialize)]
pub struct MatchExceptionResponse {
    pub bill_line_id: Uuid,
    pub exception_type: String,
    pub expected_value: String,
    pub actual_value: String,
    pub message: String,
}

impl From<VendorBill> for VendorBillResponse {
    fn from(b: VendorBill) -> Self {
        Self {
            id: b.base.id,
            bill_number: b.bill_number,
            vendor_invoice_number: b.vendor_invoice_number,
            vendor_id: b.vendor_id,
            purchase_order_id: b.purchase_order_id,
            bill_date: b.bill_date.to_rfc3339(),
            due_date: b.due_date.to_rfc3339(),
            lines: b.lines.into_iter().map(VendorBillLineResponse::from).collect(),
            subtotal: b.subtotal.amount,
            tax_amount: b.tax_amount.amount,
            total: b.total.amount,
            amount_paid: b.amount_paid.amount,
            status: format!("{:?}", b.status),
            match_status: format!("{:?}", b.match_status),
            notes: b.notes,
            created_at: b.base.created_at.to_rfc3339(),
            updated_at: b.base.updated_at.to_rfc3339(),
        }
    }
}

impl From<erp_vendor_bills::VendorBillLine> for VendorBillLineResponse {
    fn from(l: erp_vendor_bills::VendorBillLine) -> Self {
        Self {
            id: l.id,
            bill_id: l.bill_id,
            po_line_id: l.po_line_id,
            product_id: l.product_id,
            description: l.description,
            quantity: l.quantity,
            unit_price: l.unit_price.amount,
            tax_rate: l.tax_rate,
            line_total: l.line_total.amount,
            match_quantity: l.match_quantity,
            match_status: format!("{:?}", l.match_status),
        }
    }
}

impl From<ThreeWayMatchResult> for ThreeWayMatchResponse {
    fn from(r: ThreeWayMatchResult) -> Self {
        Self {
            bill_id: r.bill_id,
            po_id: r.po_id,
            total_matched_lines: r.total_matched_lines,
            total_unmatched_lines: r.total_unmatched_lines,
            total_exceptions: r.total_exceptions,
            match_status: format!("{:?}", r.match_status),
            exceptions: r.exceptions.into_iter().map(MatchExceptionResponse::from).collect(),
        }
    }
}

impl From<erp_vendor_bills::MatchException> for MatchExceptionResponse {
    fn from(e: erp_vendor_bills::MatchException) -> Self {
        Self {
            bill_line_id: e.bill_line_id,
            exception_type: format!("{:?}", e.exception_type),
            expected_value: e.expected_value,
            actual_value: e.actual_value,
            message: e.message,
        }
    }
}

pub async fn list_vendor_bills(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<VendorBillResponse>>> {
    let res = VendorBillService::list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(
        res.items.into_iter().map(VendorBillResponse::from).collect(),
        res.total,
        Pagination {
            page: res.page,
            per_page: res.per_page,
        },
    )))
}

pub async fn get_vendor_bill(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<VendorBillResponse>> {
    Ok(Json(VendorBillResponse::from(VendorBillService::get(&state.pool, id).await?)))
}

pub async fn list_vendor_bills_by_vendor(
    State(state): State<AppState>,
    Path(vendor_id): Path<Uuid>,
) -> ApiResult<Json<Vec<VendorBillResponse>>> {
    let bills = VendorBillService::list_by_vendor(&state.pool, vendor_id).await?;
    Ok(Json(bills.into_iter().map(VendorBillResponse::from).collect()))
}

pub async fn create_vendor_bill(
    State(state): State<AppState>,
    Json(req): Json<CreateVendorBillRequest>,
) -> ApiResult<Json<VendorBillResponse>> {
    let bill_date = chrono::DateTime::parse_from_rfc3339(&req.bill_date)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    let due_date = chrono::DateTime::parse_from_rfc3339(&req.due_date)
        .map(|d| d.with_timezone(&Utc))
        .unwrap_or_else(|_| Utc::now());

    let lines: Vec<VendorBillLineCreateRequest> = req
        .lines
        .into_iter()
        .map(|l| VendorBillLineCreateRequest {
            po_line_id: l.po_line_id,
            product_id: l.product_id,
            description: l.description,
            quantity: l.quantity,
            unit_price: l.unit_price,
            tax_rate: l.tax_rate,
        })
        .collect();

    let bill = VendorBillService::create(
        &state.pool,
        req.vendor_id,
        req.vendor_invoice_number,
        req.purchase_order_id,
        bill_date,
        due_date,
        lines,
        req.notes,
    )
    .await?;

    Ok(Json(VendorBillResponse::from(bill)))
}

pub async fn submit_vendor_bill(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    VendorBillService::submit(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "submitted" })))
}

pub async fn approve_vendor_bill(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    VendorBillService::approve(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "approved" })))
}

pub async fn void_vendor_bill(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    VendorBillService::void(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "voided" })))
}

pub async fn record_vendor_bill_payment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<RecordPaymentRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    VendorBillService::record_payment(&state.pool, id, req.payment_id, req.amount).await?;
    Ok(Json(serde_json::json!({ "status": "payment_recorded" })))
}

pub async fn perform_three_way_match(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ThreeWayMatchResponse>> {
    let result = VendorBillService::perform_three_way_match(&state.pool, id).await?;
    Ok(Json(ThreeWayMatchResponse::from(result)))
}

pub async fn delete_vendor_bill(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    VendorBillService::delete(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "deleted" })))
}

pub fn routes() -> axum::Router<crate::db::AppState> {
    axum::Router::new()
        .route("/", axum::routing::get(list_vendor_bills).post(create_vendor_bill))
        .route("/vendor/:vendor_id", axum::routing::get(list_vendor_bills_by_vendor))
        .route(
            "/:id",
            axum::routing::get(get_vendor_bill).delete(delete_vendor_bill),
        )
        .route("/:id/submit", axum::routing::post(submit_vendor_bill))
        .route("/:id/approve", axum::routing::post(approve_vendor_bill))
        .route("/:id/void", axum::routing::post(void_vendor_bill))
        .route("/:id/payment", axum::routing::post(record_vendor_bill_payment))
        .route("/:id/match", axum::routing::post(perform_three_way_match))
}
