use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::Result;
use crate::models::*;

#[async_trait]
pub trait DocumentRepository: Send + Sync {
    async fn create_folder(&self, pool: &SqlitePool, folder: DocumentFolder) -> Result<DocumentFolder>;
    async fn get_folder(&self, pool: &SqlitePool, id: Uuid) -> Result<DocumentFolder>;
    async fn list_folders(&self, pool: &SqlitePool, parent_id: Option<Uuid>) -> Result<Vec<DocumentFolder>>;
    async fn create_document(&self, pool: &SqlitePool, doc: Document) -> Result<Document>;
    async fn get_document(&self, pool: &SqlitePool, id: Uuid) -> Result<Document>;
    async fn update_document(&self, pool: &SqlitePool, doc: Document) -> Result<Document>;
    async fn delete_document(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn list_documents(&self, pool: &SqlitePool, folder_id: Option<Uuid>, status: Option<DocumentStatus>) -> Result<Vec<Document>>;
    async fn create_version(&self, pool: &SqlitePool, version: DocumentVersion) -> Result<DocumentVersion>;
    async fn list_versions(&self, pool: &SqlitePool, document_id: Uuid) -> Result<Vec<DocumentVersion>>;
    async fn create_checkout(&self, pool: &SqlitePool, checkout: DocumentCheckout) -> Result<DocumentCheckout>;
    async fn checkin(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn create_review(&self, pool: &SqlitePool, review: DocumentReview) -> Result<DocumentReview>;
    async fn create_permission(&self, pool: &SqlitePool, perm: DocumentPermission) -> Result<DocumentPermission>;
    async fn create_relation(&self, pool: &SqlitePool, rel: DocumentRelation) -> Result<DocumentRelation>;
    async fn create_retention_policy(&self, pool: &SqlitePool, policy: RetentionPolicy) -> Result<RetentionPolicy>;
}

pub struct SqliteDocumentRepository;

#[async_trait]
impl DocumentRepository for SqliteDocumentRepository {
    async fn create_folder(&self, pool: &SqlitePool, folder: DocumentFolder) -> Result<DocumentFolder> {
        sqlx::query!(
            r#"INSERT INTO document_folders (id, name, parent_id, path, description, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            folder.base.id.to_string(),
            folder.name,
            folder.parent_id.map(|id| id.to_string()),
            folder.path,
            folder.description,
            format!("{:?}", folder.status),
            folder.base.created_at.to_rfc3339(),
            folder.base.updated_at.to_rfc3339(),
            folder.base.created_by.map(|id| id.to_string()),
            folder.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(folder)
    }

    async fn get_folder(&self, pool: &SqlitePool, id: Uuid) -> Result<DocumentFolder> {
        let row = sqlx::query!(
            r#"SELECT id, name, parent_id, path, description, status, created_at, updated_at, created_by, updated_by
               FROM document_folders WHERE id = ?"#,
            id.to_string()
        ).fetch_one(pool).await?;
        
        Ok(DocumentFolder {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            name: row.name,
            parent_id: row.parent_id.and_then(|s| Uuid::parse_str(&s).ok()),
            path: row.path,
            description: row.description,
            status: erp_core::Status::Active,
        })
    }

    async fn list_folders(&self, pool: &SqlitePool, parent_id: Option<Uuid>) -> Result<Vec<DocumentFolder>> {
        let rows = if let Some(pid) = parent_id {
            sqlx::query!(
                r#"SELECT id, name, parent_id, path, description, status, created_at, updated_at, created_by, updated_by
                   FROM document_folders WHERE parent_id = ?"#,
                pid.to_string()
            ).fetch_all(pool).await?
        } else {
            sqlx::query!(
                r#"SELECT id, name, parent_id, path, description, status, created_at, updated_at, created_by, updated_by
                   FROM document_folders WHERE parent_id IS NULL"#
            ).fetch_all(pool).await?
        };
        
        Ok(rows.into_iter().map(|row| DocumentFolder {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            name: row.name,
            parent_id: row.parent_id.and_then(|s| Uuid::parse_str(&s).ok()),
            path: row.path,
            description: row.description,
            status: erp_core::Status::Active,
        }).collect())
    }

    async fn create_document(&self, pool: &SqlitePool, doc: Document) -> Result<Document> {
        sqlx::query!(
            r#"INSERT INTO documents (id, document_number, title, description, document_type, folder_id, status,
               version, revision, access_level, file_name, file_path, file_size, mime_type, checksum,
               author_id, owner_id, checked_out_by, checked_out_at, approved_by, approved_at, published_at,
               expires_at, tags, metadata, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            doc.base.id.to_string(),
            doc.document_number,
            doc.title,
            doc.description,
            format!("{:?}", doc.document_type),
            doc.folder_id.map(|id| id.to_string()),
            format!("{:?}", doc.status),
            doc.version,
            doc.revision,
            format!("{:?}", doc.access_level),
            doc.file_name,
            doc.file_path,
            doc.file_size,
            doc.mime_type,
            doc.checksum,
            doc.author_id.map(|id| id.to_string()),
            doc.owner_id.map(|id| id.to_string()),
            doc.checked_out_by.map(|id| id.to_string()),
            doc.checked_out_at.map(|d| d.to_rfc3339()),
            doc.approved_by.map(|id| id.to_string()),
            doc.approved_at.map(|d| d.to_rfc3339()),
            doc.published_at.map(|d| d.to_rfc3339()),
            doc.expires_at.map(|d| d.to_rfc3339()),
            doc.tags,
            doc.metadata,
            doc.base.created_at.to_rfc3339(),
            doc.base.updated_at.to_rfc3339(),
            doc.base.created_by.map(|id| id.to_string()),
            doc.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(doc)
    }

    async fn get_document(&self, pool: &SqlitePool, id: Uuid) -> Result<Document> {
        let row = sqlx::query!(
            r#"SELECT id, document_number, title, description, document_type, folder_id, status,
               version, revision, access_level, file_name, file_path, file_size, mime_type, checksum,
               author_id, owner_id, checked_out_by, checked_out_at, approved_by, approved_at, published_at,
               expires_at, tags, metadata, created_at, updated_at, created_by, updated_by
               FROM documents WHERE id = ?"#,
            id.to_string()
        ).fetch_one(pool).await?;
        
        Ok(Document {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            document_number: row.document_number,
            title: row.title,
            description: row.description,
            document_type: DocumentType::Other,
            folder_id: row.folder_id.and_then(|s| Uuid::parse_str(&s).ok()),
            status: DocumentStatus::Draft,
            version: row.version,
            revision: row.revision,
            access_level: AccessLevel::Internal,
            file_name: row.file_name,
            file_path: row.file_path,
            file_size: row.file_size,
            mime_type: row.mime_type,
            checksum: row.checksum,
            author_id: row.author_id.and_then(|s| Uuid::parse_str(&s).ok()),
            owner_id: row.owner_id.and_then(|s| Uuid::parse_str(&s).ok()),
            checked_out_by: row.checked_out_by.and_then(|s| Uuid::parse_str(&s).ok()),
            checked_out_at: row.checked_out_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            approved_by: row.approved_by.and_then(|s| Uuid::parse_str(&s).ok()),
            approved_at: row.approved_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            published_at: row.published_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            expires_at: row.expires_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            tags: row.tags,
            metadata: row.metadata,
        })
    }

    async fn update_document(&self, pool: &SqlitePool, doc: Document) -> Result<Document> {
        sqlx::query!(
            r#"UPDATE documents SET title = ?, description = ?, status = ?, version = ?, revision = ?,
               access_level = ?, file_name = ?, file_path = ?, file_size = ?, mime_type = ?, checksum = ?,
               checked_out_by = ?, checked_out_at = ?, approved_by = ?, approved_at = ?, published_at = ?,
               expires_at = ?, tags = ?, metadata = ?, updated_at = ?, updated_by = ?
               WHERE id = ?"#,
            doc.title,
            doc.description,
            format!("{:?}", doc.status),
            doc.version,
            doc.revision,
            format!("{:?}", doc.access_level),
            doc.file_name,
            doc.file_path,
            doc.file_size,
            doc.mime_type,
            doc.checksum,
            doc.checked_out_by.map(|id| id.to_string()),
            doc.checked_out_at.map(|d| d.to_rfc3339()),
            doc.approved_by.map(|id| id.to_string()),
            doc.approved_at.map(|d| d.to_rfc3339()),
            doc.published_at.map(|d| d.to_rfc3339()),
            doc.expires_at.map(|d| d.to_rfc3339()),
            doc.tags,
            doc.metadata,
            doc.base.updated_at.to_rfc3339(),
            doc.base.updated_by.map(|id| id.to_string()),
            doc.base.id.to_string(),
        ).execute(pool).await?;
        Ok(doc)
    }

    async fn delete_document(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM documents WHERE id = ?", id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn list_documents(&self, pool: &SqlitePool, folder_id: Option<Uuid>, _status: Option<DocumentStatus>) -> Result<Vec<Document>> {
        let rows = if let Some(fid) = folder_id {
            sqlx::query!(
                r#"SELECT id, document_number, title, description, document_type, folder_id, status,
                   version, revision, access_level, file_name, file_path, file_size, mime_type, checksum,
                   author_id, owner_id, checked_out_by, checked_out_at, approved_by, approved_at, published_at,
                   expires_at, tags, metadata, created_at, updated_at, created_by, updated_by
                   FROM documents WHERE folder_id = ?"#,
                fid.to_string()
            ).fetch_all(pool).await?
        } else {
            sqlx::query!(
                r#"SELECT id, document_number, title, description, document_type, folder_id, status,
                   version, revision, access_level, file_name, file_path, file_size, mime_type, checksum,
                   author_id, owner_id, checked_out_by, checked_out_at, approved_by, approved_at, published_at,
                   expires_at, tags, metadata, created_at, updated_at, created_by, updated_by
                   FROM documents"#
            ).fetch_all(pool).await?
        };
        
        Ok(rows.into_iter().map(|row| Document {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            document_number: row.document_number,
            title: row.title,
            description: row.description,
            document_type: DocumentType::Other,
            folder_id: row.folder_id.and_then(|s| Uuid::parse_str(&s).ok()),
            status: DocumentStatus::Draft,
            version: row.version,
            revision: row.revision,
            access_level: AccessLevel::Internal,
            file_name: row.file_name,
            file_path: row.file_path,
            file_size: row.file_size,
            mime_type: row.mime_type,
            checksum: row.checksum,
            author_id: row.author_id.and_then(|s| Uuid::parse_str(&s).ok()),
            owner_id: row.owner_id.and_then(|s| Uuid::parse_str(&s).ok()),
            checked_out_by: row.checked_out_by.and_then(|s| Uuid::parse_str(&s).ok()),
            checked_out_at: row.checked_out_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            approved_by: row.approved_by.and_then(|s| Uuid::parse_str(&s).ok()),
            approved_at: row.approved_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            published_at: row.published_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            expires_at: row.expires_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            tags: row.tags,
            metadata: row.metadata,
        }).collect())
    }

    async fn create_version(&self, pool: &SqlitePool, version: DocumentVersion) -> Result<DocumentVersion> {
        sqlx::query!(
            r#"INSERT INTO document_versions (id, document_id, version, revision, file_path, file_size,
               checksum, change_summary, changed_by, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            version.base.id.to_string(),
            version.document_id.to_string(),
            version.version,
            version.revision,
            version.file_path,
            version.file_size,
            version.checksum,
            version.change_summary,
            version.changed_by.map(|id| id.to_string()),
            format!("{:?}", version.status),
            version.base.created_at.to_rfc3339(),
            version.base.updated_at.to_rfc3339(),
            version.base.created_by.map(|id| id.to_string()),
            version.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(version)
    }

    async fn list_versions(&self, pool: &SqlitePool, document_id: Uuid) -> Result<Vec<DocumentVersion>> {
        let rows = sqlx::query!(
            r#"SELECT id, document_id, version, revision, file_path, file_size, checksum,
               change_summary, changed_by, status, created_at, updated_at, created_by, updated_by
               FROM document_versions WHERE document_id = ? ORDER BY version DESC"#,
            document_id.to_string()
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| DocumentVersion {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            document_id: Uuid::parse_str(&row.document_id).unwrap(),
            version: row.version,
            revision: row.revision,
            file_path: row.file_path,
            file_size: row.file_size,
            checksum: row.checksum,
            change_summary: row.change_summary,
            changed_by: row.changed_by.and_then(|s| Uuid::parse_str(&s).ok()),
            status: DocumentStatus::Draft,
        }).collect())
    }

    async fn create_checkout(&self, pool: &SqlitePool, checkout: DocumentCheckout) -> Result<DocumentCheckout> {
        sqlx::query!(
            r#"INSERT INTO document_checkouts (id, document_id, user_id, checkout_at, expected_return,
               checkin_at, notes, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            checkout.base.id.to_string(),
            checkout.document_id.to_string(),
            checkout.user_id.to_string(),
            checkout.checkout_at.to_rfc3339(),
            checkout.expected_return.map(|d| d.to_rfc3339()),
            checkout.checkin_at.map(|d| d.to_rfc3339()),
            checkout.notes,
            format!("{:?}", checkout.status),
            checkout.base.created_at.to_rfc3339(),
            checkout.base.updated_at.to_rfc3339(),
            checkout.base.created_by.map(|id| id.to_string()),
            checkout.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(checkout)
    }

    async fn checkin(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query!(
            r#"UPDATE document_checkouts SET checkin_at = ?, status = 'Completed' WHERE id = ?"#,
            now, id.to_string()
        ).execute(pool).await?;
        Ok(())
    }

    async fn create_review(&self, pool: &SqlitePool, review: DocumentReview) -> Result<DocumentReview> {
        sqlx::query!(
            r#"INSERT INTO document_reviews (id, document_id, version, reviewer_id, requested_at,
               reviewed_at, status, comments, approved, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            review.base.id.to_string(),
            review.document_id.to_string(),
            review.version,
            review.reviewer_id.to_string(),
            review.requested_at.to_rfc3339(),
            review.reviewed_at.map(|d| d.to_rfc3339()),
            format!("{:?}", review.status),
            review.comments,
            review.approved.map(|b| if b { 1 } else { 0 }),
            review.base.created_at.to_rfc3339(),
            review.base.updated_at.to_rfc3339(),
            review.base.created_by.map(|id| id.to_string()),
            review.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(review)
    }

    async fn create_permission(&self, pool: &SqlitePool, perm: DocumentPermission) -> Result<DocumentPermission> {
        sqlx::query!(
            r#"INSERT INTO document_permissions (id, document_id, folder_id, user_id, role_id,
               can_read, can_write, can_delete, can_share, can_approve, can_checkout,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            perm.base.id.to_string(),
            perm.document_id.map(|id| id.to_string()),
            perm.folder_id.map(|id| id.to_string()),
            perm.user_id.map(|id| id.to_string()),
            perm.role_id.map(|id| id.to_string()),
            perm.can_read as i32,
            perm.can_write as i32,
            perm.can_delete as i32,
            perm.can_share as i32,
            perm.can_approve as i32,
            perm.can_checkout as i32,
            perm.base.created_at.to_rfc3339(),
            perm.base.updated_at.to_rfc3339(),
            perm.base.created_by.map(|id| id.to_string()),
            perm.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(perm)
    }

    async fn create_relation(&self, pool: &SqlitePool, rel: DocumentRelation) -> Result<DocumentRelation> {
        sqlx::query!(
            r#"INSERT INTO document_relations (id, source_document_id, target_document_id, relation_type,
               description, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            rel.base.id.to_string(),
            rel.source_document_id.to_string(),
            rel.target_document_id.to_string(),
            format!("{:?}", rel.relation_type),
            rel.description,
            rel.base.created_at.to_rfc3339(),
            rel.base.updated_at.to_rfc3339(),
            rel.base.created_by.map(|id| id.to_string()),
            rel.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(rel)
    }

    async fn create_retention_policy(&self, pool: &SqlitePool, policy: RetentionPolicy) -> Result<RetentionPolicy> {
        sqlx::query!(
            r#"INSERT INTO retention_policies (id, name, description, document_types, retention_years,
               review_after_years, disposition, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            policy.base.id.to_string(),
            policy.name,
            policy.description,
            policy.document_types,
            policy.retention_years,
            policy.review_after_years,
            format!("{:?}", policy.disposition),
            format!("{:?}", policy.status),
            policy.base.created_at.to_rfc3339(),
            policy.base.updated_at.to_rfc3339(),
            policy.base.created_by.map(|id| id.to_string()),
            policy.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(policy)
    }
}
