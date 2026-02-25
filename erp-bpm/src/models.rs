use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessDefinition {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub category: String,
    pub version: i32,
    pub status: String,
    pub bpmn_xml: Option<String>,
    pub diagram_data: Option<serde_json::Value>,
    pub variables: Option<serde_json::Value>,
    pub forms: Option<serde_json::Value>,
    pub owner_id: Uuid,
    pub published_at: Option<DateTime<Utc>>,
    pub published_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessNode {
    pub id: Uuid,
    pub process_definition_id: Uuid,
    pub node_id: String,
    pub name: String,
    pub task_type: String,
    pub gateway_type: Option<String>,
    pub config: Option<serde_json::Value>,
    pub assignee_expression: Option<String>,
    pub candidate_groups: Option<serde_json::Value>,
    pub form_key: Option<String>,
    pub script: Option<String>,
    pub service_name: Option<String>,
    pub position_x: i32,
    pub position_y: i32,
    pub incoming_flows: Option<serde_json::Value>,
    pub outgoing_flows: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessFlow {
    pub id: Uuid,
    pub process_definition_id: Uuid,
    pub flow_id: String,
    pub name: Option<String>,
    pub source_node_id: String,
    pub target_node_id: String,
    pub condition_expression: Option<String>,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInstance {
    pub id: Uuid,
    pub process_definition_id: Uuid,
    pub process_definition_version: i32,
    pub business_key: Option<String>,
    pub status: String,
    pub variables: Option<serde_json::Value>,
    pub started_by: Uuid,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub current_node_id: Option<String>,
    pub parent_instance_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessTask {
    pub id: Uuid,
    pub process_instance_id: Uuid,
    pub process_definition_id: Uuid,
    pub node_id: String,
    pub name: String,
    pub task_type: String,
    pub status: String,
    pub assignee_id: Option<Uuid>,
    pub candidate_users: Option<serde_json::Value>,
    pub candidate_groups: Option<serde_json::Value>,
    pub form_key: Option<String>,
    pub form_data: Option<serde_json::Value>,
    pub variables: Option<serde_json::Value>,
    pub priority: i32,
    pub due_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub claimed_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub completed_by: Option<Uuid>,
    pub outcome: Option<String>,
    pub updated_at: DateTime<Utc>,
}
