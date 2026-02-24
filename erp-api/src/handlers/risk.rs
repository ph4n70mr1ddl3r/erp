use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::db::AppState;
use erp_risk::{RiskService, MitigationService, RiskControlService, CreateRiskRequest, CreateMitigationRequest, RiskCategory, RiskLevel};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/risks", get(list_risks).post(create_risk))
        .route("/risks/:id", get(get_risk))
        .route("/risks/:id/status", post(update_risk_status))
        .route("/risks/:id/assess", post(assess_risk))
        .route("/risks/dashboard", get(get_dashboard))
        .route("/mitigations", post(create_mitigation))
        .route("/mitigations/:plan_id/tasks", post(add_mitigation_task))
        .route("/mitigations/tasks/:task_id/complete", post(complete_task))
        .route("/controls", post(create_control))
        .route("/controls/:control_id/link/:risk_id", post(link_control))
}

#[derive(Deserialize)]
pub struct CreateRiskBody {
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub category: String,
    #[serde(default = "default_probability")]
    pub probability: f64,
    #[serde(default)]
    pub impact: String,
    pub owner_id: Option<Uuid>,
    pub department: Option<String>,
    pub target_resolution_date: Option<chrono::DateTime<chrono::Utc>>,
}

fn default_probability() -> f64 { 0.5 }

async fn create_risk(
    State(state): State<AppState>,
    Json(body): Json<CreateRiskBody>,
) -> Json<serde_json::Value> {
    let req = CreateRiskRequest {
        title: body.title,
        description: body.description,
        category: match body.category.as_str() {
            "Strategic" => RiskCategory::Strategic,
            "Financial" => RiskCategory::Financial,
            "Compliance" => RiskCategory::Compliance,
            "Reputational" => RiskCategory::Reputational,
            "Technology" => RiskCategory::Technology,
            "Environmental" => RiskCategory::Environmental,
            "SupplyChain" => RiskCategory::SupplyChain,
            _ => RiskCategory::Operational,
        },
        probability: body.probability,
        impact: match body.impact.as_str() {
            "VeryLow" => RiskLevel::VeryLow,
            "Low" => RiskLevel::Low,
            "High" => RiskLevel::High,
            "VeryHigh" => RiskLevel::VeryHigh,
            "Critical" => RiskLevel::Critical,
            _ => RiskLevel::Medium,
        },
        owner_id: body.owner_id,
        department: body.department,
        target_resolution_date: body.target_resolution_date,
    };
    match RiskService::create(&state.pool, req, None).await {
        Ok(risk) => Json(json!({
            "id": risk.id,
            "code": risk.code,
            "title": risk.title,
            "category": risk.category,
            "risk_score": risk.risk_score,
            "status": risk.status
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn list_risks(State(state): State<AppState>) -> Json<serde_json::Value> {
    match RiskService::list(&state.pool).await {
        Ok(risks) => Json(json!({
            "items": risks.iter().map(|r| json!({
                "id": r.id,
                "code": r.code,
                "title": r.title,
                "category": r.category,
                "risk_score": r.risk_score,
                "residual_risk_level": r.residual_risk_level,
                "status": r.status
            })).collect::<Vec<_>>()
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn get_risk(State(state): State<AppState>, Path(id): Path<Uuid>) -> Json<serde_json::Value> {
    match RiskService::get(&state.pool, id).await {
        Ok(Some(risk)) => Json(json!({
            "id": risk.id,
            "code": risk.code,
            "title": risk.title,
            "description": risk.description,
            "category": risk.category,
            "probability": risk.probability,
            "impact": risk.impact,
            "risk_score": risk.risk_score,
            "inherent_risk_level": risk.inherent_risk_level,
            "residual_risk_level": risk.residual_risk_level,
            "status": risk.status,
            "owner_id": risk.owner_id,
            "department": risk.department
        })),
        Ok(None) => Json(json!({ "error": "Risk not found" })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct UpdateStatusBody {
    pub status: String,
}

async fn update_risk_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateStatusBody>,
) -> Json<serde_json::Value> {
    let status = match body.status.as_str() {
        "Assessing" => erp_risk::RiskStatus::Assessing,
        "Mitigating" => erp_risk::RiskStatus::Mitigating,
        "Monitoring" => erp_risk::RiskStatus::Monitoring,
        "Closed" => erp_risk::RiskStatus::Closed,
        "Accepted" => erp_risk::RiskStatus::Accepted,
        _ => erp_risk::RiskStatus::Identified,
    };
    match RiskService::update_status(&state.pool, id, status).await {
        Ok(risk) => Json(json!({
            "id": risk.id,
            "status": risk.status
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct AssessRiskBody {
    pub probability: f64,
    pub impact: String,
}

async fn assess_risk(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<AssessRiskBody>,
) -> Json<serde_json::Value> {
    let impact = match body.impact.as_str() {
        "VeryLow" => RiskLevel::VeryLow,
        "Low" => RiskLevel::Low,
        "High" => RiskLevel::High,
        "VeryHigh" => RiskLevel::VeryHigh,
        "Critical" => RiskLevel::Critical,
        _ => RiskLevel::Medium,
    };
    match RiskService::assess(&state.pool, id, body.probability, impact, None).await {
        Ok(assessment) => Json(json!({
            "id": assessment.id,
            "probability_after": assessment.probability_after,
            "impact_after": assessment.impact_after
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn get_dashboard(State(state): State<AppState>) -> Json<serde_json::Value> {
    match RiskService::get_dashboard(&state.pool).await {
        Ok(dashboard) => Json(json!({
            "total_risks": dashboard.total_risks,
            "high_risks": dashboard.high_risks,
            "medium_risks": dashboard.medium_risks,
            "low_risks": dashboard.low_risks,
            "open_incidents": dashboard.open_incidents,
            "mitigations_in_progress": dashboard.mitigations_in_progress,
            "overdue_mitigations": dashboard.overdue_mitigations,
            "risk_by_category": dashboard.risk_by_category
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct CreateMitigationBody {
    pub risk_id: Uuid,
    pub title: String,
    pub description: String,
    pub strategy: String,
    pub owner_id: Option<Uuid>,
    #[serde(default = "default_priority")]
    pub priority: String,
    pub target_date: chrono::DateTime<chrono::Utc>,
    #[serde(default)]
    pub budget: i64,
}

fn default_priority() -> String { "Medium".to_string() }

async fn create_mitigation(
    State(state): State<AppState>,
    Json(body): Json<CreateMitigationBody>,
) -> Json<serde_json::Value> {
    let req = CreateMitigationRequest {
        risk_id: body.risk_id,
        title: body.title,
        description: body.description,
        strategy: body.strategy,
        owner_id: body.owner_id,
        priority: body.priority,
        target_date: body.target_date,
        budget: body.budget,
    };
    match MitigationService::create(&state.pool, req).await {
        Ok(plan) => Json(json!({
            "id": plan.id,
            "risk_id": plan.risk_id,
            "title": plan.title,
            "strategy": plan.strategy,
            "status": plan.status,
            "priority": plan.priority
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct AddTaskBody {
    pub title: String,
    pub assigned_to: Option<Uuid>,
    pub due_date: chrono::DateTime<chrono::Utc>,
}

async fn add_mitigation_task(
    State(state): State<AppState>,
    Path(plan_id): Path<Uuid>,
    Json(body): Json<AddTaskBody>,
) -> Json<serde_json::Value> {
    match MitigationService::add_task(&state.pool, plan_id, body.title, body.assigned_to, body.due_date).await {
        Ok(task) => Json(json!({
            "id": task.id,
            "title": task.title,
            "status": task.status,
            "due_date": task.due_date
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn complete_task(
    State(state): State<AppState>,
    Path(task_id): Path<Uuid>,
) -> Json<serde_json::Value> {
    match MitigationService::complete_task(&state.pool, task_id).await {
        Ok(task) => Json(json!({
            "id": task.id,
            "status": task.status,
            "completed_at": task.completed_at
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct CreateControlBody {
    pub code: String,
    pub name: String,
    pub description: String,
    #[serde(default = "default_control_type")]
    pub control_type: String,
    #[serde(default = "default_frequency")]
    pub frequency: String,
    pub owner_id: Option<Uuid>,
}

fn default_control_type() -> String { "Preventive".to_string() }
fn default_frequency() -> String { "Continuous".to_string() }

async fn create_control(
    State(state): State<AppState>,
    Json(body): Json<CreateControlBody>,
) -> Json<serde_json::Value> {
    match RiskControlService::create(&state.pool, body.code, body.name, body.description, body.control_type, body.frequency, body.owner_id).await {
        Ok(control) => Json(json!({
            "id": control.id,
            "code": control.code,
            "name": control.name,
            "control_type": control.control_type,
            "is_active": control.is_active
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct LinkControlBody {
    #[serde(default = "default_effectiveness")]
    pub effectiveness: String,
}

fn default_effectiveness() -> String { "Effective".to_string() }

async fn link_control(
    State(state): State<AppState>,
    Path((control_id, risk_id)): Path<(Uuid, Uuid)>,
    Json(body): Json<LinkControlBody>,
) -> Json<serde_json::Value> {
    match RiskControlService::link_to_risk(&state.pool, risk_id, control_id, body.effectiveness).await {
        Ok(mapping) => Json(json!({
            "id": mapping.id,
            "risk_id": mapping.risk_id,
            "control_id": mapping.control_id,
            "control_effectiveness": mapping.control_effectiveness
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}
