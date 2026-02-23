use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Money, Currency};
use serde::Serialize;
use crate::models::*;
use crate::repository::*;

pub struct TaxJurisdictionService { repo: SqliteTaxJurisdictionRepository }
impl TaxJurisdictionService {
    pub fn new() -> Self { Self { repo: SqliteTaxJurisdictionRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<TaxJurisdiction> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<TaxJurisdiction>> {
        self.repo.find_all(pool, pagination).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut jurisdiction: TaxJurisdiction) -> Result<TaxJurisdiction> {
        if jurisdiction.code.is_empty() || jurisdiction.name.is_empty() {
            return Err(Error::validation("Jurisdiction code and name are required"));
        }
        jurisdiction.base = BaseEntity::new();
        jurisdiction.status = erp_core::Status::Active;
        self.repo.create(pool, jurisdiction).await
    }
}

pub struct TaxRateService { repo: SqliteTaxRateRepository }
impl TaxRateService {
    pub fn new() -> Self { Self { repo: SqliteTaxRateRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<TaxRate> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<TaxRate>> {
        self.repo.find_all(pool, pagination).await
    }
    
    pub async fn list_by_jurisdiction(&self, pool: &SqlitePool, jurisdiction_id: Uuid) -> Result<Vec<TaxRate>> {
        self.repo.find_by_jurisdiction(pool, jurisdiction_id).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut rate: TaxRate) -> Result<TaxRate> {
        if rate.name.is_empty() || rate.code.is_empty() {
            return Err(Error::validation("Tax rate name and code are required"));
        }
        if rate.rate < 0.0 || rate.rate > 100.0 {
            return Err(Error::validation("Tax rate must be between 0 and 100"));
        }
        rate.base = BaseEntity::new();
        rate.status = erp_core::Status::Active;
        self.repo.create(pool, rate).await
    }
    
    pub async fn update(&self, pool: &SqlitePool, mut rate: TaxRate) -> Result<TaxRate> {
        rate.base.updated_at = Utc::now();
        self.repo.update(pool, rate).await
    }
}

pub struct TaxCalculationService;
impl TaxCalculationService {
    pub fn new() -> Self { Self }
    
    pub async fn calculate(
        pool: &SqlitePool,
        jurisdiction_id: Uuid,
        amount: i64,
        customer_id: Option<Uuid>,
        product_tax_class_id: Option<Uuid>,
    ) -> Result<TaxCalculationResult> {
        let rates_repo = SqliteTaxRateRepository;
        let rates = rates_repo.find_by_jurisdiction(pool, jurisdiction_id).await?;
        
        if rates.is_empty() {
            return Ok(TaxCalculationResult {
                taxable_amount: Money::new(amount, Currency::USD),
                total_tax: Money::zero(Currency::USD),
                tax_breakdown: vec![],
            });
        }
        
        let exemption_amount = if let Some(cid) = customer_id {
            Self::check_exemption(pool, cid, jurisdiction_id).await?
        } else {
            0
        };
        
        let taxable_amount = (amount - exemption_amount).max(0);
        let mut total_tax = 0i64;
        let mut tax_breakdown = Vec::new();
        
        for rate in rates {
            let tax = ((taxable_amount as f64 * rate.rate / 100.0) * 100.0) as i64;
            total_tax += tax;
            
            tax_breakdown.push(TaxBreakdown {
                tax_rate_id: rate.base.id,
                name: rate.name.clone(),
                rate: rate.rate,
                taxable_amount: Money::new(taxable_amount, Currency::USD),
                tax_amount: Money::new(tax, Currency::USD),
            });
        }
        
        Ok(TaxCalculationResult {
            taxable_amount: Money::new(taxable_amount, Currency::USD),
            total_tax: Money::new(total_tax, Currency::USD),
            tax_breakdown,
        })
    }
    
    async fn check_exemption(pool: &SqlitePool, customer_id: Uuid, jurisdiction_id: Uuid) -> Result<i64> {
        let exemption_repo = SqliteTaxExemptionRepository;
        let exemptions = exemption_repo.find_by_customer(pool, customer_id).await?;
        
        for exemption in exemptions {
            if exemption.jurisdiction_id.is_none() || exemption.jurisdiction_id == Some(jurisdiction_id) {
                if let Some(expiry) = exemption.expiry_date {
                    if expiry > Utc::now() {
                        return Ok(i64::MAX);
                    }
                } else {
                    return Ok(i64::MAX);
                }
            }
        }
        
        Ok(0)
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct TaxCalculationResult {
    pub taxable_amount: Money,
    pub total_tax: Money,
    pub tax_breakdown: Vec<TaxBreakdown>,
}

#[derive(Debug, Clone, Serialize)]
pub struct TaxBreakdown {
    pub tax_rate_id: Uuid,
    pub name: String,
    pub rate: f64,
    pub taxable_amount: Money,
    pub tax_amount: Money,
}

pub struct TaxTransactionService { repo: SqliteTaxTransactionRepository }
impl TaxTransactionService {
    pub fn new() -> Self { Self { repo: SqliteTaxTransactionRepository } }
    
    pub async fn record(
        &self,
        pool: &SqlitePool,
        transaction_type: &str,
        transaction_id: Uuid,
        jurisdiction_id: Uuid,
        tax_rate_id: Uuid,
        taxable_amount: i64,
        tax_amount: i64,
        customer_id: Option<Uuid>,
        source: TaxTransactionSource,
    ) -> Result<TaxTransaction> {
        let rate_repo = SqliteTaxRateRepository;
        let rate = rate_repo.find_by_id(pool, tax_rate_id).await?;
        
        let transaction = TaxTransaction {
            base: BaseEntity::new(),
            transaction_type: transaction_type.to_string(),
            transaction_id,
            transaction_date: Utc::now(),
            customer_id,
            jurisdiction_id,
            tax_rate_id,
            tax_class_id: None,
            tax_type: rate.tax_type,
            taxable_amount: Money::new(taxable_amount, Currency::USD),
            tax_rate: rate.rate,
            tax_amount: Money::new(tax_amount, Currency::USD),
            exemption_id: None,
            exempt_amount: Money::zero(Currency::USD),
            source,
            external_id: None,
        };
        
        self.repo.create(pool, transaction).await
    }
    
    pub async fn get_by_reference(&self, pool: &SqlitePool, transaction_type: &str, transaction_id: Uuid) -> Result<Vec<TaxTransaction>> {
        self.repo.find_by_reference(pool, transaction_type, transaction_id).await
    }
}

pub struct TaxExemptionService { repo: SqliteTaxExemptionRepository }
impl TaxExemptionService {
    pub fn new() -> Self { Self { repo: SqliteTaxExemptionRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<TaxExemption> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn list_by_customer(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<TaxExemption>> {
        self.repo.find_by_customer(pool, customer_id).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut exemption: TaxExemption) -> Result<TaxExemption> {
        if exemption.certificate_number.is_empty() {
            return Err(Error::validation("Certificate number is required"));
        }
        exemption.base = BaseEntity::new();
        exemption.status = erp_core::Status::Active;
        self.repo.create(pool, exemption).await
    }
}

pub struct TaxReportService;
impl TaxReportService {
    pub fn new() -> Self { Self }
    
    pub async fn generate(
        pool: &SqlitePool,
        jurisdiction_id: Uuid,
        period_start: chrono::DateTime<Utc>,
        period_end: chrono::DateTime<Utc>,
    ) -> Result<TaxReport> {
        let total_sales: i64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(taxable_amount), 0) FROM tax_transactions WHERE jurisdiction_id = ? AND transaction_date >= ? AND transaction_date <= ?"
        )
        .bind(jurisdiction_id.to_string())
        .bind(period_start.to_rfc3339())
        .bind(period_end.to_rfc3339())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let taxable_sales: i64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(taxable_amount), 0) FROM tax_transactions WHERE jurisdiction_id = ? AND transaction_date >= ? AND transaction_date <= ? AND exempt_amount = 0"
        )
        .bind(jurisdiction_id.to_string())
        .bind(period_start.to_rfc3339())
        .bind(period_end.to_rfc3339())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let tax_collected: i64 = sqlx::query_scalar(
            "SELECT COALESCE(SUM(tax_amount), 0) FROM tax_transactions WHERE jurisdiction_id = ? AND transaction_date >= ? AND transaction_date <= ?"
        )
        .bind(jurisdiction_id.to_string())
        .bind(period_start.to_rfc3339())
        .bind(period_end.to_rfc3339())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let exempt_sales = total_sales - taxable_sales;
        
        let report = TaxReport {
            base: BaseEntity::new(),
            name: format!("Tax Report - {}", period_start.format("%Y-%m")),
            jurisdiction_id,
            report_period: period_start.format("%Y-%m").to_string(),
            period_start,
            period_end,
            total_sales: Money::new(total_sales, Currency::USD),
            taxable_sales: Money::new(taxable_sales, Currency::USD),
            exempt_sales: Money::new(exempt_sales, Currency::USD),
            tax_collected: Money::new(tax_collected, Currency::USD),
            tax_paid: Money::zero(Currency::USD),
            tax_due: Money::new(tax_collected, Currency::USD),
            status: TaxReportStatus::Generated,
            filed_at: None,
            filed_by: None,
            filing_reference: None,
        };
        
        sqlx::query(
            "INSERT INTO tax_reports (id, name, jurisdiction_id, report_period, period_start, period_end, total_sales, taxable_sales, exempt_sales, tax_collected, tax_paid, tax_due, status, filed_at, filed_by, filing_reference, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, NULL, ?, ?)"
        )
        .bind(report.base.id.to_string())
        .bind(&report.name)
        .bind(report.jurisdiction_id.to_string())
        .bind(&report.report_period)
        .bind(report.period_start.to_rfc3339())
        .bind(report.period_end.to_rfc3339())
        .bind(report.total_sales.amount)
        .bind(report.taxable_sales.amount)
        .bind(report.exempt_sales.amount)
        .bind(report.tax_collected.amount)
        .bind(report.tax_paid.amount)
        .bind(report.tax_due.amount)
        .bind(format!("{:?}", report.status))
        .bind(report.base.created_at.to_rfc3339())
        .bind(report.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(report)
    }
}
