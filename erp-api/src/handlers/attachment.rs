use axum::{
    extract::{Multipart, Path, Query, State},
    Json,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{Attachment, AttachmentService};

const ALLOWED_ENTITY_TYPES: &[&str] = &[
    "products", "customers", "vendors", "orders", "invoices", 
    "accounts", "journal_entries", "employees"
];

const MAX_FILE_SIZE: usize = 10 * 1024 * 1024;

fn sanitize_filename(filename: &str) -> String {
    let sanitized: String = filename
        .chars()
        .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '-' || *c == '_')
        .collect();
    if sanitized.is_empty() {
        "file".to_string()
    } else {
        sanitized
    }
}

fn validate_entity_type(entity_type: &str) -> Result<(), erp_core::Error> {
    if !ALLOWED_ENTITY_TYPES.contains(&entity_type) {
        return Err(erp_core::Error::validation(&format!(
            "Invalid entity_type: '{}'. Allowed types: {}",
            entity_type,
            ALLOWED_ENTITY_TYPES.join(", ")
        )));
    }
    Ok(())
}

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
    validate_entity_type(&query.entity_type)?;
    
    while let Some(field) = multipart.next_field().await.map_err(|e| {
        erp_core::Error::validation(&format!("Failed to read multipart: {}", e))
    })? {
        let original_filename = sanitize_filename(field.file_name().unwrap_or("unknown"));
        let mime_type = field.content_type().unwrap_or("application/octet-stream").to_string();
        let data = field.bytes().await.map_err(|e| {
            erp_core::Error::validation(&format!("Failed to read file data: {}", e))
        })?;
        
        if data.len() > MAX_FILE_SIZE {
            return Err(erp_core::Error::validation(&format!(
                "File too large. Maximum size is {} bytes",
                MAX_FILE_SIZE
            )).into());
        }
        
        let file_size = data.len() as i64;
        
        let upload_dir = std::path::PathBuf::from("uploads").join(&query.entity_type);
        std::fs::create_dir_all(&upload_dir).map_err(|e| {
            erp_core::Error::validation(&format!("Failed to create upload directory: {}", e))
        })?;
        
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
