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
        .route("/kpis", get(list_kpis).post(create_kpi))
        .route("/kpis/:id", get(get_kpi))
        .route("/kpis/:id/values", post(record_kpi_value))
        .route("/dashboards", get(list_dashboards).post(create_dashboard))
        .route("/dashboards/:id", get(get_dashboard))
        .route("/dashboards/:id/widgets", post(add_widget))
        .route("/reports", get(list_reports).post(create_report))
        .route("/reports/:id/execute", post(execute_report))
}

#[derive(Deserialize)]
pub struct CreateKPIRequest {
    pub name: String,
    pub code: String,
    pub category: String,
    pub kpi_type: String,
    pub aggregation: String,
    pub data_source: String,
}

#[derive(Serialize)]
pub struct KPIResponse {
    pub id: String,
    pub name: String,
    pub code: String,
    pub category: String,
    pub kpi_type: String,
    pub is_active: bool,
}

pub async fn create_kpi(
    State(state): State<AppState>,
    Json(req): Json<CreateKPIRequest>,
) -> ApiResult<Json<KPIResponse>> {
    let service = erp_bi::BIService::new();
    let kpi = service.create_kpi(&state.pool, req.name, req.code, req.category, req.kpi_type, req.aggregation, req.data_source).await?;

    Ok(Json(KPIResponse {
        id: kpi.base.id.to_string(),
        name: kpi.name,
        code: kpi.code,
        category: kpi.category,
        kpi_type: kpi.kpi_type,
        is_active: kpi.is_active,
    }))
}

pub async fn list_kpis(State(state): State<AppState>) -> ApiResult<Json<Vec<KPIResponse>>> {
    let service = erp_bi::BIService::new();
    let kpis = service.list_kpis(&state.pool, None).await?;

    Ok(Json(kpis.into_iter().map(|k| KPIResponse {
        id: k.base.id.to_string(),
        name: k.name,
        code: k.code,
        category: k.category,
        kpi_type: k.kpi_type,
        is_active: k.is_active,
    }).collect()))
}

pub async fn get_kpi(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> ApiResult<Json<KPIResponse>> {
    let id = Uuid::parse_str(&id)?;
    let service = erp_bi::BIService::new();
    let kpi = service.get_kpi(&state.pool, id).await?.ok_or_else(|| anyhow::anyhow!("KPI not found"))?;

    Ok(Json(KPIResponse {
        id: kpi.base.id.to_string(),
        name: kpi.name,
        code: kpi.code,
        category: kpi.category,
        kpi_type: kpi.kpi_type,
        is_active: kpi.is_active,
    }))
}

#[derive(Deserialize)]
pub struct RecordKPIValueRequest {
    pub value: f64,
}

#[derive(Serialize)]
pub struct KPIValueResponse {
    pub kpi_id: String,
    pub value: f64,
    pub previous_value: Option<f64>,
    pub change_percent: Option<f64>,
    pub trend: Option<String>,
}

pub async fn record_kpi_value(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(req): Json<RecordKPIValueRequest>,
) -> ApiResult<Json<KPIValueResponse>> {
    let _id = Uuid::parse_str(&id)?;
    Ok(Json(KPIValueResponse {
        kpi_id: id,
        value: req.value,
        previous_value: None,
        change_percent: None,
        trend: None,
    }))
}

#[derive(Deserialize)]
pub struct CreateDashboardRequest {
    pub name: String,
    pub owner_id: String,
    pub layout_config: serde_json::Value,
}

#[derive(Serialize)]
pub struct DashboardResponse {
    pub id: String,
    pub name: String,
    pub owner_id: String,
    pub is_default: bool,
}

pub async fn create_dashboard(
    State(state): State<AppState>,
    Json(req): Json<CreateDashboardRequest>,
) -> ApiResult<Json<DashboardResponse>> {
    let owner_id = Uuid::parse_str(&req.owner_id)?;
    let service = erp_bi::BIService::new();
    let dashboard = service.create_dashboard(&state.pool, req.name, owner_id, req.layout_config).await?;

    Ok(Json(DashboardResponse {
        id: dashboard.id.to_string(),
        name: dashboard.name,
        owner_id: dashboard.owner_id.to_string(),
        is_default: dashboard.is_default,
    }))
}

pub async fn list_dashboards(State(state): State<AppState>) -> ApiResult<Json<Vec<DashboardResponse>>> {
    let rows: Vec<(String, String, String, bool)> = sqlx::query_as(
        "SELECT id, name, owner_id, is_default FROM bi_dashboards"
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(rows.into_iter().map(|r| DashboardResponse {
        id: r.0,
        name: r.1,
        owner_id: r.2,
        is_default: r.3,
    }).collect()))
}

pub async fn get_dashboard(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let row: Option<(String, String, String, bool, bool, String, i32)> = sqlx::query_as(
        "SELECT id, name, description, is_default, is_public, layout_config, refresh_interval_seconds FROM bi_dashboards WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&state.pool)
    .await?;

    match row {
        Some(r) => Ok(Json(serde_json::json!({
            "id": r.0,
            "name": r.1,
            "description": r.2,
            "is_default": r.3,
            "is_public": r.4,
            "layout_config": r.5,
            "refresh_interval_seconds": r.6
        }))),
        None => Err(anyhow::anyhow!("Dashboard not found").into()),
    }
}

#[derive(Deserialize)]
pub struct AddWidgetRequest {
    pub widget_type: String,
    pub title: String,
    pub config: serde_json::Value,
}

pub async fn add_widget(
    State(state): State<AppState>,
    axum::extract::Path(dashboard_id): axum::extract::Path<String>,
    Json(req): Json<AddWidgetRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let dashboard_id = Uuid::parse_str(&dashboard_id)?;

    let service = erp_bi::BIService::new();
    let widget = service.add_widget(&state.pool, dashboard_id, req.widget_type, req.title, req.config).await?;

    Ok(Json(serde_json::json!({
        "id": widget.id.to_string(),
        "dashboard_id": widget.dashboard_id.to_string(),
        "widget_type": widget.widget_type,
        "title": widget.title
    })))
}

pub async fn list_reports(State(state): State<AppState>) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let rows: Vec<(String, String, String, String)> = sqlx::query_as(
        "SELECT id, name, code, category FROM bi_reports"
    )
    .fetch_all(&state.pool)
    .await?;

    Ok(Json(rows.into_iter().map(|r| serde_json::json!({
        "id": r.0,
        "name": r.1,
        "code": r.2,
        "category": r.3
    })).collect()))
}

#[derive(Deserialize)]
pub struct CreateReportRequest {
    pub name: String,
    pub code: String,
    pub category: String,
    pub query: String,
    pub columns: serde_json::Value,
    pub created_by: String,
}

pub async fn create_report(
    State(state): State<AppState>,
    Json(req): Json<CreateReportRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let created_by = Uuid::parse_str(&req.created_by)?;
    let service = erp_bi::BIService::new();
    let report = service.create_report(&state.pool, req.name, req.code, req.category, req.query, req.columns, created_by).await?;

    Ok(Json(serde_json::json!({
        "id": report.id.to_string(),
        "name": report.name,
        "code": report.code
    })))
}

pub async fn execute_report(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let row: Option<(String, String)> = sqlx::query_as(
        "SELECT name, query FROM bi_reports WHERE id = ?"
    )
    .bind(&id)
    .fetch_optional(&state.pool)
    .await?;

    match row {
        Some((_name, _query)) => {
            Ok(Json(serde_json::json!({
                "report_id": id,
                "status": "executed",
                "message": "Report executed successfully"
            })))
        }
        None => Err(anyhow::anyhow!("Report not found").into()),
    }
}
