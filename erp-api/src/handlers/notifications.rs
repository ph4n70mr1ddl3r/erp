use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::AppState;
use crate::error::ApiResult;

#[derive(Debug, Deserialize)]
pub struct CreateNotificationRequest {
    pub user_id: Uuid,
    pub notification_type: String,
    pub channel: String,
    pub title: String,
    pub body: String,
    pub action_url: Option<String>,
    pub action_text: Option<String>,
    pub priority: Option<String>,
    pub data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct NotificationResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub notification_type: String,
    pub channel: String,
    pub priority: String,
    pub title: String,
    pub body: String,
    pub action_url: Option<String>,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct NotificationListQuery {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

pub async fn send_notification(
    State(state): State<AppState>,
    Json(req): Json<CreateNotificationRequest>,
) -> ApiResult<Json<NotificationResponse>> {
    let notification_type = parse_notification_type(&req.notification_type)?;
    let channel = parse_notification_channel(&req.channel)?;
    let priority = req.priority.as_deref()
        .map(parse_notification_priority)
        .transpose()?
        .unwrap_or(erp_notifications::NotificationPriority::Normal);
    
    let service = erp_notifications::NotificationService::new();
    let notification = service.send(
        &state.pool,
        req.user_id,
        notification_type,
        channel,
        req.title,
        req.body,
        req.action_url,
        req.action_text,
        Some(priority),
        req.data,
    ).await?;
    
    Ok(Json(NotificationResponse {
        id: notification.base.id,
        user_id: notification.user_id,
        notification_type: format!("{:?}", notification.notification_type),
        channel: format!("{:?}", notification.channel),
        priority: format!("{:?}", notification.priority),
        title: notification.title,
        body: notification.body,
        action_url: notification.action_url,
        status: format!("{:?}", notification.status),
        created_at: notification.created_at.to_rfc3339(),
    }))
}

pub async fn list_notifications(
    State(state): State<AppState>,
    Query(query): Query<NotificationListQuery>,
) -> ApiResult<Json<Vec<NotificationResponse>>> {
    let user_id = Uuid::nil();
    let limit = query.limit.unwrap_or(50);
    let offset = query.offset.unwrap_or(0);
    
    let service = erp_notifications::NotificationService::new();
    let notifications = service.list_for_user(&state.pool, user_id, limit, offset).await?;
    
    let response: Vec<NotificationResponse> = notifications.into_iter().map(|n| NotificationResponse {
        id: n.base.id,
        user_id: n.user_id,
        notification_type: format!("{:?}", n.notification_type),
        channel: format!("{:?}", n.channel),
        priority: format!("{:?}", n.priority),
        title: n.title,
        body: n.body,
        action_url: n.action_url,
        status: format!("{:?}", n.status),
        created_at: n.created_at.to_rfc3339(),
    }).collect();
    
    Ok(Json(response))
}

pub async fn mark_notification_read(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let service = erp_notifications::NotificationService::new();
    service.mark_read(&state.pool, id).await?;
    Ok(StatusCode::OK)
}

pub async fn mark_all_notifications_read(
    State(state): State<AppState>,
) -> ApiResult<StatusCode> {
    let user_id = Uuid::nil();
    let service = erp_notifications::NotificationService::new();
    service.mark_all_read(&state.pool, user_id).await?;
    Ok(StatusCode::OK)
}

pub async fn unread_count(
    State(state): State<AppState>,
) -> ApiResult<Json<i64>> {
    let user_id = Uuid::nil();
    let service = erp_notifications::NotificationService::new();
    let count = service.unread_count(&state.pool, user_id).await?;
    Ok(Json(count))
}

fn parse_notification_type(s: &str) -> anyhow::Result<erp_notifications::NotificationType> {
    match s {
        "System" => Ok(erp_notifications::NotificationType::System),
        "Alert" => Ok(erp_notifications::NotificationType::Alert),
        "Reminder" => Ok(erp_notifications::NotificationType::Reminder),
        "Approval" => Ok(erp_notifications::NotificationType::Approval),
        "Task" => Ok(erp_notifications::NotificationType::Task),
        "Message" => Ok(erp_notifications::NotificationType::Message),
        "Event" => Ok(erp_notifications::NotificationType::Event),
        "Report" => Ok(erp_notifications::NotificationType::Report),
        "Workflow" => Ok(erp_notifications::NotificationType::Workflow),
        "Security" => Ok(erp_notifications::NotificationType::Security),
        "Billing" => Ok(erp_notifications::NotificationType::Billing),
        "Update" => Ok(erp_notifications::NotificationType::Update),
        _ => Err(anyhow::anyhow!("Invalid notification type: {}", s)),
    }
}

fn parse_notification_channel(s: &str) -> anyhow::Result<erp_notifications::NotificationChannel> {
    match s {
        "InApp" => Ok(erp_notifications::NotificationChannel::InApp),
        "Email" => Ok(erp_notifications::NotificationChannel::Email),
        "SMS" => Ok(erp_notifications::NotificationChannel::SMS),
        "Push" => Ok(erp_notifications::NotificationChannel::Push),
        "Slack" => Ok(erp_notifications::NotificationChannel::Slack),
        "Teams" => Ok(erp_notifications::NotificationChannel::Teams),
        "Webhook" => Ok(erp_notifications::NotificationChannel::Webhook),
        _ => Err(anyhow::anyhow!("Invalid notification channel: {}", s)),
    }
}

fn parse_notification_priority(s: &str) -> anyhow::Result<erp_notifications::NotificationPriority> {
    match s {
        "Low" => Ok(erp_notifications::NotificationPriority::Low),
        "Normal" => Ok(erp_notifications::NotificationPriority::Normal),
        "High" => Ok(erp_notifications::NotificationPriority::High),
        "Urgent" => Ok(erp_notifications::NotificationPriority::Urgent),
        _ => Err(anyhow::anyhow!("Invalid notification priority: {}", s)),
    }
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", axum::routing::post(send_notification).get(list_notifications))
        .route("/:id/read", axum::routing::post(mark_notification_read))
        .route("/read-all", axum::routing::post(mark_all_notifications_read))
        .route("/unread-count", axum::routing::get(unread_count))
}
