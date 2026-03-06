use axum::{
    extract::{Path, Query, State},
    Json,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::AppState;
use crate::error::ApiResult;
use erp_notes::{CreateNoteRequest, Note, NoteService, UpdateNoteRequest};

const ALLOWED_ENTITY_TYPES: &[&str] = &[
    "products", "customers", "vendors", "orders", "invoices", 
    "accounts", "journal_entries", "employees", "projects",
    "tickets", "leads", "opportunities"
];

const MAX_CONTENT_LENGTH: usize = 10000;
const MAX_TITLE_LENGTH: usize = 200;

fn validate_entity_type(entity_type: &str) -> Result<(), erp_core::Error> {
    if entity_type.contains("..") || entity_type.contains('/') || entity_type.contains('\\') {
        return Err(erp_core::Error::validation("Invalid entity_type: path traversal detected"));
    }
    if !ALLOWED_ENTITY_TYPES.contains(&entity_type) {
        return Err(erp_core::Error::validation(format!(
            "Invalid entity_type: '{}'. Allowed types: {}",
            entity_type,
            ALLOWED_ENTITY_TYPES.join(", ")
        )));
    }
    Ok(())
}

fn validate_content(content: &str) -> Result<(), erp_core::Error> {
    if content.trim().is_empty() {
        return Err(erp_core::Error::validation("Note content cannot be empty"));
    }
    if content.len() > MAX_CONTENT_LENGTH {
        return Err(erp_core::Error::validation(format!(
            "Note content too long. Maximum {} characters",
            MAX_CONTENT_LENGTH
        )));
    }
    Ok(())
}

fn validate_title(title: &Option<String>) -> Result<(), erp_core::Error> {
    if let Some(t) = title {
        if t.len() > MAX_TITLE_LENGTH {
            return Err(erp_core::Error::validation(format!(
                "Note title too long. Maximum {} characters",
                MAX_TITLE_LENGTH
            )));
        }
    }
    Ok(())
}

fn validate_reminder(reminder_at: &Option<DateTime<Utc>>) -> Result<(), erp_core::Error> {
    if let Some(reminder) = reminder_at {
        if reminder <= &Utc::now() {
            return Err(erp_core::Error::validation("Reminder must be in the future"));
        }
    }
    Ok(())
}

#[derive(Debug, Deserialize)]
pub struct NotesQuery {
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

#[derive(Debug, Serialize)]
pub struct NoteResponse {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub note_type: String,
    pub title: Option<String>,
    pub content: String,
    pub is_private: bool,
    pub is_pinned: bool,
    pub reminder_at: Option<String>,
    pub reminded_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub created_by: Option<Uuid>,
}

impl From<Note> for NoteResponse {
    fn from(note: Note) -> Self {
        Self {
            id: note.base.id,
            entity_type: note.entity_type,
            entity_id: note.entity_id,
            note_type: note.note_type.to_string(),
            title: note.title,
            content: note.content,
            is_private: note.is_private,
            is_pinned: note.is_pinned,
            reminder_at: note.reminder_at.map(|d| d.to_rfc3339()),
            reminded_at: note.reminded_at.map(|d| d.to_rfc3339()),
            created_at: note.base.created_at.to_rfc3339(),
            updated_at: note.base.updated_at.to_rfc3339(),
            created_by: note.base.created_by,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteBody {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub note_type: Option<String>,
    pub title: Option<String>,
    pub content: String,
    pub is_private: Option<bool>,
    pub reminder_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNoteBody {
    pub note_type: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_private: Option<bool>,
    pub is_pinned: Option<bool>,
    pub reminder_at: Option<DateTime<Utc>>,
}

pub async fn list_notes(
    State(state): State<AppState>,
    Query(query): Query<NotesQuery>,
) -> ApiResult<Json<Vec<NoteResponse>>> {
    let svc = NoteService::new();
    
    let notes = if let (Some(entity_type), Some(entity_id)) = (query.entity_type, query.entity_id) {
        validate_entity_type(&entity_type)?;
        svc.list_for_entity(&state.pool, &entity_type, entity_id).await?
    } else {
        let user_id = Uuid::nil();
        let page = query.page.unwrap_or(1).max(1);
        let page_size = query.page_size.unwrap_or(50).clamp(1, 100);
        svc.list_for_user(&state.pool, user_id, page, page_size).await?
    };
    
    Ok(Json(notes.into_iter().map(NoteResponse::from).collect()))
}

pub async fn create_note(
    State(state): State<AppState>,
    Json(body): Json<CreateNoteBody>,
) -> ApiResult<Json<NoteResponse>> {
    validate_entity_type(&body.entity_type)?;
    validate_content(&body.content)?;
    validate_title(&body.title)?;
    validate_reminder(&body.reminder_at)?;
    
    let svc = NoteService::new();
    let user_id = Uuid::nil();
    
    let req = CreateNoteRequest {
        entity_type: body.entity_type,
        entity_id: body.entity_id,
        note_type: body.note_type,
        title: body.title,
        content: body.content,
        is_private: body.is_private,
        reminder_at: body.reminder_at,
    };
    
    let note = svc.create(&state.pool, req, user_id).await?;
    
    Ok(Json(NoteResponse::from(note)))
}

pub async fn get_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<NoteResponse>> {
    let svc = NoteService::new();
    let note = svc.get(&state.pool, id).await?;
    Ok(Json(NoteResponse::from(note)))
}

pub async fn update_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateNoteBody>,
) -> ApiResult<Json<NoteResponse>> {
    let svc = NoteService::new();
    
    if let Some(content) = &body.content {
        validate_content(content)?;
    }
    validate_title(&body.title)?;
    validate_reminder(&body.reminder_at)?;
    
    let user_id = Uuid::nil();
    
    let req = UpdateNoteRequest {
        note_type: body.note_type,
        title: body.title,
        content: body.content,
        is_private: body.is_private,
        is_pinned: body.is_pinned,
        reminder_at: body.reminder_at,
    };
    
    let note = svc.update(&state.pool, id, req, user_id).await?;
    
    Ok(Json(NoteResponse::from(note)))
}

pub async fn delete_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let svc = NoteService::new();
    svc.delete(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "deleted" })))
}

pub async fn pin_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<NoteResponse>> {
    let svc = NoteService::new();
    let user_id = Uuid::nil();
    let note = svc.pin(&state.pool, id, user_id).await?;
    Ok(Json(NoteResponse::from(note)))
}

pub async fn unpin_note(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<NoteResponse>> {
    let svc = NoteService::new();
    let user_id = Uuid::nil();
    let note = svc.unpin(&state.pool, id, user_id).await?;
    Ok(Json(NoteResponse::from(note)))
}

pub fn routes() -> axum::Router<crate::db::AppState> {
    axum::Router::new()
        .route("/", axum::routing::get(list_notes).post(create_note))
        .route("/:id", axum::routing::get(get_note).put(update_note).delete(delete_note))
        .route("/:id/pin", axum::routing::post(pin_note))
        .route("/:id/unpin", axum::routing::post(unpin_note))
}
