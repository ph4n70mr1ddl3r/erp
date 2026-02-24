use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{BaseEntity, Status, Pagination};
use erp_documents::{
    DocumentService, Document, DocumentFolder, DocumentVersion, DocumentCheckout,
    DocumentReview, DocumentPermission, DocumentRelation, RetentionPolicy,
    DocumentType, DocumentStatus, AccessLevel, ReviewStatus, DispositionType,
};

#[derive(Deserialize)]
pub struct CreateFolderRequest {
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub description: Option<String>,
}

#[derive(Serialize)]
pub struct FolderResponse {
    pub id: Uuid,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub path: String,
    pub status: String,
}

pub async fn list_folders(
    State(state): State<AppState>,
    Query(query): Query<FolderQuery>,
) -> ApiResult<Json<Vec<FolderResponse>>> {
    let service = DocumentService::new();
    let folders = service.list_folders(&state.pool, query.parent_id).await?;
    Ok(Json(folders.into_iter().map(|f| FolderResponse {
        id: f.base.id,
        name: f.name,
        parent_id: f.parent_id,
        path: f.path,
        status: format!("{:?}", f.status),
    }).collect()))
}

#[derive(Deserialize)]
pub struct FolderQuery {
    pub parent_id: Option<Uuid>,
}

pub async fn create_folder(
    State(state): State<AppState>,
    Json(req): Json<CreateFolderRequest>,
) -> ApiResult<Json<FolderResponse>> {
    let service = DocumentService::new();
    let folder = service.create_folder(&state.pool, req.name, req.parent_id, req.description).await?;
    Ok(Json(FolderResponse {
        id: folder.base.id,
        name: folder.name,
        parent_id: folder.parent_id,
        path: folder.path,
        status: format!("{:?}", folder.status),
    }))
}

#[derive(Serialize)]
pub struct DocumentResponse {
    pub id: Uuid,
    pub document_number: String,
    pub title: String,
    pub status: String,
    pub version: i32,
    pub file_name: String,
}

pub async fn list_documents(
    State(state): State<AppState>,
    Query(query): Query<DocumentQuery>,
) -> ApiResult<Json<Vec<DocumentResponse>>> {
    let service = DocumentService::new();
    let docs = service.list_documents(&state.pool, query.folder_id, None).await?;
    Ok(Json(docs.into_iter().map(|d| DocumentResponse {
        id: d.base.id,
        document_number: d.document_number,
        title: d.title,
        status: format!("{:?}", d.status),
        version: d.version,
        file_name: d.file_name,
    }).collect()))
}

#[derive(Deserialize)]
pub struct DocumentQuery {
    pub folder_id: Option<Uuid>,
}

pub async fn get_document(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<DocumentResponse>> {
    let service = DocumentService::new();
    let doc = service.get_document(&state.pool, id).await?;
    Ok(Json(DocumentResponse {
        id: doc.base.id,
        document_number: doc.document_number,
        title: doc.title,
        status: format!("{:?}", doc.status),
        version: doc.version,
        file_name: doc.file_name,
    }))
}

#[derive(Deserialize)]
pub struct CreateDocumentRequest {
    pub title: String,
    pub description: Option<String>,
    pub folder_id: Option<Uuid>,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub checksum: String,
}

pub async fn create_document(
    State(state): State<AppState>,
    Json(req): Json<CreateDocumentRequest>,
) -> ApiResult<Json<DocumentResponse>> {
    let service = DocumentService::new();
    let doc = Document {
        base: BaseEntity::new(),
        document_number: format!("DOC-{}", chrono::Utc::now().format("%Y%m%d%H%M%S")),
        title: req.title,
        description: req.description,
        document_type: DocumentType::Other,
        folder_id: req.folder_id,
        status: DocumentStatus::Draft,
        version: 1,
        revision: "A".to_string(),
        access_level: AccessLevel::Internal,
        file_name: req.file_name,
        file_path: req.file_path,
        file_size: req.file_size,
        mime_type: req.mime_type,
        checksum: req.checksum,
        author_id: None,
        owner_id: None,
        checked_out_by: None,
        checked_out_at: None,
        approved_by: None,
        approved_at: None,
        published_at: None,
        expires_at: None,
        tags: None,
        metadata: None,
    };
    let created = service.create_document(&state.pool, doc).await?;
    Ok(Json(DocumentResponse {
        id: created.base.id,
        document_number: created.document_number,
        title: created.title,
        status: format!("{:?}", created.status),
        version: created.version,
        file_name: created.file_name,
    }))
}

#[derive(Deserialize)]
pub struct CheckoutRequest {
    pub user_id: Uuid,
}

pub async fn checkout_document(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CheckoutRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = DocumentService::new();
    service.checkout(&state.pool, id, req.user_id).await?;
    Ok(Json(serde_json::json!({ "status": "checked_out" })))
}

#[derive(Deserialize)]
pub struct CheckinRequest {
    pub checkout_id: Uuid,
}

pub async fn checkin_document(
    State(state): State<AppState>,
    Json(req): Json<CheckinRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = DocumentService::new();
    service.checkin(&state.pool, req.checkout_id).await?;
    Ok(Json(serde_json::json!({ "status": "checked_in" })))
}

#[derive(Deserialize)]
pub struct RequestReviewRequest {
    pub document_id: Uuid,
    pub version: i32,
    pub reviewer_id: Uuid,
}

pub async fn request_review(
    State(state): State<AppState>,
    Json(req): Json<RequestReviewRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = DocumentService::new();
    service.request_review(&state.pool, req.document_id, req.version, req.reviewer_id).await?;
    Ok(Json(serde_json::json!({ "status": "review_requested" })))
}

#[derive(Deserialize)]
pub struct CreateRetentionPolicyRequest {
    pub name: String,
    pub retention_years: i32,
    pub disposition: String,
}

pub async fn create_retention_policy(
    State(state): State<AppState>,
    Json(req): Json<CreateRetentionPolicyRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = DocumentService::new();
    let disposition = match req.disposition.as_str() {
        "Archive" => DispositionType::Archive,
        "Transfer" => DispositionType::Transfer,
        "Review" => DispositionType::Review,
        _ => DispositionType::Destroy,
    };
    service.create_retention_policy(&state.pool, req.name, req.retention_years, disposition).await?;
    Ok(Json(serde_json::json!({ "status": "created" })))
}

pub fn routes() -> axum::Router<crate::db::AppState> {
    axum::Router::new()
        .route("/folders", axum::routing::get(list_folders).post(create_folder))
        .route("/documents", axum::routing::get(list_documents).post(create_document))
        .route("/documents/:id", axum::routing::get(get_document))
        .route("/documents/:id/checkout", axum::routing::post(checkout_document))
        .route("/documents/checkin", axum::routing::post(checkin_document))
        .route("/documents/review", axum::routing::post(request_review))
        .route("/retention-policies", axum::routing::post(create_retention_policy))
}
