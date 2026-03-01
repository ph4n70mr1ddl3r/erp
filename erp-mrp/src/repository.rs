use crate::models::*;
use anyhow::Result;
use erp_core::BaseEntity;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

pub struct MrpRepository;

impl MrpRepository {
    pub async fn create_run(pool: &SqlitePool, run: &MRPRun) -> Result<()> {
        let id = run.base.id.to_string();
        let run_date = run.run_date.to_string();
        let start_date = run.start_date.to_string();
        let end_date = run.end_date.to_string();
        let safety_stock_method = serde_json::to_string(&run.safety_stock_method)?;
        let status = serde_json::to_string(&run.status)?;
        let completed_at = run.completed_at.map(|d| d.to_rfc3339());
        let created_by = run.created_by.map(|u| u.to_string());
        let created_at = run.base.created_at.to_rfc3339();
        let updated_at = run.base.updated_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO mrp_runs (id, run_number, name, planning_horizon_days, run_date, start_date, end_date, include_forecasts, include_sales_orders, include_work_orders, safety_stock_method, status, total_items_planned, total_suggestions, error_message, completed_at, created_by, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&id)
        .bind(&run.run_number)
        .bind(&run.name)
        .bind(run.planning_horizon_days)
        .bind(&run_date)
        .bind(&start_date)
        .bind(&end_date)
        .bind(run.include_forecasts)
        .bind(run.include_sales_orders)
        .bind(run.include_work_orders)
        .bind(&safety_stock_method)
        .bind(&status)
        .bind(run.total_items_planned)
        .bind(run.total_suggestions)
        .bind(&run.error_message)
        .bind(&completed_at)
        .bind(&created_by)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(pool).await?;
        Ok(())
    }

    pub async fn get_run(pool: &SqlitePool, id: Uuid) -> Result<Option<MRPRun>> {
        let row: Option<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT * FROM mrp_runs WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(pool).await?;
        Ok(row.map(|r| MRPRun {
            base: BaseEntity::new(),
            run_number: r.get::<String, _>("run_number"),
            name: r.get::<Option<String>, _>("name").unwrap_or_default(),
            planning_horizon_days: r.get::<i64, _>("planning_horizon_days") as i32,
            run_date: r.get::<String, _>("run_date").parse().ok().unwrap_or_else(|| chrono::Utc::now().date_naive()),
            start_date: r.get::<String, _>("start_date").parse().ok().unwrap_or_else(|| chrono::Utc::now().date_naive()),
            end_date: r.get::<String, _>("end_date").parse().ok().unwrap_or_else(|| chrono::Utc::now().date_naive()),
            include_forecasts: r.get::<i64, _>("include_forecasts") == 1,
            include_sales_orders: r.get::<i64, _>("include_sales_orders") == 1,
            include_work_orders: r.get::<i64, _>("include_work_orders") == 1,
            safety_stock_method: serde_json::from_str(&r.get::<String, _>("safety_stock_method")).unwrap(),
            status: serde_json::from_str(&r.get::<String, _>("status")).unwrap(),
            total_items_planned: r.get::<i64, _>("total_items_planned") as i32,
            total_suggestions: r.get::<i64, _>("total_suggestions") as i32,
            error_message: r.get::<Option<String>, _>("error_message"),
            completed_at: r.get::<Option<String>, _>("completed_at").map(|d| d.parse::<chrono::DateTime<chrono::Utc>>().unwrap()),
            created_by: r.get::<Option<String>, _>("created_by").as_deref().map(|u| Uuid::parse_str(u).unwrap()),
        }))
    }

    pub async fn create_suggestion(pool: &SqlitePool, suggestion: &MRPSuggestion) -> Result<()> {
        let id = suggestion.id.to_string();
        let run_id = suggestion.run_id.to_string();
        let product_id = suggestion.product_id.to_string();
        let warehouse_id = suggestion.warehouse_id.to_string();
        let action_type = serde_json::to_string(&suggestion.action_type)?;
        let required_date = suggestion.required_date.to_string();
        let suggested_date = suggestion.suggested_date.to_string();
        let source_demand_ids = serde_json::to_string(&suggestion.source_demand_ids)?;
        let status = serde_json::to_string(&suggestion.status)?;
        let converted_id = suggestion.converted_id.map(|u| u.to_string());
        let created_at = suggestion.created_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO mrp_suggestions (id, run_id, product_id, warehouse_id, action_type, quantity, required_date, suggested_date, lead_time_days, priority, reason, source_demand_ids, status, converted_type, converted_id, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&id)
        .bind(&run_id)
        .bind(&product_id)
        .bind(&warehouse_id)
        .bind(&action_type)
        .bind(suggestion.quantity)
        .bind(&required_date)
        .bind(&suggested_date)
        .bind(suggestion.lead_time_days)
        .bind(suggestion.priority)
        .bind(&suggestion.reason)
        .bind(&source_demand_ids)
        .bind(&status)
        .bind(&suggestion.converted_type)
        .bind(&converted_id)
        .bind(&created_at)
        .execute(pool).await?;
        Ok(())
    }

    pub async fn list_suggestions(pool: &SqlitePool, run_id: Uuid) -> Result<Vec<MRPSuggestion>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT * FROM mrp_suggestions WHERE run_id = ? ORDER BY priority, required_date"#
        )
        .bind(run_id.to_string())
        .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| MRPSuggestion {
            id: Uuid::parse_str(&r.get::<String, _>("id")).unwrap(),
            run_id: Uuid::parse_str(&r.get::<String, _>("run_id")).unwrap(),
            product_id: Uuid::parse_str(&r.get::<String, _>("product_id")).unwrap(),
            warehouse_id: Uuid::parse_str(&r.get::<String, _>("warehouse_id")).unwrap(),
            action_type: serde_json::from_str(&r.get::<String, _>("action_type")).unwrap(),
            quantity: r.get::<i64, _>("quantity"),
            required_date: r.get::<String, _>("required_date").parse().unwrap(),
            suggested_date: r.get::<String, _>("suggested_date").parse().unwrap(),
            lead_time_days: r.get::<i64, _>("lead_time_days") as i32,
            priority: r.get::<i64, _>("priority") as i32,
            reason: r.get::<String, _>("reason"),
            source_demand_ids: serde_json::from_str(&r.get::<String, _>("source_demand_ids")).unwrap(),
            status: serde_json::from_str(&r.get::<String, _>("status")).unwrap(),
            converted_type: r.get::<Option<String>, _>("converted_type"),
            converted_id: r.get::<Option<String>, _>("converted_id").as_deref().map(|u| Uuid::parse_str(u).unwrap()),
            created_at: r.get::<String, _>("created_at").parse().unwrap(),
        }).collect())
    }

    pub async fn create_parameter(pool: &SqlitePool, param: &MRPParameter) -> Result<()> {
        let id = param.id.to_string();
        let product_id = param.product_id.to_string();
        let warehouse_id = param.warehouse_id.to_string();
        let planning_method = serde_json::to_string(&param.planning_method)?;
        let lot_size_method = serde_json::to_string(&param.lot_size_method)?;
        let order_policy = serde_json::to_string(&param.order_policy)?;
        let status = serde_json::to_string(&param.status)?;
        let created_at = param.created_at.to_rfc3339();
        let updated_at = param.updated_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO mrp_parameters (id, product_id, warehouse_id, planning_method, lot_size_method, fixed_lot_size, min_lot_size, max_lot_size, multiple_lot_size, safety_stock, safety_time_days, lead_time_days, planning_time_fence_days, order_policy, min_order_days, max_order_days, days_of_supply, service_level_percent, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&id)
        .bind(&product_id)
        .bind(&warehouse_id)
        .bind(&planning_method)
        .bind(&lot_size_method)
        .bind(param.fixed_lot_size)
        .bind(param.min_lot_size)
        .bind(param.max_lot_size)
        .bind(param.multiple_lot_size)
        .bind(param.safety_stock)
        .bind(param.safety_time_days)
        .bind(param.lead_time_days)
        .bind(param.planning_time_fence_days)
        .bind(&order_policy)
        .bind(param.min_order_days)
        .bind(param.max_order_days)
        .bind(param.days_of_supply)
        .bind(param.service_level_percent as f64)
        .bind(&status)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(pool).await?;
        Ok(())
    }

    pub async fn create_forecast(pool: &SqlitePool, forecast: &DemandForecast) -> Result<()> {
        let id = forecast.base.id.to_string();
        let start_date = forecast.start_date.to_string();
        let end_date = forecast.end_date.to_string();
        let forecast_method = serde_json::to_string(&forecast.forecast_method)?;
        let status = serde_json::to_string(&forecast.status)?;
        let created_by = forecast.created_by.map(|u| u.to_string());
        let created_at = forecast.base.created_at.to_rfc3339();
        let updated_at = forecast.base.updated_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO demand_forecasts (id, forecast_number, name, start_date, end_date, forecast_method, status, created_by, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&id)
        .bind(&forecast.forecast_number)
        .bind(&forecast.name)
        .bind(&start_date)
        .bind(&end_date)
        .bind(&forecast_method)
        .bind(&status)
        .bind(&created_by)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(pool).await?;
        Ok(())
    }

    pub async fn create_planned_order(pool: &SqlitePool, order: &PlannedOrder) -> Result<()> {
        let id = order.base.id.to_string();
        let run_id = order.run_id.to_string();
        let product_id = order.product_id.to_string();
        let warehouse_id = order.warehouse_id.to_string();
        let order_type = serde_json::to_string(&order.order_type)?;
        let start_date = order.start_date.to_string();
        let due_date = order.due_date.to_string();
        let bom_id = order.bom_id.map(|u| u.to_string());
        let routing_id = order.routing_id.map(|u| u.to_string());
        let status = serde_json::to_string(&order.status)?;
        let converted_id = order.converted_id.map(|u| u.to_string());
        let created_at = order.base.created_at.to_rfc3339();
        let updated_at = order.base.updated_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO planned_orders (id, order_number, run_id, product_id, warehouse_id, order_type, quantity, start_date, due_date, bom_id, routing_id, source_demand_ids, status, firmed, converted_type, converted_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&id)
        .bind(&order.order_number)
        .bind(&run_id)
        .bind(&product_id)
        .bind(&warehouse_id)
        .bind(&order_type)
        .bind(order.quantity)
        .bind(&start_date)
        .bind(&due_date)
        .bind(&bom_id)
        .bind(&routing_id)
        .bind(&order.source_demand_ids)
        .bind(&status)
        .bind(order.firmed)
        .bind(&order.converted_type)
        .bind(&converted_id)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(pool).await?;
        Ok(())
    }

    pub async fn create_exception(pool: &SqlitePool, exception: &MRPException) -> Result<()> {
        let id = exception.id.to_string();
        let run_id = exception.run_id.to_string();
        let product_id = exception.product_id.to_string();
        let exception_type = serde_json::to_string(&exception.exception_type)?;
        let severity = serde_json::to_string(&exception.severity)?;
        let acknowledged_by = exception.acknowledged_by.map(|u| u.to_string());
        let acknowledged_at = exception.acknowledged_at.map(|d| d.to_rfc3339());
        let created_at = exception.created_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO mrp_exceptions (id, run_id, product_id, exception_type, severity, message, details, suggested_action, acknowledged, acknowledged_by, acknowledged_at, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&id)
        .bind(&run_id)
        .bind(&product_id)
        .bind(&exception_type)
        .bind(&severity)
        .bind(&exception.message)
        .bind(&exception.details)
        .bind(&exception.suggested_action)
        .bind(exception.acknowledged)
        .bind(&acknowledged_by)
        .bind(&acknowledged_at)
        .bind(&created_at)
        .execute(pool).await?;
        Ok(())
    }
}
