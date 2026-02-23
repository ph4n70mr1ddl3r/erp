use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReportFormat {
    PDF,
    Excel,
    CSV,
    HTML,
    JSON,
    Word,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ScheduleFrequency {
    Once,
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DeliveryMethod {
    Email,
    Download,
    FTP,
    SFTP,
    S3,
    Webhook,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReportStatus {
    Draft,
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDefinition {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub category: ReportCategory,
    pub description: Option<String>,
    pub data_source: String,
    pub query_template: String,
    pub parameters: Vec<ReportParameter>,
    pub columns: Vec<ReportColumn>,
    pub default_format: ReportFormat,
    pub allowed_formats: Vec<ReportFormat>,
    pub is_scheduled: bool,
    pub status: Status,
    pub created_by: Option<Uuid>,
    pub version: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReportCategory {
    Financial,
    Sales,
    Inventory,
    Purchasing,
    Manufacturing,
    HR,
    CRM,
    Operations,
    Compliance,
    Executive,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportParameter {
    pub name: String,
    pub label: String,
    pub param_type: ParameterType,
    pub default_value: Option<String>,
    pub is_required: bool,
    pub lookup_query: Option<String>,
    pub validation_regex: Option<String>,
    pub validation_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ParameterType {
    String,
    Integer,
    Decimal,
    Boolean,
    Date,
    DateTime,
    DateRange,
    Dropdown,
    MultiSelect,
    Customer,
    Vendor,
    Product,
    Warehouse,
    Employee,
    Account,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportColumn {
    pub name: String,
    pub label: String,
    pub data_type: ColumnDataType,
    pub format: Option<String>,
    pub is_visible: bool,
    pub is_sortable: bool,
    pub is_filterable: bool,
    pub aggregation: Option<AggregationType>,
    pub width: Option<i32>,
    pub alignment: ColumnAlignment,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ColumnDataType {
    String,
    Integer,
    Decimal,
    Currency,
    Percentage,
    Date,
    DateTime,
    Boolean,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AggregationType {
    Sum,
    Average,
    Count,
    Min,
    Max,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ColumnAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSchedule {
    pub base: BaseEntity,
    pub report_definition_id: Uuid,
    pub name: String,
    pub frequency: ScheduleFrequency,
    pub cron_expression: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: Option<DateTime<Utc>>,
    pub next_run_at: Option<DateTime<Utc>>,
    pub last_run_at: Option<DateTime<Utc>>,
    pub parameters: String,
    pub output_format: ReportFormat,
    pub delivery_methods: Vec<DeliveryMethod>,
    pub recipients: Vec<String>,
    pub email_subject: Option<String>,
    pub email_body: Option<String>,
    pub include_attachments: bool,
    pub ftp_host: Option<String>,
    pub ftp_path: Option<String>,
    pub webhook_url: Option<String>,
    pub is_active: bool,
    pub status: Status,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportExecution {
    pub base: BaseEntity,
    pub report_definition_id: Uuid,
    pub schedule_id: Option<Uuid>,
    pub parameters: String,
    pub format: ReportFormat,
    pub status: ReportStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_ms: Option<i64>,
    pub row_count: i64,
    pub file_path: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub error_message: Option<String>,
    pub delivery_status: Option<String>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub executed_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDashboard {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub layout: String,
    pub widgets: Vec<DashboardWidget>,
    pub is_default: bool,
    pub is_public: bool,
    pub refresh_interval_seconds: Option<i32>,
    pub status: Status,
    pub owner_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    pub id: Uuid,
    pub dashboard_id: Uuid,
    pub report_definition_id: Option<Uuid>,
    pub widget_type: WidgetType,
    pub title: String,
    pub position_x: i32,
    pub position_y: i32,
    pub width: i32,
    pub height: i32,
    pub parameters: String,
    pub refresh_interval_seconds: Option<i32>,
    pub chart_config: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WidgetType {
    Table,
    Chart,
    KPI,
    Gauge,
    Map,
    PivotTable,
    Sparkline,
    Treemap,
    Heatmap,
    Funnel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportSubscription {
    pub base: BaseEntity,
    pub report_definition_id: Uuid,
    pub user_id: Uuid,
    pub notify_on_completion: bool,
    pub notify_on_failure: bool,
    pub preferred_format: ReportFormat,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportTemplate {
    pub base: BaseEntity,
    pub name: String,
    pub report_definition_id: Uuid,
    pub parameters: String,
    pub styling: Option<String>,
    pub header_html: Option<String>,
    pub footer_html: Option<String>,
    pub page_size: String,
    pub orientation: PageOrientation,
    pub margins: String,
    pub status: Status,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PageOrientation {
    Portrait,
    Landscape,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportAccess {
    pub id: Uuid,
    pub report_definition_id: Uuid,
    pub principal_type: AccessPrincipalType,
    pub principal_id: Uuid,
    pub can_view: bool,
    pub can_edit: bool,
    pub can_delete: bool,
    pub can_schedule: bool,
    pub can_share: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AccessPrincipalType {
    User,
    Role,
    Department,
    Everyone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataExportJob {
    pub base: BaseEntity,
    pub name: String,
    pub source_type: ExportSourceType,
    pub source_query: String,
    pub format: ReportFormat,
    pub destination_type: DeliveryMethod,
    pub destination_path: String,
    pub parameters: String,
    pub status: ReportStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub row_count: i64,
    pub file_path: Option<String>,
    pub error_message: Option<String>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ExportSourceType {
    Table,
    Query,
    Report,
    StoredProcedure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportCache {
    pub id: Uuid,
    pub report_definition_id: Uuid,
    pub parameters_hash: String,
    pub result_data: String,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub hit_count: i64,
}
