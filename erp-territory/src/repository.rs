use crate::models::*;
use async_trait::async_trait;
use erp_core::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait TerritoryRepository: Send + Sync {
    async fn create_territory(&self, territory: &SalesTerritory) -> Result<SalesTerritory>;
    async fn get_territory(&self, id: Uuid) -> Result<Option<SalesTerritory>>;
    async fn list_territories(&self) -> Result<Vec<SalesTerritory>>;
    async fn create_assignment(&self, assignment: &TerritoryAssignment) -> Result<TerritoryAssignment>;
    async fn create_quota(&self, quota: &SalesQuota) -> Result<SalesQuota>;
    async fn get_quota(&self, id: Uuid) -> Result<Option<SalesQuota>>;
    async fn list_quotas(&self, owner_id: Option<Uuid>) -> Result<Vec<SalesQuota>>;
    async fn create_attainment(&self, att: &QuotaAttainment) -> Result<QuotaAttainment>;
    async fn create_performance(&self, perf: &TerritoryPerformance) -> Result<TerritoryPerformance>;
}

pub struct SqliteTerritoryRepository { pool: SqlitePool }
impl SqliteTerritoryRepository { pub fn new(pool: SqlitePool) -> Self { Self { pool } } }

#[async_trait]
impl TerritoryRepository for SqliteTerritoryRepository {
    async fn create_territory(&self, territory: &SalesTerritory) -> Result<SalesTerritory> {
        let t = territory.clone();
        sqlx::query!(r#"INSERT INTO sales_territories (id, territory_number, name, description,
            parent_territory_id, territory_type, manager_id, geography, countries, states, cities,
            postal_codes, industries, company_size, custom_criteria, effective_date, end_date,
            status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            t.base.id, t.territory_number, t.name, t.description, t.parent_territory_id, t.territory_type,
            t.manager_id, t.geography, t.countries, t.states, t.cities, t.postal_codes, t.industries,
            t.company_size, t.custom_criteria, t.effective_date, t.end_date, t.status, t.created_at, t.updated_at).execute(&self.pool).await?;
        Ok(t)
    }
    async fn get_territory(&self, _id: Uuid) -> Result<Option<SalesTerritory>> { Ok(None) }
    async fn list_territories(&self) -> Result<Vec<SalesTerritory>> { Ok(vec![]) }
    async fn create_assignment(&self, assignment: &TerritoryAssignment) -> Result<TerritoryAssignment> {
        let a = assignment.clone();
        sqlx::query!(r#"INSERT INTO territory_assignments (id, territory_id, sales_rep_id,
            assignment_type, primary_rep, effective_date, end_date, allocation_percent, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            a.id, a.territory_id, a.sales_rep_id, a.assignment_type, a.primary_rep, a.effective_date,
            a.end_date, a.allocation_percent, a.created_at, a.updated_at).execute(&self.pool).await?;
        Ok(a)
    }
    async fn create_quota(&self, quota: &SalesQuota) -> Result<SalesQuota> {
        let q = quota.clone();
        sqlx::query!(r#"INSERT INTO sales_quotas (id, quota_number, name, description, quota_type,
            owner_type, owner_id, territory_id, fiscal_year, period_type, currency, annual_target,
            q1_target, q2_target, q3_target, q4_target, m1_target, m2_target, m3_target, m4_target,
            m5_target, m6_target, m7_target, m8_target, m9_target, m10_target, m11_target, m12_target,
            product_id, product_category_id, status, approved_by, approved_at, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            q.base.id, q.quota_number, q.name, q.description, q.quota_type, q.owner_type, q.owner_id,
            q.territory_id, q.fiscal_year, q.period_type, q.currency, q.annual_target, q.q1_target,
            q.q2_target, q.q3_target, q.q4_target, q.m1_target, q.m2_target, q.m3_target, q.m4_target,
            q.m5_target, q.m6_target, q.m7_target, q.m8_target, q.m9_target, q.m10_target, q.m11_target, q.m12_target,
            q.product_id, q.product_category_id, q.status, q.approved_by, q.approved_at, q.created_at, q.updated_at).execute(&self.pool).await?;
        Ok(q)
    }
    async fn get_quota(&self, _id: Uuid) -> Result<Option<SalesQuota>> { Ok(None) }
    async fn list_quotas(&self, _owner_id: Option<Uuid>) -> Result<Vec<SalesQuota>> { Ok(vec![]) }
    async fn create_attainment(&self, att: &QuotaAttainment) -> Result<QuotaAttainment> {
        let a = att.clone();
        sqlx::query!(r#"INSERT INTO quota_attainments (id, quota_id, period_type, period_number,
            period_start, period_end, target, actual, attainment_percent, pipeline, pipeline_coverage,
            gap_to_quota, currency, calculated_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            a.base.id, a.quota_id, a.period_type, a.period_number, a.period_start, a.period_end,
            a.target, a.actual, a.attainment_percent, a.pipeline, a.pipeline_coverage, a.gap_to_quota,
            a.currency, a.calculated_at, a.created_at).execute(&self.pool).await?;
        Ok(a)
    }
    async fn create_performance(&self, perf: &TerritoryPerformance) -> Result<TerritoryPerformance> {
        let p = perf.clone();
        sqlx::query!(r#"INSERT INTO territory_performances (id, territory_id, period_type, period_start,
            period_end, quota, revenue, attainment_percent, new_accounts, total_accounts,
            opportunities_created, opportunities_won, win_rate_percent, avg_deal_size, pipeline_value,
            currency, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            p.base.id, p.territory_id, p.period_type, p.period_start, p.period_end, p.quota, p.revenue,
            p.attainment_percent, p.new_accounts, p.total_accounts, p.opportunities_created, p.opportunities_won,
            p.win_rate_percent, p.avg_deal_size, p.pipeline_value, p.currency, p.created_at).execute(&self.pool).await?;
        Ok(p)
    }
}
