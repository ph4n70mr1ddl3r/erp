use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::Result;
use crate::models::*;

#[async_trait]
pub trait SourcingRepository: Send + Sync {
    async fn create_event(&self, pool: &SqlitePool, event: SourcingEvent) -> Result<SourcingEvent>;
    async fn get_event(&self, pool: &SqlitePool, id: Uuid) -> Result<SourcingEvent>;
    async fn list_events(&self, pool: &SqlitePool) -> Result<Vec<SourcingEvent>>;
    async fn update_event_status(&self, pool: &SqlitePool, id: Uuid, status: SourcingStatus) -> Result<()>;
    async fn create_item(&self, pool: &SqlitePool, item: SourcingItem) -> Result<SourcingItem>;
    async fn list_items(&self, pool: &SqlitePool, event_id: Uuid) -> Result<Vec<SourcingItem>>;
    async fn create_bid(&self, pool: &SqlitePool, bid: Bid) -> Result<Bid>;
    async fn get_bid(&self, pool: &SqlitePool, id: Uuid) -> Result<Bid>;
    async fn list_bids(&self, pool: &SqlitePool, event_id: Uuid) -> Result<Vec<Bid>>;
    async fn update_bid_status(&self, pool: &SqlitePool, id: Uuid, status: BidStatus) -> Result<()>;
    async fn create_bid_line(&self, pool: &SqlitePool, line: BidLine) -> Result<BidLine>;
    async fn create_evaluation_criteria(&self, pool: &SqlitePool, criteria: EvaluationCriteria) -> Result<EvaluationCriteria>;
    async fn create_bid_evaluation(&self, pool: &SqlitePool, eval: BidEvaluation) -> Result<BidEvaluation>;
    async fn create_award(&self, pool: &SqlitePool, award: SourcingAward) -> Result<SourcingAward>;
    async fn create_contract(&self, pool: &SqlitePool, contract: SourcingContract) -> Result<SourcingContract>;
    async fn add_supplier(&self, pool: &SqlitePool, supplier: SourcingSupplier) -> Result<SourcingSupplier>;
}

pub struct SqliteSourcingRepository;

#[async_trait]
impl SourcingRepository for SqliteSourcingRepository {
    async fn create_event(&self, pool: &SqlitePool, event: SourcingEvent) -> Result<SourcingEvent> {
        sqlx::query!(
            r#"INSERT INTO sourcing_events (id, event_number, title, description, event_type, status,
               auction_type, start_date, end_date, currency, estimated_value, budget, requirements,
               evaluation_criteria, terms_conditions, buyer_id, category_id, is_public, allow_reverse_auction,
               min_bid_decrement, auto_extend, extension_minutes, created_by, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            event.base.id.to_string(),
            event.event_number,
            event.title,
            event.description,
            format!("{:?}", event.event_type),
            format!("{:?}", event.status),
            event.auction_type.map(|t| format!("{:?}", t)),
            event.start_date.to_rfc3339(),
            event.end_date.to_rfc3339(),
            event.currency,
            event.estimated_value,
            event.budget,
            event.requirements,
            event.evaluation_criteria,
            event.terms_conditions,
            event.buyer_id.map(|id| id.to_string()),
            event.category_id.map(|id| id.to_string()),
            event.is_public as i32,
            event.allow_reverse_auction as i32,
            event.min_bid_decrement,
            event.auto_extend as i32,
            event.extension_minutes,
            event.created_by.map(|id| id.to_string()),
            event.base.created_at.to_rfc3339(),
            event.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(event)
    }

    async fn get_event(&self, pool: &SqlitePool, id: Uuid) -> Result<SourcingEvent> {
        let row = sqlx::query!(
            r#"SELECT id, event_number, title, description, event_type, status, auction_type, start_date,
               end_date, currency, estimated_value, budget, requirements, evaluation_criteria,
               terms_conditions, buyer_id, category_id, is_public, allow_reverse_auction,
               min_bid_decrement, auto_extend, extension_minutes, created_by, created_at, updated_at
               FROM sourcing_events WHERE id = ?"#,
            id.to_string()
        ).fetch_one(pool).await?;
        
        Ok(SourcingEvent {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: None,
            },
            event_number: row.event_number,
            title: row.title,
            description: row.description,
            event_type: SourcingEventType::RFQ,
            status: SourcingStatus::Draft,
            auction_type: None,
            start_date: chrono::DateTime::parse_from_rfc3339(&row.start_date).unwrap().with_timezone(&chrono::Utc),
            end_date: chrono::DateTime::parse_from_rfc3339(&row.end_date).unwrap().with_timezone(&chrono::Utc),
            currency: row.currency,
            estimated_value: row.estimated_value,
            budget: row.budget,
            requirements: row.requirements,
            evaluation_criteria: row.evaluation_criteria,
            terms_conditions: row.terms_conditions,
            buyer_id: row.buyer_id.and_then(|s| Uuid::parse_str(&s).ok()),
            category_id: row.category_id.and_then(|s| Uuid::parse_str(&s).ok()),
            is_public: row.is_public == 1,
            allow_reverse_auction: row.allow_reverse_auction == 1,
            min_bid_decrement: row.min_bid_decrement,
            auto_extend: row.auto_extend == 1,
            extension_minutes: row.extension_minutes,
            created_by: None,
        })
    }

    async fn list_events(&self, pool: &SqlitePool) -> Result<Vec<SourcingEvent>> {
        let rows = sqlx::query!(
            r#"SELECT id, event_number, title, description, event_type, status, auction_type, start_date,
               end_date, currency, estimated_value, budget, requirements, evaluation_criteria,
               terms_conditions, buyer_id, category_id, is_public, allow_reverse_auction,
               min_bid_decrement, auto_extend, extension_minutes, created_by, created_at, updated_at
               FROM sourcing_events ORDER BY created_at DESC"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| SourcingEvent {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: None,
            },
            event_number: row.event_number,
            title: row.title,
            description: row.description,
            event_type: SourcingEventType::RFQ,
            status: SourcingStatus::Draft,
            auction_type: None,
            start_date: chrono::DateTime::parse_from_rfc3339(&row.start_date).unwrap().with_timezone(&chrono::Utc),
            end_date: chrono::DateTime::parse_from_rfc3339(&row.end_date).unwrap().with_timezone(&chrono::Utc),
            currency: row.currency,
            estimated_value: row.estimated_value,
            budget: row.budget,
            requirements: row.requirements,
            evaluation_criteria: row.evaluation_criteria,
            terms_conditions: row.terms_conditions,
            buyer_id: row.buyer_id.and_then(|s| Uuid::parse_str(&s).ok()),
            category_id: row.category_id.and_then(|s| Uuid::parse_str(&s).ok()),
            is_public: row.is_public == 1,
            allow_reverse_auction: row.allow_reverse_auction == 1,
            min_bid_decrement: row.min_bid_decrement,
            auto_extend: row.auto_extend == 1,
            extension_minutes: row.extension_minutes,
            created_by: None,
        }).collect())
    }

    async fn update_event_status(&self, pool: &SqlitePool, id: Uuid, status: SourcingStatus) -> Result<()> {
        sqlx::query!(
            r#"UPDATE sourcing_events SET status = ?, updated_at = ? WHERE id = ?"#,
            format!("{:?}", status),
            chrono::Utc::now().to_rfc3339(),
            id.to_string()
        ).execute(pool).await?;
        Ok(())
    }

    async fn create_item(&self, pool: &SqlitePool, item: SourcingItem) -> Result<SourcingItem> {
        sqlx::query!(
            r#"INSERT INTO sourcing_items (id, event_id, product_id, sku, name, description, quantity,
               unit_of_measure, target_price, max_price, specifications, delivery_date, delivery_location,
               sort_order, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            item.base.id.to_string(),
            item.event_id.to_string(),
            item.product_id.map(|id| id.to_string()),
            item.sku,
            item.name,
            item.description,
            item.quantity,
            item.unit_of_measure,
            item.target_price,
            item.max_price,
            item.specifications,
            item.delivery_date.map(|d| d.to_rfc3339()),
            item.delivery_location,
            item.sort_order,
            item.base.created_at.to_rfc3339(),
            item.base.updated_at.to_rfc3339(),
            item.base.created_by.map(|id| id.to_string()),
            item.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(item)
    }

    async fn list_items(&self, pool: &SqlitePool, event_id: Uuid) -> Result<Vec<SourcingItem>> {
        let rows = sqlx::query!(
            r#"SELECT id, event_id, product_id, sku, name, description, quantity, unit_of_measure,
               target_price, max_price, specifications, delivery_date, delivery_location, sort_order,
               created_at, updated_at, created_by, updated_by
               FROM sourcing_items WHERE event_id = ? ORDER BY sort_order"#,
            event_id.to_string()
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| SourcingItem {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            event_id: Uuid::parse_str(&row.event_id).unwrap(),
            product_id: row.product_id.and_then(|s| Uuid::parse_str(&s).ok()),
            sku: row.sku,
            name: row.name,
            description: row.description,
            quantity: row.quantity,
            unit_of_measure: row.unit_of_measure,
            target_price: row.target_price,
            max_price: row.max_price,
            specifications: row.specifications,
            delivery_date: row.delivery_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            delivery_location: row.delivery_location,
            sort_order: row.sort_order,
        }).collect())
    }

    async fn create_bid(&self, pool: &SqlitePool, bid: Bid) -> Result<Bid> {
        sqlx::query!(
            r#"INSERT INTO sourcing_bids (id, event_id, vendor_id, bid_number, status, submitted_at,
               valid_until, total_amount, currency, terms, notes, rank, score, is_winner,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            bid.base.id.to_string(),
            bid.event_id.to_string(),
            bid.vendor_id.to_string(),
            bid.bid_number,
            format!("{:?}", bid.status),
            bid.submitted_at.map(|d| d.to_rfc3339()),
            bid.valid_until.map(|d| d.to_rfc3339()),
            bid.total_amount,
            bid.currency,
            bid.terms,
            bid.notes,
            bid.rank,
            bid.score,
            bid.is_winner as i32,
            bid.base.created_at.to_rfc3339(),
            bid.base.updated_at.to_rfc3339(),
            bid.base.created_by.map(|id| id.to_string()),
            bid.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(bid)
    }

    async fn get_bid(&self, pool: &SqlitePool, id: Uuid) -> Result<Bid> {
        let row = sqlx::query!(
            r#"SELECT id, event_id, vendor_id, bid_number, status, submitted_at, valid_until,
               total_amount, currency, terms, notes, rank, score, is_winner,
               created_at, updated_at, created_by, updated_by
               FROM sourcing_bids WHERE id = ?"#,
            id.to_string()
        ).fetch_one(pool).await?;
        
        Ok(Bid {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            event_id: Uuid::parse_str(&row.event_id).unwrap(),
            vendor_id: Uuid::parse_str(&row.vendor_id).unwrap(),
            bid_number: row.bid_number,
            status: BidStatus::Draft,
            submitted_at: row.submitted_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_until: row.valid_until.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            total_amount: row.total_amount,
            currency: row.currency,
            terms: row.terms,
            notes: row.notes,
            rank: row.rank,
            score: row.score,
            is_winner: row.is_winner == 1,
        })
    }

    async fn list_bids(&self, pool: &SqlitePool, event_id: Uuid) -> Result<Vec<Bid>> {
        let rows = sqlx::query!(
            r#"SELECT id, event_id, vendor_id, bid_number, status, submitted_at, valid_until,
               total_amount, currency, terms, notes, rank, score, is_winner,
               created_at, updated_at, created_by, updated_by
               FROM sourcing_bids WHERE event_id = ? ORDER BY total_amount"#,
            event_id.to_string()
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| Bid {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            event_id: Uuid::parse_str(&row.event_id).unwrap(),
            vendor_id: Uuid::parse_str(&row.vendor_id).unwrap(),
            bid_number: row.bid_number,
            status: BidStatus::Draft,
            submitted_at: row.submitted_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_until: row.valid_until.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            total_amount: row.total_amount,
            currency: row.currency,
            terms: row.terms,
            notes: row.notes,
            rank: row.rank,
            score: row.score,
            is_winner: row.is_winner == 1,
        }).collect())
    }

    async fn update_bid_status(&self, pool: &SqlitePool, id: Uuid, status: BidStatus) -> Result<()> {
        sqlx::query!(
            r#"UPDATE sourcing_bids SET status = ?, updated_at = ? WHERE id = ?"#,
            format!("{:?}", status),
            chrono::Utc::now().to_rfc3339(),
            id.to_string()
        ).execute(pool).await?;
        Ok(())
    }

    async fn create_bid_line(&self, pool: &SqlitePool, line: BidLine) -> Result<BidLine> {
        sqlx::query!(
            r#"INSERT INTO sourcing_bid_lines (id, bid_id, item_id, unit_price, quantity, total_price,
               delivery_date, lead_time_days, specifications_met, notes, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            line.base.id.to_string(),
            line.bid_id.to_string(),
            line.item_id.to_string(),
            line.unit_price,
            line.quantity,
            line.total_price,
            line.delivery_date.map(|d| d.to_rfc3339()),
            line.lead_time_days,
            line.specifications_met as i32,
            line.notes,
            line.base.created_at.to_rfc3339(),
            line.base.updated_at.to_rfc3339(),
            line.base.created_by.map(|id| id.to_string()),
            line.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(line)
    }

    async fn create_evaluation_criteria(&self, pool: &SqlitePool, criteria: EvaluationCriteria) -> Result<EvaluationCriteria> {
        sqlx::query!(
            r#"INSERT INTO sourcing_evaluation_criteria (id, event_id, name, description, weight,
               max_score, evaluation_method, sort_order, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            criteria.base.id.to_string(),
            criteria.event_id.to_string(),
            criteria.name,
            criteria.description,
            criteria.weight,
            criteria.max_score,
            format!("{:?}", criteria.evaluation_method),
            criteria.sort_order,
            criteria.base.created_at.to_rfc3339(),
            criteria.base.updated_at.to_rfc3339(),
            criteria.base.created_by.map(|id| id.to_string()),
            criteria.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(criteria)
    }

    async fn create_bid_evaluation(&self, pool: &SqlitePool, eval: BidEvaluation) -> Result<BidEvaluation> {
        sqlx::query!(
            r#"INSERT INTO sourcing_bid_evaluations (id, bid_id, criteria_id, score, comments,
               evaluated_by, evaluated_at, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            eval.base.id.to_string(),
            eval.bid_id.to_string(),
            eval.criteria_id.to_string(),
            eval.score,
            eval.comments,
            eval.evaluated_by.map(|id| id.to_string()),
            eval.evaluated_at.to_rfc3339(),
            eval.base.created_at.to_rfc3339(),
            eval.base.updated_at.to_rfc3339(),
            eval.base.created_by.map(|id| id.to_string()),
            eval.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(eval)
    }

    async fn create_award(&self, pool: &SqlitePool, award: SourcingAward) -> Result<SourcingAward> {
        sqlx::query!(
            r#"INSERT INTO sourcing_awards (id, event_id, bid_id, vendor_id, item_id, awarded_quantity,
               awarded_price, total_value, currency, awarded_at, award_type, purchase_order_id,
               contract_id, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            award.base.id.to_string(),
            award.event_id.to_string(),
            award.bid_id.to_string(),
            award.vendor_id.to_string(),
            award.item_id.map(|id| id.to_string()),
            award.awarded_quantity,
            award.awarded_price,
            award.total_value,
            award.currency,
            award.awarded_at.to_rfc3339(),
            format!("{:?}", award.award_type),
            award.purchase_order_id.map(|id| id.to_string()),
            award.contract_id.map(|id| id.to_string()),
            award.base.created_at.to_rfc3339(),
            award.base.updated_at.to_rfc3339(),
            award.base.created_by.map(|id| id.to_string()),
            award.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(award)
    }

    async fn create_contract(&self, pool: &SqlitePool, contract: SourcingContract) -> Result<SourcingContract> {
        sqlx::query!(
            r#"INSERT INTO sourcing_contracts (id, event_id, vendor_id, contract_number, title,
               description, start_date, end_date, total_value, currency, terms, status,
               renewal_type, auto_renew, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            contract.base.id.to_string(),
            contract.event_id.map(|id| id.to_string()),
            contract.vendor_id.to_string(),
            contract.contract_number,
            contract.title,
            contract.description,
            contract.start_date.to_rfc3339(),
            contract.end_date.to_rfc3339(),
            contract.total_value,
            contract.currency,
            contract.terms,
            format!("{:?}", contract.status),
            contract.renewal_type.map(|t| format!("{:?}", t)),
            contract.auto_renew as i32,
            contract.base.created_at.to_rfc3339(),
            contract.base.updated_at.to_rfc3339(),
            contract.base.created_by.map(|id| id.to_string()),
            contract.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(contract)
    }

    async fn add_supplier(&self, pool: &SqlitePool, supplier: SourcingSupplier) -> Result<SourcingSupplier> {
        sqlx::query!(
            r#"INSERT INTO sourcing_suppliers (id, event_id, vendor_id, invited_at, accepted_at,
               declined_at, status, notes, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            supplier.base.id.to_string(),
            supplier.event_id.to_string(),
            supplier.vendor_id.to_string(),
            supplier.invited_at.to_rfc3339(),
            supplier.accepted_at.map(|d| d.to_rfc3339()),
            supplier.declined_at.map(|d| d.to_rfc3339()),
            format!("{:?}", supplier.status),
            supplier.notes,
            supplier.base.created_at.to_rfc3339(),
            supplier.base.updated_at.to_rfc3339(),
            supplier.base.created_by.map(|id| id.to_string()),
            supplier.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(supplier)
    }
}
