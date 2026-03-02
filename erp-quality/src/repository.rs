use async_trait::async_trait;
use sqlx::SqlitePool;

#[async_trait]
pub trait QualityRepository: Send + Sync {}

#[allow(dead_code)]
pub struct SqliteQualityRepository {
    pool: SqlitePool,
}

impl SqliteQualityRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl QualityRepository for SqliteQualityRepository {}
