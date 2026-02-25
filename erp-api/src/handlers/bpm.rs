use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::AppState;
use crate::ApiResult;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/processes", get(list_processes).post(create_process))
        .route("/processes/:id", get(get_process))
        .route("/processes/:id/publish", post(publish_process))
        .route("/processes/:id/nodes", post(add_node))
        .route("/processes/:id/flows", post(add_flow))
        .route("/instances", get(list_instances).post(start_instance))
        .route("/instances/:id", get(get_instance))
        .route("/tasks", get(list_user_tasks))
        .route("/tasks/:id/claim", post(claim_task))
        .route("/tasks/:id/complete", post(complete_task))
}

#[derive(Deserialize)]
pub struct CreateProcessRequest {
    pub name: String,
    pub code: String,
    pub category: String,
    pub owner_id: String,
    pub diagram_data: serde_json::Value,
}

#[derive(Serialize)]
pub struct ProcessResponse {
    pub id: String,
    pub name: String,
    pub code: String,
    pub category: String,
    pub status: String,
    pub version: i32,
}

pub async fn create_process(
    State(state): State<AppState>,
    Json(req): Json<CreateProcessRequest>,
) -> ApiResult<Json<ProcessResponse>> {
    let owner_id = Uuid::parse_str(&req.owner_id)?;
    let service = erp_bpm::BPMService::new();
    let process = service.create_process(&state.pool, req.name, req.code, req.category, owner_id, req.diagram_data).await?;

    Ok(Json(ProcessResponse {
        id: process.base.id.to_string(),
        name: process.name,
        code: process.code,
        category: process.category,
        status: format!("{:?}", process.status),
        version: process.version,
    }))
}

pub async fn list_processes(State(state): State<AppState>) -> ApiResult<Json<Vec<ProcessResponse>>> {
    let service = erp_bpm::BPMService::new();
    let processes = service.list_processes(&state.pool, None).await?;

    Ok(Json(processes.into_iter().map(|p| ProcessResponse {
        id: p.base.id.to_string(),
        name: p.name,
        code: p.code,
        category: p.category,
        status: format!("{:?}", p.status),
        version: p.version,
    }).collect()))
}

pub async fn get_process(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let id = Uuid::parse_str(&id)?;
    let service = erp_bpm::BPMService::new();
    let process = service.get_process(&state.pool, id).await?.ok_or_else(|| anyhow::anyhow!("Process not found"))?;

    Ok(Json(serde_json::json!({
        "id": process.base.id.to_string(),
        "name": process.name,
        "code": process.code,
        "description": process.description,
        "category": process.category,
        "version": process.version,
        "status": format!("{:?}", process.status),
        "diagram_data": process.diagram_data,
        "variables": process.variables,
        "owner_id": process.owner_id.to_string()
    })))
}

pub async fn publish_process(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(req): Json<PublishRequest>,
) -> ApiResult<Json<ProcessResponse>> {
    let id = Uuid::parse_str(&id)?;
    let published_by = Uuid::parse_str(&req.published_by)?;
    let service = erp_bpm::BPMService::new();
    let process = service.publish_process(&state.pool, id, published_by).await?;

    Ok(Json(ProcessResponse {
        id: process.base.id.to_string(),
        name: process.name,
        code: process.code,
        category: process.category,
        status: format!("{:?}", process.status),
        version: process.version,
    }))
}

#[derive(Deserialize)]
pub struct PublishRequest {
    pub published_by: String,
}

#[derive(Deserialize)]
pub struct AddNodeRequest {
    pub node_id: String,
    pub name: String,
    pub task_type: String,
    pub position_x: i32,
    pub position_y: i32,
}

pub async fn add_node(
    State(state): State<AppState>,
    axum::extract::Path(process_id): axum::extract::Path<String>,
    Json(req): Json<AddNodeRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let process_id = Uuid::parse_str(&process_id)?;
    let task_type = match req.task_type.as_str() {
        "ServiceTask" => erp_bpm::TaskType::ServiceTask,
        "ScriptTask" => erp_bpm::TaskType::ScriptTask,
        "Gateway" => erp_bpm::TaskType::Gateway,
        "Event" => erp_bpm::TaskType::Event,
        "SubProcess" => erp_bpm::TaskType::SubProcess,
        _ => erp_bpm::TaskType::UserTask,
    };

    let service = erp_bpm::BPMService::new();
    let node = service.add_node(&state.pool, process_id, req.node_id, req.name, task_type, req.position_x, req.position_y).await?;

    Ok(Json(serde_json::json!({
        "id": node.base.id.to_string(),
        "node_id": node.node_id,
        "name": node.name,
        "task_type": format!("{:?}", node.task_type)
    })))
}

#[derive(Deserialize)]
pub struct AddFlowRequest {
    pub flow_id: String,
    pub source_node_id: String,
    pub target_node_id: String,
    pub condition_expression: Option<String>,
}

pub async fn add_flow(
    State(state): State<AppState>,
    axum::extract::Path(process_id): axum::extract::Path<String>,
    Json(req): Json<AddFlowRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let process_id = Uuid::parse_str(&process_id)?;
    let service = erp_bpm::BPMService::new();
    let flow = service.add_flow(&state.pool, process_id, req.flow_id, req.source_node_id, req.target_node_id, req.condition_expression).await?;

    Ok(Json(serde_json::json!({
        "id": flow.base.id.to_string(),
        "flow_id": flow.flow_id,
        "source_node_id": flow.source_node_id,
        "target_node_id": flow.target_node_id,
        "is_default": flow.is_default
    })))
}

#[derive(Deserialize)]
pub struct StartInstanceRequest {
    pub process_definition_id: String,
    pub started_by: String,
    pub business_key: Option<String>,
    pub variables: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct InstanceResponse {
    pub id: String,
    pub process_definition_id: String,
    pub status: String,
    pub started_by: String,
    pub started_at: String,
}

pub async fn start_instance(
    State(state): State<AppState>,
    Json(req): Json<StartInstanceRequest>,
) -> ApiResult<Json<InstanceResponse>> {
    let process_definition_id = Uuid::parse_str(&req.process_definition_id)?;
    let started_by = Uuid::parse_str(&req.started_by)?;

    let service = erp_bpm::BPMService::new();
    let instance = service.start_instance(&state.pool, process_definition_id, started_by, req.business_key, req.variables).await?;

    Ok(Json(InstanceResponse {
        id: instance.base.id.to_string(),
        process_definition_id: instance.process_definition_id.to_string(),
        status: format!("{:?}", instance.status),
        started_by: instance.started_by.to_string(),
        started_at: instance.started_at.to_rfc3339(),
    }))
}

pub async fn list_instances(State(state): State<AppState>) -> ApiResult<Json<Vec<InstanceResponse>>> {
    let service = erp_bpm::BPMService::new();
    let instances = service.list_active_instances(&state.pool).await?;

    Ok(Json(instances.into_iter().map(|i| InstanceResponse {
        id: i.base.id.to_string(),
        process_definition_id: i.process_definition_id.to_string(),
        status: format!("{:?}", i.status),
        started_by: i.started_by.to_string(),
        started_at: i.started_at.to_rfc3339(),
    }).collect()))
}

pub async fn get_instance(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let id = Uuid::parse_str(&id)?;
    let service = erp_bpm::BPMService::new();
    let instance = service.get_instance(&state.pool, id).await?.ok_or_else(|| anyhow::anyhow!("Instance not found"))?;

    Ok(Json(serde_json::json!({
        "id": instance.base.id.to_string(),
        "process_definition_id": instance.process_definition_id.to_string(),
        "business_key": instance.business_key,
        "status": format!("{:?}", instance.status),
        "variables": instance.variables,
        "started_by": instance.started_by.to_string(),
        "started_at": instance.started_at.to_rfc3339(),
        "current_node_id": instance.current_node_id
    })))
}

#[derive(Serialize)]
pub struct TaskResponse {
    pub id: String,
    pub process_instance_id: String,
    pub name: String,
    pub status: String,
    pub assignee_id: Option<String>,
    pub priority: i32,
}

pub async fn list_user_tasks(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> ApiResult<Json<Vec<TaskResponse>>> {
    let user_id = params.get("user_id")
        .ok_or_else(|| anyhow::anyhow!("user_id required"))?;
    let user_id = Uuid::parse_str(user_id)?;

    let service = erp_bpm::BPMService::new();
    let tasks = service.get_user_tasks(&state.pool, user_id).await?;

    Ok(Json(tasks.into_iter().map(|t| TaskResponse {
        id: t.base.id.to_string(),
        process_instance_id: t.process_instance_id.to_string(),
        name: t.name,
        status: format!("{:?}", t.status),
        assignee_id: t.assignee_id.map(|id| id.to_string()),
        priority: t.priority,
    }).collect()))
}

#[derive(Deserialize)]
pub struct ClaimTaskRequest {
    pub user_id: String,
}

pub async fn claim_task(
    State(state): State<AppState>,
    axum::extract::Path(task_id): axum::extract::Path<String>,
    Json(req): Json<ClaimTaskRequest>,
) -> ApiResult<Json<TaskResponse>> {
    let task_id = Uuid::parse_str(&task_id)?;
    let user_id = Uuid::parse_str(&req.user_id)?;

    let service = erp_bpm::BPMService::new();
    let task = service.claim_task(&state.pool, task_id, user_id).await?;

    Ok(Json(TaskResponse {
        id: task.base.id.to_string(),
        process_instance_id: task.process_instance_id.to_string(),
        name: task.name,
        status: format!("{:?}", task.status),
        assignee_id: task.assignee_id.map(|id| id.to_string()),
        priority: task.priority,
    }))
}

#[derive(Deserialize)]
pub struct CompleteTaskRequest {
    pub user_id: String,
    pub outcome: Option<String>,
}

pub async fn complete_task(
    State(state): State<AppState>,
    axum::extract::Path(task_id): axum::extract::Path<String>,
    Json(req): Json<CompleteTaskRequest>,
) -> ApiResult<Json<TaskResponse>> {
    let task_id = Uuid::parse_str(&task_id)?;
    let user_id = Uuid::parse_str(&req.user_id)?;

    let service = erp_bpm::BPMService::new();
    let task = service.complete_task(&state.pool, task_id, user_id, req.outcome).await?;

    Ok(Json(TaskResponse {
        id: task.base.id.to_string(),
        process_instance_id: task.process_instance_id.to_string(),
        name: task.name,
        status: format!("{:?}", task.status),
        assignee_id: task.assignee_id.map(|id| id.to_string()),
        priority: task.priority,
    }))
}
