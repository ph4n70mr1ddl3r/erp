use chrono::{DateTime, Utc};
use erp_core::error::{Error, Result};
use erp_core::models::{BaseEntity, Money, Currency, Status};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct CostPoolService {
    pool_repo: SqliteCostPoolRepository,
}

impl CostPoolService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool_repo: SqliteCostPoolRepository::new(pool),
        }
    }

    pub async fn create_cost_pool(
        &self,
        _pool: &SqlitePool,
        name: String,
        code: String,
        pool_type: CostPoolType,
        total_cost: i64,
        fiscal_year: i32,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
    ) -> Result<CostPool> {
        if period_start >= period_end {
            return Err(Error::validation("Period start must be before end".to_string()));
        }
        let cost_pool = CostPool {
            base: BaseEntity::new(),
            name,
            code,
            description: None,
            pool_type,
            total_cost: Money::new(total_cost, Currency::USD),
            currency: "USD".to_string(),
            fiscal_year,
            period_start,
            period_end,
            is_active: true,
        };
        self.pool_repo.create(&cost_pool).await
    }

    pub async fn get_cost_pool(&self, _pool: &SqlitePool, id: Uuid) -> Result<Option<CostPool>> {
        self.pool_repo.find_by_id(id).await
    }

    pub async fn calculate_pool_total(&self, _pool: &SqlitePool, _id: Uuid) -> Result<i64> {
        Ok(0)
    }
}

pub struct ActivityService {
    pool: SqlitePool,
}

impl ActivityService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_activity(
        &self,
        name: String,
        code: String,
        activity_type: ActivityType,
        cost_pool_id: Uuid,
        total_cost: i64,
        cost_driver_id: Option<Uuid>,
        driver_quantity: f64,
    ) -> Result<Activity> {
        if driver_quantity <= 0.0 {
            return Err(Error::validation("Driver quantity must be positive".to_string()));
        }
        let cost_per_driver = if driver_quantity > 0.0 {
            Money::new((total_cost as f64 / driver_quantity) as i64, Currency::USD)
        } else {
            Money::new(0, Currency::USD)
        };
        let activity = Activity {
            base: BaseEntity::new(),
            name,
            code,
            description: None,
            activity_type,
            cost_pool_id,
            total_cost: Money::new(total_cost, Currency::USD),
            cost_driver_id,
            driver_quantity,
            cost_per_driver,
            department_id: None,
            process_id: None,
            is_value_added: true,
        };
        Ok(activity)
    }

    pub async fn calculate_activity_cost(
        &self,
        total_cost: i64,
        driver_quantity: f64,
    ) -> Result<Money> {
        if driver_quantity <= 0.0 {
            return Err(Error::validation("Driver quantity must be positive".to_string()));
        }
        Ok(Money::new((total_cost as f64 / driver_quantity) as i64, Currency::USD))
    }

    pub async fn classify_activities(&self, activities: &[Activity]) -> ActivityClassification {
        let mut classification = ActivityClassification {
            unit_level: Vec::new(),
            batch_level: Vec::new(),
            product_level: Vec::new(),
            customer_level: Vec::new(),
            facility_level: Vec::new(),
        };
        for activity in activities {
            match activity.activity_type {
                ActivityType::UnitLevel => classification.unit_level.push(activity.base.id),
                ActivityType::BatchLevel => classification.batch_level.push(activity.base.id),
                ActivityType::ProductLevel => classification.product_level.push(activity.base.id),
                ActivityType::CustomerLevel => classification.customer_level.push(activity.base.id),
                ActivityType::FacilityLevel => classification.facility_level.push(activity.base.id),
                ActivityType::OrganizationLevel => classification.facility_level.push(activity.base.id),
            }
        }
        classification
    }
}

pub struct ActivityClassification {
    pub unit_level: Vec<Uuid>,
    pub batch_level: Vec<Uuid>,
    pub product_level: Vec<Uuid>,
    pub customer_level: Vec<Uuid>,
    pub facility_level: Vec<Uuid>,
}

pub struct CostDriverService {
    pool: SqlitePool,
}

impl CostDriverService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_cost_driver(
        &self,
        name: String,
        code: String,
        driver_type: CostDriverType,
        unit_of_measure: String,
        total_capacity: f64,
    ) -> Result<CostDriver> {
        let driver = CostDriver {
            base: BaseEntity::new(),
            name,
            code,
            description: None,
            driver_type,
            unit_of_measure,
            total_capacity,
            used_capacity: 0.0,
            unused_capacity: total_capacity,
            utilization_percent: 0.0,
            is_active: true,
        };
        Ok(driver)
    }

    pub async fn record_usage(&self, _driver_id: Uuid, quantity: f64) -> Result<CostDriver> {
        Ok(CostDriver {
            base: BaseEntity::new(),
            name: String::new(),
            code: String::new(),
            description: None,
            driver_type: CostDriverType::Transaction,
            unit_of_measure: String::new(),
            total_capacity: 100.0,
            used_capacity: quantity,
            unused_capacity: 100.0 - quantity,
            utilization_percent: quantity,
            is_active: true,
        })
    }

    pub async fn calculate_utilization(&self, used: f64, total: f64) -> f64 {
        if total > 0.0 {
            (used / total) * 100.0
        } else {
            0.0
        }
    }
}

pub struct CostObjectService {
    pool: SqlitePool,
}

impl CostObjectService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_cost_object(
        &self,
        name: String,
        code: String,
        object_type: CostObjectType,
        parent_id: Option<Uuid>,
    ) -> Result<CostObject> {
        let object = CostObject {
            base: BaseEntity::new(),
            name,
            code,
            description: None,
            object_type,
            parent_id,
            direct_cost: Money::new(0, Currency::USD),
            indirect_cost: Money::new(0, Currency::USD),
            total_cost: Money::new(0, Currency::USD),
            revenue: Money::new(0, Currency::USD),
            profit_margin: Money::new(0, Currency::USD),
            profit_margin_percent: 0.0,
            is_active: true,
        };
        Ok(object)
    }

    pub async fn calculate_total_cost(&self, direct: i64, indirect: i64) -> Money {
        Money::new(direct + indirect, Currency::USD)
    }

    pub async fn calculate_profit_margin(&self, revenue: i64, cost: i64) -> (Money, f64) {
        let margin = revenue - cost;
        let percent = if revenue > 0 {
            (margin as f64 / revenue as f64) * 100.0
        } else {
            0.0
        };
        (Money::new(margin, Currency::USD), percent)
    }
}

pub struct AllocationService {
    pool: SqlitePool,
}

impl AllocationService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn allocate_activity_cost(
        &self,
        activity_id: Uuid,
        cost_object_id: Uuid,
        cost_pool_id: Uuid,
        driver_quantity: f64,
        rate_per_unit: i64,
    ) -> Result<ActivityAllocation> {
        let allocated = (driver_quantity * rate_per_unit as f64) as i64;
        let allocation = ActivityAllocation {
            base: BaseEntity::new(),
            activity_id,
            cost_object_id,
            cost_pool_id,
            driver_quantity,
            allocation_rate: Money::new(rate_per_unit, Currency::USD),
            allocated_amount: Money::new(allocated, Currency::USD),
            allocation_date: Utc::now(),
            fiscal_period: format!("{}", chrono::Utc::now().format("%Y-%m")),
            notes: None,
        };
        Ok(allocation)
    }

    pub async fn two_stage_allocation(
        &self,
        resources: Vec<ResourceAllocation>,
        _activities: Vec<ActivityAllocation>,
        _cost_objects: Vec<Uuid>,
    ) -> Result<AllocationResult> {
        let mut total_allocated = 0i64;
        for resource in &resources {
            total_allocated += resource.amount.amount;
        }
        Ok(AllocationResult {
            total_allocated: Money::new(total_allocated, Currency::USD),
            allocations_by_object: std::collections::HashMap::new(),
        })
    }
}

pub struct ResourceAllocation {
    pub resource_id: Uuid,
    pub activity_id: Uuid,
    pub amount: Money,
}

pub struct AllocationResult {
    pub total_allocated: Money,
    pub allocations_by_object: std::collections::HashMap<Uuid, Money>,
}

pub struct ProcessService {
    pool: SqlitePool,
}

impl ProcessService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_process(
        &self,
        name: String,
        code: String,
        owner_id: Option<Uuid>,
        department_id: Option<Uuid>,
        is_core_process: bool,
    ) -> Result<Process> {
        let process = Process {
            base: BaseEntity::new(),
            name,
            code,
            description: None,
            owner_id,
            department_id,
            total_activities: 0,
            total_cost: Money::new(0, Currency::USD),
            cycle_time_hours: 0.0,
            is_core_process,
            status: Status::Active,
        };
        Ok(process)
    }

    pub async fn add_step(
        &self,
        process_id: Uuid,
        step_number: i32,
        name: String,
        activity_id: Uuid,
        estimated_duration_hours: f64,
        cost: i64,
    ) -> Result<ProcessStep> {
        let step = ProcessStep {
            base: BaseEntity::new(),
            process_id,
            step_number,
            name,
            description: None,
            activity_id,
            estimated_duration_hours,
            actual_duration_hours: None,
            cost: Money::new(cost, Currency::USD),
            is_bottleneck: false,
            next_step_id: None,
        };
        Ok(step)
    }

    pub async fn identify_bottlenecks(&self, steps: &[ProcessStep]) -> Vec<Uuid> {
        if steps.is_empty() {
            return Vec::new();
        }
        let avg_duration: f64 = steps.iter()
            .map(|s| s.estimated_duration_hours)
            .sum::<f64>() / steps.len() as f64;
        steps.iter()
            .filter(|s| s.estimated_duration_hours > avg_duration * 1.5)
            .map(|s| s.base.id)
            .collect()
    }

    pub async fn calculate_cycle_time(&self, steps: &[ProcessStep]) -> f64 {
        steps.iter().map(|s| s.estimated_duration_hours).sum::<f64>()
    }
}

pub struct CostSimulationService {
    pool: SqlitePool,
}

impl CostSimulationService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_simulation(
        &self,
        name: String,
        simulation_type: SimulationType,
        created_by: Uuid,
    ) -> Result<CostSimulation> {
        let simulation = CostSimulation {
            base: BaseEntity::new(),
            name,
            description: None,
            simulation_type,
            base_scenario_id: None,
            status: SimulationStatus::Draft,
            created_by,
            results: serde_json::json!({}),
            variance_analysis: None,
        };
        Ok(simulation)
    }

    pub async fn run_what_if(
        &self,
        base_costs: i64,
        cost_change_percent: f64,
        volume_change_percent: f64,
    ) -> Result<WhatIfResult> {
        let new_cost = (base_costs as f64 * (1.0 + cost_change_percent / 100.0)) as i64;
        let adjusted_volume = 1.0 + volume_change_percent / 100.0;
        let cost_per_unit = if adjusted_volume > 0.0 {
            new_cost as f64 / adjusted_volume
        } else {
            new_cost as f64
        };
        Ok(WhatIfResult {
            original_cost: base_costs,
            new_cost,
            cost_change: new_cost - base_costs,
            cost_per_unit: cost_per_unit as i64,
        })
    }

    pub async fn sensitivity_analysis(
        &self,
        base_value: i64,
        variables: &[(String, f64, f64, f64)],
    ) -> Result<SensitivityResult> {
        let mut impacts = Vec::new();
        for (name, current, min, max) in variables {
            let low_impact = (base_value as f64 * min / current).round() as i64 - base_value;
            let high_impact = (base_value as f64 * max / current).round() as i64 - base_value;
            impacts.push(VariableImpact {
                name: name.clone(),
                low_impact,
                high_impact,
                sensitivity_score: (high_impact.abs() + low_impact.abs()) as f64 / 2.0,
            });
        }
        impacts.sort_by(|a, b| b.sensitivity_score.partial_cmp(&a.sensitivity_score).unwrap());
        Ok(SensitivityResult { impacts })
    }
}

pub struct WhatIfResult {
    pub original_cost: i64,
    pub new_cost: i64,
    pub cost_change: i64,
    pub cost_per_unit: i64,
}

pub struct SensitivityResult {
    pub impacts: Vec<VariableImpact>,
}

pub struct VariableImpact {
    pub name: String,
    pub low_impact: i64,
    pub high_impact: i64,
    pub sensitivity_score: f64,
}

pub struct CostAnalysisService {
    pool: SqlitePool,
}

impl CostAnalysisService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_analysis(
        &self,
        name: String,
        analysis_type: AnalysisType,
        period_start: DateTime<Utc>,
        period_end: DateTime<Utc>,
        created_by: Uuid,
    ) -> Result<CostAnalysis> {
        let analysis = CostAnalysis {
            base: BaseEntity::new(),
            name,
            analysis_type,
            period_start,
            period_end,
            total_direct_costs: Money::new(0, Currency::USD),
            total_indirect_costs: Money::new(0, Currency::USD),
            total_costs: Money::new(0, Currency::USD),
            cost_breakdown: serde_json::json!({}),
            insights: Vec::new(),
            created_by,
        };
        Ok(analysis)
    }

    pub async fn analyze_profitability(
        &self,
        revenue: i64,
        direct_cost: i64,
        indirect_cost: i64,
    ) -> ProfitabilityAnalysis {
        let total_cost = direct_cost + indirect_cost;
        let gross_profit = revenue - direct_cost;
        let net_profit = revenue - total_cost;
        let gross_margin = if revenue > 0 {
            (gross_profit as f64 / revenue as f64) * 100.0
        } else {
            0.0
        };
        let net_margin = if revenue > 0 {
            (net_profit as f64 / revenue as f64) * 100.0
        } else {
            0.0
        };
        ProfitabilityAnalysis {
            revenue,
            direct_cost,
            indirect_cost,
            total_cost,
            gross_profit,
            net_profit,
            gross_margin_percent: gross_margin,
            net_margin_percent: net_margin,
        }
    }

    pub async fn generate_insights(&self, analysis_data: &serde_json::Value) -> Vec<CostInsight> {
        let mut insights = Vec::new();
        if let Some(high_cost_activities) = analysis_data.get("high_cost_activities").and_then(|v| v.as_array()) {
            if !high_cost_activities.is_empty() {
                insights.push(CostInsight {
                    insight_type: "HighCost".to_string(),
                    description: format!("{} activities account for significant cost", high_cost_activities.len()),
                    impact_amount: Money::new(0, Currency::USD),
                    recommendation: "Consider process optimization or automation".to_string(),
                    priority: InsightPriority::High,
                });
            }
        }
        insights
    }
}

pub struct ProfitabilityAnalysis {
    pub revenue: i64,
    pub direct_cost: i64,
    pub indirect_cost: i64,
    pub total_cost: i64,
    pub gross_profit: i64,
    pub net_profit: i64,
    pub gross_margin_percent: f64,
    pub net_margin_percent: f64,
}

pub struct BillOfActivitiesService {
    pool: SqlitePool,
}

impl BillOfActivitiesService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_bill_of_activities(
        &self,
        cost_object_id: Uuid,
        name: String,
        activities: Vec<BillOfActivityLine>,
    ) -> Result<BillOfActivities> {
        let total: i64 = activities.iter().map(|a| a.total_cost.amount).sum();
        let boa = BillOfActivities {
            base: BaseEntity::new(),
            cost_object_id,
            name,
            version: 1,
            activities,
            total_cost: Money::new(total, Currency::USD),
            is_active: true,
        };
        Ok(boa)
    }

    pub async fn calculate_product_cost(&self, boa: &BillOfActivities) -> Money {
        boa.total_cost.clone()
    }
}
