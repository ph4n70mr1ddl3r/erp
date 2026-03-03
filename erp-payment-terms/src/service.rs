use chrono::{Duration, Utc};
use erp_core::{BaseEntity, Error, Paginated, Pagination, Result, Status};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::{PaymentTerm, PaymentTermCalculation};
use crate::repository::{PaymentTermRepository, SqlitePaymentTermRepository};

pub struct PaymentTermService {
    repo: SqlitePaymentTermRepository,
}

impl Default for PaymentTermService {
    fn default() -> Self {
        Self::new()
    }
}

impl PaymentTermService {
    pub fn new() -> Self {
        Self {
            repo: SqlitePaymentTermRepository,
        }
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<PaymentTerm> {
        self.repo.find_by_id(pool, id).await
    }

    pub async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> Result<PaymentTerm> {
        self.repo.find_by_code(pool, code).await
    }

    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<PaymentTerm>> {
        self.repo.find_all(pool, pagination).await
    }

    pub async fn get_default(&self, pool: &SqlitePool) -> Result<Option<PaymentTerm>> {
        self.repo.find_default(pool).await
    }

    pub async fn create(&self, pool: &SqlitePool, mut term: PaymentTerm) -> Result<PaymentTerm> {
        if term.code.is_empty() {
            return Err(Error::validation("Payment term code is required"));
        }
        if term.name.is_empty() {
            return Err(Error::validation("Payment term name is required"));
        }
        if term.due_days < 0 {
            return Err(Error::validation("Due days cannot be negative"));
        }
        if let Some(discount_days) = term.discount_days {
            if discount_days < 0 {
                return Err(Error::validation("Discount days cannot be negative"));
            }
            if discount_days > term.due_days {
                return Err(Error::validation("Discount days cannot exceed due days"));
            }
        }
        if let Some(discount_percent) = term.discount_percent {
            if !(0.0..=100.0).contains(&discount_percent) {
                return Err(Error::validation("Discount percent must be between 0 and 100"));
            }
        }
        
        term.base = BaseEntity::new();
        term.status = Status::Active;
        
        if term.is_default {
            self.repo.set_default(pool, term.base.id).await?;
        }
        
        self.repo.create(pool, term).await
    }

    pub async fn update(&self, pool: &SqlitePool, term: PaymentTerm) -> Result<PaymentTerm> {
        if term.code.is_empty() {
            return Err(Error::validation("Payment term code is required"));
        }
        if term.name.is_empty() {
            return Err(Error::validation("Payment term name is required"));
        }
        if term.due_days < 0 {
            return Err(Error::validation("Due days cannot be negative"));
        }
        if let Some(discount_days) = term.discount_days {
            if discount_days < 0 {
                return Err(Error::validation("Discount days cannot be negative"));
            }
            if discount_days > term.due_days {
                return Err(Error::validation("Discount days cannot exceed due days"));
            }
        }
        if let Some(discount_percent) = term.discount_percent {
            if !(0.0..=100.0).contains(&discount_percent) {
                return Err(Error::validation("Discount percent must be between 0 and 100"));
            }
        }

        if term.is_default {
            self.repo.set_default(pool, term.base.id).await?;
        }
        
        self.repo.update(pool, term).await
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete(pool, id).await
    }

    pub async fn set_default(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let term = self.repo.find_by_id(pool, id).await?;
        if term.status != Status::Active {
            return Err(Error::business_rule("Only active payment terms can be set as default"));
        }
        self.repo.set_default(pool, id).await
    }

    pub fn calculate_dates(&self, term: &PaymentTerm, invoice_date: chrono::DateTime<Utc>, amount: i64) -> PaymentTermCalculation {
        let due_date = invoice_date + Duration::days(term.due_days as i64);
        let discount_date = term.discount_days.map(|d| invoice_date + Duration::days(d as i64));
        let discount_amount = term.discount_percent.map(|p| (amount as f64 * p / 100.0) as i64);
        
        PaymentTermCalculation {
            term_id: term.base.id,
            invoice_date,
            due_date,
            discount_date,
            discount_amount,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;

    async fn setup_pool() -> SqlitePool {
        SqlitePool::connect(":memory:").await.unwrap()
    }

    #[test]
    fn test_calculate_dates_net_30() {
        let service = PaymentTermService::new();
        let term = PaymentTerm {
            base: BaseEntity::new(),
            code: "NET30".to_string(),
            name: "Net 30".to_string(),
            description: Some("Payment due in 30 days".to_string()),
            due_days: 30,
            discount_days: None,
            discount_percent: None,
            is_default: true,
            status: Status::Active,
        };
        
        let invoice_date = Utc::now();
        let calc = service.calculate_dates(&term, invoice_date, 10000);
        
        assert_eq!(calc.term_id, term.base.id);
        assert!(calc.discount_date.is_none());
        assert!(calc.discount_amount.is_none());
    }

    #[test]
    fn test_calculate_dates_2_10_net_30() {
        let service = PaymentTermService::new();
        let term = PaymentTerm {
            base: BaseEntity::new(),
            code: "2_10_NET30".to_string(),
            name: "2/10 Net 30".to_string(),
            description: Some("2% discount if paid in 10 days, otherwise due in 30".to_string()),
            due_days: 30,
            discount_days: Some(10),
            discount_percent: Some(2.0),
            is_default: false,
            status: Status::Active,
        };
        
        let invoice_date = Utc::now();
        let amount: i64 = 10000;
        let calc = service.calculate_dates(&term, invoice_date, amount);
        
        assert!(calc.discount_date.is_some());
        assert_eq!(calc.discount_amount, Some(200));
    }

    #[test]
    fn test_validation_empty_code() {
        let service = PaymentTermService::new();
        let term = PaymentTerm {
            base: BaseEntity::new(),
            code: "".to_string(),
            name: "Test".to_string(),
            description: None,
            due_days: 30,
            discount_days: None,
            discount_percent: None,
            is_default: false,
            status: Status::Active,
        };
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        let pool = rt.block_on(setup_pool());
        let result = rt.block_on(service.create(&pool, term));
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_negative_due_days() {
        let service = PaymentTermService::new();
        let term = PaymentTerm {
            base: BaseEntity::new(),
            code: "TEST".to_string(),
            name: "Test".to_string(),
            description: None,
            due_days: -5,
            discount_days: None,
            discount_percent: None,
            is_default: false,
            status: Status::Active,
        };
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        let pool = rt.block_on(setup_pool());
        let result = rt.block_on(service.create(&pool, term));
        assert!(result.is_err());
    }

    #[test]
    fn test_validation_discount_exceeds_due() {
        let service = PaymentTermService::new();
        let term = PaymentTerm {
            base: BaseEntity::new(),
            code: "TEST".to_string(),
            name: "Test".to_string(),
            description: None,
            due_days: 10,
            discount_days: Some(20),
            discount_percent: Some(2.0),
            is_default: false,
            status: Status::Active,
        };
        
        let rt = tokio::runtime::Runtime::new().unwrap();
        let pool = rt.block_on(setup_pool());
        let result = rt.block_on(service.create(&pool, term));
        assert!(result.is_err());
    }
}
