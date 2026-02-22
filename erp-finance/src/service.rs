use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status};
use crate::models::*;
use crate::repository::*;

pub struct AccountService {
    repo: SqliteAccountRepository,
}

impl AccountService {
    pub fn new() -> Self {
        Self { repo: SqliteAccountRepository }
    }

    pub async fn get_account(&self, pool: &SqlitePool, id: Uuid) -> Result<Account> {
        self.repo.find_by_id(pool, id).await
    }

    pub async fn get_account_by_code(&self, pool: &SqlitePool, code: &str) -> Result<Account> {
        self.repo.find_by_code(pool, code).await
    }

    pub async fn list_accounts(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Account>> {
        self.repo.find_all(pool, pagination).await
    }

    pub async fn list_accounts_by_type(&self, pool: &SqlitePool, account_type: AccountType) -> Result<Vec<Account>> {
        self.repo.find_by_type(pool, account_type).await
    }

    pub async fn create_account(&self, pool: &SqlitePool, account: Account) -> Result<Account> {
        self.validate_account(&account)?;
        
        if self.repo.find_by_code(pool, &account.code).await.is_ok() {
            return Err(Error::Conflict(format!("Account code '{}' already exists", account.code)));
        }
        
        self.repo.create(pool, account).await
    }

    pub async fn update_account(&self, pool: &SqlitePool, account: Account) -> Result<Account> {
        self.validate_account(&account)?;
        self.repo.update(pool, account).await
    }

    pub async fn delete_account(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete(pool, id).await
    }

    fn validate_account(&self, account: &Account) -> Result<()> {
        if account.code.is_empty() {
            return Err(Error::validation("Account code is required"));
        }
        if account.name.is_empty() {
            return Err(Error::validation("Account name is required"));
        }
        Ok(())
    }
}

pub struct JournalEntryService {
    repo: SqliteJournalEntryRepository,
}

impl JournalEntryService {
    pub fn new() -> Self {
        Self { repo: SqliteJournalEntryRepository }
    }

    pub async fn get_entry(&self, pool: &SqlitePool, id: Uuid) -> Result<JournalEntry> {
        self.repo.find_by_id(pool, id).await
    }

    pub async fn list_entries(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<JournalEntry>> {
        self.repo.find_all(pool, pagination).await
    }

    pub async fn create_entry(&self, pool: &SqlitePool, mut entry: JournalEntry) -> Result<JournalEntry> {
        self.validate_entry(&entry)?;
        
        entry.base = BaseEntity::new();
        entry.entry_number = self.generate_entry_number();
        entry.status = Status::Draft;
        
        for line in &mut entry.lines {
            line.id = Uuid::new_v4();
        }
        
        self.repo.create(pool, entry).await
    }

    pub async fn update_entry(&self, pool: &SqlitePool, entry: JournalEntry) -> Result<JournalEntry> {
        self.validate_entry(&entry)?;
        self.repo.update(pool, entry).await
    }

    pub async fn post_entry(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.post(pool, id).await
    }

    fn validate_entry(&self, entry: &JournalEntry) -> Result<()> {
        if entry.lines.is_empty() {
            return Err(Error::validation("Journal entry must have at least one line"));
        }
        
        if entry.description.is_empty() {
            return Err(Error::validation("Description is required"));
        }
        
        let total_debits: i64 = entry.lines.iter().map(|l| l.debit.amount).sum();
        let total_credits: i64 = entry.lines.iter().map(|l| l.credit.amount).sum();
        
        if total_debits != total_credits {
            return Err(Error::business_rule(&format!(
                "Journal entry must balance. Debits: {}, Credits: {}",
                total_debits as f64 / 100.0,
                total_credits as f64 / 100.0
            )));
        }
        
        Ok(())
    }

    fn generate_entry_number(&self) -> String {
        format!("JE-{}", chrono::Local::now().format("%Y%m%d%H%M%S"))
    }
}

pub struct FiscalYearService {
    repo: SqliteFiscalYearRepository,
}

impl FiscalYearService {
    pub fn new() -> Self {
        Self { repo: SqliteFiscalYearRepository }
    }

    pub async fn get_fiscal_year(&self, pool: &SqlitePool, id: Uuid) -> Result<FiscalYear> {
        self.repo.find_by_id(pool, id).await
    }

    pub async fn get_active_fiscal_year(&self, pool: &SqlitePool) -> Result<FiscalYear> {
        self.repo.find_active(pool).await
    }

    pub async fn list_fiscal_years(&self, pool: &SqlitePool) -> Result<Vec<FiscalYear>> {
        self.repo.find_all(pool).await
    }

    pub async fn create_fiscal_year(&self, pool: &SqlitePool, mut year: FiscalYear) -> Result<FiscalYear> {
        if year.name.is_empty() {
            return Err(Error::validation("Fiscal year name is required"));
        }
        
        if year.end_date <= year.start_date {
            return Err(Error::validation("End date must be after start date"));
        }
        
        year.base = BaseEntity::new();
        year.status = Status::Active;
        
        self.repo.create(pool, year).await
    }

    pub async fn update_fiscal_year(&self, pool: &SqlitePool, year: FiscalYear) -> Result<FiscalYear> {
        self.repo.update(pool, year).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use erp_core::Money;
    
    fn create_test_entry(debits: Vec<i64>, credits: Vec<i64>) -> JournalEntry {
        let lines: Vec<JournalLine> = debits.into_iter().map(|d| JournalLine {
            id: Uuid::nil(),
            account_id: Uuid::nil(),
            debit: Money::new(d, erp_core::Currency::USD),
            credit: Money::new(0, erp_core::Currency::USD),
            description: None,
        }).chain(credits.into_iter().map(|c| JournalLine {
            id: Uuid::nil(),
            account_id: Uuid::nil(),
            debit: Money::new(0, erp_core::Currency::USD),
            credit: Money::new(c, erp_core::Currency::USD),
            description: None,
        })).collect();
        
        JournalEntry {
            base: BaseEntity::new(),
            entry_number: "JE-001".to_string(),
            description: "Test entry".to_string(),
            date: chrono::Utc::now(),
            reference: None,
            status: Status::Draft,
            lines,
        }
    }
    
    #[test]
    fn test_validate_entry_balanced() {
        let svc = JournalEntryService::new();
        let entry = create_test_entry(vec![10000, 5000], vec![15000]);
        
        let result = svc.validate_entry(&entry);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_entry_unbalanced() {
        let svc = JournalEntryService::new();
        let entry = create_test_entry(vec![10000], vec![5000]);
        
        let result = svc.validate_entry(&entry);
        assert!(result.is_err());
        
        let err = result.unwrap_err();
        assert!(err.to_string().contains("must balance"));
    }
    
    #[test]
    fn test_validate_entry_empty_lines() {
        let svc = JournalEntryService::new();
        let entry = JournalEntry {
            base: BaseEntity::new(),
            entry_number: "JE-001".to_string(),
            description: "Test entry".to_string(),
            date: chrono::Utc::now(),
            reference: None,
            status: Status::Draft,
            lines: vec![],
        };
        
        let result = svc.validate_entry(&entry);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("at least one line"));
    }
    
    #[test]
    fn test_validate_entry_empty_description() {
        let svc = JournalEntryService::new();
        let mut entry = create_test_entry(vec![10000], vec![10000]);
        entry.description = String::new();
        
        let result = svc.validate_entry(&entry);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Description is required"));
    }
    
    #[test]
    fn test_validate_account_empty_code() {
        let svc = AccountService::new();
        let account = Account {
            base: BaseEntity::new(),
            code: String::new(),
            name: "Test Account".to_string(),
            account_type: AccountType::Asset,
            parent_id: None,
            description: None,
            status: Status::Active,
        };
        
        let result = svc.validate_account(&account);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_validate_account_empty_name() {
        let svc = AccountService::new();
        let account = Account {
            base: BaseEntity::new(),
            code: "1000".to_string(),
            name: String::new(),
            account_type: AccountType::Asset,
            parent_id: None,
            description: None,
            status: Status::Active,
        };
        
        let result = svc.validate_account(&account);
        assert!(result.is_err());
    }
}
