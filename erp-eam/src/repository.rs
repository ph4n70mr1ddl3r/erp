use crate::models::*;
use anyhow::{Context, Result};
use erp_core::BaseEntity;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

pub struct EamRepository;

impl EamRepository {
    pub async fn create_equipment(pool: &SqlitePool, equipment: &EquipmentAsset) -> Result<()> {
        let id = equipment.base.id.to_string();
        let asset_type = serde_json::to_string(&equipment.asset_type)?;
        let location_id = equipment.location_id.map(|u| u.to_string());
        let department_id = equipment.department_id.map(|u| u.to_string());
        let parent_asset_id = equipment.parent_asset_id.map(|u| u.to_string());
        let installation_date = equipment.installation_date.map(|d| d.to_string());
        let warranty_end_date = equipment.warranty_end_date.map(|d| d.to_string());
        let criticality = serde_json::to_string(&equipment.criticality)?;
        let status = serde_json::to_string(&equipment.status)?;
        let meter_type = equipment.meter_type.as_ref().map(serde_json::to_string).transpose()?;
        let created_at = equipment.created_at.to_rfc3339();
        let updated_at = equipment.updated_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO equipment_assets (id, asset_number, name, description, asset_type, category, manufacturer, model, serial_number, location_id, department_id, parent_asset_id, installation_date, warranty_end_date, criticality, status, acquisition_cost, depreciation_method, useful_life_years, current_book_value, meter_type, meter_unit, current_meter_reading, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&id)
        .bind(&equipment.asset_number)
        .bind(&equipment.name)
        .bind(&equipment.description)
        .bind(&asset_type)
        .bind(&equipment.category)
        .bind(&equipment.manufacturer)
        .bind(&equipment.model)
        .bind(&equipment.serial_number)
        .bind(&location_id)
        .bind(&department_id)
        .bind(&parent_asset_id)
        .bind(&installation_date)
        .bind(&warranty_end_date)
        .bind(&criticality)
        .bind(&status)
        .bind(equipment.acquisition_cost)
        .bind(&equipment.depreciation_method)
        .bind(equipment.useful_life_years)
        .bind(equipment.current_book_value)
        .bind(&meter_type)
        .bind(&equipment.meter_unit)
        .bind(equipment.current_meter_reading)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(pool).await?;
        Ok(())
    }

    pub async fn get_equipment(pool: &SqlitePool, id: Uuid) -> Result<Option<EquipmentAsset>> {
        let id_str = id.to_string();
        let row = sqlx::query(
            r#"SELECT * FROM equipment_assets WHERE id = ?"#
        )
        .bind(&id_str)
        .fetch_optional(pool).await?;
        
        match row {
            Some(r) => {
                let asset = EquipmentAsset {
                    base: BaseEntity::new(),
                    asset_number: r.get("asset_number"),
                    name: r.get("name"),
                    description: r.get("description"),
                    asset_type: serde_json::from_str(r.get::<String, _>("asset_type").as_str())?,
                    category: r.get("category"),
                    manufacturer: r.get("manufacturer"),
                    model: r.get("model"),
                    serial_number: r.get("serial_number"),
                    location_id: r.get::<Option<String>, _>("location_id").as_ref().and_then(|u| Uuid::parse_str(u).ok()),
                    department_id: r.get::<Option<String>, _>("department_id").as_ref().and_then(|u| Uuid::parse_str(u).ok()),
                    parent_asset_id: r.get::<Option<String>, _>("parent_asset_id").as_ref().and_then(|u| Uuid::parse_str(u).ok()),
                    installation_date: r.get::<Option<String>, _>("installation_date").as_ref().and_then(|d| d.parse().ok()),
                    warranty_end_date: r.get::<Option<String>, _>("warranty_end_date").as_ref().and_then(|d| d.parse().ok()),
                    criticality: serde_json::from_str(r.get::<String, _>("criticality").as_str())?,
                    status: r.get::<Option<String>, _>("status").as_ref().and_then(|s| serde_json::from_str(s).ok()).unwrap_or(AssetOperationalStatus::Running),
                    acquisition_cost: r.get::<Option<i64>, _>("acquisition_cost").unwrap_or(0),
                    depreciation_method: r.get("depreciation_method"),
                    useful_life_years: r.get::<Option<i32>, _>("useful_life_years"),
                    current_book_value: r.get::<Option<i64>, _>("current_book_value").unwrap_or(0),
                    meter_type: r.get::<Option<String>, _>("meter_type").as_ref().and_then(|t| serde_json::from_str(t).ok()),
                    meter_unit: r.get("meter_unit"),
                    current_meter_reading: r.get::<Option<i64>, _>("current_meter_reading"),
                    created_at: r.get::<String, _>("created_at").parse().context("Invalid created_at")?,
                    updated_at: r.get::<String, _>("updated_at").parse().context("Invalid updated_at")?,
                };
                Ok(Some(asset))
            }
            None => Ok(None),
        }
    }

    pub async fn create_work_order(pool: &SqlitePool, wo: &WorkOrder) -> Result<()> {
        let id = wo.base.id.to_string();
        let work_order_type = serde_json::to_string(&wo.work_order_type)?;
        let priority = serde_json::to_string(&wo.priority)?;
        let asset_id = wo.asset_id.map(|u| u.to_string());
        let location_id = wo.location_id.map(|u| u.to_string());
        let failure_code_id = wo.failure_code_id.map(|u| u.to_string());
        let requested_by = wo.requested_by.map(|u| u.to_string());
        let requested_date = wo.requested_date.to_string();
        let required_date = wo.required_date.map(|d| d.to_string());
        let scheduled_start = wo.scheduled_start.map(|d| d.to_rfc3339());
        let scheduled_end = wo.scheduled_end.map(|d| d.to_rfc3339());
        let actual_start = wo.actual_start.map(|d| d.to_rfc3339());
        let actual_end = wo.actual_end.map(|d| d.to_rfc3339());
        let assigned_to = wo.assigned_to.map(|u| u.to_string());
        let assigned_team_id = wo.assigned_team_id.map(|u| u.to_string());
        let status = serde_json::to_string(&wo.status)?;
        let closed_by = wo.closed_by.map(|u| u.to_string());
        let closed_at = wo.closed_at.map(|d| d.to_rfc3339());
        let created_at = wo.created_at.to_rfc3339();
        let updated_at = wo.updated_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO work_orders (id, wo_number, description, work_order_type, priority, asset_id, location_id, failure_code_id, problem_description, cause_description, remedy_description, requested_by, requested_date, required_date, scheduled_start, scheduled_end, actual_start, actual_end, assigned_to, assigned_team_id, status, estimated_labor_hours, actual_labor_hours, estimated_cost, actual_cost, downtime_hours, completion_notes, closed_by, closed_at, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&id)
        .bind(&wo.wo_number)
        .bind(&wo.description)
        .bind(&work_order_type)
        .bind(&priority)
        .bind(&asset_id)
        .bind(&location_id)
        .bind(&failure_code_id)
        .bind(&wo.problem_description)
        .bind(&wo.cause_description)
        .bind(&wo.remedy_description)
        .bind(&requested_by)
        .bind(&requested_date)
        .bind(&required_date)
        .bind(&scheduled_start)
        .bind(&scheduled_end)
        .bind(&actual_start)
        .bind(&actual_end)
        .bind(&assigned_to)
        .bind(&assigned_team_id)
        .bind(&status)
        .bind(wo.estimated_labor_hours)
        .bind(wo.actual_labor_hours)
        .bind(wo.estimated_cost)
        .bind(wo.actual_cost)
        .bind(wo.downtime_hours)
        .bind(&wo.completion_notes)
        .bind(&closed_by)
        .bind(&closed_at)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(pool).await?;
        Ok(())
    }

    pub async fn get_work_order(pool: &SqlitePool, id: Uuid) -> Result<Option<WorkOrder>> {
        let id_str = id.to_string();
        let row = sqlx::query(
            r#"SELECT * FROM work_orders WHERE id = ?"#
        )
        .bind(&id_str)
        .fetch_optional(pool).await?;
        
        match row {
            Some(r) => {
                let wo = WorkOrder {
                    base: BaseEntity::new(),
                    wo_number: r.get("wo_number"),
                    description: r.get("description"),
                    work_order_type: r.get::<Option<String>, _>("work_order_type").as_ref().and_then(|s| serde_json::from_str(s).ok()).unwrap_or(WorkOrderType::Corrective),
                    priority: r.get::<Option<String>, _>("priority").as_ref().and_then(|s| serde_json::from_str(s).ok()).unwrap_or(WorkOrderPriority::Medium),
                    asset_id: r.get::<Option<String>, _>("asset_id").as_ref().and_then(|u| Uuid::parse_str(u).ok()),
                    location_id: r.get::<Option<String>, _>("location_id").as_ref().and_then(|u| Uuid::parse_str(u).ok()),
                    failure_code_id: r.get::<Option<String>, _>("failure_code_id").as_ref().and_then(|u| Uuid::parse_str(u).ok()),
                    problem_description: r.get("problem_description"),
                    cause_description: r.get("cause_description"),
                    remedy_description: r.get("remedy_description"),
                    requested_by: r.get::<Option<String>, _>("requested_by").as_ref().and_then(|u| Uuid::parse_str(u).ok()),
                    requested_date: r.get::<Option<String>, _>("requested_date").and_then(|d| d.parse().ok()).unwrap_or_else(|| chrono::Utc::now().date_naive()),
                    required_date: r.get::<Option<String>, _>("required_date").as_ref().and_then(|d| d.parse().ok()),
                    scheduled_start: r.get::<Option<String>, _>("scheduled_start").as_ref().and_then(|d| d.parse().ok()),
                    scheduled_end: r.get::<Option<String>, _>("scheduled_end").as_ref().and_then(|d| d.parse().ok()),
                    actual_start: r.get::<Option<String>, _>("actual_start").as_ref().and_then(|d| d.parse().ok()),
                    actual_end: r.get::<Option<String>, _>("actual_end").as_ref().and_then(|d| d.parse().ok()),
                    assigned_to: r.get::<Option<String>, _>("assigned_to").as_ref().and_then(|u| Uuid::parse_str(u).ok()),
                    assigned_team_id: r.get::<Option<String>, _>("assigned_team_id").as_ref().and_then(|u| Uuid::parse_str(u).ok()),
                    status: serde_json::from_str(r.get::<String, _>("status").as_str())?,
                    estimated_labor_hours: r.get::<Option<f64>, _>("estimated_labor_hours").unwrap_or(0.0),
                    actual_labor_hours: r.get::<Option<f64>, _>("actual_labor_hours").unwrap_or(0.0),
                    estimated_cost: r.get::<Option<i64>, _>("estimated_cost").unwrap_or(0),
                    actual_cost: r.get::<Option<i64>, _>("actual_cost").unwrap_or(0),
                    downtime_hours: r.get::<Option<f64>, _>("downtime_hours").unwrap_or(0.0),
                    completion_notes: r.get("completion_notes"),
                    closed_by: r.get::<Option<String>, _>("closed_by").as_ref().and_then(|u| Uuid::parse_str(u).ok()),
                    closed_at: r.get::<Option<String>, _>("closed_at").as_ref().and_then(|d| d.parse().ok()),
                    created_at: r.get::<String, _>("created_at").parse().context("Invalid created_at")?,
                    updated_at: r.get::<String, _>("updated_at").parse().context("Invalid updated_at")?,
                };
                Ok(Some(wo))
            }
            None => Ok(None),
        }
    }

    pub async fn create_pm_schedule(pool: &SqlitePool, pm: &PreventiveMaintenanceSchedule) -> Result<()> {
        let id = pm.base.id.to_string();
        let asset_id = pm.asset_id.to_string();
        let maintenance_strategy = serde_json::to_string(&pm.maintenance_strategy)?;
        let frequency_type = serde_json::to_string(&pm.frequency_type)?;
        let last_performed_date = pm.last_performed_date.map(|d| d.to_string());
        let next_due_date = pm.next_due_date.to_string();
        let checklist_id = pm.checklist_id.map(|u| u.to_string());
        let status = serde_json::to_string(&pm.status)?;
        let created_at = pm.created_at.to_rfc3339();
        let updated_at = pm.updated_at.to_rfc3339();
        let meter_based = pm.meter_based as i32;
        let auto_generate_wo = pm.auto_generate_wo as i32;
        sqlx::query(
            r#"INSERT INTO pm_schedules (id, pm_number, name, description, asset_id, maintenance_strategy, frequency_type, frequency_value, last_performed_date, next_due_date, meter_based, last_meter_reading, next_meter_due, estimated_duration_hours, estimated_cost, auto_generate_wo, lead_time_days, checklist_id, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&id)
        .bind(&pm.pm_number)
        .bind(&pm.name)
        .bind(&pm.description)
        .bind(&asset_id)
        .bind(&maintenance_strategy)
        .bind(&frequency_type)
        .bind(pm.frequency_value)
        .bind(&last_performed_date)
        .bind(&next_due_date)
        .bind(meter_based)
        .bind(pm.last_meter_reading)
        .bind(pm.next_meter_due)
        .bind(pm.estimated_duration_hours)
        .bind(pm.estimated_cost)
        .bind(auto_generate_wo)
        .bind(pm.lead_time_days)
        .bind(&checklist_id)
        .bind(&status)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(pool).await?;
        Ok(())
    }

    pub async fn create_spare_part(pool: &SqlitePool, part: &SparePart) -> Result<()> {
        let id = part.base.id.to_string();
        let warehouse_id = part.warehouse_id.to_string();
        let status = serde_json::to_string(&part.status)?;
        let created_at = part.created_at.to_rfc3339();
        let updated_at = part.updated_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO spare_parts (id, part_number, name, description, category, manufacturer, unit_of_measure, unit_cost, min_stock_level, max_stock_level, reorder_point, current_stock, warehouse_id, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(&id)
        .bind(&part.part_number)
        .bind(&part.name)
        .bind(&part.description)
        .bind(&part.category)
        .bind(&part.manufacturer)
        .bind(&part.unit_of_measure)
        .bind(part.unit_cost)
        .bind(part.min_stock_level)
        .bind(part.max_stock_level)
        .bind(part.reorder_point)
        .bind(part.current_stock)
        .bind(&warehouse_id)
        .bind(&status)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(pool).await?;
        Ok(())
    }
}
