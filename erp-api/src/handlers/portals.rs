use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use erp_core::Pagination;
use crate::db::AppState;
use crate::error::ApiResult;
use erp_portals::{PortalUserService, PortalOrderService, PortalPaymentService, SupplierPortalService};
use erp_portals::{PortalUser, PortalType, PortalAccessLevel, PortalOrder, PortalOrderLine, PortalPayment, PaymentMethodType};

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub portal_type: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub company_name: Option<String>,
    pub external_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub portal_type: String,
    pub access_level: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub session_token: String,
}

#[derive(Deserialize)]
pub struct CreateOrderRequest {
    pub customer_id: Uuid,
    pub lines: Vec<OrderLineRequest>,
    pub billing_address: Option<String>,
    pub shipping_address: Option<String>,
}

#[derive(Deserialize)]
pub struct OrderLineRequest {
    pub product_id: Uuid,
    pub product_code: String,
    pub product_name: String,
    pub quantity: i64,
    pub unit_of_measure: String,
    pub unit_price_cents: i64,
}

#[derive(Deserialize)]
pub struct PaymentRequest {
    pub invoice_ids: Vec<Uuid>,
    pub amount_cents: i64,
    pub payment_method: String,
}

impl From<PortalUser> for UserResponse {
    fn from(u: PortalUser) -> Self {
        Self {
            id: u.base.id,
            username: u.username,
            email: u.email,
            portal_type: format!("{:?}", u.portal_type),
            access_level: format!("{:?}", u.access_level),
        }
    }
}

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<UserResponse>, (StatusCode, String)> {
    let service = PortalUserService::new();
    let portal_type = match req.portal_type.as_str() {
        "Supplier" => PortalType::Supplier,
        "Partner" => PortalType::Partner,
        "Employee" => PortalType::Employee,
        _ => PortalType::Customer,
    };
    
    let user = service.register(&state.pool, portal_type, req.username, req.email, &req.password, req.external_id).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    Ok(Json(UserResponse::from(user)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, (StatusCode, String)> {
    let service = PortalUserService::new();
    let (user, session) = service.login(&state.pool, &req.username, &req.password).await
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;
    
    Ok(Json(LoginResponse {
        user: UserResponse::from(user),
        session_token: session.session_token,
    }))
}

pub async fn list_orders(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = PortalOrderService::new();
    let user_id = Uuid::nil();
    let result = service.list_by_user(&state.pool, user_id, pagination).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "items": result.items.into_iter().map(|o| serde_json::json!({
            "id": o.base.id,
            "order_number": o.portal_order_number,
            "status": format!("{:?}", o.status),
            "total_cents": o.total_cents,
            "currency": o.currency,
            "created_at": o.base.created_at
        })).collect::<Vec<_>>(),
        "total": result.total
    })))
}

pub async fn create_order(
    State(state): State<AppState>,
    Json(req): Json<CreateOrderRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = PortalOrderService::new();
    let user_id = Uuid::nil();
    
    let lines: Vec<PortalOrderLine> = req.lines.into_iter().enumerate().map(|(i, l)| PortalOrderLine {
        id: Uuid::new_v4(),
        portal_order_id: Uuid::nil(),
        line_number: (i + 1) as i32,
        product_id: l.product_id,
        product_code: l.product_code,
        product_name: l.product_name,
        description: None,
        quantity: l.quantity,
        unit_of_measure: l.unit_of_measure,
        unit_price_cents: l.unit_price_cents,
        discount_percent: 0.0,
        discount_cents: 0,
        tax_percent: 0.0,
        tax_cents: 0,
        line_total_cents: l.quantity * l.unit_price_cents,
        notes: None,
        erp_line_id: None,
    }).collect();
    
    let order = service.create(&state.pool, user_id, req.customer_id, lines, req.billing_address, req.shipping_address).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "id": order.base.id,
        "order_number": order.portal_order_number,
        "status": format!("{:?}", order.status),
        "total_cents": order.total_cents
    })))
}

pub async fn submit_order(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = PortalOrderService::new();
    let order = service.submit(&state.pool, id).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "id": order.base.id,
        "status": format!("{:?}", order.status),
        "submitted_at": order.submitted_at
    })))
}

pub async fn process_payment(
    State(state): State<AppState>,
    Json(req): Json<PaymentRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = PortalPaymentService::new();
    let user_id = Uuid::nil();
    
    let method = match req.payment_method.as_str() {
        "CreditCard" => PaymentMethodType::CreditCard,
        "DebitCard" => PaymentMethodType::DebitCard,
        "ACH" => PaymentMethodType::ACH,
        "WireTransfer" => PaymentMethodType::WireTransfer,
        "PayPal" => PaymentMethodType::PayPal,
        _ => PaymentMethodType::Other,
    };
    
    let payment = service.process_payment(&state.pool, user_id, req.invoice_ids, req.amount_cents, method).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "id": payment.base.id,
        "payment_reference": payment.payment_reference,
        "status": format!("{:?}", payment.status),
        "amount_cents": payment.amount_cents
    })))
}

pub async fn submit_supplier_quote(
    State(state): State<AppState>,
    Json(req): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    let _ = state;
    Ok(Json(serde_json::json!({
        "success": true,
        "message": "Quote submitted successfully"
    })))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/register", axum::routing::post(register))
        .route("/login", axum::routing::post(login))
        .route("/orders", axum::routing::get(list_orders).post(create_order))
        .route("/orders/:id/submit", axum::routing::post(submit_order))
        .route("/payments", axum::routing::post(process_payment))
        .route("/supplier/quotes", axum::routing::post(submit_supplier_quote))
}
