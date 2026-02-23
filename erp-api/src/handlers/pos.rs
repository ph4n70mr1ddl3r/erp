use axum::{extract::{Path, Query, State}, Json, routing::{get, post}};
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{Pagination, BaseEntity, Money, Currency};
use erp_pos::{
    POSStore, POSStatus, POSTerminal, Register, POSTransaction, POSTransactionLine, POSTransactionPayment,
    TransactionType, PaymentMethod, GiftCard, GiftCardStatus,
    POSStoreService, POSTransactionService, GiftCardService,
};

#[derive(Serialize)]
pub struct POSStoreResponse {
    pub id: Uuid,
    pub store_code: String,
    pub name: String,
    pub city: String,
    pub status: String,
}

impl From<POSStore> for POSStoreResponse {
    fn from(s: POSStore) -> Self {
        Self {
            id: s.base.id,
            store_code: s.store_code,
            name: s.name,
            city: s.city,
            status: format!("{:?}", s.status),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateStoreRequest {
    pub store_code: String,
    pub name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub manager_id: Option<Uuid>,
    pub warehouse_id: Option<Uuid>,
}

pub async fn list_stores(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<POSStoreResponse>>> {
    let svc = POSStoreService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(
        res.items.into_iter().map(POSStoreResponse::from).collect(),
        res.total,
        Pagination { page: res.page, per_page: res.per_page },
    )))
}

pub async fn get_store(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<POSStoreResponse>> {
    let svc = POSStoreService::new();
    Ok(Json(POSStoreResponse::from(svc.get(&state.pool, id).await?)))
}

pub async fn create_store(
    State(state): State<AppState>,
    Json(req): Json<CreateStoreRequest>,
) -> ApiResult<Json<POSStoreResponse>> {
    let svc = POSStoreService::new();
    let store = POSStore {
        base: BaseEntity::new(),
        store_code: req.store_code,
        name: req.name,
        address: req.address,
        city: req.city,
        state: req.state,
        postal_code: req.postal_code,
        country: req.country,
        phone: req.phone,
        email: req.email,
        manager_id: req.manager_id,
        warehouse_id: req.warehouse_id,
        status: POSStatus::Active,
        opening_time: "09:00".to_string(),
        closing_time: "21:00".to_string(),
        timezone: "UTC".to_string(),
    };
    Ok(Json(POSStoreResponse::from(svc.create(&state.pool, store).await?)))
}

#[derive(Serialize)]
pub struct POSTransactionResponse {
    pub id: Uuid,
    pub transaction_number: String,
    pub store_id: Uuid,
    pub transaction_type: String,
    pub total: f64,
    pub status: String,
}

impl From<POSTransaction> for POSTransactionResponse {
    fn from(t: POSTransaction) -> Self {
        Self {
            id: t.base.id,
            transaction_number: t.transaction_number,
            store_id: t.store_id,
            transaction_type: format!("{:?}", t.transaction_type),
            total: t.total.to_decimal(),
            status: format!("{:?}", t.status),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateTransactionRequest {
    pub store_id: Uuid,
    pub terminal_id: Uuid,
    pub register_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub sales_rep_id: Option<Uuid>,
    pub lines: Vec<TransactionLineRequest>,
    pub payments: Vec<TransactionPaymentRequest>,
}

#[derive(Deserialize)]
pub struct TransactionLineRequest {
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
    pub unit_price: i64,
    pub discount_percent: Option<f64>,
    pub tax_amount: Option<i64>,
}

#[derive(Deserialize)]
pub struct TransactionPaymentRequest {
    pub payment_method: String,
    pub amount: i64,
    pub reference: Option<String>,
    pub card_last_four: Option<String>,
    pub card_type: Option<String>,
    pub authorization_code: Option<String>,
}

pub async fn list_transactions(
    State(state): State<AppState>,
    Query(store_id): Query<Option<Uuid>>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<POSTransactionResponse>>> {
    let svc = POSTransactionService::new();
    let res = svc.list(&state.pool, store_id, pagination).await?;
    Ok(Json(erp_core::Paginated::new(
        res.items.into_iter().map(POSTransactionResponse::from).collect(),
        res.total,
        Pagination { page: res.page, per_page: res.per_page },
    )))
}

pub async fn create_transaction(
    State(state): State<AppState>,
    Json(req): Json<CreateTransactionRequest>,
) -> ApiResult<Json<POSTransactionResponse>> {
    let svc = POSTransactionService::new();
    
    let lines: Vec<POSTransactionLine> = req.lines.into_iter().enumerate().map(|(i, l)| {
        let line_total = l.quantity * l.unit_price;
        let discount = (line_total as f64 * l.discount_percent.unwrap_or(0.0) / 100.0) as i64;
        POSTransactionLine {
            id: Uuid::nil(),
            transaction_id: Uuid::nil(),
            line_number: i as i32 + 1,
            product_id: l.product_id,
            description: l.description,
            quantity: l.quantity,
            unit_price: Money::new(l.unit_price, Currency::USD),
            discount_percent: l.discount_percent.unwrap_or(0.0),
            discount_amount: Money::new(discount, Currency::USD),
            tax_rate_id: None,
            tax_amount: Money::new(l.tax_amount.unwrap_or(0), Currency::USD),
            line_total: Money::new(line_total - discount, Currency::USD),
            lot_number: None,
            serial_number: None,
        }
    }).collect();
    
    let payments: Vec<POSTransactionPayment> = req.payments.into_iter().map(|p| {
        POSTransactionPayment {
            id: Uuid::nil(),
            transaction_id: Uuid::nil(),
            payment_method: match p.payment_method.as_str() {
                "CreditCard" => PaymentMethod::CreditCard,
                "DebitCard" => PaymentMethod::DebitCard,
                "GiftCard" => PaymentMethod::GiftCard,
                "Check" => PaymentMethod::Check,
                "MobilePayment" => PaymentMethod::MobilePayment,
                "StoreCredit" => PaymentMethod::StoreCredit,
                "Mixed" => PaymentMethod::Mixed,
                _ => PaymentMethod::Cash,
            },
            amount: Money::new(p.amount, Currency::USD),
            reference: p.reference,
            card_last_four: p.card_last_four,
            card_type: p.card_type,
            authorization_code: p.authorization_code,
            gift_card_id: None,
        }
    }).collect();
    
    let transaction = POSTransaction {
        base: BaseEntity::new(),
        transaction_number: String::new(),
        store_id: req.store_id,
        terminal_id: req.terminal_id,
        register_id: req.register_id,
        transaction_type: TransactionType::Sale,
        customer_id: req.customer_id,
        sales_rep_id: req.sales_rep_id,
        lines,
        payments,
        subtotal: Money::zero(Currency::USD),
        discount_amount: Money::zero(Currency::USD),
        tax_amount: Money::zero(Currency::USD),
        total: Money::zero(Currency::USD),
        change_amount: Money::zero(Currency::USD),
        status: erp_core::Status::Active,
        original_transaction_id: None,
        notes: None,
        completed_at: None,
    };
    
    Ok(Json(POSTransactionResponse::from(svc.create(&state.pool, transaction).await?)))
}

pub async fn void_transaction(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    POSTransactionService::new().void(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "voided" })))
}

#[derive(Serialize)]
pub struct GiftCardResponse {
    pub id: Uuid,
    pub card_number: String,
    pub current_balance: f64,
    pub status: String,
}

impl From<GiftCard> for GiftCardResponse {
    fn from(g: GiftCard) -> Self {
        Self {
            id: g.base.id,
            card_number: g.card_number,
            current_balance: g.current_balance.to_decimal(),
            status: format!("{:?}", g.status),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateGiftCardRequest {
    pub initial_amount: i64,
    pub store_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub expires_at: Option<String>,
}

#[derive(Deserialize)]
pub struct RedeemGiftCardRequest {
    pub amount: i64,
}

pub async fn create_gift_card(
    State(state): State<AppState>,
    Json(req): Json<CreateGiftCardRequest>,
) -> ApiResult<Json<GiftCardResponse>> {
    let svc = GiftCardService::new();
    let card = GiftCard {
        base: BaseEntity::new(),
        card_number: String::new(),
        initial_amount: Money::new(req.initial_amount, Currency::USD),
        current_balance: Money::new(req.initial_amount, Currency::USD),
        sold_at: Utc::now(),
        sold_at_store_id: req.store_id,
        customer_id: req.customer_id,
        expires_at: req.expires_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
            .map(|d| d.with_timezone(&chrono::Utc)),
        status: GiftCardStatus::Active,
    };
    Ok(Json(GiftCardResponse::from(svc.create(&state.pool, card).await?)))
}

pub async fn get_gift_card(
    State(state): State<AppState>,
    Path(number): Path<String>,
) -> ApiResult<Json<GiftCardResponse>> {
    let svc = GiftCardService::new();
    Ok(Json(GiftCardResponse::from(svc.get_by_number(&state.pool, &number).await?)))
}

pub async fn redeem_gift_card(
    State(state): State<AppState>,
    Path(number): Path<String>,
    Json(req): Json<RedeemGiftCardRequest>,
) -> ApiResult<Json<GiftCardResponse>> {
    let svc = GiftCardService::new();
    Ok(Json(GiftCardResponse::from(svc.redeem(&state.pool, &number, req.amount).await?)))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/stores", get(list_stores).post(create_store))
        .route("/stores/:id", get(get_store))
        .route("/transactions", get(list_transactions).post(create_transaction))
        .route("/transactions/:id/void", post(void_transaction))
        .route("/gift-cards", post(create_gift_card))
        .route("/gift-cards/:number", get(get_gift_card))
        .route("/gift-cards/:number/redeem", post(redeem_gift_card))
}
