use async_trait::async_trait;
use sqlx::SqlitePool;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity};
use crate::models::*;
use uuid::Uuid;

#[async_trait]
pub trait ReturnRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ReturnOrder>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ReturnOrder>>;
    async fn create(&self, pool: &SqlitePool, order: ReturnOrder) -> Result<ReturnOrder>;
    async fn update(&self, pool: &SqlitePool, order: ReturnOrder) -> Result<ReturnOrder>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: ReturnStatus) -> Result<()>;
}

pub struct SqliteReturnRepository;

#[async_trait]
impl ReturnRepository for SqliteReturnRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<ReturnOrder> {
        let row = sqlx::query_as::<_, ReturnOrderRow>(
            "SELECT * FROM return_orders WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("ReturnOrder", &id.to_string()))?;
        
        let lines = self.get_lines(pool, id).await?;
        Ok(row.into_order(lines))
    }
    
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<ReturnOrder>> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM return_orders")
            .fetch_one(pool)
            .await
            .map_err(Error::Database)?;
        
        let offset = (pagination.page.saturating_sub(1)) * pagination.per_page;
        let rows = sqlx::query_as::<_, ReturnOrderRow>(
            "SELECT * FROM return_orders ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        let mut orders = Vec::new();
        for row in rows {
            let lines = self.get_lines(pool, Uuid::parse_str(&row.id).unwrap_or_default()).await?;
            orders.push(row.into_order(lines));
        }
        
        Ok(Paginated::new(orders, count as u64, pagination))
    }
    
    async fn create(&self, pool: &SqlitePool, order: ReturnOrder) -> Result<ReturnOrder> {
        sqlx::query(
            "INSERT INTO return_orders (id, return_number, return_type, customer_id, vendor_id, original_order_id, original_invoice_id, request_date, received_date, processed_date, reason, notes, status, total_credit, warehouse_id, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(order.base.id.to_string())
        .bind(&order.return_number)
        .bind(format!("{:?}", order.return_type))
        .bind(order.customer_id.map(|id| id.to_string()))
        .bind(order.vendor_id.map(|id| id.to_string()))
        .bind(order.original_order_id.map(|id| id.to_string()))
        .bind(order.original_invoice_id.map(|id| id.to_string()))
        .bind(order.request_date.to_rfc3339())
        .bind(order.received_date.map(|d| d.to_rfc3339()))
        .bind(order.processed_date.map(|d| d.to_rfc3339()))
        .bind(format!("{:?}", order.reason))
        .bind(&order.notes)
        .bind(format!("{:?}", order.status))
        .bind(order.total_credit.amount)
        .bind(order.warehouse_id.map(|id| id.to_string()))
        .bind(order.base.created_at.to_rfc3339())
        .bind(order.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        for line in &order.lines {
            self.create_line(pool, line).await?;
        }
        
        Ok(order)
    }
    
    async fn update(&self, pool: &SqlitePool, order: ReturnOrder) -> Result<ReturnOrder> {
        sqlx::query(
            "UPDATE return_orders SET return_type = ?, customer_id = ?, vendor_id = ?, reason = ?, notes = ?, status = ?, total_credit = ?, updated_at = ? WHERE id = ?"
        )
        .bind(format!("{:?}", order.return_type))
        .bind(order.customer_id.map(|id| id.to_string()))
        .bind(order.vendor_id.map(|id| id.to_string()))
        .bind(format!("{:?}", order.reason))
        .bind(&order.notes)
        .bind(format!("{:?}", order.status))
        .bind(order.total_credit.amount)
        .bind(order.base.updated_at.to_rfc3339())
        .bind(order.base.id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(order)
    }
    
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: ReturnStatus) -> Result<()> {
        let now = chrono::Utc::now();
        let processed_date = matches!(status, ReturnStatus::Processed).then_some(now);
        
        sqlx::query(
            "UPDATE return_orders SET status = ?, processed_date = COALESCE(?, processed_date), updated_at = ? WHERE id = ?"
        )
        .bind(format!("{:?}", status))
        .bind(processed_date.map(|d| d.to_rfc3339()))
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

impl SqliteReturnRepository {
    async fn get_lines(&self, pool: &SqlitePool, return_order_id: Uuid) -> Result<Vec<ReturnLine>> {
        let rows = sqlx::query_as::<_, ReturnLineRow>(
            "SELECT * FROM return_lines WHERE return_order_id = ?"
        )
        .bind(return_order_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    async fn create_line(&self, pool: &SqlitePool, line: &ReturnLine) -> Result<()> {
        sqlx::query(
            "INSERT INTO return_lines (id, return_order_id, product_id, description, quantity_requested, quantity_received, quantity_approved, unit_price, reason, disposition, condition_type, inspection_notes, credit_amount)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(line.id.to_string())
        .bind(line.return_order_id.to_string())
        .bind(line.product_id.to_string())
        .bind(&line.description)
        .bind(line.quantity_requested)
        .bind(line.quantity_received)
        .bind(line.quantity_approved)
        .bind(line.unit_price.amount)
        .bind(format!("{:?}", line.reason))
        .bind(format!("{:?}", line.disposition))
        .bind(format!("{:?}", line.condition))
        .bind(&line.inspection_notes)
        .bind(line.credit_amount.amount)
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct ReturnOrderRow {
    id: String,
    return_number: String,
    return_type: String,
    customer_id: Option<String>,
    vendor_id: Option<String>,
    original_order_id: Option<String>,
    original_invoice_id: Option<String>,
    request_date: String,
    received_date: Option<String>,
    processed_date: Option<String>,
    reason: String,
    notes: Option<String>,
    status: String,
    total_credit: i64,
    warehouse_id: Option<String>,
    created_at: String,
    updated_at: String,
}

impl ReturnOrderRow {
    fn into_order(self, lines: Vec<ReturnLine>) -> ReturnOrder {
        ReturnOrder {
            base: BaseEntity {
                id: Uuid::parse_str(&self.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            return_number: self.return_number,
            return_type: match self.return_type.as_str() {
                "VendorReturn" => ReturnType::VendorReturn,
                "InternalReturn" => ReturnType::InternalReturn,
                _ => ReturnType::CustomerReturn,
            },
            customer_id: self.customer_id.and_then(|id| Uuid::parse_str(&id).ok()),
            vendor_id: self.vendor_id.and_then(|id| Uuid::parse_str(&id).ok()),
            original_order_id: self.original_order_id.and_then(|id| Uuid::parse_str(&id).ok()),
            original_invoice_id: self.original_invoice_id.and_then(|id| Uuid::parse_str(&id).ok()),
            request_date: chrono::DateTime::parse_from_rfc3339(&self.request_date)
                .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            received_date: self.received_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            processed_date: self.processed_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            reason: match self.reason.as_str() {
                "WrongItem" => ReturnReason::WrongItem,
                "NotAsDescribed" => ReturnReason::NotAsDescribed,
                "Damaged" => ReturnReason::Damaged,
                "ChangedMind" => ReturnReason::ChangedMind,
                "Warranty" => ReturnReason::Warranty,
                "Recall" => ReturnReason::Recall,
                "Other" => ReturnReason::Other,
                _ => ReturnReason::Defective,
            },
            notes: self.notes,
            lines,
            status: match self.status.as_str() {
                "Requested" => ReturnStatus::Requested,
                "Approved" => ReturnStatus::Approved,
                "Received" => ReturnStatus::Received,
                "Inspected" => ReturnStatus::Inspected,
                "Processed" => ReturnStatus::Processed,
                "Rejected" => ReturnStatus::Rejected,
                "Cancelled" => ReturnStatus::Cancelled,
                _ => ReturnStatus::Draft,
            },
            total_credit: erp_core::Money::new(self.total_credit, erp_core::Currency::USD),
            warehouse_id: self.warehouse_id.and_then(|id| Uuid::parse_str(&id).ok()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct ReturnLineRow {
    id: String,
    return_order_id: String,
    product_id: String,
    description: String,
    quantity_requested: i64,
    quantity_received: i64,
    quantity_approved: i64,
    unit_price: i64,
    reason: String,
    disposition: String,
    condition_type: String,
    inspection_notes: Option<String>,
    credit_amount: i64,
}

impl From<ReturnLineRow> for ReturnLine {
    fn from(r: ReturnLineRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            return_order_id: Uuid::parse_str(&r.return_order_id).unwrap_or_default(),
            product_id: Uuid::parse_str(&r.product_id).unwrap_or_default(),
            description: r.description,
            quantity_requested: r.quantity_requested,
            quantity_received: r.quantity_received,
            quantity_approved: r.quantity_approved,
            unit_price: erp_core::Money::new(r.unit_price, erp_core::Currency::USD),
            reason: match r.reason.as_str() {
                "WrongItem" => ReturnReason::WrongItem,
                "NotAsDescribed" => ReturnReason::NotAsDescribed,
                "Damaged" => ReturnReason::Damaged,
                "ChangedMind" => ReturnReason::ChangedMind,
                "Warranty" => ReturnReason::Warranty,
                "Recall" => ReturnReason::Recall,
                "Other" => ReturnReason::Other,
                _ => ReturnReason::Defective,
            },
            disposition: match r.disposition.as_str() {
                "Refund" => ReturnDisposition::Refund,
                "Replace" => ReturnDisposition::Replace,
                "Repair" => ReturnDisposition::Repair,
                "Scrap" => ReturnDisposition::Scrap,
                "ReturnToVendor" => ReturnDisposition::ReturnToVendor,
                "Credit" => ReturnDisposition::Credit,
                _ => ReturnDisposition::Restock,
            },
            condition: match r.condition_type.as_str() {
                "UsedGood" => ItemCondition::UsedGood,
                "UsedFair" => ItemCondition::UsedFair,
                "Damaged" => ItemCondition::Damaged,
                "Defective" => ItemCondition::Defective,
                _ => ItemCondition::New,
            },
            inspection_notes: r.inspection_notes,
            credit_amount: erp_core::Money::new(r.credit_amount, erp_core::Currency::USD),
        }
    }
}

#[async_trait]
pub trait CreditMemoRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<CreditMemo>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<CreditMemo>>;
    async fn create(&self, pool: &SqlitePool, memo: CreditMemo) -> Result<CreditMemo>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: CreditMemoStatus) -> Result<()>;
}

pub struct SqliteCreditMemoRepository;

#[async_trait]
impl CreditMemoRepository for SqliteCreditMemoRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<CreditMemo> {
        let row = sqlx::query_as::<_, CreditMemoRow>(
            "SELECT * FROM credit_memos WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("CreditMemo", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<CreditMemo>> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM credit_memos")
            .fetch_one(pool)
            .await
            .map_err(Error::Database)?;
        
        let offset = (pagination.page.saturating_sub(1)) * pagination.per_page;
        let rows = sqlx::query_as::<_, CreditMemoRow>(
            "SELECT * FROM credit_memos ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(Paginated::new(rows.into_iter().map(|r| r.into()).collect(), count as u64, pagination))
    }
    
    async fn create(&self, pool: &SqlitePool, memo: CreditMemo) -> Result<CreditMemo> {
        sqlx::query(
            "INSERT INTO credit_memos (id, memo_number, customer_id, return_order_id, invoice_id, memo_date, subtotal, tax_amount, total, status, applied_amount, reason, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(memo.base.id.to_string())
        .bind(&memo.memo_number)
        .bind(memo.customer_id.to_string())
        .bind(memo.return_order_id.map(|id| id.to_string()))
        .bind(memo.invoice_id.map(|id| id.to_string()))
        .bind(memo.memo_date.to_rfc3339())
        .bind(memo.subtotal.amount)
        .bind(memo.tax_amount.amount)
        .bind(memo.total.amount)
        .bind(format!("{:?}", memo.status))
        .bind(memo.applied_amount.amount)
        .bind(&memo.reason)
        .bind(memo.base.created_at.to_rfc3339())
        .bind(memo.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        for line in &memo.lines {
            sqlx::query(
                "INSERT INTO credit_memo_lines (id, credit_memo_id, product_id, description, quantity, unit_price, line_total)
                 VALUES (?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(line.id.to_string())
            .bind(line.credit_memo_id.to_string())
            .bind(line.product_id.map(|id| id.to_string()))
            .bind(&line.description)
            .bind(line.quantity)
            .bind(line.unit_price.amount)
            .bind(line.line_total.amount)
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        }
        
        Ok(memo)
    }
    
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: CreditMemoStatus) -> Result<()> {
        let now = chrono::Utc::now();
        sqlx::query(
            "UPDATE credit_memos SET status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(format!("{:?}", status))
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct CreditMemoRow {
    id: String,
    memo_number: String,
    customer_id: String,
    return_order_id: Option<String>,
    invoice_id: Option<String>,
    memo_date: String,
    subtotal: i64,
    tax_amount: i64,
    total: i64,
    status: String,
    applied_amount: i64,
    reason: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<CreditMemoRow> for CreditMemo {
    fn from(r: CreditMemoRow) -> Self {
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
            memo_number: r.memo_number,
            customer_id: Uuid::parse_str(&r.customer_id).unwrap_or_default(),
            return_order_id: r.return_order_id.and_then(|id| Uuid::parse_str(&id).ok()),
            invoice_id: r.invoice_id.and_then(|id| Uuid::parse_str(&id).ok()),
            memo_date: chrono::DateTime::parse_from_rfc3339(&r.memo_date)
                .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            lines: vec![],
            subtotal: erp_core::Money::new(r.subtotal, erp_core::Currency::USD),
            tax_amount: erp_core::Money::new(r.tax_amount, erp_core::Currency::USD),
            total: erp_core::Money::new(r.total, erp_core::Currency::USD),
            status: match r.status.as_str() {
                "Issued" => CreditMemoStatus::Issued,
                "Applied" => CreditMemoStatus::Applied,
                "Void" => CreditMemoStatus::Void,
                _ => CreditMemoStatus::Draft,
            },
            applied_amount: erp_core::Money::new(r.applied_amount, erp_core::Currency::USD),
            reason: r.reason,
        }
    }
}
