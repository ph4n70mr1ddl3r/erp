use axum::{
    extract::{Path, Query, State, Extension},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::AppState;
use crate::handlers::auth::AuthUser;
use erp_chat::*;

#[derive(Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    pub description: Option<String>,
    pub channel_type: ChannelType,
    pub is_private: bool,
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
    pub parent_message_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct ListMessagesQuery {
    pub limit: Option<i32>,
    pub before: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Serialize)]
pub struct ChannelResponse {
    pub id: Uuid,
    pub name: String,
    pub channel_type: ChannelType,
    pub is_private: bool,
}

impl From<ChatChannel> for ChannelResponse {
    fn from(c: ChatChannel) -> Self {
        Self {
            id: c.base.id,
            name: c.name,
            channel_type: c.channel_type,
            is_private: c.is_private,
        }
    }
}

pub async fn create_channel(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Json(req): Json<CreateChannelRequest>,
) -> Result<Json<ChannelResponse>, StatusCode> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let service = ChatChannelService::new();
    let channel = service
        .create(&state.pool, req.name, req.description, req.channel_type, req.is_private, user_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(ChannelResponse::from(channel)))
}

pub async fn list_channels(
    State(state): State<AppState>,
    Query(query): Query<ListChannelsQuery>,
) -> Result<Json<Vec<ChannelResponse>>, StatusCode> {
    let service = ChatChannelService::new();
    let channels = service
        .list(&state.pool, query.channel_type)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(channels.into_iter().map(ChannelResponse::from).collect()))
}

#[derive(Deserialize)]
pub struct ListChannelsQuery {
    pub channel_type: Option<ChannelType>,
}

pub async fn join_channel(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let service = ChatChannelService::new();
    service.join(&state.pool, id, user_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn leave_channel(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let service = ChatChannelService::new();
    service.leave(&state.pool, id, user_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn send_message(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Path(channel_id): Path<Uuid>,
    Json(req): Json<SendMessageRequest>,
) -> Result<Json<ChatMessage>, StatusCode> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let service = ChatMessageService::new();
    let message = service
        .send(&state.pool, channel_id, user_id, req.content, req.parent_message_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(message))
}

pub async fn list_messages(
    State(state): State<AppState>,
    Path(channel_id): Path<Uuid>,
    Query(query): Query<ListMessagesQuery>,
) -> Result<Json<Vec<ChatMessage>>, StatusCode> {
    let service = ChatMessageService::new();
    let messages = service
        .list_by_channel(&state.pool, channel_id, query.limit.unwrap_or(50), query.before)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(messages))
}

pub async fn delete_message(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let service = ChatMessageService::new();
    service.delete(&state.pool, id, user_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

#[derive(Deserialize)]
pub struct SendDMRequest {
    pub recipient_id: Uuid,
    pub content: String,
}

pub async fn send_direct_message(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Json(req): Json<SendDMRequest>,
) -> Result<Json<DirectMessage>, StatusCode> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let service = DirectMessageService::new();
    let dm = service
        .send(&state.pool, user_id, req.recipient_id, req.content)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(dm))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/channels", axum::routing::post(create_channel).get(list_channels))
        .route("/channels/:id/join", axum::routing::post(join_channel))
        .route("/channels/:id/leave", axum::routing::post(leave_channel))
        .route("/channels/:channel_id/messages", axum::routing::post(send_message).get(list_messages))
        .route("/messages/:id", axum::routing::delete(delete_message))
        .route("/dm", axum::routing::post(send_direct_message))
}
