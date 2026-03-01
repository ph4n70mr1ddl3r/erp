use async_trait::async_trait;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait NotificationRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, notification: &Notification) -> anyhow::Result<Notification>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<Notification>>;
    async fn list_by_user(&self, pool: &SqlitePool, user_id: Uuid, limit: i32, offset: i32) -> anyhow::Result<Vec<Notification>>;
    async fn list_unread(&self, pool: &SqlitePool, user_id: Uuid) -> anyhow::Result<Vec<Notification>>;
    async fn mark_read(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
    async fn mark_all_read(&self, pool: &SqlitePool, user_id: Uuid) -> anyhow::Result<()>;
    async fn dismiss(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: NotificationStatus) -> anyhow::Result<()>;
    async fn count_unread(&self, pool: &SqlitePool, user_id: Uuid) -> anyhow::Result<i64>;
}

pub struct SqliteNotificationRepository;

#[async_trait]
impl NotificationRepository for SqliteNotificationRepository {
    async fn create(&self, pool: &SqlitePool, notification: &Notification) -> anyhow::Result<Notification> {
        let now = Utc::now();
        sqlx::query_as::<_, Notification>(
            r#"
            INSERT INTO notifications (
                id, user_id, notification_type, channel, priority, title, body,
                action_url, action_text, icon, image_url, data, template_id,
                related_entity_type, related_entity_id, scheduled_at, sent_at,
                delivered_at, read_at, dismissed_at, status, retry_count, max_retries,
                last_error, expires_at, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(notification.base.id)
        .bind(notification.user_id)
        .bind(&notification.notification_type)
        .bind(&notification.channel)
        .bind(&notification.priority)
        .bind(&notification.title)
        .bind(&notification.body)
        .bind(&notification.action_url)
        .bind(&notification.action_text)
        .bind(&notification.icon)
        .bind(&notification.image_url)
        .bind(&notification.data)
        .bind(notification.template_id)
        .bind(&notification.related_entity_type)
        .bind(notification.related_entity_id)
        .bind(notification.scheduled_at)
        .bind(notification.sent_at)
        .bind(notification.delivered_at)
        .bind(notification.read_at)
        .bind(notification.dismissed_at)
        .bind(&notification.status)
        .bind(notification.retry_count)
        .bind(notification.max_retries)
        .bind(&notification.last_error)
        .bind(notification.expires_at)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<Notification>> {
        sqlx::query_as::<_, Notification>("SELECT * FROM notifications WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list_by_user(&self, pool: &SqlitePool, user_id: Uuid, limit: i32, offset: i32) -> anyhow::Result<Vec<Notification>> {
        sqlx::query_as::<_, Notification>(
            "SELECT * FROM notifications WHERE user_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(user_id)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn list_unread(&self, pool: &SqlitePool, user_id: Uuid) -> anyhow::Result<Vec<Notification>> {
        sqlx::query_as::<_, Notification>(
            "SELECT * FROM notifications WHERE user_id = ? AND status = 'Sent' ORDER BY created_at DESC"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn mark_read(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE notifications SET status = 'Read', read_at = ?, updated_at = ? WHERE id = ?")
            .bind(now)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn mark_all_read(&self, pool: &SqlitePool, user_id: Uuid) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE notifications SET status = 'Read', read_at = ?, updated_at = ? WHERE user_id = ? AND status = 'Sent'")
            .bind(now)
            .bind(now)
            .bind(user_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn dismiss(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE notifications SET status = 'Dismissed', dismissed_at = ?, updated_at = ? WHERE id = ?")
            .bind(now)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM notifications WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: NotificationStatus) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query("UPDATE notifications SET status = ?, updated_at = ? WHERE id = ?")
            .bind(&status)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn count_unread(&self, pool: &SqlitePool, user_id: Uuid) -> anyhow::Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM notifications WHERE user_id = ? AND status = 'Sent'"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;
        Ok(count.0)
    }
}

#[async_trait]
pub trait NotificationPreferenceRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, pref: &NotificationPreference) -> anyhow::Result<NotificationPreference>;
    async fn get_by_user_and_type(&self, pool: &SqlitePool, user_id: Uuid, notification_type: NotificationType, channel: NotificationChannel) -> anyhow::Result<Option<NotificationPreference>>;
    async fn list_by_user(&self, pool: &SqlitePool, user_id: Uuid) -> anyhow::Result<Vec<NotificationPreference>>;
    async fn update(&self, pool: &SqlitePool, pref: &NotificationPreference) -> anyhow::Result<()>;
}

pub struct SqliteNotificationPreferenceRepository;

#[async_trait]
impl NotificationPreferenceRepository for SqliteNotificationPreferenceRepository {
    async fn create(&self, pool: &SqlitePool, pref: &NotificationPreference) -> anyhow::Result<NotificationPreference> {
        let now = Utc::now();
        sqlx::query_as::<_, NotificationPreference>(
            r#"
            INSERT INTO notification_preferences (
                id, user_id, notification_type, channel, enabled, priority_threshold,
                quiet_hours_start, quiet_hours_end, quiet_hours_timezone,
                digest_enabled, digest_frequency, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(pref.base.id)
        .bind(pref.user_id)
        .bind(&pref.notification_type)
        .bind(&pref.channel)
        .bind(pref.enabled)
        .bind(&pref.priority_threshold)
        .bind(&pref.quiet_hours_start)
        .bind(&pref.quiet_hours_end)
        .bind(&pref.quiet_hours_timezone)
        .bind(pref.digest_enabled)
        .bind(&pref.digest_frequency)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_user_and_type(&self, pool: &SqlitePool, user_id: Uuid, notification_type: NotificationType, channel: NotificationChannel) -> anyhow::Result<Option<NotificationPreference>> {
        sqlx::query_as::<_, NotificationPreference>(
            "SELECT * FROM notification_preferences WHERE user_id = ? AND notification_type = ? AND channel = ?"
        )
        .bind(user_id)
        .bind(&notification_type)
        .bind(&channel)
        .fetch_optional(pool)
        .await
        .map_err(Into::into)
    }

    async fn list_by_user(&self, pool: &SqlitePool, user_id: Uuid) -> anyhow::Result<Vec<NotificationPreference>> {
        sqlx::query_as::<_, NotificationPreference>(
            "SELECT * FROM notification_preferences WHERE user_id = ?"
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, pref: &NotificationPreference) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query(
            r#"
            UPDATE notification_preferences SET
                enabled = ?, priority_threshold = ?, quiet_hours_start = ?,
                quiet_hours_end = ?, quiet_hours_timezone = ?, digest_enabled = ?,
                digest_frequency = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(pref.enabled)
        .bind(&pref.priority_threshold)
        .bind(&pref.quiet_hours_start)
        .bind(&pref.quiet_hours_end)
        .bind(&pref.quiet_hours_timezone)
        .bind(pref.digest_enabled)
        .bind(&pref.digest_frequency)
        .bind(now)
        .bind(pref.base.id)
        .execute(pool)
        .await?;
        Ok(())
    }
}

#[async_trait]
pub trait NotificationTemplateRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, template: &NotificationTemplate) -> anyhow::Result<NotificationTemplate>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<NotificationTemplate>>;
    async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> anyhow::Result<Option<NotificationTemplate>>;
    async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<NotificationTemplate>>;
    async fn update(&self, pool: &SqlitePool, template: &NotificationTemplate) -> anyhow::Result<()>;
}

pub struct SqliteNotificationTemplateRepository;

#[async_trait]
impl NotificationTemplateRepository for SqliteNotificationTemplateRepository {
    async fn create(&self, pool: &SqlitePool, template: &NotificationTemplate) -> anyhow::Result<NotificationTemplate> {
        let now = Utc::now();
        sqlx::query_as::<_, NotificationTemplate>(
            r#"
            INSERT INTO notification_templates (
                id, name, code, notification_type, channel, subject_template,
                body_template, html_template, variables, default_priority, status, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(template.base.id)
        .bind(&template.name)
        .bind(&template.code)
        .bind(&template.notification_type)
        .bind(&template.channel)
        .bind(&template.subject_template)
        .bind(&template.body_template)
        .bind(&template.html_template)
        .bind(&template.variables)
        .bind(&template.default_priority)
        .bind(&template.status)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<NotificationTemplate>> {
        sqlx::query_as::<_, NotificationTemplate>("SELECT * FROM notification_templates WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> anyhow::Result<Option<NotificationTemplate>> {
        sqlx::query_as::<_, NotificationTemplate>("SELECT * FROM notification_templates WHERE code = ?")
            .bind(code)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<NotificationTemplate>> {
        sqlx::query_as::<_, NotificationTemplate>("SELECT * FROM notification_templates ORDER BY name")
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, template: &NotificationTemplate) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query(
            r#"
            UPDATE notification_templates SET
                name = ?, subject_template = ?, body_template = ?, html_template = ?,
                variables = ?, default_priority = ?, status = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&template.name)
        .bind(&template.subject_template)
        .bind(&template.body_template)
        .bind(&template.html_template)
        .bind(&template.variables)
        .bind(&template.default_priority)
        .bind(&template.status)
        .bind(now)
        .bind(template.base.id)
        .execute(pool)
        .await?;
        Ok(())
    }
}
