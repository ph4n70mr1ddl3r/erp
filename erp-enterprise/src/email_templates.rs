use chrono::{DateTime, Utc};
use erp_core::Result;
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub id: Uuid,
    pub name: String,
    pub subject: String,
    pub body: String,
    pub template_type: String,
    pub variables: Option<Vec<String>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEmailTemplateRequest {
    pub name: String,
    pub subject: String,
    pub body: String,
    pub template_type: String,
    pub variables: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateEmailTemplateRequest {
    pub subject: Option<String>,
    pub body: Option<String>,
    pub variables: Option<Vec<String>>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderedEmail {
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailQueueItem {
    pub id: Uuid,
    pub template_id: Option<Uuid>,
    pub to_address: String,
    pub subject: String,
    pub body: String,
    pub status: String,
    pub attempts: i32,
    pub last_error: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueEmailRequest {
    pub template_name: Option<String>,
    pub to_address: String,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub variables: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendEmailRequest {
    pub to_address: String,
    pub subject: String,
    pub body: String,
}

pub struct EmailTemplateService {
    handlebars: Handlebars<'static>,
}

impl EmailTemplateService {
    pub fn new() -> Self {
        Self {
            handlebars: Handlebars::new(),
        }
    }

    pub async fn create_template(
        &self,
        pool: &SqlitePool,
        req: CreateEmailTemplateRequest,
    ) -> Result<EmailTemplate> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let variables_json = req.variables.as_ref().map(|v| serde_json::to_string(v).unwrap_or_default());

        sqlx::query(
            r#"INSERT INTO email_templates 
               (id, name, subject, body, template_type, variables, is_active, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, 1, ?, ?)"#
        )
        .bind(id.to_string())
        .bind(&req.name)
        .bind(&req.subject)
        .bind(&req.body)
        .bind(&req.template_type)
        .bind(&variables_json)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(EmailTemplate {
            id,
            name: req.name,
            subject: req.subject,
            body: req.body,
            template_type: req.template_type,
            variables: req.variables,
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_template(&self, pool: &SqlitePool, name: &str) -> Result<EmailTemplate> {
        let row = sqlx::query_as::<_, EmailTemplateRow>(
            "SELECT id, name, subject, body, template_type, variables, is_active, created_at, updated_at FROM email_templates WHERE name = ?"
        )
        .bind(name)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| erp_core::Error::not_found("EmailTemplate", name))?;

        Ok(row.into_model())
    }

    pub async fn list_templates(&self, pool: &SqlitePool) -> Result<Vec<EmailTemplate>> {
        let rows = sqlx::query_as::<_, EmailTemplateRow>(
            "SELECT id, name, subject, body, template_type, variables, is_active, created_at, updated_at FROM email_templates ORDER BY name"
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into_model()).collect())
    }

    pub async fn update_template(
        &self,
        pool: &SqlitePool,
        name: &str,
        req: UpdateEmailTemplateRequest,
    ) -> Result<EmailTemplate> {
        let mut template = self.get_template(pool, name).await?;
        let now = Utc::now();

        if let Some(subject) = req.subject {
            template.subject = subject;
        }
        if let Some(body) = req.body {
            template.body = body;
        }
        if let Some(variables) = req.variables {
            template.variables = Some(variables);
        }
        if let Some(is_active) = req.is_active {
            template.is_active = is_active;
        }
        template.updated_at = now;

        let variables_json = template.variables.as_ref().map(|v| serde_json::to_string(v).unwrap_or_default());

        sqlx::query(
            r#"UPDATE email_templates 
               SET subject = ?, body = ?, variables = ?, is_active = ?, updated_at = ?
               WHERE name = ?"#
        )
        .bind(&template.subject)
        .bind(&template.body)
        .bind(&variables_json)
        .bind(template.is_active as i32)
        .bind(template.updated_at.to_rfc3339())
        .bind(name)
        .execute(pool)
        .await?;

        Ok(template)
    }

    pub async fn delete_template(&self, pool: &SqlitePool, name: &str) -> Result<()> {
        sqlx::query("DELETE FROM email_templates WHERE name = ?")
            .bind(name)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub fn render_template(&self, template: &EmailTemplate, variables: &JsonValue) -> Result<RenderedEmail> {
        let subject = self
            .handlebars
            .render_template(&template.subject, variables)
            .map_err(|e| erp_core::Error::validation(&format!("Subject template error: {}", e)))?;

        let body = self
            .handlebars
            .render_template(&template.body, variables)
            .map_err(|e| erp_core::Error::validation(&format!("Body template error: {}", e)))?;

        Ok(RenderedEmail { subject, body })
    }

    pub async fn queue_email(&self, pool: &SqlitePool, req: QueueEmailRequest) -> Result<EmailQueueItem> {
        let (subject, body, template_id) = if let Some(template_name) = &req.template_name {
            let template = self.get_template(pool, template_name).await?;
            if !template.is_active {
                return Err(erp_core::Error::validation("Template is not active"));
            }
            let variables = req.variables.unwrap_or(serde_json::json!({}));
            let rendered = self.render_template(&template, &variables)?;
            (rendered.subject, rendered.body, Some(template.id))
        } else if req.subject.is_some() && req.body.is_some() {
            (req.subject.unwrap(), req.body.unwrap(), None)
        } else {
            return Err(erp_core::Error::validation("Either template_name or subject/body must be provided"));
        };

        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"INSERT INTO email_queue 
               (id, template_id, to_address, subject, body, status, attempts, created_at)
               VALUES (?, ?, ?, ?, ?, 'pending', 0, ?)"#
        )
        .bind(id.to_string())
        .bind(template_id.map(|id| id.to_string()))
        .bind(&req.to_address)
        .bind(&subject)
        .bind(&body)
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(EmailQueueItem {
            id,
            template_id,
            to_address: req.to_address,
            subject,
            body,
            status: "pending".to_string(),
            attempts: 0,
            last_error: None,
            sent_at: None,
            created_at: now,
        })
    }

    pub async fn get_pending_emails(&self, pool: &SqlitePool, limit: i32) -> Result<Vec<EmailQueueItem>> {
        let rows = sqlx::query_as::<_, EmailQueueRow>(
            r#"SELECT id, template_id, to_address, subject, body, status, attempts, last_error, sent_at, created_at 
               FROM email_queue 
               WHERE status = 'pending' AND attempts < 3 
               ORDER BY created_at 
               LIMIT ?"#
        )
        .bind(limit)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into_model()).collect())
    }

    pub async fn mark_email_sent(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE email_queue SET status = 'sent', sent_at = ? WHERE id = ?")
            .bind(now.to_rfc3339())
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn mark_email_failed(&self, pool: &SqlitePool, id: Uuid, error: &str) -> Result<()> {
        sqlx::query("UPDATE email_queue SET status = 'failed', attempts = attempts + 1, last_error = ? WHERE id = ?")
            .bind(error)
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get_queue_stats(&self, pool: &SqlitePool) -> Result<EmailQueueStats> {
        let pending: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM email_queue WHERE status = 'pending'")
            .fetch_one(pool)
            .await?;

        let sent: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM email_queue WHERE status = 'sent'")
            .fetch_one(pool)
            .await?;

        let failed: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM email_queue WHERE status = 'failed'")
            .fetch_one(pool)
            .await?;

        Ok(EmailQueueStats { pending, sent, failed })
    }
}

#[derive(Debug, sqlx::FromRow)]
struct EmailTemplateRow {
    id: String,
    name: String,
    subject: String,
    body: String,
    template_type: String,
    variables: Option<String>,
    is_active: i32,
    created_at: String,
    updated_at: String,
}

impl EmailTemplateRow {
    fn into_model(self) -> EmailTemplate {
        EmailTemplate {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            name: self.name,
            subject: self.subject,
            body: self.body,
            template_type: self.template_type,
            variables: self.variables.and_then(|v| serde_json::from_str(&v).ok()),
            is_active: self.is_active != 0,
            created_at: DateTime::parse_from_rfc3339(&self.created_at).ok().map(|d| d.with_timezone(&Utc)).unwrap_or_else(Utc::now),
            updated_at: DateTime::parse_from_rfc3339(&self.updated_at).ok().map(|d| d.with_timezone(&Utc)).unwrap_or_else(Utc::now),
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct EmailQueueRow {
    id: String,
    template_id: Option<String>,
    to_address: String,
    subject: String,
    body: String,
    status: String,
    attempts: i32,
    last_error: Option<String>,
    sent_at: Option<String>,
    created_at: String,
}

impl EmailQueueRow {
    fn into_model(self) -> EmailQueueItem {
        EmailQueueItem {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            template_id: self.template_id.and_then(|id| Uuid::parse_str(&id).ok()),
            to_address: self.to_address,
            subject: self.subject,
            body: self.body,
            status: self.status,
            attempts: self.attempts,
            last_error: self.last_error,
            sent_at: self.sent_at.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            created_at: DateTime::parse_from_rfc3339(&self.created_at).ok().map(|d| d.with_timezone(&Utc)).unwrap_or_else(Utc::now),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailQueueStats {
    pub pending: i64,
    pub sent: i64,
    pub failed: i64,
}
