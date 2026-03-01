use anyhow::{Result, anyhow};
use chrono::Utc;
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::*;
use crate::repository::*;

pub struct ServiceTicketService {
    repo: SqliteServiceTicketRepository,
}

impl Default for ServiceTicketService {
    fn default() -> Self {
        Self::new()
    }
}

impl ServiceTicketService {
    pub fn new() -> Self {
        Self { repo: SqliteServiceTicketRepository }
    }

    pub async fn create(&self, pool: &SqlitePool, subject: String, description: String, customer_id: Option<Uuid>, priority: TicketPriority, ticket_type: TicketType, source: TicketSource) -> Result<ServiceTicket> {
        let ticket_number = format!("TKT-{}", Utc::now().format("%Y%m%d%H%M%S"));
        let _now = Utc::now();
        let ticket = ServiceTicket {
            base: BaseEntity::new(),
            ticket_number,
            subject,
            description,
            customer_id,
            contact_id: None,
            assigned_to: None,
            team_id: None,
            priority,
            status: TicketStatus::New,
            ticket_type,
            source,
            category_id: None,
            sla_id: None,
            due_date: None,
            resolved_at: None,
            closed_at: None,
            first_response_at: None,
            satisfaction_rating: None,
            satisfaction_comment: None,
        };
        self.repo.create(pool, &ticket).await?;
        Ok(ticket)
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<ServiceTicket> {
        self.repo.find_by_id(pool, id).await?.ok_or_else(|| anyhow!("Ticket not found"))
    }

    pub async fn list(&self, pool: &SqlitePool, page: i64, page_size: i64) -> Result<Vec<ServiceTicket>> {
        let offset = (page - 1) * page_size;
        self.repo.find_all(pool, page_size, offset).await
    }

    pub async fn assign(&self, pool: &SqlitePool, id: Uuid, assignee_id: Uuid) -> Result<ServiceTicket> {
        let mut ticket = self.get(pool, id).await?;
        ticket.assigned_to = Some(assignee_id);
        if ticket.status == TicketStatus::New {
            ticket.status = TicketStatus::Open;
        }
        ticket.base.updated_at = Utc::now();
        self.repo.update(pool, &ticket).await?;
        Ok(ticket)
    }

    pub async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: TicketStatus) -> Result<ServiceTicket> {
        let mut ticket = self.get(pool, id).await?;
        let now = Utc::now();
        match status {
            TicketStatus::Resolved => ticket.resolved_at = Some(now),
            TicketStatus::Closed => ticket.closed_at = Some(now),
            _ => {}
        }
        ticket.status = status;
        ticket.base.updated_at = now;
        self.repo.update(pool, &ticket).await?;
        Ok(ticket)
    }

    pub async fn add_first_response(&self, pool: &SqlitePool, id: Uuid) -> Result<ServiceTicket> {
        let mut ticket = self.get(pool, id).await?;
        if ticket.first_response_at.is_none() {
            ticket.first_response_at = Some(Utc::now());
            ticket.base.updated_at = Utc::now();
            self.repo.update(pool, &ticket).await?;
        }
        Ok(ticket)
    }

    pub async fn set_satisfaction(&self, pool: &SqlitePool, id: Uuid, rating: i32, comment: Option<String>) -> Result<ServiceTicket> {
        let mut ticket = self.get(pool, id).await?;
        if !(1..=5).contains(&rating) {
            return Err(anyhow!("Rating must be between 1 and 5"));
        }
        ticket.satisfaction_rating = Some(rating);
        ticket.satisfaction_comment = comment;
        ticket.base.updated_at = Utc::now();
        self.repo.update(pool, &ticket).await?;
        Ok(ticket)
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete(pool, id).await
    }

    pub async fn get_by_customer(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<ServiceTicket>> {
        self.repo.find_by_customer(pool, customer_id).await
    }

    pub async fn get_by_assignee(&self, pool: &SqlitePool, assignee_id: Uuid) -> Result<Vec<ServiceTicket>> {
        self.repo.find_by_assignee(pool, assignee_id).await
    }

    pub async fn get_stats(&self, pool: &SqlitePool) -> Result<Vec<(String, i64)>> {
        self.repo.count_by_status(pool).await
    }
}

pub struct KnowledgeArticleService {
    repo: SqliteKnowledgeArticleRepository,
}

impl Default for KnowledgeArticleService {
    fn default() -> Self {
        Self::new()
    }
}

impl KnowledgeArticleService {
    pub fn new() -> Self {
        Self { repo: SqliteKnowledgeArticleRepository }
    }

    pub async fn create(&self, pool: &SqlitePool, title: String, content: String, author_id: Uuid, article_type: ArticleType, category_id: Option<Uuid>, summary: Option<String>, tags: Vec<String>) -> Result<KnowledgeArticle> {
        let now = Utc::now();
        let article = KnowledgeArticle {
            id: Uuid::new_v4(),
            title,
            content,
            summary,
            category_id,
            author_id,
            article_type,
            status: ArticleStatus::Draft,
            view_count: 0,
            helpful_count: 0,
            not_helpful_count: 0,
            tags,
            published_at: None,
            expires_at: None,
            created_at: now,
            updated_at: now,
        };
        self.repo.create(pool, &article).await?;
        Ok(article)
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<KnowledgeArticle> {
        self.repo.find_by_id(pool, id).await?.ok_or_else(|| anyhow!("Article not found"))
    }

    pub async fn list(&self, pool: &SqlitePool, page: i64, page_size: i64) -> Result<Vec<KnowledgeArticle>> {
        let offset = (page - 1) * page_size;
        self.repo.find_all(pool, page_size, offset).await
    }

    pub async fn publish(&self, pool: &SqlitePool, id: Uuid) -> Result<KnowledgeArticle> {
        let mut article = self.get(pool, id).await?;
        article.status = ArticleStatus::Published;
        article.published_at = Some(Utc::now());
        article.updated_at = Utc::now();
        self.repo.update(pool, &article).await?;
        Ok(article)
    }

    pub async fn archive(&self, pool: &SqlitePool, id: Uuid) -> Result<KnowledgeArticle> {
        let mut article = self.get(pool, id).await?;
        article.status = ArticleStatus::Archived;
        article.updated_at = Utc::now();
        self.repo.update(pool, &article).await?;
        Ok(article)
    }

    pub async fn record_view(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let mut article = self.get(pool, id).await?;
        article.view_count += 1;
        self.repo.update(pool, &article).await
    }

    pub async fn record_feedback(&self, pool: &SqlitePool, id: Uuid, helpful: bool) -> Result<KnowledgeArticle> {
        let mut article = self.get(pool, id).await?;
        if helpful {
            article.helpful_count += 1;
        } else {
            article.not_helpful_count += 1;
        }
        self.repo.update(pool, &article).await?;
        Ok(article)
    }

    pub async fn search(&self, pool: &SqlitePool, query: &str) -> Result<Vec<KnowledgeArticle>> {
        self.repo.search(pool, query).await
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete(pool, id).await
    }
}

pub struct SLAService {
    repo: SqliteSLARepository,
}

impl Default for SLAService {
    fn default() -> Self {
        Self::new()
    }
}

impl SLAService {
    pub fn new() -> Self {
        Self { repo: SqliteSLARepository }
    }

    pub async fn create(&self, pool: &SqlitePool, name: String, description: Option<String>, response_time_hours: i32, resolution_time_hours: i32, business_hours_only: bool, timezone: String) -> Result<SLA> {
        let now = Utc::now();
        let sla = SLA {
            id: Uuid::new_v4(),
            name,
            description,
            response_time_hours,
            resolution_time_hours,
            business_hours_only,
            timezone,
            escalation_rule_id: None,
            status: erp_core::Status::Active,
            created_at: now,
            updated_at: now,
        };
        self.repo.create(pool, &sla).await?;
        Ok(sla)
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<SLA> {
        self.repo.find_by_id(pool, id).await?.ok_or_else(|| anyhow!("SLA not found"))
    }

    pub async fn list(&self, pool: &SqlitePool) -> Result<Vec<SLA>> {
        self.repo.find_all(pool).await
    }

    pub async fn update(&self, pool: &SqlitePool, sla: &SLA) -> Result<()> {
        self.repo.update(pool, sla).await
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete(pool, id).await
    }
}
