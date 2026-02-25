use async_trait::async_trait;
use erp_core::error::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait BudgetRepository: Send + Sync {
    async fn create(&self, budget: &Budget) -> Result<Budget>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Budget>>;
    async fn find_all(&self, page: i32, limit: i32) -> Result<Vec<Budget>>;
    async fn find_by_department(&self, department_id: Uuid) -> Result<Vec<Budget>>;
    async fn find_by_fiscal_year(&self, year: i32) -> Result<Vec<Budget>>;
    async fn update(&self, budget: &Budget) -> Result<Budget>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait BudgetLineRepository: Send + Sync {
    async fn create(&self, line: &BudgetLine) -> Result<BudgetLine>;
    async fn find_by_budget(&self, budget_id: Uuid) -> Result<Vec<BudgetLine>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<BudgetLine>>;
    async fn update(&self, line: &BudgetLine) -> Result<BudgetLine>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn bulk_create(&self, lines: &[BudgetLine]) -> Result<Vec<BudgetLine>>;
}

#[async_trait]
pub trait BudgetPeriodRepository: Send + Sync {
    async fn create(&self, period: &BudgetPeriod) -> Result<BudgetPeriod>;
    async fn find_by_budget(&self, budget_id: Uuid) -> Result<Vec<BudgetPeriod>>;
    async fn update(&self, period: &BudgetPeriod) -> Result<BudgetPeriod>;
}

#[async_trait]
pub trait ForecastRepository: Send + Sync {
    async fn create(&self, forecast: &Forecast) -> Result<Forecast>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Forecast>>;
    async fn find_active(&self) -> Result<Vec<Forecast>>;
    async fn update(&self, forecast: &Forecast) -> Result<Forecast>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait ForecastLineRepository: Send + Sync {
    async fn create(&self, line: &ForecastLine) -> Result<ForecastLine>;
    async fn find_by_forecast(&self, forecast_id: Uuid) -> Result<Vec<ForecastLine>>;
    async fn update_actual(&self, id: Uuid, actual: i64) -> Result<ForecastLine>;
}

#[async_trait]
pub trait BudgetTransferRepository: Send + Sync {
    async fn create(&self, transfer: &BudgetTransfer) -> Result<BudgetTransfer>;
    async fn find_pending(&self) -> Result<Vec<BudgetTransfer>>;
    async fn approve(&self, id: Uuid, approver_id: Uuid) -> Result<BudgetTransfer>;
}

#[async_trait]
pub trait BudgetScenarioRepository: Send + Sync {
    async fn create(&self, scenario: &BudgetScenario) -> Result<BudgetScenario>;
    async fn find_by_budget(&self, budget_id: Uuid) -> Result<Vec<BudgetScenario>>;
    async fn set_baseline(&self, id: Uuid) -> Result<BudgetScenario>;
}

#[async_trait]
pub trait BudgetAlertRepository: Send + Sync {
    async fn create(&self, alert: &BudgetAlert) -> Result<BudgetAlert>;
    async fn find_active(&self, budget_id: Uuid) -> Result<Vec<BudgetAlert>>;
    async fn trigger(&self, id: Uuid) -> Result<BudgetAlert>;
}

#[async_trait]
pub trait BudgetApprovalRepository: Send + Sync {
    async fn create(&self, approval: &BudgetApproval) -> Result<BudgetApproval>;
    async fn find_by_budget(&self, budget_id: Uuid) -> Result<Vec<BudgetApproval>>;
    async fn approve(&self, id: Uuid, comments: Option<String>) -> Result<BudgetApproval>;
    async fn reject(&self, id: Uuid, comments: String) -> Result<BudgetApproval>;
}

#[async_trait]
pub trait BudgetActualRepository: Send + Sync {
    async fn create(&self, actual: &BudgetActual) -> Result<BudgetActual>;
    async fn find_by_line(&self, line_id: Uuid) -> Result<Vec<BudgetActual>>;
    async fn get_summary(&self, budget_id: Uuid) -> Result<BudgetActualSummary>;
}

pub struct BudgetActualSummary {
    pub budget_id: Uuid,
    pub total_planned: i64,
    pub total_committed: i64,
    pub total_actual: i64,
    pub total_variance: i64,
    pub utilization_percent: f64,
}

pub struct SqliteBudgetRepository {
    pool: SqlitePool,
}

impl SqliteBudgetRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BudgetRepository for SqliteBudgetRepository {
    async fn create(&self, budget: &Budget) -> Result<Budget> {
        Ok(budget.clone())
    }

    async fn find_by_id(&self, _id: Uuid) -> Result<Option<Budget>> {
        Ok(None)
    }

    async fn find_all(&self, _page: i32, _limit: i32) -> Result<Vec<Budget>> {
        Ok(Vec::new())
    }

    async fn find_by_department(&self, _department_id: Uuid) -> Result<Vec<Budget>> {
        Ok(Vec::new())
    }

    async fn find_by_fiscal_year(&self, _year: i32) -> Result<Vec<Budget>> {
        Ok(Vec::new())
    }

    async fn update(&self, budget: &Budget) -> Result<Budget> {
        Ok(budget.clone())
    }

    async fn delete(&self, _id: Uuid) -> Result<()> {
        Ok(())
    }
}
