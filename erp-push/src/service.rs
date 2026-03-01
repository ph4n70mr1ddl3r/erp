use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::*;
use erp_core::Result;

pub struct PushService;

impl Default for PushService {
    fn default() -> Self {
        Self::new()
    }
}

impl PushService {
    pub fn new() -> Self { Self }

    pub async fn register_device(&self, pool: &SqlitePool, user_id: Uuid, device_token: String, 
        platform: String) -> Result<Device> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query(r#"
            INSERT INTO push_devices (id, user_id, device_token, platform, device_name, device_model, 
                os_version, app_version, language, timezone, is_active, last_used_at, push_enabled, 
                badge_count, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(id.to_string())
        .bind(user_id.to_string())
        .bind(&device_token)
        .bind(&platform)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(true)
        .bind(now.to_rfc3339())
        .bind(true)
        .bind(0i32)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(Device {
            id,
            user_id,
            device_token,
            platform,
            device_name: None,
            device_model: None,
            os_version: None,
            app_version: None,
            language: None,
            timezone: None,
            is_active: true,
            last_used_at: Some(now),
            push_enabled: true,
            badge_count: 0,
            created_at: now,
            updated_at: now,
        })
    }
}

#[derive(sqlx::FromRow)]
struct DeviceRow {
    id: String,
    user_id: String,
    device_token: String,
    platform: String,
    device_name: Option<String>,
    device_model: Option<String>,
    os_version: Option<String>,
    app_version: Option<String>,
    language: Option<String>,
    timezone: Option<String>,
    is_active: bool,
    last_used_at: Option<String>,
    push_enabled: bool,
    badge_count: i32,
    created_at: String,
    updated_at: String,
}

impl PushService {
    pub async fn get_user_devices(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Vec<Device>> {
        let rows: Vec<DeviceRow> = 
            sqlx::query_as("SELECT id, user_id, device_token, platform, device_name, device_model, os_version, app_version, language, timezone, is_active, last_used_at, push_enabled, badge_count, created_at, updated_at FROM push_devices WHERE user_id = ? AND is_active = 1")
                .bind(user_id.to_string())
                .fetch_all(pool)
                .await?;

        Ok(rows.into_iter().map(|r| Device {
            id: r.id.parse().unwrap_or_default(),
            user_id: r.user_id.parse().unwrap_or_default(),
            device_token: r.device_token,
            platform: r.platform,
            device_name: r.device_name,
            device_model: r.device_model,
            os_version: r.os_version,
            app_version: r.app_version,
            language: r.language,
            timezone: r.timezone,
            is_active: r.is_active,
            last_used_at: r.last_used_at.and_then(|s| s.parse().ok()),
            push_enabled: r.push_enabled,
            badge_count: r.badge_count,
            created_at: r.created_at.parse().unwrap_or_default(),
            updated_at: r.updated_at.parse().unwrap_or_default(),
        }).collect())
    }

    pub async fn send_notification(&self, pool: &SqlitePool, title: String, body: String, 
        user_ids: Vec<Uuid>, data: Option<serde_json::Value>) -> Result<PushNotification> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query(r#"
            INSERT INTO push_notifications (id, title, body, icon, image, sound, badge, priority, 
                data, action_url, category, ttl_seconds, scheduled_at, sent_at, expires_at, status, 
                created_by, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(id.to_string())
        .bind(&title)
        .bind(&body)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind("default")
        .bind(Option::<i32>::None)
        .bind("Normal")
        .bind(&data)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(86400i32)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind("pending")
        .bind(Option::<String>::None)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        for user_id in user_ids {
            sqlx::query(r#"
                INSERT INTO push_notification_recipients (id, notification_id, user_id, device_id, 
                    status, sent_at, delivered_at, opened_at, error_message, external_id, created_at, updated_at)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#)
            .bind(Uuid::new_v4().to_string())
            .bind(id.to_string())
            .bind(user_id.to_string())
            .bind(Option::<String>::None)
            .bind("pending")
            .bind(Option::<String>::None)
            .bind(Option::<String>::None)
            .bind(Option::<String>::None)
            .bind(Option::<String>::None)
            .bind(Option::<String>::None)
            .bind(now.to_rfc3339())
            .bind(now.to_rfc3339())
            .execute(pool)
            .await?;
        }

        Ok(PushNotification {
            id,
            title,
            body,
            icon: None,
            image: None,
            sound: Some("default".to_string()),
            badge: None,
            priority: "Normal".to_string(),
            data,
            action_url: None,
            category: None,
            ttl_seconds: Some(86400),
            scheduled_at: None,
            sent_at: None,
            expires_at: None,
            status: "pending".to_string(),
            created_by: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn send_to_all(&self, pool: &SqlitePool, title: String, body: String, 
        data: Option<serde_json::Value>) -> Result<PushNotification> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query(r#"
            INSERT INTO push_notifications (id, title, body, icon, image, sound, badge, priority, 
                data, action_url, category, ttl_seconds, scheduled_at, sent_at, expires_at, status, 
                created_by, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(id.to_string())
        .bind(&title)
        .bind(&body)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind("default")
        .bind(Option::<i32>::None)
        .bind("Normal")
        .bind(&data)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(86400i32)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind("broadcast")
        .bind(Option::<String>::None)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(PushNotification {
            id,
            title,
            body,
            icon: None,
            image: None,
            sound: Some("default".to_string()),
            badge: None,
            priority: "Normal".to_string(),
            data,
            action_url: None,
            category: None,
            ttl_seconds: Some(86400),
            scheduled_at: None,
            sent_at: None,
            expires_at: None,
            status: "broadcast".to_string(),
            created_by: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn create_template(&self, pool: &SqlitePool, code: String, name: String, 
        title_template: String, body_template: String, category: String) -> Result<PushTemplate> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query(r#"
            INSERT INTO push_templates (id, code, name, description, title_template, body_template, 
                default_data, category, priority, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(id.to_string())
        .bind(&code)
        .bind(&name)
        .bind(Option::<String>::None)
        .bind(&title_template)
        .bind(&body_template)
        .bind(Option::<serde_json::Value>::None)
        .bind(&category)
        .bind("Normal")
        .bind(true)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(PushTemplate {
            id,
            code,
            name,
            description: None,
            title_template,
            body_template,
            default_data: None,
            category,
            priority: "Normal".to_string(),
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn set_preference(&self, pool: &SqlitePool, user_id: Uuid, category: String, 
        push_enabled: bool) -> Result<NotificationPreference> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query(r#"
            INSERT INTO push_preferences (id, user_id, category, push_enabled, email_enabled, 
                sms_enabled, in_app_enabled, quiet_hours_start, quiet_hours_end, quiet_hours_timezone, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(user_id, category) DO UPDATE SET push_enabled = excluded.push_enabled, updated_at = excluded.updated_at
        "#)
        .bind(id.to_string())
        .bind(user_id.to_string())
        .bind(&category)
        .bind(push_enabled)
        .bind(true)
        .bind(false)
        .bind(true)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(NotificationPreference {
            id,
            user_id,
            category,
            push_enabled,
            email_enabled: true,
            sms_enabled: false,
            in_app_enabled: true,
            quiet_hours_start: None,
            quiet_hours_end: None,
            quiet_hours_timezone: None,
            created_at: now,
            updated_at: now,
        })
    }
}
