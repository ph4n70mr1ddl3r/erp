use axum::{extract::{Path, Query, State}, Json};
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{BaseEntity, Status, Pagination, Money, Currency, ContactInfo, Address};
use erp_sales::{Customer, SalesOrder, SalesOrderLine, SalesQuote, SalesQuoteLine, CustomerService, SalesOrderService, QuotationService};

#[derive(Deserialize)] pub struct CreateCustomerRequest { pub code: String, pub name: String, pub email: Option<String>, pub phone: Option<String>, pub credit_limit: Option<i64>, pub payment_terms: Option<u32> }
#[derive(Serialize)] pub struct CustomerResponse { pub id: Uuid, pub code: String, pub name: String, pub email: Option<String>, pub phone: Option<String>, pub status: String }

impl From<Customer> for CustomerResponse {
    fn from(c: Customer) -> Self { Self { id: c.base.id, code: c.code, name: c.name, email: c.contact.email, phone: c.contact.phone, status: format!("{:?}", c.status) } }
}

pub async fn list_customers(State(state): State<AppState>, Query(pagination): Query<Pagination>) -> ApiResult<Json<erp_core::Paginated<CustomerResponse>>> {
    let svc = CustomerService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(res.items.into_iter().map(CustomerResponse::from).collect(), res.total, Pagination { page: res.page, per_page: res.per_page })))
}

pub async fn get_customer(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<CustomerResponse>> {
    let svc = CustomerService::new();
    Ok(Json(CustomerResponse::from(svc.get(&state.pool, id).await?)))
}

pub async fn create_customer(State(state): State<AppState>, Json(req): Json<CreateCustomerRequest>) -> ApiResult<Json<CustomerResponse>> {
    let svc = CustomerService::new();
    let c = Customer {
        base: BaseEntity::new(), code: req.code, name: req.name,
        contact: ContactInfo { email: req.email, phone: req.phone, fax: None, website: None },
        billing_address: Address { street: String::new(), city: String::new(), state: None, postal_code: String::new(), country: String::new() },
        shipping_address: None, credit_limit: req.credit_limit.map(|v| Money::new(v, Currency::USD)),
        payment_terms: req.payment_terms.unwrap_or(30), status: Status::Active,
    };
    Ok(Json(CustomerResponse::from(svc.create(&state.pool, c).await?)))
}

#[derive(Deserialize)] pub struct CreateOrderRequest { pub customer_id: Uuid, pub lines: Vec<OrderLineRequest> }
#[derive(Deserialize)] pub struct OrderLineRequest { pub product_id: Uuid, pub description: String, pub quantity: i64, pub unit_price: i64 }

#[derive(Serialize)] pub struct OrderResponse { pub id: Uuid, pub order_number: String, pub customer_id: Uuid, pub status: String, pub total: f64, pub lines: Vec<OrderLineResponse> }
#[derive(Serialize)] pub struct OrderLineResponse { pub product_id: Uuid, pub description: String, pub quantity: i64, pub unit_price: f64, pub line_total: f64 }

impl From<SalesOrder> for OrderResponse {
    fn from(o: SalesOrder) -> Self { Self { id: o.base.id, order_number: o.order_number, customer_id: o.customer_id, status: format!("{:?}", o.status), total: o.total.to_decimal(),
        lines: o.lines.into_iter().map(|l| OrderLineResponse { product_id: l.product_id, description: l.description, quantity: l.quantity, unit_price: l.unit_price.to_decimal(), line_total: l.line_total.to_decimal() }).collect() }
    }
}

pub async fn list_orders(State(state): State<AppState>, Query(pagination): Query<Pagination>) -> ApiResult<Json<erp_core::Paginated<OrderResponse>>> {
    let svc = SalesOrderService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(res.items.into_iter().map(OrderResponse::from).collect(), res.total, Pagination { page: res.page, per_page: res.per_page })))
}

pub async fn get_order(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<OrderResponse>> {
    let svc = SalesOrderService::new();
    Ok(Json(OrderResponse::from(svc.get(&state.pool, id).await?)))
}

pub async fn create_order(State(state): State<AppState>, Json(req): Json<CreateOrderRequest>) -> ApiResult<Json<OrderResponse>> {
    let svc = SalesOrderService::new();
    let order = SalesOrder {
        base: BaseEntity::new(), order_number: String::new(), customer_id: req.customer_id, order_date: Utc::now(), required_date: None,
        lines: req.lines.into_iter().map(|l| SalesOrderLine {
            id: Uuid::nil(), product_id: l.product_id, description: l.description, quantity: l.quantity,
            unit_price: Money::new(l.unit_price, Currency::USD), discount_percent: 0.0, tax_rate: 0.0,
            line_total: Money::new(l.quantity * l.unit_price, Currency::USD),
        }).collect(),
        subtotal: Money::zero(Currency::USD), tax_amount: Money::zero(Currency::USD), total: Money::zero(Currency::USD), status: Status::Draft,
    };
    Ok(Json(OrderResponse::from(svc.create(&state.pool, order).await?)))
}

pub async fn confirm_order(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    SalesOrderService::new().confirm(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "confirmed" })))
}

pub async fn list_invoices(State(_state): State<AppState>, Query(_pagination): Query<Pagination>) -> ApiResult<Json<Vec<serde_json::Value>>> { Ok(Json(vec![])) }
pub async fn create_invoice(State(_state): State<AppState>, Json(_req): Json<serde_json::Value>) -> ApiResult<Json<serde_json::Value>> { Ok(Json(serde_json::json!({"id": Uuid::new_v4()}))) }

#[derive(Deserialize)]
pub struct CreateQuotationRequest {
    pub customer_id: Uuid,
    pub lines: Vec<QuotationLineRequest>,
}

#[derive(Deserialize)]
pub struct QuotationLineRequest {
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
    pub unit_price: i64,
    pub discount_percent: Option<f64>,
}

#[derive(Serialize)]
pub struct QuotationResponse {
    pub id: Uuid,
    pub quote_number: String,
    pub customer_id: Uuid,
    pub status: String,
    pub total: f64,
    pub lines: Vec<QuotationLineResponse>,
}

#[derive(Serialize)]
pub struct QuotationLineResponse {
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
    pub unit_price: f64,
    pub line_total: f64,
}

impl From<SalesQuote> for QuotationResponse {
    fn from(q: SalesQuote) -> Self {
        Self {
            id: q.base.id,
            quote_number: q.quote_number,
            customer_id: q.customer_id,
            status: format!("{:?}", q.status),
            total: q.total.to_decimal(),
            lines: q.lines.into_iter().map(|l| QuotationLineResponse {
                product_id: l.product_id,
                description: l.description,
                quantity: l.quantity,
                unit_price: l.unit_price.to_decimal(),
                line_total: l.line_total.to_decimal(),
            }).collect(),
        }
    }
}

pub async fn list_quotations(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<QuotationResponse>>> {
    let svc = QuotationService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(
        res.items.into_iter().map(QuotationResponse::from).collect(),
        res.total,
        Pagination { page: res.page, per_page: res.per_page },
    )))
}

pub async fn get_quotation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<QuotationResponse>> {
    let svc = QuotationService::new();
    Ok(Json(QuotationResponse::from(svc.get(&state.pool, id).await?)))
}

pub async fn create_quotation(
    State(state): State<AppState>,
    Json(req): Json<CreateQuotationRequest>,
) -> ApiResult<Json<QuotationResponse>> {
    let svc = QuotationService::new();
    let quote = SalesQuote {
        base: BaseEntity::new(),
        quote_number: String::new(),
        customer_id: req.customer_id,
        quote_date: Utc::now(),
        valid_until: Utc::now() + chrono::Duration::days(30),
        lines: req.lines.into_iter().map(|l| {
            let line_total = l.quantity * l.unit_price;
            SalesQuoteLine {
                id: Uuid::nil(),
                product_id: l.product_id,
                description: l.description,
                quantity: l.quantity,
                unit_price: Money::new(l.unit_price, Currency::USD),
                discount_percent: l.discount_percent.unwrap_or(0.0),
                line_total: Money::new(line_total, Currency::USD),
            }
        }).collect(),
        subtotal: Money::zero(Currency::USD),
        tax_amount: Money::zero(Currency::USD),
        total: Money::zero(Currency::USD),
        status: Status::Draft,
    };
    Ok(Json(QuotationResponse::from(svc.create(&state.pool, quote).await?)))
}

pub async fn send_quotation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    QuotationService::new().send(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "sent" })))
}

pub async fn accept_quotation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    QuotationService::new().accept(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "accepted" })))
}

pub async fn reject_quotation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    QuotationService::new().reject(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "rejected" })))
}

pub async fn convert_quotation(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<OrderResponse>> {
    let svc = QuotationService::new();
    let order = svc.convert_to_order(&state.pool, id).await?;
    Ok(Json(OrderResponse::from(order)))
}
