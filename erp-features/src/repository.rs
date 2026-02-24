use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::Result;
use crate::models::*;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        sqlx::query!(
            r#"INSERT INTO feature_flags (id, key, name, description, enabled, rollout_percentage,
               target_type, target_ids, start_time, end_time, prerequisites, variants, default_variant,
               is_system, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            flag.base.id.to_string(),
            flag.key,
            flag.name,
            flag.description,
            flag.enabled as i32,
            flag.rollout_percentage,
            format!("{:?}", flag.target_type),
            flag.target_ids,
            flag.start_time.map(|t| t.to_rfc3339()),
            flag.end_time.map(|t| t.to_rfc3339()),
            flag.prerequisites,
            flag.variants,
            flag.default_variant,
            flag.is_system as i32,
            flag.base.created_at.to_rfc3339(),
            flag.base.updated_at.to_rfc3339(),
            flag.base.created_by.map(|id| id.to_string()),
            flag.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(flag)
    }

    async fn get_flag(&self, pool: &SqlitePool, key: &str) -> Result<Option<FeatureFlag>> {
        let row = sqlx::query!(
            r#"SELECT id, key, name, description, enabled, rollout_percentage, target_type,
               target_ids, start_time, end_time, prerequisites, variants, default_variant,
               is_system, created_at, updated_at
               FROM feature_flags WHERE key = ?"#,
            key
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|r| FeatureFlag {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            key: r.key,
            name: r.name,
            description: r.description,
            enabled: r.enabled == 1,
            rollout_percentage: r.rollout_percentage,
            target_type: FlagTargetType::All,
            target_ids: r.target_ids,
            start_time: r.start_time.and_then(|t| chrono::DateTime::parse_from_rfc3339(&t).ok().map(|t| t.with_timezone(&chrono::Utc))),
            end_time: r.end_time.and_then(|t| chrono::DateTime::parse_from_rfc3339(&t).ok().map(|t| t.with_timezone(&chrono::Utc))),
            prerequisites: r.prerequisites,
            variants: r.variants,
            default_variant: r.default_variant,
            is_system: r.is_system == 1,
        }))
    }

    async fn get_flag_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<FeatureFlag>> {
        let row = sqlx::query!(
            r#"SELECT id, key, name, description, enabled, rollout_percentage, target_type,
               target_ids, start_time, end_time, prerequisites, variants, default_variant,
               is_system, created_at, updated_at
               FROM feature_flags WHERE id = ?"#,
            id.to_string()
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|r| FeatureFlag {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            key: r.key,
            name: r.name,
            description: r.description,
            enabled: r.enabled == 1,
            rollout_percentage: r.rollout_percentage,
            target_type: FlagTargetType::All,
            target_ids: r.target_ids,
            start_time: r.start_time.and_then(|t| chrono::DateTime::parse_from_rfc3339(&t).ok().map(|t| t.with_timezone(&chrono::Utc))),
            end_time: r.end_time.and_then(|t| chrono::DateTime::parse_from_rfc3339(&t).ok().map(|t| t.with_timezone(&chrono::Utc))),
            prerequisites: r.prerequisites,
            variants: r.variants,
            default_variant: r.default_variant,
            is_system: r.is_system == 1,
        }))
    }

    async fn list_flags(&self, pool: &SqlitePool) -> Result<Vec<FeatureFlag>> {
        let rows = sqlx::query!(
            r#"SELECT id, key, name, description, enabled, rollout_percentage, target_type,
               target_ids, start_time, end_time, prerequisites, variants, default_variant,
               is_system, created_at, updated_at
               FROM feature_flags ORDER BY key"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| FeatureFlag {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            key: r.key,
            name: r.name,
            description: r.description,
            enabled: r.enabled == 1,
            rollout_percentage: r.rollout_percentage,
            target_type: FlagTargetType::All,
            target_ids: r.target_ids,
            start_time: r.start_time.and_then(|t| chrono::DateTime::parse_from_rfc3339(&t).ok().map(|t| t.with_timezone(&chrono::Utc))),
            end_time: r.end_time.and_then(|t| chrono::DateTime::parse_from_rfc3339(&t).ok().map(|t| t.with_timezone(&chrono::Utc))),
            prerequisites: r.prerequisites,
            variants: r.variants,
            default_variant: r.default_variant,
            is_system: r.is_system == 1,
        }).collect())
    }

    async fn update_flag(&self, pool: &SqlitePool, flag: FeatureFlag) -> Result<FeatureFlag> {
        sqlx::query!(
            r#"UPDATE feature_flags SET name = ?, description = ?, enabled = ?, rollout_percentage = ?,
               target_type = ?, target_ids = ?, start_time = ?, end_time = ?, prerequisites = ?,
               variants = ?, default_variant = ?, updated_at = ?, updated_by = ?
               WHERE id = ?"#,
            flag.name,
            flag.description,
            flag.enabled as i32,
            flag.rollout_percentage,
            format!("{:?}", flag.target_type),
            flag.target_ids,
            flag.start_time.map(|t| t.to_rfc3339()),
            flag.end_time.map(|t| t.to_rfc3339()),
            flag.prerequisites,
            flag.variants,
            flag.default_variant,
            flag.base.updated_at.to_rfc3339(),
            flag.base.updated_by.map(|id| id.to_string()),
            flag.base.id.to_string(),
        ).execute(pool).await?;
        Ok(flag)
    }

    async fn delete_flag(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM feature_flags WHERE id = ? AND is_system = 0", id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn create_override(&self, pool: &SqlitePool, override_: FeatureFlagOverride) -> Result<FeatureFlagOverride> {
        sqlx::query!(
            r#"INSERT INTO feature_flag_overrides (id, flag_id, target_type, target_id, enabled, variant, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#,
            override_.base.id.to_string(),
            override_.flag_id.to_string(),
            format!("{:?}", override_.target_type),
            override_.target_id,
            override_.enabled as i32,
            override_.variant,
            override_.base.created_at.to_rfc3339(),
            override_.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(override_)
    }

    async fn get_overrides(&self, pool: &SqlitePool, flag_id: Uuid) -> Result<Vec<FeatureFlagOverride>> {
        let rows = sqlx::query!(
            r#"SELECT id, flag_id, target_type, target_id, enabled, variant, created_at, updated_at
               FROM feature_flag_overrides WHERE flag_id = ?"#,
            flag_id.to_string()
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| FeatureFlagOverride {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            flag_id: Uuid::parse_str(&r.flag_id).unwrap(),
            target_type: OverrideTargetType::User,
            target_id: r.target_id,
            enabled: r.enabled == 1,
            variant: r.variant,
        }).collect())
    }

    async fn delete_override(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM feature_flag_overrides WHERE id = ?", id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn log_usage(&self, pool: &SqlitePool, usage: FeatureFlagUsage) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO feature_flag_usage (id, flag_id, user_id, variant, evaluated_at, context)
               VALUES (?, ?, ?, ?, ?, ?)"#,
            usage.base.id.to_string(),
            usage.flag_id.to_string(),
            usage.user_id.map(|id| id.to_string()),
            usage.variant,
            usage.evaluated_at.to_rfc3339(),
            usage.context,
        ).execute(pool).await?;
        Ok(())
    }

    async fn get_usage_stats(&self, pool: &SqlitePool, flag_id: Uuid) -> Result<FlagUsageStats> {
        let total: i64 = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM feature_flag_usage WHERE flag_id = ?",
            flag_id.to_string()
        ).fetch_one(pool).await?.unwrap_or(0);
        
        let unique: i64 = sqlx::query_scalar!(
            "SELECT COUNT(DISTINCT user_id) FROM feature_flag_usage WHERE flag_id = ?",
            flag_id.to_string()
        ).fetch_one(pool).await?.unwrap_or(0);
        
        Ok(FlagUsageStats {
            flag_id,
            total_evaluations: total,
            unique_users: unique,
            variant_counts: std::collections::HashMap::new(),
        })
    }
}
