use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::{Result, BaseEntity, Status};
use crate::models::*;
use crate::repository::*;

pub struct SourcingService {
    repo: SqliteSourcingRepository,
}

impl SourcingService {
    pub fn new() -> Self {
        Self { repo: SqliteSourcingRepository }
    }

    pub async fn create_event(&self, pool: &SqlitePool, mut event: SourcingEvent) -> Result<SourcingEvent> {
        event.event_number = format!("RFQ-{}", chrono::Utc::now().format("%Y%m%d%H%M"));
        self.repo.create_event(pool, event).await
    }

    pub async fn get_event(&self, pool: &SqlitePool, id: Uuid) -> Result<SourcingEvent> {
        self.repo.get_event(pool, id).await
    }

    pub async fn list_events(&self, pool: &SqlitePool) -> Result<Vec<SourcingEvent>> {
        self.repo.list_events(pool).await
    }

    pub async fn publish_event(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_event_status(pool, id, SourcingStatus::Published).await
    }

    pub async fn close_event(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_event_status(pool, id, SourcingStatus::Awarded).await
    }

    pub async fn add_item(&self, pool: &SqlitePool, item: SourcingItem) -> Result<SourcingItem> {
        self.repo.create_item(pool, item).await
    }

    pub async fn list_items(&self, pool: &SqlitePool, event_id: Uuid) -> Result<Vec<SourcingItem>> {
        self.repo.list_items(pool, event_id).await
    }

    pub async fn submit_bid(&self, pool: &SqlitePool, bid: Bid) -> Result<Bid> {
        self.repo.create_bid(pool, bid).await
    }

    pub async fn get_bid(&self, pool: &SqlitePool, id: Uuid) -> Result<Bid> {
        self.repo.get_bid(pool, id).await
    }

    pub async fn list_bids(&self, pool: &SqlitePool, event_id: Uuid) -> Result<Vec<Bid>> {
        self.repo.list_bids(pool, event_id).await
    }

    pub async fn accept_bid(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_bid_status(pool, id, BidStatus::Accepted).await
    }

    pub async fn reject_bid(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_bid_status(pool, id, BidStatus::Rejected).await
    }

    pub async fn add_bid_line(&self, pool: &SqlitePool, line: BidLine) -> Result<BidLine> {
        self.repo.create_bid_line(pool, line).await
    }

    pub async fn create_evaluation_criteria(&self, pool: &SqlitePool, criteria: EvaluationCriteria) -> Result<EvaluationCriteria> {
        self.repo.create_evaluation_criteria(pool, criteria).await
    }

    pub async fn evaluate_bid(&self, pool: &SqlitePool, eval: BidEvaluation) -> Result<BidEvaluation> {
        self.repo.create_bid_evaluation(pool, eval).await
    }

    pub async fn award_bid(&self, pool: &SqlitePool, event_id: Uuid, bid_id: Uuid, vendor_id: Uuid, total_value: i64, currency: String) -> Result<SourcingAward> {
        let award = SourcingAward {
            base: BaseEntity::new(),
            event_id,
            bid_id,
            vendor_id,
            item_id: None,
            awarded_quantity: 1,
            awarded_price: total_value,
            total_value,
            currency,
            awarded_at: chrono::Utc::now(),
            award_type: AwardType::Full,
            purchase_order_id: None,
            contract_id: None,
        };
        self.repo.create_award(pool, award).await
    }

    pub async fn create_contract(&self, pool: &SqlitePool, contract: SourcingContract) -> Result<SourcingContract> {
        self.repo.create_contract(pool, contract).await
    }

    pub async fn invite_supplier(&self, pool: &SqlitePool, event_id: Uuid, vendor_id: Uuid) -> Result<SourcingSupplier> {
        let supplier = SourcingSupplier {
            base: BaseEntity::new(),
            event_id,
            vendor_id,
            invited_at: chrono::Utc::now(),
            accepted_at: None,
            declined_at: None,
            status: Status::Active,
            notes: None,
        };
        self.repo.add_supplier(pool, supplier).await
    }
}
