use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{BaseEntity, Result};
use crate::models::*;
use crate::repository::{FeatureFlagRepository, SqliteFeatureFlagRepository, FlagUsageStats};

pub struct FeatureFlagService {
    repo: SqliteFeatureFlagRepository,
}

impl FeatureFlagService {
    pub fn new() -> Self {
        Self { repo: SqliteFeatureFlagRepository }
    }

    pub async fn create_flag(&self, pool: &SqlitePool, flag: FeatureFlag) -> Result<FeatureFlag> {
        self.repo.create_flag(pool, flag).await
    }

    pub async fn get_flag(&self, pool: &SqlitePool, key: &str) -> Result<Option<FeatureFlag>> {
        self.repo.get_flag(pool, key).await
    }

    pub async fn list_flags(&self, pool: &SqlitePool) -> Result<Vec<FeatureFlag>> {
        self.repo.list_flags(pool).await
    }

    pub async fn update_flag(&self, pool: &SqlitePool, flag: FeatureFlag) -> Result<FeatureFlag> {
        self.repo.update_flag(pool, flag).await
    }

    pub async fn delete_flag(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete_flag(pool, id).await
    }

    pub async fn is_enabled(&self, pool: &SqlitePool, key: &str, context: &FlagEvaluationContext) -> Result<bool> {
        let flag = self.repo.get_flag(pool, key).await?;
        
        match flag {
            Some(f) => self.evaluate_flag(pool, &f, context).await,
            None => Ok(false),
        }
    }

    async fn evaluate_flag(&self, pool: &SqlitePool, flag: &FeatureFlag, context: &FlagEvaluationContext) -> Result<bool> {
        if !flag.enabled {
            return Ok(false);
        }

        let now = Utc::now();
        if let Some(start) = flag.start_time {
            if now < start {
                return Ok(false);
            }
        }
        if let Some(end) = flag.end_time {
            if now > end {
                return Ok(false);
            }
        }

        if let Some(user_id) = context.user_id {
            let overrides = self.repo.get_overrides(pool, flag.base.id).await?;
            for override_ in overrides {
                if override_.target_type == OverrideTargetType::User && override_.target_id == user_id.to_string() {
                    return Ok(override_.enabled);
                }
            }
        }

        for group_id in &context.group_ids {
            let overrides = self.repo.get_overrides(pool, flag.base.id).await?;
            for override_ in overrides {
                if override_.target_type == OverrideTargetType::Group && override_.target_id == group_id.to_string() {
                    return Ok(override_.enabled);
                }
            }
        }

        if flag.rollout_percentage >= 100 {
            return Ok(true);
        }
        if flag.rollout_percentage <= 0 {
            return Ok(false);
        }

        Ok(flag.rollout_percentage >= 50)
    }

    pub async fn create_override(&self, pool: &SqlitePool, override_: FeatureFlagOverride) -> Result<FeatureFlagOverride> {
        self.repo.create_override(pool, override_).await
    }

    pub async fn delete_override(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete_override(pool, id).await
    }

    pub async fn toggle_flag(&self, pool: &SqlitePool, id: Uuid, enabled: bool) -> Result<FeatureFlag> {
        let mut flag = self.repo.get_flag_by_id(pool, id).await?
            .ok_or_else(|| anyhow::anyhow!("Flag not found"))?;
        flag.enabled = enabled;
        flag.base.updated_at = Utc::now();
        self.repo.update_flag(pool, flag).await
    }

    pub async fn get_usage_stats(&self, pool: &SqlitePool, flag_id: Uuid) -> Result<FlagUsageStats> {
        self.repo.get_usage_stats(pool, flag_id).await
    }
}
