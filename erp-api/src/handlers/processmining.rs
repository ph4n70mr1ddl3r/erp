use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::db::AppState;
use crate::handlers::ApiResult;
use erp_processmining::{ProcessMiningService, ProcessDefinition, ProcessCategory, ProcessStatus, ImportEventsRequest, ProcessEventImport, SimulationScenario};

#[derive(Deserialize)]
pub struct ListProcessesQuery {
    category: Option<String>,
    status: Option<String>,
}

#[derive(Deserialize)]
pub struct DashboardQuery {
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/processes", axum::routing::get(list_processes).post(create_process))
        .route("/processes/:id", axum::routing::get(get_process).delete(delete_process))
        .route("/processes/:id/discovery", axum::routing::post(run_discovery))
        .route("/processes/:id/bottlenecks", axum::routing::post(analyze_bottlenecks))
        .route("/processes/:id/conformance", axum::routing::post(check_conformance))
        .route("/processes/:id/dashboard", axum::routing::get(get_dashboard))
        .route("/events/import", axum::routing::post(import_events))
        .route("/simulations", axum::routing::post(create_simulation))
        .route("/simulations/:id", axum::routing::get(get_simulation))
        .route("/simulations/:id/run", axum::routing::post(run_simulation))
}

async fn create_process(
    State(_state): State<AppState>,
    Json(_process): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(_process))
}

async fn get_process(
    Path(id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = ProcessMiningService::new();
    let process = service.get_process(id).await?;
    Ok(Json(serde_json::to_value(process)?))
}

async fn list_processes(
    Query(query): Query<ListProcessesQuery>,
    State(_state): State<AppState>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let service = ProcessMiningService::new();
    let category = query.category.and_then(|c| match c.as_str() {
        "order_to_cash" => Some(ProcessCategory::OrderToCash),
        "procure_to_pay" => Some(ProcessCategory::ProcureToPay),
        "record_to_report" => Some(ProcessCategory::RecordToReport),
        "hire_to_retire" => Some(ProcessCategory::HireToRetire),
        "issue_to_resolution" => Some(ProcessCategory::IssueToResolution),
        "plan_to_produce" => Some(ProcessCategory::PlanToProduce),
        "design_to_build" => Some(ProcessCategory::DesignToBuild),
        _ => None,
    });
    let status = query.status.and_then(|s| match s.as_str() {
        "draft" => Some(ProcessStatus::Draft),
        "active" => Some(ProcessStatus::Active),
        "deprecated" => Some(ProcessStatus::Deprecated),
        "archived" => Some(ProcessStatus::Archived),
        _ => None,
    });
    let processes = service.list_processes(category, status).await?;
    Ok(Json(processes.into_iter().map(|p| serde_json::to_value(p).unwrap()).collect()))
}

async fn delete_process(
    Path(_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

async fn run_discovery(
    Path(process_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = ProcessMiningService::new();
    let discovery = service.run_discovery(process_id).await?;
    Ok(Json(serde_json::to_value(discovery)?))
}

async fn analyze_bottlenecks(
    Path(process_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = ProcessMiningService::new();
    let analysis = service.analyze_bottlenecks(process_id).await?;
    Ok(Json(serde_json::to_value(analysis)?))
}

async fn check_conformance(
    Path(process_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = ProcessMiningService::new();
    let check = service.check_conformance(process_id).await?;
    Ok(Json(serde_json::to_value(check)?))
}

async fn get_dashboard(
    Path(process_id): Path<Uuid>,
    Query(query): Query<DashboardQuery>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = ProcessMiningService::new();
    let start = query.start_date.unwrap_or_else(|| Utc::now() - chrono::Duration::days(30));
    let end = query.end_date.unwrap_or_else(Utc::now);
    let dashboard = service.get_dashboard(process_id, start, end).await?;
    Ok(Json(serde_json::to_value(dashboard)?))
}

#[derive(Deserialize)]
struct ImportEventsBody {
    process_id: Uuid,
    events: Vec<EventImport>,
}

#[derive(Deserialize)]
struct EventImport {
    case_id: String,
    activity: String,
    timestamp: DateTime<Utc>,
    resource: Option<String>,
    event_type: Option<String>,
    metadata: Option<serde_json::Value>,
}

async fn import_events(
    State(_state): State<AppState>,
    Json(body): Json<ImportEventsBody>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let service = ProcessMiningService::new();
    let events: Vec<ProcessEventImport> = body.events.into_iter().map(|e| ProcessEventImport {
        case_id: e.case_id,
        activity: e.activity,
        timestamp: e.timestamp,
        resource: e.resource,
        event_type: e.event_type,
        metadata: e.metadata,
    }).collect();
    let instances = service.import_events(ImportEventsRequest {
        process_id: body.process_id,
        events,
    }).await?;
    Ok(Json(instances.into_iter().map(|i| serde_json::to_value(i).unwrap()).collect()))
}

#[derive(Deserialize)]
struct CreateSimulationBody {
    process_id: Uuid,
    name: String,
    scenario: SimulationScenario,
}

async fn create_simulation(
    State(_state): State<AppState>,
    Json(body): Json<CreateSimulationBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = ProcessMiningService::new();
    let simulation = service.create_simulation(body.process_id, body.name, body.scenario).await?;
    Ok(Json(serde_json::to_value(simulation)?))
}

async fn get_simulation(
    Path(id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({})))
}

async fn run_simulation(
    Path(simulation_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = ProcessMiningService::new();
    let simulation = service.run_simulation(simulation_id).await?;
    Ok(Json(serde_json::to_value(simulation)?))
}
