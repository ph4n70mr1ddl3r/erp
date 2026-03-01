use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TemplateType {
    Email,
    Document,
    Report,
    Label,
    Invoice,
    Quote,
    PurchaseOrder,
    PackingSlip,
    Contract,
    Letter,
    SMS,
    PushNotification,
    Webhook,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TemplateFormat {
    HTML,
    PlainText,
    Markdown,
    PDF,
    JSON,
    XML,
    CSV,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Template {
    #[sqlx(flatten)]
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub template_type: TemplateType,
    pub format: TemplateFormat,
    pub subject: Option<String>,
    pub body: String,
    pub html_body: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub default_values: Option<serde_json::Value>,
    pub styles: Option<String>,
    pub header_template_id: Option<Uuid>,
    pub footer_template_id: Option<Uuid>,
    pub version: i32,
    pub parent_id: Option<Uuid>,
    pub status: erp_core::Status,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVersion {
    pub id: Uuid,
    pub template_id: Uuid,
    pub version: i32,
    pub subject: Option<String>,
    pub body: String,
    pub html_body: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub change_summary: Option<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub id: Uuid,
    pub template_id: Uuid,
    pub name: String,
    pub display_name: String,
    pub description: Option<String>,
    pub variable_type: VariableType,
    pub default_value: Option<String>,
    pub required: bool,
    pub validation_regex: Option<String>,
    pub options: Option<String>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum VariableType {
    String,
    Number,
    Boolean,
    Date,
    DateTime,
    Currency,
    List,
    Object,
    Image,
    URL,
    Email,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct GeneratedDocument {
    #[sqlx(flatten)]
    pub base: BaseEntity,
    pub template_id: Uuid,
    pub template_version: i32,
    pub name: String,
    pub output_format: TemplateFormat,
    pub content: Option<String>,
    pub file_path: Option<String>,
    pub file_size: Option<i64>,
    pub variables_used: Option<serde_json::Value>,
    pub related_entity_type: Option<String>,
    pub related_entity_id: Option<Uuid>,
    pub generated_by: Uuid,
    pub generated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct EmailTemplate {
    #[sqlx(flatten)]
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub subject_template: String,
    pub body_text: Option<String>,
    pub body_html: Option<String>,
    pub from_name: Option<String>,
    pub from_email: Option<String>,
    pub reply_to: Option<String>,
    pub cc_addresses: Option<String>,
    pub bcc_addresses: Option<String>,
    pub attachments: Option<serde_json::Value>,
    pub variables: Option<serde_json::Value>,
    pub status: erp_core::Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailCampaign {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub template_id: Uuid,
    pub recipient_list_id: Option<Uuid>,
    pub recipient_query: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub total_recipients: i32,
    pub sent_count: i32,
    pub delivered_count: i32,
    pub opened_count: i32,
    pub clicked_count: i32,
    pub bounced_count: i32,
    pub unsubscribed_count: i32,
    pub status: CampaignStatus,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CampaignStatus {
    Draft,
    Scheduled,
    Sending,
    Completed,
    Cancelled,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailLog {
    pub id: Uuid,
    pub campaign_id: Option<Uuid>,
    pub template_id: Option<Uuid>,
    pub recipient_email: String,
    pub recipient_name: Option<String>,
    pub subject: String,
    pub status: EmailStatus,
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub opened_at: Option<DateTime<Utc>>,
    pub clicked_at: Option<DateTime<Utc>>,
    pub bounced_at: Option<DateTime<Utc>>,
    pub bounce_reason: Option<String>,
    pub error_message: Option<String>,
    pub message_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EmailStatus {
    Queued,
    Sent,
    Delivered,
    Opened,
    Clicked,
    Bounced,
    Failed,
    Unsubscribed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintTemplate {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub paper_size: PaperSize,
    pub orientation: PageOrientation,
    pub margin_top_mm: f64,
    pub margin_bottom_mm: f64,
    pub margin_left_mm: f64,
    pub margin_right_mm: f64,
    pub header_template: Option<String>,
    pub footer_template: Option<String>,
    pub body_template: String,
    pub css_styles: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub status: erp_core::Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaperSize {
    A4,
    A3,
    Letter,
    Legal,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PageOrientation {
    Portrait,
    Landscape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplate {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub report_type: ReportType,
    pub data_source: String,
    pub query: Option<String>,
    pub parameters: Option<serde_json::Value>,
    pub columns: Option<serde_json::Value>,
    pub groupings: Option<serde_json::Value>,
    pub filters: Option<serde_json::Value>,
    pub sort_order: Option<serde_json::Value>,
    pub chart_config: Option<serde_json::Value>,
    pub template_content: Option<String>,
    pub default_format: ExportFormat,
    pub schedule_id: Option<Uuid>,
    pub status: erp_core::Status,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReportType {
    Tabular,
    Summary,
    Matrix,
    Chart,
    Dashboard,
    Crosstab,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ExportFormat {
    PDF,
    Excel,
    CSV,
    HTML,
    JSON,
    XML,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateCategory {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub parent_id: Option<Uuid>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateTranslation {
    pub id: Uuid,
    pub template_id: Uuid,
    pub language_code: String,
    pub subject: Option<String>,
    pub body: String,
    pub html_body: Option<String>,
    pub variables: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
