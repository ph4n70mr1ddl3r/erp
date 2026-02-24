use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait RevRecRepository: Send + Sync {
    async fn create_contract(&self, _contract: &RevenueContract) -> anyhow::Result<()> { Ok(()) }
    async fn get_contract(&self, _id: Uuid) -> anyhow::Result<Option<RevenueContract>> { Ok(None) }
    async fn list_contracts(&self, _customer_id: Option<Uuid>, _status: Option<ContractStatus>) -> anyhow::Result<Vec<RevenueContract>> { Ok(vec![]) }
    async fn create_obligation(&self, _obligation: &PerformanceObligation) -> anyhow::Result<()> { Ok(()) }
    async fn get_obligation(&self, _id: Uuid) -> anyhow::Result<Option<PerformanceObligation>> { Ok(None) }
    async fn list_obligations(&self, _contract_id: Uuid) -> anyhow::Result<Vec<PerformanceObligation>> { Ok(vec![]) }
    async fn create_schedule(&self, _schedule: &RevenueSchedule) -> anyhow::Result<()> { Ok(()) }
    async fn list_schedules(&self, _obligation_id: Uuid) -> anyhow::Result<Vec<RevenueSchedule>> { Ok(vec![]) }
    async fn create_event(&self, _event: &RevenueEvent) -> anyhow::Result<()> { Ok(()) }
    async fn list_events(&self, _contract_id: Uuid) -> anyhow::Result<Vec<RevenueEvent>> { Ok(vec![]) }
    async fn create_deferred(&self, _deferred: &DeferredRevenue) -> anyhow::Result<()> { Ok(()) }
    async fn get_deferred(&self, _obligation_id: Uuid) -> anyhow::Result<Option<DeferredRevenue>> { Ok(None) }
    async fn update_deferred(&self, _id: Uuid, _amount: i64) -> anyhow::Result<()> { Ok(()) }
    async fn create_modification(&self, _modification: &ContractModification) -> anyhow::Result<()> { Ok(()) }
    async fn list_modifications(&self, _contract_id: Uuid) -> anyhow::Result<Vec<ContractModification>> { Ok(vec![]) }
    async fn create_allocation_rule(&self, _rule: &AllocationRule) -> anyhow::Result<()> { Ok(()) }
    async fn list_allocation_rules(&self) -> anyhow::Result<Vec<AllocationRule>> { Ok(vec![]) }
}

pub struct SqliteRevRecRepository {
    pub pool: SqlitePool,
}

impl SqliteRevRecRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl RevRecRepository for SqliteRevRecRepository {}
