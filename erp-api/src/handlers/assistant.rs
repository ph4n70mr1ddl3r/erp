use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use crate::db::AppState;
use crate::error::ApiResult;
use erp_assistant::{AssistantService, CreateConversationRequest, SendMessageRequest, MessageFeedbackRequest};

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/conversations", axum::routing::get(list_conversations).post(create_conversation))
        .route("/conversations/:id", axum::routing::get(get_conversation).delete(archive_conversation))
        .route("/conversations/:id/messages", axum::routing::post(send_message).get(list_messages))
        .route("/messages/:id/feedback", axum::routing::post(provide_feedback))
        .route("/intents", axum::routing::get(list_intents).post(create_intent))
        .route("/intents/:id", axum::routing::get(get_intent).delete(delete_intent))
        .route("/skills", axum::routing::get(list_skills).post(create_skill))
        .route("/skills/:id", axum::routing::get(get_skill).delete(delete_skill))
        .route("/quick-actions", axum::routing::get(list_quick_actions).post(create_quick_action))
        .route("/quick-actions/:id", axum::routing::delete(delete_quick_action))
        .route("/parse", axum::routing::post(parse_query))
}

#[derive(Deserialize)]
struct CreateConversationBody {
    title: Option<String>,
    initial_message: Option<String>,
}

async fn create_conversation(
    State(_state): State<AppState>,
    axum::Extension(user_id): axum::Extension<Uuid>,
    Json(body): Json<CreateConversationBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = AssistantService::new();
    let conversation = service.create_conversation(user_id, CreateConversationRequest {
        title: body.title,
        initial_message: body.initial_message,
    }).await?;
    Ok(Json(serde_json::to_value(conversation)?))
}

async fn get_conversation(
    Path(id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = AssistantService::new();
    let conversation = service.get_conversation(id).await?;
    Ok(Json(serde_json::to_value(conversation)?))
}

async fn list_conversations(
    State(_state): State<AppState>,
    axum::Extension(user_id): axum::Extension<Uuid>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let service = AssistantService::new();
    let conversations = service.list_conversations(user_id, 50).await?;
    Ok(Json(conversations.into_iter().map(|c| serde_json::to_value(c).unwrap()).collect()))
}

async fn archive_conversation(
    Path(id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<StatusCode> {
    let service = AssistantService::new();
    service.archive_conversation(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
struct SendMessageBody {
    message: String,
    context: Option<erp_assistant::AssistantContext>,
}

async fn send_message(
    Path(conversation_id): Path<Uuid>,
    State(_state): State<AppState>,
    axum::Extension(user_id): axum::Extension<Uuid>,
    Json(body): Json<SendMessageBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = AssistantService::new();
    let response = service.send_message(user_id, SendMessageRequest {
        conversation_id,
        message: body.message,
        context: body.context,
    }).await?;
    Ok(Json(serde_json::to_value(response)?))
}

async fn list_messages(
    Path(_conversation_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    Ok(Json(Vec::new()))
}

#[derive(Deserialize)]
struct FeedbackBody {
    rating: i32,
    comment: Option<String>,
}

async fn provide_feedback(
    Path(message_id): Path<Uuid>,
    State(_state): State<AppState>,
    Json(body): Json<FeedbackBody>,
) -> ApiResult<StatusCode> {
    let service = AssistantService::new();
    service.provide_feedback(MessageFeedbackRequest {
        message_id,
        rating: body.rating,
        comment: body.comment,
    }).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn create_intent(
    State(_state): State<AppState>,
    Json(_intent): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(_intent))
}

async fn list_intents(
    State(_state): State<AppState>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let service = AssistantService::new();
    let intents = service.list_intents().await?;
    Ok(Json(intents.into_iter().map(|i| serde_json::to_value(i).unwrap()).collect()))
}

async fn get_intent(
    Path(_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({})))
}

async fn delete_intent(
    Path(_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

async fn create_skill(
    State(_state): State<AppState>,
    Json(_skill): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(_skill))
}

async fn list_skills(
    State(_state): State<AppState>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let service = AssistantService::new();
    let skills = service.list_skills(false).await?;
    Ok(Json(skills.into_iter().map(|s| serde_json::to_value(s).unwrap()).collect()))
}

async fn get_skill(
    Path(_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({})))
}

async fn delete_skill(
    Path(_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

async fn create_quick_action(
    State(_state): State<AppState>,
    Json(_action): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(_action))
}

async fn list_quick_actions(
    State(_state): State<AppState>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let service = AssistantService::new();
    let actions = service.list_quick_actions().await?;
    Ok(Json(actions.into_iter().map(|a| serde_json::to_value(a).unwrap()).collect()))
}

async fn delete_quick_action(
    Path(_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
struct ParseQueryBody {
    query: String,
}

async fn parse_query(
    State(_state): State<AppState>,
    Json(body): Json<ParseQueryBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = AssistantService::new();
    let parsed = service.parse_query(&body.query)?;
    Ok(Json(serde_json::to_value(parsed)?))
}
