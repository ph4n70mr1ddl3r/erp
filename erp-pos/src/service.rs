use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Money, Currency};
use serde::Serialize;
use crate::models::*;
use crate::repository::*;

pub struct POSStoreService { repo: SqlitePOSStoreRepository }
impl POSStoreService {
    pub fn new() -> Self { Self { repo: SqlitePOSStoreRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<POSStore> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<POSStore>> {
        self.repo.find_all(pool, pagination).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut store: POSStore) -> Result<POSStore> {
        if store.store_code.is_empty() {
            return Err(Error::validation("Store code is required"));
        }
        store.base = BaseEntity::new();
        store.status = POSStatus::Active;
        self.repo.create(pool, store).await
    }
    
    pub async fn update(&self, pool: &SqlitePool, mut store: POSStore) -> Result<POSStore> {
        store.base.updated_at = Utc::now();
        self.repo.update(pool, store).await
    }
}

pub struct POSTransactionService { repo: SqlitePOSTransactionRepository }
impl POSTransactionService {
    pub fn new() -> Self { Self { repo: SqlitePOSTransactionRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<POSTransaction> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn list(&self, pool: &SqlitePool, store_id: Option<Uuid>, pagination: Pagination) -> Result<Paginated<POSTransaction>> {
        self.repo.find_all(pool, store_id, pagination).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut transaction: POSTransaction) -> Result<POSTransaction> {
        if transaction.lines.is_empty() {
            return Err(Error::validation("Transaction must have at least one line"));
        }
        
        transaction.transaction_number = format!("POS-{}", Utc::now().format("%Y%m%d%H%M%S"));
        transaction.base = BaseEntity::new();
        transaction.status = erp_core::Status::Active;
        
        let subtotal: i64 = transaction.lines.iter().map(|l| l.line_total.amount).sum();
        transaction.subtotal = Money::new(subtotal, Currency::USD);
        
        let tax: i64 = transaction.lines.iter().map(|l| l.tax_amount.amount).sum();
        transaction.tax_amount = Money::new(tax, Currency::USD);
        
        let discount: i64 = transaction.lines.iter().map(|l| l.discount_amount.amount).sum();
        transaction.discount_amount = Money::new(discount, Currency::USD);
        
        let total = subtotal + tax - discount;
        transaction.total = Money::new(total, Currency::USD);
        
        let payment_total: i64 = transaction.payments.iter().map(|p| p.amount.amount).sum();
        let change = payment_total - total;
        transaction.change_amount = Money::new(change.max(0), Currency::USD);
        
        for line in &mut transaction.lines {
            line.id = Uuid::new_v4();
            line.transaction_id = transaction.base.id;
        }
        
        for payment in &mut transaction.payments {
            payment.id = Uuid::new_v4();
            payment.transaction_id = transaction.base.id;
        }
        
        transaction.completed_at = Some(Utc::now());
        
        self.repo.create(pool, transaction).await
    }
    
    pub async fn void(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.void(pool, id).await
    }
    
    pub async fn get_daily_summary(&self, pool: &SqlitePool, store_id: Uuid, date: chrono::NaiveDate) -> Result<DailySummary> {
        let start = date.and_hms_opt(0, 0, 0).unwrap();
        let end = date.and_hms_opt(23, 59, 59).unwrap();
        
        let start_dt: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_naive_utc_and_offset(start, chrono::Utc);
        let end_dt: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_naive_utc_and_offset(end, chrono::Utc);
        
        let total_sales: i64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(total), 0) FROM pos_transactions WHERE store_id = ? AND created_at >= ? AND created_at <= ? AND status = 'Active'"
        )
        .bind(store_id.to_string())
        .bind(start_dt.to_rfc3339())
        .bind(end_dt.to_rfc3339())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let transaction_count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM pos_transactions WHERE store_id = ? AND created_at >= ? AND created_at <= ? AND status = 'Active'"
        )
        .bind(store_id.to_string())
        .bind(start_dt.to_rfc3339())
        .bind(end_dt.to_rfc3339())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let total_tax: i64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(tax_amount), 0) FROM pos_transactions WHERE store_id = ? AND created_at >= ? AND created_at <= ? AND status = 'Active'"
        )
        .bind(store_id.to_string())
        .bind(start_dt.to_rfc3339())
        .bind(end_dt.to_rfc3339())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(DailySummary {
            store_id,
            date,
            total_sales: Money::new(total_sales, Currency::USD),
            transaction_count,
            total_tax: Money::new(total_tax, Currency::USD),
        })
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct DailySummary {
    pub store_id: Uuid,
    pub date: chrono::NaiveDate,
    pub total_sales: Money,
    pub transaction_count: i64,
    pub total_tax: Money,
}

pub struct GiftCardService { repo: SqliteGiftCardRepository }
impl GiftCardService {
    pub fn new() -> Self { Self { repo: SqliteGiftCardRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<GiftCard> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn get_by_number(&self, pool: &SqlitePool, number: &str) -> Result<GiftCard> {
        self.repo.find_by_number(pool, number).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut card: GiftCard) -> Result<GiftCard> {
        card.card_number = format!("GC{}", Utc::now().format("%Y%m%d%H%M%S"));
        card.base = BaseEntity::new();
        card.current_balance = card.initial_amount.clone();
        card.status = GiftCardStatus::Active;
        card.sold_at = Utc::now();
        self.repo.create(pool, card).await
    }
    
    pub async fn redeem(&self, pool: &SqlitePool, card_number: &str, amount: i64) -> Result<GiftCard> {
        let card = self.repo.find_by_number(pool, card_number).await?;
        
        if card.status != GiftCardStatus::Active {
            return Err(Error::validation("Gift card is not active"));
        }
        
        if let Some(expires) = card.expires_at {
            if expires < Utc::now() {
                return Err(Error::validation("Gift card has expired"));
            }
        }
        
        if card.current_balance.amount < amount {
            return Err(Error::validation("Insufficient balance on gift card"));
        }
        
        let new_balance = card.current_balance.amount - amount;
        self.repo.update_balance(pool, card.base.id, new_balance).await?;
        
        let status = if new_balance == 0 { GiftCardStatus::Redeemed } else { card.status };
        sqlx::query(
            "UPDATE gift_cards SET status = ? WHERE id = ?"
        )
        .bind(format!("{:?}", status))
        .bind(card.base.id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        self.repo.find_by_number(pool, card_number).await
    }
    
    pub async fn reload(&self, pool: &SqlitePool, card_number: &str, amount: i64) -> Result<GiftCard> {
        let card = self.repo.find_by_number(pool, card_number).await?;
        
        if card.status != GiftCardStatus::Active {
            return Err(Error::validation("Gift card is not active"));
        }
        
        let new_balance = card.current_balance.amount + amount;
        self.repo.update_balance(pool, card.base.id, new_balance).await?;
        
        self.repo.find_by_number(pool, card_number).await
    }
}

pub struct LoyaltyService;
impl LoyaltyService {
    pub fn new() -> Self { Self }
    
    pub async fn earn_points(
        pool: &SqlitePool,
        account_id: Uuid,
        transaction_amount: i64,
        points_per_currency: f64,
    ) -> Result<i64> {
        let points = (transaction_amount as f64 * points_per_currency / 100.0) as i64;
        
        sqlx::query(
            "UPDATE loyalty_accounts SET points_balance = points_balance + ?, updated_at = ? WHERE id = ?"
        )
        .bind(points)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(account_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(points)
    }
    
    pub async fn redeem_points(
        pool: &SqlitePool,
        account_id: Uuid,
        points: i64,
    ) -> Result<Money> {
        let balance: i64 = sqlx::query_scalar(
            "SELECT points_balance FROM loyalty_accounts WHERE id = ?"
        )
        .bind(account_id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .unwrap_or(0);
        
        if balance < points {
            return Err(Error::validation("Insufficient loyalty points"));
        }
        
        let redemption_rate = 0.01;
        let value = (points as f64 * redemption_rate * 100.0) as i64;
        
        sqlx::query(
            "UPDATE loyalty_accounts SET points_balance = points_balance - ?, updated_at = ? WHERE id = ?"
        )
        .bind(points)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(account_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(Money::new(value, Currency::USD))
    }
}
