use async_trait::async_trait;
use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::models::*;

#[derive(sqlx::FromRow)]
struct TicketRow {
    id: String,
    ticket_number: String,
    subject: String,
    description: String,
    customer_id: Option<String>,
    contact_id: Option<String>,
    assigned_to: Option<String>,
    team_id: Option<String>,
    priority: String,
    status: String,
    ticket_type: String,
    source: String,
    category_id: Option<String>,
    sla_id: Option<String>,
    due_date: Option<String>,
    resolved_at: Option<String>,
    closed_at: Option<String>,
    first_response_at: Option<String>,
    satisfaction_rating: Option<i32>,
    satisfaction_comment: Option<String>,
    created_at: String,
    updated_at: String,
}

impl TicketRow {
    fn into_ticket(self) -> ServiceTicket {
        ServiceTicket {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&self.id).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&self.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&self.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            ticket_number: self.ticket_number,
            subject: self.subject,
            description: self.description,
            customer_id: self.customer_id.and_then(|s| Uuid::parse_str(&s).ok()),
            contact_id: self.contact_id.and_then(|s| Uuid::parse_str(&s).ok()),
            assigned_to: self.assigned_to.and_then(|s| Uuid::parse_str(&s).ok()),
            team_id: self.team_id.and_then(|s| Uuid::parse_str(&s).ok()),
            priority: serde_json::from_str(&self.priority).unwrap_or(TicketPriority::Medium),
            status: serde_json::from_str(&self.status).unwrap_or(TicketStatus::New),
            ticket_type: serde_json::from_str(&self.ticket_type).unwrap_or(TicketType::Incident),
            source: serde_json::from_str(&self.source).unwrap_or(TicketSource::WebPortal),
            category_id: self.category_id.and_then(|s| Uuid::parse_str(&s).ok()),
            sla_id: self.sla_id.and_then(|s| Uuid::parse_str(&s).ok()),
            due_date: self.due_date.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            resolved_at: self.resolved_at.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            closed_at: self.closed_at.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            first_response_at: self.first_response_at.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            satisfaction_rating: self.satisfaction_rating,
            satisfaction_comment: self.satisfaction_comment,
        }
    }
}

#[async_trait]
pub trait ServiceTicketRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, ticket: &ServiceTicket) -> Result<()>;
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<ServiceTicket>>;
    async fn find_all(&self, pool: &SqlitePool, limit: i64, offset: i64) -> Result<Vec<ServiceTicket>>;
    async fn update(&self, pool: &SqlitePool, ticket: &ServiceTicket) -> Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn find_by_customer(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<ServiceTicket>>;
    async fn find_by_assignee(&self, pool: &SqlitePool, assignee_id: Uuid) -> Result<Vec<ServiceTicket>>;
    async fn find_by_status(&self, pool: &SqlitePool, status: TicketStatus) -> Result<Vec<ServiceTicket>>;
    async fn count_by_status(&self, pool: &SqlitePool) -> Result<Vec<(String, i64)>>;
}

pub struct SqliteServiceTicketRepository;

#[async_trait]
impl ServiceTicketRepository for SqliteServiceTicketRepository {
    async fn create(&self, pool: &SqlitePool, ticket: &ServiceTicket) -> Result<()> {
        sqlx::query(
            "INSERT INTO service_tickets (id, ticket_number, subject, description, customer_id, contact_id, assigned_to, team_id, priority, status, ticket_type, source, category_id, sla_id, due_date, resolved_at, closed_at, first_response_at, satisfaction_rating, satisfaction_comment, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(ticket.base.id.to_string())
            .bind(&ticket.ticket_number)
            .bind(&ticket.subject)
            .bind(&ticket.description)
            .bind(ticket.customer_id.map(|id| id.to_string()))
            .bind(ticket.contact_id.map(|id| id.to_string()))
            .bind(ticket.assigned_to.map(|id| id.to_string()))
            .bind(ticket.team_id.map(|id| id.to_string()))
            .bind(serde_json::to_string(&ticket.priority)?)
            .bind(serde_json::to_string(&ticket.status)?)
            .bind(serde_json::to_string(&ticket.ticket_type)?)
            .bind(serde_json::to_string(&ticket.source)?)
            .bind(ticket.category_id.map(|id| id.to_string()))
            .bind(ticket.sla_id.map(|id| id.to_string()))
            .bind(ticket.due_date.map(|d| d.to_rfc3339()))
            .bind(ticket.resolved_at.map(|d| d.to_rfc3339()))
            .bind(ticket.closed_at.map(|d| d.to_rfc3339()))
            .bind(ticket.first_response_at.map(|d| d.to_rfc3339()))
            .bind(ticket.satisfaction_rating)
            .bind(&ticket.satisfaction_comment)
            .bind(ticket.base.created_at.to_rfc3339())
            .bind(ticket.base.updated_at.to_rfc3339())
            .execute(pool).await?;
        Ok(())
    }

    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<ServiceTicket>> {
        let row = sqlx::query_as::<_, TicketRow>(
            "SELECT id, ticket_number, subject, description, customer_id, contact_id, assigned_to, team_id, priority, status, ticket_type, source, category_id, sla_id, due_date, resolved_at, closed_at, first_response_at, satisfaction_rating, satisfaction_comment, created_at, updated_at FROM service_tickets WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(pool).await?;
        Ok(row.map(|r| r.into_ticket()))
    }

    async fn find_all(&self, pool: &SqlitePool, limit: i64, offset: i64) -> Result<Vec<ServiceTicket>> {
        let rows = sqlx::query_as::<_, TicketRow>(
            "SELECT id, ticket_number, subject, description, customer_id, contact_id, assigned_to, team_id, priority, status, ticket_type, source, category_id, sla_id, due_date, resolved_at, closed_at, first_response_at, satisfaction_rating, satisfaction_comment, created_at, updated_at FROM service_tickets ORDER BY created_at DESC LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_ticket()).collect())
    }

    async fn update(&self, pool: &SqlitePool, ticket: &ServiceTicket) -> Result<()> {
        sqlx::query(
            "UPDATE service_tickets SET ticket_number = ?, subject = ?, description = ?, customer_id = ?, contact_id = ?, assigned_to = ?, team_id = ?, priority = ?, status = ?, ticket_type = ?, source = ?, category_id = ?, sla_id = ?, due_date = ?, resolved_at = ?, closed_at = ?, first_response_at = ?, satisfaction_rating = ?, satisfaction_comment = ?, updated_at = ? WHERE id = ?")
            .bind(&ticket.ticket_number)
            .bind(&ticket.subject)
            .bind(&ticket.description)
            .bind(ticket.customer_id.map(|id| id.to_string()))
            .bind(ticket.contact_id.map(|id| id.to_string()))
            .bind(ticket.assigned_to.map(|id| id.to_string()))
            .bind(ticket.team_id.map(|id| id.to_string()))
            .bind(serde_json::to_string(&ticket.priority)?)
            .bind(serde_json::to_string(&ticket.status)?)
            .bind(serde_json::to_string(&ticket.ticket_type)?)
            .bind(serde_json::to_string(&ticket.source)?)
            .bind(ticket.category_id.map(|id| id.to_string()))
            .bind(ticket.sla_id.map(|id| id.to_string()))
            .bind(ticket.due_date.map(|d| d.to_rfc3339()))
            .bind(ticket.resolved_at.map(|d| d.to_rfc3339()))
            .bind(ticket.closed_at.map(|d| d.to_rfc3339()))
            .bind(ticket.first_response_at.map(|d| d.to_rfc3339()))
            .bind(ticket.satisfaction_rating)
            .bind(&ticket.satisfaction_comment)
            .bind(ticket.base.updated_at.to_rfc3339())
            .bind(ticket.base.id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM service_tickets WHERE id = ?")
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn find_by_customer(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<ServiceTicket>> {
        let rows = sqlx::query_as::<_, TicketRow>(
            "SELECT id, ticket_number, subject, description, customer_id, contact_id, assigned_to, team_id, priority, status, ticket_type, source, category_id, sla_id, due_date, resolved_at, closed_at, first_response_at, satisfaction_rating, satisfaction_comment, created_at, updated_at FROM service_tickets WHERE customer_id = ? ORDER BY created_at DESC")
            .bind(customer_id.to_string())
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_ticket()).collect())
    }

    async fn find_by_assignee(&self, pool: &SqlitePool, assignee_id: Uuid) -> Result<Vec<ServiceTicket>> {
        let rows = sqlx::query_as::<_, TicketRow>(
            "SELECT id, ticket_number, subject, description, customer_id, contact_id, assigned_to, team_id, priority, status, ticket_type, source, category_id, sla_id, due_date, resolved_at, closed_at, first_response_at, satisfaction_rating, satisfaction_comment, created_at, updated_at FROM service_tickets WHERE assigned_to = ? ORDER BY created_at DESC")
            .bind(assignee_id.to_string())
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_ticket()).collect())
    }

    async fn find_by_status(&self, pool: &SqlitePool, status: TicketStatus) -> Result<Vec<ServiceTicket>> {
        let status_str = serde_json::to_string(&status)?;
        let rows = sqlx::query_as::<_, TicketRow>(
            "SELECT id, ticket_number, subject, description, customer_id, contact_id, assigned_to, team_id, priority, status, ticket_type, source, category_id, sla_id, due_date, resolved_at, closed_at, first_response_at, satisfaction_rating, satisfaction_comment, created_at, updated_at FROM service_tickets WHERE status = ? ORDER BY created_at DESC")
            .bind(status_str)
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_ticket()).collect())
    }

    async fn count_by_status(&self, pool: &SqlitePool) -> Result<Vec<(String, i64)>> {
        #[derive(sqlx::FromRow)]
        struct StatusCount { status: String, count: i64 }
        let rows = sqlx::query_as::<_, StatusCount>(
            "SELECT status, COUNT(*) as count FROM service_tickets GROUP BY status")
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| (r.status, r.count)).collect())
    }
}

#[derive(sqlx::FromRow)]
struct ArticleRow {
    id: String,
    title: String,
    content: String,
    summary: Option<String>,
    category_id: Option<String>,
    author_id: String,
    article_type: String,
    status: String,
    view_count: i64,
    helpful_count: i64,
    not_helpful_count: i64,
    tags: String,
    published_at: Option<String>,
    expires_at: Option<String>,
    created_at: String,
    updated_at: String,
}

impl ArticleRow {
    fn into_article(self) -> KnowledgeArticle {
        KnowledgeArticle {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            title: self.title,
            content: self.content,
            summary: self.summary,
            category_id: self.category_id.and_then(|s| Uuid::parse_str(&s).ok()),
            author_id: Uuid::parse_str(&self.author_id).unwrap_or_default(),
            article_type: serde_json::from_str(&self.article_type).unwrap_or(ArticleType::KnowledgeBase),
            status: serde_json::from_str(&self.status).unwrap_or(ArticleStatus::Draft),
            view_count: self.view_count,
            helpful_count: self.helpful_count,
            not_helpful_count: self.not_helpful_count,
            tags: serde_json::from_str(&self.tags).unwrap_or_default(),
            published_at: self.published_at.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            expires_at: self.expires_at.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            created_at: DateTime::parse_from_rfc3339(&self.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            updated_at: DateTime::parse_from_rfc3339(&self.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[async_trait]
pub trait KnowledgeArticleRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, article: &KnowledgeArticle) -> Result<()>;
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<KnowledgeArticle>>;
    async fn find_all(&self, pool: &SqlitePool, limit: i64, offset: i64) -> Result<Vec<KnowledgeArticle>>;
    async fn find_published(&self, pool: &SqlitePool) -> Result<Vec<KnowledgeArticle>>;
    async fn update(&self, pool: &SqlitePool, article: &KnowledgeArticle) -> Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn search(&self, pool: &SqlitePool, query: &str) -> Result<Vec<KnowledgeArticle>>;
}

pub struct SqliteKnowledgeArticleRepository;

#[async_trait]
impl KnowledgeArticleRepository for SqliteKnowledgeArticleRepository {
    async fn create(&self, pool: &SqlitePool, article: &KnowledgeArticle) -> Result<()> {
        sqlx::query(
            "INSERT INTO knowledge_articles (id, title, content, summary, category_id, author_id, article_type, status, view_count, helpful_count, not_helpful_count, tags, published_at, expires_at, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(article.id.to_string())
            .bind(&article.title)
            .bind(&article.content)
            .bind(&article.summary)
            .bind(article.category_id.map(|id| id.to_string()))
            .bind(article.author_id.to_string())
            .bind(serde_json::to_string(&article.article_type)?)
            .bind(serde_json::to_string(&article.status)?)
            .bind(article.view_count)
            .bind(article.helpful_count)
            .bind(article.not_helpful_count)
            .bind(serde_json::to_string(&article.tags)?)
            .bind(article.published_at.map(|d| d.to_rfc3339()))
            .bind(article.expires_at.map(|d| d.to_rfc3339()))
            .bind(article.created_at.to_rfc3339())
            .bind(article.updated_at.to_rfc3339())
            .execute(pool).await?;
        Ok(())
    }

    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<KnowledgeArticle>> {
        let row = sqlx::query_as::<_, ArticleRow>(
            "SELECT id, title, content, summary, category_id, author_id, article_type, status, view_count, helpful_count, not_helpful_count, tags, published_at, expires_at, created_at, updated_at FROM knowledge_articles WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(pool).await?;
        Ok(row.map(|r| r.into_article()))
    }

    async fn find_all(&self, pool: &SqlitePool, limit: i64, offset: i64) -> Result<Vec<KnowledgeArticle>> {
        let rows = sqlx::query_as::<_, ArticleRow>(
            "SELECT id, title, content, summary, category_id, author_id, article_type, status, view_count, helpful_count, not_helpful_count, tags, published_at, expires_at, created_at, updated_at FROM knowledge_articles ORDER BY created_at DESC LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_article()).collect())
    }

    async fn find_published(&self, pool: &SqlitePool) -> Result<Vec<KnowledgeArticle>> {
        let rows = sqlx::query_as::<_, ArticleRow>(
            "SELECT id, title, content, summary, category_id, author_id, article_type, status, view_count, helpful_count, not_helpful_count, tags, published_at, expires_at, created_at, updated_at FROM knowledge_articles WHERE status = '\"Published\"' ORDER BY published_at DESC")
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_article()).collect())
    }

    async fn update(&self, pool: &SqlitePool, article: &KnowledgeArticle) -> Result<()> {
        sqlx::query(
            "UPDATE knowledge_articles SET title = ?, content = ?, summary = ?, category_id = ?, author_id = ?, article_type = ?, status = ?, view_count = ?, helpful_count = ?, not_helpful_count = ?, tags = ?, published_at = ?, expires_at = ?, updated_at = ? WHERE id = ?")
            .bind(&article.title)
            .bind(&article.content)
            .bind(&article.summary)
            .bind(article.category_id.map(|id| id.to_string()))
            .bind(article.author_id.to_string())
            .bind(serde_json::to_string(&article.article_type)?)
            .bind(serde_json::to_string(&article.status)?)
            .bind(article.view_count)
            .bind(article.helpful_count)
            .bind(article.not_helpful_count)
            .bind(serde_json::to_string(&article.tags)?)
            .bind(article.published_at.map(|d| d.to_rfc3339()))
            .bind(article.expires_at.map(|d| d.to_rfc3339()))
            .bind(article.updated_at.to_rfc3339())
            .bind(article.id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM knowledge_articles WHERE id = ?")
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn search(&self, pool: &SqlitePool, query: &str) -> Result<Vec<KnowledgeArticle>> {
        let search_pattern = format!("%{}%", query);
        let rows = sqlx::query_as::<_, ArticleRow>(
            "SELECT id, title, content, summary, category_id, author_id, article_type, status, view_count, helpful_count, not_helpful_count, tags, published_at, expires_at, created_at, updated_at FROM knowledge_articles WHERE (title LIKE ? OR content LIKE ? OR summary LIKE ?) AND status = '\"Published\"' ORDER BY view_count DESC")
            .bind(&search_pattern)
            .bind(&search_pattern)
            .bind(&search_pattern)
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_article()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct SLARow {
    id: String,
    name: String,
    description: Option<String>,
    response_time_hours: i32,
    resolution_time_hours: i32,
    business_hours_only: i32,
    timezone: String,
    escalation_rule_id: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
}

impl SLARow {
    fn into_sla(self) -> SLA {
        SLA {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            name: self.name,
            description: self.description,
            response_time_hours: self.response_time_hours,
            resolution_time_hours: self.resolution_time_hours,
            business_hours_only: self.business_hours_only != 0,
            timezone: self.timezone,
            escalation_rule_id: self.escalation_rule_id.and_then(|s| Uuid::parse_str(&s).ok()),
            status: serde_json::from_str(&self.status).unwrap_or(erp_core::Status::Active),
            created_at: DateTime::parse_from_rfc3339(&self.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            updated_at: DateTime::parse_from_rfc3339(&self.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[async_trait]
pub trait SLARepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, sla: &SLA) -> Result<()>;
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<SLA>>;
    async fn find_all(&self, pool: &SqlitePool) -> Result<Vec<SLA>>;
    async fn update(&self, pool: &SqlitePool, sla: &SLA) -> Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteSLARepository;

#[async_trait]
impl SLARepository for SqliteSLARepository {
    async fn create(&self, pool: &SqlitePool, sla: &SLA) -> Result<()> {
        sqlx::query(
            "INSERT INTO slas (id, name, description, response_time_hours, resolution_time_hours, business_hours_only, timezone, escalation_rule_id, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(sla.id.to_string())
            .bind(&sla.name)
            .bind(&sla.description)
            .bind(sla.response_time_hours)
            .bind(sla.resolution_time_hours)
            .bind(if sla.business_hours_only { 1 } else { 0 })
            .bind(&sla.timezone)
            .bind(sla.escalation_rule_id.map(|id| id.to_string()))
            .bind(serde_json::to_string(&sla.status)?)
            .bind(sla.created_at.to_rfc3339())
            .bind(sla.updated_at.to_rfc3339())
            .execute(pool).await?;
        Ok(())
    }

    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<SLA>> {
        let row = sqlx::query_as::<_, SLARow>(
            "SELECT id, name, description, response_time_hours, resolution_time_hours, business_hours_only, timezone, escalation_rule_id, status, created_at, updated_at FROM slas WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(pool).await?;
        Ok(row.map(|r| r.into_sla()))
    }

    async fn find_all(&self, pool: &SqlitePool) -> Result<Vec<SLA>> {
        let rows = sqlx::query_as::<_, SLARow>(
            "SELECT id, name, description, response_time_hours, resolution_time_hours, business_hours_only, timezone, escalation_rule_id, status, created_at, updated_at FROM slas ORDER BY name")
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_sla()).collect())
    }

    async fn update(&self, pool: &SqlitePool, sla: &SLA) -> Result<()> {
        sqlx::query(
            "UPDATE slas SET name = ?, description = ?, response_time_hours = ?, resolution_time_hours = ?, business_hours_only = ?, timezone = ?, escalation_rule_id = ?, status = ?, updated_at = ? WHERE id = ?")
            .bind(&sla.name)
            .bind(&sla.description)
            .bind(sla.response_time_hours)
            .bind(sla.resolution_time_hours)
            .bind(if sla.business_hours_only { 1 } else { 0 })
            .bind(&sla.timezone)
            .bind(sla.escalation_rule_id.map(|id| id.to_string()))
            .bind(serde_json::to_string(&sla.status)?)
            .bind(sla.updated_at.to_rfc3339())
            .bind(sla.id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM slas WHERE id = ?")
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }
}
