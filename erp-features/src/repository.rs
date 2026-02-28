use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::{Result, Error};
use crate::models::*;

#[derive(sqlx::FromRow)]
struct FeatureFlagRow {
    id: String,
    key: String,
    name: String,
    description: Option<String>,
    enabled: i32,
    rollout_percentage: i32,
    target_type: String,
    target_ids: Option<String>,
    start_time: Option<String>,
    end_time: Option<String>,
    prerequisites: Option<String>,
    variants: Option<String>,
    default_variant: Option<String>,
    is_system: i32,
    created_at: String,
    updated_at: String,
    created_by: Option<String>,
    updated_by: Option<String>,
}

impl FeatureFlagRow {
    fn into_feature_flag(self) -> Result<FeatureFlag> {
        let id = Uuid::parse_str(&self.id)
            .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid UUID '{}': {}", self.id, e)))?;
        
        Ok(FeatureFlag {
            base: erp_core::BaseEntity {
                id,
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
                    .map(|d| d.with_timezone(&chrono::Utc))
                    .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid datetime: {}", e)))?,
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at)
                    .map(|d| d.with_timezone(&chrono::Utc))
                    .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid datetime: {}", e)))?,
                created_by: self.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: self.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            key: self.key,
            name: self.name,
            description: self.description,
            enabled: self.enabled == 1,
            rollout_percentage: self.rollout_percentage,
            target_type: FlagTargetType::All,
            target_ids: self.target_ids,
            start_time: self.start_time.and_then(|t| chrono::DateTime::parse_from_rfc3339(&t).ok().map(|t| t.with_timezone(&chrono::Utc))),
            end_time: self.end_time.and_then(|t| chrono::DateTime::parse_from_rfc3339(&t).ok().map(|t| t.with_timezone(&chrono::Utc))),
            prerequisites: self.prerequisites,
            variants: self.variants,
            default_variant: self.default_variant,
            is_system: self.is_system == 1,
        })
    }
}

#[derive(sqlx::FromRow)]
struct FeatureFlagOverrideRow {
    id: String,
    flag_id: String,
    target_type: String,
    target_id: String,
    enabled: i32,
    variant: Option<String>,
    created_at: String,
    updated_at: String,
}

impl FeatureFlagOverrideRow {
    fn into_override(self) -> Result<FeatureFlagOverride> {
        let id = Uuid::parse_str(&self.id)
            .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid UUID '{}': {}", self.id, e)))?;
        let flag_id = Uuid::parse_str(&self.flag_id)
            .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid UUID '{}': {}", self.flag_id, e)))?;
        
        Ok(FeatureFlagOverride {
            base: erp_core::BaseEntity {
                id,
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
                    .map(|d| d.with_timezone(&chrono::Utc))
                    .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid datetime: {}", e)))?,
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at)
                    .map(|d| d.with_timezone(&chrono::Utc))
                    .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid datetime: {}", e)))?,
                created_by: None,
                updated_by: None,
            },
            flag_id,
            target_type: OverrideTargetType::User,
            target_id: self.target_id,
            enabled: self.enabled == 1,
            variant: self.variant,
        })
    }
}

#[async_trait]
pub trait FeatureFlagRepository: Send + Sync {
    async fn create_flag(&self, pool: &SqlitePool, flag: FeatureFlag) -> Result<FeatureFlag>;
    async fn get_flag(&self, pool: &SqlitePool, key: &str) -> Result<Option<FeatureFlag>>;
    async fn get_flag_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<FeatureFlag>>;
    async fn list_flags(&self, pool: &SqlitePool) -> Result<Vec<FeatureFlag>>;
    async fn update_flag(&self, pool: &SqlitePool, flag: FeatureFlag) -> Result<FeatureFlag>;
    async fn delete_flag(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn create_override(&self, pool: &SqlitePool, override_: FeatureFlagOverride) -> Result<FeatureFlagOverride>;
    async fn get_overrides(&self, pool: &SqlitePool, flag_id: Uuid) -> Result<Vec<FeatureFlagOverride>>;
    async fn delete_override(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn log_usage(&self, pool: &SqlitePool, usage: FeatureFlagUsage) -> Result<()>;
    async fn get_usage_stats(&self, pool: &SqlitePool, flag_id: Uuid) -> Result<FlagUsageStats>;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FlagUsageStats {
    pub flag_id: Uuid,
    pub total_evaluations: i64,
    pub unique_users: i64,
    pub variant_counts: std::collections::HashMap<String, i64>,
}

pub struct SqliteFeatureFlagRepository;

#[async_trait]
impl FeatureFlagRepository for SqliteFeatureFlagRepository {
    async fn create_flag(&self, pool: &SqlitePool, flag: FeatureFlag) -> Result<FeatureFlag> {
        sqlx::query(r#"INSERT INTO feature_flags (id, key, name, description, enabled, rollout_percentage,
               target_type, target_ids, start_time, end_time, prerequisites, variants, default_variant,
               is_system, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(flag.base.id.to_string())
            .bind(&flag.key)
            .bind(&flag.name)
            .bind(&flag.description)
            .bind(flag.enabled as i32)
            .bind(flag.rollout_percentage)
            .bind(format!("{:?}", flag.target_type))
            .bind(&flag.target_ids)
            .bind(flag.start_time.map(|t| t.to_rfc3339()))
            .bind(flag.end_time.map(|t| t.to_rfc3339()))
            .bind(&flag.prerequisites)
            .bind(&flag.variants)
            .bind(&flag.default_variant)
            .bind(flag.is_system as i32)
            .bind(flag.base.created_at.to_rfc3339())
            .bind(flag.base.updated_at.to_rfc3339())
            .bind(flag.base.created_by.map(|id| id.to_string()))
            .bind(flag.base.updated_by.map(|id| id.to_string()))
            .execute(pool).await?;
        Ok(flag)
    }

    async fn get_flag(&self, pool: &SqlitePool, key: &str) -> Result<Option<FeatureFlag>> {
        let row = sqlx::query_as::<_, FeatureFlagRow>(
            r#"SELECT id, key, name, description, enabled, rollout_percentage, target_type,
               target_ids, start_time, end_time, prerequisites, variants, default_variant,
               is_system, created_at, updated_at, created_by, updated_by
               FROM feature_flags WHERE key = ?"#,
        )
        .bind(key)
        .fetch_optional(pool)
        .await?;
        
        row.map(|r| r.into_feature_flag()).transpose()
    }

    async fn get_flag_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<FeatureFlag>> {
        let row = sqlx::query_as::<_, FeatureFlagRow>(
            r#"SELECT id, key, name, description, enabled, rollout_percentage, target_type,
               target_ids, start_time, end_time, prerequisites, variants, default_variant,
               is_system, created_at, updated_at, created_by, updated_by
               FROM feature_flags WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;
        
        row.map(|r| r.into_feature_flag()).transpose()
    }

    async fn list_flags(&self, pool: &SqlitePool) -> Result<Vec<FeatureFlag>> {
        let rows = sqlx::query_as::<_, FeatureFlagRow>(
            r#"SELECT id, key, name, description, enabled, rollout_percentage, target_type,
               target_ids, start_time, end_time, prerequisites, variants, default_variant,
               is_system, created_at, updated_at, created_by, updated_by
               FROM feature_flags ORDER BY key"#,
        )
        .fetch_all(pool)
        .await?;
        
        rows.into_iter().map(|r| r.into_feature_flag()).collect()
    }

    async fn update_flag(&self, pool: &SqlitePool, flag: FeatureFlag) -> Result<FeatureFlag> {
        sqlx::query(r#"UPDATE feature_flags SET name = ?, description = ?, enabled = ?, rollout_percentage = ?,
               target_type = ?, target_ids = ?, start_time = ?, end_time = ?, prerequisites = ?,
               variants = ?, default_variant = ?, updated_at = ?, updated_by = ?
               WHERE id = ?"#)
            .bind(&flag.name)
            .bind(&flag.description)
            .bind(flag.enabled as i32)
            .bind(flag.rollout_percentage)
            .bind(format!("{:?}", flag.target_type))
            .bind(&flag.target_ids)
            .bind(flag.start_time.map(|t| t.to_rfc3339()))
            .bind(flag.end_time.map(|t| t.to_rfc3339()))
            .bind(&flag.prerequisites)
            .bind(&flag.variants)
            .bind(&flag.default_variant)
            .bind(flag.base.updated_at.to_rfc3339())
            .bind(flag.base.updated_by.map(|id| id.to_string()))
            .bind(flag.base.id.to_string())
            .execute(pool).await?;
        Ok(flag)
    }

    async fn delete_flag(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM feature_flags WHERE id = ? AND is_system = 0")
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn create_override(&self, pool: &SqlitePool, override_: FeatureFlagOverride) -> Result<FeatureFlagOverride> {
        sqlx::query(r#"INSERT INTO feature_flag_overrides (id, flag_id, target_type, target_id, enabled, variant, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(override_.base.id.to_string())
            .bind(override_.flag_id.to_string())
            .bind(format!("{:?}", override_.target_type))
            .bind(&override_.target_id)
            .bind(override_.enabled as i32)
            .bind(&override_.variant)
            .bind(override_.base.created_at.to_rfc3339())
            .bind(override_.base.updated_at.to_rfc3339())
            .execute(pool).await?;
        Ok(override_)
    }

    async fn get_overrides(&self, pool: &SqlitePool, flag_id: Uuid) -> Result<Vec<FeatureFlagOverride>> {
        let rows = sqlx::query_as::<_, FeatureFlagOverrideRow>(
            r#"SELECT id, flag_id, target_type, target_id, enabled, variant, created_at, updated_at
               FROM feature_flag_overrides WHERE flag_id = ?"#,
        )
        .bind(flag_id.to_string())
        .fetch_all(pool)
        .await?;
        
        rows.into_iter().map(|r| r.into_override()).collect()
    }

    async fn delete_override(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM feature_flag_overrides WHERE id = ?")
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn log_usage(&self, pool: &SqlitePool, usage: FeatureFlagUsage) -> Result<()> {
        sqlx::query(r#"INSERT INTO feature_flag_usage (id, flag_id, user_id, variant, evaluated_at, context)
               VALUES (?, ?, ?, ?, ?, ?)"#)
            .bind(usage.base.id.to_string())
            .bind(usage.flag_id.to_string())
            .bind(usage.user_id.map(|id| id.to_string()))
            .bind(&usage.variant)
            .bind(usage.evaluated_at.to_rfc3339())
            .bind(&usage.context)
            .execute(pool).await?;
        Ok(())
    }

    async fn get_usage_stats(&self, pool: &SqlitePool, flag_id: Uuid) -> Result<FlagUsageStats> {
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM feature_flag_usage WHERE flag_id = ?")
            .bind(flag_id.to_string())
            .fetch_one(pool)
            .await?;
        
        let unique: (i64,) = sqlx::query_as("SELECT COUNT(DISTINCT user_id) FROM feature_flag_usage WHERE flag_id = ?")
            .bind(flag_id.to_string())
            .fetch_one(pool)
            .await?;
        
        Ok(FlagUsageStats {
            flag_id,
            total_evaluations: total.0,
            unique_users: unique.0,
            variant_counts: std::collections::HashMap::new(),
        })
    }
}
