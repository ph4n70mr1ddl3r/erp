use crate::models::*;
use async_trait::async_trait;
use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Result};
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait GiftCardRepository: Send + Sync {
    async fn create(&self, card: &GiftCard) -> Result<GiftCard>;
    async fn get_by_id(&self, id: Uuid) -> Result<Option<GiftCard>>;
    async fn get_by_card_number(&self, card_number: &str) -> Result<Option<GiftCard>>;
    async fn update(&self, card: &GiftCard) -> Result<GiftCard>;
    async fn list(&self, page: i32, per_page: i32) -> Result<Vec<GiftCard>>;
    async fn list_by_customer(&self, customer_id: Uuid) -> Result<Vec<GiftCard>>;
    async fn create_transaction(&self, tx: &GiftCardTransaction) -> Result<GiftCardTransaction>;
    async fn list_transactions(&self, gift_card_id: Uuid) -> Result<Vec<GiftCardTransaction>>;
}

pub struct SqliteGiftCardRepository {
    pool: SqlitePool,
}

impl SqliteGiftCardRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GiftCardRepository for SqliteGiftCardRepository {
    async fn create(&self, card: &GiftCard) -> Result<GiftCard> {
        let c = card.clone();
        sqlx::query(
            r#"INSERT INTO gift_cards (id, card_number, pin, barcode, gift_card_type, 
               initial_balance, current_balance, currency, customer_id, order_id, 
               purchased_by, recipient_email, recipient_name, message, status, 
               issued_date, expiry_date, last_used_at, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(c.base.id)
        .bind(&c.card_number)
        .bind(&c.pin)
        .bind(&c.barcode)
        .bind(&c.gift_card_type)
        .bind(c.initial_balance)
        .bind(c.current_balance)
        .bind(&c.currency)
        .bind(c.customer_id)
        .bind(c.order_id)
        .bind(c.purchased_by)
        .bind(&c.recipient_email)
        .bind(&c.recipient_name)
        .bind(&c.message)
        .bind(&c.status)
        .bind(c.issued_date)
        .bind(c.expiry_date)
        .bind(c.last_used_at)
        .bind(c.created_at)
        .bind(c.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(c)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Option<GiftCard>> {
        let row = sqlx::query_as::<_, GiftCardRow>(
            "SELECT * FROM gift_cards WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| r.into()))
    }

    async fn get_by_card_number(&self, card_number: &str) -> Result<Option<GiftCard>> {
        let row = sqlx::query_as::<_, GiftCardRow>(
            "SELECT * FROM gift_cards WHERE card_number = ?"
        )
        .bind(card_number)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(|r| r.into()))
    }

    async fn update(&self, card: &GiftCard) -> Result<GiftCard> {
        let c = card.clone();
        sqlx::query(
            r#"UPDATE gift_cards SET current_balance = ?, status = ?, 
               last_used_at = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(c.current_balance)
        .bind(&c.status)
        .bind(c.last_used_at)
        .bind(c.updated_at)
        .bind(c.base.id)
        .execute(&self.pool)
        .await?;
        Ok(c)
    }

    async fn list(&self, page: i32, per_page: i32) -> Result<Vec<GiftCard>> {
        let offset = (page - 1) * per_page;
        let rows = sqlx::query_as::<_, GiftCardRow>(
            "SELECT * FROM gift_cards ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn list_by_customer(&self, customer_id: Uuid) -> Result<Vec<GiftCard>> {
        let rows = sqlx::query_as::<_, GiftCardRow>(
            "SELECT * FROM gift_cards WHERE customer_id = ? ORDER BY created_at DESC"
        )
        .bind(customer_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    async fn create_transaction(&self, tx: &GiftCardTransaction) -> Result<GiftCardTransaction> {
        let t = tx.clone();
        sqlx::query(
            r#"INSERT INTO gift_card_transactions (id, transaction_number, gift_card_id, 
               transaction_type, amount, balance_before, balance_after, order_id, 
               reference, notes, created_by, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(t.base.id)
        .bind(&t.transaction_number)
        .bind(t.gift_card_id)
        .bind(&t.transaction_type)
        .bind(t.amount)
        .bind(t.balance_before)
        .bind(t.balance_after)
        .bind(t.order_id)
        .bind(&t.reference)
        .bind(&t.notes)
        .bind(t.created_by)
        .bind(t.created_at)
        .execute(&self.pool)
        .await?;
        Ok(t)
    }

    async fn list_transactions(&self, gift_card_id: Uuid) -> Result<Vec<GiftCardTransaction>> {
        let rows = sqlx::query_as::<_, GiftCardTransactionRow>(
            "SELECT * FROM gift_card_transactions WHERE gift_card_id = ? ORDER BY created_at DESC"
        )
        .bind(gift_card_id)
        .fetch_all(&self.pool)
        .await?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct GiftCardRow {
    id: Uuid,
    card_number: String,
    pin: Option<String>,
    barcode: Option<String>,
    gift_card_type: GiftCardType,
    initial_balance: i64,
    current_balance: i64,
    currency: String,
    customer_id: Option<Uuid>,
    order_id: Option<Uuid>,
    purchased_by: Option<Uuid>,
    recipient_email: Option<String>,
    recipient_name: Option<String>,
    message: Option<String>,
    status: GiftCardStatus,
    issued_date: NaiveDate,
    expiry_date: Option<NaiveDate>,
    last_used_at: Option<DateTime<Utc>>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
}

impl From<GiftCardRow> for GiftCard {
    fn from(row: GiftCardRow) -> Self {
        Self {
            base: BaseEntity::new_with_id(row.id),
            card_number: row.card_number,
            pin: row.pin,
            barcode: row.barcode,
            gift_card_type: row.gift_card_type,
            initial_balance: row.initial_balance,
            current_balance: row.current_balance,
            currency: row.currency,
            customer_id: row.customer_id,
            order_id: row.order_id,
            purchased_by: row.purchased_by,
            recipient_email: row.recipient_email,
            recipient_name: row.recipient_name,
            message: row.message,
            status: row.status,
            issued_date: row.issued_date,
            expiry_date: row.expiry_date,
            last_used_at: row.last_used_at,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

#[derive(sqlx::FromRow)]
struct GiftCardTransactionRow {
    id: Uuid,
    transaction_number: String,
    gift_card_id: Uuid,
    transaction_type: GiftCardTransactionType,
    amount: i64,
    balance_before: i64,
    balance_after: i64,
    order_id: Option<Uuid>,
    reference: Option<String>,
    notes: Option<String>,
    created_by: Option<Uuid>,
    created_at: DateTime<Utc>,
}

impl From<GiftCardTransactionRow> for GiftCardTransaction {
    fn from(row: GiftCardTransactionRow) -> Self {
        Self {
            base: BaseEntity::new_with_id(row.id),
            transaction_number: row.transaction_number,
            gift_card_id: row.gift_card_id,
            transaction_type: row.transaction_type,
            amount: row.amount,
            balance_before: row.balance_before,
            balance_after: row.balance_after,
            order_id: row.order_id,
            reference: row.reference,
            notes: row.notes,
            created_by: row.created_by,
            created_at: row.created_at,
        }
    }
}
