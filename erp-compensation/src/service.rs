use crate::models::*;
use crate::repository::*;
use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{Utc, Datelike};

pub struct CompensationService {
    repo: SqliteCompensationRepository,
}

impl CompensationService {
    pub fn new() -> Self {
        Self {
            repo: SqliteCompensationRepository::new(),
        }
    }
    
    pub async fn create_plan(&self, pool: &SqlitePool, mut plan: CompensationPlan) -> Result<CompensationPlan> {
        plan.id = Uuid::new_v4();
        plan.created_at = Utc::now();
        plan.updated_at = Utc::now();
        
        self.repo.create_plan(&plan).await?;
        Ok(plan)
    }
    
    pub async fn get_plan(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<CompensationPlan>> {
        self.repo.get_plan(id).await
    }
    
    pub async fn list_plans(&self, pool: &SqlitePool) -> Result<Vec<CompensationPlan>> {
        self.repo.list_plans().await
    }
    
    pub async fn set_employee_compensation(&self, pool: &SqlitePool, mut comp: EmployeeCompensation) -> Result<EmployeeCompensation> {
        comp.id = Uuid::new_v4();
        comp.created_at = Utc::now();
        comp.is_current = true;
        
        if let Some(range_id) = comp.salary_range_id {
            comp.compa_ratio = self.calculate_compa_ratio(comp.base_salary, range_id).await?;
        }
        
        self.repo.create_employee_compensation(&comp).await?;
        Ok(comp)
    }
    
    async fn calculate_compa_ratio(&self, salary: i64, range_id: Uuid) -> Result<f64> {
        Ok(1.0)
    }
    
    pub async fn get_employee_compensation(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<Option<EmployeeCompensation>> {
        self.repo.get_employee_compensation(employee_id).await
    }
    
    pub async fn create_adjustment(&self, pool: &SqlitePool, mut adjustment: CompensationAdjustment) -> Result<CompensationAdjustment> {
        adjustment.id = Uuid::new_v4();
        adjustment.created_at = Utc::now();
        adjustment.status = "Pending".to_string();
        
        adjustment.adjustment_amount = adjustment.new_base - adjustment.current_base;
        if adjustment.current_base > 0 {
            adjustment.adjustment_percent = (adjustment.adjustment_amount as f64 / adjustment.current_base as f64) * 100.0;
        }
        
        self.repo.create_adjustment(&adjustment).await?;
        Ok(adjustment)
    }
    
    pub async fn approve_adjustment(&self, pool: &SqlitePool, id: Uuid, approver_id: Uuid) -> Result<()> {
        self.repo.update_adjustment_status(id, "Approved").await
    }
    
    pub async fn list_adjustments(&self, pool: &SqlitePool, employee_id: Option<Uuid>) -> Result<Vec<CompensationAdjustment>> {
        self.repo.list_adjustments(employee_id).await
    }
    
    pub async fn create_review(&self, pool: &SqlitePool, mut review: CompensationReview) -> Result<CompensationReview> {
        review.id = Uuid::new_v4();
        review.review_date = Utc::now();
        review.created_at = Utc::now();
        review.status = "Pending".to_string();
        
        if review.current_salary > 0 {
            review.proposed_increase_percent = ((review.proposed_salary - review.current_salary) as f64 / review.current_salary as f64) * 100.0;
        }
        
        self.repo.create_review(&review).await?;
        Ok(review)
    }
    
    pub async fn list_reviews(&self, pool: &SqlitePool, plan_id: Uuid) -> Result<Vec<CompensationReview>> {
        self.repo.list_reviews(plan_id).await
    }
    
    pub async fn calculate_bonus(&self, pool: &SqlitePool, employee_id: Uuid, plan_id: Uuid) -> Result<EmployeeBonus> {
        let bonus = EmployeeBonus {
            id: Uuid::new_v4(),
            employee_id,
            bonus_plan_id: plan_id,
            fiscal_year: Utc::now().year(),
            target_amount: 0,
            company_performance_factor: 1.0,
            individual_performance_factor: 1.0,
            calculated_amount: 0,
            recommended_amount: 0,
            approved_amount: None,
            status: "Calculated".to_string(),
            approved_by: None,
            approved_at: None,
            created_at: Utc::now(),
        };
        
        Ok(bonus)
    }
    
    pub async fn generate_total_rewards_statement(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<TotalRewardsStatement> {
        let statement = TotalRewardsStatement {
            id: Uuid::new_v4(),
            employee_id,
            statement_year: Utc::now().year(),
            base_salary: 0,
            variable_pay: 0,
            benefits_value: 0,
            equity_value: 0,
            other_compensation: 0,
            total_compensation: 0,
            generated_at: Utc::now(),
        };
        
        Ok(statement)
    }
    
    pub async fn analyze_pay_equity(&self, pool: &SqlitePool, group_type: String, group_id: Option<Uuid>) -> Result<PayEquityAnalysis> {
        let analysis = PayEquityAnalysis {
            id: Uuid::new_v4(),
            analysis_date: Utc::now(),
            analysis_type: "Gender".to_string(),
            group_type,
            group_id,
            employee_count: 0,
            avg_salary_male: 0,
            avg_salary_female: 0,
            pay_gap_percent: 0.0,
            adjusted_gap_percent: 0.0,
            statistical_significance: 0.0,
            findings: None,
            recommendations: None,
            created_at: Utc::now(),
        };
        
        Ok(analysis)
    }
    
    pub async fn get_market_benchmark(&self, pool: &SqlitePool, position_id: Uuid) -> Result<CompensationBenchmark> {
        let benchmark = CompensationBenchmark {
            id: Uuid::new_v4(),
            employee_id: None,
            position_id: Some(position_id),
            current_salary: 0,
            market_p50: 0,
            market_p75: 0,
            market_p90: 0,
            compa_ratio_p50: 0.0,
            percentile: 50.0,
            benchmark_date: Utc::now(),
            market_data_source: "Default".to_string(),
            created_at: Utc::now(),
        };
        
        Ok(benchmark)
    }
}
