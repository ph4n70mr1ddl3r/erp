use async_trait::async_trait;
use chrono::Utc;
use erp_core::{BaseEntity, Error, Paginated, Pagination, Result, Status};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::PaymentTerm;

#[derive(sqlx::FromRow)]
struct PaymentTermRow {
    id: String,
    code: String,
    name: String,
    description: Option<String>,
    due_days: i32,
    discount_days: Option<i32>,
    discount_percent: Option<f64>,
    is_default: i32,
    status: String,
    created_at: String,
    updated_at: String,
}

impl PaymentTermRow {
    fn into_payment_term(self) -> PaymentTerm {
        PaymentTerm {
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
            code: self.code,
            name: self.name,
            description: self.description,
            due_days: self.due_days,
            discount_days: self.discount_days,
            discount_percent: self.discount_percent,
            is_default: self.is_default != 0,
            status: match self.status.as_str() {
                "Inactive" => Status::Inactive,
                _ => Status::Active,
            },
        }
    }
}

pub struct SqlitePaymentTermRepository;

#[async_trait]
pub trait PaymentTermRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<PaymentTerm>;
    async fn find_by_code(&self, pool: &SqlitePool, code: &str) -> Result<PaymentTerm>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<PaymentTerm>>;
    async fn find_default(&self, pool: &SqlitePool) -> Result<Option<PaymentTerm>>;
    async fn create(&self, pool: &SqlitePool, term: PaymentTerm) -> Result<PaymentTerm>;
    async fn update(&self, pool: &SqlitePool, term: PaymentTerm) -> Result<PaymentTerm>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn set_default(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

#[async_trait]
impl PaymentTermRepository for SqlitePaymentTermRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<PaymentTerm> {
        let row = sqlx::query_as::<_, PaymentTermRow>(
            "SELECT id, code, name, description, due_days, discount_days, discount_percent, is_default, status, created_at, updated_at
             FROM payment_terms WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("PaymentTerm", &id.to_string()))?;
        Ok(row.into_payment_term())
    }

    async fn find_by_code(&self, pool: &SqlitePool, code: &str) -> Result<PaymentTerm> {
        let row = sqlx::query_as::<_, PaymentTermRow>(
            "SELECT id, code, name, description, due_days, discount_days, discount_percent, is_default, status, created_at, updated_at
             FROM payment_terms WHERE code = ?"
        )
        .bind(code)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("PaymentTerm", code))?;
        Ok(row.into_payment_term())
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<PaymentTerm>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM payment_terms WHERE status != 'Deleted'")
            .fetch_one(pool)
            .await?;
        let rows = sqlx::query_as::<_, PaymentTermRow>(
            "SELECT id, code, name, description, due_days, discount_days, discount_percent, is_default, status, created_at, updated_at
             FROM payment_terms WHERE status != 'Deleted' ORDER BY is_default DESC, code LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await?;
        Ok(Paginated::new(
            rows.into_iter().map(|r| r.into_payment_term()).collect(),
            count.0 as u64,
            pagination,
        ))
    }

    async fn find_default(&self, pool: &SqlitePool) -> Result<Option<PaymentTerm>> {
        let row = sqlx::query_as::<_, PaymentTermRow>(
            "SELECT id, code, name, description, due_days, discount_days, discount_percent, is_default, status, created_at, updated_at
             FROM payment_terms WHERE is_default = 1 AND status = 'Active'"
        )
        .fetch_optional(pool)
        .await?;
        Ok(row.map(|r| r.into_payment_term()))
    }

    async fn create(&self, pool: &SqlitePool, term: PaymentTerm) -> Result<PaymentTerm> {
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO payment_terms (id, code, name, description, due_days, discount_days, discount_percent, is_default, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(term.base.id.to_string())
        .bind(&term.code)
        .bind(&term.name)
        .bind(&term.description)
        .bind(term.due_days)
        .bind(term.discount_days)
        .bind(term.discount_percent)
        .bind(if term.is_default { 1 } else { 0 })
        .bind(format!("{:?}", term.status))
        .bind(term.base.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;
        Ok(term)
    }

    async fn update(&self, pool: &SqlitePool, term: PaymentTerm) -> Result<PaymentTerm> {
        let now = Utc::now();
        let id = term.base.id.to_string();
        let rows = sqlx::query(
            "UPDATE payment_terms SET code=?, name=?, description=?, due_days=?, discount_days=?, discount_percent=?, is_default=?, status=?, updated_at=? WHERE id=?"
        )
        .bind(&term.code)
        .bind(&term.name)
        .bind(&term.description)
        .bind(term.due_days)
        .bind(term.discount_days)
        .bind(term.discount_percent)
        .bind(if term.is_default { 1 } else { 0 })
        .bind(format!("{:?}", term.status))
        .bind(now.to_rfc3339())
        .bind(&id)
        .execute(pool)
        .await?;
        if rows.rows_affected() == 0 {
            return Err(Error::not_found("PaymentTerm", &id));
        }
        Ok(term)
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let rows = sqlx::query("UPDATE payment_terms SET status = 'Deleted', updated_at = ? WHERE id = ?")
            .bind(Utc::now().to_rfc3339())
            .bind(id.to_string())
            .execute(pool)
            .await?;
        if rows.rows_affected() == 0 {
            return Err(Error::not_found("PaymentTerm", &id.to_string()));
        }
        Ok(())
    }

    async fn set_default(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let mut tx = pool.begin().await?;
        sqlx::query("UPDATE payment_terms SET is_default = 0, updated_at = ?")
            .bind(Utc::now().to_rfc3339())
            .execute(&mut *tx)
            .await?;
        let rows = sqlx::query("UPDATE payment_terms SET is_default = 1, updated_at = ? WHERE id = ?")
            .bind(Utc::now().to_rfc3339())
            .bind(id.to_string())
            .execute(&mut *tx)
            .await?;
        tx.commit().await?;
        if rows.rows_affected() == 0 {
            return Err(Error::not_found("PaymentTerm", &id.to_string()));
        }
        Ok(())
    }
}
