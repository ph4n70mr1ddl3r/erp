use async_trait::async_trait;
use erp_core::error::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait CostPoolRepository: Send + Sync {
    async fn create(&self, pool: &CostPool) -> Result<CostPool>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CostPool>>;
    async fn find_all(&self) -> Result<Vec<CostPool>>;
    async fn find_by_type(&self, pool_type: CostPoolType) -> Result<Vec<CostPool>>;
    async fn update(&self, pool: &CostPool) -> Result<CostPool>;
}

#[async_trait]
pub trait ActivityRepository: Send + Sync {
    async fn create(&self, activity: &Activity) -> Result<Activity>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Activity>>;
    async fn find_by_cost_pool(&self, cost_pool_id: Uuid) -> Result<Vec<Activity>>;
    async fn find_by_type(&self, activity_type: ActivityType) -> Result<Vec<Activity>>;
    async fn update(&self, activity: &Activity) -> Result<Activity>;
}

#[async_trait]
pub trait CostDriverRepository: Send + Sync {
    async fn create(&self, driver: &CostDriver) -> Result<CostDriver>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CostDriver>>;
    async fn find_all(&self) -> Result<Vec<CostDriver>>;
    async fn update(&self, driver: &CostDriver) -> Result<CostDriver>;
}

#[async_trait]
pub trait CostObjectRepository: Send + Sync {
    async fn create(&self, object: &CostObject) -> Result<CostObject>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<CostObject>>;
    async fn find_by_type(&self, object_type: CostObjectType) -> Result<Vec<CostObject>>;
    async fn update(&self, object: &CostObject) -> Result<CostObject>;
}

#[async_trait]
pub trait ActivityAllocationRepository: Send + Sync {
    async fn create(&self, allocation: &ActivityAllocation) -> Result<ActivityAllocation>;
    async fn find_by_activity(&self, activity_id: Uuid) -> Result<Vec<ActivityAllocation>>;
    async fn find_by_cost_object(&self, cost_object_id: Uuid) -> Result<Vec<ActivityAllocation>>;
    async fn get_summary(&self, cost_object_id: Uuid) -> Result<AllocationSummary>;
}

pub struct AllocationSummary {
    pub cost_object_id: Uuid,
    pub total_allocated: i64,
    pub activity_count: i32,
    pub top_activities: Vec<ActivityAllocationSummary>,
}

pub struct ActivityAllocationSummary {
    pub activity_id: Uuid,
    pub activity_name: String,
    pub allocated_amount: i64,
    pub percentage: f64,
}

pub struct SqliteCostPoolRepository {
    pool: SqlitePool,
}

impl SqliteCostPoolRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CostPoolRepository for SqliteCostPoolRepository {
    async fn create(&self, cost_pool: &CostPool) -> Result<CostPool> {
        Ok(cost_pool.clone())
    }

    async fn find_by_id(&self, _id: Uuid) -> Result<Option<CostPool>> {
        Ok(None)
    }

    async fn find_all(&self) -> Result<Vec<CostPool>> {
        Ok(Vec::new())
    }

    async fn find_by_type(&self, _pool_type: CostPoolType) -> Result<Vec<CostPool>> {
        Ok(Vec::new())
    }

    async fn update(&self, pool: &CostPool) -> Result<CostPool> {
        Ok(pool.clone())
    }
}
