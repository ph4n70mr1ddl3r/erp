use async_trait::async_trait;
use sqlx::SqlitePool;

#[async_trait]
pub trait ExpensesRepository: Send + Sync {}

#[allow(dead_code)]
pub struct SqliteExpensesRepository {
    pool: SqlitePool,
}

impl SqliteExpensesRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ExpensesRepository for SqliteExpensesRepository {}
