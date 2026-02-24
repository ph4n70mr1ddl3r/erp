use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum NotificationChannel {
    InApp,
    Email,
    SMS,
    Push,
    Slack,
    Teams,
    Webhook,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum NotificationPriority {
    Low,
    Normal,
    High,
    Urgent,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum NotificationStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
    Read,
    Dismissed,
    Bounced,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum NotificationType {
    System,
    Alert,
    Reminder,
    Approval,
    Task,
    Message,
    Event,
    Report,
    Workflow,
    Security,
    Billing,
    Update,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub base: BaseEntity,
    pub user_id: Uuid,
    pub notification_type: NotificationType,
    pub channel: NotificationChannel,
    pub priority: NotificationPriority,
    pub title: String,
    pub body: String,
    pub action_url: Option<String>,
    pub action_text: Option<String>,
    pub icon: Option<String>,
    pub image_url: Option<String>,
    pub data: Option<serde_json::Value>,
    pub template_id: Option<Uuid>,
    pub related_entity_type: Option<String>,
    pub related_entity_id: Option<Uuid>,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
    pub dismissed_at: Option<DateTime<Utc>>,
    pub status: NotificationStatus,
    pub retry_count: i32,
    pub max_retries: i32,
    pub last_error: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationPreference {
    pub base: BaseEntity,
    pub user_id: Uuid,
    pub notification_type: NotificationType,
    pub channel: NotificationChannel,
    pub enabled: bool,
    pub priority_threshold: NotificationPriority,
    pub quiet_hours_start: Option<String>,
    pub quiet_hours_end: Option<String>,
    pub quiet_hours_timezone: Option<String>,
    pub digest_enabled: bool,
    pub digest_frequency: Option<DigestFrequency>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DigestFrequency {
    Immediate,
    Hourly,
    Daily,
    Weekly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationTemplate {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub notification_type: NotificationType,
    pub channel: NotificationChannel,
    pub subject_template: Option<String>,
    pub body_template: String,
    pub html_template: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub default_priority: NotificationPriority,
    pub status: erp_core::Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationBatch {
    pub base: BaseEntity,
    pub batch_number: String,
    pub name: String,
    pub description: Option<String>,
    pub channel: NotificationChannel,
    pub template_id: Option<Uuid>,
    pub total_recipients: i32,
    pub sent_count: i32,
    pub delivered_count: i32,
    pub failed_count: i32,
    pub opened_count: i32,
    pub clicked_count: i32,
    pub status: BatchStatus,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BatchStatus {
    Draft,
    Scheduled,
    Processing,
    Completed,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationDeliveryLog {
    pub id: Uuid,
    pub notification_id: Uuid,
    pub batch_id: Option<Uuid>,
    pub channel: NotificationChannel,
    pub provider: String,
    pub external_id: Option<String>,
    pub status: NotificationStatus,
    pub response_code: Option<i32>,
    pub response_message: Option<String>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub opened_at: Option<DateTime<Utc>>,
    pub clicked_at: Option<DateTime<Utc>>,
    pub bounced_at: Option<DateTime<Utc>>,
    pub bounce_reason: Option<String>,
    pub retry_count: i32,
    pub next_retry_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationProvider {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub channel: NotificationChannel,
    pub provider_type: String,
    pub configuration: Option<serde_json::Value>,
    pub credentials_encrypted: Option<String>,
    pub webhook_url: Option<String>,
    pub is_default: bool,
    pub daily_limit: Option<i32>,
    pub daily_used: i32,
    pub monthly_limit: Option<i32>,
    pub monthly_used: i32,
    pub status: erp_core::Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationQueue {
    pub id: Uuid,
    pub notification_id: Uuid,
    pub priority: NotificationPriority,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub attempts: i32,
    pub max_attempts: i32,
    pub next_attempt_at: Option<DateTime<Utc>>,
    pub locked_by: Option<String>,
    pub locked_at: Option<DateTime<Utc>>,
    pub status: QueueStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum QueueStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotificationSummary {
    pub id: Uuid,
    pub user_id: Uuid,
    pub date: chrono::NaiveDate,
    pub total_received: i32,
    pub total_read: i32,
    pub total_unread: i32,
    pub total_dismissed: i32,
    pub by_type: Option<serde_json::Value>,
    pub by_channel: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceToken {
    pub base: BaseEntity,
    pub user_id: Uuid,
    pub device_type: DeviceType,
    pub token: String,
    pub device_name: Option<String>,
    pub os_version: Option<String>,
    pub app_version: Option<String>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DeviceType {
    IOS,
    Android,
    Web,
    Desktop,
}
