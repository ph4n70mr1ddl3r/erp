use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
