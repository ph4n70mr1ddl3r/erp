use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
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
        sqlx::query(
            r#"INSERT INTO document_folders (id, name, parent_id, path, description, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(folder.base.id.to_string())
        .bind(&folder.name)
        .bind(folder.parent_id.map(|id| id.to_string()))
        .bind(&folder.path)
        .bind(&folder.description)
        .bind(format!("{:?}", folder.status))
        .bind(folder.base.created_at.to_rfc3339())
        .bind(folder.base.updated_at.to_rfc3339())
        .bind(folder.base.created_by.map(|id| id.to_string()))
        .bind(folder.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(folder)
    }

    async fn get_folder(&self, pool: &SqlitePool, id: Uuid) -> Result<DocumentFolder> {
        let row = sqlx::query(
            r#"SELECT id, name, parent_id, path, description, status, created_at, updated_at, created_by, updated_by
               FROM document_folders WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_one(pool).await?;
        
        Ok(DocumentFolder {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.try_get::<&str, _>("id")?).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.try_get::<&str, _>("created_at")?).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.try_get::<&str, _>("updated_at")?).unwrap().with_timezone(&chrono::Utc),
                created_by: row.try_get::<Option<&str>, _>("created_by")?.and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.try_get::<Option<&str>, _>("updated_by")?.and_then(|s| Uuid::parse_str(s).ok()),
            },
            name: row.try_get::<&str, _>("name")?.to_string(),
            parent_id: row.try_get::<Option<&str>, _>("parent_id")?.and_then(|s| Uuid::parse_str(s).ok()),
            path: row.try_get::<&str, _>("path")?.to_string(),
            description: row.try_get::<Option<&str>, _>("description")?.map(|s| s.to_string()),
            status: erp_core::Status::Active,
        })
    }

    async fn list_folders(&self, pool: &SqlitePool, parent_id: Option<Uuid>) -> Result<Vec<DocumentFolder>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = if let Some(pid) = parent_id {
            sqlx::query(
                r#"SELECT id, name, parent_id, path, description, status, created_at, updated_at, created_by, updated_by
                   FROM document_folders WHERE parent_id = ?"#
            )
            .bind(pid.to_string())
            .fetch_all(pool).await?
        } else {
            sqlx::query(
                r#"SELECT id, name, parent_id, path, description, status, created_at, updated_at, created_by, updated_by
                   FROM document_folders WHERE parent_id IS NULL"#
            )
            .fetch_all(pool).await?
        };
        
        let mut folders = Vec::new();
        for row in rows {
            folders.push(DocumentFolder {
                base: erp_core::BaseEntity {
                    id: Uuid::parse_str(row.try_get::<&str, _>("id")?).unwrap(),
                    created_at: chrono::DateTime::parse_from_rfc3339(row.try_get::<&str, _>("created_at")?).unwrap().with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(row.try_get::<&str, _>("updated_at")?).unwrap().with_timezone(&chrono::Utc),
                    created_by: row.try_get::<Option<&str>, _>("created_by")?.and_then(|s| Uuid::parse_str(s).ok()),
                    updated_by: row.try_get::<Option<&str>, _>("updated_by")?.and_then(|s| Uuid::parse_str(s).ok()),
                },
                name: row.try_get::<&str, _>("name")?.to_string(),
                parent_id: row.try_get::<Option<&str>, _>("parent_id")?.and_then(|s| Uuid::parse_str(s).ok()),
                path: row.try_get::<&str, _>("path")?.to_string(),
                description: row.try_get::<Option<&str>, _>("description")?.map(|s| s.to_string()),
                status: erp_core::Status::Active,
            });
        }
        Ok(folders)
    }

    async fn create_document(&self, pool: &SqlitePool, doc: Document) -> Result<Document> {
        sqlx::query(
            r#"INSERT INTO documents (id, document_number, title, description, document_type, folder_id, status,
               version, revision, access_level, file_name, file_path, file_size, mime_type, checksum,
               author_id, owner_id, checked_out_by, checked_out_at, approved_by, approved_at, published_at,
               expires_at, tags, metadata, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(doc.base.id.to_string())
        .bind(&doc.document_number)
        .bind(&doc.title)
        .bind(&doc.description)
        .bind(format!("{:?}", doc.document_type))
        .bind(doc.folder_id.map(|id| id.to_string()))
        .bind(format!("{:?}", doc.status))
        .bind(doc.version)
        .bind(&doc.revision)
        .bind(format!("{:?}", doc.access_level))
        .bind(&doc.file_name)
        .bind(&doc.file_path)
        .bind(doc.file_size)
        .bind(&doc.mime_type)
        .bind(&doc.checksum)
        .bind(doc.author_id.map(|id| id.to_string()))
        .bind(doc.owner_id.map(|id| id.to_string()))
        .bind(doc.checked_out_by.map(|id| id.to_string()))
        .bind(doc.checked_out_at.map(|d| d.to_rfc3339()))
        .bind(doc.approved_by.map(|id| id.to_string()))
        .bind(doc.approved_at.map(|d| d.to_rfc3339()))
        .bind(doc.published_at.map(|d| d.to_rfc3339()))
        .bind(doc.expires_at.map(|d| d.to_rfc3339()))
        .bind(&doc.tags)
        .bind(&doc.metadata)
        .bind(doc.base.created_at.to_rfc3339())
        .bind(doc.base.updated_at.to_rfc3339())
        .bind(doc.base.created_by.map(|id| id.to_string()))
        .bind(doc.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(doc)
    }

    async fn get_document(&self, pool: &SqlitePool, id: Uuid) -> Result<Document> {
        let row = sqlx::query(
            r#"SELECT id, document_number, title, description, document_type, folder_id, status,
               version, revision, access_level, file_name, file_path, file_size, mime_type, checksum,
               author_id, owner_id, checked_out_by, checked_out_at, approved_by, approved_at, published_at,
               expires_at, tags, metadata, created_at, updated_at, created_by, updated_by
               FROM documents WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_one(pool).await?;
        
        Ok(Document {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.try_get::<&str, _>("id")?).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.try_get::<&str, _>("created_at")?).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.try_get::<&str, _>("updated_at")?).unwrap().with_timezone(&chrono::Utc),
                created_by: row.try_get::<Option<&str>, _>("created_by")?.and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.try_get::<Option<&str>, _>("updated_by")?.and_then(|s| Uuid::parse_str(s).ok()),
            },
            document_number: row.try_get::<&str, _>("document_number")?.to_string(),
            title: row.try_get::<&str, _>("title")?.to_string(),
            description: row.try_get::<Option<&str>, _>("description")?.map(|s| s.to_string()),
            document_type: DocumentType::Other,
            folder_id: row.try_get::<Option<&str>, _>("folder_id")?.and_then(|s| Uuid::parse_str(s).ok()),
            status: DocumentStatus::Draft,
            version: row.try_get::<i32, _>("version")?,
            revision: row.try_get::<&str, _>("revision")?.to_string(),
            access_level: AccessLevel::Internal,
            file_name: row.try_get::<&str, _>("file_name")?.to_string(),
            file_path: row.try_get::<&str, _>("file_path")?.to_string(),
            file_size: row.try_get::<i64, _>("file_size")?,
            mime_type: row.try_get::<&str, _>("mime_type")?.to_string(),
            checksum: row.try_get::<&str, _>("checksum")?.to_string(),
            author_id: row.try_get::<Option<&str>, _>("author_id")?.and_then(|s| Uuid::parse_str(s).ok()),
            owner_id: row.try_get::<Option<&str>, _>("owner_id")?.and_then(|s| Uuid::parse_str(s).ok()),
            checked_out_by: row.try_get::<Option<&str>, _>("checked_out_by")?.and_then(|s| Uuid::parse_str(s).ok()),
            checked_out_at: row.try_get::<Option<&str>, _>("checked_out_at")?.and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            approved_by: row.try_get::<Option<&str>, _>("approved_by")?.and_then(|s| Uuid::parse_str(s).ok()),
            approved_at: row.try_get::<Option<&str>, _>("approved_at")?.and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            published_at: row.try_get::<Option<&str>, _>("published_at")?.and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            expires_at: row.try_get::<Option<&str>, _>("expires_at")?.and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            tags: row.try_get::<Option<&str>, _>("tags")?.map(|s| s.to_string()),
            metadata: row.try_get::<Option<&str>, _>("metadata")?.map(|s| s.to_string()),
        })
    }

    async fn update_document(&self, pool: &SqlitePool, doc: Document) -> Result<Document> {
        sqlx::query(
            r#"UPDATE documents SET title = ?, description = ?, status = ?, version = ?, revision = ?,
               access_level = ?, file_name = ?, file_path = ?, file_size = ?, mime_type = ?, checksum = ?,
               checked_out_by = ?, checked_out_at = ?, approved_by = ?, approved_at = ?, published_at = ?,
               expires_at = ?, tags = ?, metadata = ?, updated_at = ?, updated_by = ?
               WHERE id = ?"#
        )
        .bind(&doc.title)
        .bind(&doc.description)
        .bind(format!("{:?}", doc.status))
        .bind(doc.version)
        .bind(&doc.revision)
        .bind(format!("{:?}", doc.access_level))
        .bind(&doc.file_name)
        .bind(&doc.file_path)
        .bind(doc.file_size)
        .bind(&doc.mime_type)
        .bind(&doc.checksum)
        .bind(doc.checked_out_by.map(|id| id.to_string()))
        .bind(doc.checked_out_at.map(|d| d.to_rfc3339()))
        .bind(doc.approved_by.map(|id| id.to_string()))
        .bind(doc.approved_at.map(|d| d.to_rfc3339()))
        .bind(doc.published_at.map(|d| d.to_rfc3339()))
        .bind(doc.expires_at.map(|d| d.to_rfc3339()))
        .bind(&doc.tags)
        .bind(&doc.metadata)
        .bind(doc.base.updated_at.to_rfc3339())
        .bind(doc.base.updated_by.map(|id| id.to_string()))
        .bind(doc.base.id.to_string())
        .execute(pool).await?;
        Ok(doc)
    }

    async fn delete_document(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM documents WHERE id = ?")
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn list_documents(&self, pool: &SqlitePool, folder_id: Option<Uuid>, _status: Option<DocumentStatus>) -> Result<Vec<Document>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = if let Some(fid) = folder_id {
            sqlx::query(
                r#"SELECT id, document_number, title, description, document_type, folder_id, status,
                   version, revision, access_level, file_name, file_path, file_size, mime_type, checksum,
                   author_id, owner_id, checked_out_by, checked_out_at, approved_by, approved_at, published_at,
                   expires_at, tags, metadata, created_at, updated_at, created_by, updated_by
                   FROM documents WHERE folder_id = ?"#
            )
            .bind(fid.to_string())
            .fetch_all(pool).await?
        } else {
            sqlx::query(
                r#"SELECT id, document_number, title, description, document_type, folder_id, status,
                   version, revision, access_level, file_name, file_path, file_size, mime_type, checksum,
                   author_id, owner_id, checked_out_by, checked_out_at, approved_by, approved_at, published_at,
                   expires_at, tags, metadata, created_at, updated_at, created_by, updated_by
                   FROM documents"#
            )
            .fetch_all(pool).await?
        };
        
        let mut documents = Vec::new();
        for row in rows {
            documents.push(Document {
                base: erp_core::BaseEntity {
                    id: Uuid::parse_str(row.try_get::<&str, _>("id")?).unwrap(),
                    created_at: chrono::DateTime::parse_from_rfc3339(row.try_get::<&str, _>("created_at")?).unwrap().with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(row.try_get::<&str, _>("updated_at")?).unwrap().with_timezone(&chrono::Utc),
                    created_by: row.try_get::<Option<&str>, _>("created_by")?.and_then(|s| Uuid::parse_str(s).ok()),
                    updated_by: row.try_get::<Option<&str>, _>("updated_by")?.and_then(|s| Uuid::parse_str(s).ok()),
                },
                document_number: row.try_get::<&str, _>("document_number")?.to_string(),
                title: row.try_get::<&str, _>("title")?.to_string(),
                description: row.try_get::<Option<&str>, _>("description")?.map(|s| s.to_string()),
                document_type: DocumentType::Other,
                folder_id: row.try_get::<Option<&str>, _>("folder_id")?.and_then(|s| Uuid::parse_str(s).ok()),
                status: DocumentStatus::Draft,
                version: row.try_get::<i32, _>("version")?,
                revision: row.try_get::<&str, _>("revision")?.to_string(),
                access_level: AccessLevel::Internal,
                file_name: row.try_get::<&str, _>("file_name")?.to_string(),
                file_path: row.try_get::<&str, _>("file_path")?.to_string(),
                file_size: row.try_get::<i64, _>("file_size")?,
                mime_type: row.try_get::<&str, _>("mime_type")?.to_string(),
                checksum: row.try_get::<&str, _>("checksum")?.to_string(),
                author_id: row.try_get::<Option<&str>, _>("author_id")?.and_then(|s| Uuid::parse_str(s).ok()),
                owner_id: row.try_get::<Option<&str>, _>("owner_id")?.and_then(|s| Uuid::parse_str(s).ok()),
                checked_out_by: row.try_get::<Option<&str>, _>("checked_out_by")?.and_then(|s| Uuid::parse_str(s).ok()),
                checked_out_at: row.try_get::<Option<&str>, _>("checked_out_at")?.and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
                approved_by: row.try_get::<Option<&str>, _>("approved_by")?.and_then(|s| Uuid::parse_str(s).ok()),
                approved_at: row.try_get::<Option<&str>, _>("approved_at")?.and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
                published_at: row.try_get::<Option<&str>, _>("published_at")?.and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
                expires_at: row.try_get::<Option<&str>, _>("expires_at")?.and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
                tags: row.try_get::<Option<&str>, _>("tags")?.map(|s| s.to_string()),
                metadata: row.try_get::<Option<&str>, _>("metadata")?.map(|s| s.to_string()),
            });
        }
        Ok(documents)
    }

    async fn create_version(&self, pool: &SqlitePool, version: DocumentVersion) -> Result<DocumentVersion> {
        sqlx::query(
            r#"INSERT INTO document_versions (id, document_id, version, revision, file_path, file_size,
               checksum, change_summary, changed_by, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(version.base.id.to_string())
        .bind(version.document_id.to_string())
        .bind(version.version)
        .bind(&version.revision)
        .bind(&version.file_path)
        .bind(version.file_size)
        .bind(&version.checksum)
        .bind(&version.change_summary)
        .bind(version.changed_by.map(|id| id.to_string()))
        .bind(format!("{:?}", version.status))
        .bind(version.base.created_at.to_rfc3339())
        .bind(version.base.updated_at.to_rfc3339())
        .bind(version.base.created_by.map(|id| id.to_string()))
        .bind(version.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(version)
    }

    async fn list_versions(&self, pool: &SqlitePool, document_id: Uuid) -> Result<Vec<DocumentVersion>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, document_id, version, revision, file_path, file_size, checksum,
               change_summary, changed_by, status, created_at, updated_at, created_by, updated_by
               FROM document_versions WHERE document_id = ? ORDER BY version DESC"#
        )
        .bind(document_id.to_string())
        .fetch_all(pool).await?;
        
        let mut versions = Vec::new();
        for row in rows {
            versions.push(DocumentVersion {
                base: erp_core::BaseEntity {
                    id: Uuid::parse_str(row.try_get::<&str, _>("id")?).unwrap(),
                    created_at: chrono::DateTime::parse_from_rfc3339(row.try_get::<&str, _>("created_at")?).unwrap().with_timezone(&chrono::Utc),
                    updated_at: chrono::DateTime::parse_from_rfc3339(row.try_get::<&str, _>("updated_at")?).unwrap().with_timezone(&chrono::Utc),
                    created_by: row.try_get::<Option<&str>, _>("created_by")?.and_then(|s| Uuid::parse_str(s).ok()),
                    updated_by: row.try_get::<Option<&str>, _>("updated_by")?.and_then(|s| Uuid::parse_str(s).ok()),
                },
                document_id: Uuid::parse_str(row.try_get::<&str, _>("document_id")?).unwrap(),
                version: row.try_get::<i32, _>("version")?,
                revision: row.try_get::<&str, _>("revision")?.to_string(),
                file_path: row.try_get::<&str, _>("file_path")?.to_string(),
                file_size: row.try_get::<i64, _>("file_size")?,
                checksum: row.try_get::<&str, _>("checksum")?.to_string(),
                change_summary: row.try_get::<Option<&str>, _>("change_summary")?.map(|s| s.to_string()),
                changed_by: row.try_get::<Option<&str>, _>("changed_by")?.and_then(|s| Uuid::parse_str(s).ok()),
                status: DocumentStatus::Draft,
            });
        }
        Ok(versions)
    }

    async fn create_checkout(&self, pool: &SqlitePool, checkout: DocumentCheckout) -> Result<DocumentCheckout> {
        sqlx::query(
            r#"INSERT INTO document_checkouts (id, document_id, user_id, checkout_at, expected_return,
               checkin_at, notes, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(checkout.base.id.to_string())
        .bind(checkout.document_id.to_string())
        .bind(checkout.user_id.to_string())
        .bind(checkout.checkout_at.to_rfc3339())
        .bind(checkout.expected_return.map(|d| d.to_rfc3339()))
        .bind(checkout.checkin_at.map(|d| d.to_rfc3339()))
        .bind(&checkout.notes)
        .bind(format!("{:?}", checkout.status))
        .bind(checkout.base.created_at.to_rfc3339())
        .bind(checkout.base.updated_at.to_rfc3339())
        .bind(checkout.base.created_by.map(|id| id.to_string()))
        .bind(checkout.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(checkout)
    }

    async fn checkin(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339();
        sqlx::query(
            r#"UPDATE document_checkouts SET checkin_at = ?, status = 'Completed' WHERE id = ?"#
        )
        .bind(now)
        .bind(id.to_string())
        .execute(pool).await?;
        Ok(())
    }

    async fn create_review(&self, pool: &SqlitePool, review: DocumentReview) -> Result<DocumentReview> {
        sqlx::query(
            r#"INSERT INTO document_reviews (id, document_id, version, reviewer_id, requested_at,
               reviewed_at, status, comments, approved, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(review.base.id.to_string())
        .bind(review.document_id.to_string())
        .bind(review.version)
        .bind(review.reviewer_id.to_string())
        .bind(review.requested_at.to_rfc3339())
        .bind(review.reviewed_at.map(|d| d.to_rfc3339()))
        .bind(format!("{:?}", review.status))
        .bind(&review.comments)
        .bind(review.approved.map(|b| if b { 1 } else { 0 }))
        .bind(review.base.created_at.to_rfc3339())
        .bind(review.base.updated_at.to_rfc3339())
        .bind(review.base.created_by.map(|id| id.to_string()))
        .bind(review.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(review)
    }

    async fn create_permission(&self, pool: &SqlitePool, perm: DocumentPermission) -> Result<DocumentPermission> {
        sqlx::query(
            r#"INSERT INTO document_permissions (id, document_id, folder_id, user_id, role_id,
               can_read, can_write, can_delete, can_share, can_approve, can_checkout,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(perm.base.id.to_string())
        .bind(perm.document_id.map(|id| id.to_string()))
        .bind(perm.folder_id.map(|id| id.to_string()))
        .bind(perm.user_id.map(|id| id.to_string()))
        .bind(perm.role_id.map(|id| id.to_string()))
        .bind(perm.can_read as i32)
        .bind(perm.can_write as i32)
        .bind(perm.can_delete as i32)
        .bind(perm.can_share as i32)
        .bind(perm.can_approve as i32)
        .bind(perm.can_checkout as i32)
        .bind(perm.base.created_at.to_rfc3339())
        .bind(perm.base.updated_at.to_rfc3339())
        .bind(perm.base.created_by.map(|id| id.to_string()))
        .bind(perm.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(perm)
    }

    async fn create_relation(&self, pool: &SqlitePool, rel: DocumentRelation) -> Result<DocumentRelation> {
        sqlx::query(
            r#"INSERT INTO document_relations (id, source_document_id, target_document_id, relation_type,
               description, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(rel.base.id.to_string())
        .bind(rel.source_document_id.to_string())
        .bind(rel.target_document_id.to_string())
        .bind(format!("{:?}", rel.relation_type))
        .bind(&rel.description)
        .bind(rel.base.created_at.to_rfc3339())
        .bind(rel.base.updated_at.to_rfc3339())
        .bind(rel.base.created_by.map(|id| id.to_string()))
        .bind(rel.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(rel)
    }

    async fn create_retention_policy(&self, pool: &SqlitePool, policy: RetentionPolicy) -> Result<RetentionPolicy> {
        sqlx::query(
            r#"INSERT INTO retention_policies (id, name, description, document_types, retention_years,
               review_after_years, disposition, status, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(policy.base.id.to_string())
        .bind(&policy.name)
        .bind(&policy.description)
        .bind(&policy.document_types)
        .bind(policy.retention_years)
        .bind(policy.review_after_years)
        .bind(format!("{:?}", policy.disposition))
        .bind(format!("{:?}", policy.status))
        .bind(policy.base.created_at.to_rfc3339())
        .bind(policy.base.updated_at.to_rfc3339())
        .bind(policy.base.created_by.map(|id| id.to_string()))
        .bind(policy.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(policy)
    }
}
