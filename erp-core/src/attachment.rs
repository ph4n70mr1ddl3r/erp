use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: String,
    pub filename: String,
    pub original_filename: String,
    pub mime_type: String,
    pub file_size: i64,
    pub file_path: String,
    pub uploaded_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl Attachment {
    pub fn new(
        entity_type: &str,
        entity_id: &str,
        original_filename: &str,
        mime_type: &str,
        file_size: i64,
        uploaded_by: Option<Uuid>,
    ) -> Self {
        let extension = std::path::Path::new(original_filename)
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("");
        
        let filename = format!("{}{}.{}", 
            Uuid::new_v4(),
            if extension.is_empty() { "" } else { "." },
            extension
        );
        
        Self {
            id: Uuid::new_v4(),
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            filename: filename.clone(),
            original_filename: original_filename.to_string(),
            mime_type: mime_type.to_string(),
            file_size,
            file_path: format!("uploads/{}/{}", entity_type, filename),
            uploaded_by,
            created_at: Utc::now(),
        }
    }
}

pub struct AttachmentService;

impl AttachmentService {
    pub async fn list_for_entity(
        pool: &sqlx::SqlitePool,
        entity_type: &str,
        entity_id: &str,
    ) -> crate::Result<Vec<Attachment>> {
        let rows = sqlx::query_as::<_, AttachmentRow>(
            "SELECT id, entity_type, entity_id, filename, original_filename, mime_type, file_size, file_path, uploaded_by, created_at
             FROM attachments WHERE entity_type = ? AND entity_id = ? ORDER BY created_at DESC"
        )
        .bind(entity_type)
        .bind(entity_id)
        .fetch_all(pool)
        .await
        .map_err(|e| crate::Error::Database(e.into()))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn create(
        pool: &sqlx::SqlitePool,
        attachment: Attachment,
    ) -> crate::Result<Attachment> {
        sqlx::query(
            "INSERT INTO attachments (id, entity_type, entity_id, filename, original_filename, mime_type, file_size, file_path, uploaded_by, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(attachment.id.to_string())
        .bind(&attachment.entity_type)
        .bind(&attachment.entity_id)
        .bind(&attachment.filename)
        .bind(&attachment.original_filename)
        .bind(&attachment.mime_type)
        .bind(attachment.file_size)
        .bind(&attachment.file_path)
        .bind(attachment.uploaded_by.map(|id| id.to_string()))
        .bind(attachment.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| crate::Error::Database(e.into()))?;
        
        Ok(attachment)
    }

    pub async fn get(
        pool: &sqlx::SqlitePool,
        id: Uuid,
    ) -> crate::Result<Attachment> {
        let row = sqlx::query_as::<_, AttachmentRow>(
            "SELECT id, entity_type, entity_id, filename, original_filename, mime_type, file_size, file_path, uploaded_by, created_at
             FROM attachments WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| crate::Error::Database(e.into()))?
        .ok_or_else(|| crate::Error::not_found("Attachment", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn delete(
        pool: &sqlx::SqlitePool,
        id: Uuid,
    ) -> crate::Result<()> {
        let rows = sqlx::query("DELETE FROM attachments WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await
            .map_err(|e| crate::Error::Database(e.into()))?;
        
        if rows.rows_affected() == 0 {
            return Err(crate::Error::not_found("Attachment", &id.to_string()));
        }
        
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct AttachmentRow {
    id: String,
    entity_type: String,
    entity_id: String,
    filename: String,
    original_filename: String,
    mime_type: String,
    file_size: i64,
    file_path: String,
    uploaded_by: Option<String>,
    created_at: String,
}

impl From<AttachmentRow> for Attachment {
    fn from(r: AttachmentRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            entity_type: r.entity_type,
            entity_id: r.entity_id,
            filename: r.filename,
            original_filename: r.original_filename,
            mime_type: r.mime_type,
            file_size: r.file_size,
            file_path: r.file_path,
            uploaded_by: r.uploaded_by.and_then(|id| Uuid::parse_str(&id).ok()),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}
