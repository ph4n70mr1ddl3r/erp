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
        sqlx::query(r#"INSERT INTO sales_territories (id, territory_number, name, description,
            parent_territory_id, territory_type, manager_id, geography, countries, states, cities,
            postal_codes, industries, company_size, custom_criteria, effective_date, end_date,
            status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(t.base.id)
            .bind(&t.territory_number)
            .bind(&t.name)
            .bind(&t.description)
            .bind(t.parent_territory_id)
            .bind(&t.territory_type)
            .bind(t.manager_id)
            .bind(&t.geography)
            .bind(&t.countries)
            .bind(&t.states)
            .bind(&t.cities)
            .bind(&t.postal_codes)
            .bind(&t.industries)
            .bind(&t.company_size)
            .bind(&t.custom_criteria)
            .bind(t.effective_date)
            .bind(t.end_date)
            .bind(&t.status)
            .bind(t.created_at)
            .bind(t.updated_at)
            .execute(&self.pool).await?;
        Ok(t)
    }
    async fn get_territory(&self, _id: Uuid) -> Result<Option<SalesTerritory>> { Ok(None) }
    async fn list_territories(&self) -> Result<Vec<SalesTerritory>> { Ok(vec![]) }
    async fn create_assignment(&self, assignment: &TerritoryAssignment) -> Result<TerritoryAssignment> {
        let a = assignment.clone();
        sqlx::query(r#"INSERT INTO territory_assignments (id, territory_id, sales_rep_id,
            assignment_type, primary_rep, effective_date, end_date, allocation_percent, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(a.id)
            .bind(a.territory_id)
            .bind(a.sales_rep_id)
            .bind(&a.assignment_type)
            .bind(a.primary_rep)
            .bind(a.effective_date)
            .bind(a.end_date)
            .bind(a.allocation_percent)
            .bind(a.created_at)
            .bind(a.updated_at)
            .execute(&self.pool).await?;
        Ok(a)
    }
    async fn create_quota(&self, quota: &SalesQuota) -> Result<SalesQuota> {
        let q = quota.clone();
        sqlx::query(r#"INSERT INTO sales_quotas (id, quota_number, name, description, quota_type,
            owner_type, owner_id, territory_id, fiscal_year, period_type, currency, annual_target,
            q1_target, q2_target, q3_target, q4_target, m1_target, m2_target, m3_target, m4_target,
            m5_target, m6_target, m7_target, m8_target, m9_target, m10_target, m11_target, m12_target,
            product_id, product_category_id, status, approved_by, approved_at, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(q.base.id)
            .bind(&q.quota_number)
            .bind(&q.name)
            .bind(&q.description)
            .bind(&q.quota_type)
            .bind(&q.owner_type)
            .bind(q.owner_id)
            .bind(q.territory_id)
            .bind(q.fiscal_year)
            .bind(&q.period_type)
            .bind(&q.currency)
            .bind(q.annual_target)
            .bind(q.q1_target)
            .bind(q.q2_target)
            .bind(q.q3_target)
            .bind(q.q4_target)
            .bind(q.m1_target)
            .bind(q.m2_target)
            .bind(q.m3_target)
            .bind(q.m4_target)
            .bind(q.m5_target)
            .bind(q.m6_target)
            .bind(q.m7_target)
            .bind(q.m8_target)
            .bind(q.m9_target)
            .bind(q.m10_target)
            .bind(q.m11_target)
            .bind(q.m12_target)
            .bind(q.product_id)
            .bind(q.product_category_id)
            .bind(&q.status)
            .bind(q.approved_by)
            .bind(q.approved_at)
            .bind(q.created_at)
            .bind(q.updated_at)
            .execute(&self.pool).await?;
        Ok(q)
    }
    async fn get_quota(&self, _id: Uuid) -> Result<Option<SalesQuota>> { Ok(None) }
    async fn list_quotas(&self, _owner_id: Option<Uuid>) -> Result<Vec<SalesQuota>> { Ok(vec![]) }
    async fn create_attainment(&self, att: &QuotaAttainment) -> Result<QuotaAttainment> {
        let a = att.clone();
        sqlx::query(r#"INSERT INTO quota_attainments (id, quota_id, period_type, period_number,
            period_start, period_end, target, actual, attainment_percent, pipeline, pipeline_coverage,
            gap_to_quota, currency, calculated_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(a.base.id)
            .bind(a.quota_id)
            .bind(&a.period_type)
            .bind(a.period_number)
            .bind(a.period_start)
            .bind(a.period_end)
            .bind(a.target)
            .bind(a.actual)
            .bind(a.attainment_percent)
            .bind(a.pipeline)
            .bind(a.pipeline_coverage)
            .bind(a.gap_to_quota)
            .bind(&a.currency)
            .bind(a.calculated_at)
            .bind(a.created_at)
            .execute(&self.pool).await?;
        Ok(a)
    }
    async fn create_performance(&self, perf: &TerritoryPerformance) -> Result<TerritoryPerformance> {
        let p = perf.clone();
        sqlx::query(r#"INSERT INTO territory_performances (id, territory_id, period_type, period_start,
            period_end, quota, revenue, attainment_percent, new_accounts, total_accounts,
            opportunities_created, opportunities_won, win_rate_percent, avg_deal_size, pipeline_value,
            currency, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(p.base.id)
            .bind(p.territory_id)
            .bind(&p.period_type)
            .bind(p.period_start)
            .bind(p.period_end)
            .bind(p.quota)
            .bind(p.revenue)
            .bind(p.attainment_percent)
            .bind(p.new_accounts)
            .bind(p.total_accounts)
            .bind(p.opportunities_created)
            .bind(p.opportunities_won)
            .bind(p.win_rate_percent)
            .bind(p.avg_deal_size)
            .bind(p.pipeline_value)
            .bind(&p.currency)
            .bind(p.created_at)
            .execute(&self.pool).await?;
        Ok(p)
    }
}
