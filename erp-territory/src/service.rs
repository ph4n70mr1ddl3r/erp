use crate::models::*;
use crate::repository::{TerritoryRepository, SqliteTerritoryRepository};
use chrono::{NaiveDate, Utc};
use erp_core::{BaseEntity, Result};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct TerritoryService { repo: SqliteTerritoryRepository }
impl TerritoryService {
    pub fn new(pool: SqlitePool) -> Self { Self { repo: SqliteTerritoryRepository::new(pool) } }

    pub async fn create_territory(&self, pool: &SqlitePool, req: CreateTerritoryRequest) -> Result<SalesTerritory> {
        let territory = SalesTerritory {
            base: BaseEntity::new(),
            territory_number: format!("TER-{}", Uuid::new_v4()),
            name: req.name,
            description: req.description,
            parent_territory_id: req.parent_territory_id,
            territory_type: req.territory_type,
            manager_id: req.manager_id,
            geography: req.geography,
            countries: req.countries,
            states: req.states,
            cities: req.cities,
            postal_codes: req.postal_codes,
            industries: req.industries,
            company_size: req.company_size,
            custom_criteria: req.custom_criteria,
            effective_date: req.effective_date.unwrap_or_else(|| Utc::now().date_naive()),
            end_date: req.end_date,
            status: erp_core::Status::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_territory(&territory).await
    }

    pub async fn assign_rep(&self, pool: &SqlitePool, req: AssignRepRequest) -> Result<TerritoryAssignment> {
        let assignment = TerritoryAssignment {
            id: Uuid::new_v4(),
            territory_id: req.territory_id,
            sales_rep_id: req.sales_rep_id,
            assignment_type: req.assignment_type,
            primary_rep: req.primary_rep.unwrap_or(false),
            effective_date: req.effective_date.unwrap_or_else(|| Utc::now().date_naive()),
            end_date: req.end_date,
            allocation_percent: req.allocation_percent.unwrap_or(100),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_assignment(&assignment).await
    }

    pub async fn create_quota(&self, pool: &SqlitePool, req: CreateQuotaRequest) -> Result<SalesQuota> {
        let annual = req.annual_target;
        let quarterly = annual / 4;
        let monthly = annual / 12;
        let quota = SalesQuota {
            base: BaseEntity::new(),
            quota_number: format!("QUO-{}", Uuid::new_v4()),
            name: req.name,
            description: req.description,
            quota_type: req.quota_type,
            owner_type: req.owner_type,
            owner_id: req.owner_id,
            territory_id: req.territory_id,
            fiscal_year: req.fiscal_year,
            period_type: req.period_type.unwrap_or(QuotaPeriodType::Monthly),
            currency: req.currency,
            annual_target: annual,
            q1_target: quarterly, q2_target: quarterly, q3_target: quarterly, q4_target: quarterly,
            m1_target: monthly, m2_target: monthly, m3_target: monthly, m4_target: monthly,
            m5_target: monthly, m6_target: monthly, m7_target: monthly, m8_target: monthly,
            m9_target: monthly, m10_target: monthly, m11_target: monthly, m12_target: monthly,
            product_id: req.product_id,
            product_category_id: req.product_category_id,
            status: QuotaStatus::Draft,
            approved_by: None,
            approved_at: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_quota(&quota).await
    }

    pub async fn record_attainment(&self, pool: &SqlitePool, quota_id: Uuid, period_type: QuotaPeriodType, period_number: i32, target: i64, actual: i64) -> Result<QuotaAttainment> {
        let attainment = QuotaAttainment {
            base: BaseEntity::new(),
            quota_id,
            period_type,
            period_number,
            period_start: Utc::now().date_naive(),
            period_end: Utc::now().date_naive(),
            target,
            actual,
            attainment_percent: if target > 0 { (actual as f64 / target as f64) * 100.0 } else { 0.0 },
            pipeline: 0,
            pipeline_coverage: 0.0,
            gap_to_quota: target - actual,
            currency: "USD".to_string(),
            calculated_at: Utc::now(),
            created_at: Utc::now(),
        };
        self.repo.create_attainment(&attainment).await
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateTerritoryRequest {
    pub name: String,
    pub description: Option<String>,
    pub parent_territory_id: Option<Uuid>,
    pub territory_type: TerritoryType,
    pub manager_id: Option<Uuid>,
    pub geography: Option<String>,
    pub countries: Option<String>,
    pub states: Option<String>,
    pub cities: Option<String>,
    pub postal_codes: Option<String>,
    pub industries: Option<String>,
    pub company_size: Option<String>,
    pub custom_criteria: Option<String>,
    pub effective_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, serde::Deserialize)]
pub struct AssignRepRequest {
    pub territory_id: Uuid,
    pub sales_rep_id: Uuid,
    pub assignment_type: AssignmentType,
    pub primary_rep: Option<bool>,
    pub effective_date: Option<NaiveDate>,
    pub end_date: Option<NaiveDate>,
    pub allocation_percent: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateQuotaRequest {
    pub name: String,
    pub description: Option<String>,
    pub quota_type: QuotaType,
    pub owner_type: QuotaOwnerType,
    pub owner_id: Uuid,
    pub territory_id: Option<Uuid>,
    pub fiscal_year: i32,
    pub period_type: Option<QuotaPeriodType>,
    pub currency: String,
    pub annual_target: i64,
    pub product_id: Option<Uuid>,
    pub product_category_id: Option<Uuid>,
}
