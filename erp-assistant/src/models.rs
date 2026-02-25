use chrono::{DateTime, Utc};
use erp_core::models::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantConversation {
    pub base: BaseEntity,
    pub user_id: Uuid,
    pub title: String,
    pub context: serde_json::Value,
    pub status: ConversationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum ConversationStatus {
    Active,
    Archived,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMessage {
    pub base: BaseEntity,
    pub conversation_id: Uuid,
    pub role: MessageRole,
    pub content: String,
    pub intent: Option<String>,
    pub entities: serde_json::Value,
    pub action_taken: Option<String>,
    pub action_result: Option<serde_json::Value>,
    pub feedback: Option<MessageFeedback>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageFeedback {
    pub rating: i32,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentDefinition {
    pub base: BaseEntity,
    pub name: String,
    pub description: String,
    pub training_phrases: Vec<String>,
    pub action_template: String,
    pub parameters: Vec<IntentParameter>,
    pub confidence_threshold: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntentParameter {
    pub name: String,
    pub param_type: ParameterType,
    pub required: bool,
    pub prompt: Option<String>,
    pub default_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ParameterType {
    String,
    Number,
    Date,
    Entity(String),
    Enum(Vec<String>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantSkill {
    pub base: BaseEntity,
    pub name: String,
    pub description: String,
    pub category: SkillCategory,
    pub trigger_phrases: Vec<String>,
    pub handler_module: String,
    pub handler_function: String,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SkillCategory {
    Finance,
    Inventory,
    Sales,
    Purchasing,
    Hr,
    Analytics,
    Reporting,
    Workflow,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedQuery {
    pub intent: String,
    pub confidence: f64,
    pub entities: Vec<ExtractedEntity>,
    pub original_query: String,
    pub normalized_query: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedEntity {
    pub name: String,
    pub value: String,
    pub entity_type: String,
    pub confidence: f64,
    pub position: (usize, usize),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantResponse {
    pub message: String,
    pub action: Option<AssistantAction>,
    pub suggestions: Vec<String>,
    pub data: Option<serde_json::Value>,
    pub visualization: Option<VisualizationSpec>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantAction {
    pub action_type: String,
    pub parameters: serde_json::Value,
    pub confirmation_required: bool,
    pub confirmation_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualizationSpec {
    pub chart_type: String,
    pub title: String,
    pub data: serde_json::Value,
    pub options: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuickAction {
    pub base: BaseEntity,
    pub name: String,
    pub description: String,
    pub icon: String,
    pub action: String,
    pub category: String,
    pub shortcut: Option<String>,
    pub position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantContext {
    pub current_entity: Option<EntityReference>,
    pub recent_entities: Vec<EntityReference>,
    pub user_preferences: serde_json::Value,
    pub session_data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityReference {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub display_name: String,
    pub context_data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateConversationRequest {
    pub title: Option<String>,
    pub initial_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub conversation_id: Uuid,
    pub message: String,
    pub context: Option<AssistantContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageFeedbackRequest {
    pub message_id: Uuid,
    pub rating: i32,
    pub comment: Option<String>,
}
