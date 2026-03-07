use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::common::Status;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailTemplate {
    pub id: Uuid,
    pub template_code: String,
    pub name: String,
    pub subject: String,
    pub body_html: String,
    pub body_text: Option<String>,
    pub category: Option<String>,
    pub variables: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EmailStatus {
    Pending,
    Sent,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailQueue {
    pub id: Uuid,
    pub template_id: Option<Uuid>,
    pub to_address: String,
    pub cc_addresses: Option<String>,
    pub bcc_addresses: Option<String>,
    pub subject: String,
    pub body: String,
    pub attachments: Option<String>,
    pub priority: i32,
    pub attempts: i32,
    pub max_attempts: i32,
    pub sent_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub status: EmailStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EmailDirection {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailLog {
    pub id: Uuid,
    pub message_id: Option<String>,
    pub direction: EmailDirection,
    pub from_address: String,
    pub to_address: String,
    pub subject: Option<String>,
    pub body: Option<String>,
    pub attachments: Option<String>,
    pub related_entity_type: Option<String>,
    pub related_entity_id: Option<Uuid>,
    pub sent_at: DateTime<Utc>,
    pub status: EmailStatus,
}
