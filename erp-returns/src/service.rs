use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Money, Currency};
use crate::models::*;
use crate::repository::*;

pub struct ReturnService { repo: SqliteReturnRepository }
impl ReturnService {
    pub fn new() -> Self { Self { repo: SqliteReturnRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<ReturnOrder> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ReturnOrder>> {
        self.repo.find_all(pool, pagination).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut order: ReturnOrder) -> Result<ReturnOrder> {
        if order.lines.is_empty() {
            return Err(Error::validation("Return must have at least one line"));
        }
        
        order.return_number = format!("RMA-{}", Utc::now().format("%Y%m%d%H%M%S"));
        order.base = BaseEntity::new();
        order.status = ReturnStatus::Draft;
        
        let total_credit: i64 = order.lines.iter().map(|l| l.credit_amount.amount).sum();
        order.total_credit = Money::new(total_credit, Currency::USD);
        
        for line in &mut order.lines {
            line.id = Uuid::new_v4();
            line.return_order_id = order.base.id;
        }
        
        self.repo.create(pool, order).await
    }
    
    pub async fn approve(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, ReturnStatus::Approved).await
    }
    
    pub async fn receive(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let now = chrono::Utc::now();
        sqlx::query(
            "UPDATE return_orders SET status = 'Received', received_date = ?, updated_at = ? WHERE id = ?"
        )
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(())
    }
    
    pub async fn inspect(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, ReturnStatus::Inspected).await
    }
    
    pub async fn process(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, ReturnStatus::Processed).await
    }
    
    pub async fn reject(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, ReturnStatus::Rejected).await
    }
    
    pub async fn cancel(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, ReturnStatus::Cancelled).await
    }
    
    pub async fn update_line_inspection(
        &self,
        pool: &SqlitePool,
        return_order_id: Uuid,
        line_id: Uuid,
        quantity_received: i64,
        quantity_approved: i64,
        condition: ItemCondition,
        disposition: ReturnDisposition,
        notes: Option<&str>,
    ) -> Result<()> {
        let credit_amount = quantity_approved * 1000;
        
        sqlx::query(
            "UPDATE return_lines SET quantity_received = ?, quantity_approved = ?, condition_type = ?, disposition = ?, inspection_notes = ?, credit_amount = ? WHERE id = ? AND return_order_id = ?"
        )
        .bind(quantity_received)
        .bind(quantity_approved)
        .bind(format!("{:?}", condition))
        .bind(format!("{:?}", disposition))
        .bind(notes)
        .bind(credit_amount)
        .bind(line_id.to_string())
        .bind(return_order_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(())
    }
}

pub struct CreditMemoService { repo: SqliteCreditMemoRepository }
impl CreditMemoService {
    pub fn new() -> Self { Self { repo: SqliteCreditMemoRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<CreditMemo> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<CreditMemo>> {
        self.repo.find_all(pool, pagination).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut memo: CreditMemo) -> Result<CreditMemo> {
        if memo.lines.is_empty() {
            return Err(Error::validation("Credit memo must have at least one line"));
        }
        
        memo.memo_number = format!("CM-{}", Utc::now().format("%Y%m%d%H%M%S"));
        memo.base = BaseEntity::new();
        memo.status = CreditMemoStatus::Draft;
        
        let subtotal: i64 = memo.lines.iter().map(|l| l.line_total.amount).sum();
        memo.subtotal = Money::new(subtotal, Currency::USD);
        memo.total = Money::new(subtotal + memo.tax_amount.amount, Currency::USD);
        memo.applied_amount = Money::zero(Currency::USD);
        
        for line in &mut memo.lines {
            line.id = Uuid::new_v4();
            line.credit_memo_id = memo.base.id;
        }
        
        self.repo.create(pool, memo).await
    }
    
    pub async fn issue(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, CreditMemoStatus::Issued).await
    }
    
    pub async fn apply(&self, pool: &SqlitePool, id: Uuid, amount: i64) -> Result<()> {
        let memo = self.repo.find_by_id(pool, id).await?;
        let new_applied = memo.applied_amount.amount + amount;
        
        let status = if new_applied >= memo.total.amount {
            CreditMemoStatus::Applied
        } else {
            memo.status
        };
        
        sqlx::query(
            "UPDATE credit_memos SET applied_amount = ?, status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(new_applied)
        .bind(format!("{:?}", status))
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(())
    }
    
    pub async fn void(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, CreditMemoStatus::Void).await
    }
    
    pub async fn create_from_return(&self, pool: &SqlitePool, return_order: &ReturnOrder) -> Result<CreditMemo> {
        let customer_id = return_order.customer_id
            .ok_or_else(|| Error::validation("Return order must have a customer to create credit memo"))?;
        
        let lines: Vec<CreditMemoLine> = return_order.lines.iter().map(|rl| {
            CreditMemoLine {
                id: Uuid::new_v4(),
                credit_memo_id: Uuid::nil(),
                product_id: Some(rl.product_id),
                description: rl.description.clone(),
                quantity: rl.quantity_approved,
                unit_price: rl.unit_price.clone(),
                line_total: rl.credit_amount.clone(),
            }
        }).collect();
        
        let memo = CreditMemo {
            base: BaseEntity::new(),
            memo_number: String::new(),
            customer_id,
            return_order_id: Some(return_order.base.id),
            invoice_id: return_order.original_invoice_id,
            memo_date: Utc::now(),
            lines,
            subtotal: Money::zero(Currency::USD),
            tax_amount: Money::zero(Currency::USD),
            total: return_order.total_credit.clone(),
            status: CreditMemoStatus::Draft,
            applied_amount: Money::zero(Currency::USD),
            reason: Some(format!("{:?}", return_order.reason)),
        };
        
        self.create(pool, memo).await
    }
}

pub struct RefundService;
impl RefundService {
    pub fn new() -> Self { Self }
    
    pub async fn create(
        pool: &SqlitePool,
        customer_id: Uuid,
        credit_memo_id: Option<Uuid>,
        return_order_id: Option<Uuid>,
        amount: i64,
        method: RefundMethod,
    ) -> Result<Refund> {
        let now = chrono::Utc::now();
        let refund_number = format!("REF-{}", now.format("%Y%m%d%H%M%S"));
        
        let refund = Refund {
            id: Uuid::new_v4(),
            refund_number: refund_number.clone(),
            customer_id,
            credit_memo_id,
            return_order_id,
            refund_date: now,
            amount: Money::new(amount, Currency::USD),
            method,
            reference: None,
            status: RefundStatus::Pending,
            processed_at: None,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO refunds (id, refund_number, customer_id, credit_memo_id, return_order_id, refund_date, amount, method, reference, status, processed_at, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'Pending', NULL, ?)"
        )
        .bind(refund.id.to_string())
        .bind(&refund.refund_number)
        .bind(refund.customer_id.to_string())
        .bind(refund.credit_memo_id.map(|id| id.to_string()))
        .bind(refund.return_order_id.map(|id| id.to_string()))
        .bind(refund.refund_date.to_rfc3339())
        .bind(refund.amount.amount)
        .bind(format!("{:?}", refund.method))
        .bind(&refund.reference)
        .bind(refund.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(refund)
    }
    
    pub async fn process(pool: &SqlitePool, id: Uuid, reference: Option<&str>) -> Result<Refund> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE refunds SET status = 'Completed', reference = COALESCE(?, reference), processed_at = ? WHERE id = ?"
        )
        .bind(reference)
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get(pool, id).await
    }
    
    pub async fn get(pool: &SqlitePool, id: Uuid) -> Result<Refund> {
        let row = sqlx::query_as::<_, RefundRow>(
            "SELECT id, refund_number, customer_id, credit_memo_id, return_order_id, refund_date, amount, method, reference, status, processed_at, created_at FROM refunds WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("Refund", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    pub async fn list(pool: &SqlitePool, customer_id: Option<Uuid>) -> Result<Vec<Refund>> {
        let rows = match customer_id {
            Some(cid) => {
                sqlx::query_as::<_, RefundRow>(
                    "SELECT id, refund_number, customer_id, credit_memo_id, return_order_id, refund_date, amount, method, reference, status, processed_at, created_at FROM refunds WHERE customer_id = ? ORDER BY created_at DESC"
                )
                .bind(cid.to_string())
                .fetch_all(pool)
                .await
            }
            None => {
                sqlx::query_as::<_, RefundRow>(
                    "SELECT id, refund_number, customer_id, credit_memo_id, return_order_id, refund_date, amount, method, reference, status, processed_at, created_at FROM refunds ORDER BY created_at DESC"
                )
                .fetch_all(pool)
                .await
            }
        }.map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct RefundRow {
    id: String,
    refund_number: String,
    customer_id: String,
    credit_memo_id: Option<String>,
    return_order_id: Option<String>,
    refund_date: String,
    amount: i64,
    method: String,
    reference: Option<String>,
    status: String,
    processed_at: Option<String>,
    created_at: String,
}

impl From<RefundRow> for Refund {
    fn from(r: RefundRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            refund_number: r.refund_number,
            customer_id: Uuid::parse_str(&r.customer_id).unwrap_or_default(),
            credit_memo_id: r.credit_memo_id.and_then(|id| Uuid::parse_str(&id).ok()),
            return_order_id: r.return_order_id.and_then(|id| Uuid::parse_str(&id).ok()),
            refund_date: chrono::DateTime::parse_from_rfc3339(&r.refund_date)
                .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            amount: Money::new(r.amount, Currency::USD),
            method: match r.method.as_str() {
                "Check" => RefundMethod::Check,
                "BankTransfer" => RefundMethod::BankTransfer,
                "StoreCredit" => RefundMethod::StoreCredit,
                _ => RefundMethod::OriginalPayment,
            },
            reference: r.reference,
            status: match r.status.as_str() {
                "Processing" => RefundStatus::Processing,
                "Completed" => RefundStatus::Completed,
                "Failed" => RefundStatus::Failed,
                "Cancelled" => RefundStatus::Cancelled,
                _ => RefundStatus::Pending,
            },
            processed_at: r.processed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

pub struct ReturnPolicyService;
impl ReturnPolicyService {
    pub fn new() -> Self { Self }
    
    pub async fn create(
        pool: &SqlitePool,
        name: &str,
        description: Option<&str>,
        return_window_days: i32,
        requires_receipt: bool,
        requires_original_packaging: bool,
        restocking_fee_percent: f64,
        allows_exchange: bool,
        allows_refund: bool,
        allows_store_credit: bool,
    ) -> Result<ReturnPolicy> {
        let now = chrono::Utc::now();
        let policy = ReturnPolicy {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            return_window_days,
            requires_receipt,
            requires_original_packaging,
            restocking_fee_percent,
            allows_exchange,
            allows_refund,
            allows_store_credit,
            excluded_categories: None,
            status: erp_core::Status::Active,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO return_policies (id, name, description, return_window_days, requires_receipt, requires_original_packaging, restocking_fee_percent, allows_exchange, allows_refund, allows_store_credit, excluded_categories, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, 'Active', ?)"
        )
        .bind(policy.id.to_string())
        .bind(&policy.name)
        .bind(&policy.description)
        .bind(policy.return_window_days)
        .bind(policy.requires_receipt as i32)
        .bind(policy.requires_original_packaging as i32)
        .bind(policy.restocking_fee_percent)
        .bind(policy.allows_exchange as i32)
        .bind(policy.allows_refund as i32)
        .bind(policy.allows_store_credit as i32)
        .bind(policy.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(policy)
    }
    
    pub async fn list(pool: &SqlitePool) -> Result<Vec<ReturnPolicy>> {
        let rows = sqlx::query_as::<_, ReturnPolicyRow>(
            "SELECT id, name, description, return_window_days, requires_receipt, requires_original_packaging, restocking_fee_percent, allows_exchange, allows_refund, allows_store_credit, excluded_categories, status, created_at FROM return_policies WHERE status = 'Active' ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct ReturnPolicyRow {
    id: String,
    name: String,
    description: Option<String>,
    return_window_days: i32,
    requires_receipt: i32,
    requires_original_packaging: i32,
    restocking_fee_percent: f64,
    allows_exchange: i32,
    allows_refund: i32,
    allows_store_credit: i32,
    excluded_categories: Option<String>,
    status: String,
    created_at: String,
}

impl From<ReturnPolicyRow> for ReturnPolicy {
    fn from(r: ReturnPolicyRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            name: r.name,
            description: r.description,
            return_window_days: r.return_window_days,
            requires_receipt: r.requires_receipt != 0,
            requires_original_packaging: r.requires_original_packaging != 0,
            restocking_fee_percent: r.restocking_fee_percent,
            allows_exchange: r.allows_exchange != 0,
            allows_refund: r.allows_refund != 0,
            allows_store_credit: r.allows_store_credit != 0,
            excluded_categories: r.excluded_categories,
            status: match r.status.as_str() {
                "Inactive" => erp_core::Status::Inactive,
                _ => erp_core::Status::Active,
            },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}
