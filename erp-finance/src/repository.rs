use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status};
use crate::models::*;

#[derive(sqlx::FromRow)]
struct AccountRow {
    id: String,
    code: String,
    name: String,
    account_type: String,
    parent_id: Option<String>,
    status: String,
    description: Option<String>,
    created_at: String,
    updated_at: String,
    created_by: Option<String>,
    updated_by: Option<String>,
}

impl AccountRow {
    fn into_account(self) -> Result<Account> {
        let id = Uuid::parse_str(&self.id)
            .map_err(|_| Error::validation("Invalid account ID format"))?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&self.created_at)
            .map(|d| d.with_timezone(&Utc))
            .map_err(|_| Error::validation("Invalid created_at timestamp"))?;
        let updated_at = chrono::DateTime::parse_from_rfc3339(&self.updated_at)
            .map(|d| d.with_timezone(&Utc))
            .map_err(|_| Error::validation("Invalid updated_at timestamp"))?;
        Ok(Account {
            base: BaseEntity {
                id,
                created_at,
                updated_at,
                created_by: self.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: self.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            code: self.code,
            name: self.name,
            account_type: match self.account_type.as_str() {
                "Liability" => AccountType::Liability,
                "Equity" => AccountType::Equity,
                "Revenue" => AccountType::Revenue,
                "Expense" => AccountType::Expense,
                _ => AccountType::Asset,
            },
            parent_id: self.parent_id.and_then(|s| Uuid::parse_str(&s).ok()),
            status: match self.status.as_str() {
                "Inactive" => Status::Inactive,
                _ => Status::Active,
            },
            description: self.description,
        })
    }
}

pub struct SqliteAccountRepository;

#[async_trait]
impl AccountRepository for SqliteAccountRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Account> {
        let row = sqlx::query_as::<_, AccountRow>(
            "SELECT id, code, name, account_type, parent_id, status, description,
                    created_at, updated_at, created_by, updated_by
             FROM accounts WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("Account", &id.to_string()))?;
        
        Ok(row.into_account()?)
    }

    async fn find_by_code(&self, pool: &SqlitePool, code: &str) -> Result<Account> {
        let row = sqlx::query_as::<_, AccountRow>(
            "SELECT id, code, name, account_type, parent_id, status, description,
                    created_at, updated_at, created_by, updated_by
             FROM accounts WHERE code = ?"
        )
        .bind(code)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("Account", code))?;
        
        Ok(row.into_account()?)
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Account>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM accounts WHERE status != 'Deleted'")
            .fetch_one(pool)
            .await?;
        
        let rows = sqlx::query_as::<_, AccountRow>(
            "SELECT id, code, name, account_type, parent_id, status, description,
                    created_at, updated_at, created_by, updated_by
             FROM accounts 
             WHERE status != 'Deleted'
             ORDER BY code 
             LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await?;
        
        let items: Result<Vec<Account>> = rows.into_iter().map(|r| r.into_account()).collect();
        Ok(Paginated::new(items?, count.0 as u64, pagination))
    }

    async fn find_by_type(&self, pool: &SqlitePool, account_type: AccountType) -> Result<Vec<Account>> {
        let type_str = format!("{:?}", account_type);
        let rows = sqlx::query_as::<_, AccountRow>(
            "SELECT id, code, name, account_type, parent_id, status, description,
                    created_at, updated_at, created_by, updated_by
             FROM accounts 
             WHERE account_type = ? AND status = 'Active'
             ORDER BY code"
        )
        .bind(&type_str)
        .fetch_all(pool)
        .await?;
        
        Ok(rows.into_iter().map(|r| r.into_account()).collect::<Result<Vec<_>>>()?)
    }

    async fn create(&self, pool: &SqlitePool, account: Account) -> Result<Account> {
        let now = Utc::now();
        sqlx::query("INSERT INTO accounts (id, code, name, account_type, parent_id, status, description,
             created_at, updated_at, created_by, updated_by)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(account.base.id.to_string())
        .bind(&account.code)
        .bind(&account.name)
        .bind(format!("{:?}", account.account_type))
        .bind(account.parent_id.map(|id| id.to_string()))
        .bind(format!("{:?}", account.status))
        .bind(&account.description)
        .bind(account.base.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(account.base.created_by.map(|id| id.to_string()))
        .bind(account.base.updated_by.map(|id| id.to_string()))
        .execute(pool)
        .await?;
        
        Ok(account)
    }

    async fn update(&self, pool: &SqlitePool, account: Account) -> Result<Account> {
        let now = Utc::now();
        let rows = sqlx::query("UPDATE accounts SET code = ?, name = ?, account_type = ?, parent_id = ?, 
             status = ?, description = ?, updated_at = ?, updated_by = ?
             WHERE id = ?")
        .bind(&account.code)
        .bind(&account.name)
        .bind(format!("{:?}", account.account_type))
        .bind(account.parent_id.map(|id| id.to_string()))
        .bind(format!("{:?}", account.status))
        .bind(&account.description)
        .bind(now.to_rfc3339())
        .bind(account.base.updated_by.map(|id| id.to_string()))
        .bind(account.base.id.to_string())
        .execute(pool)
        .await?;
        
        if rows.rows_affected() == 0 {
            return Err(Error::not_found("Account", &account.base.id.to_string()));
        }
        
        Ok(account)
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let rows = sqlx::query("UPDATE accounts SET status = 'Deleted', updated_at = ? WHERE id = ?")
        .bind(Utc::now().to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await?;
        
        if rows.rows_affected() == 0 {
            return Err(Error::not_found("Account", &id.to_string()));
        }
        
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct JournalEntryRow {
    id: String,
    entry_number: String,
    date: String,
    description: String,
    reference: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
    created_by: Option<String>,
    updated_by: Option<String>,
}

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct JournalLineRow {
    id: String,
    journal_entry_id: String,
    account_id: String,
    debit: i64,
    credit: i64,
    description: Option<String>,
}

pub struct SqliteJournalEntryRepository;

#[async_trait]
impl JournalEntryRepository for SqliteJournalEntryRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<JournalEntry> {
        let row = sqlx::query_as::<_, JournalEntryRow>(
            "SELECT id, entry_number, date, description, reference, status,
                    created_at, updated_at, created_by, updated_by
             FROM journal_entries WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("JournalEntry", &id.to_string()))?;
        
        let lines = self.load_lines(pool, id).await?;
        self.row_to_entry(row, lines)
    }

    async fn find_by_number(&self, pool: &SqlitePool, number: &str) -> Result<JournalEntry> {
        let row = sqlx::query_as::<_, JournalEntryRow>(
            "SELECT id, entry_number, date, description, reference, status,
                    created_at, updated_at, created_by, updated_by
             FROM journal_entries WHERE entry_number = ?"
        )
        .bind(number)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("JournalEntry", number))?;
        
        let id = Uuid::parse_str(&row.id).unwrap_or_default();
        let lines = self.load_lines(pool, id).await?;
        self.row_to_entry(row, lines)
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<JournalEntry>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM journal_entries")
            .fetch_one(pool)
            .await?;
        
        let rows = sqlx::query_as::<_, JournalEntryRow>(
            "SELECT id, entry_number, date, description, reference, status,
                    created_at, updated_at, created_by, updated_by
             FROM journal_entries 
             ORDER BY date DESC, created_at DESC
             LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await?;
        
        let entry_ids: Vec<String> = rows.iter().map(|r| r.id.clone()).collect();
        let all_lines = self.load_lines_batch(pool, &entry_ids).await?;
        
        let mut entries = Vec::new();
        for row in rows {
            let _id = Uuid::parse_str(&row.id)
                .map_err(|_| Error::validation("Invalid journal entry ID format"))?;
            let lines = all_lines.get(&row.id).cloned().unwrap_or_default();
            entries.push(self.row_to_entry(row, lines)?);
        }
        
        Ok(Paginated::new(entries, count.0 as u64, pagination))
    }

    async fn create(&self, pool: &SqlitePool, entry: JournalEntry) -> Result<JournalEntry> {
        let now = Utc::now();
        let mut tx = pool.begin().await?;
        
        sqlx::query("INSERT INTO journal_entries (id, entry_number, date, description, reference, status,
             created_at, updated_at, created_by, updated_by)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(entry.base.id.to_string())
        .bind(&entry.entry_number)
        .bind(entry.date.to_rfc3339())
        .bind(&entry.description)
        .bind(&entry.reference)
        .bind(format!("{:?}", entry.status))
        .bind(entry.base.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(entry.base.created_by.map(|id| id.to_string()))
        .bind(entry.base.updated_by.map(|id| id.to_string()))
        .execute(&mut *tx)
        .await?;
        
        for line in &entry.lines {
            sqlx::query("INSERT INTO journal_lines (id, journal_entry_id, account_id, debit, credit, description)
                 VALUES (?, ?, ?, ?, ?, ?)")
            .bind(line.id.to_string())
            .bind(entry.base.id.to_string())
            .bind(line.account_id.to_string())
            .bind(line.debit.amount)
            .bind(line.credit.amount)
            .bind(&line.description)
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        Ok(entry)
    }

    async fn update(&self, pool: &SqlitePool, entry: JournalEntry) -> Result<JournalEntry> {
        let existing = self.find_by_id(pool, entry.base.id).await?;
        if existing.status != Status::Draft {
            return Err(Error::business_rule("Cannot modify posted journal entry"));
        }
        
        let now = Utc::now();
        let mut tx = pool.begin().await?;
        
        sqlx::query("UPDATE journal_entries SET description = ?, reference = ?, updated_at = ?, updated_by = ?
             WHERE id = ?")
        .bind(&entry.description)
        .bind(&entry.reference)
        .bind(now.to_rfc3339())
        .bind(entry.base.updated_by.map(|id| id.to_string()))
        .bind(entry.base.id.to_string())
        .execute(&mut *tx)
        .await?;
        
        sqlx::query("DELETE FROM journal_lines WHERE journal_entry_id = ?")
            .bind(entry.base.id.to_string())
            .execute(&mut *tx)
            .await?;
        
        for line in &entry.lines {
            sqlx::query("INSERT INTO journal_lines (id, journal_entry_id, account_id, debit, credit, description)
                 VALUES (?, ?, ?, ?, ?, ?)")
            .bind(line.id.to_string())
            .bind(entry.base.id.to_string())
            .bind(line.account_id.to_string())
            .bind(line.debit.amount)
            .bind(line.credit.amount)
            .bind(&line.description)
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        Ok(entry)
    }

    async fn post(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let entry = self.find_by_id(pool, id).await?;
        
        let total_debits: i64 = entry.lines.iter().map(|l| l.debit.amount).sum();
        let total_credits: i64 = entry.lines.iter().map(|l| l.credit.amount).sum();
        
        if total_debits != total_credits {
            return Err(Error::business_rule("Journal entry must balance (debits must equal credits)"));
        }
        
        let rows = sqlx::query("UPDATE journal_entries SET status = 'Posted', updated_at = ? WHERE id = ? AND status = 'Draft'")
        .bind(Utc::now().to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await?;
        
        if rows.rows_affected() == 0 {
            return Err(Error::business_rule("Journal entry not found or already posted"));
        }
        
        Ok(())
    }
}

impl SqliteJournalEntryRepository {
    async fn load_lines(&self, pool: &SqlitePool, entry_id: Uuid) -> Result<Vec<JournalLine>> {
        use erp_core::{Money, Currency};
        
        let rows = sqlx::query_as::<_, JournalLineRow>(
            "SELECT id, journal_entry_id, account_id, debit, credit, description
             FROM journal_lines WHERE journal_entry_id = ?"
        )
        .bind(entry_id.to_string())
        .fetch_all(pool)
        .await?;
        
        rows.into_iter().map(|r| {
            let id = Uuid::parse_str(&r.id)
                .map_err(|_| Error::validation("Invalid journal line ID format"))?;
            let account_id = Uuid::parse_str(&r.account_id)
                .map_err(|_| Error::validation("Invalid account ID format in journal line"))?;
            Ok(JournalLine {
                id,
                account_id,
                debit: Money::new(r.debit, Currency::USD),
                credit: Money::new(r.credit, Currency::USD),
                description: r.description,
            })
        }).collect()
    }

    async fn load_lines_batch(&self, pool: &SqlitePool, entry_ids: &[String]) -> Result<std::collections::HashMap<String, Vec<JournalLine>>> {
        use erp_core::{Money, Currency};
        use std::collections::HashMap;
        
        if entry_ids.is_empty() {
            return Ok(HashMap::new());
        }
        
        let placeholders = entry_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query = format!(
            "SELECT id, journal_entry_id, account_id, debit, credit, description
             FROM journal_lines WHERE journal_entry_id IN ({})",
            placeholders
        );
        
        let mut sql_query = sqlx::query_as::<_, JournalLineRow>(&query);
        for id in entry_ids {
            sql_query = sql_query.bind(id);
        }
        
        let rows = sql_query.fetch_all(pool).await?;
        
        let mut result: HashMap<String, Vec<JournalLine>> = HashMap::new();
        for r in rows {
            let id = match Uuid::parse_str(&r.id) {
                Ok(id) => id,
                Err(_) => continue,
            };
            let account_id = match Uuid::parse_str(&r.account_id) {
                Ok(id) => id,
                Err(_) => continue,
            };
            let line = JournalLine {
                id,
                account_id,
                debit: Money::new(r.debit, Currency::USD),
                credit: Money::new(r.credit, Currency::USD),
                description: r.description,
            };
            result.entry(r.journal_entry_id).or_default().push(line);
        }
        
        Ok(result)
    }

    fn row_to_entry(&self, row: JournalEntryRow, lines: Vec<JournalLine>) -> Result<JournalEntry> {
        let id = Uuid::parse_str(&row.id)
            .map_err(|_| Error::validation("Invalid journal entry ID format"))?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&row.created_at)
            .map(|d| d.with_timezone(&Utc))
            .map_err(|_| Error::validation("Invalid created_at timestamp"))?;
        let updated_at = chrono::DateTime::parse_from_rfc3339(&row.updated_at)
            .map(|d| d.with_timezone(&Utc))
            .map_err(|_| Error::validation("Invalid updated_at timestamp"))?;
        let date = chrono::DateTime::parse_from_rfc3339(&row.date)
            .map(|d| d.with_timezone(&Utc))
            .map_err(|_| Error::validation("Invalid date timestamp"))?;
        Ok(JournalEntry {
            base: BaseEntity {
                id,
                created_at,
                updated_at,
                created_by: row.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: row.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            entry_number: row.entry_number,
            date,
            description: row.description,
            reference: row.reference,
            lines,
            status: match row.status.as_str() {
                "Posted" => Status::Approved,
                _ => Status::Draft,
            },
        })
    }
}

#[derive(sqlx::FromRow)]
struct FiscalYearRow {
    id: String,
    name: String,
    start_date: String,
    end_date: String,
    status: String,
    created_at: String,
    updated_at: String,
}

pub struct SqliteFiscalYearRepository;

#[async_trait]
impl FiscalYearRepository for SqliteFiscalYearRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<FiscalYear> {
        let row = sqlx::query_as::<_, FiscalYearRow>(
            "SELECT id, name, start_date, end_date, status, created_at, updated_at
             FROM fiscal_years WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("FiscalYear", &id.to_string()))?;
        
        self.row_to_fiscal_year(row)
    }

    async fn find_active(&self, pool: &SqlitePool) -> Result<FiscalYear> {
        let row = sqlx::query_as::<_, FiscalYearRow>(
            "SELECT id, name, start_date, end_date, status, created_at, updated_at
             FROM fiscal_years WHERE status = 'Active' ORDER BY start_date DESC LIMIT 1"
        )
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("FiscalYear", "active"))?;
        
        self.row_to_fiscal_year(row)
    }

    async fn find_all(&self, pool: &SqlitePool) -> Result<Vec<FiscalYear>> {
        let rows = sqlx::query_as::<_, FiscalYearRow>(
            "SELECT id, name, start_date, end_date, status, created_at, updated_at
             FROM fiscal_years ORDER BY start_date DESC"
        )
        .fetch_all(pool)
        .await?;
        
        rows.into_iter().map(|r| self.row_to_fiscal_year(r)).collect()
    }

    async fn create(&self, pool: &SqlitePool, year: FiscalYear) -> Result<FiscalYear> {
        let now = Utc::now();
        sqlx::query("INSERT INTO fiscal_years (id, name, start_date, end_date, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)")
        .bind(year.base.id.to_string())
        .bind(&year.name)
        .bind(year.start_date.to_rfc3339())
        .bind(year.end_date.to_rfc3339())
        .bind(format!("{:?}", year.status))
        .bind(year.base.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;
        
        Ok(year)
    }

    async fn update(&self, pool: &SqlitePool, year: FiscalYear) -> Result<FiscalYear> {
        let now = Utc::now();
        let rows = sqlx::query("UPDATE fiscal_years SET name = ?, start_date = ?, end_date = ?, status = ?, updated_at = ?
             WHERE id = ?")
        .bind(&year.name)
        .bind(year.start_date.to_rfc3339())
        .bind(year.end_date.to_rfc3339())
        .bind(format!("{:?}", year.status))
        .bind(now.to_rfc3339())
        .bind(year.base.id.to_string())
        .execute(pool)
        .await?;
        
        if rows.rows_affected() == 0 {
            return Err(Error::not_found("FiscalYear", &year.base.id.to_string()));
        }
        
        Ok(year)
    }
}

impl SqliteFiscalYearRepository {
    fn row_to_fiscal_year(&self, row: FiscalYearRow) -> Result<FiscalYear> {
        let id = Uuid::parse_str(&row.id)
            .map_err(|_| Error::validation("Invalid fiscal year ID format"))?;
        let created_at = chrono::DateTime::parse_from_rfc3339(&row.created_at)
            .map(|d| d.with_timezone(&Utc))
            .map_err(|_| Error::validation("Invalid created_at timestamp"))?;
        let updated_at = chrono::DateTime::parse_from_rfc3339(&row.updated_at)
            .map(|d| d.with_timezone(&Utc))
            .map_err(|_| Error::validation("Invalid updated_at timestamp"))?;
        let start_date = chrono::DateTime::parse_from_rfc3339(&row.start_date)
            .map(|d| d.with_timezone(&Utc))
            .map_err(|_| Error::validation("Invalid start_date timestamp"))?;
        let end_date = chrono::DateTime::parse_from_rfc3339(&row.end_date)
            .map(|d| d.with_timezone(&Utc))
            .map_err(|_| Error::validation("Invalid end_date timestamp"))?;
        Ok(FiscalYear {
            base: BaseEntity {
                id,
                created_at,
                updated_at,
                created_by: None,
                updated_by: None,
            },
            name: row.name,
            start_date,
            end_date,
            status: match row.status.as_str() {
                "Closed" => Status::Completed,
                _ => Status::Active,
            },
        })
    }
}

#[async_trait]
pub trait AccountRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Account>;
    async fn find_by_code(&self, pool: &SqlitePool, code: &str) -> Result<Account>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Account>>;
    async fn find_by_type(&self, pool: &SqlitePool, account_type: AccountType) -> Result<Vec<Account>>;
    async fn create(&self, pool: &SqlitePool, account: Account) -> Result<Account>;
    async fn update(&self, pool: &SqlitePool, account: Account) -> Result<Account>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait JournalEntryRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<JournalEntry>;
    async fn find_by_number(&self, pool: &SqlitePool, number: &str) -> Result<JournalEntry>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<JournalEntry>>;
    async fn create(&self, pool: &SqlitePool, entry: JournalEntry) -> Result<JournalEntry>;
    async fn update(&self, pool: &SqlitePool, entry: JournalEntry) -> Result<JournalEntry>;
    async fn post(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait FiscalYearRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<FiscalYear>;
    async fn find_active(&self, pool: &SqlitePool) -> Result<FiscalYear>;
    async fn find_all(&self, pool: &SqlitePool) -> Result<Vec<FiscalYear>>;
    async fn create(&self, pool: &SqlitePool, year: FiscalYear) -> Result<FiscalYear>;
    async fn update(&self, pool: &SqlitePool, year: FiscalYear) -> Result<FiscalYear>;
}
