use async_trait::async_trait;
use erp_core::error::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait QualityRuleRepository: Send + Sync {
    async fn create(&self, rule: &DataQualityRule) -> Result<DataQualityRule>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DataQualityRule>>;
    async fn find_all(&self) -> Result<Vec<DataQualityRule>>;
    async fn find_by_entity(&self, entity: &str) -> Result<Vec<DataQualityRule>>;
    async fn find_active(&self) -> Result<Vec<DataQualityRule>>;
    async fn update(&self, rule: &DataQualityRule) -> Result<DataQualityRule>;
    async fn delete(&self, id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait QualityExecutionRepository: Send + Sync {
    async fn create(&self, execution: &DataQualityExecution) -> Result<DataQualityExecution>;
    async fn find_by_rule(&self, rule_id: Uuid, limit: i32) -> Result<Vec<DataQualityExecution>>;
    async fn find_latest(&self, rule_id: Uuid) -> Result<Option<DataQualityExecution>>;
}

#[async_trait]
pub trait DataProfileRepository: Send + Sync {
    async fn create(&self, profile: &DataQualityProfile) -> Result<DataQualityProfile>;
    async fn find_by_entity(&self, entity: &str) -> Result<Option<DataQualityProfile>>;
    async fn find_latest(&self, entity: &str) -> Result<Option<DataQualityProfile>>;
}

#[async_trait]
pub trait CleansingJobRepository: Send + Sync {
    async fn create(&self, job: &DataCleansingJob) -> Result<DataCleansingJob>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<DataCleansingJob>>;
    async fn update(&self, job: &DataCleansingJob) -> Result<DataCleansingJob>;
}

#[allow(dead_code)]
pub struct SqliteQualityRuleRepository {
    pool: SqlitePool,
}

impl SqliteQualityRuleRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl QualityRuleRepository for SqliteQualityRuleRepository {
    async fn create(&self, rule: &DataQualityRule) -> Result<DataQualityRule> {
        Ok(rule.clone())
    }

    async fn find_by_id(&self, _id: Uuid) -> Result<Option<DataQualityRule>> {
        Ok(None)
    }

    async fn find_all(&self) -> Result<Vec<DataQualityRule>> {
        Ok(Vec::new())
    }

    async fn find_by_entity(&self, _entity: &str) -> Result<Vec<DataQualityRule>> {
        Ok(Vec::new())
    }

    async fn find_active(&self) -> Result<Vec<DataQualityRule>> {
        Ok(Vec::new())
    }

    async fn update(&self, rule: &DataQualityRule) -> Result<DataQualityRule> {
        Ok(rule.clone())
    }

    async fn delete(&self, _id: Uuid) -> Result<()> {
        Ok(())
    }
}
