use crate::models::*;
use crate::repository::*;
use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

pub struct CommissionService {
    repo: SqliteCommissionRepository,
}

impl CommissionService {
    pub fn new() -> Self {
        Self {
            repo: SqliteCommissionRepository::new(),
        }
    }
    
    pub async fn create_plan(&self, pool: &SqlitePool, mut plan: CommissionPlan) -> Result<CommissionPlan> {
        plan.id = Uuid::new_v4();
        plan.created_at = Utc::now();
        plan.updated_at = Utc::now();
        
        self.repo.create_plan(&plan).await?;
        Ok(plan)
    }
    
    pub async fn get_plan(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<CommissionPlan>> {
        self.repo.get_plan(id).await
    }
    
    pub async fn list_plans(&self, pool: &SqlitePool) -> Result<Vec<CommissionPlan>> {
        self.repo.list_plans().await
    }
    
    pub async fn calculate_commission(&self, pool: &SqlitePool, sales_rep_id: Uuid, plan_id: Uuid, period_start: chrono::DateTime<Utc>, period_end: chrono::DateTime<Utc>) -> Result<CommissionCalculation> {
        let mut calc = CommissionCalculation {
            id: Uuid::new_v4(),
            calculation_number: format!("CALC-{}", Utc::now().format("%Y%m%d%H%M%S")),
            sales_rep_id,
            plan_id,
            period_start,
            period_end,
            gross_revenue: 0,
            returns: 0,
            net_revenue: 0,
            cost_of_goods: 0,
            gross_margin: 0,
            base_commission: 0,
            tier_bonus: 0,
            override_commission: 0,
            adjustments: 0,
            clawbacks: 0,
            total_commission: 0,
            status: CalculationStatus::Calculating,
            calculated_at: Some(Utc::now()),
            approved_at: None,
            approved_by: None,
            paid_at: None,
            created_at: Utc::now(),
        };
        
        calc.net_revenue = calc.gross_revenue - calc.returns;
        calc.gross_margin = calc.net_revenue - calc.cost_of_goods;
        
        let plan = self.repo.get_plan(plan_id).await?;
        if let Some(plan) = plan {
            calc.base_commission = (calc.net_revenue as f64 * plan.default_rate / 100.0) as i64;
        }
        
        calc.total_commission = calc.base_commission + calc.tier_bonus + calc.override_commission - calc.clawbacks + calc.adjustments;
        calc.status = CalculationStatus::Completed;
        
        self.repo.create_calculation(&calc).await?;
        Ok(calc)
    }
    
    pub async fn get_calculation(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<CommissionCalculation>> {
        self.repo.get_calculation(id).await
    }
    
    pub async fn list_calculations(&self, pool: &SqlitePool, sales_rep_id: Option<Uuid>) -> Result<Vec<CommissionCalculation>> {
        self.repo.list_calculations(sales_rep_id).await
    }
    
    pub async fn approve_calculation(&self, pool: &SqlitePool, id: Uuid, approver_id: Uuid) -> Result<()> {
        self.repo.update_calculation_status(id, CalculationStatus::Approved).await
    }
    
    pub async fn create_quota(&self, pool: &SqlitePool, mut quota: SalesQuota) -> Result<SalesQuota> {
        quota.id = Uuid::new_v4();
        quota.created_at = Utc::now();
        
        self.repo.create_quota(&quota).await?;
        Ok(quota)
    }
    
    pub async fn get_quota_progress(&self, pool: &SqlitePool, quota_id: Uuid) -> Result<QuotaProgress> {
        let progress = QuotaProgress {
            id: Uuid::new_v4(),
            quota_id,
            as_of_date: Utc::now(),
            achieved_amount: 0,
            percent_achieved: 0.0,
            forecast_amount: 0,
            gap_to_quota: 0,
            updated_at: Utc::now(),
        };
        
        Ok(progress)
    }
    
    pub async fn create_adjustment(&self, pool: &SqlitePool, mut adjustment: CommissionAdjustment) -> Result<CommissionAdjustment> {
        adjustment.id = Uuid::new_v4();
        adjustment.created_at = Utc::now();
        
        Ok(adjustment)
    }
}
