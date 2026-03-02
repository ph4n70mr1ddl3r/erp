use async_trait::async_trait;
use sqlx::SqlitePool;

#[async_trait]
pub trait TreasuryRepository: Send + Sync {}

#[allow(dead_code)]
pub struct SqliteTreasuryRepository {
    pool: SqlitePool,
}

impl SqliteTreasuryRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TreasuryRepository for SqliteTreasuryRepository {}
