use crate::models::*;
use crate::repository::MrpRepository;
use anyhow::Result;
use chrono::{NaiveDate, Utc};
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct MrpService;

impl MrpService {
    pub async fn create_run(
        pool: &SqlitePool,
        name: String,
        planning_horizon_days: i32,
        include_forecasts: bool,
        include_sales_orders: bool,
        include_work_orders: bool,
        created_by: Option<Uuid>,
    ) -> Result<MRPRun> {
        let now = Utc::now();
        let run_date = now.date_naive();
        let end_date = run_date + chrono::Duration::days(planning_horizon_days as i64);
        
        let run = MRPRun {
            base: BaseEntity::new(),
            run_number: format!("MRP-{}", now.format("%Y%m%d%H%M%S")),
            name,
            planning_horizon_days,
            run_date,
            start_date: run_date,
            end_date,
            include_forecasts,
            include_sales_orders,
            include_work_orders,
            safety_stock_method: SafetyStockMethod::DaysOfSupply,
            status: MRPStatus::Draft,
            total_items_planned: 0,
            total_suggestions: 0,
            error_message: None,
            completed_at: None,
            created_by,
        };
        
        MrpRepository::create_run(pool, &run).await?;
        Ok(run)
    }

    pub async fn create_parameter(
        pool: &SqlitePool,
        product_id: Uuid,
        warehouse_id: Uuid,
        lead_time_days: i32,
        safety_stock: i64,
    ) -> Result<MRPParameter> {
        let now = Utc::now();
        let param = MRPParameter {
            id: Uuid::new_v4(),
            product_id,
            warehouse_id,
            planning_method: PlanningMethod::MRP,
            lot_size_method: LotSizeMethod::LotForLot,
            fixed_lot_size: 0,
            min_lot_size: 1,
            max_lot_size: 0,
            multiple_lot_size: 1,
            safety_stock,
            safety_time_days: 0,
            lead_time_days,
            planning_time_fence_days: 7,
            order_policy: OrderPolicy::Backward,
            min_order_days: 1,
            max_order_days: 365,
            days_of_supply: 14,
            service_level_percent: 95,
            status: Status::Active,
            created_at: now,
            updated_at: now,
        };
        
        MrpRepository::create_parameter(pool, &param).await?;
        Ok(param)
    }

    pub async fn create_forecast(
        pool: &SqlitePool,
        name: String,
        start_date: NaiveDate,
        end_date: NaiveDate,
        forecast_method: ForecastMethod,
        created_by: Option<Uuid>,
    ) -> Result<DemandForecast> {
        let forecast = DemandForecast {
            base: BaseEntity::new(),
            forecast_number: format!("FC-{}", Utc::now().format("%Y%m%d%H%M%S")),
            name,
            start_date,
            end_date,
            forecast_method,
            status: Status::Draft,
            created_by,
            created_at: Utc::now(),
        };
        
        MrpRepository::create_forecast(pool, &forecast).await?;
        Ok(forecast)
    }

    pub async fn create_planned_order(
        pool: &SqlitePool,
        run_id: Uuid,
        product_id: Uuid,
        warehouse_id: Uuid,
        order_type: MRPActionType,
        quantity: i64,
        due_date: NaiveDate,
        lead_time_days: i32,
        source_demand_ids: Vec<Uuid>,
    ) -> Result<PlannedOrder> {
        let start_date = due_date - chrono::Duration::days(lead_time_days as i64);
        let order = PlannedOrder {
            base: BaseEntity::new(),
            order_number: format!("PLN-{}", Utc::now().format("%Y%m%d%H%M%S")),
            run_id,
            product_id,
            warehouse_id,
            order_type,
            quantity,
            start_date,
            due_date,
            bom_id: None,
            routing_id: None,
            source_demand_ids: serde_json::to_string(&source_demand_ids)?,
            status: PlannedOrderStatus::Open,
            firmed: false,
            converted_type: None,
            converted_id: None,
        };
        
        MrpRepository::create_planned_order(pool, &order).await?;
        Ok(order)
    }

    pub async fn get_run(pool: &SqlitePool, id: Uuid) -> Result<Option<MRPRun>> {
        MrpRepository::get_run(pool, id).await
    }

    pub async fn list_suggestions(pool: &SqlitePool, run_id: Uuid) -> Result<Vec<MRPSuggestion>> {
        MrpRepository::list_suggestions(pool, run_id).await
    }
}
