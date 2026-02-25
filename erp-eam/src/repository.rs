use crate::models::*;
use anyhow::Result;
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct EamRepository;

impl EamRepository {
    pub async fn create_equipment(pool: &SqlitePool, equipment: &EquipmentAsset) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO equipment_assets (id, asset_number, name, description, asset_type, category, manufacturer, model, serial_number, location_id, department_id, parent_asset_id, installation_date, warranty_end_date, criticality, status, acquisition_cost, depreciation_method, useful_life_years, current_book_value, meter_type, meter_unit, current_meter_reading, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            equipment.base.id.to_string(),
            equipment.asset_number,
            equipment.name,
            equipment.description,
            serde_json::to_string(&equipment.asset_type)?,
            equipment.category,
            equipment.manufacturer,
            equipment.model,
            equipment.serial_number,
            equipment.location_id.map(|u| u.to_string()),
            equipment.department_id.map(|u| u.to_string()),
            equipment.parent_asset_id.map(|u| u.to_string()),
            equipment.installation_date.map(|d| d.to_string()),
            equipment.warranty_end_date.map(|d| d.to_string()),
            serde_json::to_string(&equipment.criticality)?,
            serde_json::to_string(&equipment.status)?,
            equipment.acquisition_cost,
            equipment.depreciation_method,
            equipment.useful_life_years,
            equipment.current_book_value,
            equipment.meter_type.as_ref().map(|t| serde_json::to_string(t).unwrap()),
            equipment.meter_unit,
            equipment.current_meter_reading,
            equipment.created_at.to_rfc3339(),
            equipment.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(())
    }

    pub async fn get_equipment(pool: &SqlitePool, id: Uuid) -> Result<Option<EquipmentAsset>> {
        let row = sqlx::query!(
            r#"SELECT * FROM equipment_assets WHERE id = ?"#,
            id.to_string()
        ).fetch_optional(pool).await?;
        Ok(row.map(|r| EquipmentAsset {
            base: BaseEntity::new(),
            asset_number: r.asset_number,
            name: r.name,
            description: r.description,
            asset_type: serde_json::from_str(&r.asset_type).unwrap(),
            category: r.category,
            manufacturer: r.manufacturer,
            model: r.model,
            serial_number: r.serial_number,
            location_id: r.location_id.map(|u| Uuid::parse_str(&u).unwrap()),
            department_id: r.department_id.map(|u| Uuid::parse_str(&u).unwrap()),
            parent_asset_id: r.parent_asset_id.map(|u| Uuid::parse_str(&u).unwrap()),
            installation_date: r.installation_date.map(|d| d.parse().unwrap()),
            warranty_end_date: r.warranty_end_date.map(|d| d.parse().unwrap()),
            criticality: serde_json::from_str(&r.criticality).unwrap(),
            status: serde_json::from_str(&r.status).unwrap(),
            acquisition_cost: r.acquisition_cost,
            depreciation_method: r.depreciation_method,
            useful_life_years: r.useful_life_years,
            current_book_value: r.current_book_value,
            meter_type: r.meter_type.map(|t| serde_json::from_str(&t).unwrap()),
            meter_unit: r.meter_unit,
            current_meter_reading: r.current_meter_reading,
            created_at: r.created_at.parse().unwrap(),
            updated_at: r.updated_at.parse().unwrap(),
        }))
    }

    pub async fn create_work_order(pool: &SqlitePool, wo: &WorkOrder) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO work_orders (id, wo_number, description, work_order_type, priority, asset_id, location_id, failure_code_id, problem_description, cause_description, remedy_description, requested_by, requested_date, required_date, scheduled_start, scheduled_end, actual_start, actual_end, assigned_to, assigned_team_id, status, estimated_labor_hours, actual_labor_hours, estimated_cost, actual_cost, downtime_hours, completion_notes, closed_by, closed_at, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            wo.base.id.to_string(),
            wo.wo_number,
            wo.description,
            serde_json::to_string(&wo.work_order_type)?,
            serde_json::to_string(&wo.priority)?,
            wo.asset_id.map(|u| u.to_string()),
            wo.location_id.map(|u| u.to_string()),
            wo.failure_code_id.map(|u| u.to_string()),
            wo.problem_description,
            wo.cause_description,
            wo.remedy_description,
            wo.requested_by.map(|u| u.to_string()),
            wo.requested_date.to_string(),
            wo.required_date.map(|d| d.to_string()),
            wo.scheduled_start.map(|d| d.to_rfc3339()),
            wo.scheduled_end.map(|d| d.to_rfc3339()),
            wo.actual_start.map(|d| d.to_rfc3339()),
            wo.actual_end.map(|d| d.to_rfc3339()),
            wo.assigned_to.map(|u| u.to_string()),
            wo.assigned_team_id.map(|u| u.to_string()),
            serde_json::to_string(&wo.status)?,
            wo.estimated_labor_hours,
            wo.actual_labor_hours,
            wo.estimated_cost,
            wo.actual_cost,
            wo.downtime_hours,
            wo.completion_notes,
            wo.closed_by.map(|u| u.to_string()),
            wo.closed_at.map(|d| d.to_rfc3339()),
            wo.created_at.to_rfc3339(),
            wo.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(())
    }

    pub async fn get_work_order(pool: &SqlitePool, id: Uuid) -> Result<Option<WorkOrder>> {
        let row = sqlx::query!(
            r#"SELECT * FROM work_orders WHERE id = ?"#,
            id.to_string()
        ).fetch_optional(pool).await?;
        Ok(row.map(|r| WorkOrder {
            base: BaseEntity::new(),
            wo_number: r.wo_number,
            description: r.description,
            work_order_type: serde_json::from_str(&r.work_order_type).unwrap(),
            priority: serde_json::from_str(&r.priority).unwrap(),
            asset_id: r.asset_id.map(|u| Uuid::parse_str(&u).unwrap()),
            location_id: r.location_id.map(|u| Uuid::parse_str(&u).unwrap()),
            failure_code_id: r.failure_code_id.map(|u| Uuid::parse_str(&u).unwrap()),
            problem_description: r.problem_description,
            cause_description: r.cause_description,
            remedy_description: r.remedy_description,
            requested_by: r.requested_by.map(|u| Uuid::parse_str(&u).unwrap()),
            requested_date: r.requested_date.parse().unwrap(),
            required_date: r.required_date.map(|d| d.parse().unwrap()),
            scheduled_start: r.scheduled_start.map(|d| d.parse().unwrap()),
            scheduled_end: r.scheduled_end.map(|d| d.parse().unwrap()),
            actual_start: r.actual_start.map(|d| d.parse().unwrap()),
            actual_end: r.actual_end.map(|d| d.parse().unwrap()),
            assigned_to: r.assigned_to.map(|u| Uuid::parse_str(&u).unwrap()),
            assigned_team_id: r.assigned_team_id.map(|u| Uuid::parse_str(&u).unwrap()),
            status: serde_json::from_str(&r.status).unwrap(),
            estimated_labor_hours: r.estimated_labor_hours,
            actual_labor_hours: r.actual_labor_hours,
            estimated_cost: r.estimated_cost,
            actual_cost: r.actual_cost,
            downtime_hours: r.downtime_hours,
            completion_notes: r.completion_notes,
            closed_by: r.closed_by.map(|u| Uuid::parse_str(&u).unwrap()),
            closed_at: r.closed_at.map(|d| d.parse().unwrap()),
            created_at: r.created_at.parse().unwrap(),
            updated_at: r.updated_at.parse().unwrap(),
        }))
    }

    pub async fn create_pm_schedule(pool: &SqlitePool, pm: &PreventiveMaintenanceSchedule) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO pm_schedules (id, pm_number, name, description, asset_id, maintenance_strategy, frequency_type, frequency_value, last_performed_date, next_due_date, meter_based, last_meter_reading, next_meter_due, estimated_duration_hours, estimated_cost, auto_generate_wo, lead_time_days, checklist_id, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            pm.base.id.to_string(),
            pm.pm_number,
            pm.name,
            pm.description,
            pm.asset_id.to_string(),
            serde_json::to_string(&pm.maintenance_strategy)?,
            serde_json::to_string(&pm.frequency_type)?,
            pm.frequency_value,
            pm.last_performed_date.map(|d| d.to_string()),
            pm.next_due_date.to_string(),
            pm.meter_based as i32,
            pm.last_meter_reading,
            pm.next_meter_due,
            pm.estimated_duration_hours,
            pm.estimated_cost,
            pm.auto_generate_wo as i32,
            pm.lead_time_days,
            pm.checklist_id.map(|u| u.to_string()),
            serde_json::to_string(&pm.status)?,
            pm.created_at.to_rfc3339(),
            pm.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(())
    }

    pub async fn create_spare_part(pool: &SqlitePool, part: &SparePart) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO spare_parts (id, part_number, name, description, category, manufacturer, unit_of_measure, unit_cost, min_stock_level, max_stock_level, reorder_point, current_stock, warehouse_id, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            part.base.id.to_string(),
            part.part_number,
            part.name,
            part.description,
            part.category,
            part.manufacturer,
            part.unit_of_measure,
            part.unit_cost,
            part.min_stock_level,
            part.max_stock_level,
            part.reorder_point,
            part.current_stock,
            part.warehouse_id.to_string(),
            serde_json::to_string(&part.status)?,
            part.created_at.to_rfc3339(),
            part.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(())
    }
}
