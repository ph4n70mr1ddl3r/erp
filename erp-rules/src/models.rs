use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RuleType {
    Validation,
    Calculation,
    Transformation,
    Routing,
    Approval,
    Notification,
    Discount,
    Pricing,
    Workflow,
    BusinessConstraint,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RuleStatus {
    Draft,
    Active,
    Inactive,
    Deprecated,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LogicalOperator {
    And,
    Or,
    Not,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ComparisonOperator {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Contains,
    NotContains,
    StartsWith,
    EndsWith,
    In,
    NotIn,
    IsNull,
    IsNotNull,
    IsEmpty,
    IsNotEmpty,
    Between,
    Matches,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BusinessRule {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub rule_type: RuleType,
    pub entity_type: String,
    pub status: RuleStatus,
    pub priority: i32,
    pub version: i32,
    pub effective_from: Option<DateTime<Utc>>,
    pub effective_to: Option<DateTime<Utc>>,
    pub conditions: String,
    pub actions: String,
    pub else_actions: Option<String>,
    pub tags: Option<String>,
    pub owner_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    pub base: BaseEntity,
    pub rule_id: Uuid,
    pub group_id: Option<Uuid>,
    pub field: String,
    pub operator: ComparisonOperator,
    pub value: String,
    pub value_type: String,
    pub logical_operator: LogicalOperator,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConditionGroup {
    pub base: BaseEntity,
    pub rule_id: Uuid,
    pub parent_group_id: Option<Uuid>,
    pub logical_operator: LogicalOperator,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleAction {
    pub base: BaseEntity,
    pub rule_id: Uuid,
    pub action_type: String,
    pub target: String,
    pub parameters: Option<String>,
    pub execution_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSet {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub entity_type: String,
    pub status: RuleStatus,
    pub version: i32,
    pub effective_from: Option<DateTime<Utc>>,
    pub effective_to: Option<DateTime<Utc>>,
    pub execution_mode: ExecutionMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ExecutionMode {
    Sequential,
    Parallel,
    FirstMatch,
    AllMatches,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSetMember {
    pub base: BaseEntity,
    pub ruleset_id: Uuid,
    pub rule_id: Uuid,
    pub sort_order: i32,
    pub is_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleExecution {
    pub base: BaseEntity,
    pub rule_id: Uuid,
    pub ruleset_id: Option<Uuid>,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub triggered_at: DateTime<Utc>,
    pub conditions_evaluated: String,
    pub matched: bool,
    pub actions_executed: String,
    pub result: Option<String>,
    pub error: Option<String>,
    pub execution_time_ms: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleVariable {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub data_type: String,
    pub default_value: Option<String>,
    pub source_type: VariableSource,
    pub source_config: Option<String>,
    pub is_constant: bool,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum VariableSource {
    Static,
    Field,
    Function,
    Query,
    Api,
    Context,
    User,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleFunction {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub return_type: String,
    pub parameters: Option<String>,
    pub function_body: Option<String>,
    pub is_builtin: bool,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleTemplate {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub rule_type: RuleType,
    pub entity_type: String,
    pub template: String,
    pub variables: Option<String>,
    pub is_builtin: bool,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionTable {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub entity_type: String,
    pub input_columns: String,
    pub output_columns: String,
    pub hit_policy: HitPolicy,
    pub status: RuleStatus,
    pub version: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum HitPolicy {
    First,
    Unique,
    Any,
    Priority,
    Collect,
    RuleOrder,
    OutputOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DecisionTableRow {
    pub base: BaseEntity,
    pub table_id: Uuid,
    pub row_number: i32,
    pub inputs: String,
    pub outputs: String,
    pub description: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleVersion {
    pub base: BaseEntity,
    pub rule_id: Uuid,
    pub version: i32,
    pub conditions: String,
    pub actions: String,
    pub changed_by: Option<Uuid>,
    pub changed_at: DateTime<Utc>,
    pub change_reason: Option<String>,
    pub status: RuleStatus,
}
