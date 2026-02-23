use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceTicket {
    pub base: BaseEntity,
    pub ticket_number: String,
    pub subject: String,
    pub description: String,
    pub customer_id: Option<Uuid>,
    pub contact_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub team_id: Option<Uuid>,
    pub priority: TicketPriority,
    pub status: TicketStatus,
    pub ticket_type: TicketType,
    pub source: TicketSource,
    pub category_id: Option<Uuid>,
    pub sla_id: Option<Uuid>,
    pub due_date: Option<DateTime<Utc>>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub first_response_at: Option<DateTime<Utc>>,
    pub satisfaction_rating: Option<i32>,
    pub satisfaction_comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum TicketPriority {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum TicketStatus {
    New,
    Open,
    Pending,
    OnHold,
    Resolved,
    Closed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TicketType {
    Incident,
    ServiceRequest,
    Problem,
    ChangeRequest,
    Information,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TicketSource {
    Email,
    Phone,
    WebPortal,
    Chat,
    Api,
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketComment {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub author_id: Uuid,
    pub author_type: CommentAuthorType,
    pub body: String,
    pub is_internal: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CommentAuthorType {
    Agent,
    Customer,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketAttachment {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub comment_id: Option<Uuid>,
    pub filename: String,
    pub file_path: String,
    pub file_size: i64,
    pub content_type: String,
    pub uploaded_by: Uuid,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLA {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub response_time_hours: i32,
    pub resolution_time_hours: i32,
    pub business_hours_only: bool,
    pub timezone: String,
    pub escalation_rule_id: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SLAPolicy {
    pub id: Uuid,
    pub sla_id: Uuid,
    pub priority: TicketPriority,
    pub response_time_hours: i32,
    pub resolution_time_hours: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationRule {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub conditions: Vec<EscalationCondition>,
    pub actions: Vec<EscalationAction>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationCondition {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub condition_type: EscalationConditionType,
    pub operator: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EscalationConditionType {
    TimeElapsed,
    Priority,
    Status,
    AssignedGroup,
    CustomerTier,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EscalationAction {
    pub id: Uuid,
    pub rule_id: Uuid,
    pub action_type: EscalationActionType,
    pub target_id: Option<Uuid>,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EscalationActionType {
    AssignToAgent,
    AssignToTeam,
    NotifyAgent,
    NotifyManager,
    NotifyCustomer,
    ChangePriority,
    ChangeStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketCategory {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub sla_id: Option<Uuid>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceTeam {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub manager_id: Option<Uuid>,
    pub members: Vec<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeArticle {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub summary: Option<String>,
    pub category_id: Option<Uuid>,
    pub author_id: Uuid,
    pub article_type: ArticleType,
    pub status: ArticleStatus,
    pub view_count: i64,
    pub helpful_count: i64,
    pub not_helpful_count: i64,
    pub tags: Vec<String>,
    pub published_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ArticleType {
    KnowledgeBase,
    HowTo,
    Troubleshooting,
    FAQ,
    Policy,
    BestPractice,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ArticleStatus {
    Draft,
    InReview,
    Published,
    Archived,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeCategory {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub position: i32,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCatalog {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub owner_id: Option<Uuid>,
    pub approval_required: bool,
    pub sla_id: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceCatalogItem {
    pub id: Uuid,
    pub catalog_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub service_type: CatalogServiceType,
    pub price: i64,
    pub currency: String,
    pub delivery_time_hours: i32,
    pub form_schema: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CatalogServiceType {
    Request,
    Incident,
    Change,
    Access,
    Information,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceContract {
    pub id: Uuid,
    pub contract_number: String,
    pub customer_id: Uuid,
    pub name: String,
    pub contract_type: ContractType,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub sla_id: Option<Uuid>,
    pub max_tickets: Option<i32>,
    pub max_hours: Option<f64>,
    pub used_tickets: i32,
    pub used_hours: f64,
    pub value: i64,
    pub currency: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ContractType {
    PerIncident,
    HoursBased,
    Unlimited,
    Retainer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Problem {
    pub id: Uuid,
    pub problem_number: String,
    pub title: String,
    pub description: String,
    pub root_cause: Option<String>,
    pub workaround: Option<String>,
    pub impact: ProblemImpact,
    pub urgency: ProblemUrgency,
    pub priority: TicketPriority,
    pub status: ProblemStatus,
    pub assigned_to: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub related_incidents: Vec<Uuid>,
    pub resolution: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ProblemImpact {
    Extensive,
    Significant,
    Moderate,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ProblemUrgency {
    Critical,
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ProblemStatus {
    New,
    Investigating,
    RootCauseIdentified,
    WorkaroundAvailable,
    Resolved,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeRequest {
    pub id: Uuid,
    pub change_number: String,
    pub title: String,
    pub description: String,
    pub reason: String,
    pub change_type: ChangeType,
    pub risk_level: RiskLevel,
    pub impact_assessment: Option<String>,
    pub rollback_plan: Option<String>,
    pub status: ChangeStatus,
    pub requester_id: Uuid,
    pub approver_id: Option<Uuid>,
    pub assigned_to: Option<Uuid>,
    pub planned_start: Option<DateTime<Utc>>,
    pub planned_end: Option<DateTime<Utc>>,
    pub actual_start: Option<DateTime<Utc>>,
    pub actual_end: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ChangeType {
    Standard,
    Normal,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RiskLevel {
    High,
    Medium,
    Low,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ChangeStatus {
    Draft,
    Submitted,
    Assessment,
    Planning,
    Approved,
    Scheduled,
    InProgress,
    Completed,
    Failed,
    Cancelled,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeTask {
    pub id: Uuid,
    pub change_request_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub sequence: i32,
    pub status: TaskStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRelation {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub asset_id: Uuid,
    pub relation_type: AssetRelationType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AssetRelationType {
    Affected,
    CausedBy,
    Related,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CannedResponse {
    pub id: Uuid,
    pub name: String,
    pub subject: Option<String>,
    pub body: String,
    pub category: Option<String>,
    pub tags: Vec<String>,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketMerge {
    pub id: Uuid,
    pub primary_ticket_id: Uuid,
    pub merged_ticket_id: Uuid,
    pub merged_by: Uuid,
    pub merged_at: DateTime<Utc>,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerContact {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub title: Option<String>,
    pub is_primary: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetric {
    pub id: Uuid,
    pub metric_date: NaiveDate,
    pub team_id: Option<Uuid>,
    pub agent_id: Option<Uuid>,
    pub tickets_created: i32,
    pub tickets_resolved: i32,
    pub tickets_open: i32,
    pub avg_first_response_hours: f64,
    pub avg_resolution_hours: f64,
    pub sla_breached: i32,
    pub customer_satisfaction: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentWorkload {
    pub agent_id: Uuid,
    pub open_tickets: i32,
    pub overdue_tickets: i32,
    pub avg_resolution_hours: f64,
    pub capacity: i32,
    pub availability: AgentAvailability,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AgentAvailability {
    Online,
    Busy,
    Away,
    Offline,
}
