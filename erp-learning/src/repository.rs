use async_trait::async_trait;
use sqlx::SqlitePool;

#[async_trait]
pub trait LearningRepository: Send + Sync {}

#[allow(dead_code)]
pub struct SqliteLearningRepository {
    pool: SqlitePool,
}

impl SqliteLearningRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LearningRepository for SqliteLearningRepository {}
