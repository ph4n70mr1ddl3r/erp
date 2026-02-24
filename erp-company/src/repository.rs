use crate::models::*;
use anyhow::Result;
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

pub struct CompanyRepository;

impl CompanyRepository {
    pub async fn create(pool: &SqlitePool, company: &Company) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO companies (id, code, name, legal_name, company_type, parent_id, tax_id, registration_number, currency, fiscal_year_start, consolidation_method, ownership_percentage, street, city, state, postal_code, country, phone, email, website, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(company.id.to_string())
        .bind(&company.code)
        .bind(&company.name)
        .bind(&company.legal_name)
        .bind(format!("{:?}", company.company_type))
        .bind(company.parent_id.map(|id| id.to_string()))
        .bind(&company.tax_id)
        .bind(&company.registration_number)
        .bind(&company.currency)
        .bind(company.fiscal_year_start)
        .bind(format!("{:?}", company.consolidation_method))
        .bind(company.ownership_percentage)
        .bind(&company.street)
        .bind(&company.city)
        .bind(&company.state)
        .bind(&company.postal_code)
        .bind(&company.country)
        .bind(&company.phone)
        .bind(&company.email)
        .bind(&company.website)
        .bind(&company.status)
        .bind(company.created_at.to_rfc3339())
        .bind(company.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(())
    }

    pub async fn get_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<Company>> {
        let row = sqlx::query(
            r#"SELECT id, code, name, legal_name, company_type, parent_id, tax_id, registration_number, currency, fiscal_year_start, consolidation_method, ownership_percentage, street, city, state, postal_code, country, phone, email, website, status, created_at, updated_at FROM companies WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(pool).await?;
        
        Ok(row.map(|r| Company {
            id: Uuid::parse_str(r.get::<String, _>("id").as_str()).unwrap(),
            code: r.get("code"),
            name: r.get("name"),
            legal_name: r.get("legal_name"),
            company_type: match r.get::<String, _>("company_type").as_str() {
                "Parent" => CompanyType::Parent,
                "Subsidiary" => CompanyType::Subsidiary,
                "Division" => CompanyType::Division,
                _ => CompanyType::Branch,
            },
            parent_id: r.get::<Option<String>, _>("parent_id").and_then(|s| Uuid::parse_str(&s).ok()),
            tax_id: r.get("tax_id"),
            registration_number: r.get("registration_number"),
            currency: r.get("currency"),
            fiscal_year_start: r.get("fiscal_year_start"),
            consolidation_method: match r.get::<String, _>("consolidation_method").as_str() {
                "Full" => ConsolidationMethod::Full,
                "Equity" => ConsolidationMethod::Equity,
                "Proportional" => ConsolidationMethod::Proportional,
                _ => ConsolidationMethod::None,
            },
            ownership_percentage: r.get("ownership_percentage"),
            street: r.get("street"),
            city: r.get("city"),
            state: r.get("state"),
            postal_code: r.get("postal_code"),
            country: r.get("country"),
            phone: r.get("phone"),
            email: r.get("email"),
            website: r.get("website"),
            status: r.get("status"),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
        }))
    }

    pub async fn list_all(pool: &SqlitePool) -> Result<Vec<Company>> {
        let rows = sqlx::query(
            r#"SELECT id, code, name, legal_name, company_type, parent_id, tax_id, registration_number, currency, fiscal_year_start, consolidation_method, ownership_percentage, street, city, state, postal_code, country, phone, email, website, status, created_at, updated_at FROM companies ORDER BY code"#
        )
        .fetch_all(pool).await?;
        
        Ok(rows.iter().map(|r| Company {
            id: Uuid::parse_str(r.get::<String, _>("id").as_str()).unwrap(),
            code: r.get("code"),
            name: r.get("name"),
            legal_name: r.get("legal_name"),
            company_type: match r.get::<String, _>("company_type").as_str() {
                "Parent" => CompanyType::Parent,
                "Subsidiary" => CompanyType::Subsidiary,
                "Division" => CompanyType::Division,
                _ => CompanyType::Branch,
            },
            parent_id: r.get::<Option<String>, _>("parent_id").and_then(|s| Uuid::parse_str(&s).ok()),
            tax_id: r.get("tax_id"),
            registration_number: r.get("registration_number"),
            currency: r.get("currency"),
            fiscal_year_start: r.get("fiscal_year_start"),
            consolidation_method: match r.get::<String, _>("consolidation_method").as_str() {
                "Full" => ConsolidationMethod::Full,
                "Equity" => ConsolidationMethod::Equity,
                "Proportional" => ConsolidationMethod::Proportional,
                _ => ConsolidationMethod::None,
            },
            ownership_percentage: r.get("ownership_percentage"),
            street: r.get("street"),
            city: r.get("city"),
            state: r.get("state"),
            postal_code: r.get("postal_code"),
            country: r.get("country"),
            phone: r.get("phone"),
            email: r.get("email"),
            website: r.get("website"),
            status: r.get("status"),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
        }).collect())
    }
}

pub struct IntercompanyRepository;

impl IntercompanyRepository {
    pub async fn create(pool: &SqlitePool, txn: &IntercompanyTransaction) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO intercompany_transactions (id, transaction_number, from_company_id, to_company_id, transaction_type, reference_type, reference_id, amount, currency, exchange_rate, base_amount, description, due_date, status, elimination_entry_id, created_at, created_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(txn.id.to_string())
        .bind(&txn.transaction_number)
        .bind(txn.from_company_id.to_string())
        .bind(txn.to_company_id.to_string())
        .bind(&txn.transaction_type)
        .bind(&txn.reference_type)
        .bind(txn.reference_id.map(|id| id.to_string()))
        .bind(txn.amount)
        .bind(&txn.currency)
        .bind(txn.exchange_rate)
        .bind(txn.base_amount)
        .bind(&txn.description)
        .bind(txn.due_date.map(|d| d.to_rfc3339()))
        .bind(&txn.status)
        .bind(txn.elimination_entry_id.map(|id| id.to_string()))
        .bind(txn.created_at.to_rfc3339())
        .bind(txn.created_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(())
    }

    pub async fn list_all(pool: &SqlitePool) -> Result<Vec<IntercompanyTransaction>> {
        let rows = sqlx::query(
            r#"SELECT id, transaction_number, from_company_id, to_company_id, transaction_type, reference_type, reference_id, amount, currency, exchange_rate, base_amount, description, due_date, status, elimination_entry_id, created_at, created_by FROM intercompany_transactions ORDER BY created_at DESC"#
        )
        .fetch_all(pool).await?;
        
        Ok(rows.iter().map(|r| IntercompanyTransaction {
            id: Uuid::parse_str(r.get::<String, _>("id").as_str()).unwrap(),
            transaction_number: r.get("transaction_number"),
            from_company_id: Uuid::parse_str(r.get::<String, _>("from_company_id").as_str()).unwrap(),
            to_company_id: Uuid::parse_str(r.get::<String, _>("to_company_id").as_str()).unwrap(),
            transaction_type: r.get("transaction_type"),
            reference_type: r.get("reference_type"),
            reference_id: r.get::<Option<String>, _>("reference_id").and_then(|s| Uuid::parse_str(&s).ok()),
            amount: r.get("amount"),
            currency: r.get("currency"),
            exchange_rate: r.get("exchange_rate"),
            base_amount: r.get("base_amount"),
            description: r.get("description"),
            due_date: r.get::<Option<String>, _>("due_date").and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&chrono::Utc))),
            status: r.get("status"),
            elimination_entry_id: r.get::<Option<String>, _>("elimination_entry_id").and_then(|s| Uuid::parse_str(&s).ok()),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
            created_by: r.get::<Option<String>, _>("created_by").and_then(|s| Uuid::parse_str(&s).ok()),
        }).collect())
    }
}
