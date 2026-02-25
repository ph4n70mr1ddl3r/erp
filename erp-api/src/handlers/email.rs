use axum::{
    extract::{Path, Query, State, Extension},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::db::AppState;
use crate::handlers::auth::AuthUser;
use erp_email::*;

#[derive(Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub code: String,
    pub category: Option<String>,
    pub subject: String,
    pub preheader: Option<String>,
    pub html_body: String,
    pub text_body: Option<String>,
    pub variables: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct CreateCampaignRequest {
    pub name: String,
    pub description: Option<String>,
    pub subject: String,
    pub html_body: String,
    pub from_name: String,
    pub from_email: String,
    pub list_ids: Vec<Uuid>,
}

#[derive(Deserialize)]
pub struct CreateListRequest {
    pub name: String,
    pub description: Option<String>,
    pub list_type: ListType,
}

#[derive(Deserialize)]
pub struct AddSubscriberRequest {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Deserialize)]
pub struct ListQuery {
    pub category: Option<String>,
    pub status: Option<CampaignStatus>,
}

pub async fn create_template(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Json(req): Json<CreateTemplateRequest>,
) -> Result<Json<EmailTemplate>, StatusCode> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let service = EmailTemplateService::new();
    let template = service
        .create(
            &state.pool,
            req.name,
            req.code,
            req.category,
            req.subject,
            req.preheader,
            req.html_body,
            req.text_body,
            req.variables,
            user_id,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(template))
}

pub async fn list_templates(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<EmailTemplate>>, StatusCode> {
    let service = EmailTemplateService::new();
    let templates = service
        .list(&state.pool, query.category)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(templates))
}

pub async fn get_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<EmailTemplate>, StatusCode> {
    let service = EmailTemplateService::new();
    let template = service
        .get(&state.pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(template))
}

pub async fn delete_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let service = EmailTemplateService::new();
    service.delete(&state.pool, id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn create_campaign(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Json(req): Json<CreateCampaignRequest>,
) -> Result<Json<EmailCampaign>, StatusCode> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let service = EmailCampaignService::new();
    let campaign = service
        .create(
            &state.pool,
            req.name,
            req.description,
            req.subject,
            req.html_body,
            req.from_name,
            req.from_email,
            req.list_ids,
            user_id,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(campaign))
}

pub async fn list_campaigns(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> Result<Json<Vec<EmailCampaign>>, StatusCode> {
    let service = EmailCampaignService::new();
    let campaigns = service
        .list(&state.pool, query.status)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(campaigns))
}

pub async fn get_campaign(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<EmailCampaign>, StatusCode> {
    let service = EmailCampaignService::new();
    let campaign = service
        .get(&state.pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(campaign))
}

pub async fn schedule_campaign(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ScheduleRequest>,
) -> Result<StatusCode, StatusCode> {
    let service = EmailCampaignService::new();
    service.schedule(&state.pool, id, req.scheduled_at).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct ScheduleRequest {
    pub scheduled_at: chrono::DateTime<chrono::Utc>,
}

pub async fn send_campaign(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let service = EmailCampaignService::new();
    service.send(&state.pool, id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn get_campaign_stats(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<CampaignStats>, StatusCode> {
    let service = EmailCampaignService::new();
    let stats = service
        .get_stats(&state.pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(stats))
}

pub async fn create_list(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Json(req): Json<CreateListRequest>,
) -> Result<Json<EmailList>, StatusCode> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let service = EmailListService::new();
    let list = service
        .create(&state.pool, req.name, req.description, req.list_type, user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(list))
}

pub async fn add_subscriber(
    State(state): State<AppState>,
    Path(list_id): Path<Uuid>,
    Json(req): Json<AddSubscriberRequest>,
) -> Result<Json<EmailSubscriber>, StatusCode> {
    let service = EmailListService::new();
    let subscriber = service
        .add_subscriber(&state.pool, list_id, req.email, req.first_name, req.last_name)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(subscriber))
}

#[derive(Deserialize)]
pub struct UnsubscribeRequest {
    pub email: String,
}

pub async fn unsubscribe(
    State(state): State<AppState>,
    Path(list_id): Path<Uuid>,
    Json(req): Json<UnsubscribeRequest>,
) -> Result<StatusCode, StatusCode> {
    let service = EmailListService::new();
    service.unsubscribe(&state.pool, list_id, &req.email).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/templates", axum::routing::post(create_template).get(list_templates))
        .route("/templates/:id", axum::routing::get(get_template).delete(delete_template))
        .route("/campaigns", axum::routing::post(create_campaign).get(list_campaigns))
        .route("/campaigns/:id", axum::routing::get(get_campaign))
        .route("/campaigns/:id/schedule", axum::routing::post(schedule_campaign))
        .route("/campaigns/:id/send", axum::routing::post(send_campaign))
        .route("/campaigns/:id/stats", axum::routing::get(get_campaign_stats))
        .route("/lists", axum::routing::post(create_list))
        .route("/lists/:list_id/subscribers", axum::routing::post(add_subscriber))
        .route("/lists/:list_id/unsubscribe", axum::routing::post(unsubscribe))
}
