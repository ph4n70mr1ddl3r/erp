use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::AppState;
use erp_giftcards::{
    AdjustGiftCardRequest, CreateGiftCardRequest, GiftCardResponse, GiftCardTransactionResponse,
    GiftCardType, RedeemGiftCardRequest, ReloadGiftCardRequest,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(create_gift_card).get(list_gift_cards))
        .route("/:id", get(get_gift_card))
        .route("/:id/redeem", post(redeem_gift_card))
        .route("/:id/reload", post(reload_gift_card))
        .route("/:id/adjust", post(adjust_gift_card))
        .route("/:id/cancel", post(cancel_gift_card))
        .route("/:id/transactions", get(list_transactions))
        .route("/check-balance", post(check_balance))
}

#[derive(Deserialize)]
struct CreateRequest {
    gift_card_type: GiftCardType,
    initial_balance: i64,
    currency: Option<String>,
    customer_id: Option<Uuid>,
    order_id: Option<Uuid>,
    purchased_by: Option<Uuid>,
    recipient_email: Option<String>,
    recipient_name: Option<String>,
    message: Option<String>,
    expiry_date: Option<String>,
}

async fn create_gift_card(
    State(state): State<AppState>,
    Json(req): Json<CreateRequest>,
) -> Json<GiftCardResponse> {
    let service = erp_giftcards::GiftCardService::new(state.pool.clone());
    let expiry = req
        .expiry_date
        .and_then(|d| chrono::NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    let create_req = CreateGiftCardRequest {
        gift_card_type: req.gift_card_type,
        initial_balance: req.initial_balance,
        currency: req.currency,
        customer_id: req.customer_id,
        order_id: req.order_id,
        purchased_by: req.purchased_by,
        recipient_email: req.recipient_email,
        recipient_name: req.recipient_name,
        message: req.message,
        expiry_date: expiry,
    };
    let card = service.create(create_req).await.unwrap();
    Json(card.into())
}

async fn list_gift_cards(State(state): State<AppState>) -> Json<Vec<GiftCardResponse>> {
    let service = erp_giftcards::GiftCardService::new(state.pool.clone());
    let cards = service.list(1, 50).await.unwrap();
    Json(cards.into_iter().map(|c| c.into()).collect())
}

async fn get_gift_card(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<GiftCardResponse> {
    let service = erp_giftcards::GiftCardService::new(state.pool.clone());
    let card = service
        .get(id)
        .await
        .unwrap()
        .unwrap();
    Json(card.into())
}

#[derive(Deserialize)]
struct RedeemRequest {
    amount: i64,
    order_id: Option<Uuid>,
    reference: Option<String>,
}

async fn redeem_gift_card(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<RedeemRequest>,
) -> Json<GiftCardTransactionResponse> {
    let service = erp_giftcards::GiftCardService::new(state.pool.clone());
    let tx = service
        .redeem(
            id,
            RedeemGiftCardRequest {
                amount: req.amount,
                order_id: req.order_id,
                reference: req.reference,
            },
            None,
        )
        .await
        .unwrap();
    Json(tx.into())
}

#[derive(Deserialize)]
struct ReloadRequest {
    amount: i64,
    order_id: Option<Uuid>,
    reference: Option<String>,
}

async fn reload_gift_card(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ReloadRequest>,
) -> Json<GiftCardTransactionResponse> {
    let service = erp_giftcards::GiftCardService::new(state.pool.clone());
    let tx = service
        .reload(
            id,
            ReloadGiftCardRequest {
                amount: req.amount,
                order_id: req.order_id,
                reference: req.reference,
            },
            None,
        )
        .await
        .unwrap();
    Json(tx.into())
}

#[derive(Deserialize)]
struct AdjustRequest {
    amount: i64,
    reason: String,
}

async fn adjust_gift_card(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<AdjustRequest>,
) -> Json<GiftCardTransactionResponse> {
    let service = erp_giftcards::GiftCardService::new(state.pool.clone());
    let tx = service
        .adjust(
            id,
            AdjustGiftCardRequest {
                amount: req.amount,
                reason: req.reason,
            },
            None,
        )
        .await
        .unwrap();
    Json(tx.into())
}

#[derive(Deserialize)]
struct CancelRequest {
    reason: String,
}

#[derive(Serialize)]
struct CancelResponse {
    id: Uuid,
    card_number: String,
    status: String,
}

async fn cancel_gift_card(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CancelRequest>,
) -> Json<CancelResponse> {
    let service = erp_giftcards::GiftCardService::new(state.pool.clone());
    let card = service.cancel(id, req.reason, None).await.unwrap();
    Json(CancelResponse {
        id: card.base.id,
        card_number: card.card_number,
        status: "Cancelled".to_string(),
    })
}

async fn list_transactions(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<Vec<GiftCardTransactionResponse>> {
    let service = erp_giftcards::GiftCardService::new(state.pool.clone());
    let transactions = service.list_transactions(id).await.unwrap();
    Json(transactions.into_iter().map(|t| t.into()).collect())
}

#[derive(Deserialize)]
struct CheckBalanceRequest {
    card_number: String,
    pin: Option<String>,
}

async fn check_balance(
    State(state): State<AppState>,
    Json(req): Json<CheckBalanceRequest>,
) -> Json<GiftCardResponse> {
    let service = erp_giftcards::GiftCardService::new(state.pool.clone());
    let card = service
        .check_balance(&req.card_number, req.pin.as_deref())
        .await
        .unwrap();
    Json(card.into())
}
