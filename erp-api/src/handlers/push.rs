use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::AppState;
use crate::ApiResult;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/devices", post(register_device).get(get_user_devices))
        .route("/send", post(send_notification))
        .route("/broadcast", post(send_broadcast))
        .route("/templates", post(create_template).get(list_templates))
        .route("/preferences", post(set_preference))
}

#[derive(Deserialize)]
pub struct RegisterDeviceRequest {
    pub user_id: String,
    pub device_token: String,
    pub platform: String,
}

#[derive(Serialize)]
pub struct DeviceResponse {
    pub id: String,
    pub user_id: String,
    pub platform: String,
    pub is_active: bool,
    pub push_enabled: bool,
}

pub async fn register_device(
    State(state): State<AppState>,
    Json(req): Json<RegisterDeviceRequest>,
) -> ApiResult<Json<DeviceResponse>> {
    let user_id = Uuid::parse_str(&req.user_id)?;
    // Platform is stored as a string in the model
    let platform = req.platform;

    let service = erp_push::PushService::new();
    let device = service.register_device(&state.pool, user_id, req.device_token, platform).await?;

    Ok(Json(DeviceResponse {
        id: device.id.to_string(),
        user_id: device.user_id.to_string(),
        platform: format!("{:?}", device.platform),
        is_active: device.is_active,
        push_enabled: device.push_enabled,
    }))
}

pub async fn get_user_devices(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> ApiResult<Json<Vec<DeviceResponse>>> {
    let user_id = params.get("user_id")
        .ok_or_else(|| anyhow::anyhow!("user_id required"))?;
    let user_id = Uuid::parse_str(user_id)?;

    let service = erp_push::PushService::new();
    let devices = service.get_user_devices(&state.pool, user_id).await?;

    Ok(Json(devices.into_iter().map(|d| DeviceResponse {
        id: d.id.to_string(),
        user_id: d.user_id.to_string(),
        platform: format!("{:?}", d.platform),
        is_active: d.is_active,
        push_enabled: d.push_enabled,
    }).collect()))
}

#[derive(Deserialize)]
pub struct SendNotificationRequest {
    pub title: String,
    pub body: String,
    pub user_ids: Vec<String>,
    pub data: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct NotificationResponse {
    pub id: String,
    pub title: String,
    pub body: String,
    pub status: String,
}

pub async fn send_notification(
    State(state): State<AppState>,
    Json(req): Json<SendNotificationRequest>,
) -> ApiResult<Json<NotificationResponse>> {
    let user_ids: Result<Vec<Uuid>, _> = req.user_ids.iter().map(|id| Uuid::parse_str(id)).collect();
    let user_ids = user_ids?;

    let service = erp_push::PushService::new();
    let notification = service.send_notification(&state.pool, req.title, req.body, user_ids, req.data).await?;

    Ok(Json(NotificationResponse {
        id: notification.id.to_string(),
        title: notification.title,
        body: notification.body,
        status: notification.status,
    }))
}

#[derive(Deserialize)]
pub struct SendBroadcastRequest {
    pub title: String,
    pub body: String,
    pub data: Option<serde_json::Value>,
}

pub async fn send_broadcast(
    State(state): State<AppState>,
    Json(req): Json<SendBroadcastRequest>,
) -> ApiResult<Json<NotificationResponse>> {
    let service = erp_push::PushService::new();
    let notification = service.send_to_all(&state.pool, req.title, req.body, req.data).await?;

    Ok(Json(NotificationResponse {
        id: notification.id.to_string(),
        title: notification.title,
        body: notification.body,
        status: notification.status,
    }))
}

#[derive(Deserialize)]
pub struct CreateTemplateRequest {
    pub code: String,
    pub name: String,
    pub title_template: String,
    pub body_template: String,
    pub category: String,
}

#[derive(Serialize)]
pub struct TemplateResponse {
    pub id: String,
    pub code: String,
    pub name: String,
    pub category: String,
    pub is_active: bool,
}

pub async fn create_template(
    State(state): State<AppState>,
    Json(req): Json<CreateTemplateRequest>,
) -> ApiResult<Json<TemplateResponse>> {
    let service = erp_push::PushService::new();
    let template = service.create_template(&state.pool, req.code, req.name, req.title_template, req.body_template, req.category).await?;

    Ok(Json(TemplateResponse {
        id: template.id.to_string(),
        code: template.code,
        name: template.name,
        category: template.category,
        is_active: template.is_active,
    }))
}

pub async fn list_templates(State(state): State<AppState>) -> ApiResult<Json<Vec<TemplateResponse>>> {
    let rows: Vec<(String, String, String, String, bool)> = sqlx::query_as(
        "SELECT id, code, name, category, is_active FROM push_templates"
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(rows.into_iter().map(|r| TemplateResponse {
        id: r.0,
        code: r.1,
        name: r.2,
        category: r.3,
        is_active: r.4,
    }).collect()))
}

#[derive(Deserialize)]
pub struct SetPreferenceRequest {
    pub user_id: String,
    pub category: String,
    pub push_enabled: bool,
}

#[derive(Serialize)]
pub struct PreferenceResponse {
    pub user_id: String,
    pub category: String,
    pub push_enabled: bool,
}

pub async fn set_preference(
    State(state): State<AppState>,
    Json(req): Json<SetPreferenceRequest>,
) -> ApiResult<Json<PreferenceResponse>> {
    let user_id = Uuid::parse_str(&req.user_id)?;
    let service = erp_push::PushService::new();
    let pref = service.set_preference(&state.pool, user_id, req.category, req.push_enabled).await?;

    Ok(Json(PreferenceResponse {
        user_id: pref.user_id.to_string(),
        category: pref.category,
        push_enabled: pref.push_enabled,
    }))
}
