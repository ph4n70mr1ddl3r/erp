use chrono::{DateTime, Utc};
use erp_core::models::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessDefinition {
    pub base: BaseEntity,
    pub name: String,
    pub description: String,
    pub category: ProcessCategory,
    pub version: String,
    pub status: ProcessStatus,
    pub owner_id: Option<Uuid>,
    pub start_event: String,
    pub end_events: Vec<String>,
    pub expected_duration_hours: Option<i32>,
    pub sla_hours: Option<i32>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum ProcessCategory {
    OrderToCash,
    ProcureToPay,
    RecordToReport,
    HireToRetire,
    IssueToResolution,
    PlanToProduce,
    DesignToBuild,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum ProcessStatus {
    Draft,
    Active,
    Deprecated,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInstance {
    pub base: BaseEntity,
    pub process_id: Uuid,
    pub case_id: String,
    pub status: InstanceStatus,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_hours: Option<f64>,
    pub initiator_id: Option<Uuid>,
    pub entity_type: Option<String>,
    pub entity_id: Option<Uuid>,
    pub variant_id: Option<Uuid>,
    pub is_compliant: Option<bool>,
    pub deviation_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum InstanceStatus {
    Running,
    Completed,
    Cancelled,
    Error,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessEvent {
    pub base: BaseEntity,
    pub instance_id: Uuid,
    pub event_type: EventType,
    pub activity_name: String,
    pub timestamp: DateTime<Utc>,
    pub user_id: Option<Uuid>,
    pub role_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub resource: Option<String>,
    pub previous_state: Option<String>,
    pub new_state: Option<String>,
    pub duration_ms: Option<i64>,
    pub metadata: serde_json::Value,
    pub cost_cents: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum EventType {
    Start,
    Complete,
    Assign,
    Reassign,
    Suspend,
    Resume,
    Cancel,
    Skip,
    Manual,
    Automated,
    Gateway,
    Timer,
    Message,
    Signal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessVariant {
    pub base: BaseEntity,
    pub process_id: Uuid,
    pub variant_hash: String,
    pub activity_sequence: Vec<String>,
    pub frequency: i64,
    pub percentage: f64,
    pub avg_duration_hours: f64,
    pub min_duration_hours: f64,
    pub max_duration_hours: f64,
    pub is_happy_path: bool,
    pub deviation_from_standard: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessDiscovery {
    pub base: BaseEntity,
    pub process_id: Uuid,
    pub discovery_date: DateTime<Utc>,
    pub total_cases: i64,
    pub total_events: i64,
    pub unique_activities: i64,
    pub unique_variants: i64,
    pub avg_case_duration_hours: f64,
    pub median_case_duration_hours: f64,
    pub activity_frequencies: serde_json::Value,
    pub transition_frequencies: serde_json::Value,
    pub start_activities: Vec<String>,
    pub end_activities: Vec<String>,
    pub self_loops: Vec<ActivityLoop>,
    pub rework_loops: Vec<ActivityLoop>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivityLoop {
    pub activity: String,
    pub count: i64,
    pub avg_iterations: f64,
    pub cases_affected: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BottleneckAnalysis {
    pub base: BaseEntity,
    pub process_id: Uuid,
    pub analysis_date: DateTime<Utc>,
    pub bottlenecks: Vec<Bottleneck>,
    pub waiting_time_analysis: Vec<WaitingTime>,
    pub resource_utilization: Vec<ResourceUtilization>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bottleneck {
    pub activity_name: String,
    pub avg_waiting_time_hours: f64,
    pub avg_processing_time_hours: f64,
    pub cases_affected: i64,
    pub impact_score: f64,
    pub root_causes: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaitingTime {
    pub from_activity: String,
    pub to_activity: String,
    pub avg_waiting_hours: f64,
    pub median_waiting_hours: f64,
    pub p95_waiting_hours: f64,
    pub frequency: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceUtilization {
    pub resource: String,
    pub resource_type: ResourceType,
    pub total_activities: i64,
    pub total_active_hours: f64,
    pub utilization_rate: f64,
    pub avg_activity_duration_hours: f64,
    pub workload_distribution: Vec<HourlyWorkload>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResourceType {
    User,
    Role,
    Department,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HourlyWorkload {
    pub hour: i32,
    pub activity_count: i64,
    pub avg_duration_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConformanceCheck {
    pub base: BaseEntity,
    pub process_id: Uuid,
    pub check_date: DateTime<Utc>,
    pub total_cases: i64,
    pub conformant_cases: i64,
    pub conformance_rate: f64,
    pub deviations: Vec<Deviation>,
    pub deviating_variants: Vec<VariantDeviation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Deviation {
    pub deviation_type: DeviationType,
    pub activity: Option<String>,
    pub from_activity: Option<String>,
    pub to_activity: Option<String>,
    pub frequency: i64,
    pub affected_cases: i64,
    pub severity: DeviationSeverity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviationType {
    MissingActivity,
    ExtraActivity,
    WrongOrder,
    DuplicateActivity,
    PrematureEnd,
    LateStart,
    UnauthorizedPerformer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviationSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantDeviation {
    pub variant_id: Uuid,
    pub conformance_score: f64,
    pub deviations: Vec<Deviation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetric {
    pub base: BaseEntity,
    pub process_id: Uuid,
    pub metric_type: MetricType,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub value: f64,
    pub previous_value: Option<f64>,
    pub change_percentage: Option<f64>,
    pub trend: Trend,
    pub target: Option<f64>,
    pub status: MetricStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricType {
    CycleTime,
    Throughput,
    CaseVolume,
    ErrorRate,
    ReworkRate,
    FirstPassYield,
    OnTimeDelivery,
    CostPerCase,
    ResourceEfficiency,
    AutomationRate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Trend {
    Improving,
    Declining,
    Stable,
    Volatile,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MetricStatus {
    OnTarget,
    AtRisk,
    BelowTarget,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessSimulation {
    pub base: BaseEntity,
    pub process_id: Uuid,
    pub simulation_name: String,
    pub scenario: SimulationScenario,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: SimulationStatus,
    pub results: Option<SimulationResults>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationScenario {
    pub name: String,
    pub modifications: Vec<ScenarioModification>,
    pub duration_days: i32,
    pub case_arrival_rate: f64,
    pub resource_allocation: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScenarioModification {
    pub modification_type: ModificationType,
    pub target: String,
    pub change: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModificationType {
    AddResource,
    RemoveResource,
    ChangeDuration,
    AddActivity,
    RemoveActivity,
    ChangeSequence,
    AutomateActivity,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum SimulationStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResults {
    pub avg_cycle_time_hours: f64,
    pub throughput_per_day: f64,
    pub resource_utilization: serde_json::Value,
    pub bottleneck_activities: Vec<String>,
    pub improvement_percentage: f64,
    pub projected_cost_savings: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessMiningDashboard {
    pub process_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub summary: ProcessSummary,
    pub top_variants: Vec<VariantSummary>,
    pub top_bottlenecks: Vec<Bottleneck>,
    pub performance_trends: Vec<PerformanceTrend>,
    pub resource_heatmap: serde_json::Value,
    pub case_distribution: CaseDistribution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessSummary {
    pub total_cases: i64,
    pub completed_cases: i64,
    pub active_cases: i64,
    pub avg_cycle_time_hours: f64,
    pub conformance_rate: f64,
    pub automation_rate: f64,
    pub rework_rate: f64,
    pub on_time_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariantSummary {
    pub variant_id: Uuid,
    pub activity_count: i32,
    pub frequency: i64,
    pub percentage: f64,
    pub avg_duration_hours: f64,
    pub is_happy_path: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceTrend {
    pub date: DateTime<Utc>,
    pub cycle_time_hours: f64,
    pub throughput: f64,
    pub conformance_rate: f64,
    pub case_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseDistribution {
    pub by_status: serde_json::Value,
    pub by_duration_range: serde_json::Value,
    pub by_variant: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportEventsRequest {
    pub process_id: Uuid,
    pub events: Vec<ProcessEventImport>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessEventImport {
    pub case_id: String,
    pub activity: String,
    pub timestamp: DateTime<Utc>,
    pub resource: Option<String>,
    pub event_type: Option<String>,
    pub metadata: Option<serde_json::Value>,
}
