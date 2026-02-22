use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
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

pub struct FinancialReportingService {
    account_repo: SqliteAccountRepository,
    journal_repo: SqliteJournalEntryRepository,
}

impl FinancialReportingService {
    pub fn new() -> Self {
        Self {
            account_repo: SqliteAccountRepository,
            journal_repo: SqliteJournalEntryRepository,
        }
    }

    pub async fn get_account_balances(&self, pool: &SqlitePool) -> Result<Vec<AccountBalance>> {
        let accounts = self.account_repo.find_all(pool, Pagination { page: 1, per_page: 1000 }).await?;
        let mut balances = Vec::new();
        
        for account in accounts.items {
            let balance = self.calculate_account_balance(pool, account.base.id).await?;
            balances.push(AccountBalance {
                account_id: account.base.id,
                account_code: account.code.clone(),
                account_name: account.name.clone(),
                account_type: account.account_type.clone(),
                balance,
            });
        }
        
        Ok(balances)
    }

    async fn calculate_account_balance(&self, pool: &SqlitePool, account_id: Uuid) -> Result<i64> {
        let balance: (i64, i64) = sqlx::query_as(
            "SELECT COALESCE(SUM(debit), 0), COALESCE(SUM(credit), 0)
             FROM journal_lines jl
             JOIN journal_entries je ON jl.journal_entry_id = je.id
             WHERE jl.account_id = ? AND je.status = 'Completed'"
        )
        .bind(account_id.to_string())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(balance.0 - balance.1)
    }

    pub async fn get_balance_sheet(&self, pool: &SqlitePool) -> Result<BalanceSheet> {
        let balances = self.get_account_balances(pool).await?;
        
        let assets: Vec<AccountBalance> = balances.iter()
            .filter(|b| matches!(b.account_type, AccountType::Asset))
            .filter(|b| b.balance != 0)
            .cloned()
            .collect();
        let total_assets: i64 = assets.iter().map(|a| a.balance).sum();
        
        let liabilities: Vec<AccountBalance> = balances.iter()
            .filter(|b| matches!(b.account_type, AccountType::Liability))
            .filter(|b| b.balance != 0)
            .cloned()
            .collect();
        let total_liabilities: i64 = liabilities.iter().map(|a| a.balance).sum();
        
        let equity: Vec<AccountBalance> = balances.iter()
            .filter(|b| matches!(b.account_type, AccountType::Equity))
            .filter(|b| b.balance != 0)
            .cloned()
            .collect();
        let total_equity: i64 = equity.iter().map(|a| a.balance).sum();
        
        Ok(BalanceSheet {
            as_of_date: chrono::Utc::now(),
            assets,
            total_assets,
            liabilities,
            total_liabilities,
            equity,
            total_equity,
        })
    }

    pub async fn get_profit_and_loss(&self, pool: &SqlitePool, from_date: Option<DateTime<chrono::Utc>>, to_date: Option<DateTime<chrono::Utc>>) -> Result<ProfitAndLoss> {
        let balances = self.get_account_balances(pool).await?;
        
        let revenue: Vec<AccountBalance> = balances.iter()
            .filter(|b| matches!(b.account_type, AccountType::Revenue))
            .filter(|b| b.balance != 0)
            .cloned()
            .collect();
        let total_revenue: i64 = revenue.iter().map(|a| a.balance.abs()).sum();
        
        let expenses: Vec<AccountBalance> = balances.iter()
            .filter(|b| matches!(b.account_type, AccountType::Expense))
            .filter(|b| b.balance != 0)
            .cloned()
            .collect();
        let total_expenses: i64 = expenses.iter().map(|a| a.balance.abs()).sum();
        
        Ok(ProfitAndLoss {
            from_date: from_date.unwrap_or_else(|| chrono::Utc::now() - chrono::Duration::days(365)),
            to_date: to_date.unwrap_or_else(chrono::Utc::now),
            revenue,
            total_revenue,
            expenses,
            total_expenses,
            net_income: total_revenue - total_expenses,
        })
    }

    pub async fn get_trial_balance(&self, pool: &SqlitePool) -> Result<TrialBalance> {
        let balances = self.get_account_balances(pool).await?;
        
        let accounts: Vec<TrialBalanceLine> = balances.iter()
            .filter(|b| b.balance != 0)
            .map(|b| TrialBalanceLine {
                account_id: b.account_id,
                account_code: b.account_code.clone(),
                account_name: b.account_name.clone(),
                debit: if b.balance > 0 { b.balance.abs() } else { 0 },
                credit: if b.balance < 0 { b.balance.abs() } else { 0 },
            })
            .collect();
        
        let total_debits: i64 = accounts.iter().map(|a| a.debit).sum();
        let total_credits: i64 = accounts.iter().map(|a| a.credit).sum();
        
        Ok(TrialBalance {
            as_of_date: chrono::Utc::now(),
            accounts,
            total_debits,
            total_credits,
        })
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
