use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::AppState;
use crate::handlers::ApiResult;
use erp_ocr::{OcrService, UploadDocumentRequest, OcrDocument, DocumentType, OcrTemplate, OcrSettings};

#[derive(Deserialize)]
pub struct ListDocumentsQuery {
    status: Option<String>,
    #[serde(default = "default_limit")]
    limit: i64,
    #[serde(default)]
    offset: i64,
}

fn default_limit() -> i64 { 50 }

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/documents", axum::routing::get(list_documents).post(upload_document))
        .route("/documents/:id", axum::routing::get(get_document).delete(delete_document))
        .route("/documents/:id/process", axum::routing::post(process_document))
        .route("/documents/:id/review", axum::routing::post(review_document))
        .route("/templates", axum::routing::get(list_templates).post(create_template))
        .route("/templates/:id", axum::routing::get(get_template).delete(delete_template))
        .route("/batch-jobs", axum::routing::post(create_batch_job))
        .route("/batch-jobs/:id", axum::routing::get(get_batch_job))
        .route("/settings", axum::routing::get(get_settings).put(update_settings))
}

#[derive(Deserialize)]
struct UploadDocumentBody {
    document_type: String,
    filename: String,
    content: String,
    template_id: Option<Uuid>,
    #[serde(default)]
    auto_process: bool,
}

async fn upload_document(
    State(_state): State<AppState>,
    Json(body): Json<UploadDocumentBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = OcrService::new();
    let doc_type = match body.document_type.as_str() {
        "invoice" => DocumentType::Invoice,
        "receipt" => DocumentType::Receipt,
        "purchase_order" => DocumentType::PurchaseOrder,
        "sales_order" => DocumentType::SalesOrder,
        "delivery_note" => DocumentType::DeliveryNote,
        "bank_statement" => DocumentType::BankStatement,
        "check" => DocumentType::Check,
        "contract" => DocumentType::Contract,
        "id_document" => DocumentType::IdDocument,
        "business_card" => DocumentType::BusinessCard,
        "form" => DocumentType::Form,
        _ => DocumentType::Other,
    };
    
    let doc = service.upload_document(UploadDocumentRequest {
        document_type: doc_type,
        filename: body.filename,
        content: body.content,
        template_id: body.template_id,
        auto_process: body.auto_process,
    }).await?;
    Ok(Json(serde_json::to_value(doc)?))
}

async fn get_document(
    Path(id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = OcrService::new();
    let doc = service.get_document(id).await?;
    Ok(Json(serde_json::to_value(doc)?))
}

async fn list_documents(
    Query(query): Query<ListDocumentsQuery>,
    State(_state): State<AppState>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let service = OcrService::new();
    let status = query.status.and_then(|s| match s.as_str() {
        "pending" => Some(erp_ocr::OcrStatus::Pending),
        "processing" => Some(erp_ocr::OcrStatus::Processing),
        "completed" => Some(erp_ocr::OcrStatus::Completed),
        "failed" => Some(erp_ocr::OcrStatus::Failed),
        "requires_review" => Some(erp_ocr::OcrStatus::RequiresReview),
        "validated" => Some(erp_ocr::OcrStatus::Validated),
        _ => None,
    });
    let docs = service.list_documents(status, query.limit, query.offset).await?;
    Ok(Json(docs.into_iter().map(|d| serde_json::to_value(d).unwrap()).collect()))
}

async fn delete_document(
    Path(_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

async fn process_document(
    Path(id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = OcrService::new();
    let result = service.process_document(id).await?;
    Ok(Json(serde_json::to_value(result)?))
}

#[derive(Deserialize)]
struct ReviewDocumentBody {
    corrections: Option<serde_json::Value>,
}

async fn review_document(
    Path(id): Path<Uuid>,
    State(_state): State<AppState>,
    axum::Extension(user_id): axum::Extension<Uuid>,
    Json(body): Json<ReviewDocumentBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = OcrService::new();
    let doc = service.review_document(id, user_id, body.corrections).await?;
    Ok(Json(serde_json::to_value(doc)?))
}

async fn create_template(
    State(_state): State<AppState>,
    Json(_template): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(_template))
}

async fn get_template(
    Path(_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({})))
}

async fn list_templates(
    State(_state): State<AppState>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let service = OcrService::new();
    let templates = service.list_templates(None).await?;
    Ok(Json(templates.into_iter().map(|t| serde_json::to_value(t).unwrap()).collect()))
}

async fn delete_template(
    Path(_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
struct CreateBatchJobBody {
    name: String,
    document_ids: Vec<Uuid>,
    template_id: Option<Uuid>,
}

async fn create_batch_job(
    State(_state): State<AppState>,
    Json(body): Json<CreateBatchJobBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = OcrService::new();
    let job = service.create_batch_job(body.name, body.document_ids, body.template_id).await?;
    Ok(Json(serde_json::to_value(job)?))
}

async fn get_batch_job(
    Path(_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({})))
}

async fn get_settings(
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = OcrService::new();
    let settings = service.get_settings().await?;
    Ok(Json(serde_json::to_value(settings)?))
}

async fn update_settings(
    State(_state): State<AppState>,
    Json(settings): Json<OcrSettings>,
) -> ApiResult<StatusCode> {
    let service = OcrService::new();
    service.update_settings(settings).await?;
    Ok(StatusCode::NO_CONTENT)
}
