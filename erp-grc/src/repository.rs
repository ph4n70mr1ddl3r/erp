use async_trait::async_trait;
use sqlx::SqlitePool;

#[async_trait]
pub trait GRCRepository: Send + Sync {}

pub struct SqliteGRCRepository {
    pool: SqlitePool,
}

impl SqliteGRCRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GRCRepository for SqliteGRCRepository {}
