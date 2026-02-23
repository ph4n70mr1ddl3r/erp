use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use erp_core::Pagination;
use crate::db::AppState;
use erp_automation::{WorkflowService, WorkflowExecutionService, ScheduledJobService, WebhookService, ActionTemplateService};
use erp_automation::{AutomationWorkflow, AutomationType, AutomationStatus, WorkflowExecution, TriggerType, ExecutionStatus};

#[derive(Deserialize)]
pub struct CreateWorkflowRequest {
    pub name: String,
    pub code: String,
    pub category: String,
    pub automation_type: String,
    pub trigger_config: String,
    pub actions: String,
    pub description: Option<String>,
    pub schedule_cron: Option<String>,
}

#[derive(Deserialize)]
pub struct StartExecutionRequest {
    pub workflow_id: Uuid,
    pub trigger_type: String,
    pub input_data: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateScheduledJobRequest {
    pub name: String,
    pub job_type: String,
    pub schedule_cron: String,
    pub workflow_id: Option<Uuid>,
    pub parameters: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateWebhookRequest {
    pub name: String,
    pub path: String,
    pub workflow_id: Uuid,
    pub auth_type: Option<String>,
}

#[derive(Serialize)]
pub struct WorkflowResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub category: String,
    pub status: String,
    pub total_runs: i64,
    pub successful_runs: i64,
}

impl From<AutomationWorkflow> for WorkflowResponse {
    fn from(w: AutomationWorkflow) -> Self {
        Self {
            id: w.base.id,
            name: w.name,
            code: w.code,
            category: w.category,
            status: format!("{:?}", w.status),
            total_runs: w.total_runs,
            successful_runs: w.successful_runs,
        }
    }
}

#[derive(Serialize)]
pub struct ExecutionResponse {
    pub id: Uuid,
    pub workflow_id: Uuid,
    pub execution_number: String,
    pub status: String,
    pub progress_percent: i32,
    pub started_at: String,
}

impl From<WorkflowExecution> for ExecutionResponse {
    fn from(e: WorkflowExecution) -> Self {
        Self {
            id: e.base.id,
            workflow_id: e.workflow_id,
            execution_number: e.execution_number,
            status: format!("{:?}", e.status),
            progress_percent: e.progress_percent,
            started_at: e.started_at.to_rfc3339(),
        }
    }
}

pub async fn list_workflows(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = WorkflowService::new();
    let result = service.list(&state.pool, pagination).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "items": result.items.into_iter().map(WorkflowResponse::from).collect::<Vec<_>>(),
        "total": result.total
    })))
}

pub async fn get_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<WorkflowResponse>, (StatusCode, String)> {
    let service = WorkflowService::new();
    let workflow = service.get(&state.pool, id).await
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;
    Ok(Json(WorkflowResponse::from(workflow)))
}

pub async fn create_workflow(
    State(state): State<AppState>,
    Json(req): Json<CreateWorkflowRequest>,
) -> Result<Json<WorkflowResponse>, (StatusCode, String)> {
    let service = WorkflowService::new();
    
    let workflow = AutomationWorkflow {
        base: erp_core::BaseEntity::new(),
        name: req.name,
        code: req.code,
        description: req.description,
        category: req.category,
        automation_type: match req.automation_type.as_str() {
            "EventDriven" => AutomationType::EventDriven,
            "Trigger" => AutomationType::Trigger,
            "Webhook" => AutomationType::Webhook,
            "API" => AutomationType::API,
            "Manual" => AutomationType::Manual,
            "Recurring" => AutomationType::Recurring,
            _ => AutomationType::Scheduled,
        },
        trigger_config: req.trigger_config,
        actions: req.actions,
        schedule_cron: req.schedule_cron,
        status: AutomationStatus::Draft,
        ..Default::default()
    };
    
    let workflow = service.create(&state.pool, workflow).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    Ok(Json(WorkflowResponse::from(workflow)))
}

pub async fn publish_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<WorkflowResponse>, (StatusCode, String)> {
    let service = WorkflowService::new();
    let workflow = service.publish(&state.pool, id, Uuid::nil()).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(WorkflowResponse::from(workflow)))
}

pub async fn pause_workflow(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<WorkflowResponse>, (StatusCode, String)> {
    let service = WorkflowService::new();
    let workflow = service.pause(&state.pool, id).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(WorkflowResponse::from(workflow)))
}

pub async fn start_execution(
    State(state): State<AppState>,
    Json(req): Json<StartExecutionRequest>,
) -> Result<Json<ExecutionResponse>, (StatusCode, String)> {
    let service = WorkflowExecutionService::new();
    let trigger_type = match req.trigger_type.as_str() {
        "Schedule" => TriggerType::Schedule,
        "Webhook" => TriggerType::Webhook,
        "API" => TriggerType::API,
        "Event" => TriggerType::Event,
        _ => TriggerType::Manual,
    };
    
    let execution = service.start(&state.pool, req.workflow_id, trigger_type, None, req.input_data, None).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    Ok(Json(ExecutionResponse::from(execution)))
}

pub async fn get_execution(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ExecutionResponse>, (StatusCode, String)> {
    let service = WorkflowExecutionService::new();
    let execution = service.get(&state.pool, id).await
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;
    Ok(Json(ExecutionResponse::from(execution)))
}

pub async fn cancel_execution(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = WorkflowExecutionService::new();
    service.cancel(&state.pool, id).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn list_executions(
    State(state): State<AppState>,
    Path(workflow_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = WorkflowExecutionService::new();
    let executions = service.list_by_workflow(&state.pool, workflow_id, 100).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "executions": executions.into_iter().map(ExecutionResponse::from).collect::<Vec<_>>()
    })))
}

pub async fn list_scheduled_jobs(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let _ = state;
    Ok(Json(serde_json::json!({ "jobs": [] })))
}

pub async fn create_scheduled_job(
    State(state): State<AppState>,
    Json(req): Json<CreateScheduledJobRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = ScheduledJobService::new();
    
    let job = erp_automation::ScheduledJob {
        base: erp_core::BaseEntity::new(),
        name: req.name,
        job_type: req.job_type,
        schedule_cron: req.schedule_cron,
        workflow_id: req.workflow_id,
        parameters: req.parameters,
        ..Default::default()
    };
    
    let job = service.create(&state.pool, job).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "id": job.base.id,
        "name": job.name,
        "schedule_cron": job.schedule_cron,
        "next_run_at": job.next_run_at
    })))
}

pub async fn list_webhooks(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let _ = state;
    Ok(Json(serde_json::json!({ "webhooks": [] })))
}

pub async fn create_webhook(
    State(state): State<AppState>,
    Json(req): Json<CreateWebhookRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = WebhookService::new();
    let auth_type = match req.auth_type.as_deref() {
        Some("APIKey") => erp_automation::WebhookAuthType::APIKey,
        Some("BasicAuth") => erp_automation::WebhookAuthType::BasicAuth,
        Some("HMAC") => erp_automation::WebhookAuthType::HMAC,
        Some("JWT") => erp_automation::WebhookAuthType::JWT,
        _ => erp_automation::WebhookAuthType::None,
    };
    
    let webhook = service.create_endpoint(&state.pool, req.name, req.path, req.workflow_id, auth_type).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "id": webhook.base.id,
        "name": webhook.name,
        "endpoint_path": webhook.endpoint_path,
        "secret_key": webhook.secret_key
    })))
}

pub async fn list_action_templates(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = ActionTemplateService::new();
    let templates = service.list(&state.pool, None).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "templates": templates.into_iter().map(|t| serde_json::json!({
            "id": t.base.id,
            "name": t.name,
            "code": t.code,
            "category": t.category,
            "action_type": format!("{:?}", t.action_type)
        })).collect::<Vec<_>>()
    })))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/workflows", axum::routing::get(list_workflows).post(create_workflow))
        .route("/workflows/:id", axum::routing::get(get_workflow))
        .route("/workflows/:id/publish", axum::routing::post(publish_workflow))
        .route("/workflows/:id/pause", axum::routing::post(pause_workflow))
        .route("/executions", axum::routing::post(start_execution))
        .route("/executions/:id", axum::routing::get(get_execution))
        .route("/executions/:id/cancel", axum::routing::post(cancel_execution))
        .route("/workflows/:workflow_id/executions", axum::routing::get(list_executions))
        .route("/scheduled-jobs", axum::routing::get(list_scheduled_jobs).post(create_scheduled_job))
        .route("/webhooks", axum::routing::get(list_webhooks).post(create_webhook))
        .route("/action-templates", axum::routing::get(list_action_templates))
}
