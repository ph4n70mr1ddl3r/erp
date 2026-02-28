use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::*;
use erp_core::{Result, BaseEntity};

#[async_trait]
pub trait KPIRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, kpi: &KPI) -> Result<KPI>;
    async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<KPI>>;
    async fn list(&self, pool: &SqlitePool, category: Option<&str>) -> Result<Vec<KPI>>;
    async fn update(&self, pool: &SqlitePool, kpi: &KPI) -> Result<KPI>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait DashboardRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, dashboard: &Dashboard) -> Result<Dashboard>;
    async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<Dashboard>>;
    async fn list_by_user(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Vec<Dashboard>>;
    async fn update(&self, pool: &SqlitePool, dashboard: &Dashboard) -> Result<Dashboard>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait WidgetRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, widget: &DashboardWidget) -> Result<DashboardWidget>;
    async fn list_by_dashboard(&self, pool: &SqlitePool, dashboard_id: Uuid) -> Result<Vec<DashboardWidget>>;
    async fn update(&self, pool: &SqlitePool, widget: &DashboardWidget) -> Result<DashboardWidget>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteKPIRepository;

#[async_trait]
impl KPIRepository for SqliteKPIRepository {
    async fn create(&self, pool: &SqlitePool, kpi: &KPI) -> Result<KPI> {
        sqlx::query(r#"
            INSERT INTO bi_kpis (id, name, code, description, category, kpi_type, aggregation, 
                data_source, query, target_value, warning_threshold, critical_threshold, unit, 
                refresh_interval_seconds, is_active, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(kpi.base.id.to_string())
        .bind(&kpi.name)
        .bind(&kpi.code)
        .bind(&kpi.description)
        .bind(&kpi.category)
        .bind(&kpi.kpi_type)
        .bind(&kpi.aggregation)
        .bind(&kpi.data_source)
        .bind(&kpi.query)
        .bind(kpi.target_value)
        .bind(kpi.warning_threshold)
        .bind(kpi.critical_threshold)
        .bind(&kpi.unit)
        .bind(kpi.refresh_interval_seconds)
        .bind(kpi.is_active)
        .bind(kpi.base.created_at.to_rfc3339())
        .bind(kpi.base.updated_at.to_rfc3339())
        .execute(pool)
        .await?;
        Ok(kpi.clone())
    }

    async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<KPI>> {
        let row: Option<(String, String, String, Option<String>, String, String, String, String, Option<String>, Option<f64>, Option<f64>, Option<f64>, Option<String>, i32, bool, String, String)> = 
            sqlx::query_as("SELECT id, name, code, description, category, kpi_type, aggregation, data_source, query, target_value, warning_threshold, critical_threshold, unit, refresh_interval_seconds, is_active, created_at, updated_at FROM bi_kpis WHERE id = ?")
                .bind(id.to_string())
                .fetch_optional(pool)
                .await?;
        
        Ok(row.map(|r| KPI {
            base: BaseEntity { id, created_at: r.15.parse().unwrap_or_default(), updated_at: r.16.parse().unwrap_or_default() },
            name: r.1,
            code: r.2,
            description: r.3,
            category: r.4,
            kpi_type: r.5.parse().ok().unwrap_or(KPIType::Counter),
            aggregation: r.6.parse().ok().unwrap_or(AggregationType::Sum),
            data_source: r.7,
            query: r.8,
            target_value: r.9,
            warning_threshold: r.10,
            critical_threshold: r.11,
            unit: r.12,
            refresh_interval_seconds: r.13,
            is_active: r.14,
        }))
    }

    async fn list(&self, pool: &SqlitePool, category: Option<&str>) -> Result<Vec<KPI>> {
        let rows: Vec<(String, String, String, Option<String>, String, String, String, String, Option<String>, Option<f64>, Option<f64>, Option<f64>, Option<String>, i32, bool, String, String)> = 
            if let Some(cat) = category {
                sqlx::query_as("SELECT id, name, code, description, category, kpi_type, aggregation, data_source, query, target_value, warning_threshold, critical_threshold, unit, refresh_interval_seconds, is_active, created_at, updated_at FROM bi_kpis WHERE category = ?")
                    .bind(cat)
                    .fetch_all(pool)
                    .await?
            } else {
                sqlx::query_as("SELECT id, name, code, description, category, kpi_type, aggregation, data_source, query, target_value, warning_threshold, critical_threshold, unit, refresh_interval_seconds, is_active, created_at, updated_at FROM bi_kpis")
                    .fetch_all(pool)
                    .await?
            };
        
        Ok(rows.into_iter().map(|r| KPI {
            base: BaseEntity { id: r.0.parse().unwrap_or_default(), created_at: r.15.parse().unwrap_or_default(), updated_at: r.16.parse().unwrap_or_default() },
            name: r.1,
            code: r.2,
            description: r.3,
            category: r.4,
            kpi_type: r.5.parse().ok().unwrap_or(KPIType::Counter),
            aggregation: r.6.parse().ok().unwrap_or(AggregationType::Sum),
            data_source: r.7,
            query: r.8,
            target_value: r.9,
            warning_threshold: r.10,
            critical_threshold: r.11,
            unit: r.12,
            refresh_interval_seconds: r.13,
            is_active: r.14,
        }).collect())
    }

    async fn update(&self, pool: &SqlitePool, kpi: &KPI) -> Result<KPI> {
        sqlx::query(r#"
            UPDATE bi_kpis SET name = ?, code = ?, description = ?, category = ?, kpi_type = ?,
                aggregation = ?, data_source = ?, query = ?, target_value = ?, warning_threshold = ?,
                critical_threshold = ?, unit = ?, refresh_interval_seconds = ?, is_active = ?, updated_at = ?
            WHERE id = ?
        "#)
        .bind(&kpi.name)
        .bind(&kpi.code)
        .bind(&kpi.description)
        .bind(&kpi.category)
        .bind(&kpi.kpi_type)
        .bind(&kpi.aggregation)
        .bind(&kpi.data_source)
        .bind(&kpi.query)
        .bind(kpi.target_value)
        .bind(kpi.warning_threshold)
        .bind(kpi.critical_threshold)
        .bind(&kpi.unit)
        .bind(kpi.refresh_interval_seconds)
        .bind(kpi.is_active)
        .bind(kpi.base.updated_at.to_rfc3339())
        .bind(kpi.base.id.to_string())
        .execute(pool)
        .await?;
        Ok(kpi.clone())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM bi_kpis WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }
}
