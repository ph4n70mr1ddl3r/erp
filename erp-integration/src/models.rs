use chrono::{DateTime, NaiveDate, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum APIKeyStatus {
    Active,
    Inactive,
    Expired,
    Revoked,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct APIKey {
    #[sqlx(flatten)]
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub key_hash: String,
    pub key_prefix: String,
    pub user_id: Option<Uuid>,
    #[sqlx(json)]
    pub scopes: Vec<String>,
    pub rate_limit_per_minute: Option<i32>,
    pub rate_limit_per_hour: Option<i32>,
    pub rate_limit_per_day: Option<i32>,
    pub allowed_ips: Option<String>,
    pub allowed_origins: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub usage_count: i64,
    pub status: APIKeyStatus,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct APIKeyUsage {
    pub id: Uuid,
    pub api_key_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub endpoint: String,
    pub method: String,
    pub status_code: i32,
    pub response_time_ms: i64,
    pub request_size: i64,
    pub response_size: i64,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ConnectionStatus {
    Active,
    Inactive,
    Error,
    Pending,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct ExternalConnection {
    #[sqlx(flatten)]
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub connection_type: ConnectionType,
    pub description: Option<String>,
    pub endpoint_url: Option<String>,
    pub configuration: Option<serde_json::Value>,
    pub credentials_encrypted: Option<String>,
    pub auth_type: AuthType,
    pub auth_config: Option<serde_json::Value>,
    pub status: ConnectionStatus,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub last_sync_status: Option<String>,
    pub last_error: Option<String>,
    pub sync_interval_minutes: Option<i32>,
    pub auto_sync: bool,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ConnectionType {
    Database,
    REST,
    GraphQL,
    SOAP,
    FTP,
    SFTP,
    Email,
    OAuth2,
    SAML,
    LDAP,
    Webhook,
    MessageQueue,
    FileStorage,
    PaymentGateway,
    ShippingProvider,
    CRM,
    ERP,
    Accounting,
    ECommerce,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AuthType {
    None,
    Basic,
    Bearer,
    APIKey,
    OAuth2,
    OAuth2ClientCredentials,
    MutualTLS,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionSyncLog {
    pub id: Uuid,
    pub connection_id: Uuid,
    pub sync_type: SyncType,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub status: SyncStatus,
    pub records_processed: i64,
    pub records_created: i64,
    pub records_updated: i64,
    pub records_failed: i64,
    pub error_message: Option<String>,
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SyncType {
    Full,
    Incremental,
    Delta,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SyncStatus {
    Running,
    Completed,
    Failed,
    Cancelled,
    Partial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataMapping {
    pub base: BaseEntity,
    pub name: String,
    pub source_connection_id: Uuid,
    pub target_connection_id: Option<Uuid>,
    pub source_entity: String,
    pub target_entity: String,
    pub field_mappings: serde_json::Value,
    pub transformations: Option<serde_json::Value>,
    pub filters: Option<serde_json::Value>,
    pub sync_direction: SyncDirection,
    pub schedule_id: Option<Uuid>,
    pub enabled: bool,
    pub last_sync_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SyncDirection {
    Import,
    Export,
    Bidirectional,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct IntegrationFlow {
    #[sqlx(flatten)]
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub trigger_type: TriggerType,
    pub trigger_config: Option<serde_json::Value>,
    pub steps: serde_json::Value,
    pub error_handling: ErrorHandlingStrategy,
    pub retry_policy: Option<serde_json::Value>,
    pub enabled: bool,
    pub execution_count: i64,
    pub success_count: i64,
    pub failure_count: i64,
    pub last_execution_at: Option<DateTime<Utc>>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TriggerType {
    Manual,
    Schedule,
    Webhook,
    Event,
    API,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ErrorHandlingStrategy {
    StopOnError,
    ContinueOnError,
    RetryThenStop,
    RetryThenContinue,
    QueueForReview,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowExecution {
    pub base: BaseEntity,
    pub flow_id: Uuid,
    pub trigger_type: TriggerType,
    pub trigger_data: Option<serde_json::Value>,
    pub status: FlowExecutionStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub current_step: Option<i32>,
    pub total_steps: i32,
    pub step_results: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub error_step: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum FlowExecutionStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
    Paused,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationCredential {
    pub base: BaseEntity,
    pub connection_id: Uuid,
    pub credential_type: CredentialType,
    pub name: String,
    pub key_encrypted: Option<String>,
    pub certificate_encrypted: Option<String>,
    pub expires_at: Option<DateTime<Utc>>,
    pub refresh_token_encrypted: Option<String>,
    pub last_refreshed_at: Option<DateTime<Utc>>,
    pub status: CredentialStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CredentialType {
    APIKey,
    OAuth2Token,
    Certificate,
    UsernamePassword,
    SSHKey,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CredentialStatus {
    Valid,
    Expired,
    Invalid,
    NeedsRefresh,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationMetric {
    pub id: Uuid,
    pub connection_id: Uuid,
    pub date: NaiveDate,
    pub hour: i32,
    pub requests_total: i64,
    pub requests_success: i64,
    pub requests_failed: i64,
    pub avg_response_time_ms: i64,
    pub max_response_time_ms: i64,
    pub data_transferred_bytes: i64,
    pub rate_limit_hits: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitConfig {
    pub base: BaseEntity,
    pub name: String,
    pub requests_per_minute: Option<i32>,
    pub requests_per_hour: Option<i32>,
    pub requests_per_day: Option<i32>,
    pub burst_size: Option<i32>,
    pub key_type: RateLimitKeyType,
    pub scope: RateLimitScope,
    pub status: erp_core::Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RateLimitKeyType {
    APIKey,
    IPAddress,
    User,
    Global,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RateLimitScope {
    Global,
    Endpoint,
    Resource,
}
