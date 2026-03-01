use chrono::Utc;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct DemandService<R: DemandRepository> {
    pub repo: R,
}

impl DemandService<SqliteDemandRepository> {
    pub fn new(repo: SqliteDemandRepository) -> Self {
        Self { repo }
    }
}

impl<R: DemandRepository> DemandService<R> {
    pub async fn run_forecast(&self, _req: RunForecastRequest) -> anyhow::Result<ForecastResult> {
        Ok(ForecastResult {
            model_id: Uuid::new_v4(),
            forecasts: vec![],
            accuracy_metrics: ForecastAccuracyMetrics {
                mape: 0.15,
                mad: 10.0,
                mse: 150.0,
                rmse: 12.25,
            },
        })
    }

    pub async fn create_demand_plan(&self, req: CreateDemandPlanRequest) -> anyhow::Result<DemandPlan> {
        let plan = DemandPlan {
            id: Uuid::new_v4(),
            plan_name: req.plan_name,
            plan_type: req.plan_type,
            start_date: req.start_date,
            end_date: req.end_date,
            status: PlanStatus::Draft,
            version: 1,
            baseline_id: None,
            approved_by: None,
            approved_at: None,
            created_at: Utc::now(),
        };
        self.repo.create_plan(&plan).await?;
        Ok(plan)
    }

    pub async fn calculate_safety_stock(&self, req: CalculateSafetyStockRequest) -> anyhow::Result<SafetyStock> {
        let stock = SafetyStock {
            id: Uuid::new_v4(),
            product_id: req.product_id,
            warehouse_id: req.warehouse_id,
            safety_qty: 100,
            reorder_point: 200,
            service_level: req.service_level,
            lead_time_days: req.lead_time_days,
            demand_variability: 20.0,
            last_calculated: Utc::now(),
        };
        self.repo.upsert_safety_stock(&stock).await?;
        Ok(stock)
    }

    pub async fn get_forecast_accuracy(&self, product_id: Uuid, period: String) -> anyhow::Result<ForecastAccuracy> {
        Ok(ForecastAccuracy {
            product_id,
            period,
            mape: 12.5,
            mad: 8.3,
            mse: 120.0,
            bias: -2.1,
        })
    }

    pub async fn add_demand_signal(&self, signal_type: SignalType, source: String, value: f64, product_ids: Vec<Uuid>) -> anyhow::Result<DemandSensingSignal> {
        let signal = DemandSensingSignal {
            id: Uuid::new_v4(),
            signal_type,
            source,
            value,
            weight: 1.0,
            timestamp: Utc::now(),
            product_ids,
        };
        self.repo.add_sensing_signal(&signal).await?;
        Ok(signal)
    }
}
