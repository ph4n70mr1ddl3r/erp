use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemConfig {
    pub base: BaseEntity,
    pub category: String,
    pub key: String,
    pub value: String,
    pub value_type: ConfigValueType,
    pub description: Option<String>,
    pub is_encrypted: bool,
    pub is_system: bool,
    pub is_public: bool,
    pub default_value: Option<String>,
    pub validation_regex: Option<String>,
    pub group_name: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ConfigValueType {
    String,
    Integer,
    Decimal,
    Boolean,
    Json,
    Array,
    Date,
    DateTime,
    Color,
    File,
    Url,
    Email,
    Phone,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigHistory {
    pub base: BaseEntity,
    pub config_id: Uuid,
    pub old_value: Option<String>,
    pub new_value: String,
    pub changed_by: Option<Uuid>,
    pub changed_at: DateTime<Utc>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanySetting {
    pub base: BaseEntity,
    pub company_name: String,
    pub legal_name: Option<String>,
    pub tax_id: Option<String>,
    pub registration_number: Option<String>,
    pub logo_url: Option<String>,
    pub favicon_url: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub timezone: String,
    pub date_format: String,
    pub time_format: String,
    pub currency: String,
    pub language: String,
    pub fiscal_year_start: i32,
    pub week_start: i32,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberSequence {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub current_value: i64,
    pub increment: i32,
    pub padding: i32,
    pub reset_period: Option<ResetPeriod>,
    pub last_reset: Option<DateTime<Utc>>,
    pub format: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ResetPeriod {
    Never,
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    Yearly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmailConfig {
    pub base: BaseEntity,
    pub smtp_host: String,
    pub smtp_port: i32,
    pub smtp_user: String,
    pub smtp_password: Option<String>,
    pub use_tls: bool,
    pub use_ssl: bool,
    pub from_address: String,
    pub from_name: Option<String>,
    pub reply_to: Option<String>,
    pub is_default: bool,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageConfig {
    pub base: BaseEntity,
    pub storage_type: StorageType,
    pub name: String,
    pub endpoint: Option<String>,
    pub bucket: Option<String>,
    pub region: Option<String>,
    pub access_key: Option<String>,
    pub secret_key: Option<String>,
    pub base_path: Option<String>,
    pub max_file_size: i64,
    pub allowed_types: Option<String>,
    pub is_default: bool,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum StorageType {
    Local,
    S3,
    Azure,
    GCS,
    FTP,
    SFTP,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentGateway {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub gateway_type: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub merchant_id: Option<String>,
    pub endpoint_url: Option<String>,
    pub webhook_url: Option<String>,
    pub supported_currencies: Option<String>,
    pub supported_methods: Option<String>,
    pub fee_percent: Option<f64>,
    pub fee_fixed: Option<i64>,
    pub is_sandbox: bool,
    pub is_default: bool,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingProvider {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub account_number: Option<String>,
    pub endpoint_url: Option<String>,
    pub tracking_url: Option<String>,
    pub supported_services: Option<String>,
    pub supported_countries: Option<String>,
    pub is_default: bool,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Localization {
    pub base: BaseEntity,
    pub language_code: String,
    pub locale: String,
    pub name: String,
    pub native_name: String,
    pub date_format: String,
    pub time_format: String,
    pub number_format: String,
    pub currency_symbol: Option<String>,
    pub currency_position: Option<String>,
    pub decimal_separator: String,
    pub thousand_separator: String,
    pub is_rtl: bool,
    pub is_default: bool,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditSetting {
    pub base: BaseEntity,
    pub log_retention_days: i32,
    pub log_sensitive_data: bool,
    pub log_login_attempts: bool,
    pub log_data_changes: bool,
    pub log_api_requests: bool,
    pub alert_on_suspicious: bool,
    pub max_login_attempts: i32,
    pub lockout_duration_minutes: i32,
    pub password_expiry_days: Option<i32>,
    pub require_mfa: bool,
    pub session_timeout_minutes: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub integration_type: String,
    pub api_endpoint: Option<String>,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub config_json: Option<String>,
    pub sync_enabled: bool,
    pub sync_frequency: Option<String>,
    pub last_sync: Option<DateTime<Utc>>,
    pub sync_status: Option<String>,
    pub is_active: bool,
}
