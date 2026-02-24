use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::AppState;
use crate::error::ApiResult;

#[derive(Debug, Deserialize)]
pub struct CreateTemplateRequest {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub template_type: String,
    pub format: String,
    pub subject: Option<String>,
    pub body: String,
    pub html_body: Option<String>,
    pub variables: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct TemplateResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub template_type: String,
    pub format: String,
    pub version: i32,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct RenderTemplateRequest {
    pub template_id: Option<Uuid>,
    pub template_code: Option<String>,
    pub variables: serde_json::Value,
}

#[derive(Debug, Serialize)]
pub struct RenderedTemplateResponse {
    pub subject: Option<String>,
    pub body: String,
    pub html_body: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct TemplateListQuery {
    pub template_type: Option<String>,
}

pub async fn create_template(
    State(state): State<AppState>,
    Json(req): Json<CreateTemplateRequest>,
) -> ApiResult<Json<TemplateResponse>> {
    let created_by = Uuid::nil();
    let template_type = parse_template_type(&req.template_type)?;
    let format = parse_template_format(&req.format)?;
    
    let service = erp_templates::TemplateService::new();
    let template = service.create(
        &state.pool,
        req.name,
        req.code,
        req.description,
        template_type,
        format,
        req.subject,
        req.body,
        req.html_body,
        req.variables,
        created_by,
    ).await?;
    
    Ok(Json(TemplateResponse {
        id: template.base.id,
        name: template.name,
        code: template.code,
        template_type: format!("{:?}", template.template_type),
        format: format!("{:?}", template.format),
        version: template.version,
        status: format!("{:?}", template.status),
        created_at: template.created_at.to_rfc3339(),
    }))
}

pub async fn list_templates(
    State(state): State<AppState>,
    Query(query): Query<TemplateListQuery>,
) -> ApiResult<Json<Vec<TemplateResponse>>> {
    let template_type = query.template_type.as_deref()
        .map(parse_template_type)
        .transpose()?;
    
    let service = erp_templates::TemplateService::new();
    let templates = service.list(&state.pool, template_type).await?;
    
    let response: Vec<TemplateResponse> = templates.into_iter().map(|t| TemplateResponse {
        id: t.base.id,
        name: t.name,
        code: t.code,
        template_type: format!("{:?}", t.template_type),
        format: format!("{:?}", t.format),
        version: t.version,
        status: format!("{:?}", t.status),
        created_at: t.created_at.to_rfc3339(),
    }).collect();
    
    Ok(Json(response))
}

pub async fn get_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = erp_templates::TemplateService::new();
    let template = service.get(&state.pool, id).await?
        .ok_or_else(|| crate::error::ApiError::NotFound("Template not found".into()))?;
    
    Ok(Json(serde_json::json!({
        "id": template.base.id,
        "name": template.name,
        "code": template.code,
        "description": template.description,
        "template_type": format!("{:?}", template.template_type),
        "format": format!("{:?}", template.format),
        "subject": template.subject,
        "body": template.body,
        "html_body": template.html_body,
        "variables": template.variables,
        "version": template.version,
        "status": format!("{:?}", template.status),
        "created_at": template.created_at.to_rfc3339(),
    })))
}

pub async fn render_template(
    State(state): State<AppState>,
    Json(req): Json<RenderTemplateRequest>,
) -> ApiResult<Json<RenderedTemplateResponse>> {
    let service = erp_templates::TemplateService::new();
    
    let template = if let Some(id) = req.template_id {
        service.get(&state.pool, id).await?
    } else if let Some(code) = req.template_code.as_ref() {
        service.get_by_code(&state.pool, code).await?
    } else {
        return Err(crate::error::ApiError::BadRequest("Either template_id or template_code is required".into()));
    };
    
    let template = template.ok_or_else(|| crate::error::ApiError::NotFound("Template not found".into()))?;
    let rendered = service.render(&template, &req.variables)?;
    
    Ok(Json(RenderedTemplateResponse {
        subject: rendered.subject,
        body: rendered.body,
        html_body: rendered.html_body,
    }))
}

pub async fn delete_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let service = erp_templates::TemplateService::new();
    service.delete(&state.pool, id).await?;
    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
pub struct CreateEmailTemplateRequest {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub subject_template: String,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub from_name: Option<String>,
    pub from_email: Option<String>,
    pub variables: Option<serde_json::Value>,
}

pub async fn create_email_template(
    State(state): State<AppState>,
    Json(req): Json<CreateEmailTemplateRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = erp_templates::EmailTemplateService::new();
    let template = service.create(
        &state.pool,
        req.name,
        req.code,
        req.description,
        req.subject_template,
        req.body_text,
        req.body_html,
        req.from_name,
        req.from_email,
        req.variables,
    ).await?;
    
    Ok(Json(serde_json::json!({
        "id": template.base.id,
        "name": template.name,
        "code": template.code,
        "status": format!("{:?}", template.status),
        "created_at": template.created_at.to_rfc3339(),
    })))
}

pub async fn list_email_templates(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let service = erp_templates::EmailTemplateService::new();
    let templates = service.list(&state.pool).await?;
    
    let response: Vec<serde_json::Value> = templates.into_iter().map(|t| serde_json::json!({
        "id": t.base.id,
        "name": t.name,
        "code": t.code,
        "subject_template": t.subject_template,
        "from_name": t.from_name,
        "from_email": t.from_email,
        "status": format!("{:?}", t.status),
        "created_at": t.created_at.to_rfc3339(),
    })).collect();
    
    Ok(Json(response))
}

pub async fn render_email_template(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(variables): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = erp_templates::EmailTemplateService::new();
    let template = service.get(&state.pool, id).await?
        .ok_or_else(|| crate::error::ApiError::NotFound("Email template not found".into()))?;
    
    let rendered = service.render(&template, &variables)?;
    
    Ok(Json(serde_json::json!({
        "subject": rendered.subject,
        "body_text": rendered.body_text,
        "body_html": rendered.body_html,
        "from_name": rendered.from_name,
        "from_email": rendered.from_email,
    })))
}

fn parse_template_type(s: &str) -> anyhow::Result<erp_templates::TemplateType> {
    match s {
        "Email" => Ok(erp_templates::TemplateType::Email),
        "Document" => Ok(erp_templates::TemplateType::Document),
        "Report" => Ok(erp_templates::TemplateType::Report),
        "Label" => Ok(erp_templates::TemplateType::Label),
        "Invoice" => Ok(erp_templates::TemplateType::Invoice),
        "Quote" => Ok(erp_templates::TemplateType::Quote),
        "PurchaseOrder" => Ok(erp_templates::TemplateType::PurchaseOrder),
        "PackingSlip" => Ok(erp_templates::TemplateType::PackingSlip),
        "Contract" => Ok(erp_templates::TemplateType::Contract),
        "Letter" => Ok(erp_templates::TemplateType::Letter),
        "SMS" => Ok(erp_templates::TemplateType::SMS),
        "PushNotification" => Ok(erp_templates::TemplateType::PushNotification),
        "Webhook" => Ok(erp_templates::TemplateType::Webhook),
        _ => Err(anyhow::anyhow!("Invalid template type: {}", s)),
    }
}

fn parse_template_format(s: &str) -> anyhow::Result<erp_templates::TemplateFormat> {
    match s {
        "HTML" => Ok(erp_templates::TemplateFormat::HTML),
        "PlainText" => Ok(erp_templates::TemplateFormat::PlainText),
        "Markdown" => Ok(erp_templates::TemplateFormat::Markdown),
        "PDF" => Ok(erp_templates::TemplateFormat::PDF),
        "JSON" => Ok(erp_templates::TemplateFormat::JSON),
        "XML" => Ok(erp_templates::TemplateFormat::XML),
        "CSV" => Ok(erp_templates::TemplateFormat::CSV),
        _ => Err(anyhow::anyhow!("Invalid template format: {}", s)),
    }
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", axum::routing::post(create_template).get(list_templates))
        .route("/:id", axum::routing::get(get_template).delete(delete_template))
        .route("/render", axum::routing::post(render_template))
        .route("/email", axum::routing::post(create_email_template).get(list_email_templates))
        .route("/email/:id/render", axum::routing::post(render_email_template))
}
