use crate::models::*;
use async_trait::async_trait;
use erp_core::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait TreasuryRepository: Send + Sync {}

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
