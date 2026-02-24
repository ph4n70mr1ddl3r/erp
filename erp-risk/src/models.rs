use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RiskCategory {
    Strategic,
    Operational,
    Financial,
    Compliance,
    Reputational,
    Technology,
    Environmental,
    SupplyChain,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RiskLevel {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RiskStatus {
    Identified,
    Assessing,
    Mitigating,
    Monitoring,
    Closed,
    Accepted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Risk {
    pub id: Uuid,
    pub code: String,
    pub title: String,
    pub description: String,
    pub category: RiskCategory,
    pub status: RiskStatus,
    pub probability: f64,
    pub impact: RiskLevel,
    pub risk_score: i32,
    pub inherent_risk_level: RiskLevel,
    pub residual_risk_level: RiskLevel,
    pub owner_id: Option<Uuid>,
    pub department: Option<String>,
    pub identified_date: DateTime<Utc>,
    pub target_resolution_date: Option<DateTime<Utc>>,
    pub actual_resolution_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub id: Uuid,
    pub risk_id: Uuid,
    pub assessment_date: DateTime<Utc>,
    pub assessor_id: Option<Uuid>,
    pub probability_before: f64,
    pub impact_before: RiskLevel,
    pub probability_after: f64,
    pub impact_after: RiskLevel,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationPlan {
    pub id: Uuid,
    pub risk_id: Uuid,
    pub title: String,
    pub description: String,
    pub strategy: String,
    pub owner_id: Option<Uuid>,
    pub status: String,
    pub priority: String,
    pub start_date: DateTime<Utc>,
    pub target_date: DateTime<Utc>,
    pub completion_date: Option<DateTime<Utc>>,
    pub budget: i64,
    pub actual_cost: i64,
    pub effectiveness: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MitigationTask {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub status: String,
    pub due_date: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskControl {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: String,
    pub control_type: String,
    pub frequency: String,
    pub owner_id: Option<Uuid>,
    pub effectiveness: String,
    pub last_test_date: Option<DateTime<Utc>>,
    pub next_test_date: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskControlMapping {
    pub id: Uuid,
    pub risk_id: Uuid,
    pub control_id: Uuid,
    pub control_effectiveness: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskIncident {
    pub id: Uuid,
    pub risk_id: Uuid,
    pub incident_number: String,
    pub title: String,
    pub description: String,
    pub incident_date: DateTime<Utc>,
    pub detected_date: DateTime<Utc>,
    pub reported_by: Option<Uuid>,
    pub impact_amount: i64,
    pub currency: String,
    pub status: String,
    pub root_cause: Option<String>,
    pub lessons_learned: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskRegister {
    pub id: Uuid,
    pub name: String,
    pub department: Option<String>,
    pub owner_id: Option<Uuid>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRiskRequest {
    pub title: String,
    pub description: String,
    pub category: RiskCategory,
    pub probability: f64,
    pub impact: RiskLevel,
    pub owner_id: Option<Uuid>,
    pub department: Option<String>,
    pub target_resolution_date: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMitigationRequest {
    pub risk_id: Uuid,
    pub title: String,
    pub description: String,
    pub strategy: String,
    pub owner_id: Option<Uuid>,
    pub priority: String,
    pub target_date: DateTime<Utc>,
    pub budget: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskDashboard {
    pub total_risks: i32,
    pub high_risks: i32,
    pub medium_risks: i32,
    pub low_risks: i32,
    pub open_incidents: i32,
    pub mitigations_in_progress: i32,
    pub overdue_mitigations: i32,
    pub risk_by_category: serde_json::Value,
    pub risk_trend: serde_json::Value,
}
