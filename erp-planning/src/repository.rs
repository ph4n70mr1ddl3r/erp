use async_trait::async_trait;
use sqlx::SqlitePool;

#[async_trait]
pub trait PlanningRepository: Send + Sync {}

pub struct SqlitePlanningRepository {
    pool: SqlitePool,
}

impl SqlitePlanningRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PlanningRepository for SqlitePlanningRepository {}
