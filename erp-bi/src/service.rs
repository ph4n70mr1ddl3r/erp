use sqlx::{Row, SqlitePool};
use uuid::Uuid;
use chrono::Utc;
use crate::models::*;
use erp_core::{BaseEntity, Result};

pub struct BIService;

impl BIService {
    pub fn new() -> Self { Self }

    pub async fn create_kpi(&self, pool: &SqlitePool, name: String, code: String, category: String, 
        kpi_type: String, aggregation: String, data_source: String) -> Result<KPI> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query(r#"
            INSERT INTO bi_kpis (id, name, code, description, category, kpi_type, aggregation, 
                data_source, query, target_value, warning_threshold, critical_threshold, unit, 
                refresh_interval_seconds, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(id.to_string())
        .bind(&name)
        .bind(&code)
        .bind(Option::<String>::None)
        .bind(&category)
        .bind(&kpi_type)
        .bind(&aggregation)
        .bind(&data_source)
        .bind(Option::<String>::None)
        .bind(Option::<f64>::None)
        .bind(Option::<f64>::None)
        .bind(Option::<f64>::None)
        .bind(Option::<String>::None)
        .bind(300i32)
        .bind(true)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(KPI {
            base: BaseEntity::new(),
            name,
            code,
            description: None,
            category,
            kpi_type,
            aggregation,
            data_source,
            query: None,
            target_value: None,
            warning_threshold: None,
            critical_threshold: None,
            unit: None,
            refresh_interval_seconds: 300,
            is_active: true,
        })
    }

    pub async fn get_kpi(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<KPI>> {
        let row = sqlx::query(
            "SELECT id, name, code, description, category, kpi_type, aggregation, data_source, query, target_value, warning_threshold, critical_threshold, unit, refresh_interval_seconds, is_active, created_at, updated_at FROM bi_kpis WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| KPI {
            base: BaseEntity {
                id: r.get::<String, _>("id").parse().unwrap_or_default(),
                created_at: r.get::<String, _>("created_at").parse().unwrap_or_default(),
                updated_at: r.get::<String, _>("updated_at").parse().unwrap_or_default(),
                created_by: None,
                updated_by: None,
            },
            name: r.get("name"),
            code: r.get("code"),
            description: r.get("description"),
            category: r.get("category"),
            kpi_type: r.get("kpi_type"),
            aggregation: r.get("aggregation"),
            data_source: r.get("data_source"),
            query: r.get("query"),
            target_value: r.get("target_value"),
            warning_threshold: r.get("warning_threshold"),
            critical_threshold: r.get("critical_threshold"),
            unit: r.get("unit"),
            refresh_interval_seconds: r.get::<i32, _>("refresh_interval_seconds"),
            is_active: r.get::<bool, _>("is_active"),
        }))
    }

    pub async fn list_kpis(&self, pool: &SqlitePool, category: Option<&str>) -> Result<Vec<KPI>> {
        let rows = if let Some(cat) = category {
            sqlx::query(
                "SELECT id, name, code, description, category, kpi_type, aggregation, data_source, query, target_value, warning_threshold, critical_threshold, unit, refresh_interval_seconds, is_active, created_at, updated_at FROM bi_kpis WHERE category = ?"
            )
            .bind(cat)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query(
                "SELECT id, name, code, description, category, kpi_type, aggregation, data_source, query, target_value, warning_threshold, critical_threshold, unit, refresh_interval_seconds, is_active, created_at, updated_at FROM bi_kpis"
            )
            .fetch_all(pool)
            .await?
        };
        
        Ok(rows.into_iter().map(|r| KPI {
            base: BaseEntity {
                id: r.get::<String, _>("id").parse().unwrap_or_default(),
                created_at: r.get::<String, _>("created_at").parse().unwrap_or_default(),
                updated_at: r.get::<String, _>("updated_at").parse().unwrap_or_default(),
                created_by: None,
                updated_by: None,
            },
            name: r.get("name"),
            code: r.get("code"),
            description: r.get("description"),
            category: r.get("category"),
            kpi_type: r.get("kpi_type"),
            aggregation: r.get("aggregation"),
            data_source: r.get("data_source"),
            query: r.get("query"),
            target_value: r.get("target_value"),
            warning_threshold: r.get("warning_threshold"),
            critical_threshold: r.get("critical_threshold"),
            unit: r.get("unit"),
            refresh_interval_seconds: r.get::<i32, _>("refresh_interval_seconds"),
            is_active: r.get::<bool, _>("is_active"),
        }).collect())
    }

    pub async fn create_dashboard(&self, pool: &SqlitePool, name: String, owner_id: Uuid, 
        layout_config: serde_json::Value) -> Result<Dashboard> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query(r#"
            INSERT INTO bi_dashboards (id, name, description, owner_id, is_default, is_public, 
                layout_config, refresh_interval_seconds, filters, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(id.to_string())
        .bind(&name)
        .bind(Option::<String>::None)
        .bind(owner_id.to_string())
        .bind(false)
        .bind(false)
        .bind(&layout_config)
        .bind(300i32)
        .bind(Option::<serde_json::Value>::None)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(Dashboard {
            id,
            name,
            description: None,
            owner_id,
            is_default: false,
            is_public: false,
            layout_config,
            refresh_interval_seconds: 300,
            filters: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn add_widget(&self, pool: &SqlitePool, dashboard_id: Uuid, widget_type: String,
        title: String, config: serde_json::Value) -> Result<DashboardWidget> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query(r#"
            INSERT INTO bi_dashboard_widgets (id, dashboard_id, kpi_id, widget_type, title, 
                position_x, position_y, width, height, config, data_source, custom_query, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(id.to_string())
        .bind(dashboard_id.to_string())
        .bind(Option::<String>::None)
        .bind(&widget_type)
        .bind(&title)
        .bind(0i32)
        .bind(0i32)
        .bind(4i32)
        .bind(3i32)
        .bind(&config)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(DashboardWidget {
            id,
            dashboard_id,
            kpi_id: None,
            widget_type,
            title,
            position_x: 0,
            position_y: 0,
            width: 4,
            height: 3,
            config,
            data_source: None,
            custom_query: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn create_report(&self, pool: &SqlitePool, name: String, code: String, 
        category: String, query: String, columns: serde_json::Value, created_by: Uuid) -> Result<Report> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query(r#"
            INSERT INTO bi_reports (id, name, code, description, category, query, parameters, 
                columns, chart_config, is_scheduled, schedule_cron, last_run_at, created_by, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(id.to_string())
        .bind(&name)
        .bind(&code)
        .bind(Option::<String>::None)
        .bind(&category)
        .bind(&query)
        .bind(Option::<serde_json::Value>::None)
        .bind(&columns)
        .bind(Option::<serde_json::Value>::None)
        .bind(false)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(created_by.to_string())
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(Report {
            id,
            name,
            code,
            description: None,
            category,
            query,
            parameters: None,
            columns,
            chart_config: None,
            is_scheduled: false,
            schedule_cron: None,
            last_run_at: None,
            created_by,
            created_at: now,
            updated_at: now,
        })
    }
}
