use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait EdiRepository: Send + Sync {
    async fn create_partner(&self, _partner: &EdiPartner) -> anyhow::Result<()> { Ok(()) }
    async fn get_partner(&self, _id: Uuid) -> anyhow::Result<Option<EdiPartner>> { Ok(None) }
    async fn get_partner_by_code(&self, _code: &str) -> anyhow::Result<Option<EdiPartner>> { Ok(None) }
    async fn list_partners(&self, _partner_type: Option<PartnerType>) -> anyhow::Result<Vec<EdiPartner>> { Ok(vec![]) }
    async fn create_transaction(&self, _txn: &EdiTransaction) -> anyhow::Result<()> { Ok(()) }
    async fn get_transaction(&self, _id: Uuid) -> anyhow::Result<Option<EdiTransaction>> { Ok(None) }
    async fn list_transactions(&self, _partner_id: Option<Uuid>, _txn_type: Option<EdiTransactionType>) -> anyhow::Result<Vec<EdiTransaction>> { Ok(vec![]) }
    async fn update_transaction_status(&self, _id: Uuid, _status: EdiStatus, _parsed_data: Option<serde_json::Value>) -> anyhow::Result<()> { Ok(()) }
    async fn create_mapping(&self, _mapping: &EdiMapping) -> anyhow::Result<()> { Ok(()) }
    async fn get_mapping(&self, _txn_type: EdiTransactionType) -> anyhow::Result<Option<EdiMapping>> { Ok(None) }
    async fn create_acknowledgment(&self, _ack: &EdiAcknowledgment) -> anyhow::Result<()> { Ok(()) }
    async fn get_acknowledgment(&self, _txn_id: Uuid) -> anyhow::Result<Option<EdiAcknowledgment>> { Ok(None) }
    async fn create_850(&self, _order: &Edi850PurchaseOrder) -> anyhow::Result<()> { Ok(()) }
    async fn get_850(&self, _id: Uuid) -> anyhow::Result<Option<Edi850PurchaseOrder>> { Ok(None) }
    async fn create_810(&self, _invoice: &Edi810Invoice) -> anyhow::Result<()> { Ok(()) }
    async fn get_810(&self, _id: Uuid) -> anyhow::Result<Option<Edi810Invoice>> { Ok(None) }
    async fn create_856(&self, _asn: &Edi856ASN) -> anyhow::Result<()> { Ok(()) }
    async fn get_856(&self, _id: Uuid) -> anyhow::Result<Option<Edi856ASN>> { Ok(None) }
}

pub struct SqliteEdiRepository {
    pub pool: SqlitePool,
}

impl SqliteEdiRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl EdiRepository for SqliteEdiRepository {}
