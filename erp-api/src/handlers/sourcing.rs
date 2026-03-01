use axum::{
    extract::{Path, State},
    Json,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::BaseEntity;
use erp_sourcing::{
    SourcingService, SourcingEvent, SourcingItem, Bid,
    SourcingEventType, SourcingStatus, BidStatus,
};

#[derive(Deserialize)]
pub struct CreateEventRequest {
    pub title: String,
    pub description: Option<String>,
    pub event_type: String,
    pub start_date: String,
    pub end_date: String,
    pub currency: String,
    pub estimated_value: i64,
}

#[derive(Serialize)]
pub struct EventResponse {
    pub id: Uuid,
    pub event_number: String,
    pub title: String,
    pub status: String,
    pub event_type: String,
}

pub async fn list_events(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<EventResponse>>> {
    let service = SourcingService::new();
    let events = service.list_events(&state.pool).await?;
    Ok(Json(events.into_iter().map(|e| EventResponse {
        id: e.base.id,
        event_number: e.event_number,
        title: e.title,
        status: format!("{:?}", e.status),
        event_type: format!("{:?}", e.event_type),
    }).collect()))
}

pub async fn create_event(
    State(state): State<AppState>,
    Json(req): Json<CreateEventRequest>,
) -> ApiResult<Json<EventResponse>> {
    let service = SourcingService::new();
    let event_type = match req.event_type.as_str() {
        "RFI" => SourcingEventType::RFI,
        "RFP" => SourcingEventType::RFP,
        "Auction" => SourcingEventType::Auction,
        "Tender" => SourcingEventType::Tender,
        _ => SourcingEventType::RFQ,
    };
    let start_date = chrono::DateTime::parse_from_rfc3339(&req.start_date)
        .map_err(|_| erp_core::Error::validation("Invalid start date"))?
        .with_timezone(&chrono::Utc);
    let end_date = chrono::DateTime::parse_from_rfc3339(&req.end_date)
        .map_err(|_| erp_core::Error::validation("Invalid end date"))?
        .with_timezone(&chrono::Utc);
    
    let event = SourcingEvent {
        base: BaseEntity::new(),
        event_number: String::new(),
        title: req.title,
        description: req.description,
        event_type,
        status: SourcingStatus::Draft,
        auction_type: None,
        start_date,
        end_date,
        currency: req.currency,
        estimated_value: req.estimated_value,
        budget: None,
        requirements: None,
        evaluation_criteria: None,
        terms_conditions: None,
        buyer_id: None,
        category_id: None,
        is_public: true,
        allow_reverse_auction: false,
        min_bid_decrement: None,
        auto_extend: false,
        extension_minutes: None,
        created_by: None,
    };
    let created = service.create_event(&state.pool, event).await?;
    Ok(Json(EventResponse {
        id: created.base.id,
        event_number: created.event_number,
        title: created.title,
        status: format!("{:?}", created.status),
        event_type: format!("{:?}", created.event_type),
    }))
}

pub async fn get_event(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<EventResponse>> {
    let service = SourcingService::new();
    let event = service.get_event(&state.pool, id).await?;
    Ok(Json(EventResponse {
        id: event.base.id,
        event_number: event.event_number,
        title: event.title,
        status: format!("{:?}", event.status),
        event_type: format!("{:?}", event.event_type),
    }))
}

pub async fn publish_event(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = SourcingService::new();
    service.publish_event(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "published" })))
}

#[derive(Deserialize)]
pub struct AddItemRequest {
    pub event_id: Uuid,
    pub name: String,
    pub quantity: i32,
    pub unit_of_measure: String,
    pub target_price: Option<i64>,
}

pub async fn add_item(
    State(state): State<AppState>,
    Json(req): Json<AddItemRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = SourcingService::new();
    let item = SourcingItem {
        base: BaseEntity::new(),
        event_id: req.event_id,
        product_id: None,
        sku: None,
        name: req.name,
        description: None,
        quantity: req.quantity,
        unit_of_measure: req.unit_of_measure,
        target_price: req.target_price,
        max_price: None,
        specifications: None,
        delivery_date: None,
        delivery_location: None,
        sort_order: 0,
    };
    service.add_item(&state.pool, item).await?;
    Ok(Json(serde_json::json!({ "status": "added" })))
}

#[derive(Deserialize)]
pub struct SubmitBidRequest {
    pub event_id: Uuid,
    pub vendor_id: Uuid,
    pub total_amount: i64,
    pub currency: String,
}

#[derive(Serialize)]
pub struct BidResponse {
    pub id: Uuid,
    pub bid_number: String,
    pub status: String,
    pub total_amount: f64,
}

pub async fn submit_bid(
    State(state): State<AppState>,
    Json(req): Json<SubmitBidRequest>,
) -> ApiResult<Json<BidResponse>> {
    let service = SourcingService::new();
    let bid = Bid {
        base: BaseEntity::new(),
        event_id: req.event_id,
        vendor_id: req.vendor_id,
        bid_number: format!("BID-{}", chrono::Utc::now().format("%Y%m%d%H%M%S")),
        status: BidStatus::Submitted,
        submitted_at: Some(chrono::Utc::now()),
        valid_until: None,
        total_amount: req.total_amount,
        currency: req.currency,
        terms: None,
        notes: None,
        rank: None,
        score: None,
        is_winner: false,
    };
    let created = service.submit_bid(&state.pool, bid).await?;
    Ok(Json(BidResponse {
        id: created.base.id,
        bid_number: created.bid_number,
        status: format!("{:?}", created.status),
        total_amount: created.total_amount as f64 / 100.0,
    }))
}

pub async fn list_bids(
    State(state): State<AppState>,
    Path(event_id): Path<Uuid>,
) -> ApiResult<Json<Vec<BidResponse>>> {
    let service = SourcingService::new();
    let bids = service.list_bids(&state.pool, event_id).await?;
    Ok(Json(bids.into_iter().map(|b| BidResponse {
        id: b.base.id,
        bid_number: b.bid_number,
        status: format!("{:?}", b.status),
        total_amount: b.total_amount as f64 / 100.0,
    }).collect()))
}

pub async fn accept_bid(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = SourcingService::new();
    service.accept_bid(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "accepted" })))
}

#[derive(Deserialize)]
pub struct AwardBidRequest {
    pub event_id: Uuid,
    pub bid_id: Uuid,
    pub vendor_id: Uuid,
    pub total_value: i64,
    pub currency: String,
}

pub async fn award_bid(
    State(state): State<AppState>,
    Json(req): Json<AwardBidRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = SourcingService::new();
    service.award_bid(&state.pool, req.event_id, req.bid_id, req.vendor_id, req.total_value, req.currency).await?;
    Ok(Json(serde_json::json!({ "status": "awarded" })))
}

#[derive(Deserialize)]
pub struct InviteSupplierRequest {
    pub event_id: Uuid,
    pub vendor_id: Uuid,
}

pub async fn invite_supplier(
    State(state): State<AppState>,
    Json(req): Json<InviteSupplierRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = SourcingService::new();
    service.invite_supplier(&state.pool, req.event_id, req.vendor_id).await?;
    Ok(Json(serde_json::json!({ "status": "invited" })))
}

pub fn routes() -> axum::Router<crate::db::AppState> {
    axum::Router::new()
        .route("/events", axum::routing::get(list_events).post(create_event))
        .route("/events/:id", axum::routing::get(get_event))
        .route("/events/:id/publish", axum::routing::post(publish_event))
        .route("/items", axum::routing::post(add_item))
        .route("/bids", axum::routing::post(submit_bid))
        .route("/bids/:event_id", axum::routing::get(list_bids))
        .route("/bids/:id/accept", axum::routing::post(accept_bid))
        .route("/award", axum::routing::post(award_bid))
        .route("/invite", axum::routing::post(invite_supplier))
}
