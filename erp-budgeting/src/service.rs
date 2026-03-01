use chrono::{DateTime, Utc};
use erp_core::error::{Error, Result};
use erp_core::models::{BaseEntity, Money, Currency, Status};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct BudgetService {
    budget_repo: SqliteBudgetRepository,
}

impl BudgetService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            budget_repo: SqliteBudgetRepository::new(pool),
        }
    }

    pub async fn create_budget(
        &self,
        _pool: &SqlitePool,
        name: String,
        code: String,
        budget_type: BudgetType,
        fiscal_year: i32,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        total_amount: i64,
        currency: Currency,
        owner_id: Uuid,
        department_id: Option<Uuid>,
        project_id: Option<Uuid>,
    ) -> Result<Budget> {
        if start_date >= end_date {
            return Err(Error::validation("Start date must be before end date".to_string()));
        }
        if !(2000..=2100).contains(&fiscal_year) {
            return Err(Error::validation("Invalid fiscal year".to_string()));
        }
        let budget = Budget {
            base: BaseEntity::new(),
            name,
            code,
            description: None,
            budget_type,
            status: BudgetStatus::Draft,
            fiscal_year,
            start_date,
            end_date,
            total_amount: Money::new(total_amount, currency.clone()),
            currency: currency.to_string(),
            department_id,
            project_id,
            owner_id,
            approval_workflow_id: None,
            version: 1,
            parent_budget_id: None,
            is_template: false,
        };
        self.budget_repo.create(&budget).await
    }

    pub async fn get_budget(&self, _pool: &SqlitePool, id: Uuid) -> Result<Option<Budget>> {
        self.budget_repo.find_by_id(id).await
    }

    pub async fn list_budgets(&self, _pool: &SqlitePool, page: i32, limit: i32) -> Result<Vec<Budget>> {
        self.budget_repo.find_all(page, limit).await
    }

    pub async fn update_budget(&self, _pool: &SqlitePool, budget: Budget) -> Result<Budget> {
        self.budget_repo.update(&budget).await
    }

    pub async fn submit_for_approval(&self, _pool: &SqlitePool, id: Uuid) -> Result<Budget> {
        let mut budget = self.budget_repo.find_by_id(id).await?
            .ok_or(Error::not_found("budget", &id.to_string()))?;
        if budget.status != BudgetStatus::Draft {
            return Err(Error::validation("Only draft budgets can be submitted".to_string()));
        }
        budget.status = BudgetStatus::Submitted;
        self.budget_repo.update(&budget).await
    }

    pub async fn approve_budget(&self, _pool: &SqlitePool, id: Uuid) -> Result<Budget> {
        let mut budget = self.budget_repo.find_by_id(id).await?
            .ok_or(Error::not_found("budget", &id.to_string()))?;
        if budget.status != BudgetStatus::Submitted && budget.status != BudgetStatus::UnderReview {
            return Err(Error::validation("Budget must be submitted first".to_string()));
        }
        budget.status = BudgetStatus::Approved;
        self.budget_repo.update(&budget).await
    }

    pub async fn activate_budget(&self, _pool: &SqlitePool, id: Uuid) -> Result<Budget> {
        let mut budget = self.budget_repo.find_by_id(id).await?
            .ok_or(Error::not_found("budget", &id.to_string()))?;
        if budget.status != BudgetStatus::Approved {
            return Err(Error::validation("Only approved budgets can be activated".to_string()));
        }
        budget.status = BudgetStatus::Active;
        self.budget_repo.update(&budget).await
    }

    pub async fn create_budget_line(
        &self,
        _pool: &SqlitePool,
        budget_id: Uuid,
        account_id: Uuid,
        account_code: String,
        account_name: String,
        planned_amount: i64,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<BudgetLine> {
        let line = BudgetLine {
            base: BaseEntity::new(),
            budget_id,
            account_id,
            account_code,
            account_name,
            description: None,
            planned_amount: Money::new(planned_amount, Currency::USD),
            committed_amount: Money::new(0, Currency::USD),
            actual_amount: Money::new(0, Currency::USD),
            variance_amount: Money::new(planned_amount, Currency::USD),
            variance_percent: 100.0,
            period_start,
            period_end,
            cost_center_id: None,
            notes: None,
        };
        Ok(line)
    }

    pub async fn calculate_variance(&self, planned: i64, actual: i64) -> (i64, f64) {
        let variance = planned - actual;
        let percent = if planned != 0 {
            (variance as f64 / planned as f64) * 100.0
        } else {
            0.0
        };
        (variance, percent)
    }
}

pub struct ForecastService {
    pool: SqlitePool,
}

impl ForecastService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_forecast(
        &self,
        name: String,
        forecast_type: BudgetType,
        method: ForecastMethod,
        fiscal_year: i32,
        start_date: DateTime<Utc>,
        end_date: DateTime<Utc>,
        created_by: Uuid,
    ) -> Result<Forecast> {
        let forecast = Forecast {
            base: BaseEntity::new(),
            name,
            forecast_type,
            method,
            fiscal_year,
            start_date,
            end_date,
            total_forecast: Money::new(0, Currency::USD),
            confidence_level: 0.0,
            created_by,
            is_active: true,
        };
        Ok(forecast)
    }

    pub async fn generate_forecast(
        &self,
        historical_data: &[i64],
        method: ForecastMethod,
        periods_ahead: i32,
    ) -> Result<Vec<i64>> {
        if historical_data.is_empty() {
            return Err(Error::validation("No historical data provided".to_string()));
        }
        let forecasts = match method {
            ForecastMethod::Linear => self.linear_forecast(historical_data, periods_ahead)?,
            ForecastMethod::MovingAverage => self.moving_average_forecast(historical_data, periods_ahead)?,
            ForecastMethod::Exponential => self.exponential_forecast(historical_data, periods_ahead)?,
            _ => historical_data.to_vec(),
        };
        Ok(forecasts)
    }

    fn linear_forecast(&self, data: &[i64], periods: i32) -> Result<Vec<i64>> {
        let n = data.len() as f64;
        if n < 2.0 {
            return Ok(vec![data.last().copied().unwrap_or(0); periods as usize]);
        }
        let sum_x: f64 = (0..data.len()).map(|i| i as f64).sum();
        let sum_y: f64 = data.iter().map(|&v| v as f64).sum();
        let sum_xy: f64 = data.iter().enumerate()
            .map(|(i, &v)| i as f64 * v as f64).sum();
        let sum_x2: f64 = (0..data.len()).map(|i| (i as f64).powi(2)).sum();
        let slope = (n * sum_xy - sum_x * sum_y) / (n * sum_x2 - sum_x.powi(2));
        let intercept = (sum_y - slope * sum_x) / n;
        let mut result = Vec::new();
        for i in 0..periods {
            let x = data.len() as f64 + i as f64;
            let forecast = intercept + slope * x;
            result.push(forecast.round() as i64);
        }
        Ok(result)
    }

    fn moving_average_forecast(&self, data: &[i64], periods: i32) -> Result<Vec<i64>> {
        let window = 3.min(data.len());
        let last_avg = data.iter().rev().take(window).sum::<i64>() / window as i64;
        Ok(vec![last_avg; periods as usize])
    }

    fn exponential_forecast(&self, data: &[i64], periods: i32) -> Result<Vec<i64>> {
        let alpha = 0.3;
        let mut smoothed = data[0] as f64;
        for &value in &data[1..] {
            smoothed = alpha * value as f64 + (1.0 - alpha) * smoothed;
        }
        Ok(vec![smoothed.round() as i64; periods as usize])
    }

    pub fn calculate_accuracy(&self, forecasted: &[i64], actual: &[i64]) -> f64 {
        if forecasted.len() != actual.len() || forecasted.is_empty() {
            return 0.0;
        }
        let sum_abs_pct_error: f64 = forecasted.iter()
            .zip(actual.iter())
            .map(|(&f, &a)| {
                if a != 0 {
                    ((f - a).abs() as f64 / a.abs() as f64) * 100.0
                } else {
                    0.0
                }
            })
            .sum();
        100.0 - (sum_abs_pct_error / forecasted.len() as f64)
    }
}

pub struct BudgetAlertService {
    pool: SqlitePool,
}

impl BudgetAlertService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn check_budget_thresholds(&self, _budget_id: Uuid) -> Result<Vec<BudgetAlert>> {
        Ok(Vec::new())
    }

    pub async fn create_alert(
        &self,
        budget_id: Uuid,
        budget_line_id: Option<Uuid>,
        alert_type: AlertType,
        threshold_percent: f64,
        notify_users: Vec<Uuid>,
    ) -> Result<BudgetAlert> {
        let alert = BudgetAlert {
            base: BaseEntity::new(),
            budget_id,
            budget_line_id,
            alert_type,
            threshold_percent,
            is_active: true,
            last_triggered: None,
            notify_users,
        };
        Ok(alert)
    }
}

pub struct BudgetTransferService {
    pool: SqlitePool,
}

impl BudgetTransferService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn request_transfer(
        &self,
        from_line_id: Uuid,
        to_line_id: Uuid,
        amount: i64,
        reason: String,
        requested_by: Uuid,
    ) -> Result<BudgetTransfer> {
        let transfer = BudgetTransfer {
            base: BaseEntity::new(),
            from_budget_line_id: from_line_id,
            to_budget_line_id: to_line_id,
            amount: Money::new(amount, Currency::USD),
            reason,
            requested_by,
            approved_by: None,
            approved_at: None,
            status: Status::Pending,
        };
        Ok(transfer)
    }

    pub async fn approve_transfer(&self, _transfer_id: Uuid, approver_id: Uuid) -> Result<BudgetTransfer> {
        Ok(BudgetTransfer {
            base: BaseEntity::new(),
            from_budget_line_id: Uuid::nil(),
            to_budget_line_id: Uuid::nil(),
            amount: Money::new(0, Currency::USD),
            reason: String::new(),
            requested_by: Uuid::nil(),
            approved_by: Some(approver_id),
            approved_at: Some(Utc::now()),
            status: Status::Active,
        })
    }
}

pub struct BudgetScenarioService {
    pool: SqlitePool,
}

impl BudgetScenarioService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_scenario(
        &self,
        budget_id: Uuid,
        name: String,
        scenario_type: ScenarioType,
        adjustment_factor: f64,
        base_amount: i64,
    ) -> Result<BudgetScenario> {
        let adjusted = (base_amount as f64 * adjustment_factor) as i64;
        let scenario = BudgetScenario {
            base: BaseEntity::new(),
            budget_id,
            name,
            scenario_type,
            adjustment_factor,
            description: None,
            total_amount: Money::new(adjusted, Currency::USD),
            is_baseline: false,
        };
        Ok(scenario)
    }

    pub async fn compare_scenarios(&self, scenarios: &[BudgetScenario]) -> Vec<ScenarioComparison> {
        scenarios.iter().map(|s| ScenarioComparison {
            scenario_id: s.base.id,
            name: s.name.clone(),
            total_amount: s.total_amount.amount,
            variance_from_baseline: 0,
        }).collect()
    }
}

pub struct ScenarioComparison {
    pub scenario_id: Uuid,
    pub name: String,
    pub total_amount: i64,
    pub variance_from_baseline: i64,
}
