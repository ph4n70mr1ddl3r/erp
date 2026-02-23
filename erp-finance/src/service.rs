use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status};
use serde::{Deserialize, Serialize};
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

pub struct CurrencyService;

impl CurrencyService {
    pub fn new() -> Self { Self }

    pub async fn list_currencies(pool: &SqlitePool) -> Result<Vec<CurrencyDef>> {
        let rows = sqlx::query_as::<_, CurrencyRow>(
            "SELECT code, name, symbol, is_base, status FROM currencies ORDER BY code"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn get_exchange_rate(pool: &SqlitePool, from: &str, to: &str) -> Result<f64> {
        if from == to {
            return Ok(1.0);
        }
        
        let row: Option<(f64,)> = sqlx::query_as(
            "SELECT rate FROM exchange_rates 
             WHERE from_currency = ? AND to_currency = ? 
             ORDER BY effective_date DESC LIMIT 1"
        )
        .bind(from)
        .bind(to)
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        row.map(|r| r.0)
            .ok_or_else(|| Error::not_found("ExchangeRate", &format!("{}->{}", from, to)))
    }

    pub async fn set_exchange_rate(pool: &SqlitePool, from: &str, to: &str, rate: f64) -> Result<ExchangeRate> {
        let now = chrono::Utc::now();
        let er = ExchangeRate {
            id: Uuid::new_v4(),
            from_currency: from.to_string(),
            to_currency: to.to_string(),
            rate,
            effective_date: now,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO exchange_rates (id, from_currency, to_currency, rate, effective_date, created_at)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(er.id.to_string())
        .bind(&er.from_currency)
        .bind(&er.to_currency)
        .bind(er.rate)
        .bind(er.effective_date.to_rfc3339())
        .bind(er.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(er)
    }

    pub async fn convert_amount(pool: &SqlitePool, amount: i64, from: &str, to: &str) -> Result<i64> {
        let rate = Self::get_exchange_rate(pool, from, to).await?;
        Ok((amount as f64 * rate) as i64)
    }
}

#[derive(sqlx::FromRow)]
struct CurrencyRow {
    code: String,
    name: String,
    symbol: String,
    is_base: i64,
    status: String,
}

impl From<CurrencyRow> for CurrencyDef {
    fn from(r: CurrencyRow) -> Self {
        Self {
            code: r.code,
            name: r.name,
            symbol: r.symbol,
            is_base: r.is_base != 0,
            status: match r.status.as_str() {
                "Inactive" => Status::Inactive,
                _ => Status::Active,
            },
        }
    }
}

pub struct BudgetService;

impl BudgetService {
    pub fn new() -> Self { Self }

    pub async fn list_budgets(pool: &SqlitePool) -> Result<Vec<BudgetWithVariance>> {
        let rows = sqlx::query_as::<_, BudgetRow>(
            "SELECT id, name, start_date, end_date, total_amount, status, created_at, updated_at
             FROM budgets ORDER BY start_date DESC"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let mut budgets = Vec::new();
        for row in rows {
            let lines = Self::get_budget_lines_with_variance(pool, row.id.clone()).await?;
            let total_actual: i64 = lines.iter().map(|l| l.actual_amount).sum();
            let total_variance: i64 = lines.iter().map(|l| l.variance).sum();
            let variance_percent = if row.total_amount > 0 {
                (total_variance as f64 / row.total_amount as f64) * 100.0
            } else { 0.0 };
            
            budgets.push(BudgetWithVariance {
                base: BaseEntity {
                    id: Uuid::parse_str(&row.id).unwrap_or_default(),
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                        .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at)
                        .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                    created_by: None,
                    updated_by: None,
                },
                name: row.name,
                start_date: chrono::DateTime::parse_from_rfc3339(&row.start_date)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                end_date: chrono::DateTime::parse_from_rfc3339(&row.end_date)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                total_amount: row.total_amount,
                total_actual,
                total_variance,
                variance_percent,
                status: match row.status.as_str() {
                    "Approved" => Status::Approved,
                    "Completed" => Status::Completed,
                    _ => Status::Draft,
                },
                lines,
            });
        }
        
        Ok(budgets)
    }

    async fn get_budget_lines_with_variance(pool: &SqlitePool, budget_id: String) -> Result<Vec<BudgetLineWithVariance>> {
        let rows = sqlx::query_as::<_, BudgetLineRow>(
            "SELECT bl.id, bl.account_id, bl.period, bl.amount, bl.actual, bl.variance,
                    a.code as account_code, a.name as account_name
             FROM budget_lines bl
             LEFT JOIN accounts a ON bl.account_id = a.id
             WHERE bl.budget_id = ?
             ORDER BY a.code, bl.period"
        )
        .bind(&budget_id)
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| {
            let variance = r.amount - r.actual;
            let variance_percent = if r.amount > 0 {
                (variance as f64 / r.amount as f64) * 100.0
            } else { 0.0 };
            
            BudgetLineWithVariance {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                account_id: Uuid::parse_str(&r.account_id).unwrap_or_default(),
                account_code: r.account_code.unwrap_or_default(),
                account_name: r.account_name.unwrap_or_default(),
                period: r.period as u32,
                budget_amount: r.amount,
                actual_amount: r.actual,
                variance,
                variance_percent,
            }
        }).collect())
    }

    pub async fn create_budget(pool: &SqlitePool, name: &str, start_date: &str, end_date: &str, lines: Vec<(String, u32, i64)>) -> Result<BudgetWithVariance> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now();
        let total: i64 = lines.iter().map(|(_, _, amt)| amt).sum();
        
        sqlx::query(
            "INSERT INTO budgets (id, name, start_date, end_date, total_amount, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, 'Draft', ?, ?)"
        )
        .bind(id.to_string())
        .bind(name)
        .bind(start_date)
        .bind(end_date)
        .bind(total)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        for (account_id, period, amount) in lines {
            let line_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO budget_lines (id, budget_id, account_id, period, amount, actual, variance)
                 VALUES (?, ?, ?, ?, ?, 0, 0)"
            )
            .bind(line_id.to_string())
            .bind(id.to_string())
            .bind(&account_id)
            .bind(period as i64)
            .bind(amount)
            .execute(pool)
            .await
            .map_err(|e| Error::Database(e))?;
        }
        
        let budgets = Self::list_budgets(pool).await?;
        budgets.into_iter().next()
            .ok_or_else(|| Error::not_found("Budget", &id.to_string()))
    }
}

#[derive(sqlx::FromRow)]
struct BudgetRow {
    id: String,
    name: String,
    start_date: String,
    end_date: String,
    total_amount: i64,
    status: String,
    created_at: String,
    updated_at: String,
}

#[derive(sqlx::FromRow)]
struct BudgetLineRow {
    id: String,
    account_id: String,
    period: i64,
    amount: i64,
    actual: i64,
    variance: i64,
    account_code: Option<String>,
    account_name: Option<String>,
}

pub struct FixedAssetService;

impl FixedAssetService {
    pub fn new() -> Self { Self }

    pub async fn create_asset(
        pool: &SqlitePool,
        asset_code: &str,
        name: &str,
        category: &str,
        cost: i64,
        salvage_value: i64,
        useful_life_years: i32,
        depreciation_method: DepreciationMethod,
        acquisition_date: &str,
        location: Option<&str>,
        description: Option<&str>,
    ) -> Result<FixedAsset> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        
        let asset = FixedAsset {
            id,
            asset_code: asset_code.to_string(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            category: category.to_string(),
            location: location.map(|s| s.to_string()),
            cost,
            salvage_value,
            useful_life_years,
            depreciation_method,
            acquisition_date: chrono::DateTime::parse_from_rfc3339(acquisition_date)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or(now),
            depreciation_start_date: None,
            accumulated_depreciation: 0,
            net_book_value: cost,
            status: Status::Active,
            created_at: now,
            updated_at: now,
        };
        
        sqlx::query(
            "INSERT INTO fixed_assets (id, asset_code, name, description, category, location, cost, salvage_value, useful_life_years, depreciation_method, acquisition_date, depreciation_start_date, accumulated_depreciation, net_book_value, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, 0, ?, 'Active', ?, ?)"
        )
        .bind(asset.id.to_string())
        .bind(&asset.asset_code)
        .bind(&asset.name)
        .bind(&asset.description)
        .bind(&asset.category)
        .bind(&asset.location)
        .bind(asset.cost)
        .bind(asset.salvage_value)
        .bind(asset.useful_life_years)
        .bind(format!("{:?}", asset.depreciation_method))
        .bind(asset.acquisition_date.to_rfc3339())
        .bind(asset.net_book_value)
        .bind(asset.created_at.to_rfc3339())
        .bind(asset.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(asset)
    }

    pub async fn get_asset(pool: &SqlitePool, id: Uuid) -> Result<FixedAsset> {
        let row = sqlx::query_as::<_, AssetRow>(
            "SELECT id, asset_code, name, description, category, location, cost, salvage_value, useful_life_years, depreciation_method, acquisition_date, depreciation_start_date, accumulated_depreciation, net_book_value, status, created_at, updated_at
             FROM fixed_assets WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("FixedAsset", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn list_assets(pool: &SqlitePool) -> Result<Vec<FixedAsset>> {
        let rows = sqlx::query_as::<_, AssetRow>(
            "SELECT id, asset_code, name, description, category, location, cost, salvage_value, useful_life_years, depreciation_method, acquisition_date, depreciation_start_date, accumulated_depreciation, net_book_value, status, created_at, updated_at
             FROM fixed_assets WHERE status = 'Active' ORDER BY asset_code"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn calculate_depreciation(pool: &SqlitePool, id: Uuid) -> Result<AssetDepreciation> {
        let asset = Self::get_asset(pool, id).await?;
        let now = chrono::Utc::now();
        let period = now.format("%Y-%m").to_string();
        
        let depreciable_amount = asset.cost - asset.salvage_value;
        let monthly_depreciation = match asset.depreciation_method {
            DepreciationMethod::StraightLine => {
                let months = asset.useful_life_years * 12;
                if months > 0 { depreciable_amount / months as i64 } else { 0 }
            }
            DepreciationMethod::DecliningBalance => {
                let rate = 0.2;
                (asset.net_book_value as f64 * rate / 12.0) as i64
            }
            _ => depreciable_amount / (asset.useful_life_years * 12) as i64,
        };
        
        let new_accumulated = asset.accumulated_depreciation + monthly_depreciation;
        let new_nbv = asset.cost - new_accumulated;
        
        let dep = AssetDepreciation {
            id: Uuid::new_v4(),
            asset_id: id,
            period: period.clone(),
            depreciation_amount: monthly_depreciation,
            accumulated_depreciation: new_accumulated,
            posted_at: now,
        };
        
        sqlx::query(
            "INSERT INTO asset_depreciation (id, asset_id, period, depreciation_amount, accumulated_depreciation, posted_at)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(dep.id.to_string())
        .bind(dep.asset_id.to_string())
        .bind(&dep.period)
        .bind(dep.depreciation_amount)
        .bind(dep.accumulated_depreciation)
        .bind(dep.posted_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        sqlx::query(
            "UPDATE fixed_assets SET accumulated_depreciation = ?, net_book_value = ?, updated_at = ? WHERE id = ?"
        )
        .bind(new_accumulated)
        .bind(new_nbv.max(asset.salvage_value))
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(dep)
    }

    pub async fn dispose_asset(pool: &SqlitePool, id: Uuid) -> Result<FixedAsset> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE fixed_assets SET status = 'Disposed', updated_at = ? WHERE id = ?"
        )
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get_asset(pool, id).await
    }
}

#[derive(sqlx::FromRow)]
struct AssetRow {
    id: String,
    asset_code: String,
    name: String,
    description: Option<String>,
    category: String,
    location: Option<String>,
    cost: i64,
    salvage_value: i64,
    useful_life_years: i64,
    depreciation_method: String,
    acquisition_date: String,
    depreciation_start_date: Option<String>,
    accumulated_depreciation: i64,
    net_book_value: i64,
    status: String,
    created_at: String,
    updated_at: String,
}

impl From<AssetRow> for FixedAsset {
    fn from(r: AssetRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            asset_code: r.asset_code,
            name: r.name,
            description: r.description,
            category: r.category,
            location: r.location,
            cost: r.cost,
            salvage_value: r.salvage_value,
            useful_life_years: r.useful_life_years as i32,
            depreciation_method: match r.depreciation_method.as_str() {
                "DecliningBalance" => DepreciationMethod::DecliningBalance,
                "SumOfYearsDigits" => DepreciationMethod::SumOfYearsDigits,
                "UnitsOfProduction" => DepreciationMethod::UnitsOfProduction,
                _ => DepreciationMethod::StraightLine,
            },
            acquisition_date: chrono::DateTime::parse_from_rfc3339(&r.acquisition_date)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            depreciation_start_date: r.depreciation_start_date
                .and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            accumulated_depreciation: r.accumulated_depreciation,
            net_book_value: r.net_book_value,
            status: match r.status.as_str() {
                "Disposed" => Status::Inactive,
                _ => Status::Active,
            },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
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
        #[derive(sqlx::FromRow)]
        struct AccountBalanceRow {
            id: String,
            code: String,
            name: String,
            account_type: String,
            total_debit: i64,
            total_credit: i64,
        }
        
        let rows = sqlx::query_as::<_, AccountBalanceRow>(
            "SELECT a.id, a.code, a.name, a.account_type,
                    COALESCE(SUM(jl.debit), 0) as total_debit,
                    COALESCE(SUM(jl.credit), 0) as total_credit
             FROM accounts a
             LEFT JOIN journal_lines jl ON a.id = jl.account_id
             LEFT JOIN journal_entries je ON jl.journal_entry_id = je.id AND je.status = 'Completed'
             WHERE a.status != 'Deleted'
             GROUP BY a.id, a.code, a.name, a.account_type"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let balances: Vec<AccountBalance> = rows.into_iter().map(|r| AccountBalance {
            account_id: Uuid::parse_str(&r.id).unwrap_or_default(),
            account_code: r.code,
            account_name: r.name,
            account_type: match r.account_type.as_str() {
                "Asset" => AccountType::Asset,
                "Liability" => AccountType::Liability,
                "Equity" => AccountType::Equity,
                "Revenue" => AccountType::Revenue,
                _ => AccountType::Expense,
            },
            balance: r.total_debit - r.total_credit,
        }).collect();
        
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
        .map_err(|e| Error::Database(e))?;
        
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

pub struct BankReconciliationService;

impl BankReconciliationService {
    pub fn new() -> Self { Self }

    pub async fn create_bank_account(
        pool: &SqlitePool,
        account_id: Uuid,
        bank_name: &str,
        account_number: &str,
        account_type: BankAccountType,
        currency: &str,
        gl_code: Option<&str>,
    ) -> Result<BankAccount> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        
        let account = BankAccount {
            id,
            account_id,
            bank_name: bank_name.to_string(),
            account_number: account_number.to_string(),
            account_type,
            currency: currency.to_string(),
            gl_code: gl_code.map(|s| s.to_string()),
            status: Status::Active,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO bank_accounts (id, account_id, bank_name, account_number, account_type, currency, gl_code, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, 'Active', ?)"
        )
        .bind(account.id.to_string())
        .bind(account.account_id.to_string())
        .bind(&account.bank_name)
        .bind(&account.account_number)
        .bind(format!("{:?}", account.account_type))
        .bind(&account.currency)
        .bind(&account.gl_code)
        .bind(account.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(account)
    }

    pub async fn get_bank_account(pool: &SqlitePool, id: Uuid) -> Result<BankAccount> {
        let row = sqlx::query_as::<_, BankAccountRow>(
            "SELECT id, account_id, bank_name, account_number, account_type, currency, gl_code, status, created_at
             FROM bank_accounts WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("BankAccount", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn list_bank_accounts(pool: &SqlitePool) -> Result<Vec<BankAccount>> {
        let rows = sqlx::query_as::<_, BankAccountRow>(
            "SELECT id, account_id, bank_name, account_number, account_type, currency, gl_code, status, created_at
             FROM bank_accounts WHERE status = 'Active' ORDER BY bank_name"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn import_statement(
        pool: &SqlitePool,
        bank_account_id: Uuid,
        statement_date: DateTime<Utc>,
        opening_balance: i64,
        closing_balance: i64,
        transactions: Vec<BankTransactionImport>,
    ) -> Result<BankStatement> {
        let now = chrono::Utc::now();
        let statement_id = Uuid::new_v4();
        
        let statement = BankStatement {
            id: statement_id,
            bank_account_id,
            statement_date,
            opening_balance,
            closing_balance,
            status: Status::Draft,
            reconciled_at: None,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO bank_statements (id, bank_account_id, statement_date, opening_balance, closing_balance, status, reconciled_at, created_at)
             VALUES (?, ?, ?, ?, ?, 'Draft', NULL, ?)"
        )
        .bind(statement.id.to_string())
        .bind(statement.bank_account_id.to_string())
        .bind(statement.statement_date.to_rfc3339())
        .bind(statement.opening_balance)
        .bind(statement.closing_balance)
        .bind(statement.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        for tx in transactions {
            let tx_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO bank_transactions (id, bank_account_id, statement_id, transaction_date, value_date, description, reference, debit, credit, balance, reconciled, journal_entry_id, created_at)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 0, NULL, ?)"
            )
            .bind(tx_id.to_string())
            .bind(bank_account_id.to_string())
            .bind(statement_id.to_string())
            .bind(tx.transaction_date.to_rfc3339())
            .bind(tx.value_date.map(|d| d.to_rfc3339()))
            .bind(&tx.description)
            .bind(&tx.reference)
            .bind(tx.debit)
            .bind(tx.credit)
            .bind(tx.balance)
            .bind(now.to_rfc3339())
            .execute(pool)
            .await
            .map_err(|e| Error::Database(e))?;
        }
        
        Ok(statement)
    }

    pub async fn reconcile_transaction(pool: &SqlitePool, transaction_id: Uuid, journal_entry_id: Option<Uuid>) -> Result<BankTransaction> {
        sqlx::query(
            "UPDATE bank_transactions SET reconciled = 1, journal_entry_id = ? WHERE id = ?"
        )
        .bind(journal_entry_id.map(|id| id.to_string()))
        .bind(transaction_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get_transaction(pool, transaction_id).await
    }

    async fn get_transaction(pool: &SqlitePool, id: Uuid) -> Result<BankTransaction> {
        let row = sqlx::query_as::<_, BankTransactionRow>(
            "SELECT id, bank_account_id, statement_id, transaction_date, value_date, description, reference, debit, credit, balance, reconciled, journal_entry_id, created_at
             FROM bank_transactions WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("BankTransaction", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn auto_reconcile(pool: &SqlitePool, bank_account_id: Uuid) -> Result<i32> {
        let rules = sqlx::query_as::<_, ReconciliationRuleRow>(
            "SELECT id, bank_account_id, rule_type, match_field, match_pattern, tolerance_days, tolerance_amount, auto_match, created_at
             FROM reconciliation_rules WHERE bank_account_id = ? AND auto_match = 1"
        )
        .bind(bank_account_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let mut matched_count = 0;
        
        for rule in rules {
            let transactions = sqlx::query_as::<_, BankTransactionRow>(
                "SELECT id, bank_account_id, statement_id, transaction_date, value_date, description, reference, debit, credit, balance, reconciled, journal_entry_id, created_at
                 FROM bank_transactions WHERE bank_account_id = ? AND reconciled = 0"
            )
            .bind(bank_account_id.to_string())
            .fetch_all(pool)
            .await
            .map_err(|e| Error::Database(e))?;
            
            for tx in transactions {
                let matches = match rule.rule_type.as_str() {
                    "ExactMatch" => Self::match_exact(pool, &tx, &rule).await?,
                    "AmountRange" => Self::match_amount_range(&tx, &rule),
                    _ => false,
                };
                
                if matches {
                    sqlx::query("UPDATE bank_transactions SET reconciled = 1 WHERE id = ?")
                        .bind(tx.id.clone())
                        .execute(pool)
                        .await
                        .map_err(|e| Error::Database(e))?;
                    matched_count += 1;
                }
            }
        }
        
        Ok(matched_count)
    }

    async fn match_exact(pool: &SqlitePool, tx: &BankTransactionRow, _rule: &ReconciliationRuleRow) -> Result<bool> {
        let gl_entry: Option<(String,)> = sqlx::query_as(
            "SELECT je.id FROM journal_lines jl
             JOIN journal_entries je ON jl.journal_entry_id = je.id
             WHERE jl.account_id = (SELECT account_id FROM bank_accounts WHERE id = ?)
             AND (jl.debit = ? OR jl.credit = ?)
             AND je.status = 'Completed'
             LIMIT 1"
        )
        .bind(tx.bank_account_id.clone())
        .bind(tx.debit)
        .bind(tx.credit)
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(gl_entry.is_some())
    }

    fn match_amount_range(tx: &BankTransactionRow, rule: &ReconciliationRuleRow) -> bool {
        let amount = tx.debit.abs().max(tx.credit.abs());
        let tolerance = rule.tolerance_amount;
        amount >= tolerance - 100 && amount <= tolerance + 100
    }
}

pub struct BankTransactionImport {
    pub transaction_date: DateTime<Utc>,
    pub value_date: Option<DateTime<Utc>>,
    pub description: Option<String>,
    pub reference: Option<String>,
    pub debit: i64,
    pub credit: i64,
    pub balance: i64,
}

#[derive(sqlx::FromRow)]
struct BankAccountRow {
    id: String,
    account_id: String,
    bank_name: String,
    account_number: String,
    account_type: String,
    currency: String,
    gl_code: Option<String>,
    status: String,
    created_at: String,
}

impl From<BankAccountRow> for BankAccount {
    fn from(r: BankAccountRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            account_id: Uuid::parse_str(&r.account_id).unwrap_or_default(),
            bank_name: r.bank_name,
            account_number: r.account_number,
            account_type: match r.account_type.as_str() {
                "Savings" => BankAccountType::Savings,
                "MoneyMarket" => BankAccountType::MoneyMarket,
                "CreditCard" => BankAccountType::CreditCard,
                "Loan" => BankAccountType::Loan,
                _ => BankAccountType::Checking,
            },
            currency: r.currency,
            gl_code: r.gl_code,
            status: if r.status == "Inactive" { Status::Inactive } else { Status::Active },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct BankTransactionRow {
    id: String,
    bank_account_id: String,
    statement_id: Option<String>,
    transaction_date: String,
    value_date: Option<String>,
    description: Option<String>,
    reference: Option<String>,
    debit: i64,
    credit: i64,
    balance: i64,
    reconciled: i64,
    journal_entry_id: Option<String>,
    created_at: String,
}

impl From<BankTransactionRow> for BankTransaction {
    fn from(r: BankTransactionRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            bank_account_id: Uuid::parse_str(&r.bank_account_id).unwrap_or_default(),
            statement_id: r.statement_id.and_then(|s| Uuid::parse_str(&s).ok()),
            transaction_date: chrono::DateTime::parse_from_rfc3339(&r.transaction_date)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            value_date: r.value_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            description: r.description,
            reference: r.reference,
            debit: r.debit,
            credit: r.credit,
            balance: r.balance,
            reconciled: r.reconciled != 0,
            journal_entry_id: r.journal_entry_id.and_then(|s| Uuid::parse_str(&s).ok()),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct ReconciliationRuleRow {
    id: String,
    bank_account_id: String,
    rule_type: String,
    match_field: String,
    match_pattern: Option<String>,
    tolerance_days: i64,
    tolerance_amount: i64,
    auto_match: i64,
    created_at: String,
}

pub struct CashFlowService;

impl CashFlowService {
    pub fn new() -> Self { Self }

    pub async fn create_forecast(
        pool: &SqlitePool,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
        opening_balance: i64,
        notes: Option<&str>,
    ) -> Result<CashFlowForecast> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        
        let forecast = CashFlowForecast {
            id,
            forecast_date: now,
            period_start,
            period_end,
            opening_balance,
            expected_inflows: 0,
            expected_outflows: 0,
            closing_balance: opening_balance,
            notes: notes.map(|s| s.to_string()),
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO cash_flow_forecasts (id, forecast_date, period_start, period_end, opening_balance, expected_inflows, expected_outflows, closing_balance, notes, created_at)
             VALUES (?, ?, ?, ?, ?, 0, 0, ?, ?, ?)"
        )
        .bind(forecast.id.to_string())
        .bind(forecast.forecast_date.to_rfc3339())
        .bind(forecast.period_start.to_rfc3339())
        .bind(forecast.period_end.to_rfc3339())
        .bind(forecast.opening_balance)
        .bind(forecast.closing_balance)
        .bind(&forecast.notes)
        .bind(forecast.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(forecast)
    }

    pub async fn get_forecast(pool: &SqlitePool, id: Uuid) -> Result<CashFlowForecast> {
        let row = sqlx::query_as::<_, CashFlowForecastRow>(
            "SELECT id, forecast_date, period_start, period_end, opening_balance, expected_inflows, expected_outflows, closing_balance, notes, created_at
             FROM cash_flow_forecasts WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("CashFlowForecast", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn add_cash_flow_item(
        pool: &SqlitePool,
        forecast_id: Uuid,
        category_id: Uuid,
        description: &str,
        expected_date: Option<DateTime<Utc>>,
        amount: i64,
        probability: i32,
        notes: Option<&str>,
    ) -> Result<CashFlowItem> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        
        let item = CashFlowItem {
            id,
            forecast_id,
            category_id,
            description: description.to_string(),
            expected_date,
            amount,
            probability,
            actual_amount: None,
            actual_date: None,
            notes: notes.map(|s| s.to_string()),
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO cash_flow_items (id, forecast_id, category_id, description, expected_date, amount, probability, actual_amount, actual_date, notes, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, NULL, NULL, ?, ?)"
        )
        .bind(item.id.to_string())
        .bind(item.forecast_id.to_string())
        .bind(item.category_id.to_string())
        .bind(&item.description)
        .bind(item.expected_date.map(|d| d.to_rfc3339()))
        .bind(item.amount)
        .bind(item.probability)
        .bind(&item.notes)
        .bind(item.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::recalculate_forecast_totals(pool, forecast_id).await?;
        
        Ok(item)
    }

    pub async fn update_item_actual(
        pool: &SqlitePool,
        item_id: Uuid,
        actual_amount: i64,
        actual_date: DateTime<Utc>,
    ) -> Result<CashFlowItem> {
        sqlx::query(
            "UPDATE cash_flow_items SET actual_amount = ?, actual_date = ? WHERE id = ?"
        )
        .bind(actual_amount)
        .bind(actual_date.to_rfc3339())
        .bind(item_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let row = sqlx::query_as::<_, CashFlowItemRow>(
            "SELECT id, forecast_id, category_id, description, expected_date, amount, probability, actual_amount, actual_date, notes, created_at
             FROM cash_flow_items WHERE id = ?"
        )
        .bind(item_id.to_string())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::recalculate_forecast_totals(pool, Uuid::parse_str(&row.forecast_id).unwrap_or_default()).await?;
        
        Ok(row.into())
    }

    pub async fn calculate_projected_balance(pool: &SqlitePool, forecast_id: Uuid) -> Result<i64> {
        let forecast = Self::get_forecast(pool, forecast_id).await?;
        
        let items: Vec<CashFlowItemRow> = sqlx::query_as(
            "SELECT id, forecast_id, category_id, description, expected_date, amount, probability, actual_amount, actual_date, notes, created_at
             FROM cash_flow_items WHERE forecast_id = ?"
        )
        .bind(forecast_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let total_inflows: i64 = items.iter()
            .filter(|i| i.amount > 0)
            .map(|i| (i.amount as f64 * (i.probability as f64 / 100.0)) as i64)
            .sum();
        
        let total_outflows: i64 = items.iter()
            .filter(|i| i.amount < 0)
            .map(|i| (i.amount.abs() as f64 * (i.probability as f64 / 100.0)) as i64)
            .sum();
        
        Ok(forecast.opening_balance + total_inflows - total_outflows)
    }

    async fn recalculate_forecast_totals(pool: &SqlitePool, forecast_id: Uuid) -> Result<()> {
        let items: Vec<CashFlowItemRow> = sqlx::query_as(
            "SELECT id, forecast_id, category_id, description, expected_date, amount, probability, actual_amount, actual_date, notes, created_at
             FROM cash_flow_items WHERE forecast_id = ?"
        )
        .bind(forecast_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let inflows: i64 = items.iter().filter(|i| i.amount > 0).map(|i| i.amount).sum();
        let outflows: i64 = items.iter().filter(|i| i.amount < 0).map(|i| i.amount.abs()).sum();
        
        let opening: (i64,) = sqlx::query_as(
            "SELECT opening_balance FROM cash_flow_forecasts WHERE id = ?"
        )
        .bind(forecast_id.to_string())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        sqlx::query(
            "UPDATE cash_flow_forecasts SET expected_inflows = ?, expected_outflows = ?, closing_balance = ? WHERE id = ?"
        )
        .bind(inflows)
        .bind(outflows)
        .bind(opening.0 + inflows - outflows)
        .bind(forecast_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct CashFlowForecastRow {
    id: String,
    forecast_date: String,
    period_start: String,
    period_end: String,
    opening_balance: i64,
    expected_inflows: i64,
    expected_outflows: i64,
    closing_balance: i64,
    notes: Option<String>,
    created_at: String,
}

impl From<CashFlowForecastRow> for CashFlowForecast {
    fn from(r: CashFlowForecastRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            forecast_date: chrono::DateTime::parse_from_rfc3339(&r.forecast_date)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            period_start: chrono::DateTime::parse_from_rfc3339(&r.period_start)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            period_end: chrono::DateTime::parse_from_rfc3339(&r.period_end)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            opening_balance: r.opening_balance,
            expected_inflows: r.expected_inflows,
            expected_outflows: r.expected_outflows,
            closing_balance: r.closing_balance,
            notes: r.notes,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct CashFlowItemRow {
    id: String,
    forecast_id: String,
    category_id: String,
    description: String,
    expected_date: Option<String>,
    amount: i64,
    probability: i64,
    actual_amount: Option<i64>,
    actual_date: Option<String>,
    notes: Option<String>,
    created_at: String,
}

impl From<CashFlowItemRow> for CashFlowItem {
    fn from(r: CashFlowItemRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            forecast_id: Uuid::parse_str(&r.forecast_id).unwrap_or_default(),
            category_id: Uuid::parse_str(&r.category_id).unwrap_or_default(),
            description: r.description,
            expected_date: r.expected_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            amount: r.amount,
            probability: r.probability as i32,
            actual_amount: r.actual_amount,
            actual_date: r.actual_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            notes: r.notes,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

pub struct CostAccountingService;

impl CostAccountingService {
    pub fn new() -> Self { Self }

    pub async fn create_cost_center(
        pool: &SqlitePool,
        code: &str,
        name: &str,
        department_id: Option<Uuid>,
        manager_id: Option<Uuid>,
        cost_center_type: CostCenterType,
        allocation_method: AllocationMethod,
    ) -> Result<CostCenter> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        
        let center = CostCenter {
            id,
            code: code.to_string(),
            name: name.to_string(),
            department_id,
            manager_id,
            cost_center_type,
            allocation_method,
            status: Status::Active,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO cost_centers (id, code, name, department_id, manager_id, cost_center_type, allocation_method, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, 'Active', ?)"
        )
        .bind(center.id.to_string())
        .bind(&center.code)
        .bind(&center.name)
        .bind(center.department_id.map(|id| id.to_string()))
        .bind(center.manager_id.map(|id| id.to_string()))
        .bind(format!("{:?}", center.cost_center_type))
        .bind(format!("{:?}", center.allocation_method))
        .bind(center.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(center)
    }

    pub async fn create_cost_element(
        pool: &SqlitePool,
        code: &str,
        name: &str,
        element_type: CostElementType,
        account_id: Option<Uuid>,
    ) -> Result<CostElement> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        
        let element = CostElement {
            id,
            code: code.to_string(),
            name: name.to_string(),
            element_type,
            account_id,
            status: Status::Active,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO cost_elements (id, code, name, element_type, account_id, status, created_at)
             VALUES (?, ?, ?, ?, ?, 'Active', ?)"
        )
        .bind(element.id.to_string())
        .bind(&element.code)
        .bind(&element.name)
        .bind(format!("{:?}", element.element_type))
        .bind(element.account_id.map(|id| id.to_string()))
        .bind(element.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(element)
    }

    pub async fn create_cost_pool(
        pool: &SqlitePool,
        name: &str,
        cost_center_id: Uuid,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
        total_cost: i64,
        allocation_base: &str,
        allocation_rate: f64,
    ) -> Result<CostPool> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        
        let cost_pool = CostPool {
            id,
            name: name.to_string(),
            cost_center_id,
            period_start,
            period_end,
            total_cost,
            allocation_base: allocation_base.to_string(),
            allocation_rate,
            status: Status::Active,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO cost_pools (id, name, cost_center_id, period_start, period_end, total_cost, allocation_base, allocation_rate, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'Active', ?)"
        )
        .bind(cost_pool.id.to_string())
        .bind(&cost_pool.name)
        .bind(cost_pool.cost_center_id.to_string())
        .bind(cost_pool.period_start.to_rfc3339())
        .bind(cost_pool.period_end.to_rfc3339())
        .bind(cost_pool.total_cost)
        .bind(&cost_pool.allocation_base)
        .bind(cost_pool.allocation_rate)
        .bind(cost_pool.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(cost_pool)
    }

    pub async fn allocate_costs(
        pool: &SqlitePool,
        pool_id: Uuid,
        from_cost_center_id: Uuid,
        to_cost_center_id: Uuid,
        allocation_base_value: f64,
    ) -> Result<CostAllocation> {
        #[derive(sqlx::FromRow)]
        struct CostPoolInfo {
            total_cost: i64,
            allocation_rate: f64,
        }
        
        let pool_row: CostPoolInfo = sqlx::query_as(
            "SELECT total_cost, allocation_rate FROM cost_pools WHERE id = ?"
        )
        .bind(pool_id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("CostPool", &pool_id.to_string()))?;
        
        let allocated_amount = (allocation_base_value * pool_row.allocation_rate) as i64;
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        
        let allocation = CostAllocation {
            id,
            pool_id,
            from_cost_center_id,
            to_cost_center_id,
            allocation_base_value,
            allocated_amount,
            allocated_at: now,
        };
        
        sqlx::query(
            "INSERT INTO cost_allocations (id, pool_id, from_cost_center_id, to_cost_center_id, allocation_base_value, allocated_amount, allocated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(allocation.id.to_string())
        .bind(allocation.pool_id.to_string())
        .bind(allocation.from_cost_center_id.to_string())
        .bind(allocation.to_cost_center_id.to_string())
        .bind(allocation.allocation_base_value)
        .bind(allocation.allocated_amount)
        .bind(allocation.allocated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(allocation)
    }

    pub async fn calculate_activity_cost(
        pool: &SqlitePool,
        activity_type_id: Uuid,
        cost_pool_id: Uuid,
        total_activities: i64,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<ActivityCost> {
        let pool_row: (i64,) = sqlx::query_as(
            "SELECT total_cost FROM cost_pools WHERE id = ?"
        )
        .bind(cost_pool_id.to_string())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let cost_per_activity = if total_activities > 0 {
            pool_row.0 / total_activities
        } else {
            0
        };
        
        let id = Uuid::new_v4();
        let activity_cost = ActivityCost {
            id,
            activity_type_id,
            cost_pool_id,
            total_activities,
            cost_per_activity,
            period_start,
            period_end,
        };
        
        sqlx::query(
            "INSERT INTO activity_costs (id, activity_type_id, cost_pool_id, total_activities, cost_per_activity, period_start, period_end)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(activity_cost.id.to_string())
        .bind(activity_cost.activity_type_id.to_string())
        .bind(activity_cost.cost_pool_id.to_string())
        .bind(activity_cost.total_activities)
        .bind(activity_cost.cost_per_activity)
        .bind(activity_cost.period_start.to_rfc3339())
        .bind(activity_cost.period_end.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(activity_cost)
    }
}

pub struct IntercompanyService;

impl IntercompanyService {
    pub fn new() -> Self { Self }

    pub async fn create_company(
        pool: &SqlitePool,
        code: &str,
        name: &str,
        legal_name: Option<&str>,
        tax_id: Option<&str>,
        registration_number: Option<&str>,
        currency: &str,
        address: Option<&str>,
        city: Option<&str>,
        country: Option<&str>,
        is_consolidation_entity: bool,
        parent_company_id: Option<Uuid>,
    ) -> Result<Company> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        
        let company = Company {
            id,
            code: code.to_string(),
            name: name.to_string(),
            legal_name: legal_name.map(|s| s.to_string()),
            tax_id: tax_id.map(|s| s.to_string()),
            registration_number: registration_number.map(|s| s.to_string()),
            currency: currency.to_string(),
            address: address.map(|s| s.to_string()),
            city: city.map(|s| s.to_string()),
            country: country.map(|s| s.to_string()),
            is_consolidation_entity,
            parent_company_id,
            status: Status::Active,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO companies (id, code, name, legal_name, tax_id, registration_number, currency, address, city, country, is_consolidation_entity, parent_company_id, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'Active', ?)"
        )
        .bind(company.id.to_string())
        .bind(&company.code)
        .bind(&company.name)
        .bind(&company.legal_name)
        .bind(&company.tax_id)
        .bind(&company.registration_number)
        .bind(&company.currency)
        .bind(&company.address)
        .bind(&company.city)
        .bind(&company.country)
        .bind(company.is_consolidation_entity as i64)
        .bind(company.parent_company_id.map(|id| id.to_string()))
        .bind(company.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(company)
    }

    pub async fn get_company(pool: &SqlitePool, id: Uuid) -> Result<Company> {
        let row = sqlx::query_as::<_, CompanyRow>(
            "SELECT id, code, name, legal_name, tax_id, registration_number, currency, address, city, country, is_consolidation_entity, parent_company_id, status, created_at
             FROM companies WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("Company", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn create_transaction(
        pool: &SqlitePool,
        from_company_id: Uuid,
        to_company_id: Uuid,
        transaction_date: DateTime<Utc>,
        amount: i64,
        currency: &str,
        description: Option<&str>,
        reference: Option<&str>,
    ) -> Result<IntercompanyTransaction> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        let transaction_number = format!("IC-{}", now.format("%Y%m%d%H%M%S"));
        
        let transaction = IntercompanyTransaction {
            id,
            transaction_number,
            from_company_id,
            to_company_id,
            transaction_date,
            amount,
            currency: currency.to_string(),
            description: description.map(|s| s.to_string()),
            reference: reference.map(|s| s.to_string()),
            from_journal_entry_id: None,
            to_journal_entry_id: None,
            status: Status::Draft,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO intercompany_transactions (id, transaction_number, from_company_id, to_company_id, transaction_date, amount, currency, description, reference, from_journal_entry_id, to_journal_entry_id, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, 'Draft', ?)"
        )
        .bind(transaction.id.to_string())
        .bind(&transaction.transaction_number)
        .bind(transaction.from_company_id.to_string())
        .bind(transaction.to_company_id.to_string())
        .bind(transaction.transaction_date.to_rfc3339())
        .bind(transaction.amount)
        .bind(&transaction.currency)
        .bind(&transaction.description)
        .bind(&transaction.reference)
        .bind(transaction.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(transaction)
    }

    pub async fn post_transaction(pool: &SqlitePool, id: Uuid) -> Result<IntercompanyTransaction> {
        sqlx::query(
            "UPDATE intercompany_transactions SET status = 'Completed' WHERE id = ?"
        )
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let row = sqlx::query_as::<_, IntercompanyTransactionRow>(
            "SELECT id, transaction_number, from_company_id, to_company_id, transaction_date, amount, currency, description, reference, from_journal_entry_id, to_journal_entry_id, status, created_at
             FROM intercompany_transactions WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(row.into())
    }
}

#[derive(sqlx::FromRow)]
struct CompanyRow {
    id: String,
    code: String,
    name: String,
    legal_name: Option<String>,
    tax_id: Option<String>,
    registration_number: Option<String>,
    currency: String,
    address: Option<String>,
    city: Option<String>,
    country: Option<String>,
    is_consolidation_entity: i64,
    parent_company_id: Option<String>,
    status: String,
    created_at: String,
}

impl From<CompanyRow> for Company {
    fn from(r: CompanyRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            code: r.code,
            name: r.name,
            legal_name: r.legal_name,
            tax_id: r.tax_id,
            registration_number: r.registration_number,
            currency: r.currency,
            address: r.address,
            city: r.city,
            country: r.country,
            is_consolidation_entity: r.is_consolidation_entity != 0,
            parent_company_id: r.parent_company_id.and_then(|s| Uuid::parse_str(&s).ok()),
            status: if r.status == "Inactive" { Status::Inactive } else { Status::Active },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct IntercompanyTransactionRow {
    id: String,
    transaction_number: String,
    from_company_id: String,
    to_company_id: String,
    transaction_date: String,
    amount: i64,
    currency: String,
    description: Option<String>,
    reference: Option<String>,
    from_journal_entry_id: Option<String>,
    to_journal_entry_id: Option<String>,
    status: String,
    created_at: String,
}

impl From<IntercompanyTransactionRow> for IntercompanyTransaction {
    fn from(r: IntercompanyTransactionRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            transaction_number: r.transaction_number,
            from_company_id: Uuid::parse_str(&r.from_company_id).unwrap_or_default(),
            to_company_id: Uuid::parse_str(&r.to_company_id).unwrap_or_default(),
            transaction_date: chrono::DateTime::parse_from_rfc3339(&r.transaction_date)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            amount: r.amount,
            currency: r.currency,
            description: r.description,
            reference: r.reference,
            from_journal_entry_id: r.from_journal_entry_id.and_then(|s| Uuid::parse_str(&s).ok()),
            to_journal_entry_id: r.to_journal_entry_id.and_then(|s| Uuid::parse_str(&s).ok()),
            status: match r.status.as_str() {
                "Completed" => Status::Completed,
                "Pending" => Status::Pending,
                _ => Status::Draft,
            },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

pub struct RevenueRecognitionService;

impl RevenueRecognitionService {
    pub fn new() -> Self { Self }

    pub async fn create_schedule(
        pool: &SqlitePool,
        name: &str,
        recognition_method: RecognitionMethod,
        total_amount: i64,
        start_date: DateTime<Utc>,
        end_date: Option<DateTime<Utc>>,
    ) -> Result<RevenueSchedule> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        let schedule_number = format!("REV-{}", now.format("%Y%m%d%H%M%S"));
        
        let schedule = RevenueSchedule {
            id,
            schedule_number,
            name: name.to_string(),
            recognition_method,
            total_amount,
            recognized_amount: 0,
            deferred_amount: total_amount,
            start_date,
            end_date,
            status: Status::Active,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO revenue_schedules (id, schedule_number, name, recognition_method, total_amount, recognized_amount, deferred_amount, start_date, end_date, status, created_at)
             VALUES (?, ?, ?, ?, ?, 0, ?, ?, ?, 'Active', ?)"
        )
        .bind(schedule.id.to_string())
        .bind(&schedule.schedule_number)
        .bind(&schedule.name)
        .bind(format!("{:?}", schedule.recognition_method))
        .bind(schedule.total_amount)
        .bind(schedule.deferred_amount)
        .bind(schedule.start_date.to_rfc3339())
        .bind(schedule.end_date.map(|d| d.to_rfc3339()))
        .bind(schedule.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(schedule)
    }

    pub async fn add_recognition_line(
        pool: &SqlitePool,
        schedule_id: Uuid,
        line_number: i32,
        recognition_date: DateTime<Utc>,
        amount: i64,
    ) -> Result<RevenueScheduleLine> {
        let id = Uuid::new_v4();
        
        let line = RevenueScheduleLine {
            id,
            schedule_id,
            line_number,
            recognition_date,
            amount,
            recognized: false,
            journal_entry_id: None,
            recognized_at: None,
        };
        
        sqlx::query(
            "INSERT INTO revenue_schedule_lines (id, schedule_id, line_number, recognition_date, amount, recognized, journal_entry_id, recognized_at)
             VALUES (?, ?, ?, ?, ?, 0, NULL, NULL)"
        )
        .bind(line.id.to_string())
        .bind(line.schedule_id.to_string())
        .bind(line.line_number)
        .bind(line.recognition_date.to_rfc3339())
        .bind(line.amount)
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(line)
    }

    pub async fn recognize_revenue(
        pool: &SqlitePool,
        schedule_id: Uuid,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<Vec<RevenueScheduleLine>> {
        let now = chrono::Utc::now();
        
        let lines: Vec<RevenueScheduleLineRow> = sqlx::query_as(
            "SELECT id, schedule_id, line_number, recognition_date, amount, recognized, journal_entry_id, recognized_at
             FROM revenue_schedule_lines
             WHERE schedule_id = ? AND recognized = 0 AND recognition_date >= ? AND recognition_date <= ?"
        )
        .bind(schedule_id.to_string())
        .bind(period_start.to_rfc3339())
        .bind(period_end.to_rfc3339())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let mut recognized_lines = Vec::new();
        let mut total_recognized = 0i64;
        
        for line_row in lines {
            sqlx::query(
                "UPDATE revenue_schedule_lines SET recognized = 1, recognized_at = ? WHERE id = ?"
            )
            .bind(now.to_rfc3339())
            .bind(&line_row.id)
            .execute(pool)
            .await
            .map_err(|e| Error::Database(e))?;
            
            total_recognized += line_row.amount;
            recognized_lines.push(line_row.into());
        }
        
        if total_recognized > 0 {
            sqlx::query(
                "UPDATE revenue_schedules SET recognized_amount = recognized_amount + ?, deferred_amount = deferred_amount - ? WHERE id = ?"
            )
            .bind(total_recognized)
            .bind(total_recognized)
            .bind(schedule_id.to_string())
            .execute(pool)
            .await
            .map_err(|e| Error::Database(e))?;
        }
        
        Ok(recognized_lines)
    }
}

#[derive(sqlx::FromRow)]
struct RevenueScheduleLineRow {
    id: String,
    schedule_id: String,
    line_number: i64,
    recognition_date: String,
    amount: i64,
    recognized: i64,
    journal_entry_id: Option<String>,
    recognized_at: Option<String>,
}

impl From<RevenueScheduleLineRow> for RevenueScheduleLine {
    fn from(r: RevenueScheduleLineRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            schedule_id: Uuid::parse_str(&r.schedule_id).unwrap_or_default(),
            line_number: r.line_number as i32,
            recognition_date: chrono::DateTime::parse_from_rfc3339(&r.recognition_date)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            amount: r.amount,
            recognized: r.recognized != 0,
            journal_entry_id: r.journal_entry_id.and_then(|s| Uuid::parse_str(&s).ok()),
            recognized_at: r.recognized_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
        }
    }
}

pub struct ConsolidationService;

impl ConsolidationService {
    pub fn new() -> Self { Self }

    pub async fn create_consolidation(
        pool: &SqlitePool,
        name: &str,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
        parent_company_id: Uuid,
    ) -> Result<ConsolidationSchedule> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        
        let consolidation = ConsolidationSchedule {
            id,
            name: name.to_string(),
            period_start,
            period_end,
            parent_company_id,
            status: Status::Draft,
            elimination_entries: 0,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO consolidation_schedules (id, name, period_start, period_end, parent_company_id, status, elimination_entries, created_at)
             VALUES (?, ?, ?, ?, ?, 'Draft', 0, ?)"
        )
        .bind(consolidation.id.to_string())
        .bind(&consolidation.name)
        .bind(consolidation.period_start.to_rfc3339())
        .bind(consolidation.period_end.to_rfc3339())
        .bind(consolidation.parent_company_id.to_string())
        .bind(consolidation.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(consolidation)
    }

    pub async fn add_company(
        pool: &SqlitePool,
        consolidation_id: Uuid,
        company_id: Uuid,
        ownership_percent: f64,
        consolidation_method: ConsolidationMethod,
        exchange_rate: f64,
        translation_method: TranslationMethod,
    ) -> Result<ConsolidationCompany> {
        let id = Uuid::new_v4();
        
        let company = ConsolidationCompany {
            id,
            consolidation_id,
            company_id,
            ownership_percent,
            consolidation_method,
            exchange_rate,
            translation_method,
        };
        
        sqlx::query(
            "INSERT INTO consolidation_companies (id, consolidation_id, company_id, ownership_percent, consolidation_method, exchange_rate, translation_method)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(company.id.to_string())
        .bind(company.consolidation_id.to_string())
        .bind(company.company_id.to_string())
        .bind(company.ownership_percent)
        .bind(format!("{:?}", company.consolidation_method))
        .bind(company.exchange_rate)
        .bind(format!("{:?}", company.translation_method))
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(company)
    }

    pub async fn run_eliminations(pool: &SqlitePool, consolidation_id: Uuid) -> Result<Vec<EliminationEntry>> {
        let now = chrono::Utc::now();
        
        let rules: Vec<EliminationRuleRow> = sqlx::query_as(
            "SELECT id, name, from_account_pattern, to_account_pattern, elimination_account_id, description, status
             FROM elimination_rules WHERE status = 'Active'"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let mut entries = Vec::new();
        
        for rule in rules {
            let matches: Vec<(String, String, i64)> = sqlx::query_as(
                "SELECT a.id, a.code, COALESCE(SUM(jl.debit - jl.credit), 0) as balance
                 FROM accounts a
                 LEFT JOIN journal_lines jl ON jl.account_id = a.id
                 LEFT JOIN journal_entries je ON jl.journal_entry_id = je.id AND je.status = 'Completed'
                 WHERE a.code LIKE ?
                 GROUP BY a.id"
            )
            .bind(format!("{}%", rule.from_account_pattern))
            .fetch_all(pool)
            .await
            .map_err(|e| Error::Database(e))?;
            
            for (account_id, _code, balance) in matches {
                if balance != 0 {
                    let entry_id = Uuid::new_v4();
                    let entry = EliminationEntry {
                        id: entry_id,
                        consolidation_id,
                        elimination_rule_id: Some(Uuid::parse_str(&rule.id).unwrap_or_default()),
                        description: rule.description.clone().unwrap_or_else(|| format!("Elimination: {}", rule.name)),
                        debit_account_id: Uuid::parse_str(&account_id).unwrap_or_default(),
                        credit_account_id: Uuid::parse_str(&rule.elimination_account_id).unwrap_or_default(),
                        amount: balance.abs(),
                        journal_entry_id: None,
                        created_at: now,
                    };
                    
                    sqlx::query(
                        "INSERT INTO elimination_entries (id, consolidation_id, elimination_rule_id, description, debit_account_id, credit_account_id, amount, journal_entry_id, created_at)
                         VALUES (?, ?, ?, ?, ?, ?, ?, NULL, ?)"
                    )
                    .bind(entry.id.to_string())
                    .bind(entry.consolidation_id.to_string())
                    .bind(entry.elimination_rule_id.map(|id| id.to_string()))
                    .bind(&entry.description)
                    .bind(entry.debit_account_id.to_string())
                    .bind(entry.credit_account_id.to_string())
                    .bind(entry.amount)
                    .bind(entry.created_at.to_rfc3339())
                    .execute(pool)
                    .await
                    .map_err(|e| Error::Database(e))?;
                    
                    entries.push(entry);
                }
            }
        }
        
        sqlx::query(
            "UPDATE consolidation_schedules SET elimination_entries = ? WHERE id = ?"
        )
        .bind(entries.len() as i32)
        .bind(consolidation_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(entries)
    }

    pub async fn generate_consolidated_statements(pool: &SqlitePool, consolidation_id: Uuid) -> Result<ConsolidatedStatements> {
        let companies: Vec<ConsolidationCompanyRow> = sqlx::query_as(
            "SELECT id, consolidation_id, company_id, ownership_percent, consolidation_method, exchange_rate, translation_method
             FROM consolidation_companies WHERE consolidation_id = ?"
        )
        .bind(consolidation_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let mut total_assets = 0i64;
        let mut total_liabilities = 0i64;
        let mut total_equity = 0i64;
        let mut total_revenue = 0i64;
        let mut total_expenses = 0i64;
        
        for company in companies {
            let exchange_rate = company.exchange_rate;
            let ownership = company.ownership_percent / 100.0;
            
            let balances: Vec<(String, i64)> = sqlx::query_as(
                "SELECT a.account_type, COALESCE(SUM(jl.debit - jl.credit), 0)
                 FROM accounts a
                 LEFT JOIN journal_lines jl ON jl.account_id = a.id
                 LEFT JOIN journal_entries je ON jl.journal_entry_id = je.id AND je.status = 'Completed'
                 WHERE a.company_id IS NULL OR a.company_id = ?
                 GROUP BY a.account_type"
            )
            .bind(&company.company_id)
            .fetch_all(pool)
            .await
            .map_err(|e| Error::Database(e))?;
            
            for (account_type, balance) in balances {
                let adjusted = ((balance as f64 * exchange_rate) * ownership) as i64;
                match account_type.as_str() {
                    "Asset" => total_assets += adjusted,
                    "Liability" => total_liabilities += adjusted,
                    "Equity" => total_equity += adjusted,
                    "Revenue" => total_revenue += adjusted.abs(),
                    "Expense" => total_expenses += adjusted.abs(),
                    _ => {}
                }
            }
        }
        
        Ok(ConsolidatedStatements {
            consolidation_id,
            balance_sheet: ConsolidatedBalanceSheet {
                total_assets,
                total_liabilities,
                total_equity,
                total_non_controlling_interest: 0,
            },
            income_statement: ConsolidatedIncomeStatement {
                total_revenue,
                total_expenses,
                net_income: total_revenue - total_expenses,
                net_income_attributable_to_nci: 0,
            },
        })
    }
}

#[derive(sqlx::FromRow)]
struct EliminationRuleRow {
    id: String,
    name: String,
    from_account_pattern: String,
    to_account_pattern: String,
    elimination_account_id: String,
    description: Option<String>,
    status: String,
}

#[derive(sqlx::FromRow)]
struct ConsolidationCompanyRow {
    id: String,
    consolidation_id: String,
    company_id: String,
    ownership_percent: f64,
    consolidation_method: String,
    exchange_rate: f64,
    translation_method: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedStatements {
    pub consolidation_id: Uuid,
    pub balance_sheet: ConsolidatedBalanceSheet,
    pub income_statement: ConsolidatedIncomeStatement,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedBalanceSheet {
    pub total_assets: i64,
    pub total_liabilities: i64,
    pub total_equity: i64,
    pub total_non_controlling_interest: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidatedIncomeStatement {
    pub total_revenue: i64,
    pub total_expenses: i64,
    pub net_income: i64,
    pub net_income_attributable_to_nci: i64,
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
