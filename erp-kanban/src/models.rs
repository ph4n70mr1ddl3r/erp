use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum KanbanCardType {
    Task,
    Defect,
    Story,
    Epic,
    ProductionOrder,
    PurchaseRequest,
    MaintenanceRequest,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum KanbanCardPriority {
    Lowest,
    Low,
    Medium,
    High,
    Highest,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum KanbanSwimlaneType {
    None,
    Assignee,
    Priority,
    Project,
    Product,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanBoard {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub board_type: KanbanBoardType,
    pub team_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub columns: Vec<KanbanColumn>,
    pub swimlane_type: KanbanSwimlaneType,
    pub swimlanes: Vec<KanbanSwimlane>,
    pub default_wip_limit: Option<i32>,
    pub allow_card_reordering: bool,
    pub show_card_count: bool,
    pub show_wip_limits: bool,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum KanbanBoardType {
    Generic,
    Production,
    Purchase,
    Sales,
    Support,
    Maintenance,
    Project,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanColumn {
    pub id: Uuid,
    pub board_id: Uuid,
    pub name: String,
    pub position: i32,
    pub wip_limit: Option<i32>,
    pub is_done_column: bool,
    pub is_backlog: bool,
    pub color: Option<String>,
    pub auto_assign_on_move: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanSwimlane {
    pub id: Uuid,
    pub board_id: Uuid,
    pub name: String,
    pub position: i32,
    pub color: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanCard {
    pub base: BaseEntity,
    pub board_id: Uuid,
    pub column_id: Uuid,
    pub swimlane_id: Option<Uuid>,
    pub card_type: KanbanCardType,
    pub title: String,
    pub description: Option<String>,
    pub priority: KanbanCardPriority,
    pub position: i32,
    pub assignee_ids: Vec<Uuid>,
    pub reporter_id: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub start_date: Option<DateTime<Utc>>,
    pub completed_date: Option<DateTime<Utc>>,
    pub estimated_hours: Option<f64>,
    pub actual_hours: Option<f64>,
    pub story_points: Option<i32>,
    pub tags: Vec<String>,
    pub external_ref_type: Option<String>,
    pub external_ref_id: Option<Uuid>,
    pub blocked: bool,
    pub blocked_reason: Option<String>,
    pub parent_card_id: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanCardComment {
    pub id: Uuid,
    pub card_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanCardAttachment {
    pub id: Uuid,
    pub card_id: Uuid,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub content_type: String,
    pub uploaded_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanCardChecklist {
    pub id: Uuid,
    pub card_id: Uuid,
    pub title: String,
    pub position: i32,
    pub items: Vec<KanbanCardChecklistItem>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanCardChecklistItem {
    pub id: Uuid,
    pub checklist_id: Uuid,
    pub content: String,
    pub position: i32,
    pub completed: bool,
    pub completed_by: Option<Uuid>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanCardMove {
    pub id: Uuid,
    pub card_id: Uuid,
    pub from_column_id: Uuid,
    pub to_column_id: Uuid,
    pub from_position: i32,
    pub to_position: i32,
    pub moved_by: Uuid,
    pub moved_at: DateTime<Utc>,
    pub time_in_from_column_seconds: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanCardLabel {
    pub id: Uuid,
    pub board_id: Uuid,
    pub name: String,
    pub color: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanCardLabelAssignment {
    pub id: Uuid,
    pub card_id: Uuid,
    pub label_id: Uuid,
    pub assigned_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanActivityLog {
    pub id: Uuid,
    pub board_id: Uuid,
    pub card_id: Option<Uuid>,
    pub action_type: KanbanActionType,
    pub actor_id: Uuid,
    pub description: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum KanbanActionType {
    CardCreated,
    CardMoved,
    CardUpdated,
    CardDeleted,
    CardAssigned,
    CardUnassigned,
    CommentAdded,
    CommentDeleted,
    AttachmentAdded,
    AttachmentDeleted,
    ChecklistCreated,
    ChecklistItemCompleted,
    LabelAdded,
    LabelRemoved,
    CardBlocked,
    CardUnblocked,
    DueDateChanged,
    PriorityChanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanWipViolation {
    pub id: Uuid,
    pub board_id: Uuid,
    pub column_id: Uuid,
    pub current_count: i32,
    pub wip_limit: i32,
    pub violated_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KanbanMetrics {
    pub board_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub total_cards_created: i32,
    pub total_cards_completed: i32,
    pub average_cycle_time_days: f64,
    pub average_lead_time_days: f64,
    pub average_time_in_column: Vec<ColumnTimeMetric>,
    pub throughput_per_day: f64,
    pub wip_violations_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnTimeMetric {
    pub column_id: Uuid,
    pub column_name: String,
    pub average_time_seconds: i64,
    pub card_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBoardRequest {
    pub name: String,
    pub description: Option<String>,
    pub board_type: KanbanBoardType,
    pub team_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub columns: Vec<CreateColumnRequest>,
    pub default_wip_limit: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateColumnRequest {
    pub name: String,
    pub wip_limit: Option<i32>,
    pub is_done_column: bool,
    pub is_backlog: bool,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCardRequest {
    pub board_id: Uuid,
    pub column_id: Uuid,
    pub swimlane_id: Option<Uuid>,
    pub card_type: KanbanCardType,
    pub title: String,
    pub description: Option<String>,
    pub priority: KanbanCardPriority,
    pub assignee_ids: Vec<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub estimated_hours: Option<f64>,
    pub story_points: Option<i32>,
    pub tags: Vec<String>,
    pub external_ref_type: Option<String>,
    pub external_ref_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveCardRequest {
    pub card_id: Uuid,
    pub to_column_id: Uuid,
    pub to_position: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoardSummary {
    pub board_id: Uuid,
    pub board_name: String,
    pub total_cards: i32,
    pub cards_by_column: Vec<ColumnSummary>,
    pub wip_violations: i32,
    pub overdue_cards: i32,
    pub blocked_cards: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnSummary {
    pub column_id: Uuid,
    pub column_name: String,
    pub card_count: i32,
    pub wip_limit: Option<i32>,
    pub is_over_wip: bool,
}
