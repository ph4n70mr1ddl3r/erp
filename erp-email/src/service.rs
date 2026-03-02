use chrono::Utc;
use erp_core::BaseEntity;
use handlebars::Handlebars;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct EmailTemplateService {
    template_repo: SqliteEmailTemplateRepository,
}

impl Default for EmailTemplateService {
    fn default() -> Self {
        Self::new()
    }
}

impl EmailTemplateService {
    pub fn new() -> Self {
        Self {
            template_repo: SqliteEmailTemplateRepository,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        pool: &SqlitePool,
        name: String,
        code: String,
        category: Option<String>,
        subject: String,
        preheader: Option<String>,
        html_body: String,
        text_body: Option<String>,
        variables: Option<serde_json::Value>,
        created_by: Uuid,
    ) -> anyhow::Result<EmailTemplate> {
        let template = EmailTemplate {
            base: BaseEntity::new(),
            name,
            code,
            category,
            subject,
            preheader,
            html_body,
            text_body,
            variables,
            attachments: None,
            tracking_enabled: true,
            track_opens: true,
            track_clicks: true,
            status: erp_core::Status::Active,
            version: 1,
            created_by,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.template_repo.create(pool, &template).await
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<EmailTemplate>> {
        self.template_repo.get_by_id(pool, id).await
    }

    pub async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> anyhow::Result<Option<EmailTemplate>> {
        self.template_repo.get_by_code(pool, code).await
    }

    pub async fn list(&self, pool: &SqlitePool, category: Option<String>) -> anyhow::Result<Vec<EmailTemplate>> {
        self.template_repo.list(pool, category).await
    }

    pub async fn update(&self, pool: &SqlitePool, id: Uuid, name: String, subject: String, html_body: String, text_body: Option<String>) -> anyhow::Result<()> {
        if let Some(mut template) = self.template_repo.get_by_id(pool, id).await? {
            template.name = name;
            template.subject = subject;
            template.html_body = html_body;
            template.text_body = text_body;
            self.template_repo.update(pool, &template).await?;
        }
        Ok(())
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.template_repo.delete(pool, id).await
    }

    pub fn render(&self, template: &EmailTemplate, variables: &serde_json::Value) -> anyhow::Result<(String, String)> {
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("subject", &template.subject)?;
        handlebars.register_template_string("html", &template.html_body)?;
        
        let subject = handlebars.render("subject", variables)?;
        let html_body = handlebars.render("html", variables)?;
        
        Ok((subject, html_body))
    }
}

#[allow(dead_code)]
pub struct EmailCampaignService {
    campaign_repo: SqliteEmailCampaignRepository,
    template_repo: SqliteEmailTemplateRepository,
    queue_repo: SqliteEmailQueueRepository,
}

impl Default for EmailCampaignService {
    fn default() -> Self {
        Self::new()
    }
}

impl EmailCampaignService {
    pub fn new() -> Self {
        Self {
            campaign_repo: SqliteEmailCampaignRepository,
            template_repo: SqliteEmailTemplateRepository,
            queue_repo: SqliteEmailQueueRepository,
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        pool: &SqlitePool,
        name: String,
        description: Option<String>,
        subject: String,
        html_body: String,
        from_name: String,
        from_email: String,
        list_ids: Vec<Uuid>,
        created_by: Uuid,
    ) -> anyhow::Result<EmailCampaign> {
        let campaign = EmailCampaign {
            base: BaseEntity::new(),
            name,
            description,
            template_id: None,
            subject,
            preheader: None,
            html_body,
            text_body: None,
            from_name,
            from_email,
            reply_to: None,
            list_ids,
            segment_rules: None,
            status: CampaignStatus::Draft,
            scheduled_at: None,
            sent_at: None,
            completed_at: None,
            total_recipients: 0,
            sent_count: 0,
            delivered_count: 0,
            opened_count: 0,
            clicked_count: 0,
            bounced_count: 0,
            unsubscribed_count: 0,
            complaint_count: 0,
            track_opens: true,
            track_clicks: true,
            attachments: None,
            created_by,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.campaign_repo.create(pool, &campaign).await
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<EmailCampaign>> {
        self.campaign_repo.get_by_id(pool, id).await
    }

    pub async fn list(&self, pool: &SqlitePool, status: Option<CampaignStatus>) -> anyhow::Result<Vec<EmailCampaign>> {
        self.campaign_repo.list(pool, status).await
    }

    pub async fn schedule(&self, pool: &SqlitePool, id: Uuid, scheduled_at: chrono::DateTime<Utc>) -> anyhow::Result<()> {
        if let Some(mut campaign) = self.campaign_repo.get_by_id(pool, id).await? {
            campaign.status = CampaignStatus::Scheduled;
            campaign.scheduled_at = Some(scheduled_at);
            self.campaign_repo.update(pool, &campaign).await?;
        }
        Ok(())
    }

    pub async fn send(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        if let Some(mut campaign) = self.campaign_repo.get_by_id(pool, id).await? {
            campaign.status = CampaignStatus::Sending;
            campaign.sent_at = Some(Utc::now());
            self.campaign_repo.update(pool, &campaign).await?;
            
            let recipients = self.get_campaign_recipients(pool, &campaign).await?;
            campaign.total_recipients = recipients.len() as i64;
            
            for recipient in recipients {
                let email = EmailQueue {
                    id: Uuid::new_v4(),
                    campaign_id: Some(id),
                    subscriber_id: recipient.subscriber_id,
                    to_email: recipient.email,
                    to_name: recipient.name,
                    subject: campaign.subject.clone(),
                    html_body: campaign.html_body.clone(),
                    text_body: campaign.text_body.clone(),
                    from_email: campaign.from_email.clone(),
                    from_name: campaign.from_name.clone(),
                    reply_to: campaign.reply_to.clone(),
                    headers: None,
                    attachments: None,
                    variables: recipient.variables,
                    tracking_id: Some(format!("trk_{}", Uuid::new_v4())),
                    status: EmailStatus::Queued,
                    priority: 5,
                    scheduled_at: None,
                    sent_at: None,
                    delivered_at: None,
                    opened_at: None,
                    clicked_at: None,
                    bounced_at: None,
                    bounce_reason: None,
                    retry_count: 0,
                    max_retries: 3,
                    next_retry_at: None,
                    error_message: None,
                    external_id: None,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                };
                self.queue_repo.create(pool, &email).await?;
            }
            
            self.campaign_repo.update(pool, &campaign).await?;
        }
        Ok(())
    }

    async fn get_campaign_recipients(&self, pool: &SqlitePool, campaign: &EmailCampaign) -> anyhow::Result<Vec<CampaignRecipient>> {
        let list_ids = serde_json::to_string(&campaign.list_ids)?;
        sqlx::query_as::<_, CampaignRecipient>(
            r#"SELECT s.id as subscriber_id, s.email, COALESCE(s.first_name || ' ' || s.last_name, s.first_name, s.last_name, s.email) as name, 
               json_object('first_name', s.first_name, 'last_name', s.last_name, 'email', s.email) as variables
               FROM email_subscribers s
               JOIN list_memberships lm ON s.id = lm.subscriber_id
               WHERE lm.list_id IN (SELECT value FROM json_each(?)) AND s.status = 'Active'"#
        )
        .bind(&list_ids)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    pub async fn get_stats(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<CampaignStats>> {
        if let Some(campaign) = self.campaign_repo.get_by_id(pool, id).await? {
            Ok(Some(CampaignStats {
                total_recipients: campaign.total_recipients,
                sent_count: campaign.sent_count,
                delivered_count: campaign.delivered_count,
                opened_count: campaign.opened_count,
                clicked_count: campaign.clicked_count,
                bounced_count: campaign.bounced_count,
                unsubscribed_count: campaign.unsubscribed_count,
                open_rate: if campaign.delivered_count > 0 {
                    (campaign.opened_count as f64 / campaign.delivered_count as f64) * 100.0
                } else { 0.0 },
                click_rate: if campaign.delivered_count > 0 {
                    (campaign.clicked_count as f64 / campaign.delivered_count as f64) * 100.0
                } else { 0.0 },
                bounce_rate: if campaign.sent_count > 0 {
                    (campaign.bounced_count as f64 / campaign.sent_count as f64) * 100.0
                } else { 0.0 },
            }))
        } else {
            Ok(None)
        }
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.campaign_repo.delete(pool, id).await
    }
}

#[derive(Debug, Clone, serde::Serialize, sqlx::FromRow)]
pub struct CampaignRecipient {
    pub subscriber_id: Option<Uuid>,
    pub email: String,
    pub name: Option<String>,
    pub variables: Option<serde_json::Value>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct CampaignStats {
    pub total_recipients: i64,
    pub sent_count: i64,
    pub delivered_count: i64,
    pub opened_count: i64,
    pub clicked_count: i64,
    pub bounced_count: i64,
    pub unsubscribed_count: i64,
    pub open_rate: f64,
    pub click_rate: f64,
    pub bounce_rate: f64,
}

pub struct EmailListService;

impl Default for EmailListService {
    fn default() -> Self {
        Self::new()
    }
}

impl EmailListService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create(
        &self,
        pool: &SqlitePool,
        name: String,
        description: Option<String>,
        list_type: ListType,
        created_by: Uuid,
    ) -> anyhow::Result<EmailList> {
        let now = Utc::now();
        sqlx::query_as::<_, EmailList>(
            r#"INSERT INTO email_lists (
                id, name, description, list_type, subscriber_count, active_count, bounced_count,
                unsubscribed_count, double_optin, welcome_email_id, unsubscribe_page, confirmation_page,
                created_by, created_at, updated_at
            ) VALUES (?, ?, ?, ?, 0, 0, 0, 0, false, NULL, NULL, NULL, ?, ?, ?)
            RETURNING *"#,
        )
        .bind(Uuid::new_v4())
        .bind(&name)
        .bind(&description)
        .bind(&list_type)
        .bind(created_by)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    pub async fn add_subscriber(
        &self,
        pool: &SqlitePool,
        list_id: Uuid,
        email: String,
        first_name: Option<String>,
        last_name: Option<String>,
    ) -> anyhow::Result<EmailSubscriber> {
        let now = Utc::now();
        
        let subscriber = sqlx::query_as::<_, EmailSubscriber>(
            r#"INSERT INTO email_subscribers (
                id, email, first_name, last_name, status, subscribed_at, open_count, click_count, created_at, updated_at
            ) VALUES (?, ?, ?, ?, 'Active', ?, 0, 0, ?, ?)
            RETURNING *"#,
        )
        .bind(Uuid::new_v4())
        .bind(&email)
        .bind(&first_name)
        .bind(&last_name)
        .bind(now)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await?;
        
        sqlx::query(
            "INSERT INTO list_memberships (id, list_id, subscriber_id, added_at) VALUES (?, ?, ?, ?)"
        )
        .bind(Uuid::new_v4())
        .bind(list_id)
        .bind(subscriber.base.id)
        .bind(now)
        .execute(pool)
        .await?;
        
        sqlx::query("UPDATE email_lists SET subscriber_count = subscriber_count + 1, active_count = active_count + 1 WHERE id = ?")
            .bind(list_id)
            .execute(pool)
            .await?;
        
        Ok(subscriber)
    }

    pub async fn unsubscribe(&self, pool: &SqlitePool, list_id: Uuid, email: &str) -> anyhow::Result<()> {
        let now = Utc::now();
        
        sqlx::query("UPDATE email_subscribers SET status = 'Unsubscribed', unsubscribed_at = ?, updated_at = ? WHERE email = ?")
            .bind(now)
            .bind(now)
            .bind(email)
            .execute(pool)
            .await?;
        
        sqlx::query("UPDATE email_lists SET active_count = active_count - 1, unsubscribed_count = unsubscribed_count + 1 WHERE id = ?")
            .bind(list_id)
            .execute(pool)
            .await?;
        
        Ok(())
    }
}
