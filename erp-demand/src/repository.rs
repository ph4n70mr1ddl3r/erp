use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait DemandRepository: Send + Sync {
    async fn create_forecast(&self, _forecast: &DemandForecast) -> anyhow::Result<()> { Ok(()) }
    async fn get_forecasts(&self, _product_id: Uuid, _start: chrono::NaiveDate, _end: chrono::NaiveDate) -> anyhow::Result<Vec<DemandForecast>> { Ok(vec![]) }
    async fn create_model(&self, _model: &ForecastModel) -> anyhow::Result<()> { Ok(()) }
    async fn create_plan(&self, _plan: &DemandPlan) -> anyhow::Result<()> { Ok(()) }
    async fn upsert_safety_stock(&self, _stock: &SafetyStock) -> anyhow::Result<()> { Ok(()) }
    async fn add_sensing_signal(&self, _signal: &DemandSensingSignal) -> anyhow::Result<()> { Ok(()) }
}

pub struct SqliteDemandRepository {
    pub pool: SqlitePool,
}

impl SqliteDemandRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DemandRepository for SqliteDemandRepository {}
