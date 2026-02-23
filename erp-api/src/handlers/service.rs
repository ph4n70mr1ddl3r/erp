use axum::{extract::{Path, Query, State}, Json};
use uuid::Uuid;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{BaseEntity, Status, Pagination};
use erp_service::{ServiceTicket, KnowledgeArticle, SLA, ServiceTicketService, KnowledgeArticleService, SLAService, TicketPriority, TicketStatus, TicketType, TicketSource, ArticleStatus};

#[derive(Deserialize)]
pub struct CreateTicketRequest {
    pub subject: String,
    pub description: String,
    pub customer_id: Option<Uuid>,
    pub priority: Option<String>,
    pub ticket_type: Option<String>,
    pub source: Option<String>,
}

#[derive(Serialize)]
pub struct TicketResponse {
    pub id: Uuid,
    pub ticket_number: String,
    pub subject: String,
    pub description: String,
    pub customer_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub priority: String,
    pub status: String,
    pub ticket_type: String,
    pub created_at: String,
}

impl From<ServiceTicket> for TicketResponse {
    fn from(t: ServiceTicket) -> Self {
        Self {
            id: t.base.id,
            ticket_number: t.ticket_number,
            subject: t.subject,
            description: t.description,
            customer_id: t.customer_id,
            assigned_to: t.assigned_to,
            priority: format!("{:?}", t.priority),
            status: format!("{:?}", t.status),
            ticket_type: format!("{:?}", t.ticket_type),
            created_at: t.base.created_at.to_rfc3339(),
        }
    }
}

pub async fn list_tickets(State(state): State<AppState>, Query(pagination): Query<Pagination>) -> ApiResult<Json<erp_core::Paginated<TicketResponse>>> {
    let svc = ServiceTicketService::new();
    let tickets = svc.list(&state.pool, pagination.page, pagination.per_page).await?;
    Ok(Json(erp_core::Paginated::new(
        tickets.into_iter().map(TicketResponse::from).collect(),
        0,
        pagination,
    )))
}

pub async fn get_ticket(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<TicketResponse>> {
    let svc = ServiceTicketService::new();
    let ticket = svc.get(&state.pool, id).await?;
    Ok(Json(TicketResponse::from(ticket)))
}

pub async fn create_ticket(State(state): State<AppState>, Json(req): Json<CreateTicketRequest>) -> ApiResult<Json<TicketResponse>> {
    let svc = ServiceTicketService::new();
    let priority = match req.priority.as_deref() {
        Some("Critical") => TicketPriority::Critical,
        Some("High") => TicketPriority::High,
        Some("Low") => TicketPriority::Low,
        _ => TicketPriority::Medium,
    };
    let ticket_type = match req.ticket_type.as_deref() {
        Some("ServiceRequest") => TicketType::ServiceRequest,
        Some("Problem") => TicketType::Problem,
        Some("ChangeRequest") => TicketType::ChangeRequest,
        Some("Information") => TicketType::Information,
        _ => TicketType::Incident,
    };
    let source = match req.source.as_deref() {
        Some("Email") => TicketSource::Email,
        Some("Phone") => TicketSource::Phone,
        Some("Chat") => TicketSource::Chat,
        Some("Api") => TicketSource::Api,
        _ => TicketSource::WebPortal,
    };
    let ticket = svc.create(&state.pool, req.subject, req.description, req.customer_id, priority, ticket_type, source).await?;
    Ok(Json(TicketResponse::from(ticket)))
}

#[derive(Deserialize)]
pub struct AssignTicketRequest {
    pub assignee_id: Uuid,
}

pub async fn assign_ticket(State(state): State<AppState>, Path(id): Path<Uuid>, Json(req): Json<AssignTicketRequest>) -> ApiResult<Json<TicketResponse>> {
    let svc = ServiceTicketService::new();
    let ticket = svc.assign(&state.pool, id, req.assignee_id).await?;
    Ok(Json(TicketResponse::from(ticket)))
}

#[derive(Deserialize)]
pub struct UpdateStatusRequest {
    pub status: String,
}

pub async fn update_ticket_status(State(state): State<AppState>, Path(id): Path<Uuid>, Json(req): Json<UpdateStatusRequest>) -> ApiResult<Json<TicketResponse>> {
    let svc = ServiceTicketService::new();
    let status = match req.status.as_str() {
        "Open" => TicketStatus::Open,
        "Pending" => TicketStatus::Pending,
        "OnHold" => TicketStatus::OnHold,
        "Resolved" => TicketStatus::Resolved,
        "Closed" => TicketStatus::Closed,
        "Cancelled" => TicketStatus::Cancelled,
        _ => TicketStatus::New,
    };
    let ticket = svc.update_status(&state.pool, id, status).await?;
    Ok(Json(TicketResponse::from(ticket)))
}

#[derive(Deserialize)]
pub struct SatisfactionRequest {
    pub rating: i32,
    pub comment: Option<String>,
}

pub async fn set_satisfaction(State(state): State<AppState>, Path(id): Path<Uuid>, Json(req): Json<SatisfactionRequest>) -> ApiResult<Json<TicketResponse>> {
    let svc = ServiceTicketService::new();
    let ticket = svc.set_satisfaction(&state.pool, id, req.rating, req.comment).await?;
    Ok(Json(TicketResponse::from(ticket)))
}

pub async fn ticket_stats(State(state): State<AppState>) -> ApiResult<Json<serde_json::Value>> {
    let svc = ServiceTicketService::new();
    let stats = svc.get_stats(&state.pool).await?;
    Ok(Json(serde_json::json!({ "by_status": stats.into_iter().map(|(s, c)| (s, c)).collect::<std::collections::HashMap<_, _>>() })))
}

#[derive(Deserialize)]
pub struct CreateArticleRequest {
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
    pub category_id: Option<Uuid>,
    pub summary: Option<String>,
    pub tags: Option<Vec<String>>,
    pub article_type: Option<String>,
}

#[derive(Serialize)]
pub struct ArticleResponse {
    pub id: Uuid,
    pub title: String,
    pub summary: Option<String>,
    pub status: String,
    pub view_count: i64,
    pub created_at: String,
}

impl From<KnowledgeArticle> for ArticleResponse {
    fn from(a: KnowledgeArticle) -> Self {
        Self {
            id: a.id,
            title: a.title,
            summary: a.summary,
            status: format!("{:?}", a.status),
            view_count: a.view_count,
            created_at: a.created_at.to_rfc3339(),
        }
    }
}

pub async fn list_articles(State(state): State<AppState>, Query(pagination): Query<Pagination>) -> ApiResult<Json<erp_core::Paginated<ArticleResponse>>> {
    let svc = KnowledgeArticleService::new();
    let articles = svc.list(&state.pool, pagination.page, pagination.per_page).await?;
    Ok(Json(erp_core::Paginated::new(
        articles.into_iter().map(ArticleResponse::from).collect(),
        0,
        pagination,
    )))
}

pub async fn get_article(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    let svc = KnowledgeArticleService::new();
    let article = svc.get(&state.pool, id).await?;
    svc.record_view(&state.pool, id).await?;
    Ok(Json(serde_json::to_value(article)?))
}

pub async fn create_article(State(state): State<AppState>, Json(req): Json<CreateArticleRequest>) -> ApiResult<Json<ArticleResponse>> {
    let svc = KnowledgeArticleService::new();
    let article_type = match req.article_type.as_deref() {
        Some("HowTo") => erp_service::ArticleType::HowTo,
        Some("Troubleshooting") => erp_service::ArticleType::Troubleshooting,
        Some("FAQ") => erp_service::ArticleType::FAQ,
        Some("Policy") => erp_service::ArticleType::Policy,
        Some("BestPractice") => erp_service::ArticleType::BestPractice,
        _ => erp_service::ArticleType::KnowledgeBase,
    };
    let article = svc.create(&state.pool, req.title, req.content, req.author_id, article_type, req.category_id, req.summary, req.tags.unwrap_or_default()).await?;
    Ok(Json(ArticleResponse::from(article)))
}

pub async fn publish_article(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<ArticleResponse>> {
    let svc = KnowledgeArticleService::new();
    let article = svc.publish(&state.pool, id).await?;
    Ok(Json(ArticleResponse::from(article)))
}

pub async fn archive_article(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<ArticleResponse>> {
    let svc = KnowledgeArticleService::new();
    let article = svc.archive(&state.pool, id).await?;
    Ok(Json(ArticleResponse::from(article)))
}

#[derive(Deserialize)]
pub struct SearchArticlesQuery {
    pub q: String,
}

pub async fn search_articles(State(state): State<AppState>, Query(query): Query<SearchArticlesQuery>) -> ApiResult<Json<Vec<ArticleResponse>>> {
    let svc = KnowledgeArticleService::new();
    let articles = svc.search(&state.pool, &query.q).await?;
    Ok(Json(articles.into_iter().map(ArticleResponse::from).collect()))
}

#[derive(Deserialize)]
pub struct FeedbackRequest {
    pub helpful: bool,
}

pub async fn article_feedback(State(state): State<AppState>, Path(id): Path<Uuid>, Json(req): Json<FeedbackRequest>) -> ApiResult<Json<ArticleResponse>> {
    let svc = KnowledgeArticleService::new();
    let article = svc.record_feedback(&state.pool, id, req.helpful).await?;
    Ok(Json(ArticleResponse::from(article)))
}

#[derive(Deserialize)]
pub struct CreateSLARequest {
    pub name: String,
    pub description: Option<String>,
    pub response_time_hours: i32,
    pub resolution_time_hours: i32,
    pub business_hours_only: Option<bool>,
    pub timezone: Option<String>,
}

#[derive(Serialize)]
pub struct SLAResponse {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub response_time_hours: i32,
    pub resolution_time_hours: i32,
    pub status: String,
}

impl From<SLA> for SLAResponse {
    fn from(s: SLA) -> Self {
        Self {
            id: s.id,
            name: s.name,
            description: s.description,
            response_time_hours: s.response_time_hours,
            resolution_time_hours: s.resolution_time_hours,
            status: format!("{:?}", s.status),
        }
    }
}

pub async fn list_slas(State(state): State<AppState>) -> ApiResult<Json<Vec<SLAResponse>>> {
    let svc = SLAService::new();
    let slas = svc.list(&state.pool).await?;
    Ok(Json(slas.into_iter().map(SLAResponse::from).collect()))
}

pub async fn create_sla(State(state): State<AppState>, Json(req): Json<CreateSLARequest>) -> ApiResult<Json<SLAResponse>> {
    let svc = SLAService::new();
    let sla = svc.create(&state.pool, req.name, req.description, req.response_time_hours, req.resolution_time_hours, req.business_hours_only.unwrap_or(true), req.timezone.unwrap_or_else(|| "UTC".to_string())).await?;
    Ok(Json(SLAResponse::from(sla)))
}
