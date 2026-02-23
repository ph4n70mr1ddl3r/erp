use crate::models::*;
use async_trait::async_trait;
use erp_core::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait LearningRepository: Send + Sync {}

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
