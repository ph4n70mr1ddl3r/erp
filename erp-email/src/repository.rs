use async_trait::async_trait;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait EmailTemplateRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, template: &EmailTemplate) -> anyhow::Result<EmailTemplate>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<EmailTemplate>>;
    async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> anyhow::Result<Option<EmailTemplate>>;
    async fn list(&self, pool: &SqlitePool, category: Option<String>) -> anyhow::Result<Vec<EmailTemplate>>;
    async fn update(&self, pool: &SqlitePool, template: &EmailTemplate) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteEmailTemplateRepository;

#[async_trait]
impl EmailTemplateRepository for SqliteEmailTemplateRepository {
    async fn create(&self, pool: &SqlitePool, template: &EmailTemplate) -> anyhow::Result<EmailTemplate> {
        let now = Utc::now();
        sqlx::query_as::<_, EmailTemplate>(
            r#"INSERT INTO email_templates (
                id, name, code, category, subject, preheader, html_body, text_body,
                variables, attachments, tracking_enabled, track_opens, track_clicks,
                status, version, created_by, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(template.base.id)
        .bind(&template.name)
        .bind(&template.code)
        .bind(&template.category)
        .bind(&template.subject)
        .bind(&template.preheader)
        .bind(&template.html_body)
        .bind(&template.text_body)
        .bind(&template.variables)
        .bind(&template.attachments)
        .bind(template.tracking_enabled)
        .bind(template.track_opens)
        .bind(template.track_clicks)
        .bind(&template.status)
        .bind(template.version)
        .bind(template.created_by)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<EmailTemplate>> {
        sqlx::query_as::<_, EmailTemplate>("SELECT * FROM email_templates WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> anyhow::Result<Option<EmailTemplate>> {
        sqlx::query_as::<_, EmailTemplate>("SELECT * FROM email_templates WHERE code = ? AND status = 'Active'")
            .bind(code)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list(&self, pool: &SqlitePool, category: Option<String>) -> anyhow::Result<Vec<EmailTemplate>> {
        let query = if category.is_some() {
            "SELECT * FROM email_templates WHERE category = ? ORDER BY name"
        } else {
            "SELECT * FROM email_templates ORDER BY name"
        };
        
        let mut q = sqlx::query_as::<_, EmailTemplate>(query);
        if let Some(cat) = category { q = q.bind(cat); }
        q.fetch_all(pool).await.map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, template: &EmailTemplate) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE email_templates SET name=?, subject=?, html_body=?, text_body=?, status=?, version=version+1, updated_at=? WHERE id=?")
            .bind(&template.name)
            .bind(&template.subject)
            .bind(&template.html_body)
            .bind(&template.text_body)
            .bind(&template.status)
            .bind(now)
            .bind(template.base.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM email_templates WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait EmailCampaignRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, campaign: &EmailCampaign) -> anyhow::Result<EmailCampaign>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<EmailCampaign>>;
    async fn list(&self, pool: &SqlitePool, status: Option<CampaignStatus>) -> anyhow::Result<Vec<EmailCampaign>>;
    async fn update(&self, pool: &SqlitePool, campaign: &EmailCampaign) -> anyhow::Result<()>;
    async fn update_stats(&self, pool: &SqlitePool, id: Uuid, sent: i64, delivered: i64, opened: i64, clicked: i64, bounced: i64) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteEmailCampaignRepository;

#[async_trait]
impl EmailCampaignRepository for SqliteEmailCampaignRepository {
    async fn create(&self, pool: &SqlitePool, campaign: &EmailCampaign) -> anyhow::Result<EmailCampaign> {
        let now = Utc::now();
        sqlx::query_as::<_, EmailCampaign>(
            r#"INSERT INTO email_campaigns (
                id, name, description, template_id, subject, preheader, html_body, text_body,
                from_name, from_email, reply_to, list_ids, segment_rules, status, scheduled_at,
                sent_at, completed_at, total_recipients, sent_count, delivered_count, opened_count,
                clicked_count, bounced_count, unsubscribed_count, complaint_count, track_opens,
                track_clicks, attachments, created_by, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(campaign.base.id)
        .bind(&campaign.name)
        .bind(&campaign.description)
        .bind(campaign.template_id)
        .bind(&campaign.subject)
        .bind(&campaign.preheader)
        .bind(&campaign.html_body)
        .bind(&campaign.text_body)
        .bind(&campaign.from_name)
        .bind(&campaign.from_email)
        .bind(&campaign.reply_to)
        .bind(serde_json::to_string(&campaign.list_ids)?)
        .bind(&campaign.segment_rules)
        .bind(&campaign.status)
        .bind(campaign.scheduled_at)
        .bind(campaign.sent_at)
        .bind(campaign.completed_at)
        .bind(campaign.total_recipients)
        .bind(campaign.sent_count)
        .bind(campaign.delivered_count)
        .bind(campaign.opened_count)
        .bind(campaign.clicked_count)
        .bind(campaign.bounced_count)
        .bind(campaign.unsubscribed_count)
        .bind(campaign.complaint_count)
        .bind(campaign.track_opens)
        .bind(campaign.track_clicks)
        .bind(&campaign.attachments)
        .bind(campaign.created_by)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<EmailCampaign>> {
        sqlx::query_as::<_, EmailCampaign>("SELECT * FROM email_campaigns WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list(&self, pool: &SqlitePool, status: Option<CampaignStatus>) -> anyhow::Result<Vec<EmailCampaign>> {
        let query = if status.is_some() {
            "SELECT * FROM email_campaigns WHERE status = ? ORDER BY created_at DESC"
        } else {
            "SELECT * FROM email_campaigns ORDER BY created_at DESC"
        };
        
        let mut q = sqlx::query_as::<_, EmailCampaign>(query);
        if let Some(s) = status { q = q.bind(s); }
        q.fetch_all(pool).await.map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, campaign: &EmailCampaign) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE email_campaigns SET name=?, description=?, status=?, updated_at=? WHERE id=?")
            .bind(&campaign.name)
            .bind(&campaign.description)
            .bind(&campaign.status)
            .bind(now)
            .bind(campaign.base.id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn update_stats(&self, pool: &SqlitePool, id: Uuid, sent: i64, delivered: i64, opened: i64, clicked: i64, bounced: i64) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE email_campaigns SET sent_count=?, delivered_count=?, opened_count=?, clicked_count=?, bounced_count=?, updated_at=? WHERE id=?")
            .bind(sent)
            .bind(delivered)
            .bind(opened)
            .bind(clicked)
            .bind(bounced)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM email_campaigns WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait EmailQueueRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, email: &EmailQueue) -> anyhow::Result<EmailQueue>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<EmailQueue>>;
    async fn list_pending(&self, pool: &SqlitePool, limit: i32) -> anyhow::Result<Vec<EmailQueue>>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: EmailStatus, error: Option<String>) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteEmailQueueRepository;

#[async_trait]
impl EmailQueueRepository for SqliteEmailQueueRepository {
    async fn create(&self, pool: &SqlitePool, email: &EmailQueue) -> anyhow::Result<EmailQueue> {
        let now = Utc::now();
        sqlx::query_as::<_, EmailQueue>(
            r#"INSERT INTO email_queue (
                id, campaign_id, subscriber_id, to_email, to_name, subject, html_body, text_body,
                from_email, from_name, reply_to, headers, attachments, variables, tracking_id,
                status, priority, scheduled_at, sent_at, delivered_at, opened_at, clicked_at,
                bounced_at, bounce_reason, retry_count, max_retries, next_retry_at, error_message,
                external_id, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(email.id)
        .bind(email.campaign_id)
        .bind(email.subscriber_id)
        .bind(&email.to_email)
        .bind(&email.to_name)
        .bind(&email.subject)
        .bind(&email.html_body)
        .bind(&email.text_body)
        .bind(&email.from_email)
        .bind(&email.from_name)
        .bind(&email.reply_to)
        .bind(&email.headers)
        .bind(&email.attachments)
        .bind(&email.variables)
        .bind(&email.tracking_id)
        .bind(&email.status)
        .bind(email.priority)
        .bind(email.scheduled_at)
        .bind(email.sent_at)
        .bind(email.delivered_at)
        .bind(email.opened_at)
        .bind(email.clicked_at)
        .bind(email.bounced_at)
        .bind(&email.bounce_reason)
        .bind(email.retry_count)
        .bind(email.max_retries)
        .bind(email.next_retry_at)
        .bind(&email.error_message)
        .bind(&email.external_id)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<EmailQueue>> {
        sqlx::query_as::<_, EmailQueue>("SELECT * FROM email_queue WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_pending(&self, pool: &SqlitePool, limit: i32) -> anyhow::Result<Vec<EmailQueue>> {
        sqlx::query_as::<_, EmailQueue>("SELECT * FROM email_queue WHERE status = 'Queued' AND (scheduled_at IS NULL OR scheduled_at <= ?) ORDER BY priority DESC, created_at ASC LIMIT ?")
            .bind(Utc::now())
            .bind(limit)
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: EmailStatus, error: Option<String>) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE email_queue SET status=?, error_message=?, updated_at=? WHERE id=?")
            .bind(&status)
            .bind(&error)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM email_queue WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
