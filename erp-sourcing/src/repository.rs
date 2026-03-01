use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
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
        sqlx::query(
            r#"INSERT INTO sourcing_events (id, event_number, title, description, event_type, status,
               auction_type, start_date, end_date, currency, estimated_value, budget, requirements,
               evaluation_criteria, terms_conditions, buyer_id, category_id, is_public, allow_reverse_auction,
               min_bid_decrement, auto_extend, extension_minutes, created_by, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(event.base.id.to_string())
        .bind(&event.event_number)
        .bind(&event.title)
        .bind(&event.description)
        .bind(format!("{:?}", event.event_type))
        .bind(format!("{:?}", event.status))
        .bind(event.auction_type.clone().map(|t| format!("{:?}", t)))
        .bind(event.start_date.to_rfc3339())
        .bind(event.end_date.to_rfc3339())
        .bind(&event.currency)
        .bind(event.estimated_value)
        .bind(event.budget)
        .bind(&event.requirements)
        .bind(&event.evaluation_criteria)
        .bind(&event.terms_conditions)
        .bind(event.buyer_id.map(|id| id.to_string()))
        .bind(event.category_id.map(|id| id.to_string()))
        .bind(event.is_public as i32)
        .bind(event.allow_reverse_auction as i32)
        .bind(event.min_bid_decrement)
        .bind(event.auto_extend as i32)
        .bind(event.extension_minutes)
        .bind(event.created_by.map(|id| id.to_string()))
        .bind(event.base.created_at.to_rfc3339())
        .bind(event.base.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(event)
    }

    async fn get_event(&self, pool: &SqlitePool, id: Uuid) -> Result<SourcingEvent> {
        let row = sqlx::query(
            r#"SELECT id, event_number, title, description, event_type, status, auction_type, start_date,
               end_date, currency, estimated_value, budget, requirements, evaluation_criteria,
               terms_conditions, buyer_id, category_id, is_public, allow_reverse_auction,
               min_bid_decrement, auto_extend, extension_minutes, created_by, created_at, updated_at
               FROM sourcing_events WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_one(pool).await?;
        
        Ok(SourcingEvent {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: None,
            },
            event_number: row.get::<&str, _>("event_number").to_string(),
            title: row.get::<&str, _>("title").to_string(),
            description: row.get::<Option<&str>, _>("description").map(|s| s.to_string()),
            event_type: SourcingEventType::RFQ,
            status: SourcingStatus::Draft,
            auction_type: None,
            start_date: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("start_date")).unwrap().with_timezone(&chrono::Utc),
            end_date: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("end_date")).unwrap().with_timezone(&chrono::Utc),
            currency: row.get::<&str, _>("currency").to_string(),
            estimated_value: row.get::<i64, _>("estimated_value"),
            budget: row.get::<Option<i64>, _>("budget"),
            requirements: row.get::<Option<&str>, _>("requirements").map(|s| s.to_string()),
            evaluation_criteria: row.get::<Option<&str>, _>("evaluation_criteria").map(|s| s.to_string()),
            terms_conditions: row.get::<Option<&str>, _>("terms_conditions").map(|s| s.to_string()),
            buyer_id: row.get::<Option<&str>, _>("buyer_id").and_then(|s| Uuid::parse_str(s).ok()),
            category_id: row.get::<Option<&str>, _>("category_id").and_then(|s| Uuid::parse_str(s).ok()),
            is_public: row.get::<i32, _>("is_public") == 1,
            allow_reverse_auction: row.get::<i32, _>("allow_reverse_auction") == 1,
            min_bid_decrement: row.get::<Option<i64>, _>("min_bid_decrement"),
            auto_extend: row.get::<i32, _>("auto_extend") == 1,
            extension_minutes: row.get::<Option<i32>, _>("extension_minutes"),
            created_by: None,
        })
    }

    async fn list_events(&self, pool: &SqlitePool) -> Result<Vec<SourcingEvent>> {
        let rows = sqlx::query(
            r#"SELECT id, event_number, title, description, event_type, status, auction_type, start_date,
               end_date, currency, estimated_value, budget, requirements, evaluation_criteria,
               terms_conditions, buyer_id, category_id, is_public, allow_reverse_auction,
               min_bid_decrement, auto_extend, extension_minutes, created_by, created_at, updated_at
               FROM sourcing_events ORDER BY created_at DESC"#,
        )
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| SourcingEvent {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: None,
            },
            event_number: row.get::<&str, _>("event_number").to_string(),
            title: row.get::<&str, _>("title").to_string(),
            description: row.get::<Option<&str>, _>("description").map(|s| s.to_string()),
            event_type: SourcingEventType::RFQ,
            status: SourcingStatus::Draft,
            auction_type: None,
            start_date: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("start_date")).unwrap().with_timezone(&chrono::Utc),
            end_date: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("end_date")).unwrap().with_timezone(&chrono::Utc),
            currency: row.get::<&str, _>("currency").to_string(),
            estimated_value: row.get::<i64, _>("estimated_value"),
            budget: row.get::<Option<i64>, _>("budget"),
            requirements: row.get::<Option<&str>, _>("requirements").map(|s| s.to_string()),
            evaluation_criteria: row.get::<Option<&str>, _>("evaluation_criteria").map(|s| s.to_string()),
            terms_conditions: row.get::<Option<&str>, _>("terms_conditions").map(|s| s.to_string()),
            buyer_id: row.get::<Option<&str>, _>("buyer_id").and_then(|s| Uuid::parse_str(s).ok()),
            category_id: row.get::<Option<&str>, _>("category_id").and_then(|s| Uuid::parse_str(s).ok()),
            is_public: row.get::<i32, _>("is_public") == 1,
            allow_reverse_auction: row.get::<i32, _>("allow_reverse_auction") == 1,
            min_bid_decrement: row.get::<Option<i64>, _>("min_bid_decrement"),
            auto_extend: row.get::<i32, _>("auto_extend") == 1,
            extension_minutes: row.get::<Option<i32>, _>("extension_minutes"),
            created_by: None,
        }).collect())
    }

    async fn update_event_status(&self, pool: &SqlitePool, id: Uuid, status: SourcingStatus) -> Result<()> {
        sqlx::query(
            r#"UPDATE sourcing_events SET status = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(format!("{:?}", status))
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(id.to_string())
        .execute(pool).await?;
        Ok(())
    }

    async fn create_item(&self, pool: &SqlitePool, item: SourcingItem) -> Result<SourcingItem> {
        sqlx::query(
            r#"INSERT INTO sourcing_items (id, event_id, product_id, sku, name, description, quantity,
               unit_of_measure, target_price, max_price, specifications, delivery_date, delivery_location,
               sort_order, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(item.base.id.to_string())
        .bind(item.event_id.to_string())
        .bind(item.product_id.map(|id| id.to_string()))
        .bind(&item.sku)
        .bind(&item.name)
        .bind(&item.description)
        .bind(item.quantity)
        .bind(&item.unit_of_measure)
        .bind(item.target_price)
        .bind(item.max_price)
        .bind(&item.specifications)
        .bind(item.delivery_date.map(|d| d.to_rfc3339()))
        .bind(&item.delivery_location)
        .bind(item.sort_order)
        .bind(item.base.created_at.to_rfc3339())
        .bind(item.base.updated_at.to_rfc3339())
        .bind(item.base.created_by.map(|id| id.to_string()))
        .bind(item.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(item)
    }

    async fn list_items(&self, pool: &SqlitePool, event_id: Uuid) -> Result<Vec<SourcingItem>> {
        let rows = sqlx::query(
            r#"SELECT id, event_id, product_id, sku, name, description, quantity, unit_of_measure,
               target_price, max_price, specifications, delivery_date, delivery_location, sort_order,
               created_at, updated_at, created_by, updated_by
               FROM sourcing_items WHERE event_id = ? ORDER BY sort_order"#,
        )
        .bind(event_id.to_string())
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| SourcingItem {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.get::<Option<&str>, _>("updated_by").and_then(|s| Uuid::parse_str(s).ok()),
            },
            event_id: Uuid::parse_str(row.get::<&str, _>("event_id")).unwrap(),
            product_id: row.get::<Option<&str>, _>("product_id").and_then(|s| Uuid::parse_str(s).ok()),
            sku: row.get::<Option<&str>, _>("sku").map(|s| s.to_string()),
            name: row.get::<&str, _>("name").to_string(),
            description: row.get::<Option<&str>, _>("description").map(|s| s.to_string()),
            quantity: row.get::<i32, _>("quantity"),
            unit_of_measure: row.get::<&str, _>("unit_of_measure").to_string(),
            target_price: row.get::<Option<i64>, _>("target_price"),
            max_price: row.get::<Option<i64>, _>("max_price"),
            specifications: row.get::<Option<&str>, _>("specifications").map(|s| s.to_string()),
            delivery_date: row.get::<Option<&str>, _>("delivery_date").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            delivery_location: row.get::<Option<&str>, _>("delivery_location").map(|s| s.to_string()),
            sort_order: row.get::<i32, _>("sort_order"),
        }).collect())
    }

    async fn create_bid(&self, pool: &SqlitePool, bid: Bid) -> Result<Bid> {
        sqlx::query(
            r#"INSERT INTO sourcing_bids (id, event_id, vendor_id, bid_number, status, submitted_at,
               valid_until, total_amount, currency, terms, notes, rank, score, is_winner,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(bid.base.id.to_string())
        .bind(bid.event_id.to_string())
        .bind(bid.vendor_id.to_string())
        .bind(&bid.bid_number)
        .bind(format!("{:?}", bid.status))
        .bind(bid.submitted_at.map(|d| d.to_rfc3339()))
        .bind(bid.valid_until.map(|d| d.to_rfc3339()))
        .bind(bid.total_amount)
        .bind(&bid.currency)
        .bind(&bid.terms)
        .bind(&bid.notes)
        .bind(bid.rank)
        .bind(bid.score)
        .bind(bid.is_winner as i32)
        .bind(bid.base.created_at.to_rfc3339())
        .bind(bid.base.updated_at.to_rfc3339())
        .bind(bid.base.created_by.map(|id| id.to_string()))
        .bind(bid.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(bid)
    }

    async fn get_bid(&self, pool: &SqlitePool, id: Uuid) -> Result<Bid> {
        let row = sqlx::query(
            r#"SELECT id, event_id, vendor_id, bid_number, status, submitted_at, valid_until,
               total_amount, currency, terms, notes, rank, score, is_winner,
               created_at, updated_at, created_by, updated_by
               FROM sourcing_bids WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_one(pool).await?;
        
        Ok(Bid {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.get::<Option<&str>, _>("updated_by").and_then(|s| Uuid::parse_str(s).ok()),
            },
            event_id: Uuid::parse_str(row.get::<&str, _>("event_id")).unwrap(),
            vendor_id: Uuid::parse_str(row.get::<&str, _>("vendor_id")).unwrap(),
            bid_number: row.get::<&str, _>("bid_number").to_string(),
            status: BidStatus::Draft,
            submitted_at: row.get::<Option<&str>, _>("submitted_at").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_until: row.get::<Option<&str>, _>("valid_until").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            total_amount: row.get::<i64, _>("total_amount"),
            currency: row.get::<&str, _>("currency").to_string(),
            terms: row.get::<Option<&str>, _>("terms").map(|s| s.to_string()),
            notes: row.get::<Option<&str>, _>("notes").map(|s| s.to_string()),
            rank: row.get::<Option<i32>, _>("rank"),
            score: row.get::<Option<f64>, _>("score"),
            is_winner: row.get::<i32, _>("is_winner") == 1,
        })
    }

    async fn list_bids(&self, pool: &SqlitePool, event_id: Uuid) -> Result<Vec<Bid>> {
        let rows = sqlx::query(
            r#"SELECT id, event_id, vendor_id, bid_number, status, submitted_at, valid_until,
               total_amount, currency, terms, notes, rank, score, is_winner,
               created_at, updated_at, created_by, updated_by
               FROM sourcing_bids WHERE event_id = ? ORDER BY total_amount"#,
        )
        .bind(event_id.to_string())
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|row| Bid {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(row.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(row.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: row.get::<Option<&str>, _>("created_by").and_then(|s| Uuid::parse_str(s).ok()),
                updated_by: row.get::<Option<&str>, _>("updated_by").and_then(|s| Uuid::parse_str(s).ok()),
            },
            event_id: Uuid::parse_str(row.get::<&str, _>("event_id")).unwrap(),
            vendor_id: Uuid::parse_str(row.get::<&str, _>("vendor_id")).unwrap(),
            bid_number: row.get::<&str, _>("bid_number").to_string(),
            status: BidStatus::Draft,
            submitted_at: row.get::<Option<&str>, _>("submitted_at").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            valid_until: row.get::<Option<&str>, _>("valid_until").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            total_amount: row.get::<i64, _>("total_amount"),
            currency: row.get::<&str, _>("currency").to_string(),
            terms: row.get::<Option<&str>, _>("terms").map(|s| s.to_string()),
            notes: row.get::<Option<&str>, _>("notes").map(|s| s.to_string()),
            rank: row.get::<Option<i32>, _>("rank"),
            score: row.get::<Option<f64>, _>("score"),
            is_winner: row.get::<i32, _>("is_winner") == 1,
        }).collect())
    }

    async fn update_bid_status(&self, pool: &SqlitePool, id: Uuid, status: BidStatus) -> Result<()> {
        sqlx::query(
            r#"UPDATE sourcing_bids SET status = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(format!("{:?}", status))
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(id.to_string())
        .execute(pool).await?;
        Ok(())
    }

    async fn create_bid_line(&self, pool: &SqlitePool, line: BidLine) -> Result<BidLine> {
        sqlx::query(
            r#"INSERT INTO sourcing_bid_lines (id, bid_id, item_id, unit_price, quantity, total_price,
               delivery_date, lead_time_days, specifications_met, notes, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(line.base.id.to_string())
        .bind(line.bid_id.to_string())
        .bind(line.item_id.to_string())
        .bind(line.unit_price)
        .bind(line.quantity)
        .bind(line.total_price)
        .bind(line.delivery_date.map(|d| d.to_rfc3339()))
        .bind(line.lead_time_days)
        .bind(line.specifications_met as i32)
        .bind(&line.notes)
        .bind(line.base.created_at.to_rfc3339())
        .bind(line.base.updated_at.to_rfc3339())
        .bind(line.base.created_by.map(|id| id.to_string()))
        .bind(line.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(line)
    }

    async fn create_evaluation_criteria(&self, pool: &SqlitePool, criteria: EvaluationCriteria) -> Result<EvaluationCriteria> {
        sqlx::query(
            r#"INSERT INTO sourcing_evaluation_criteria (id, event_id, name, description, weight,
               max_score, evaluation_method, sort_order, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(criteria.base.id.to_string())
        .bind(criteria.event_id.to_string())
        .bind(&criteria.name)
        .bind(&criteria.description)
        .bind(criteria.weight)
        .bind(criteria.max_score)
        .bind(format!("{:?}", criteria.evaluation_method))
        .bind(criteria.sort_order)
        .bind(criteria.base.created_at.to_rfc3339())
        .bind(criteria.base.updated_at.to_rfc3339())
        .bind(criteria.base.created_by.map(|id| id.to_string()))
        .bind(criteria.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(criteria)
    }

    async fn create_bid_evaluation(&self, pool: &SqlitePool, eval: BidEvaluation) -> Result<BidEvaluation> {
        sqlx::query(
            r#"INSERT INTO sourcing_bid_evaluations (id, bid_id, criteria_id, score, comments,
               evaluated_by, evaluated_at, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(eval.base.id.to_string())
        .bind(eval.bid_id.to_string())
        .bind(eval.criteria_id.to_string())
        .bind(eval.score)
        .bind(&eval.comments)
        .bind(eval.evaluated_by.map(|id| id.to_string()))
        .bind(eval.evaluated_at.to_rfc3339())
        .bind(eval.base.created_at.to_rfc3339())
        .bind(eval.base.updated_at.to_rfc3339())
        .bind(eval.base.created_by.map(|id| id.to_string()))
        .bind(eval.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(eval)
    }

    async fn create_award(&self, pool: &SqlitePool, award: SourcingAward) -> Result<SourcingAward> {
        sqlx::query(
            r#"INSERT INTO sourcing_awards (id, event_id, bid_id, vendor_id, item_id, awarded_quantity,
               awarded_price, total_value, currency, awarded_at, award_type, purchase_order_id,
               contract_id, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(award.base.id.to_string())
        .bind(award.event_id.to_string())
        .bind(award.bid_id.to_string())
        .bind(award.vendor_id.to_string())
        .bind(award.item_id.map(|id| id.to_string()))
        .bind(award.awarded_quantity)
        .bind(award.awarded_price)
        .bind(award.total_value)
        .bind(&award.currency)
        .bind(award.awarded_at.to_rfc3339())
        .bind(format!("{:?}", award.award_type))
        .bind(award.purchase_order_id.map(|id| id.to_string()))
        .bind(award.contract_id.map(|id| id.to_string()))
        .bind(award.base.created_at.to_rfc3339())
        .bind(award.base.updated_at.to_rfc3339())
        .bind(award.base.created_by.map(|id| id.to_string()))
        .bind(award.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(award)
    }

    async fn create_contract(&self, pool: &SqlitePool, contract: SourcingContract) -> Result<SourcingContract> {
        sqlx::query(
            r#"INSERT INTO sourcing_contracts (id, event_id, vendor_id, contract_number, title,
               description, start_date, end_date, total_value, currency, terms, status,
               renewal_type, auto_renew, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(contract.base.id.to_string())
        .bind(contract.event_id.map(|id| id.to_string()))
        .bind(contract.vendor_id.to_string())
        .bind(&contract.contract_number)
        .bind(&contract.title)
        .bind(&contract.description)
        .bind(contract.start_date.to_rfc3339())
        .bind(contract.end_date.to_rfc3339())
        .bind(contract.total_value)
        .bind(&contract.currency)
        .bind(&contract.terms)
        .bind(format!("{:?}", contract.status))
        .bind(contract.renewal_type.clone().map(|t| format!("{:?}", t)))
        .bind(contract.auto_renew as i32)
        .bind(contract.base.created_at.to_rfc3339())
        .bind(contract.base.updated_at.to_rfc3339())
        .bind(contract.base.created_by.map(|id| id.to_string()))
        .bind(contract.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(contract)
    }

    async fn add_supplier(&self, pool: &SqlitePool, supplier: SourcingSupplier) -> Result<SourcingSupplier> {
        sqlx::query(
            r#"INSERT INTO sourcing_suppliers (id, event_id, vendor_id, invited_at, accepted_at,
               declined_at, status, notes, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(supplier.base.id.to_string())
        .bind(supplier.event_id.to_string())
        .bind(supplier.vendor_id.to_string())
        .bind(supplier.invited_at.to_rfc3339())
        .bind(supplier.accepted_at.map(|d| d.to_rfc3339()))
        .bind(supplier.declined_at.map(|d| d.to_rfc3339()))
        .bind(format!("{:?}", supplier.status))
        .bind(&supplier.notes)
        .bind(supplier.base.created_at.to_rfc3339())
        .bind(supplier.base.updated_at.to_rfc3339())
        .bind(supplier.base.created_by.map(|id| id.to_string()))
        .bind(supplier.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(supplier)
    }
}
