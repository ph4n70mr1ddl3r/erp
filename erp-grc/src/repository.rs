use crate::models::*;
use async_trait::async_trait;
use erp_core::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

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
