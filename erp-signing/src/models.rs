use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DocumentStatus {
    Draft,
    Pending,
    Sent,
    Viewed,
    Signed,
    Completed,
    Cancelled,
    Expired,
    Declined,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SignatureType {
    Drawn,
    Typed,
    Uploaded,
    ClickToSign,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum SignerStatus {
    Pending,
    Viewed,
    Sent,
    Signed,
    Declined,
    Delegated,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AuthenticationMethod {
    None,
    Email,
    SMS,
    AccessCode,
    IDVerification,
    Biometric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningDocument {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub document_type: String,
    pub file_path: String,
    pub file_name: String,
    pub file_size: i64,
    pub file_hash: String,
    pub pages: i32,
    pub status: DocumentStatus,
    pub envelope_id: Option<String>,
    pub sender_id: Uuid,
    pub message: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub sent_at: Option<DateTime<Utc>>,
    pub viewed_at: Option<DateTime<Utc>>,
    pub reminder_count: i32,
    pub last_reminder_at: Option<DateTime<Utc>>,
    pub auto_remind: bool,
    pub remind_days: i32,
    pub sequential_signing: bool,
    pub current_signer_order: Option<i32>,
    pub final_signed_file: Option<String>,
    pub final_signed_at: Option<DateTime<Utc>>,
    pub audit_trail_file: Option<String>,
    pub certificate_of_completion: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signer {
    pub base: BaseEntity,
    pub document_id: Uuid,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub user_id: Option<Uuid>,
    pub order_index: i32,
    pub role: SignerRole,
    pub status: SignerStatus,
    pub authentication_method: AuthenticationMethod,
    pub access_code: Option<String>,
    pub viewed_at: Option<DateTime<Utc>>,
    pub signed_at: Option<DateTime<Utc>>,
    pub declined_at: Option<DateTime<Utc>>,
    pub declined_reason: Option<String>,
    pub delegated_to: Option<Uuid>,
    pub email_sent_at: Option<DateTime<Utc>>,
    pub reminder_sent_at: Option<DateTime<Utc>>,
    pub signature_ip: Option<String>,
    pub signature_user_agent: Option<String>,
    pub signature_location: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SignerRole {
    Signer,
    Approver,
    CC,
    Witness,
    Notary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureField {
    pub base: BaseEntity,
    pub document_id: Uuid,
    pub signer_id: Uuid,
    pub field_type: SignatureFieldType,
    pub page: i32,
    pub x_position: f64,
    pub y_position: f64,
    pub width: f64,
    pub height: f64,
    pub required: bool,
    pub value: Option<String>,
    pub signature_data: Option<String>,
    pub signature_type: Option<SignatureType>,
    pub signed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SignatureFieldType {
    Signature,
    Initial,
    Date,
    Text,
    Checkbox,
    Radio,
    Dropdown,
    Attachment,
    Stamp,
    FullName,
    Email,
    Company,
    Title,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Signature {
    pub base: BaseEntity,
    pub signer_id: Uuid,
    pub signature_type: SignatureType,
    pub signature_data: String,
    pub initials_data: Option<String>,
    pub signed_at: DateTime<Utc>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub geolocation: Option<String>,
    pub device_fingerprint: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SigningTemplate {
    #[sqlx(flatten)]
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub template_type: String,
    pub document_path: Option<String>,
    pub field_config: serde_json::Value,
    pub signer_config: serde_json::Value,
    pub message_template: Option<String>,
    pub auto_expire_days: i32,
    pub remind_days: i32,
    pub sequential_signing: bool,
    pub status: erp_core::Status,
    pub usage_count: i64,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct SigningAudit {
    pub id: Uuid,
    pub document_id: Uuid,
    pub signer_id: Option<Uuid>,
    pub action: String,
    pub details: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub geolocation: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SigningWebhook {
    pub id: Uuid,
    pub document_id: Uuid,
    pub event_type: String,
    pub payload: serde_json::Value,
    pub url: String,
    pub sent_at: Option<DateTime<Utc>>,
    pub response_status: Option<i32>,
    pub response_body: Option<String>,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
}
