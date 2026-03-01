use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct BaseEntity {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl Default for BaseEntity {
    fn default() -> Self {
        Self::new()
    }
}

impl BaseEntity {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }

    pub fn new_with_id(id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id,
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type, Default)]
#[sqlx(type_name = "TEXT")]
pub enum Status {
    #[default]
    Active,
    Inactive,
    Draft,
    Pending,
    Approved,
    Rejected,
    Completed,
    Cancelled,
}

impl std::str::FromStr for Status {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Status::Active),
            "inactive" => Ok(Status::Inactive),
            "draft" => Ok(Status::Draft),
            "pending" => Ok(Status::Pending),
            "approved" => Ok(Status::Approved),
            "rejected" => Ok(Status::Rejected),
            "completed" => Ok(Status::Completed),
            "cancelled" | "canceled" => Ok(Status::Cancelled),
            _ => Ok(Status::Active),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub amount: i64,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: i64, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn zero(currency: Currency) -> Self {
        Self {
            amount: 0,
            currency,
        }
    }

    pub fn from_decimal(amount: f64, currency: Currency) -> Self {
        Self {
            amount: (amount * 100.0) as i64,
            currency,
        }
    }

    pub fn to_decimal(&self) -> f64 {
        self.amount as f64 / 100.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Default)]
#[sqlx(type_name = "TEXT")]
pub enum Currency {
    #[default]
    USD,
    EUR,
    GBP,
    JPY,
    CNY,
    CAD,
    AUD,
    CHF,
    INR,
    MXN,
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::USD => write!(f, "USD"),
            Currency::EUR => write!(f, "EUR"),
            Currency::GBP => write!(f, "GBP"),
            Currency::JPY => write!(f, "JPY"),
            Currency::CNY => write!(f, "CNY"),
            Currency::CAD => write!(f, "CAD"),
            Currency::AUD => write!(f, "AUD"),
            Currency::CHF => write!(f, "CHF"),
            Currency::INR => write!(f, "INR"),
            Currency::MXN => write!(f, "MXN"),
        }
    }
}

impl std::str::FromStr for Currency {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            "GBP" => Ok(Currency::GBP),
            "JPY" => Ok(Currency::JPY),
            "CNY" => Ok(Currency::CNY),
            "CAD" => Ok(Currency::CAD),
            "AUD" => Ok(Currency::AUD),
            "CHF" => Ok(Currency::CHF),
            "INR" => Ok(Currency::INR),
            "MXN" => Ok(Currency::MXN),
            _ => Ok(Currency::USD),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldDefinition {
    pub id: Uuid,
    pub entity_type: String,
    pub field_name: String,
    pub field_label: String,
    pub field_type: CustomFieldType,
    pub required: bool,
    pub options: Option<String>,
    pub sort_order: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CustomFieldType {
    Text,
    Number,
    Date,
    Boolean,
    Select,
    MultiSelect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldValue {
    pub id: Uuid,
    pub definition_id: Uuid,
    pub entity_id: Uuid,
    pub value: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TenantPlanType {
    Free,
    Starter,
    Professional,
    Enterprise,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: Uuid,
    pub tenant_code: String,
    pub name: String,
    pub plan_type: TenantPlanType,
    pub max_users: i32,
    pub max_storage_mb: i32,
    pub settings: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TenantUserRole {
    Owner,
    Admin,
    User,
    ReadOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantUser {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub role: TenantUserRole,
    pub joined_at: DateTime<Utc>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TriggerEvent {
    Create,
    Update,
    Delete,
    StatusChange,
    FieldChange,
    Scheduled,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationRule {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub entity_type: String,
    pub trigger_event: TriggerEvent,
    pub conditions: Option<String>,
    pub actions: String,
    pub priority: i32,
    pub active: bool,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationLog {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub trigger_data: Option<String>,
    pub action_results: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
    pub executed_at: DateTime<Utc>,
}

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportDefinition {
    pub id: Uuid,
    pub report_code: String,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub data_source: String,
    pub query_text: String,
    pub parameters: Option<String>,
    pub columns: Option<String>,
    pub filters: Option<String>,
    pub sorting: Option<String>,
    pub grouping: Option<String>,
    pub chart_type: Option<String>,
    pub is_scheduled: bool,
    pub schedule_cron: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReportExecutionStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportExecution {
    pub id: Uuid,
    pub report_id: Uuid,
    pub parameters: Option<String>,
    pub row_count: Option<i32>,
    pub file_path: Option<String>,
    pub file_format: Option<String>,
    pub file_size: Option<i32>,
    pub execution_time_ms: Option<i32>,
    pub status: ReportExecutionStatus,
    pub error_message: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DeviceType {
    Ios,
    Android,
    Web,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileDevice {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_type: DeviceType,
    pub device_token: String,
    pub device_name: Option<String>,
    pub os_version: Option<String>,
    pub app_version: Option<String>,
    pub last_active: Option<DateTime<Utc>>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileSession {
    pub id: Uuid,
    pub device_id: Uuid,
    pub user_id: Uuid,
    pub login_at: DateTime<Utc>,
    pub logout_at: Option<DateTime<Utc>>,
    pub ip_address: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PushNotificationStatus {
    Pending,
    Sent,
    Delivered,
    Read,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushNotification {
    pub id: Uuid,
    pub device_id: Uuid,
    pub title: String,
    pub message: String,
    pub data: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
    pub status: PushNotificationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIKey {
    pub id: Uuid,
    pub user_id: Uuid,
    pub key_hash: String,
    pub name: String,
    pub permissions: Option<String>,
    pub rate_limit: i32,
    pub last_used: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIUsageLog {
    pub id: Uuid,
    pub api_key_id: Option<Uuid>,
    pub endpoint: String,
    pub method: String,
    pub request_size: Option<i32>,
    pub response_size: Option<i32>,
    pub response_code: i32,
    pub response_time_ms: Option<i32>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DashboardType {
    Executive,
    Operational,
    Financial,
    Sales,
    Inventory,
    Manufacturing,
    HR,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WidgetType {
    Chart,
    Table,
    KPI,
    Gauge,
    Map,
    Text,
    Image,
    Counter,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum KPICategory {
    Financial,
    Operational,
    Sales,
    Customer,
    HR,
    Quality,
    Efficiency,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RefreshFrequency {
    RealTime,
    Hourly,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AlertType {
    System,
    Business,
    Threshold,
    Anomaly,
    Scheduled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum Severity {
    Info,
    Warning,
    Critical,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TrendDirection {
    Up,
    Down,
    Flat,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ForecastModelType {
    LinearRegression,
    MovingAverage,
    ExponentialSmoothing,
    ARIMA,
    Prophet,
    NeuralNetwork,
    Ensemble,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dashboard {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub dashboard_type: DashboardType,
    pub is_default: bool,
    pub layout: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    pub id: Uuid,
    pub dashboard_id: Uuid,
    pub widget_type: WidgetType,
    pub title: String,
    pub data_source: String,
    pub query_text: Option<String>,
    pub refresh_interval: i32,
    pub position_x: i32,
    pub position_y: i32,
    pub width: i32,
    pub height: i32,
    pub config: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPIDefinition {
    pub id: Uuid,
    pub kpi_code: String,
    pub name: String,
    pub description: Option<String>,
    pub category: KPICategory,
    pub unit: Option<String>,
    pub target_value: Option<f64>,
    pub warning_threshold: Option<f64>,
    pub critical_threshold: Option<f64>,
    pub calculation_formula: Option<String>,
    pub data_source: Option<String>,
    pub refresh_frequency: RefreshFrequency,
    pub owner: Option<Uuid>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KPIValue {
    pub id: Uuid,
    pub kpi_id: Uuid,
    pub period: String,
    pub value: f64,
    pub target: Option<f64>,
    pub variance: Option<f64>,
    pub variance_percent: Option<f64>,
    pub trend: Option<TrendDirection>,
    pub calculated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub id: Uuid,
    pub alert_type: AlertType,
    pub severity: Severity,
    pub title: String,
    pub message: String,
    pub source_entity: Option<String>,
    pub source_id: Option<Uuid>,
    pub rule_id: Option<Uuid>,
    pub acknowledged: bool,
    pub acknowledged_by: Option<Uuid>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlertRule {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub entity_type: String,
    pub condition_field: String,
    pub operator: String,
    pub threshold_value: String,
    pub severity: Severity,
    pub notification_channels: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastModel {
    pub id: Uuid,
    pub name: String,
    pub model_type: ForecastModelType,
    pub target_entity: String,
    pub features: Option<String>,
    pub parameters: Option<String>,
    pub accuracy_score: Option<f64>,
    pub last_trained: Option<DateTime<Utc>>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub id: Uuid,
    pub model_id: Uuid,
    pub entity_id: Option<Uuid>,
    pub prediction_date: DateTime<Utc>,
    pub predicted_value: f64,
    pub confidence_lower: Option<f64>,
    pub confidence_upper: Option<f64>,
    pub actual_value: Option<f64>,
    pub created_at: DateTime<Utc>,
}
