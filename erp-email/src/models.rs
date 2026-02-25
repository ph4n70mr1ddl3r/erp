use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EmailStatus {
    Draft,
    Queued,
    Sending,
    Sent,
    Delivered,
    Opened,
    Clicked,
    Bounced,
    Spam,
    Unsubscribed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CampaignStatus {
    Draft,
    Scheduled,
    Sending,
    Sent,
    Paused,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ListType {
    Static,
    Dynamic,
    Segmented,
    Imported,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SubscriberStatus {
    Active,
    Unsubscribed,
    Bounced,
    Complaint,
    Pending,
    Cleaned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub category: Option<String>,
    pub subject: String,
    pub preheader: Option<String>,
    pub html_body: String,
    pub text_body: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub attachments: Option<serde_json::Value>,
    pub tracking_enabled: bool,
    pub track_opens: bool,
    pub track_clicks: bool,
    pub status: erp_core::Status,
    pub version: i32,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailCampaign {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub template_id: Option<Uuid>,
    pub subject: String,
    pub preheader: Option<String>,
    pub html_body: String,
    pub text_body: Option<String>,
    pub from_name: String,
    pub from_email: String,
    pub reply_to: Option<String>,
    pub list_ids: Vec<Uuid>,
    pub segment_rules: Option<serde_json::Value>,
    pub status: CampaignStatus,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub sent_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub total_recipients: i64,
    pub sent_count: i64,
    pub delivered_count: i64,
    pub opened_count: i64,
    pub clicked_count: i64,
    pub bounced_count: i64,
    pub unsubscribed_count: i64,
    pub complaint_count: i64,
    pub track_opens: bool,
    pub track_clicks: bool,
    pub attachments: Option<serde_json::Value>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailList {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub list_type: ListType,
    pub subscriber_count: i64,
    pub active_count: i64,
    pub bounced_count: i64,
    pub unsubscribed_count: i64,
    pub double_optin: bool,
    pub welcome_email_id: Option<Uuid>,
    pub unsubscribe_page: Option<String>,
    pub confirmation_page: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSubscriber {
    pub base: BaseEntity,
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub company: Option<String>,
    pub phone: Option<String>,
    pub status: SubscriberStatus,
    pub subscribed_at: Option<DateTime<Utc>>,
    pub unsubscribed_at: Option<DateTime<Utc>>,
    pub bounced_at: Option<DateTime<Utc>>,
    pub bounce_reason: Option<String>,
    pub confirmed_at: Option<DateTime<Utc>>,
    pub confirmation_token: Option<String>,
    pub preferences: Option<serde_json::Value>,
    pub custom_fields: Option<serde_json::Value>,
    pub source: Option<String>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub last_email_at: Option<DateTime<Utc>>,
    pub open_count: i32,
    pub click_count: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListMembership {
    pub id: Uuid,
    pub list_id: Uuid,
    pub subscriber_id: Uuid,
    pub added_at: DateTime<Utc>,
    pub added_by: Option<Uuid>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailQueue {
    pub id: Uuid,
    pub campaign_id: Option<Uuid>,
    pub subscriber_id: Option<Uuid>,
    pub to_email: String,
    pub to_name: Option<String>,
    pub subject: String,
    pub html_body: String,
    pub text_body: Option<String>,
    pub from_email: String,
    pub from_name: String,
    pub reply_to: Option<String>,
    pub headers: Option<serde_json::Value>,
    pub attachments: Option<serde_json::Value>,
    pub variables: Option<serde_json::Value>,
    pub tracking_id: Option<String>,
    pub status: EmailStatus,
    pub priority: i32,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub opened_at: Option<DateTime<Utc>>,
    pub clicked_at: Option<DateTime<Utc>>,
    pub bounced_at: Option<DateTime<Utc>>,
    pub bounce_reason: Option<String>,
    pub retry_count: i32,
    pub max_retries: i32,
    pub next_retry_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub external_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailEvent {
    pub id: Uuid,
    pub queue_id: Option<Uuid>,
    pub campaign_id: Option<Uuid>,
    pub subscriber_id: Option<Uuid>,
    pub event_type: String,
    pub email: String,
    pub timestamp: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub url: Option<String>,
    pub details: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailSuppression {
    pub base: BaseEntity,
    pub email: String,
    pub reason: SuppressionReason,
    pub source: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SuppressionReason {
    Bounce,
    Complaint,
    Unsubscribe,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailProvider {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub provider_type: String,
    pub configuration: Option<serde_json::Value>,
    pub credentials_encrypted: Option<String>,
    pub daily_limit: Option<i64>,
    pub daily_used: i64,
    pub monthly_limit: Option<i64>,
    pub monthly_used: i64,
    pub is_default: bool,
    pub status: erp_core::Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
