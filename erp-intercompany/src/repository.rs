use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait IntercompanyRepository: Send + Sync {
    async fn create_entity(&self, _entity: &IntercompanyEntity) -> anyhow::Result<()> { Ok(()) }
    async fn get_entity(&self, _id: Uuid) -> anyhow::Result<Option<IntercompanyEntity>> { Ok(None) }
    async fn list_entities(&self, _status: Option<EntityStatus>) -> anyhow::Result<Vec<IntercompanyEntity>> { Ok(vec![]) }
    async fn create_transaction(&self, _txn: &IntercompanyTransaction) -> anyhow::Result<()> { Ok(()) }
    async fn get_transaction(&self, _id: Uuid) -> anyhow::Result<Option<IntercompanyTransaction>> { Ok(None) }
    async fn list_transactions(&self, _source: Option<Uuid>, _target: Option<Uuid>) -> anyhow::Result<Vec<IntercompanyTransaction>> { Ok(vec![]) }
    async fn update_transaction_status(&self, _id: Uuid, _status: ICTransactionStatus) -> anyhow::Result<()> { Ok(()) }
    async fn create_transfer_price(&self, _price: &TransferPrice) -> anyhow::Result<()> { Ok(()) }
    async fn get_transfer_price(&self, _product_id: Uuid, _source: Uuid, _target: Uuid) -> anyhow::Result<Option<TransferPrice>> { Ok(None) }
    async fn list_transfer_prices(&self) -> anyhow::Result<Vec<TransferPrice>> { Ok(vec![]) }
    async fn create_due_to_from(&self, _dtf: &DueToFrom) -> anyhow::Result<()> { Ok(()) }
    async fn get_due_to_from(&self, _source: Uuid, _target: Uuid) -> anyhow::Result<Option<DueToFrom>> { Ok(None) }
    async fn create_consolidation(&self, _consolidation: &Consolidation) -> anyhow::Result<()> { Ok(()) }
    async fn get_consolidation(&self, _id: Uuid) -> anyhow::Result<Option<Consolidation>> { Ok(None) }
    async fn update_consolidation_status(&self, _id: Uuid, _status: ConsolidationStatus) -> anyhow::Result<()> { Ok(()) }
    async fn create_elimination(&self, _entry: &EliminationEntry) -> anyhow::Result<()> { Ok(()) }
    async fn list_eliminations(&self, _consolidation_id: Uuid) -> anyhow::Result<Vec<EliminationEntry>> { Ok(vec![]) }
    async fn create_agreement(&self, _agreement: &IntercompanyAgreement) -> anyhow::Result<()> { Ok(()) }
    async fn get_agreement(&self, _id: Uuid) -> anyhow::Result<Option<IntercompanyAgreement>> { Ok(None) }
    async fn list_agreements(&self, _source: Option<Uuid>) -> anyhow::Result<Vec<IntercompanyAgreement>> { Ok(vec![]) }
}

pub struct SqliteIntercompanyRepository {
    pub pool: SqlitePool,
}

impl SqliteIntercompanyRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IntercompanyRepository for SqliteIntercompanyRepository {}
