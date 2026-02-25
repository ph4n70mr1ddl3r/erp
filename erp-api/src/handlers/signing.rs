use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::db::AppState;
use erp_signing::*;

#[derive(Deserialize)]
pub struct CreateDocumentRequest {
    pub name: String,
    pub description: Option<String>,
    pub document_type: String,
    pub file_path: String,
    pub file_name: String,
    pub file_size: i64,
    pub file_hash: String,
    pub pages: i32,
    pub message: Option<String>,
}

#[derive(Deserialize)]
pub struct AddSignerRequest {
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub user_id: Option<Uuid>,
    pub role: SignerRole,
    pub order_index: i32,
}

#[derive(Deserialize)]
pub struct AddFieldRequest {
    pub signer_id: Uuid,
    pub field_type: SignatureFieldType,
    pub page: i32,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub required: bool,
}

#[derive(Deserialize)]
pub struct SignFieldRequest {
    pub value: String,
    pub signature_data: String,
    pub signature_type: SignatureType,
}

#[derive(Deserialize)]
pub struct DeclineRequest {
    pub reason: String,
}

pub async fn create_document(
    State(state): State<AppState>,
    Json(req): Json<CreateDocumentRequest>,
) -> Result<Json<SigningDocument>, StatusCode> {
    let user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let service = SigningService::new();
    let doc = service
        .create_document(
            &state.pool,
            req.name,
            req.description,
            req.document_type,
            req.file_path,
            req.file_name,
            req.file_size,
            req.file_hash,
            req.pages,
            user_id,
            req.message,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(doc))
}

pub async fn list_documents(
    State(state): State<AppState>,
) -> Result<Json<Vec<SigningDocument>>, StatusCode> {
    let user_id = Uuid::parse_str("00000000-0000-0000-0000-000000000001").unwrap();
    let service = SigningService::new();
    let docs = service
        .list(&state.pool, Some(user_id), None)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(docs))
}

pub async fn get_document(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<SigningDocument>, StatusCode> {
    let service = SigningService::new();
    let doc = service
        .get(&state.pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok(Json(doc))
}

pub async fn add_signer(
    State(state): State<AppState>,
    Path(document_id): Path<Uuid>,
    Json(req): Json<AddSignerRequest>,
) -> Result<Json<Signer>, StatusCode> {
    let service = SigningService::new();
    let signer = service
        .add_signer(
            &state.pool,
            document_id,
            req.name,
            req.email,
            req.phone,
            req.user_id,
            req.role,
            req.order_index,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(signer))
}

pub async fn add_field(
    State(state): State<AppState>,
    Path(document_id): Path<Uuid>,
    Json(req): Json<AddFieldRequest>,
) -> Result<Json<SignatureField>, StatusCode> {
    let service = SigningService::new();
    let field = service
        .add_signature_field(
            &state.pool,
            document_id,
            req.signer_id,
            req.field_type,
            req.page,
            req.x,
            req.y,
            req.width,
            req.height,
            req.required,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(field))
}

pub async fn send_for_signature(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let service = SigningService::new();
    service.send_for_signature(&state.pool, id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn view_document(
    State(state): State<AppState>,
    Path((document_id, signer_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    let service = SigningService::new();
    service.view_document(&state.pool, document_id, signer_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn sign_field(
    State(state): State<AppState>,
    Path(field_id): Path<Uuid>,
    Json(req): Json<SignFieldRequest>,
) -> Result<StatusCode, StatusCode> {
    let service = SigningService::new();
    service
        .sign_field(&state.pool, field_id, req.value, req.signature_data, req.signature_type)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn complete_signing(
    State(state): State<AppState>,
    Path((document_id, signer_id)): Path<(Uuid, Uuid)>,
) -> Result<StatusCode, StatusCode> {
    let service = SigningService::new();
    service.complete_signing(&state.pool, document_id, signer_id).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn decline(
    State(state): State<AppState>,
    Path((document_id, signer_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<DeclineRequest>,
) -> Result<StatusCode, StatusCode> {
    let service = SigningService::new();
    service.decline(&state.pool, document_id, signer_id, req.reason).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn get_audit_trail(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Vec<SigningAudit>>, StatusCode> {
    let service = SigningService::new();
    let audit = service
        .get_audit_trail(&state.pool, id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(audit))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", axum::routing::post(create_document).get(list_documents))
        .route("/:id", axum::routing::get(get_document))
        .route("/:document_id/signers", axum::routing::post(add_signer))
        .route("/:document_id/fields", axum::routing::post(add_field))
        .route("/:id/send", axum::routing::post(send_for_signature))
        .route("/:document_id/:signer_id/view", axum::routing::post(view_document))
        .route("/fields/:field_id/sign", axum::routing::post(sign_field))
        .route("/:document_id/:signer_id/complete", axum::routing::post(complete_signing))
        .route("/:document_id/:signer_id/decline", axum::routing::post(decline))
        .route("/:id/audit", axum::routing::get(get_audit_trail))
}
