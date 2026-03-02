use async_trait::async_trait;
use erp_core::error::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait GrantRepository: Send + Sync {
    async fn create(&self, grant: &Grant) -> Result<Grant>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Grant>>;
    async fn find_all(&self, page: i32, limit: i32) -> Result<Vec<Grant>>;
    async fn find_by_status(&self, status: GrantStatus) -> Result<Vec<Grant>>;
    async fn find_by_investigator(&self, investigator_id: Uuid) -> Result<Vec<Grant>>;
    async fn update(&self, grant: &Grant) -> Result<Grant>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait GrantBudgetRepository: Send + Sync {
    async fn create(&self, budget: &GrantBudget) -> Result<GrantBudget>;
    async fn find_by_grant(&self, grant_id: Uuid) -> Result<Vec<GrantBudget>>;
    async fn update(&self, budget: &GrantBudget) -> Result<GrantBudget>;
}

#[async_trait]
pub trait GrantTransactionRepository: Send + Sync {
    async fn create(&self, transaction: &GrantTransaction) -> Result<GrantTransaction>;
    async fn find_by_grant(&self, grant_id: Uuid) -> Result<Vec<GrantTransaction>>;
    async fn get_summary(&self, grant_id: Uuid) -> Result<GrantTransactionSummary>;
}

pub struct GrantTransactionSummary {
    pub grant_id: Uuid,
    pub total_budgeted: i64,
    pub total_expended: i64,
    pub total_encumbered: i64,
    pub available_balance: i64,
}

#[async_trait]
pub trait GrantMilestoneRepository: Send + Sync {
    async fn create(&self, milestone: &GrantMilestone) -> Result<GrantMilestone>;
    async fn find_by_grant(&self, grant_id: Uuid) -> Result<Vec<GrantMilestone>>;
    async fn complete(&self, id: Uuid, completed_by: Uuid) -> Result<GrantMilestone>;
    async fn find_overdue(&self) -> Result<Vec<GrantMilestone>>;
}

#[async_trait]
pub trait GrantReportRepository: Send + Sync {
    async fn create(&self, report: &GrantReport) -> Result<GrantReport>;
    async fn find_by_grant(&self, grant_id: Uuid) -> Result<Vec<GrantReport>>;
    async fn submit(&self, id: Uuid) -> Result<GrantReport>;
    async fn find_due_soon(&self, days: i32) -> Result<Vec<GrantReport>>;
}

#[allow(dead_code)]
pub struct SqliteGrantRepository {
    pool: SqlitePool,
}

impl SqliteGrantRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GrantRepository for SqliteGrantRepository {
    async fn create(&self, grant: &Grant) -> Result<Grant> {
        Ok(grant.clone())
    }

    async fn find_by_id(&self, _id: Uuid) -> Result<Option<Grant>> {
        Ok(None)
    }

    async fn find_all(&self, _page: i32, _limit: i32) -> Result<Vec<Grant>> {
        Ok(Vec::new())
    }

    async fn find_by_status(&self, _status: GrantStatus) -> Result<Vec<Grant>> {
        Ok(Vec::new())
    }

    async fn find_by_investigator(&self, _investigator_id: Uuid) -> Result<Vec<Grant>> {
        Ok(Vec::new())
    }

    async fn update(&self, grant: &Grant) -> Result<Grant> {
        Ok(grant.clone())
    }

    async fn delete(&self, _id: Uuid) -> Result<()> {
        Ok(())
    }
}
