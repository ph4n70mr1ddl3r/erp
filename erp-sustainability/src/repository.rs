use async_trait::async_trait;
use sqlx::SqlitePool;

#[async_trait]
pub trait SustainabilityRepository: Send + Sync {}

#[allow(dead_code)]
pub struct SqliteSustainabilityRepository {
    pool: SqlitePool,
}

impl SqliteSustainabilityRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl SustainabilityRepository for SqliteSustainabilityRepository {}
