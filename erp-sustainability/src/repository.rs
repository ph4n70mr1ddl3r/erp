use crate::models::*;
use async_trait::async_trait;
use erp_core::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait SustainabilityRepository: Send + Sync {}

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
