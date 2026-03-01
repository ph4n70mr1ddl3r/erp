use async_trait::async_trait;
use sqlx::SqlitePool;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity};
use crate::models::*;
use uuid::Uuid;

#[async_trait]
pub trait POSStoreRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<POSStore>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<POSStore>>;
    async fn create(&self, pool: &SqlitePool, store: POSStore) -> Result<POSStore>;
    async fn update(&self, pool: &SqlitePool, store: POSStore) -> Result<POSStore>;
}

pub struct SqlitePOSStoreRepository;

#[async_trait]
impl POSStoreRepository for SqlitePOSStoreRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<POSStore> {
        let row = sqlx::query_as::<_, POSStoreRow>(
            "SELECT * FROM pos_stores WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("POSStore", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<POSStore>> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pos_stores")
            .fetch_one(pool)
            .await
            .map_err(Error::Database)?;
        
        let offset = (pagination.page.saturating_sub(1)) * pagination.per_page;
        let rows = sqlx::query_as::<_, POSStoreRow>(
            "SELECT * FROM pos_stores ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(Paginated::new(rows.into_iter().map(|r| r.into()).collect(), count as u64, pagination))
    }
    
    async fn create(&self, pool: &SqlitePool, store: POSStore) -> Result<POSStore> {
        sqlx::query(
            "INSERT INTO pos_stores (id, store_code, name, address, city, state, postal_code, country, phone, email, manager_id, warehouse_id, status, opening_time, closing_time, timezone, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(store.base.id.to_string())
        .bind(&store.store_code)
        .bind(&store.name)
        .bind(&store.address)
        .bind(&store.city)
        .bind(&store.state)
        .bind(&store.postal_code)
        .bind(&store.country)
        .bind(&store.phone)
        .bind(&store.email)
        .bind(store.manager_id.map(|id| id.to_string()))
        .bind(store.warehouse_id.map(|id| id.to_string()))
        .bind(format!("{:?}", store.status))
        .bind(&store.opening_time)
        .bind(&store.closing_time)
        .bind(&store.timezone)
        .bind(store.base.created_at.to_rfc3339())
        .bind(store.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(store)
    }
    
    async fn update(&self, pool: &SqlitePool, store: POSStore) -> Result<POSStore> {
        sqlx::query(
            "UPDATE pos_stores SET name = ?, address = ?, status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&store.name)
        .bind(&store.address)
        .bind(format!("{:?}", store.status))
        .bind(store.base.updated_at.to_rfc3339())
        .bind(store.base.id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(store)
    }
}

#[derive(sqlx::FromRow)]
struct POSStoreRow {
    id: String,
    store_code: String,
    name: String,
    address: String,
    city: String,
    state: String,
    postal_code: String,
    country: String,
    phone: Option<String>,
    email: Option<String>,
    manager_id: Option<String>,
    warehouse_id: Option<String>,
    status: String,
    opening_time: String,
    closing_time: String,
    timezone: String,
    created_at: String,
    updated_at: String,
}

impl From<POSStoreRow> for POSStore {
    fn from(r: POSStoreRow) -> Self {
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            store_code: r.store_code,
            name: r.name,
            address: r.address,
            city: r.city,
            state: r.state,
            postal_code: r.postal_code,
            country: r.country,
            phone: r.phone,
            email: r.email,
            manager_id: r.manager_id.and_then(|id| Uuid::parse_str(&id).ok()),
            warehouse_id: r.warehouse_id.and_then(|id| Uuid::parse_str(&id).ok()),
            status: match r.status.as_str() {
                "Inactive" => POSStatus::Inactive,
                "Maintenance" => POSStatus::Maintenance,
                _ => POSStatus::Active,
            },
            opening_time: r.opening_time,
            closing_time: r.closing_time,
            timezone: r.timezone,
        }
    }
}

#[async_trait]
pub trait POSTransactionRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<POSTransaction>;
    async fn find_all(&self, pool: &SqlitePool, store_id: Option<Uuid>, pagination: Pagination) -> Result<Paginated<POSTransaction>>;
    async fn create(&self, pool: &SqlitePool, transaction: POSTransaction) -> Result<POSTransaction>;
    async fn void(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqlitePOSTransactionRepository;

#[async_trait]
impl POSTransactionRepository for SqlitePOSTransactionRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<POSTransaction> {
        let row = sqlx::query_as::<_, POSTransactionRow>(
            "SELECT * FROM pos_transactions WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("POSTransaction", &id.to_string()))?;
        
        let lines = self.get_lines(pool, id).await?;
        let payments = self.get_payments(pool, id).await?;
        Ok(row.into_transaction(lines, payments))
    }
    
    async fn find_all(&self, pool: &SqlitePool, store_id: Option<Uuid>, pagination: Pagination) -> Result<Paginated<POSTransaction>> {
        let (count, rows) = match store_id {
            Some(sid) => {
                let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pos_transactions WHERE store_id = ?")
                    .bind(sid.to_string())
                    .fetch_one(pool)
                    .await
                    .map_err(Error::Database)?;
                
                let offset = (pagination.page.saturating_sub(1)) * pagination.per_page;
                let rows = sqlx::query_as::<_, POSTransactionRow>(
                    "SELECT * FROM pos_transactions WHERE store_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
                )
                .bind(sid.to_string())
                .bind(pagination.per_page as i64)
                .bind(offset as i64)
                .fetch_all(pool)
                .await
                .map_err(Error::Database)?;
                
                (count, rows)
            }
            None => {
                let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pos_transactions")
                    .fetch_one(pool)
                    .await
                    .map_err(Error::Database)?;
                
                let offset = (pagination.page.saturating_sub(1)) * pagination.per_page;
                let rows = sqlx::query_as::<_, POSTransactionRow>(
                    "SELECT * FROM pos_transactions ORDER BY created_at DESC LIMIT ? OFFSET ?"
                )
                .bind(pagination.per_page as i64)
                .bind(offset as i64)
                .fetch_all(pool)
                .await
                .map_err(Error::Database)?;
                
                (count, rows)
            }
        };
        
        let mut transactions = Vec::new();
        for row in rows {
            let lines = self.get_lines(pool, Uuid::parse_str(&row.id).unwrap_or_default()).await?;
            let payments = self.get_payments(pool, Uuid::parse_str(&row.id).unwrap_or_default()).await?;
            transactions.push(row.into_transaction(lines, payments));
        }
        
        Ok(Paginated::new(transactions, count as u64, pagination))
    }
    
    async fn create(&self, pool: &SqlitePool, transaction: POSTransaction) -> Result<POSTransaction> {
        sqlx::query(
            "INSERT INTO pos_transactions (id, transaction_number, store_id, terminal_id, register_id, transaction_type, customer_id, sales_rep_id, subtotal, discount_amount, tax_amount, total, change_amount, status, original_transaction_id, notes, completed_at, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(transaction.base.id.to_string())
        .bind(&transaction.transaction_number)
        .bind(transaction.store_id.to_string())
        .bind(transaction.terminal_id.to_string())
        .bind(transaction.register_id.to_string())
        .bind(format!("{:?}", transaction.transaction_type))
        .bind(transaction.customer_id.map(|id| id.to_string()))
        .bind(transaction.sales_rep_id.map(|id| id.to_string()))
        .bind(transaction.subtotal.amount)
        .bind(transaction.discount_amount.amount)
        .bind(transaction.tax_amount.amount)
        .bind(transaction.total.amount)
        .bind(transaction.change_amount.amount)
        .bind(format!("{:?}", transaction.status))
        .bind(transaction.original_transaction_id.map(|id| id.to_string()))
        .bind(&transaction.notes)
        .bind(transaction.completed_at.map(|d| d.to_rfc3339()))
        .bind(transaction.base.created_at.to_rfc3339())
        .bind(transaction.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        for line in &transaction.lines {
            self.create_line(pool, line).await?;
        }
        
        for payment in &transaction.payments {
            self.create_payment(pool, payment).await?;
        }
        
        Ok(transaction)
    }
    
    async fn void(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let now = chrono::Utc::now();
        sqlx::query(
            "UPDATE pos_transactions SET status = 'Inactive', updated_at = ? WHERE id = ?"
        )
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

impl SqlitePOSTransactionRepository {
    async fn get_lines(&self, pool: &SqlitePool, transaction_id: Uuid) -> Result<Vec<POSTransactionLine>> {
        let rows = sqlx::query_as::<_, POSTransactionLineRow>(
            "SELECT * FROM pos_transaction_lines WHERE transaction_id = ?"
        )
        .bind(transaction_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    async fn create_line(&self, pool: &SqlitePool, line: &POSTransactionLine) -> Result<()> {
        sqlx::query(
            "INSERT INTO pos_transaction_lines (id, transaction_id, line_number, product_id, description, quantity, unit_price, discount_percent, discount_amount, tax_rate_id, tax_amount, line_total, lot_number, serial_number)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(line.id.to_string())
        .bind(line.transaction_id.to_string())
        .bind(line.line_number)
        .bind(line.product_id.to_string())
        .bind(&line.description)
        .bind(line.quantity)
        .bind(line.unit_price.amount)
        .bind(line.discount_percent)
        .bind(line.discount_amount.amount)
        .bind(line.tax_rate_id.map(|id| id.to_string()))
        .bind(line.tax_amount.amount)
        .bind(line.line_total.amount)
        .bind(&line.lot_number)
        .bind(&line.serial_number)
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
    
    async fn get_payments(&self, pool: &SqlitePool, transaction_id: Uuid) -> Result<Vec<POSTransactionPayment>> {
        let rows = sqlx::query_as::<_, POSTransactionPaymentRow>(
            "SELECT * FROM pos_transaction_payments WHERE transaction_id = ?"
        )
        .bind(transaction_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    async fn create_payment(&self, pool: &SqlitePool, payment: &POSTransactionPayment) -> Result<()> {
        sqlx::query(
            "INSERT INTO pos_transaction_payments (id, transaction_id, payment_method, amount, reference, card_last_four, card_type, authorization_code, gift_card_id)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(payment.id.to_string())
        .bind(payment.transaction_id.to_string())
        .bind(format!("{:?}", payment.payment_method))
        .bind(payment.amount.amount)
        .bind(&payment.reference)
        .bind(&payment.card_last_four)
        .bind(&payment.card_type)
        .bind(&payment.authorization_code)
        .bind(payment.gift_card_id.map(|id| id.to_string()))
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct POSTransactionRow {
    id: String,
    transaction_number: String,
    store_id: String,
    terminal_id: String,
    register_id: String,
    transaction_type: String,
    customer_id: Option<String>,
    sales_rep_id: Option<String>,
    subtotal: i64,
    discount_amount: i64,
    tax_amount: i64,
    total: i64,
    change_amount: i64,
    status: String,
    original_transaction_id: Option<String>,
    notes: Option<String>,
    completed_at: Option<String>,
    created_at: String,
    updated_at: String,
}

impl POSTransactionRow {
    fn into_transaction(self, lines: Vec<POSTransactionLine>, payments: Vec<POSTransactionPayment>) -> POSTransaction {
        POSTransaction {
            base: BaseEntity {
                id: Uuid::parse_str(&self.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            transaction_number: self.transaction_number,
            store_id: Uuid::parse_str(&self.store_id).unwrap_or_default(),
            terminal_id: Uuid::parse_str(&self.terminal_id).unwrap_or_default(),
            register_id: Uuid::parse_str(&self.register_id).unwrap_or_default(),
            transaction_type: match self.transaction_type.as_str() {
                "Return" => TransactionType::Return,
                "Void" => TransactionType::Void,
                "Refund" => TransactionType::Refund,
                "Layaway" => TransactionType::Layaway,
                "SpecialOrder" => TransactionType::SpecialOrder,
                _ => TransactionType::Sale,
            },
            customer_id: self.customer_id.and_then(|id| Uuid::parse_str(&id).ok()),
            sales_rep_id: self.sales_rep_id.and_then(|id| Uuid::parse_str(&id).ok()),
            lines,
            payments,
            subtotal: erp_core::Money::new(self.subtotal, erp_core::Currency::USD),
            discount_amount: erp_core::Money::new(self.discount_amount, erp_core::Currency::USD),
            tax_amount: erp_core::Money::new(self.tax_amount, erp_core::Currency::USD),
            total: erp_core::Money::new(self.total, erp_core::Currency::USD),
            change_amount: erp_core::Money::new(self.change_amount, erp_core::Currency::USD),
            status: match self.status.as_str() {
                "Inactive" => erp_core::Status::Inactive,
                _ => erp_core::Status::Active,
            },
            original_transaction_id: self.original_transaction_id.and_then(|id| Uuid::parse_str(&id).ok()),
            notes: self.notes,
            completed_at: self.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
        }
    }
}

#[derive(sqlx::FromRow)]
struct POSTransactionLineRow {
    id: String,
    transaction_id: String,
    line_number: i32,
    product_id: String,
    description: String,
    quantity: i64,
    unit_price: i64,
    discount_percent: f64,
    discount_amount: i64,
    tax_rate_id: Option<String>,
    tax_amount: i64,
    line_total: i64,
    lot_number: Option<String>,
    serial_number: Option<String>,
}

impl From<POSTransactionLineRow> for POSTransactionLine {
    fn from(r: POSTransactionLineRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            transaction_id: Uuid::parse_str(&r.transaction_id).unwrap_or_default(),
            line_number: r.line_number,
            product_id: Uuid::parse_str(&r.product_id).unwrap_or_default(),
            description: r.description,
            quantity: r.quantity,
            unit_price: erp_core::Money::new(r.unit_price, erp_core::Currency::USD),
            discount_percent: r.discount_percent,
            discount_amount: erp_core::Money::new(r.discount_amount, erp_core::Currency::USD),
            tax_rate_id: r.tax_rate_id.and_then(|id| Uuid::parse_str(&id).ok()),
            tax_amount: erp_core::Money::new(r.tax_amount, erp_core::Currency::USD),
            line_total: erp_core::Money::new(r.line_total, erp_core::Currency::USD),
            lot_number: r.lot_number,
            serial_number: r.serial_number,
        }
    }
}

#[derive(sqlx::FromRow)]
struct POSTransactionPaymentRow {
    id: String,
    transaction_id: String,
    payment_method: String,
    amount: i64,
    reference: Option<String>,
    card_last_four: Option<String>,
    card_type: Option<String>,
    authorization_code: Option<String>,
    gift_card_id: Option<String>,
}

impl From<POSTransactionPaymentRow> for POSTransactionPayment {
    fn from(r: POSTransactionPaymentRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            transaction_id: Uuid::parse_str(&r.transaction_id).unwrap_or_default(),
            payment_method: match r.payment_method.as_str() {
                "CreditCard" => PaymentMethod::CreditCard,
                "DebitCard" => PaymentMethod::DebitCard,
                "GiftCard" => PaymentMethod::GiftCard,
                "Check" => PaymentMethod::Check,
                "MobilePayment" => PaymentMethod::MobilePayment,
                "StoreCredit" => PaymentMethod::StoreCredit,
                "Mixed" => PaymentMethod::Mixed,
                _ => PaymentMethod::Cash,
            },
            amount: erp_core::Money::new(r.amount, erp_core::Currency::USD),
            reference: r.reference,
            card_last_four: r.card_last_four,
            card_type: r.card_type,
            authorization_code: r.authorization_code,
            gift_card_id: r.gift_card_id.and_then(|id| Uuid::parse_str(&id).ok()),
        }
    }
}

#[async_trait]
pub trait GiftCardRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<GiftCard>;
    async fn find_by_number(&self, pool: &SqlitePool, number: &str) -> Result<GiftCard>;
    async fn create(&self, pool: &SqlitePool, card: GiftCard) -> Result<GiftCard>;
    async fn update_balance(&self, pool: &SqlitePool, id: Uuid, new_balance: i64) -> Result<()>;
}

pub struct SqliteGiftCardRepository;

#[async_trait]
impl GiftCardRepository for SqliteGiftCardRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<GiftCard> {
        let row = sqlx::query_as::<_, GiftCardRow>(
            "SELECT * FROM gift_cards WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("GiftCard", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn find_by_number(&self, pool: &SqlitePool, number: &str) -> Result<GiftCard> {
        let row = sqlx::query_as::<_, GiftCardRow>(
            "SELECT * FROM gift_cards WHERE card_number = ?"
        )
        .bind(number)
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("GiftCard", number))?;
        
        Ok(row.into())
    }
    
    async fn create(&self, pool: &SqlitePool, card: GiftCard) -> Result<GiftCard> {
        sqlx::query(
            "INSERT INTO gift_cards (id, card_number, initial_amount, current_balance, sold_at, sold_at_store_id, customer_id, expires_at, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(card.base.id.to_string())
        .bind(&card.card_number)
        .bind(card.initial_amount.amount)
        .bind(card.current_balance.amount)
        .bind(card.sold_at.to_rfc3339())
        .bind(card.sold_at_store_id.to_string())
        .bind(card.customer_id.map(|id| id.to_string()))
        .bind(card.expires_at.map(|d| d.to_rfc3339()))
        .bind(format!("{:?}", card.status))
        .bind(card.base.created_at.to_rfc3339())
        .bind(card.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(card)
    }
    
    async fn update_balance(&self, pool: &SqlitePool, id: Uuid, new_balance: i64) -> Result<()> {
        let now = chrono::Utc::now();
        sqlx::query(
            "UPDATE gift_cards SET current_balance = ?, updated_at = ? WHERE id = ?"
        )
        .bind(new_balance)
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct GiftCardRow {
    id: String,
    card_number: String,
    initial_amount: i64,
    current_balance: i64,
    sold_at: String,
    sold_at_store_id: String,
    customer_id: Option<String>,
    expires_at: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
}

impl From<GiftCardRow> for GiftCard {
    fn from(r: GiftCardRow) -> Self {
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            card_number: r.card_number,
            initial_amount: erp_core::Money::new(r.initial_amount, erp_core::Currency::USD),
            current_balance: erp_core::Money::new(r.current_balance, erp_core::Currency::USD),
            sold_at: chrono::DateTime::parse_from_rfc3339(&r.sold_at)
                .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            sold_at_store_id: Uuid::parse_str(&r.sold_at_store_id).unwrap_or_default(),
            customer_id: r.customer_id.and_then(|id| Uuid::parse_str(&id).ok()),
            expires_at: r.expires_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            status: match r.status.as_str() {
                "Redeemed" => GiftCardStatus::Redeemed,
                "Expired" => GiftCardStatus::Expired,
                "Cancelled" => GiftCardStatus::Cancelled,
                _ => GiftCardStatus::Active,
            },
        }
    }
}
