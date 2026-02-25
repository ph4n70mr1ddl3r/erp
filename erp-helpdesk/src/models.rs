use chrono::{DateTime, Utc};
use erp_core::models::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[derive(PartialEq)]
pub enum TicketStatus {
    New,
    Open,
    InProgress,
    Pending,
    OnHold,
    Resolved,
    Closed,
    Reopened,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
#[derive(PartialEq)]
pub enum TicketPriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TicketSource {
    Email,
    Phone,
    Web,
    Chat,
    SocialMedia,
    API,
    Internal,
    Mobile,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TicketType {
    Incident,
    ServiceRequest,
    Problem,
    ChangeRequest,
    Information,
    Complaint,
    Feedback,
    Task,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub base: BaseEntity,
    pub ticket_number: String,
    pub subject: String,
    pub description: String,
    pub ticket_type: TicketType,
    pub status: TicketStatus,
    pub priority: TicketPriority,
    pub source: TicketSource,
    pub requester_id: Uuid,
    pub requester_email: String,
    pub requester_name: String,
    pub assignee_id: Option<Uuid>,
    pub team_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub subcategory_id: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub resolution_date: Option<DateTime<Utc>>,
    pub first_response_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub sla_id: Option<Uuid>,
    pub sla_breached: bool,
    pub satisfaction_rating: Option<i32>,
    pub satisfaction_comment: Option<String>,
    pub tags: Vec<String>,
    pub custom_fields: serde_json::Value,
    pub related_tickets: Vec<Uuid>,
    pub parent_ticket_id: Option<Uuid>,
    pub knowledge_article_id: Option<Uuid>,
    pub asset_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketCategory {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub sort_order: i32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketSubcategory {
    pub base: BaseEntity,
    pub category_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketComment {
    pub base: BaseEntity,
    pub ticket_id: Uuid,
    pub author_id: Uuid,
    pub author_name: String,
    pub content: String,
    pub comment_type: CommentType,
    pub is_internal: bool,
    pub created_at: DateTime<Utc>,
    pub attachments: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CommentType {
    Reply,
    Note,
    System,
    Email,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketAttachment {
    pub base: BaseEntity,
    pub ticket_id: Uuid,
    pub comment_id: Option<Uuid>,
    pub filename: String,
    pub content_type: String,
    pub size: i64,
    pub storage_path: String,
    pub uploaded_by: Uuid,
    pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketHistory {
    pub base: BaseEntity,
    pub ticket_id: Uuid,
    pub field_name: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub changed_by: Uuid,
    pub changed_at: DateTime<Utc>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportTeam {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub email: String,
    pub leader_id: Option<Uuid>,
    pub members: Vec<Uuid>,
    pub category_ids: Vec<Uuid>,
    pub is_active: bool,
    pub working_hours: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportAgent {
    pub base: BaseEntity,
    pub user_id: Uuid,
    pub name: String,
    pub email: String,
    pub team_id: Option<Uuid>,
    pub role: AgentRole,
    pub skills: Vec<String>,
    pub max_tickets: i32,
    pub active_tickets: i32,
    pub is_available: bool,
    pub last_activity: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AgentRole {
    Agent,
    SeniorAgent,
    TeamLead,
    Manager,
    Admin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLAPolicy {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub priority_rules: Vec<SLAPriorityRule>,
    pub calendar_id: Option<Uuid>,
    pub is_default: bool,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLAPriorityRule {
    pub priority: TicketPriority,
    pub first_response_hours: i32,
    pub resolution_hours: i32,
    pub update_hours: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLATracker {
    pub base: BaseEntity,
    pub ticket_id: Uuid,
    pub sla_id: Uuid,
    pub first_response_due: DateTime<Utc>,
    pub first_response_met: Option<bool>,
    pub resolution_due: DateTime<Utc>,
    pub resolution_met: Option<bool>,
    pub next_update_due: Option<DateTime<Utc>>,
    pub paused_at: Option<DateTime<Utc>>,
    pub total_pause_duration_secs: i64,
    pub status: SLAStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SLAStatus {
    Active,
    Paused,
    Met,
    Breached,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRule {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub conditions: Vec<EscalationCondition>,
    pub actions: Vec<EscalationAction>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationCondition {
    pub field: String,
    pub operator: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationAction {
    pub action_type: EscalationActionType,
    pub target_id: Option<Uuid>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EscalationActionType {
    AssignToAgent,
    AssignToTeam,
    IncreasePriority,
    SendNotification,
    SendEmail,
    AddTag,
    Webhook,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CannedResponse {
    pub base: BaseEntity,
    pub name: String,
    pub title: String,
    pub content: String,
    pub category_id: Option<Uuid>,
    pub tags: Vec<String>,
    pub created_by: Uuid,
    pub use_count: i32,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketMerge {
    pub base: BaseEntity,
    pub primary_ticket_id: Uuid,
    pub merged_ticket_ids: Vec<Uuid>,
    pub merged_by: Uuid,
    pub merged_at: DateTime<Utc>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketSplit {
    pub base: BaseEntity,
    pub original_ticket_id: Uuid,
    pub new_ticket_id: Uuid,
    pub split_by: Uuid,
    pub split_at: DateTime<Utc>,
    pub reason: String,
    pub comment_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketWorkflow {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub ticket_type: TicketType,
    pub states: Vec<WorkflowState>,
    pub transitions: Vec<WorkflowTransition>,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowState {
    pub status: TicketStatus,
    pub name: String,
    pub is_initial: bool,
    pub is_final: bool,
    pub auto_assign: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowTransition {
    pub from_status: TicketStatus,
    pub to_status: TicketStatus,
    pub name: String,
    pub required_permission: Option<String>,
    pub auto_actions: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportDashboard {
    pub base: BaseEntity,
    pub user_id: Uuid,
    pub name: String,
    pub widgets: Vec<DashboardWidget>,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardWidget {
    pub widget_type: WidgetType,
    pub title: String,
    pub config: serde_json::Value,
    pub position: WidgetPosition,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WidgetType {
    TicketStats,
    Chart,
    RecentTickets,
    MyTickets,
    TeamQueue,
    SLAMonitor,
    AgentPerformance,
    KnowledgeSearch,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetPosition {
    pub row: i32,
    pub col: i32,
    pub width: i32,
    pub height: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketSurvey {
    pub base: BaseEntity,
    pub ticket_id: Uuid,
    pub rating: i32,
    pub feedback: Option<String>,
    pub categories: Vec<SurveyCategoryRating>,
    pub submitted_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurveyCategoryRating {
    pub category: String,
    pub rating: i32,
}
