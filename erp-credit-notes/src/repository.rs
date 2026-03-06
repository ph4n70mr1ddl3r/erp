use anyhow::Result;
use chrono::Utc;
use erp_core::{BaseEntity, Currency, Money, Paginated, Pagination};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

pub struct CreditNoteRepository;

impl CreditNoteRepository {
    pub async fn create(pool: &SqlitePool, credit_note: &CreditNote) -> Result<()> {
        let base = &credit_note.base;
        
        sqlx::query(
            r#"
            INSERT INTO credit_notes (
                id, credit_note_number, customer_id, invoice_id, credit_note_date,
                subtotal, subtotal_currency, tax_amount, tax_currency, total, total_currency,
                reason, notes, status, applied_amount, applied_currency,
                created_at, updated_at, created_by, updated_by
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(base.id.to_string())
        .bind(&credit_note.credit_note_number)
        .bind(credit_note.customer_id.to_string())
        .bind(credit_note.invoice_id.map(|id| id.to_string()))
        .bind(credit_note.credit_note_date.to_rfc3339())
        .bind(credit_note.subtotal.amount)
        .bind(credit_note.subtotal.currency.to_string())
        .bind(credit_note.tax_amount.amount)
        .bind(credit_note.tax_amount.currency.to_string())
        .bind(credit_note.total.amount)
        .bind(credit_note.total.currency.to_string())
        .bind(serde_json::to_string(&credit_note.reason)?)
        .bind(&credit_note.notes)
        .bind(serde_json::to_string(&credit_note.status)?)
        .bind(credit_note.applied_amount.amount)
        .bind(credit_note.applied_amount.currency.to_string())
        .bind(base.created_at.to_rfc3339())
        .bind(base.updated_at.to_rfc3339())
        .bind(base.created_by.map(|id| id.to_string()))
        .bind(base.updated_by.map(|id| id.to_string()))
        .execute(pool)
        .await?;

        for line in &credit_note.lines {
            Self::create_line(pool, &credit_note.base.id, line).await?;
        }

        Ok(())
    }

    async fn create_line(pool: &SqlitePool, credit_note_id: &Uuid, line: &CreditNoteLine) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO credit_note_lines (
                id, credit_note_id, product_id, description, quantity,
                unit_price, unit_price_currency, line_total, line_total_currency
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(line.id.to_string())
        .bind(credit_note_id.to_string())
        .bind(line.product_id.map(|id| id.to_string()))
        .bind(&line.description)
        .bind(line.quantity)
        .bind(line.unit_price.amount)
        .bind(line.unit_price.currency.to_string())
        .bind(line.line_total.amount)
        .bind(line.line_total.currency.to_string())
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<CreditNote>> {
        let row = sqlx::query_as::<_, CreditNoteRow>(
            "SELECT * FROM credit_notes WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;

        match row {
            Some(r) => {
                let lines = Self::get_lines(pool, id).await?;
                Ok(Some(r.into_model(lines)?))
            }
            None => Ok(None),
        }
    }

    async fn get_lines(pool: &SqlitePool, credit_note_id: Uuid) -> Result<Vec<CreditNoteLine>> {
        let rows = sqlx::query_as::<_, CreditNoteLineRow>(
            "SELECT * FROM credit_note_lines WHERE credit_note_id = ?"
        )
        .bind(credit_note_id.to_string())
        .fetch_all(pool)
        .await?;

        rows.into_iter().map(|r| r.into_model()).collect()
    }

    pub async fn list(pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<CreditNote>> {
        let offset = pagination.offset();
        let limit = pagination.per_page as i32;

        let rows = sqlx::query_as::<_, CreditNoteRow>(
            "SELECT * FROM credit_notes ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;

        let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM credit_notes")
            .fetch_one(pool)
            .await?;

        let mut items = Vec::new();
        for row in rows {
            let lines = Self::get_lines(pool, row.id.parse()?).await?;
            items.push(row.into_model(lines)?);
        }

        Ok(Paginated::new(items, total as u64, pagination))
    }

    pub async fn list_by_customer(pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<CreditNote>> {
        let rows = sqlx::query_as::<_, CreditNoteRow>(
            "SELECT * FROM credit_notes WHERE customer_id = ? ORDER BY created_at DESC"
        )
        .bind(customer_id.to_string())
        .fetch_all(pool)
        .await?;

        let mut items = Vec::new();
        for row in rows {
            let lines = Self::get_lines(pool, row.id.parse()?).await?;
            items.push(row.into_model(lines)?);
        }

        Ok(items)
    }

    pub async fn update_status(pool: &SqlitePool, id: Uuid, status: &CreditNoteStatus) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE credit_notes SET status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(serde_json::to_string(status)?)
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update_applied_amount(pool: &SqlitePool, id: Uuid, amount: i64) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE credit_notes SET applied_amount = ?, updated_at = ? WHERE id = ?"
        )
        .bind(amount)
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn create_application(pool: &SqlitePool, application: &CreditNoteApplication) -> Result<()> {
        sqlx::query(
            r#"
            INSERT INTO credit_note_applications (
                id, credit_note_id, invoice_id, amount, currency, applied_at
            ) VALUES (?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(application.id.to_string())
        .bind(application.credit_note_id.to_string())
        .bind(application.invoice_id.to_string())
        .bind(application.amount.amount)
        .bind(application.amount.currency.to_string())
        .bind(application.applied_at.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_next_number(pool: &SqlitePool) -> Result<String> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM credit_notes")
            .fetch_one(pool)
            .await?;
        Ok(format!("CN-{:06}", count + 1))
    }
}

#[derive(sqlx::FromRow)]
struct CreditNoteRow {
    id: String,
    credit_note_number: String,
    customer_id: String,
    invoice_id: Option<String>,
    credit_note_date: String,
    subtotal: i64,
    subtotal_currency: String,
    tax_amount: i64,
    tax_currency: String,
    total: i64,
    total_currency: String,
    reason: String,
    notes: Option<String>,
    status: String,
    applied_amount: i64,
    applied_currency: String,
    created_at: String,
    updated_at: String,
    created_by: Option<String>,
    updated_by: Option<String>,
}

impl CreditNoteRow {
    fn into_model(self, lines: Vec<CreditNoteLine>) -> Result<CreditNote> {
        let currency: Currency = self.total_currency.parse().map_err(|e: String| anyhow::anyhow!(e))?;
        let reason: CreditNoteReason = serde_json::from_str(&self.reason)?;
        let status: CreditNoteStatus = serde_json::from_str(&self.status)?;

        Ok(CreditNote {
            base: BaseEntity {
                id: self.id.parse()?,
                created_at: self.created_at.parse()?,
                updated_at: self.updated_at.parse()?,
                created_by: self.created_by.map(|s| s.parse()).transpose()?,
                updated_by: self.updated_by.map(|s| s.parse()).transpose()?,
            },
            credit_note_number: self.credit_note_number,
            customer_id: self.customer_id.parse()?,
            invoice_id: self.invoice_id.map(|s| s.parse()).transpose()?,
            credit_note_date: self.credit_note_date.parse()?,
            lines,
            subtotal: Money::new(self.subtotal, self.subtotal_currency.parse().map_err(|e: String| anyhow::anyhow!(e))?),
            tax_amount: Money::new(self.tax_amount, self.tax_currency.parse().map_err(|e: String| anyhow::anyhow!(e))?),
            total: Money::new(self.total, currency.clone()),
            reason,
            notes: self.notes,
            status,
            applied_amount: Money::new(self.applied_amount, self.applied_currency.parse().map_err(|e: String| anyhow::anyhow!(e))?),
        })
    }
}

#[derive(sqlx::FromRow)]
struct CreditNoteLineRow {
    #[allow(dead_code)]
    credit_note_id: String,
    id: String,
    product_id: Option<String>,
    description: String,
    quantity: i64,
    unit_price: i64,
    unit_price_currency: String,
    line_total: i64,
    line_total_currency: String,
}

impl CreditNoteLineRow {
    fn into_model(self) -> Result<CreditNoteLine> {
        Ok(CreditNoteLine {
            id: self.id.parse()?,
            product_id: self.product_id.map(|s| s.parse()).transpose()?,
            description: self.description,
            quantity: self.quantity,
            unit_price: Money::new(self.unit_price, self.unit_price_currency.parse().map_err(|e: String| anyhow::anyhow!(e))?),
            line_total: Money::new(self.line_total, self.line_total_currency.parse().map_err(|e: String| anyhow::anyhow!(e))?),
        })
    }
}
