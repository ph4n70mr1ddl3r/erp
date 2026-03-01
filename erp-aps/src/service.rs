use crate::models::*;
use crate::repository::*;
use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

pub struct ApsService {
    repo: SqliteApsRepository,
}

impl Default for ApsService {
    fn default() -> Self {
        Self::new()
    }
}

impl ApsService {
    pub fn new() -> Self {
        Self {
            repo: SqliteApsRepository::new(),
        }
    }
    
    pub async fn create_mps(&self, _pool: &SqlitePool, mut mps: MasterProductionSchedule) -> Result<MasterProductionSchedule> {
        mps.id = Uuid::new_v4();
        mps.schedule_number = format!("MPS-{}", Utc::now().format("%Y%m%d%H%M%S"));
        mps.created_at = Utc::now();
        mps.updated_at = Utc::now();
        mps.status = ScheduleStatus::Draft;
        
        self.repo.create_mps(&mps).await?;
        Ok(mps)
    }
    
    pub async fn get_mps(&self, _pool: &SqlitePool, id: Uuid) -> Result<Option<MasterProductionSchedule>> {
        self.repo.get_mps(id).await
    }
    
    pub async fn list_mps(&self, _pool: &SqlitePool) -> Result<Vec<MasterProductionSchedule>> {
        self.repo.list_mps().await
    }
    
    pub async fn run_mrp(&self, _pool: &SqlitePool, mps_id: Option<Uuid>, horizon_days: i32) -> Result<MaterialRequirementsPlan> {
        let mut mrp = MaterialRequirementsPlan {
            id: Uuid::new_v4(),
            mrp_number: format!("MRP-{}", Utc::now().format("%Y%m%d%H%M%S")),
            mps_id,
            planning_date: Utc::now(),
            planning_horizon_days: horizon_days,
            regenerate: true,
            status: ScheduleStatus::InProgress,
            run_started_at: Some(Utc::now()),
            run_completed_at: None,
            created_at: Utc::now(),
        };
        
        self.repo.create_mrp(&mrp).await?;
        
        mrp.run_completed_at = Some(Utc::now());
        mrp.status = ScheduleStatus::Completed;
        
        Ok(mrp)
    }
    
    pub async fn get_mrp(&self, _pool: &SqlitePool, id: Uuid) -> Result<Option<MaterialRequirementsPlan>> {
        self.repo.get_mrp(id).await
    }
    
    pub async fn list_mrp(&self, _pool: &SqlitePool) -> Result<Vec<MaterialRequirementsPlan>> {
        self.repo.list_mrp().await
    }
    
    pub async fn create_detailed_schedule(&self, _pool: &SqlitePool, mut schedule: DetailedSchedule) -> Result<DetailedSchedule> {
        schedule.id = Uuid::new_v4();
        schedule.schedule_number = format!("SCH-{}", Utc::now().format("%Y%m%d%H%M%S"));
        schedule.created_at = Utc::now();
        schedule.status = ScheduleStatus::Draft;
        
        self.repo.create_detailed_schedule(&schedule).await?;
        Ok(schedule)
    }
    
    pub async fn get_detailed_schedule(&self, _pool: &SqlitePool, id: Uuid) -> Result<Option<DetailedSchedule>> {
        self.repo.get_detailed_schedule(id).await
    }
    
    pub async fn list_detailed_schedules(&self, _pool: &SqlitePool) -> Result<Vec<DetailedSchedule>> {
        self.repo.list_detailed_schedules().await
    }
    
    pub async fn optimize_schedule(&self, _pool: &SqlitePool, schedule_id: Uuid, _goal: String) -> Result<DetailedSchedule> {
        let schedule = self.repo.get_detailed_schedule(schedule_id).await?;
        
        Ok(schedule.unwrap())
    }
    
    pub async fn analyze_capacity(&self, _pool: &SqlitePool, _resource_id: Option<Uuid>, start_date: chrono::DateTime<Utc>, end_date: chrono::DateTime<Utc>) -> Result<CapacityPlan> {
        let plan = CapacityPlan {
            id: Uuid::new_v4(),
            plan_number: format!("CAP-{}", Utc::now().format("%Y%m%d%H%M%S")),
            name: "Capacity Analysis".to_string(),
            planning_horizon_days: ((end_date - start_date).num_days()) as i32,
            bucket_size: "Daily".to_string(),
            status: ScheduleStatus::Completed,
            created_at: Utc::now(),
        };
        
        Ok(plan)
    }
    
    pub async fn create_resource_capacity(&self, _pool: &SqlitePool, mut capacity: ResourceCapacity) -> Result<ResourceCapacity> {
        capacity.id = Uuid::new_v4();
        capacity.created_at = Utc::now();
        
        self.repo.create_resource_capacity(&capacity).await?;
        Ok(capacity)
    }
    
    pub async fn get_planning_exceptions(&self, _pool: &SqlitePool) -> Result<Vec<PlanningException>> {
        Ok(Vec::new())
    }
    
    pub async fn create_what_if_scenario(&self, _pool: &SqlitePool, mut scenario: WhatIfScenario) -> Result<WhatIfScenario> {
        scenario.id = Uuid::new_v4();
        scenario.scenario_number = format!("WIF-{}", Utc::now().format("%Y%m%d%H%M%S"));
        scenario.created_at = Utc::now();
        
        Ok(scenario)
    }
}
