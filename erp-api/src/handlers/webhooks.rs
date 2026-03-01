use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::AppState;
use crate::error::ApiResult;

#[derive(Debug, Deserialize)]
pub struct CreateWebhookRequest {
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub events: Vec<String>,
    pub headers: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct WebhookEndpointResponse {
    pub id: Uuid,
    pub name: String,
    pub url: String,
    pub status: String,
    pub total_triggers: i64,
    pub successful_triggers: i64,
    pub failed_triggers: i64,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct TriggerWebhookRequest {
    pub event_type: String,
    pub source_entity_type: String,
    pub source_entity_id: Uuid,
    pub payload: serde_json::Value,
}

pub async fn create_webhook(
    State(state): State<AppState>,
    Json(req): Json<CreateWebhookRequest>,
) -> ApiResult<Json<WebhookEndpointResponse>> {
    let created_by = Uuid::nil();
    let events = req.events.iter()
        .map(|s| parse_event_type(s))
        .collect::<Result<Vec<_>, _>>()?;
    
    let service = erp_webhooks::WebhookService::new();
    let endpoint = service.create_endpoint(
        &state.pool,
        req.name,
        req.description,
        req.url,
        events,
        created_by,
        req.headers,
        None,
    ).await?;
    
    Ok(Json(WebhookEndpointResponse {
        id: endpoint.base.id,
        name: endpoint.name,
        url: endpoint.url,
        status: format!("{:?}", endpoint.status),
        total_triggers: endpoint.total_triggers,
        successful_triggers: endpoint.successful_triggers,
        failed_triggers: endpoint.failed_triggers,
        created_at: endpoint.created_at.to_rfc3339(),
    }))
}

pub async fn list_webhooks(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<WebhookEndpointResponse>>> {
    let service = erp_webhooks::WebhookService::new();
    let endpoints = service.list_endpoints(&state.pool).await?;
    
    let response: Vec<WebhookEndpointResponse> = endpoints.into_iter().map(|e| WebhookEndpointResponse {
        id: e.base.id,
        name: e.name,
        url: e.url,
        status: format!("{:?}", e.status),
        total_triggers: e.total_triggers,
        successful_triggers: e.successful_triggers,
        failed_triggers: e.failed_triggers,
        created_at: e.created_at.to_rfc3339(),
    }).collect();
    
    Ok(Json(response))
}

pub async fn get_webhook(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<WebhookEndpointResponse>> {
    let service = erp_webhooks::WebhookService::new();
    let endpoint = service.get_endpoint(&state.pool, id).await?
        .ok_or_else(|| erp_core::Error::NotFound("Webhook not found".into()))?;
    
    Ok(Json(WebhookEndpointResponse {
        id: endpoint.base.id,
        name: endpoint.name,
        url: endpoint.url,
        status: format!("{:?}", endpoint.status),
        total_triggers: endpoint.total_triggers,
        successful_triggers: endpoint.successful_triggers,
        failed_triggers: endpoint.failed_triggers,
        created_at: endpoint.created_at.to_rfc3339(),
    }))
}

pub async fn delete_webhook(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let service = erp_webhooks::WebhookService::new();
    service.delete_endpoint(&state.pool, id).await?;
    Ok(StatusCode::OK)
}

pub async fn trigger_webhook(
    State(state): State<AppState>,
    Json(req): Json<TriggerWebhookRequest>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let triggered_by = Uuid::nil();
    let event_type = parse_event_type(&req.event_type)?;
    
    let service = erp_webhooks::WebhookService::new();
    let deliveries = service.trigger(
        &state.pool,
        event_type,
        req.source_entity_type,
        req.source_entity_id,
        req.payload,
        triggered_by,
    ).await?;
    
    let response: Vec<serde_json::Value> = deliveries.iter().map(|d| serde_json::json!({
        "id": d.base.id,
        "status": format!("{:?}", d.status),
        "event_type": format!("{:?}", d.event_type),
    })).collect();
    
    Ok(Json(response))
}

pub async fn ping_webhook(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = erp_webhooks::WebhookService::new();
    let success = service.ping_endpoint(&state.pool, id).await?;
    
    Ok(Json(serde_json::json!({
        "success": success,
        "message": if success { "Webhook endpoint is reachable" } else { "Webhook endpoint is not reachable" }
    })))
}

pub async fn rotate_webhook_secret(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = erp_webhooks::WebhookService::new();
    let new_secret = service.rotate_secret(&state.pool, id).await?;
    
    Ok(Json(serde_json::json!({
        "secret": new_secret,
        "message": "Secret rotated successfully. Update your webhook handler with the new secret."
    })))
}

fn parse_event_type(s: &str) -> anyhow::Result<erp_webhooks::WebhookEventType> {
    match s {
        "OrderCreated" => Ok(erp_webhooks::WebhookEventType::OrderCreated),
        "OrderUpdated" => Ok(erp_webhooks::WebhookEventType::OrderUpdated),
        "OrderCancelled" => Ok(erp_webhooks::WebhookEventType::OrderCancelled),
        "OrderCompleted" => Ok(erp_webhooks::WebhookEventType::OrderCompleted),
        "InvoiceCreated" => Ok(erp_webhooks::WebhookEventType::InvoiceCreated),
        "InvoicePaid" => Ok(erp_webhooks::WebhookEventType::InvoicePaid),
        "PaymentReceived" => Ok(erp_webhooks::WebhookEventType::PaymentReceived),
        "ShipmentCreated" => Ok(erp_webhooks::WebhookEventType::ShipmentCreated),
        "ShipmentDelivered" => Ok(erp_webhooks::WebhookEventType::ShipmentDelivered),
        "CustomerCreated" => Ok(erp_webhooks::WebhookEventType::CustomerCreated),
        "TicketCreated" => Ok(erp_webhooks::WebhookEventType::TicketCreated),
        "TicketClosed" => Ok(erp_webhooks::WebhookEventType::TicketClosed),
        "TaskCreated" => Ok(erp_webhooks::WebhookEventType::TaskCreated),
        "TaskCompleted" => Ok(erp_webhooks::WebhookEventType::TaskCompleted),
        "ApprovalRequested" => Ok(erp_webhooks::WebhookEventType::ApprovalRequested),
        "ApprovalApproved" => Ok(erp_webhooks::WebhookEventType::ApprovalApproved),
        "ApprovalRejected" => Ok(erp_webhooks::WebhookEventType::ApprovalRejected),
        "Custom" => Ok(erp_webhooks::WebhookEventType::Custom),
        _ => Err(anyhow::anyhow!("Invalid event type: {}", s)),
    }
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", axum::routing::post(create_webhook).get(list_webhooks))
        .route("/:id", axum::routing::get(get_webhook).delete(delete_webhook))
        .route("/:id/ping", axum::routing::post(ping_webhook))
        .route("/:id/rotate-secret", axum::routing::post(rotate_webhook_secret))
        .route("/trigger", axum::routing::post(trigger_webhook))
}
