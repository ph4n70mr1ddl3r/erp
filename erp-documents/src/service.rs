use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::{Result, BaseEntity, Status, Pagination, Paginated};
use crate::models::*;
use crate::repository::*;

pub struct DocumentService {
    repo: SqliteDocumentRepository,
}

impl DocumentService {
    pub fn new() -> Self {
        Self { repo: SqliteDocumentRepository }
    }

    pub async fn create_folder(&self, pool: &SqlitePool, name: String, parent_id: Option<Uuid>, description: Option<String>) -> Result<DocumentFolder> {
        let path = if let Some(pid) = parent_id {
            let parent = self.repo.get_folder(pool, pid).await?;
            format!("{}/{}", parent.path, name)
        } else {
            format!("/{}", name)
        };
        
        let folder = DocumentFolder {
            base: BaseEntity::new(),
            name,
            parent_id,
            path,
            description,
            status: Status::Active,
        };
        
        self.repo.create_folder(pool, folder).await
    }

    pub async fn list_folders(&self, pool: &SqlitePool, parent_id: Option<Uuid>) -> Result<Vec<DocumentFolder>> {
        self.repo.list_folders(pool, parent_id).await
    }

    pub async fn get_folder(&self, pool: &SqlitePool, id: Uuid) -> Result<DocumentFolder> {
        self.repo.get_folder(pool, id).await
    }

    pub async fn create_document(&self, pool: &SqlitePool, doc: Document) -> Result<Document> {
        self.repo.create_document(pool, doc).await
    }

    pub async fn get_document(&self, pool: &SqlitePool, id: Uuid) -> Result<Document> {
        self.repo.get_document(pool, id).await
    }

    pub async fn update_document(&self, pool: &SqlitePool, doc: Document) -> Result<Document> {
        self.repo.update_document(pool, doc).await
    }

    pub async fn delete_document(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete_document(pool, id).await
    }

    pub async fn list_documents(&self, pool: &SqlitePool, folder_id: Option<Uuid>, status: Option<DocumentStatus>) -> Result<Vec<Document>> {
        self.repo.list_documents(pool, folder_id, status).await
    }

    pub async fn checkout(&self, pool: &SqlitePool, document_id: Uuid, user_id: Uuid) -> Result<DocumentCheckout> {
        let checkout = DocumentCheckout {
            base: BaseEntity::new(),
            document_id,
            user_id,
            checkout_at: chrono::Utc::now(),
            expected_return: None,
            checkin_at: None,
            notes: None,
            status: Status::Active,
        };
        self.repo.create_checkout(pool, checkout).await
    }

    pub async fn checkin(&self, pool: &SqlitePool, checkout_id: Uuid) -> Result<()> {
        self.repo.checkin(pool, checkout_id).await
    }

    pub async fn create_version(&self, pool: &SqlitePool, version: DocumentVersion) -> Result<DocumentVersion> {
        self.repo.create_version(pool, version).await
    }

    pub async fn list_versions(&self, pool: &SqlitePool, document_id: Uuid) -> Result<Vec<DocumentVersion>> {
        self.repo.list_versions(pool, document_id).await
    }

    pub async fn request_review(&self, pool: &SqlitePool, document_id: Uuid, version: i32, reviewer_id: Uuid) -> Result<DocumentReview> {
        let review = DocumentReview {
            base: BaseEntity::new(),
            document_id,
            version,
            reviewer_id,
            requested_at: chrono::Utc::now(),
            reviewed_at: None,
            status: ReviewStatus::Pending,
            comments: None,
            approved: None,
        };
        self.repo.create_review(pool, review).await
    }

    pub async fn set_permission(&self, pool: &SqlitePool, perm: DocumentPermission) -> Result<DocumentPermission> {
        self.repo.create_permission(pool, perm).await
    }

    pub async fn create_relation(&self, pool: &SqlitePool, source_id: Uuid, target_id: Uuid, relation_type: DocumentRelationType, description: Option<String>) -> Result<DocumentRelation> {
        let rel = DocumentRelation {
            base: BaseEntity::new(),
            source_document_id: source_id,
            target_document_id: target_id,
            relation_type,
            description,
        };
        self.repo.create_relation(pool, rel).await
    }

    pub async fn create_retention_policy(&self, pool: &SqlitePool, name: String, retention_years: i32, disposition: DispositionType) -> Result<RetentionPolicy> {
        let policy = RetentionPolicy {
            base: BaseEntity::new(),
            name,
            description: None,
            document_types: None,
            retention_years,
            review_after_years: None,
            disposition,
            status: Status::Active,
        };
        self.repo.create_retention_policy(pool, policy).await
    }
}
