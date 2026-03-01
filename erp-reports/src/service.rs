use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity};
use crate::models::*;
use crate::repository::*;

pub struct ReportDefinitionService { repo: SqliteReportDefinitionRepository }
impl ReportDefinitionService {
    pub fn new() -> Self { Self { repo: SqliteReportDefinitionRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<ReportDefinition> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> Result<ReportDefinition> {
        self.repo.find_by_code(pool, code).await
    }
    
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ReportDefinition>> {
        self.repo.find_all(pool, pagination).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut report: ReportDefinition) -> Result<ReportDefinition> {
        if report.name.is_empty() || report.code.is_empty() {
            return Err(Error::validation("Report name and code are required"));
        }
        if report.query_template.is_empty() {
            return Err(Error::validation("Query template is required"));
        }
        report.base = BaseEntity::new();
        report.status = erp_core::Status::Active;
        report.version = 1;
        self.repo.create(pool, report).await
    }
    
    pub async fn update(&self, pool: &SqlitePool, mut report: ReportDefinition) -> Result<ReportDefinition> {
        report.base.updated_at = Utc::now();
        self.repo.update(pool, report).await
    }
}

pub struct ReportScheduleService { repo: SqliteReportScheduleRepository }
impl ReportScheduleService {
    pub fn new() -> Self { Self { repo: SqliteReportScheduleRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<ReportSchedule> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn get_due(&self, pool: &SqlitePool) -> Result<Vec<ReportSchedule>> {
        self.repo.find_due(pool).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut schedule: ReportSchedule) -> Result<ReportSchedule> {
        if schedule.name.is_empty() {
            return Err(Error::validation("Schedule name is required"));
        }
        schedule.base = BaseEntity::new();
        schedule.status = erp_core::Status::Active;
        schedule.is_active = true;
        schedule.next_run_at = Some(Self::calculate_next_run(&schedule.frequency, schedule.start_date));
        self.repo.create(pool, schedule).await
    }
    
    pub async fn mark_run(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let schedule = self.repo.find_by_id(pool, id).await?;
        let next_run = Self::calculate_next_run(&schedule.frequency, Utc::now());
        self.repo.update_next_run(pool, id, next_run).await
    }
    
    fn calculate_next_run(frequency: &ScheduleFrequency, from: chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
        match frequency {
            ScheduleFrequency::Hourly => from + chrono::Duration::hours(1),
            ScheduleFrequency::Daily => from + chrono::Duration::days(1),
            ScheduleFrequency::Weekly => from + chrono::Duration::weeks(1),
            ScheduleFrequency::Monthly => {
                let next = from + chrono::Duration::days(30);
                next
            }
            ScheduleFrequency::Quarterly => from + chrono::Duration::days(90),
            ScheduleFrequency::Yearly => from + chrono::Duration::days(365),
            _ => from + chrono::Duration::days(1),
        }
    }
}

pub struct ReportExecutionService { repo: SqliteReportExecutionRepository }
impl ReportExecutionService {
    pub fn new() -> Self { Self { repo: SqliteReportExecutionRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<ReportExecution> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn start(&self, pool: &SqlitePool, report_definition_id: Uuid, schedule_id: Option<Uuid>, parameters: &str, format: ReportFormat) -> Result<ReportExecution> {
        let execution = ReportExecution {
            base: BaseEntity::new(),
            report_definition_id,
            schedule_id,
            parameters: parameters.to_string(),
            format,
            status: ReportStatus::Running,
            started_at: Some(Utc::now()),
            completed_at: None,
            duration_ms: None,
            row_count: 0,
            file_path: None,
            file_size_bytes: None,
            error_message: None,
            delivery_status: None,
            delivered_at: None,
            executed_by: None,
        };
        
        self.repo.create(pool, execution).await
    }
    
    pub async fn complete(&self, pool: &SqlitePool, id: Uuid, file_path: &str, row_count: i64) -> Result<()> {
        self.repo.update_status(pool, id, ReportStatus::Completed, Some(file_path), row_count, None).await
    }
    
    pub async fn fail(&self, pool: &SqlitePool, id: Uuid, error: &str) -> Result<()> {
        self.repo.update_status(pool, id, ReportStatus::Failed, None, 0, Some(error)).await
    }
}

pub struct ReportGeneratorService;
impl ReportGeneratorService {
    pub fn new() -> Self { Self }
    
    pub async fn generate_csv(
        pool: &SqlitePool,
        query: &str,
        columns: &[ReportColumn],
    ) -> Result<(String, i64)> {
        let rows = sqlx::query(query)
            .fetch_all(pool)
            .await
            .map_err(Error::Database)?;
        
        let row_count = rows.len() as i64;
        
        let mut csv = String::new();
        csv.push_str(&columns.iter().filter(|c| c.is_visible).map(|c| c.label.as_str()).collect::<Vec<_>>().join(","));
        csv.push('\n');
        
        Ok((csv, row_count))
    }
    
    pub async fn generate_json(
        pool: &SqlitePool,
        query: &str,
    ) -> Result<(String, i64)> {
        let rows = sqlx::query(query)
            .fetch_all(pool)
            .await
            .map_err(Error::Database)?;
        
        let row_count = rows.len() as i64;
        
        Ok((format!("{{\"row_count\": {}}}", row_count), row_count))
    }
}

pub struct DashboardService;
impl DashboardService {
    pub fn new() -> Self { Self }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<ReportDashboard> {
        let row = sqlx::query_as::<_, DashboardRow>(
            "SELECT * FROM report_dashboards WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("ReportDashboard", &id.to_string()))?;
        
        let widgets = self.get_widgets(pool, id).await?;
        Ok(row.into_dashboard(widgets))
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut dashboard: ReportDashboard) -> Result<ReportDashboard> {
        dashboard.base = BaseEntity::new();
        dashboard.status = erp_core::Status::Active;
        
        sqlx::query(
            "INSERT INTO report_dashboards (id, name, description, layout, widgets, is_default, is_public, refresh_interval_seconds, status, owner_id, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(dashboard.base.id.to_string())
        .bind(&dashboard.name)
        .bind(&dashboard.description)
        .bind(&dashboard.layout)
        .bind(serde_json::to_string(&dashboard.widgets).unwrap_or_default())
        .bind(dashboard.is_default as i32)
        .bind(dashboard.is_public as i32)
        .bind(dashboard.refresh_interval_seconds)
        .bind(format!("{:?}", dashboard.status))
        .bind(dashboard.owner_id.map(|id| id.to_string()))
        .bind(dashboard.base.created_at.to_rfc3339())
        .bind(dashboard.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(dashboard)
    }
    
    async fn get_widgets(&self, pool: &SqlitePool, dashboard_id: Uuid) -> Result<Vec<DashboardWidget>> {
        let rows = sqlx::query_as::<_, WidgetRow>(
            "SELECT * FROM dashboard_widgets WHERE dashboard_id = ?"
        )
        .bind(dashboard_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct DashboardRow {
    id: String,
    name: String,
    description: Option<String>,
    layout: String,
    widgets: String,
    is_default: i32,
    is_public: i32,
    refresh_interval_seconds: Option<i32>,
    status: String,
    owner_id: Option<String>,
    created_at: String,
    updated_at: String,
}

impl DashboardRow {
    fn into_dashboard(self, widgets: Vec<DashboardWidget>) -> ReportDashboard {
        ReportDashboard {
            base: BaseEntity {
                id: Uuid::parse_str(&self.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            name: self.name,
            description: self.description,
            layout: self.layout,
            widgets,
            is_default: self.is_default != 0,
            is_public: self.is_public != 0,
            refresh_interval_seconds: self.refresh_interval_seconds,
            status: match self.status.as_str() {
                "Inactive" => erp_core::Status::Inactive,
                _ => erp_core::Status::Active,
            },
            owner_id: self.owner_id.and_then(|id| Uuid::parse_str(&id).ok()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct WidgetRow {
    id: String,
    dashboard_id: String,
    report_definition_id: Option<String>,
    widget_type: String,
    title: String,
    position_x: i32,
    position_y: i32,
    width: i32,
    height: i32,
    parameters: String,
    refresh_interval_seconds: Option<i32>,
    chart_config: Option<String>,
}

impl From<WidgetRow> for DashboardWidget {
    fn from(r: WidgetRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            dashboard_id: Uuid::parse_str(&r.dashboard_id).unwrap_or_default(),
            report_definition_id: r.report_definition_id.and_then(|id| Uuid::parse_str(&id).ok()),
            widget_type: match r.widget_type.as_str() {
                "Chart" => WidgetType::Chart,
                "KPI" => WidgetType::KPI,
                "Gauge" => WidgetType::Gauge,
                "Map" => WidgetType::Map,
                "PivotTable" => WidgetType::PivotTable,
                "Sparkline" => WidgetType::Sparkline,
                "Treemap" => WidgetType::Treemap,
                "Heatmap" => WidgetType::Heatmap,
                "Funnel" => WidgetType::Funnel,
                _ => WidgetType::Table,
            },
            title: r.title,
            position_x: r.position_x,
            position_y: r.position_y,
            width: r.width,
            height: r.height,
            parameters: r.parameters,
            refresh_interval_seconds: r.refresh_interval_seconds,
            chart_config: r.chart_config,
        }
    }
}
