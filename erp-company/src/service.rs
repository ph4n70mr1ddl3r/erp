use crate::models::*;
use crate::repository::*;
use anyhow::Result;
use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct CompanyService;

impl CompanyService {
    pub async fn create(pool: &SqlitePool, req: CreateCompanyRequest) -> Result<Company> {
        let now = Utc::now();
        let company = Company {
            id: Uuid::new_v4(),
            code: req.code,
            name: req.name,
            legal_name: req.legal_name,
            company_type: req.company_type,
            parent_id: req.parent_id,
            tax_id: req.tax_id,
            registration_number: req.registration_number,
            currency: req.currency,
            fiscal_year_start: req.fiscal_year_start,
            consolidation_method: req.consolidation_method,
            ownership_percentage: req.ownership_percentage,
            street: req.street,
            city: req.city,
            state: req.state,
            postal_code: req.postal_code,
            country: req.country,
            phone: req.phone,
            email: req.email,
            website: req.website,
            status: "Active".to_string(),
            created_at: now,
            updated_at: now,
        };
        CompanyRepository::create(pool, &company).await?;
        Ok(company)
    }

    pub async fn get(pool: &SqlitePool, id: Uuid) -> Result<Option<Company>> {
        CompanyRepository::get_by_id(pool, id).await
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<Company>> {
        CompanyRepository::list_all(pool).await
    }

    pub async fn get_children(pool: &SqlitePool, parent_id: Uuid) -> Result<Vec<Company>> {
        let all = CompanyRepository::list_all(pool).await?;
        Ok(all.into_iter().filter(|c| c.parent_id == Some(parent_id)).collect())
    }

    pub async fn get_company_tree(pool: &SqlitePool, company_id: Uuid) -> Result<Vec<Company>> {
        let mut result = vec![];
        let all = CompanyRepository::list_all(pool).await?;
        fn collect_children(companies: &[Company], parent_id: Uuid, result: &mut Vec<Company>) {
            for c in companies {
                if c.parent_id == Some(parent_id) {
                    result.push(c.clone());
                    collect_children(companies, c.id, result);
                }
            }
        }
        collect_children(&all, company_id, &mut result);
        Ok(result)
    }
}

pub struct IntercompanyService;

impl IntercompanyService {
    pub async fn create(pool: &SqlitePool, req: CreateIntercompanyRequest, user_id: Uuid) -> Result<IntercompanyTransaction> {
        let now = Utc::now();
        let transaction_number = format!("IC-{}", now.format("%Y%m%d%H%M%S"));
        let base_amount = (req.amount as f64 * req.exchange_rate) as i64;
        
        let txn = IntercompanyTransaction {
            id: Uuid::new_v4(),
            transaction_number,
            from_company_id: req.from_company_id,
            to_company_id: req.to_company_id,
            transaction_type: req.transaction_type,
            reference_type: req.reference_type,
            reference_id: req.reference_id,
            amount: req.amount,
            currency: req.currency,
            exchange_rate: req.exchange_rate,
            base_amount,
            description: req.description,
            due_date: req.due_date,
            status: "Pending".to_string(),
            elimination_entry_id: None,
            created_at: now,
            created_by: Some(user_id),
        };
        IntercompanyRepository::create(pool, &txn).await?;
        Ok(txn)
    }

    pub async fn list(pool: &SqlitePool) -> Result<Vec<IntercompanyTransaction>> {
        IntercompanyRepository::list_all(pool).await
    }

    pub async fn get_pending_eliminations(pool: &SqlitePool) -> Result<Vec<IntercompanyTransaction>> {
        let all = IntercompanyRepository::list_all(pool).await?;
        Ok(all.into_iter().filter(|t| t.elimination_entry_id.is_none()).collect())
    }
}

pub struct ConsolidationService;

impl ConsolidationService {
    pub async fn create_consolidation(pool: &SqlitePool, name: String, period_start: chrono::DateTime<Utc>, period_end: chrono::DateTime<Utc>, user_id: Uuid) -> Result<Consolidation> {
        let now = Utc::now();
        let consolidation = Consolidation {
            id: Uuid::new_v4(),
            name,
            period_start,
            period_end,
            status: "Draft".to_string(),
            total_eliminations: 0,
            created_at: now,
            created_by: Some(user_id),
        };
        sqlx::query(
            r#"INSERT INTO consolidations (id, name, period_start, period_end, status, total_eliminations, created_at, created_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(consolidation.id.to_string())
        .bind(&consolidation.name)
        .bind(consolidation.period_start.to_rfc3339())
        .bind(consolidation.period_end.to_rfc3339())
        .bind(&consolidation.status)
        .bind(consolidation.total_eliminations)
        .bind(consolidation.created_at.to_rfc3339())
        .bind(consolidation.created_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(consolidation)
    }

    pub async fn run_eliminations(pool: &SqlitePool, consolidation_id: Uuid) -> Result<i64> {
        let pending = IntercompanyService::get_pending_eliminations(pool).await?;
        let mut total_eliminations = 0i64;
        
        for txn in pending {
            let entry = ConsolidationEntry {
                id: Uuid::new_v4(),
                consolidation_id,
                company_id: txn.from_company_id,
                account_code: "IC-ELIM".to_string(),
                debit: if txn.amount > 0 { txn.amount.abs() } else { 0 },
                credit: if txn.amount < 0 { txn.amount.abs() } else { 0 },
                elimination_type: "Intercompany".to_string(),
                description: format!("Elimination: {}", txn.description),
                created_at: Utc::now(),
            };
            
            sqlx::query(
                r#"INSERT INTO consolidation_entries (id, consolidation_id, company_id, account_code, debit, credit, elimination_type, description, created_at)
                   VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
            )
            .bind(entry.id.to_string())
            .bind(entry.consolidation_id.to_string())
            .bind(entry.company_id.to_string())
            .bind(&entry.account_code)
            .bind(entry.debit)
            .bind(entry.credit)
            .bind(&entry.elimination_type)
            .bind(&entry.description)
            .bind(entry.created_at.to_rfc3339())
            .execute(pool).await?;
            
            total_eliminations += entry.debit;
            
            sqlx::query("UPDATE intercompany_transactions SET elimination_entry_id = ? WHERE id = ?")
                .bind(entry.id.to_string())
                .bind(txn.id.to_string())
                .execute(pool).await?;
        }
        
        sqlx::query("UPDATE consolidations SET total_eliminations = ?, status = 'Completed' WHERE id = ?")
            .bind(total_eliminations)
            .bind(consolidation_id.to_string())
            .execute(pool).await?;
        
        Ok(total_eliminations)
    }
}
