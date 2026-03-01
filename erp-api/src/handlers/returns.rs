use axum::{extract::{Path, Query, State}, Json, routing::{get, post}};
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{Pagination, BaseEntity, Money, Currency};
use erp_returns::{
    ReturnOrder, ReturnLine, ReturnType, ReturnReason, ReturnStatus, ReturnDisposition, ItemCondition,
    CreditMemo, CreditMemoLine, CreditMemoStatus,
    Refund, RefundMethod,
    ReturnService, CreditMemoService, RefundService,
};

#[derive(Serialize)]
pub struct ReturnOrderResponse {
    pub id: Uuid,
    pub return_number: String,
    pub return_type: String,
    pub customer_id: Option<Uuid>,
    pub status: String,
    pub total_credit: f64,
}

impl From<ReturnOrder> for ReturnOrderResponse {
    fn from(r: ReturnOrder) -> Self {
        Self {
            id: r.base.id,
            return_number: r.return_number,
            return_type: format!("{:?}", r.return_type),
            customer_id: r.customer_id,
            status: format!("{:?}", r.status),
            total_credit: r.total_credit.to_decimal(),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateReturnRequest {
    pub return_type: Option<String>,
    pub customer_id: Option<Uuid>,
    pub vendor_id: Option<Uuid>,
    pub original_order_id: Option<Uuid>,
    pub reason: String,
    pub notes: Option<String>,
    pub warehouse_id: Option<Uuid>,
    pub lines: Vec<ReturnLineRequest>,
}

#[derive(Deserialize)]
pub struct ReturnLineRequest {
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
    pub unit_price: i64,
    pub reason: Option<String>,
}

pub async fn list_returns(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<ReturnOrderResponse>>> {
    let svc = ReturnService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(
        res.items.into_iter().map(ReturnOrderResponse::from).collect(),
        res.total,
        Pagination { page: res.page, per_page: res.per_page },
    )))
}

pub async fn get_return(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ReturnOrderResponse>> {
    let svc = ReturnService::new();
    Ok(Json(ReturnOrderResponse::from(svc.get(&state.pool, id).await?)))
}

pub async fn create_return(
    State(state): State<AppState>,
    Json(req): Json<CreateReturnRequest>,
) -> ApiResult<Json<ReturnOrderResponse>> {
    let svc = ReturnService::new();
    
    let return_type = match req.return_type.as_deref() {
        Some("VendorReturn") => ReturnType::VendorReturn,
        Some("InternalReturn") => ReturnType::InternalReturn,
        _ => ReturnType::CustomerReturn,
    };
    
    let reason = match req.reason.as_str() {
        "WrongItem" => ReturnReason::WrongItem,
        "NotAsDescribed" => ReturnReason::NotAsDescribed,
        "Damaged" => ReturnReason::Damaged,
        "ChangedMind" => ReturnReason::ChangedMind,
        "Warranty" => ReturnReason::Warranty,
        "Recall" => ReturnReason::Recall,
        _ => ReturnReason::Other,
    };
    
    let lines: Vec<ReturnLine> = req.lines.into_iter().map(|l| {
        let credit_amount = l.quantity * l.unit_price;
        ReturnLine {
            id: Uuid::nil(),
            return_order_id: Uuid::nil(),
            product_id: l.product_id,
            description: l.description,
            quantity_requested: l.quantity,
            quantity_received: 0,
            quantity_approved: 0,
            unit_price: Money::new(l.unit_price, Currency::USD),
            reason: reason.clone(),
            disposition: ReturnDisposition::Restock,
            condition: ItemCondition::New,
            inspection_notes: None,
            credit_amount: Money::new(credit_amount, Currency::USD),
        }
    }).collect();
    
    let order = ReturnOrder {
        base: BaseEntity::new(),
        return_number: String::new(),
        return_type,
        customer_id: req.customer_id,
        vendor_id: req.vendor_id,
        original_order_id: req.original_order_id,
        original_invoice_id: None,
        request_date: Utc::now(),
        received_date: None,
        processed_date: None,
        reason,
        notes: req.notes,
        lines,
        status: ReturnStatus::Draft,
        total_credit: Money::zero(Currency::USD),
        warehouse_id: req.warehouse_id,
    };
    
    Ok(Json(ReturnOrderResponse::from(svc.create(&state.pool, order).await?)))
}

pub async fn approve_return(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    ReturnService::new().approve(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "approved" })))
}

pub async fn receive_return(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    ReturnService::new().receive(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "received" })))
}

pub async fn process_return(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    ReturnService::new().process(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "processed" })))
}

#[derive(Serialize)]
pub struct CreditMemoResponse {
    pub id: Uuid,
    pub memo_number: String,
    pub customer_id: Uuid,
    pub status: String,
    pub total: f64,
}

impl From<CreditMemo> for CreditMemoResponse {
    fn from(m: CreditMemo) -> Self {
        Self {
            id: m.base.id,
            memo_number: m.memo_number,
            customer_id: m.customer_id,
            status: format!("{:?}", m.status),
            total: m.total.to_decimal(),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateCreditMemoRequest {
    pub customer_id: Uuid,
    pub return_order_id: Option<Uuid>,
    pub invoice_id: Option<Uuid>,
    pub reason: Option<String>,
    pub lines: Vec<CreditMemoLineRequest>,
}

#[derive(Deserialize)]
pub struct CreditMemoLineRequest {
    pub product_id: Option<Uuid>,
    pub description: String,
    pub quantity: i64,
    pub unit_price: i64,
}

pub async fn list_credit_memos(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<CreditMemoResponse>>> {
    let svc = CreditMemoService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(
        res.items.into_iter().map(CreditMemoResponse::from).collect(),
        res.total,
        Pagination { page: res.page, per_page: res.per_page },
    )))
}

pub async fn create_credit_memo(
    State(state): State<AppState>,
    Json(req): Json<CreateCreditMemoRequest>,
) -> ApiResult<Json<CreditMemoResponse>> {
    let svc = CreditMemoService::new();
    
    let lines: Vec<CreditMemoLine> = req.lines.into_iter().map(|l| {
        CreditMemoLine {
            id: Uuid::nil(),
            credit_memo_id: Uuid::nil(),
            product_id: l.product_id,
            description: l.description,
            quantity: l.quantity,
            unit_price: Money::new(l.unit_price, Currency::USD),
            line_total: Money::new(l.quantity * l.unit_price, Currency::USD),
        }
    }).collect();
    
    let memo = CreditMemo {
        base: BaseEntity::new(),
        memo_number: String::new(),
        customer_id: req.customer_id,
        return_order_id: req.return_order_id,
        invoice_id: req.invoice_id,
        memo_date: Utc::now(),
        lines,
        subtotal: Money::zero(Currency::USD),
        tax_amount: Money::zero(Currency::USD),
        total: Money::zero(Currency::USD),
        status: CreditMemoStatus::Draft,
        applied_amount: Money::zero(Currency::USD),
        reason: req.reason,
    };
    
    Ok(Json(CreditMemoResponse::from(svc.create(&state.pool, memo).await?)))
}

pub async fn issue_credit_memo(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    CreditMemoService::new().issue(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "issued" })))
}

#[derive(Serialize)]
pub struct RefundResponse {
    pub id: Uuid,
    pub refund_number: String,
    pub customer_id: Uuid,
    pub amount: f64,
    pub method: String,
    pub status: String,
}

impl From<Refund> for RefundResponse {
    fn from(r: Refund) -> Self {
        Self {
            id: r.id,
            refund_number: r.refund_number,
            customer_id: r.customer_id,
            amount: r.amount.to_decimal(),
            method: format!("{:?}", r.method),
            status: format!("{:?}", r.status),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateRefundRequest {
    pub customer_id: Uuid,
    pub credit_memo_id: Option<Uuid>,
    pub return_order_id: Option<Uuid>,
    pub amount: i64,
    pub method: Option<String>,
}

pub async fn list_refunds(
    State(state): State<AppState>,
    Query(customer_id): Query<Option<Uuid>>,
) -> ApiResult<Json<Vec<RefundResponse>>> {
    let refunds = RefundService::list(&state.pool, customer_id).await?;
    Ok(Json(refunds.into_iter().map(RefundResponse::from).collect()))
}

pub async fn create_refund(
    State(state): State<AppState>,
    Json(req): Json<CreateRefundRequest>,
) -> ApiResult<Json<RefundResponse>> {
    let method = match req.method.as_deref() {
        Some("Check") => RefundMethod::Check,
        Some("BankTransfer") => RefundMethod::BankTransfer,
        Some("StoreCredit") => RefundMethod::StoreCredit,
        _ => RefundMethod::OriginalPayment,
    };
    
    let refund = RefundService::create(
        &state.pool,
        req.customer_id,
        req.credit_memo_id,
        req.return_order_id,
        req.amount,
        method,
    ).await?;
    
    Ok(Json(RefundResponse::from(refund)))
}

pub async fn process_refund(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<RefundResponse>> {
    let refund = RefundService::process(&state.pool, id, None).await?;
    Ok(Json(RefundResponse::from(refund)))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", get(list_returns).post(create_return))
        .route("/:id", get(get_return))
        .route("/:id/approve", post(approve_return))
        .route("/:id/receive", post(receive_return))
        .route("/:id/process", post(process_return))
        .route("/credit-memos", get(list_credit_memos).post(create_credit_memo))
        .route("/credit-memos/:id/issue", post(issue_credit_memo))
        .route("/refunds", get(list_refunds).post(create_refund))
        .route("/refunds/:id/process", post(process_refund))
}
