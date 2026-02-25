use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Device {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_token: String,
    pub platform: String,
    pub device_name: Option<String>,
    pub device_model: Option<String>,
    pub os_version: Option<String>,
    pub app_version: Option<String>,
    pub language: Option<String>,
    pub timezone: Option<String>,
    pub is_active: bool,
    pub last_used_at: Option<DateTime<Utc>>,
    pub push_enabled: bool,
    pub badge_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushNotification {
    pub id: Uuid,
    pub title: String,
    pub body: String,
    pub icon: Option<String>,
    pub image: Option<String>,
    pub sound: Option<String>,
    pub badge: Option<i32>,
    pub priority: String,
    pub data: Option<serde_json::Value>,
    pub action_url: Option<String>,
    pub category: Option<String>,
    pub ttl_seconds: Option<i32>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub sent_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: String,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushTemplate {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub title_template: String,
    pub body_template: String,
    pub default_data: Option<serde_json::Value>,
    pub category: String,
    pub priority: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreference {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category: String,
    pub push_enabled: bool,
    pub email_enabled: bool,
    pub sms_enabled: bool,
    pub in_app_enabled: bool,
    pub quiet_hours_start: Option<String>,
    pub quiet_hours_end: Option<String>,
    pub quiet_hours_timezone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
