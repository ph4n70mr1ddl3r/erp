use crate::models::*;
use crate::repository::EamRepository;
use anyhow::Result;
use chrono::Utc;
use erp_core::{BaseEntity, Status};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct EamService;

impl EamService {
    pub async fn create_equipment(
        pool: &SqlitePool,
        asset_number: String,
        name: String,
        asset_type: AssetType,
        criticality: AssetCriticality,
        acquisition_cost: i64,
    ) -> Result<EquipmentAsset> {
        let now = Utc::now();
        let equipment = EquipmentAsset {
            base: BaseEntity::new(),
            asset_number,
            name,
            description: None,
            asset_type,
            category: None,
            manufacturer: None,
            model: None,
            serial_number: None,
            location_id: None,
            department_id: None,
            parent_asset_id: None,
            installation_date: None,
            warranty_end_date: None,
            criticality,
            status: AssetOperationalStatus::Running,
            acquisition_cost,
            depreciation_method: None,
            useful_life_years: None,
            current_book_value: acquisition_cost,
            meter_type: None,
            meter_unit: None,
            current_meter_reading: None,
            created_at: now,
            updated_at: now,
        };
        
        EamRepository::create_equipment(pool, &equipment).await?;
        Ok(equipment)
    }

    pub async fn create_work_order(
        pool: &SqlitePool,
        description: String,
        work_order_type: WorkOrderType,
        priority: WorkOrderPriority,
        asset_id: Option<Uuid>,
        requested_by: Option<Uuid>,
    ) -> Result<WorkOrder> {
        let now = Utc::now();
        let wo = WorkOrder {
            base: BaseEntity::new(),
            wo_number: format!("WO-{}", now.format("%Y%m%d%H%M%S")),
            description,
            work_order_type,
            priority,
            asset_id,
            location_id: None,
            failure_code_id: None,
            problem_description: None,
            cause_description: None,
            remedy_description: None,
            requested_by,
            requested_date: now.date_naive(),
            required_date: None,
            scheduled_start: None,
            scheduled_end: None,
            actual_start: None,
            actual_end: None,
            assigned_to: None,
            assigned_team_id: None,
            status: WorkOrderStatus::Requested,
            estimated_labor_hours: 0.0,
            actual_labor_hours: 0.0,
            estimated_cost: 0,
            actual_cost: 0,
            downtime_hours: 0.0,
            completion_notes: None,
            closed_by: None,
            closed_at: None,
            created_at: now,
            updated_at: now,
        };
        
        EamRepository::create_work_order(pool, &wo).await?;
        Ok(wo)
    }

    pub async fn create_pm_schedule(
        pool: &SqlitePool,
        name: String,
        asset_id: Uuid,
        maintenance_strategy: MaintenanceStrategy,
        frequency_type: FrequencyType,
        frequency_value: i32,
    ) -> Result<PreventiveMaintenanceSchedule> {
        let now = Utc::now();
        let next_due = now.date_naive() + chrono::Duration::days(frequency_value as i64);
        
        let pm = PreventiveMaintenanceSchedule {
            base: BaseEntity::new(),
            pm_number: format!("PM-{}", now.format("%Y%m%d%H%M%S")),
            name,
            description: None,
            asset_id,
            maintenance_strategy,
            frequency_type,
            frequency_value,
            last_performed_date: None,
            next_due_date: next_due,
            meter_based: false,
            last_meter_reading: None,
            next_meter_due: None,
            estimated_duration_hours: 1.0,
            estimated_cost: 0,
            auto_generate_wo: true,
            lead_time_days: 7,
            checklist_id: None,
            status: Status::Active,
            created_at: now,
            updated_at: now,
        };
        
        EamRepository::create_pm_schedule(pool, &pm).await?;
        Ok(pm)
    }

    pub async fn create_spare_part(
        pool: &SqlitePool,
        part_number: String,
        name: String,
        unit_of_measure: String,
        unit_cost: i64,
        warehouse_id: Uuid,
    ) -> Result<SparePart> {
        let now = Utc::now();
        let part = SparePart {
            base: BaseEntity::new(),
            part_number,
            name,
            description: None,
            category: None,
            manufacturer: None,
            unit_of_measure,
            unit_cost,
            min_stock_level: 0,
            max_stock_level: 0,
            reorder_point: 0,
            current_stock: 0,
            warehouse_id,
            status: Status::Active,
            created_at: now,
            updated_at: now,
        };
        
        EamRepository::create_spare_part(pool, &part).await?;
        Ok(part)
    }

    pub async fn get_equipment(pool: &SqlitePool, id: Uuid) -> Result<Option<EquipmentAsset>> {
        EamRepository::get_equipment(pool, id).await
    }

    pub async fn get_work_order(pool: &SqlitePool, id: Uuid) -> Result<Option<WorkOrder>> {
        EamRepository::get_work_order(pool, id).await
    }
}
