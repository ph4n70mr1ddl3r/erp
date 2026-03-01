use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WebhookStatus {
    Active,
    Inactive,
    Disabled,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WebhookEventType {
    OrderCreated,
    OrderUpdated,
    OrderCancelled,
    OrderCompleted,
    InvoiceCreated,
    InvoicePaid,
    InvoiceVoided,
    PaymentReceived,
    PaymentFailed,
    ShipmentCreated,
    ShipmentDelivered,
    InventoryLow,
    InventoryOut,
    CustomerCreated,
    CustomerUpdated,
    VendorCreated,
    VendorUpdated,
    EmployeeCreated,
    EmployeeUpdated,
    TicketCreated,
    TicketUpdated,
    TicketClosed,
    ProjectCreated,
    ProjectCompleted,
    TaskCreated,
    TaskCompleted,
    WorkflowStarted,
    WorkflowCompleted,
    ApprovalRequested,
    ApprovalApproved,
    ApprovalRejected,
    ReportGenerated,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEndpoint {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub url: String,
    pub secret: String,
    pub events: Vec<WebhookEventType>,
    pub headers: Option<serde_json::Value>,
    pub authentication: Option<WebhookAuth>,
    pub timeout_seconds: i32,
    pub retry_policy: RetryPolicy,
    pub status: WebhookStatus,
    pub created_by: Uuid,
    pub last_triggered_at: Option<DateTime<Utc>>,
    pub last_success_at: Option<DateTime<Utc>>,
    pub last_failure_at: Option<DateTime<Utc>>,
    pub total_triggers: i64,
    pub successful_triggers: i64,
    pub failed_triggers: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for WebhookEndpoint {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> sqlx::Result<Self> {
        use sqlx::Row;
        
        let events_json: String = row.try_get("events")?;
        let events: Vec<WebhookEventType> = serde_json::from_str(&events_json)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        
        let headers: Option<String> = row.try_get("headers")?;
        let headers: Option<serde_json::Value> = headers
            .map(|h| serde_json::from_str(&h))
            .transpose()
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        
        let auth_json: Option<String> = row.try_get("authentication")?;
        let authentication: Option<WebhookAuth> = auth_json
            .map(|a| serde_json::from_str(&a))
            .transpose()
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        
        let retry_json: String = row.try_get("retry_policy")?;
        let retry_policy: RetryPolicy = serde_json::from_str(&retry_json)
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        
        let base = BaseEntity {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            created_by: None,
            updated_by: None,
        };
        
        Ok(Self {
            base,
            name: row.try_get("name")?,
            description: row.try_get("description")?,
            url: row.try_get("url")?,
            secret: row.try_get("secret")?,
            events,
            headers,
            authentication,
            timeout_seconds: row.try_get("timeout_seconds")?,
            retry_policy,
            status: row.try_get("status")?,
            created_by: row.try_get("created_by")?,
            last_triggered_at: row.try_get("last_triggered_at")?,
            last_success_at: row.try_get("last_success_at")?,
            last_failure_at: row.try_get("last_failure_at")?,
            total_triggers: row.try_get("total_triggers")?,
            successful_triggers: row.try_get("successful_triggers")?,
            failed_triggers: row.try_get("failed_triggers")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookAuth {
    pub auth_type: WebhookAuthType,
    pub username: Option<String>,
    pub password: Option<String>,
    pub api_key: Option<String>,
    pub api_key_header: Option<String>,
    pub bearer_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WebhookAuthType {
    None,
    Basic,
    Bearer,
    ApiKey,
    OAuth2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_retries: i32,
    pub initial_delay_ms: i64,
    pub max_delay_ms: i64,
    pub backoff_multiplier: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 5,
            initial_delay_ms: 1000,
            max_delay_ms: 60000,
            backoff_multiplier: 2.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookDelivery {
    pub base: BaseEntity,
    pub endpoint_id: Uuid,
    pub event_type: WebhookEventType,
    pub event_id: Uuid,
    pub payload: serde_json::Value,
    pub headers: Option<serde_json::Value>,
    pub response_status: Option<i32>,
    pub response_body: Option<String>,
    pub response_headers: Option<serde_json::Value>,
    pub duration_ms: Option<i64>,
    pub attempt_number: i32,
    pub max_attempts: i32,
    pub next_retry_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub status: DeliveryStatus,
    pub error_message: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for WebhookDelivery {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> sqlx::Result<Self> {
        use sqlx::Row;
        
        let headers: Option<String> = row.try_get("headers")?;
        let headers: Option<serde_json::Value> = headers
            .map(|h| serde_json::from_str(&h))
            .transpose()
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        
        let response_headers: Option<String> = row.try_get("response_headers")?;
        let response_headers: Option<serde_json::Value> = response_headers
            .map(|h| serde_json::from_str(&h))
            .transpose()
            .map_err(|e| sqlx::Error::Decode(Box::new(e)))?;
        
        let base = BaseEntity {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("created_at")?, // Use created_at as updated_at since deliveries don't update
            created_by: None,
            updated_by: None,
        };
        
        Ok(Self {
            base,
            endpoint_id: row.try_get("endpoint_id")?,
            event_type: row.try_get("event_type")?,
            event_id: row.try_get("event_id")?,
            payload: serde_json::from_str(row.try_get::<&str, _>("payload")?)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
            headers,
            response_status: row.try_get("response_status")?,
            response_body: row.try_get("response_body")?,
            response_headers,
            duration_ms: row.try_get("duration_ms")?,
            attempt_number: row.try_get("attempt_number")?,
            max_attempts: row.try_get("max_attempts")?,
            next_retry_at: row.try_get("next_retry_at")?,
            delivered_at: row.try_get("delivered_at")?,
            status: row.try_get("status")?,
            error_message: row.try_get("error_message")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DeliveryStatus {
    Pending,
    Processing,
    Delivered,
    Failed,
    Retrying,
    Abandoned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEvent {
    pub base: BaseEntity,
    pub event_type: WebhookEventType,
    pub source_entity_type: String,
    pub source_entity_id: Uuid,
    pub payload: serde_json::Value,
    pub triggered_by: Uuid,
    pub triggered_at: DateTime<Utc>,
    pub delivered: bool,
    pub delivery_count: i32,
    pub created_at: DateTime<Utc>,
}

impl<'r> sqlx::FromRow<'r, sqlx::sqlite::SqliteRow> for WebhookEvent {
    fn from_row(row: &'r sqlx::sqlite::SqliteRow) -> sqlx::Result<Self> {
        use sqlx::Row;
        
        let base = BaseEntity {
            id: row.try_get("id")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("created_at")?, // Use created_at as updated_at since events don't update
            created_by: None,
            updated_by: None,
        };
        
        Ok(Self {
            base,
            event_type: row.try_get("event_type")?,
            source_entity_type: row.try_get("source_entity_type")?,
            source_entity_id: row.try_get("source_entity_id")?,
            payload: serde_json::from_str(row.try_get::<&str, _>("payload")?)
                .map_err(|e| sqlx::Error::Decode(Box::new(e)))?,
            triggered_by: row.try_get("triggered_by")?,
            triggered_at: row.try_get("triggered_at")?,
            delivered: row.try_get("delivered")?,
            delivery_count: row.try_get("delivery_count")?,
            created_at: row.try_get("created_at")?,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSubscription {
    pub base: BaseEntity,
    pub endpoint_id: Uuid,
    pub event_type: WebhookEventType,
    pub filter_rules: Option<serde_json::Value>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookLog {
    pub id: Uuid,
    pub endpoint_id: Uuid,
    pub delivery_id: Option<Uuid>,
    pub event_type: WebhookEventType,
    pub level: LogLevel,
    pub message: String,
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookSignature {
    pub algorithm: String,
    pub timestamp: i64,
    pub signature: String,
}

impl WebhookSignature {
    pub fn new(secret: &str, payload: &[u8]) -> Self {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        let timestamp = Utc::now().timestamp();
        let payload_with_timestamp = format!("{}.{}", timestamp, String::from_utf8_lossy(payload));

        let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
        mac.update(payload_with_timestamp.as_bytes());
        let result = mac.finalize();
        let signature = hex::encode(result.into_bytes());

        Self {
            algorithm: "sha256".to_string(),
            timestamp,
            signature,
        }
    }

    pub fn verify(&self, secret: &str, payload: &[u8]) -> bool {
        let expected = Self::new(secret, payload);
        self.signature == expected.signature && self.timestamp == expected.timestamp
    }
}
