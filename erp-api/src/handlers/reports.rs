use axum::{extract::{Query, State}, Json, routing::{get, post}};
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{Pagination, BaseEntity, Status};
use erp_reports::{
    ReportDefinition, ReportCategory, ReportFormat,
    ReportSchedule, ScheduleFrequency, DeliveryMethod,
    ReportExecution, ReportDashboard,
    ReportDefinitionService, ReportScheduleService, ReportExecutionService,
};

#[derive(Serialize)]
pub struct ReportDefinitionResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub category: String,
    pub status: String,
}

impl From<ReportDefinition> for ReportDefinitionResponse {
    fn from(r: ReportDefinition) -> Self {
        Self {
            id: r.base.id,
            name: r.name,
            code: r.code,
            category: format!("{:?}", r.category),
            status: format!("{:?}", r.status),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateReportRequest {
    pub name: String,
    pub code: String,
    pub category: String,
    pub description: Option<String>,
    pub data_source: String,
    pub query_template: String,
}

pub async fn list_reports(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> ApiResult<Json<erp_core::Paginated<ReportDefinitionResponse>>> {
    let svc = ReportDefinitionService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(
        res.items.into_iter().map(ReportDefinitionResponse::from).collect(),
        res.total,
        Pagination { page: res.page, per_page: res.per_page },
    )))
}

pub async fn create_report(
    State(state): State<AppState>,
    Json(req): Json<CreateReportRequest>,
) -> ApiResult<Json<ReportDefinitionResponse>> {
    let svc = ReportDefinitionService::new();
    let report = ReportDefinition {
        base: BaseEntity::new(),
        name: req.name,
        code: req.code,
        category: match req.category.as_str() {
            "Sales" => ReportCategory::Sales,
            "Inventory" => ReportCategory::Inventory,
            "Purchasing" => ReportCategory::Purchasing,
            "Manufacturing" => ReportCategory::Manufacturing,
            "HR" => ReportCategory::HR,
            "CRM" => ReportCategory::CRM,
            "Operations" => ReportCategory::Operations,
            "Compliance" => ReportCategory::Compliance,
            "Executive" => ReportCategory::Executive,
            "Custom" => ReportCategory::Custom,
            _ => ReportCategory::Financial,
        },
        description: req.description,
        data_source: req.data_source,
        query_template: req.query_template,
        parameters: vec![],
        columns: vec![],
        default_format: ReportFormat::PDF,
        allowed_formats: vec![ReportFormat::PDF, ReportFormat::Excel, ReportFormat::CSV],
        is_scheduled: false,
        status: Status::Active,
        created_by: None,
        version: 1,
    };
    Ok(Json(ReportDefinitionResponse::from(svc.create(&state.pool, report).await?)))
}

#[derive(Serialize)]
pub struct ReportScheduleResponse {
    pub id: Uuid,
    pub report_definition_id: Uuid,
    pub name: String,
    pub frequency: String,
    pub is_active: bool,
    pub next_run_at: Option<String>,
}

impl From<ReportSchedule> for ReportScheduleResponse {
    fn from(s: ReportSchedule) -> Self {
        Self {
            id: s.base.id,
            report_definition_id: s.report_definition_id,
            name: s.name,
            frequency: format!("{:?}", s.frequency),
            is_active: s.is_active,
            next_run_at: s.next_run_at.map(|d| d.to_rfc3339()),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateScheduleRequest {
    pub report_definition_id: Uuid,
    pub name: String,
    pub frequency: String,
    pub output_format: String,
    pub recipients: Vec<String>,
    pub email_subject: Option<String>,
}

pub async fn create_schedule(
    State(state): State<AppState>,
    Json(req): Json<CreateScheduleRequest>,
) -> ApiResult<Json<ReportScheduleResponse>> {
    let svc = ReportScheduleService::new();
    let schedule = ReportSchedule {
        base: BaseEntity::new(),
        report_definition_id: req.report_definition_id,
        name: req.name,
        frequency: match req.frequency.as_str() {
            "Hourly" => ScheduleFrequency::Hourly,
            "Daily" => ScheduleFrequency::Daily,
            "Weekly" => ScheduleFrequency::Weekly,
            "Monthly" => ScheduleFrequency::Monthly,
            "Quarterly" => ScheduleFrequency::Quarterly,
            "Yearly" => ScheduleFrequency::Yearly,
            "Custom" => ScheduleFrequency::Custom,
            _ => ScheduleFrequency::Once,
        },
        cron_expression: None,
        start_date: Utc::now(),
        end_date: None,
        next_run_at: None,
        last_run_at: None,
        parameters: "{}".to_string(),
        output_format: match req.output_format.as_str() {
            "Excel" => ReportFormat::Excel,
            "CSV" => ReportFormat::CSV,
            "HTML" => ReportFormat::HTML,
            "JSON" => ReportFormat::JSON,
            "Word" => ReportFormat::Word,
            _ => ReportFormat::PDF,
        },
        delivery_methods: vec![DeliveryMethod::Email],
        recipients: req.recipients,
        email_subject: req.email_subject,
        email_body: None,
        include_attachments: true,
        ftp_host: None,
        ftp_path: None,
        webhook_url: None,
        is_active: true,
        status: Status::Active,
        created_by: None,
    };
    Ok(Json(ReportScheduleResponse::from(svc.create(&state.pool, schedule).await?)))
}

#[derive(Serialize)]
pub struct ReportExecutionResponse {
    pub id: Uuid,
    pub report_definition_id: Uuid,
    pub format: String,
    pub status: String,
    pub row_count: i64,
    pub file_path: Option<String>,
    pub error_message: Option<String>,
}

impl From<ReportExecution> for ReportExecutionResponse {
    fn from(e: ReportExecution) -> Self {
        Self {
            id: e.base.id,
            report_definition_id: e.report_definition_id,
            format: format!("{:?}", e.format),
            status: format!("{:?}", e.status),
            row_count: e.row_count,
            file_path: e.file_path,
            error_message: e.error_message,
        }
    }
}

#[derive(Deserialize)]
pub struct RunReportRequest {
    pub report_definition_id: Uuid,
    pub format: String,
    pub parameters: Option<String>,
}

pub async fn run_report(
    State(state): State<AppState>,
    Json(req): Json<RunReportRequest>,
) -> ApiResult<Json<ReportExecutionResponse>> {
    let svc = ReportExecutionService::new();
    let format = match req.format.as_str() {
        "Excel" => ReportFormat::Excel,
        "CSV" => ReportFormat::CSV,
        "HTML" => ReportFormat::HTML,
        "JSON" => ReportFormat::JSON,
        "Word" => ReportFormat::Word,
        _ => ReportFormat::PDF,
    };
    
    let execution = svc.start(
        &state.pool,
        req.report_definition_id,
        None,
        req.parameters.as_deref().unwrap_or("{}"),
        format,
    ).await?;
    
    let def_svc = ReportDefinitionService::new();
    let report = def_svc.get(&state.pool, req.report_definition_id).await?;
    
    let (_csv, row_count) = erp_reports::ReportGeneratorService::generate_csv(
        &state.pool,
        &report.query_template,
        &report.columns,
    ).await?;
    
    let file_path = format!("/tmp/report_{}.csv", execution.base.id);
    svc.complete(&state.pool, execution.base.id, &file_path, row_count).await?;
    
    Ok(Json(ReportExecutionResponse::from(svc.get(&state.pool, execution.base.id).await?)))
}

#[derive(Serialize)]
pub struct DashboardResponse {
    pub id: Uuid,
    pub name: String,
    pub is_public: bool,
    pub widget_count: usize,
}

impl From<ReportDashboard> for DashboardResponse {
    fn from(d: ReportDashboard) -> Self {
        Self {
            id: d.base.id,
            name: d.name,
            is_public: d.is_public,
            widget_count: d.widgets.len(),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateDashboardRequest {
    pub name: String,
    pub description: Option<String>,
    pub is_public: Option<bool>,
}

pub async fn create_dashboard(
    State(state): State<AppState>,
    Json(req): Json<CreateDashboardRequest>,
) -> ApiResult<Json<DashboardResponse>> {
    let svc = erp_reports::DashboardService::new();
    let dashboard = ReportDashboard {
        base: BaseEntity::new(),
        name: req.name,
        description: req.description,
        layout: "grid".to_string(),
        widgets: vec![],
        is_default: false,
        is_public: req.is_public.unwrap_or(true),
        refresh_interval_seconds: Some(300),
        status: Status::Active,
        owner_id: None,
    };
    Ok(Json(DashboardResponse::from(svc.create(&state.pool, dashboard).await?)))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/definitions", get(list_reports).post(create_report))
        .route("/schedules", post(create_schedule))
        .route("/run", post(run_report))
        .route("/dashboards", post(create_dashboard))
}
