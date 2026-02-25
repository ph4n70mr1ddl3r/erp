use crate::models::*;
use anyhow::Result;
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct MrpRepository;

impl MrpRepository {
    pub async fn create_run(pool: &SqlitePool, run: &MRPRun) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO mrp_runs (id, run_number, name, planning_horizon_days, run_date, start_date, end_date, include_forecasts, include_sales_orders, include_work_orders, safety_stock_method, status, total_items_planned, total_suggestions, error_message, completed_at, created_by, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            run.base.id.to_string(),
            run.run_number,
            run.name,
            run.planning_horizon_days,
            run.run_date.to_string(),
            run.start_date.to_string(),
            run.end_date.to_string(),
            run.include_forecasts,
            run.include_sales_orders,
            run.include_work_orders,
            serde_json::to_string(&run.safety_stock_method)?,
            serde_json::to_string(&run.status)?,
            run.total_items_planned,
            run.total_suggestions,
            run.error_message,
            run.completed_at.map(|d| d.to_rfc3339()),
            run.created_by.map(|u| u.to_string()),
            run.base.created_at.to_rfc3339(),
            run.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(())
    }

    pub async fn get_run(pool: &SqlitePool, id: Uuid) -> Result<Option<MRPRun>> {
        let row = sqlx::query!(
            r#"SELECT * FROM mrp_runs WHERE id = ?"#,
            id.to_string()
        ).fetch_optional(pool).await?;
        Ok(row.map(|r| MRPRun {
            base: BaseEntity::new(),
            run_number: r.run_number,
            name: r.name,
            planning_horizon_days: r.planning_horizon_days,
            run_date: r.run_date.parse().unwrap(),
            start_date: r.start_date.parse().unwrap(),
            end_date: r.end_date.parse().unwrap(),
            include_forecasts: r.include_forecasts == 1,
            include_sales_orders: r.include_sales_orders == 1,
            include_work_orders: r.include_work_orders == 1,
            safety_stock_method: serde_json::from_str(&r.safety_stock_method).unwrap(),
            status: serde_json::from_str(&r.status).unwrap(),
            total_items_planned: r.total_items_planned,
            total_suggestions: r.total_suggestions,
            error_message: r.error_message,
            completed_at: r.completed_at.map(|d| d.parse().unwrap()),
            created_by: r.created_by.map(|u| Uuid::parse_str(&u).unwrap()),
        }))
    }

    pub async fn create_suggestion(pool: &SqlitePool, suggestion: &MRPSuggestion) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO mrp_suggestions (id, run_id, product_id, warehouse_id, action_type, quantity, required_date, suggested_date, lead_time_days, priority, reason, source_demand_ids, status, converted_type, converted_id, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            suggestion.id.to_string(),
            suggestion.run_id.to_string(),
            suggestion.product_id.to_string(),
            suggestion.warehouse_id.to_string(),
            serde_json::to_string(&suggestion.action_type)?,
            suggestion.quantity,
            suggestion.required_date.to_string(),
            suggestion.suggested_date.to_string(),
            suggestion.lead_time_days,
            suggestion.priority,
            suggestion.reason,
            serde_json::to_string(&suggestion.source_demand_ids)?,
            serde_json::to_string(&suggestion.status)?,
            suggestion.converted_type,
            suggestion.converted_id.map(|u| u.to_string()),
            suggestion.created_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(())
    }

    pub async fn list_suggestions(pool: &SqlitePool, run_id: Uuid) -> Result<Vec<MRPSuggestion>> {
        let rows = sqlx::query!(
            r#"SELECT * FROM mrp_suggestions WHERE run_id = ? ORDER BY priority, required_date"#,
            run_id.to_string()
        ).fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| MRPSuggestion {
            id: Uuid::parse_str(&r.id).unwrap(),
            run_id: Uuid::parse_str(&r.run_id).unwrap(),
            product_id: Uuid::parse_str(&r.product_id).unwrap(),
            warehouse_id: Uuid::parse_str(&r.warehouse_id).unwrap(),
            action_type: serde_json::from_str(&r.action_type).unwrap(),
            quantity: r.quantity,
            required_date: r.required_date.parse().unwrap(),
            suggested_date: r.suggested_date.parse().unwrap(),
            lead_time_days: r.lead_time_days,
            priority: r.priority,
            reason: r.reason,
            source_demand_ids: serde_json::from_str(&r.source_demand_ids).unwrap(),
            status: serde_json::from_str(&r.status).unwrap(),
            converted_type: r.converted_type,
            converted_id: r.converted_id.map(|u| Uuid::parse_str(&u).unwrap()),
            created_at: r.created_at.parse().unwrap(),
        }).collect())
    }

    pub async fn create_parameter(pool: &SqlitePool, param: &MRPParameter) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO mrp_parameters (id, product_id, warehouse_id, planning_method, lot_size_method, fixed_lot_size, min_lot_size, max_lot_size, multiple_lot_size, safety_stock, safety_time_days, lead_time_days, planning_time_fence_days, order_policy, min_order_days, max_order_days, days_of_supply, service_level_percent, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            param.id.to_string(),
            param.product_id.to_string(),
            param.warehouse_id.to_string(),
            serde_json::to_string(&param.planning_method)?,
            serde_json::to_string(&param.lot_size_method)?,
            param.fixed_lot_size,
            param.min_lot_size,
            param.max_lot_size,
            param.multiple_lot_size,
            param.safety_stock,
            param.safety_time_days,
            param.lead_time_days,
            param.planning_time_fence_days,
            serde_json::to_string(&param.order_policy)?,
            param.min_order_days,
            param.max_order_days,
            param.days_of_supply,
            param.service_level_percent,
            serde_json::to_string(&param.status)?,
            param.created_at.to_rfc3339(),
            param.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(())
    }

    pub async fn create_forecast(pool: &SqlitePool, forecast: &DemandForecast) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO demand_forecasts (id, forecast_number, name, start_date, end_date, forecast_method, status, created_by, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            forecast.base.id.to_string(),
            forecast.forecast_number,
            forecast.name,
            forecast.start_date.to_string(),
            forecast.end_date.to_string(),
            serde_json::to_string(&forecast.forecast_method)?,
            serde_json::to_string(&forecast.status)?,
            forecast.created_by.map(|u| u.to_string()),
            forecast.base.created_at.to_rfc3339(),
            forecast.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(())
    }

    pub async fn create_planned_order(pool: &SqlitePool, order: &PlannedOrder) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO planned_orders (id, order_number, run_id, product_id, warehouse_id, order_type, quantity, start_date, due_date, bom_id, routing_id, source_demand_ids, status, firmed, converted_type, converted_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            order.base.id.to_string(),
            order.order_number,
            order.run_id.to_string(),
            order.product_id.to_string(),
            order.warehouse_id.to_string(),
            serde_json::to_string(&order.order_type)?,
            order.quantity,
            order.start_date.to_string(),
            order.due_date.to_string(),
            order.bom_id.map(|u| u.to_string()),
            order.routing_id.map(|u| u.to_string()),
            order.source_demand_ids,
            serde_json::to_string(&order.status)?,
            order.firmed as i32,
            order.converted_type,
            order.converted_id.map(|u| u.to_string()),
            order.base.created_at.to_rfc3339(),
            order.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(())
    }

    pub async fn create_exception(pool: &SqlitePool, exception: &MRPException) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO mrp_exceptions (id, run_id, product_id, exception_type, severity, message, details, suggested_action, acknowledged, acknowledged_by, acknowledged_at, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            exception.id.to_string(),
            exception.run_id.to_string(),
            exception.product_id.to_string(),
            serde_json::to_string(&exception.exception_type)?,
            serde_json::to_string(&exception.severity)?,
            exception.message,
            exception.details,
            exception.suggested_action,
            exception.acknowledged as i32,
            exception.acknowledged_by.map(|u| u.to_string()),
            exception.acknowledged_at.map(|d| d.to_rfc3339()),
            exception.created_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(())
    }
}
