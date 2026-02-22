use axum::{
    extract::{Multipart, Path, Query, State},
    Json,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{Attachment, AttachmentService};

#[derive(Debug, Deserialize)]
pub struct AttachmentQuery {
    pub entity_type: String,
    pub entity_id: String,
}

#[derive(Debug, Serialize)]
pub struct AttachmentResponse {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: String,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub created_at: String,
}

impl From<Attachment> for AttachmentResponse {
    fn from(a: Attachment) -> Self {
        Self {
            id: a.id,
            entity_type: a.entity_type,
            entity_id: a.entity_id,
            filename: a.filename,
            original_filename: a.original_filename,
            mime_type: a.mime_type,
            file_size: a.file_size,
            created_at: a.created_at.to_rfc3339(),
        }
    }
}

pub async fn list_attachments(
    State(state): State<AppState>,
    Query(query): Query<AttachmentQuery>,
) -> ApiResult<Json<Vec<AttachmentResponse>>> {
    let attachments = AttachmentService::list_for_entity(
        &state.pool,
        &query.entity_type,
        &query.entity_id,
    ).await?;
    
    Ok(Json(attachments.into_iter().map(AttachmentResponse::from).collect()))
}

pub async fn upload_attachment(
    State(state): State<AppState>,
    Query(query): Query<AttachmentQuery>,
    mut multipart: Multipart,
) -> ApiResult<Json<AttachmentResponse>> {
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        erp_core::Error::validation(&format!("Failed to read multipart: {}", e))
    })? {
        let original_filename = field.file_name().unwrap_or("unknown").to_string();
        let mime_type = field.content_type().unwrap_or("application/octet-stream").to_string();
        let data = field.bytes().await.map_err(|e| {
            erp_core::Error::validation(&format!("Failed to read file data: {}", e))
        })?;
        
        let file_size = data.len() as i64;
        
        let upload_dir = std::path::PathBuf::from("uploads").join(&query.entity_type);
        std::fs::create_dir_all(&upload_dir).ok();
        
        let attachment = Attachment::new(
            &query.entity_type,
            &query.entity_id,
            &original_filename,
            &mime_type,
            file_size,
            None,
        );
        
        let file_path = std::path::PathBuf::from(&attachment.file_path);
        std::fs::write(&file_path, &data).map_err(|e| {
            erp_core::Error::validation(&format!("Failed to write file: {}", e))
        })?;
        
        let created = AttachmentService::create(&state.pool, attachment).await?;
        
        return Ok(Json(AttachmentResponse::from(created)));
    }
    
    Err(erp_core::Error::validation("No file provided").into())
}

pub async fn get_attachment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<AttachmentResponse>> {
    let attachment = AttachmentService::get(&state.pool, id).await?;
    Ok(Json(AttachmentResponse::from(attachment)))
}

pub async fn delete_attachment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let attachment = AttachmentService::get(&state.pool, id).await?;
    
    std::fs::remove_file(&attachment.file_path).ok();
    
    AttachmentService::delete(&state.pool, id).await?;
    
    Ok(Json(serde_json::json!({ "status": "deleted" })))
}
