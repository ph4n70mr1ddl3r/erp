use async_trait::async_trait;
use erp_core::Result;
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::*;

pub struct SqlitePerformanceRepository;

#[async_trait]
pub trait PerformanceRepository: Send + Sync {
    async fn create_cycle(&self, pool: &SqlitePool, cycle: &PerformanceCycle) -> Result<PerformanceCycle>;
    async fn list_cycles(&self, pool: &SqlitePool) -> Result<Vec<PerformanceCycle>>;
    async fn get_cycle(&self, pool: &SqlitePool, id: Uuid) -> Result<PerformanceCycle>;
    async fn activate_cycle(&self, pool: &SqlitePool, id: Uuid) -> Result<PerformanceCycle>;
    async fn close_cycle(&self, pool: &SqlitePool, id: Uuid) -> Result<PerformanceCycle>;
    async fn create_goal(&self, pool: &SqlitePool, goal: &PerformanceGoal) -> Result<PerformanceGoal>;
    async fn list_goals_by_cycle(&self, pool: &SqlitePool, cycle_id: Uuid) -> Result<Vec<PerformanceGoal>>;
    async fn list_goals_by_employee(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<Vec<PerformanceGoal>>;
    async fn get_goal(&self, pool: &SqlitePool, id: Uuid) -> Result<PerformanceGoal>;
    async fn update_goal(&self, pool: &SqlitePool, goal: &PerformanceGoal) -> Result<PerformanceGoal>;
    async fn create_review(&self, pool: &SqlitePool, review: &PerformanceReview) -> Result<PerformanceReview>;
    async fn list_reviews_by_cycle(&self, pool: &SqlitePool, cycle_id: Uuid) -> Result<Vec<PerformanceReview>>;
    async fn list_reviews_by_employee(&self, pool: &SqlitePool, employee_id: Uuid) -> Result<Vec<PerformanceReview>>;
    async fn get_review(&self, pool: &SqlitePool, id: Uuid) -> Result<PerformanceReview>;
    async fn submit_review(&self, pool: &SqlitePool, id: Uuid, review: SubmitReviewRequest) -> Result<PerformanceReview>;
}

impl SqlitePerformanceRepository {
    pub fn new() -> Self { Self }
}

impl Default for SqlitePerformanceRepository {
    fn default() -> Self { Self::new() }
}
