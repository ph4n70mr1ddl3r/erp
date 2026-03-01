use erp_core::BaseEntity;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct NotificationService {
    notification_repo: SqliteNotificationRepository,
    preference_repo: SqliteNotificationPreferenceRepository,
    template_repo: SqliteNotificationTemplateRepository,
}

impl Default for NotificationService {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationService {
    pub fn new() -> Self {
        Self {
            notification_repo: SqliteNotificationRepository,
            preference_repo: SqliteNotificationPreferenceRepository,
            template_repo: SqliteNotificationTemplateRepository,
        }
    }

    pub async fn send(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        notification_type: NotificationType,
        channel: NotificationChannel,
        title: String,
        body: String,
        action_url: Option<String>,
        action_text: Option<String>,
        priority: Option<NotificationPriority>,
        data: Option<serde_json::Value>,
    ) -> anyhow::Result<Notification> {
        let priority = priority.unwrap_or(NotificationPriority::Normal);
        
        let notification = Notification {
            base: BaseEntity::new(),
            user_id,
            notification_type,
            channel,
            priority,
            title,
            body,
            action_url,
            action_text,
            icon: None,
            image_url: None,
            data,
            template_id: None,
            related_entity_type: None,
            related_entity_id: None,
            scheduled_at: None,
            sent_at: Some(chrono::Utc::now()),
            delivered_at: None,
            read_at: None,
            dismissed_at: None,
            status: NotificationStatus::Sent,
            retry_count: 0,
            max_retries: 3,
            last_error: None,
            expires_at: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        self.notification_repo.create(pool, &notification).await
    }

    pub async fn send_with_template(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        template_code: &str,
        variables: serde_json::Value,
    ) -> anyhow::Result<Notification> {
        let template = self.template_repo.get_by_code(pool, template_code).await?
            .ok_or_else(|| anyhow::anyhow!("Template not found: {}", template_code))?;
        
        let title = render_template(&template.subject_template.clone().unwrap_or_default(), &variables)?;
        let body = render_template(&template.body_template, &variables)?;
        
        self.send(
            pool,
            user_id,
            template.notification_type,
            template.channel,
            title,
            body,
            None,
            None,
            Some(template.default_priority),
            Some(variables),
        ).await
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<Notification>> {
        self.notification_repo.get_by_id(pool, id).await
    }

    pub async fn list_for_user(&self, pool: &SqlitePool, user_id: Uuid, limit: i32, offset: i32) -> anyhow::Result<Vec<Notification>> {
        self.notification_repo.list_by_user(pool, user_id, limit, offset).await
    }

    pub async fn list_unread(&self, pool: &SqlitePool, user_id: Uuid) -> anyhow::Result<Vec<Notification>> {
        self.notification_repo.list_unread(pool, user_id).await
    }

    pub async fn mark_read(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.notification_repo.mark_read(pool, id).await
    }

    pub async fn mark_all_read(&self, pool: &SqlitePool, user_id: Uuid) -> anyhow::Result<()> {
        self.notification_repo.mark_all_read(pool, user_id).await
    }

    pub async fn dismiss(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.notification_repo.dismiss(pool, id).await
    }

    pub async fn unread_count(&self, pool: &SqlitePool, user_id: Uuid) -> anyhow::Result<i64> {
        self.notification_repo.count_unread(pool, user_id).await
    }

    pub async fn schedule(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        notification_type: NotificationType,
        channel: NotificationChannel,
        title: String,
        body: String,
        scheduled_at: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<Notification> {
        let notification = Notification {
            base: BaseEntity::new(),
            user_id,
            notification_type,
            channel,
            priority: NotificationPriority::Normal,
            title,
            body,
            action_url: None,
            action_text: None,
            icon: None,
            image_url: None,
            data: None,
            template_id: None,
            related_entity_type: None,
            related_entity_id: None,
            scheduled_at: Some(scheduled_at),
            sent_at: None,
            delivered_at: None,
            read_at: None,
            dismissed_at: None,
            status: NotificationStatus::Pending,
            retry_count: 0,
            max_retries: 3,
            last_error: None,
            expires_at: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        self.notification_repo.create(pool, &notification).await
    }

    pub async fn broadcast(
        &self,
        pool: &SqlitePool,
        user_ids: Vec<Uuid>,
        notification_type: NotificationType,
        channel: NotificationChannel,
        title: String,
        body: String,
    ) -> anyhow::Result<Vec<Notification>> {
        let mut notifications = Vec::new();
        for user_id in user_ids {
            let notification = self.send(
                pool,
                user_id,
                notification_type.clone(),
                channel.clone(),
                title.clone(),
                body.clone(),
                None,
                None,
                None,
                None,
            ).await?;
            notifications.push(notification);
        }
        Ok(notifications)
    }
}

fn render_template(template: &str, variables: &serde_json::Value) -> anyhow::Result<String> {
    let mut result = template.to_string();
    if let serde_json::Value::Object(map) = variables {
        for (key, value) in map {
            let placeholder = format!("{{{{{}}}}}", key);
            let replacement = match value {
                serde_json::Value::String(s) => s.clone(),
                _ => value.to_string(),
            };
            result = result.replace(&placeholder, &replacement);
        }
    }
    Ok(result)
}

pub struct NotificationPreferenceService {
    preference_repo: SqliteNotificationPreferenceRepository,
}

impl Default for NotificationPreferenceService {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationPreferenceService {
    pub fn new() -> Self {
        Self {
            preference_repo: SqliteNotificationPreferenceRepository,
        }
    }

    pub async fn set_preference(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        notification_type: NotificationType,
        channel: NotificationChannel,
        enabled: bool,
    ) -> anyhow::Result<NotificationPreference> {
        let existing = self.preference_repo.get_by_user_and_type(pool, user_id, notification_type.clone(), channel.clone()).await?;
        
        if let Some(mut pref) = existing {
            pref.enabled = enabled;
            self.preference_repo.update(pool, &pref).await?;
            Ok(pref)
        } else {
            let pref = NotificationPreference {
                base: BaseEntity::new(),
                user_id,
                notification_type,
                channel,
                enabled,
                priority_threshold: NotificationPriority::Normal,
                quiet_hours_start: None,
                quiet_hours_end: None,
                quiet_hours_timezone: None,
                digest_enabled: false,
                digest_frequency: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            };
            self.preference_repo.create(pool, &pref).await
        }
    }

    pub async fn get_preferences(&self, pool: &SqlitePool, user_id: Uuid) -> anyhow::Result<Vec<NotificationPreference>> {
        self.preference_repo.list_by_user(pool, user_id).await
    }

    pub async fn is_notification_enabled(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        notification_type: NotificationType,
        channel: NotificationChannel,
    ) -> anyhow::Result<bool> {
        let pref = self.preference_repo.get_by_user_and_type(pool, user_id, notification_type, channel).await?;
        Ok(pref.map(|p| p.enabled).unwrap_or(true))
    }
}

pub struct NotificationTemplateService {
    template_repo: SqliteNotificationTemplateRepository,
}

impl Default for NotificationTemplateService {
    fn default() -> Self {
        Self::new()
    }
}

impl NotificationTemplateService {
    pub fn new() -> Self {
        Self {
            template_repo: SqliteNotificationTemplateRepository,
        }
    }

    pub async fn create(
        &self,
        pool: &SqlitePool,
        name: String,
        code: String,
        notification_type: NotificationType,
        channel: NotificationChannel,
        subject_template: Option<String>,
        body_template: String,
        html_template: Option<String>,
        variables: Option<serde_json::Value>,
    ) -> anyhow::Result<NotificationTemplate> {
        let template = NotificationTemplate {
            base: BaseEntity::new(),
            name,
            code,
            notification_type,
            channel,
            subject_template,
            body_template,
            html_template,
            variables,
            default_priority: NotificationPriority::Normal,
            status: erp_core::Status::Active,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        
        self.template_repo.create(pool, &template).await
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<NotificationTemplate>> {
        self.template_repo.get_by_id(pool, id).await
    }

    pub async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> anyhow::Result<Option<NotificationTemplate>> {
        self.template_repo.get_by_code(pool, code).await
    }

    pub async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<NotificationTemplate>> {
        self.template_repo.list(pool).await
    }
}
