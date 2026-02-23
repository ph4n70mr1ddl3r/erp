use async_trait::async_trait;
use sqlx::SqlitePool;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Money, Currency};
use crate::models::*;
use uuid::Uuid;

#[async_trait]
pub trait TaxJurisdictionRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<TaxJurisdiction>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<TaxJurisdiction>>;
    async fn create(&self, pool: &SqlitePool, jurisdiction: TaxJurisdiction) -> Result<TaxJurisdiction>;
}

pub struct SqliteTaxJurisdictionRepository;

#[async_trait]
impl TaxJurisdictionRepository for SqliteTaxJurisdictionRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<TaxJurisdiction> {
        let row = sqlx::query_as::<_, TaxJurisdictionRow>(
            "SELECT * FROM tax_jurisdictions WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("TaxJurisdiction", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<TaxJurisdiction>> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tax_jurisdictions")
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e))?;
        
        let offset = (pagination.page.saturating_sub(1)) * pagination.per_page;
        let rows = sqlx::query_as::<_, TaxJurisdictionRow>(
            "SELECT * FROM tax_jurisdictions ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(Paginated::new(rows.into_iter().map(|r| r.into()).collect(), count as u64, pagination))
    }
    
    async fn create(&self, pool: &SqlitePool, jurisdiction: TaxJurisdiction) -> Result<TaxJurisdiction> {
        sqlx::query(
            "INSERT INTO tax_jurisdictions (id, code, name, country_code, state_code, county, city, postal_code_from, postal_code_to, parent_jurisdiction_id, status, effective_from, effective_to, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(jurisdiction.base.id.to_string())
        .bind(&jurisdiction.code)
        .bind(&jurisdiction.name)
        .bind(&jurisdiction.country_code)
        .bind(&jurisdiction.state_code)
        .bind(&jurisdiction.county)
        .bind(&jurisdiction.city)
        .bind(&jurisdiction.postal_code_from)
        .bind(&jurisdiction.postal_code_to)
        .bind(jurisdiction.parent_jurisdiction_id.map(|id| id.to_string()))
        .bind(format!("{:?}", jurisdiction.status))
        .bind(jurisdiction.effective_from.to_rfc3339())
        .bind(jurisdiction.effective_to.map(|d| d.to_rfc3339()))
        .bind(jurisdiction.base.created_at.to_rfc3339())
        .bind(jurisdiction.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(jurisdiction)
    }
}

#[derive(sqlx::FromRow)]
struct TaxJurisdictionRow {
    id: String,
    code: String,
    name: String,
    country_code: String,
    state_code: Option<String>,
    county: Option<String>,
    city: Option<String>,
    postal_code_from: Option<String>,
    postal_code_to: Option<String>,
    parent_jurisdiction_id: Option<String>,
    status: String,
    effective_from: String,
    effective_to: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<TaxJurisdictionRow> for TaxJurisdiction {
    fn from(r: TaxJurisdictionRow) -> Self {
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
            code: r.code,
            name: r.name,
            country_code: r.country_code,
            state_code: r.state_code,
            county: r.county,
            city: r.city,
            postal_code_from: r.postal_code_from,
            postal_code_to: r.postal_code_to,
            parent_jurisdiction_id: r.parent_jurisdiction_id.and_then(|id| Uuid::parse_str(&id).ok()),
            status: match r.status.as_str() {
                "Inactive" => erp_core::Status::Inactive,
                _ => erp_core::Status::Active,
            },
            effective_from: chrono::DateTime::parse_from_rfc3339(&r.effective_from)
                .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            effective_to: r.effective_to.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
        }
    }
}

#[async_trait]
pub trait TaxRateRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<TaxRate>;
    async fn find_by_jurisdiction(&self, pool: &SqlitePool, jurisdiction_id: Uuid) -> Result<Vec<TaxRate>>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<TaxRate>>;
    async fn create(&self, pool: &SqlitePool, rate: TaxRate) -> Result<TaxRate>;
    async fn update(&self, pool: &SqlitePool, rate: TaxRate) -> Result<TaxRate>;
}

pub struct SqliteTaxRateRepository;

#[async_trait]
impl TaxRateRepository for SqliteTaxRateRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<TaxRate> {
        let row = sqlx::query_as::<_, TaxRateRow>(
            "SELECT * FROM tax_rates WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("TaxRate", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn find_by_jurisdiction(&self, pool: &SqlitePool, jurisdiction_id: Uuid) -> Result<Vec<TaxRate>> {
        let rows = sqlx::query_as::<_, TaxRateRow>(
            "SELECT * FROM tax_rates WHERE jurisdiction_id = ? AND status = 'Active'"
        )
        .bind(jurisdiction_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<TaxRate>> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM tax_rates")
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e))?;
        
        let offset = (pagination.page.saturating_sub(1)) * pagination.per_page;
        let rows = sqlx::query_as::<_, TaxRateRow>(
            "SELECT * FROM tax_rates ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.per_page as i64)
        .bind(offset as i64)
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(Paginated::new(rows.into_iter().map(|r| r.into()).collect(), count as u64, pagination))
    }
    
    async fn create(&self, pool: &SqlitePool, rate: TaxRate) -> Result<TaxRate> {
        sqlx::query(
            "INSERT INTO tax_rates (id, jurisdiction_id, tax_type, name, code, rate, is_compound, is_recoverable, calculation_method, status, effective_from, effective_to, priority, min_amount, max_amount, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(rate.base.id.to_string())
        .bind(rate.jurisdiction_id.to_string())
        .bind(format!("{:?}", rate.tax_type))
        .bind(&rate.name)
        .bind(&rate.code)
        .bind(rate.rate)
        .bind(rate.is_compound as i32)
        .bind(rate.is_recoverable as i32)
        .bind(format!("{:?}", rate.calculation_method))
        .bind(format!("{:?}", rate.status))
        .bind(rate.effective_from.to_rfc3339())
        .bind(rate.effective_to.as_ref().map(|d| d.to_rfc3339()))
        .bind(rate.priority)
        .bind(rate.min_amount.as_ref().map(|m| m.amount))
        .bind(rate.max_amount.as_ref().map(|m| m.amount))
        .bind(rate.base.created_at.to_rfc3339())
        .bind(rate.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rate)
    }
    
    async fn update(&self, pool: &SqlitePool, rate: TaxRate) -> Result<TaxRate> {
        sqlx::query(
            "UPDATE tax_rates SET rate = ?, status = ?, updated_at = ? WHERE id = ?"
        )
        .bind(rate.rate)
        .bind(format!("{:?}", rate.status))
        .bind(rate.base.updated_at.to_rfc3339())
        .bind(rate.base.id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rate)
    }
}

#[derive(sqlx::FromRow)]
struct TaxRateRow {
    id: String,
    jurisdiction_id: String,
    tax_type: String,
    name: String,
    code: String,
    rate: f64,
    is_compound: i32,
    is_recoverable: i32,
    calculation_method: String,
    status: String,
    effective_from: String,
    effective_to: Option<String>,
    priority: i32,
    min_amount: Option<i64>,
    max_amount: Option<i64>,
    created_at: String,
    updated_at: String,
}

impl From<TaxRateRow> for TaxRate {
    fn from(r: TaxRateRow) -> Self {
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
            jurisdiction_id: Uuid::parse_str(&r.jurisdiction_id).unwrap_or_default(),
            tax_type: match r.tax_type.as_str() {
                "VAT" => TaxType::VAT,
                "GST" => TaxType::GST,
                "PST" => TaxType::PST,
                "HST" => TaxType::HST,
                "Withholding" => TaxType::Withholding,
                "Excise" => TaxType::Excise,
                "Custom" => TaxType::Custom,
                _ => TaxType::SalesTax,
            },
            name: r.name,
            code: r.code,
            rate: r.rate,
            is_compound: r.is_compound != 0,
            is_recoverable: r.is_recoverable != 0,
            calculation_method: match r.calculation_method.as_str() {
                "Inclusive" => TaxCalculationMethod::Inclusive,
                "Mixed" => TaxCalculationMethod::Mixed,
                _ => TaxCalculationMethod::Exclusive,
            },
            status: match r.status.as_str() {
                "Inactive" => erp_core::Status::Inactive,
                _ => erp_core::Status::Active,
            },
            effective_from: chrono::DateTime::parse_from_rfc3339(&r.effective_from)
                .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            effective_to: r.effective_to.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            priority: r.priority,
            min_amount: r.min_amount.map(|m| Money::new(m, Currency::USD)),
            max_amount: r.max_amount.map(|m| Money::new(m, Currency::USD)),
        }
    }
}

#[async_trait]
pub trait TaxTransactionRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<TaxTransaction>;
    async fn find_by_reference(&self, pool: &SqlitePool, transaction_type: &str, transaction_id: Uuid) -> Result<Vec<TaxTransaction>>;
    async fn create(&self, pool: &SqlitePool, transaction: TaxTransaction) -> Result<TaxTransaction>;
}

pub struct SqliteTaxTransactionRepository;

#[async_trait]
impl TaxTransactionRepository for SqliteTaxTransactionRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<TaxTransaction> {
        let row = sqlx::query_as::<_, TaxTransactionRow>(
            "SELECT * FROM tax_transactions WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("TaxTransaction", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn find_by_reference(&self, pool: &SqlitePool, transaction_type: &str, transaction_id: Uuid) -> Result<Vec<TaxTransaction>> {
        let rows = sqlx::query_as::<_, TaxTransactionRow>(
            "SELECT * FROM tax_transactions WHERE transaction_type = ? AND transaction_id = ?"
        )
        .bind(transaction_type)
        .bind(transaction_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    async fn create(&self, pool: &SqlitePool, transaction: TaxTransaction) -> Result<TaxTransaction> {
        sqlx::query(
            "INSERT INTO tax_transactions (id, transaction_type, transaction_id, transaction_date, customer_id, jurisdiction_id, tax_rate_id, tax_class_id, tax_type, taxable_amount, tax_rate, tax_amount, exemption_id, exempt_amount, source, external_id, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(transaction.base.id.to_string())
        .bind(&transaction.transaction_type)
        .bind(transaction.transaction_id.to_string())
        .bind(transaction.transaction_date.to_rfc3339())
        .bind(transaction.customer_id.map(|id| id.to_string()))
        .bind(transaction.jurisdiction_id.to_string())
        .bind(transaction.tax_rate_id.to_string())
        .bind(transaction.tax_class_id.map(|id| id.to_string()))
        .bind(format!("{:?}", transaction.tax_type))
        .bind(transaction.taxable_amount.amount)
        .bind(transaction.tax_rate)
        .bind(transaction.tax_amount.amount)
        .bind(transaction.exemption_id.map(|id| id.to_string()))
        .bind(transaction.exempt_amount.amount)
        .bind(format!("{:?}", transaction.source))
        .bind(&transaction.external_id)
        .bind(transaction.base.created_at.to_rfc3339())
        .bind(transaction.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(transaction)
    }
}

#[derive(sqlx::FromRow)]
struct TaxTransactionRow {
    id: String,
    transaction_type: String,
    transaction_id: String,
    transaction_date: String,
    customer_id: Option<String>,
    jurisdiction_id: String,
    tax_rate_id: String,
    tax_class_id: Option<String>,
    tax_type: String,
    taxable_amount: i64,
    tax_rate: f64,
    tax_amount: i64,
    exemption_id: Option<String>,
    exempt_amount: i64,
    source: String,
    external_id: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<TaxTransactionRow> for TaxTransaction {
    fn from(r: TaxTransactionRow) -> Self {
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
            transaction_type: r.transaction_type,
            transaction_id: Uuid::parse_str(&r.transaction_id).unwrap_or_default(),
            transaction_date: chrono::DateTime::parse_from_rfc3339(&r.transaction_date)
                .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            customer_id: r.customer_id.and_then(|id| Uuid::parse_str(&id).ok()),
            jurisdiction_id: Uuid::parse_str(&r.jurisdiction_id).unwrap_or_default(),
            tax_rate_id: Uuid::parse_str(&r.tax_rate_id).unwrap_or_default(),
            tax_class_id: r.tax_class_id.and_then(|id| Uuid::parse_str(&id).ok()),
            tax_type: match r.tax_type.as_str() {
                "VAT" => TaxType::VAT,
                "GST" => TaxType::GST,
                "PST" => TaxType::PST,
                "HST" => TaxType::HST,
                "Withholding" => TaxType::Withholding,
                "Excise" => TaxType::Excise,
                "Custom" => TaxType::Custom,
                _ => TaxType::SalesTax,
            },
            taxable_amount: Money::new(r.taxable_amount, Currency::USD),
            tax_rate: r.tax_rate,
            tax_amount: Money::new(r.tax_amount, Currency::USD),
            exemption_id: r.exemption_id.and_then(|id| Uuid::parse_str(&id).ok()),
            exempt_amount: Money::new(r.exempt_amount, Currency::USD),
            source: match r.source.as_str() {
                "SalesOrder" => TaxTransactionSource::SalesOrder,
                "Invoice" => TaxTransactionSource::Invoice,
                "PurchaseOrder" => TaxTransactionSource::PurchaseOrder,
                "POS" => TaxTransactionSource::POS,
                "Ecommerce" => TaxTransactionSource::Ecommerce,
                "Import" => TaxTransactionSource::Import,
                _ => TaxTransactionSource::Manual,
            },
            external_id: r.external_id,
        }
    }
}

#[async_trait]
pub trait TaxExemptionRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<TaxExemption>;
    async fn find_by_customer(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<TaxExemption>>;
    async fn create(&self, pool: &SqlitePool, exemption: TaxExemption) -> Result<TaxExemption>;
}

pub struct SqliteTaxExemptionRepository;

#[async_trait]
impl TaxExemptionRepository for SqliteTaxExemptionRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<TaxExemption> {
        let row = sqlx::query_as::<_, TaxExemptionRow>(
            "SELECT * FROM tax_exemptions WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("TaxExemption", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn find_by_customer(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<TaxExemption>> {
        let rows = sqlx::query_as::<_, TaxExemptionRow>(
            "SELECT * FROM tax_exemptions WHERE customer_id = ? AND status = 'Active'"
        )
        .bind(customer_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    async fn create(&self, pool: &SqlitePool, exemption: TaxExemption) -> Result<TaxExemption> {
        sqlx::query(
            "INSERT INTO tax_exemptions (id, customer_id, exemption_type, certificate_number, jurisdiction_id, issue_date, expiry_date, status, document_url, notes, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(exemption.base.id.to_string())
        .bind(exemption.customer_id.to_string())
        .bind(format!("{:?}", exemption.exemption_type))
        .bind(&exemption.certificate_number)
        .bind(exemption.jurisdiction_id.map(|id| id.to_string()))
        .bind(exemption.issue_date.to_rfc3339())
        .bind(exemption.expiry_date.map(|d| d.to_rfc3339()))
        .bind(format!("{:?}", exemption.status))
        .bind(&exemption.document_url)
        .bind(&exemption.notes)
        .bind(exemption.base.created_at.to_rfc3339())
        .bind(exemption.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(exemption)
    }
}

#[derive(sqlx::FromRow)]
struct TaxExemptionRow {
    id: String,
    customer_id: String,
    exemption_type: String,
    certificate_number: String,
    jurisdiction_id: Option<String>,
    issue_date: String,
    expiry_date: Option<String>,
    status: String,
    document_url: Option<String>,
    notes: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<TaxExemptionRow> for TaxExemption {
    fn from(r: TaxExemptionRow) -> Self {
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
            customer_id: Uuid::parse_str(&r.customer_id).unwrap_or_default(),
            exemption_type: match r.exemption_type.as_str() {
                "Manufacturing" => ExemptionType::Manufacturing,
                "Agricultural" => ExemptionType::Agricultural,
                "Government" => ExemptionType::Government,
                "NonProfit" => ExemptionType::NonProfit,
                "Educational" => ExemptionType::Educational,
                "DirectPay" => ExemptionType::DirectPay,
                "Other" => ExemptionType::Other,
                _ => ExemptionType::Resale,
            },
            certificate_number: r.certificate_number,
            jurisdiction_id: r.jurisdiction_id.and_then(|id| Uuid::parse_str(&id).ok()),
            issue_date: chrono::DateTime::parse_from_rfc3339(&r.issue_date)
                .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            expiry_date: r.expiry_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            status: match r.status.as_str() {
                "Inactive" => erp_core::Status::Inactive,
                _ => erp_core::Status::Active,
            },
            document_url: r.document_url,
            notes: r.notes,
        }
    }
}
