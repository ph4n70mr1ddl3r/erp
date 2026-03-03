use anyhow::Result;
use chrono::Utc;
use erp_core::{BaseEntity, Currency, Money, Paginated, Pagination};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{MatchStatus, VendorBill, VendorBillLine, VendorBillPayment, VendorBillStatus};

pub struct VendorBillRepository;

impl VendorBillRepository {
    pub async fn find_by_id(pool: &SqlitePool, id: Uuid) -> Result<VendorBill> {
        let row = sqlx::query_as::<_, VendorBillRow>(
            "SELECT id, bill_number, vendor_invoice_number, vendor_id, purchase_order_id,
                    bill_date, due_date, subtotal, tax_amount, total, amount_paid,
                    status, match_status, notes, created_at, updated_at
             FROM vendor_bills WHERE id = ?",
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| anyhow::anyhow!("VendorBill not found: {}", id))?;

        let lines = Self::find_lines(pool, id).await?;
        Ok(row.into_model(lines))
    }

    pub async fn find_all(pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<VendorBill>> {
        let offset = (pagination.page - 1) * pagination.per_page;
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM vendor_bills")
            .fetch_one(pool)
            .await?;

        let rows = sqlx::query_as::<_, VendorBillRow>(
            "SELECT id, bill_number, vendor_invoice_number, vendor_id, purchase_order_id,
                    bill_date, due_date, subtotal, tax_amount, total, amount_paid,
                    status, match_status, notes, created_at, updated_at
             FROM vendor_bills
             ORDER BY created_at DESC
             LIMIT ? OFFSET ?",
        )
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await?;

        let mut items = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id)?;
            let lines = Self::find_lines(pool, id).await?;
            items.push(row.into_model(lines));
        }

        Ok(Paginated::new(items, count.0 as u64, pagination))
    }

    pub async fn find_by_vendor(pool: &SqlitePool, vendor_id: Uuid) -> Result<Vec<VendorBill>> {
        let rows = sqlx::query_as::<_, VendorBillRow>(
            "SELECT id, bill_number, vendor_invoice_number, vendor_id, purchase_order_id,
                    bill_date, due_date, subtotal, tax_amount, total, amount_paid,
                    status, match_status, notes, created_at, updated_at
             FROM vendor_bills WHERE vendor_id = ?
             ORDER BY bill_date DESC",
        )
        .bind(vendor_id.to_string())
        .fetch_all(pool)
        .await?;

        let mut result = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id)?;
            let lines = Self::find_lines(pool, id).await?;
            result.push(row.into_model(lines));
        }
        Ok(result)
    }

    pub async fn find_lines(pool: &SqlitePool, bill_id: Uuid) -> Result<Vec<VendorBillLine>> {
        let rows = sqlx::query_as::<_, VendorBillLineRow>(
            "SELECT id, bill_id, po_line_id, product_id, description, quantity,
                    unit_price, tax_rate, line_total, match_quantity, match_status
             FROM vendor_bill_lines WHERE bill_id = ?",
        )
        .bind(bill_id.to_string())
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn create(pool: &SqlitePool, bill: VendorBill) -> Result<()> {
        sqlx::query(
            "INSERT INTO vendor_bills (id, bill_number, vendor_invoice_number, vendor_id,
             purchase_order_id, bill_date, due_date, subtotal, tax_amount, total, amount_paid,
             status, match_status, notes, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(bill.base.id.to_string())
        .bind(&bill.bill_number)
        .bind(&bill.vendor_invoice_number)
        .bind(bill.vendor_id.to_string())
        .bind(bill.purchase_order_id.map(|id| id.to_string()))
        .bind(bill.bill_date.to_rfc3339())
        .bind(bill.due_date.to_rfc3339())
        .bind(bill.subtotal.amount)
        .bind(bill.tax_amount.amount)
        .bind(bill.total.amount)
        .bind(bill.amount_paid.amount)
        .bind(format!("{:?}", bill.status))
        .bind(format!("{:?}", bill.match_status))
        .bind(&bill.notes)
        .bind(bill.base.created_at.to_rfc3339())
        .bind(bill.base.updated_at.to_rfc3339())
        .execute(pool)
        .await?;

        for line in &bill.lines {
            Self::create_line(pool, line).await?;
        }

        Ok(())
    }

    pub async fn create_line(pool: &SqlitePool, line: &VendorBillLine) -> Result<()> {
        sqlx::query(
            "INSERT INTO vendor_bill_lines (id, bill_id, po_line_id, product_id, description,
             quantity, unit_price, tax_rate, line_total, match_quantity, match_status)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(line.id.to_string())
        .bind(line.bill_id.to_string())
        .bind(line.po_line_id.map(|id| id.to_string()))
        .bind(line.product_id.map(|id| id.to_string()))
        .bind(&line.description)
        .bind(line.quantity)
        .bind(line.unit_price.amount)
        .bind(line.tax_rate)
        .bind(line.line_total.amount)
        .bind(line.match_quantity)
        .bind(format!("{:?}", line.match_status))
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update_status(pool: &SqlitePool, id: Uuid, status: VendorBillStatus) -> Result<()> {
        sqlx::query("UPDATE vendor_bills SET status = ?, updated_at = ? WHERE id = ?")
            .bind(format!("{:?}", status))
            .bind(Utc::now().to_rfc3339())
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn update_match_status(pool: &SqlitePool, id: Uuid, status: MatchStatus) -> Result<()> {
        sqlx::query("UPDATE vendor_bills SET match_status = ?, updated_at = ? WHERE id = ?")
            .bind(format!("{:?}", status))
            .bind(Utc::now().to_rfc3339())
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM vendor_bill_lines WHERE bill_id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;
        sqlx::query("DELETE FROM vendor_bills WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn record_payment(pool: &SqlitePool, payment: VendorBillPayment) -> Result<()> {
        sqlx::query(
            "INSERT INTO vendor_bill_payments (id, bill_id, payment_id, amount, applied_at)
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(payment.id.to_string())
        .bind(payment.bill_id.to_string())
        .bind(payment.payment_id.to_string())
        .bind(payment.amount.amount)
        .bind(payment.applied_at.to_rfc3339())
        .execute(pool)
        .await?;

        sqlx::query(
            "UPDATE vendor_bills SET amount_paid = amount_paid + ?, updated_at = ? WHERE id = ?",
        )
        .bind(payment.amount.amount)
        .bind(Utc::now().to_rfc3339())
        .bind(payment.bill_id.to_string())
        .execute(pool)
        .await?;

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct VendorBillRow {
    id: String,
    bill_number: String,
    vendor_invoice_number: String,
    vendor_id: String,
    purchase_order_id: Option<String>,
    bill_date: String,
    due_date: String,
    subtotal: i64,
    tax_amount: i64,
    total: i64,
    amount_paid: i64,
    status: String,
    match_status: String,
    notes: Option<String>,
    created_at: String,
    updated_at: String,
}

impl VendorBillRow {
    fn into_model(self, lines: Vec<VendorBillLine>) -> VendorBill {
        VendorBill {
            base: BaseEntity {
                id: Uuid::parse_str(&self.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at)
                    .map(|d| d.with_timezone(&Utc))
                    .unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            bill_number: self.bill_number,
            vendor_invoice_number: self.vendor_invoice_number,
            vendor_id: Uuid::parse_str(&self.vendor_id).unwrap_or_default(),
            purchase_order_id: self.purchase_order_id.and_then(|id| Uuid::parse_str(&id).ok()),
            bill_date: chrono::DateTime::parse_from_rfc3339(&self.bill_date)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            due_date: chrono::DateTime::parse_from_rfc3339(&self.due_date)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            subtotal: Money::new(self.subtotal, Currency::USD),
            tax_amount: Money::new(self.tax_amount, Currency::USD),
            total: Money::new(self.total, Currency::USD),
            amount_paid: Money::new(self.amount_paid, Currency::USD),
            status: serde_json::from_str(&format!("\"{}\"", self.status)).unwrap_or(VendorBillStatus::Draft),
            match_status: serde_json::from_str(&format!("\"{}\"", self.match_status)).unwrap_or(MatchStatus::Unmatched),
            notes: self.notes,
            lines,
        }
    }
}

#[derive(sqlx::FromRow)]
struct VendorBillLineRow {
    id: String,
    bill_id: String,
    po_line_id: Option<String>,
    product_id: Option<String>,
    description: String,
    quantity: i64,
    unit_price: i64,
    tax_rate: f64,
    line_total: i64,
    match_quantity: i64,
    match_status: String,
}

impl From<VendorBillLineRow> for VendorBillLine {
    fn from(r: VendorBillLineRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            bill_id: Uuid::parse_str(&r.bill_id).unwrap_or_default(),
            po_line_id: r.po_line_id.and_then(|id| Uuid::parse_str(&id).ok()),
            product_id: r.product_id.and_then(|id| Uuid::parse_str(&id).ok()),
            description: r.description,
            quantity: r.quantity,
            unit_price: Money::new(r.unit_price, Currency::USD),
            tax_rate: r.tax_rate,
            line_total: Money::new(r.line_total, Currency::USD),
            match_quantity: r.match_quantity,
            match_status: serde_json::from_str(&format!("\"{}\"", r.match_status)).unwrap_or(MatchStatus::Unmatched),
        }
    }
}
