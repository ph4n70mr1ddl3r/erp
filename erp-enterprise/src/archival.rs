use chrono::{DateTime, Utc};
use erp_core::Result;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchivedRecord {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: String,
    pub original_data: serde_json::Value,
    pub archived_at: DateTime<Utc>,
    pub archived_by: Uuid,
    pub retention_until: Option<DateTime<Utc>>,
    pub restored_at: Option<DateTime<Utc>>,
    pub restored_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataRetentionPolicy {
    pub id: Uuid,
    pub entity_type: String,
    pub retention_days: i32,
    pub archive_after_days: i32,
    pub delete_after_archive_days: Option<i32>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRetentionPolicyRequest {
    pub entity_type: String,
    pub retention_days: i32,
    pub archive_after_days: i32,
    pub delete_after_archive_days: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveRequest {
    pub entity_type: String,
    pub entity_id: String,
    pub data: serde_json::Value,
    pub retention_days: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreRequest {
    pub archive_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchiveStats {
    pub total_archived: i64,
    pub by_entity_type: Vec<EntityArchiveCount>,
    pub pending_deletion: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityArchiveCount {
    pub entity_type: String,
    pub count: i64,
}

#[derive(Debug, sqlx::FromRow)]
struct EntityArchiveCountRow {
    entity_type: String,
    count: i64,
}

pub struct ArchivalService;

impl Default for ArchivalService {
    fn default() -> Self {
        Self::new()
    }
}

impl ArchivalService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_retention_policy(
        &self,
        pool: &SqlitePool,
        req: CreateRetentionPolicyRequest,
    ) -> Result<DataRetentionPolicy> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"INSERT INTO data_retention_policies 
               (id, entity_type, retention_days, archive_after_days, delete_after_archive_days, is_active, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, 1, ?, ?)
               ON CONFLICT(entity_type) DO UPDATE SET 
               retention_days = excluded.retention_days,
               archive_after_days = excluded.archive_after_days,
               delete_after_archive_days = excluded.delete_after_archive_days,
               is_active = 1,
               updated_at = excluded.updated_at"#
        )
        .bind(id.to_string())
        .bind(&req.entity_type)
        .bind(req.retention_days)
        .bind(req.archive_after_days)
        .bind(req.delete_after_archive_days)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(DataRetentionPolicy {
            id,
            entity_type: req.entity_type,
            retention_days: req.retention_days,
            archive_after_days: req.archive_after_days,
            delete_after_archive_days: req.delete_after_archive_days,
            is_active: true,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_retention_policy(&self, pool: &SqlitePool, entity_type: &str) -> Result<Option<DataRetentionPolicy>> {
        let row = sqlx::query_as::<_, RetentionPolicyRow>(
            "SELECT id, entity_type, retention_days, archive_after_days, delete_after_archive_days, is_active, created_at, updated_at FROM data_retention_policies WHERE entity_type = ?"
        )
        .bind(entity_type)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.into_model()))
    }

    pub async fn list_retention_policies(&self, pool: &SqlitePool) -> Result<Vec<DataRetentionPolicy>> {
        let rows = sqlx::query_as::<_, RetentionPolicyRow>(
            "SELECT id, entity_type, retention_days, archive_after_days, delete_after_archive_days, is_active, created_at, updated_at FROM data_retention_policies WHERE is_active = 1 ORDER BY entity_type"
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into_model()).collect())
    }

    pub async fn archive_record(
        &self,
        pool: &SqlitePool,
        req: ArchiveRequest,
        archived_by: Uuid,
    ) -> Result<ArchivedRecord> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        let retention_until = req.retention_days.map(|days| {
            now + chrono::Duration::days(days as i64)
        });

        let data_json = serde_json::to_string(&req.data).unwrap_or_default();

        sqlx::query(
            r#"INSERT INTO archived_records 
               (id, entity_type, entity_id, original_data, archived_at, archived_by, retention_until, restored_at, restored_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, NULL, NULL)"#
        )
        .bind(id.to_string())
        .bind(&req.entity_type)
        .bind(&req.entity_id)
        .bind(&data_json)
        .bind(now.to_rfc3339())
        .bind(archived_by.to_string())
        .bind(retention_until.map(|d| d.to_rfc3339()))
        .execute(pool)
        .await?;

        Ok(ArchivedRecord {
            id,
            entity_type: req.entity_type,
            entity_id: req.entity_id,
            original_data: req.data,
            archived_at: now,
            archived_by,
            retention_until,
            restored_at: None,
            restored_by: None,
        })
    }

    pub async fn get_archived_record(&self, pool: &SqlitePool, id: Uuid) -> Result<ArchivedRecord> {
        let row = sqlx::query_as::<_, ArchivedRecordRow>(
            "SELECT id, entity_type, entity_id, original_data, archived_at, archived_by, retention_until, restored_at, restored_by FROM archived_records WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| erp_core::Error::not_found("ArchivedRecord", &id.to_string()))?;

        Ok(row.into_model())
    }

    pub async fn list_archived_records(
        &self,
        pool: &SqlitePool,
        entity_type: Option<&str>,
        limit: i32,
        offset: i32,
    ) -> Result<Vec<ArchivedRecord>> {
        let rows = if let Some(et) = entity_type {
            sqlx::query_as::<_, ArchivedRecordRow>(
                "SELECT id, entity_type, entity_id, original_data, archived_at, archived_by, retention_until, restored_at, restored_by FROM archived_records WHERE entity_type = ? AND restored_at IS NULL ORDER BY archived_at DESC LIMIT ? OFFSET ?"
            )
            .bind(et)
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        } else {
            sqlx::query_as::<_, ArchivedRecordRow>(
                "SELECT id, entity_type, entity_id, original_data, archived_at, archived_by, retention_until, restored_at, restored_by FROM archived_records WHERE restored_at IS NULL ORDER BY archived_at DESC LIMIT ? OFFSET ?"
            )
            .bind(limit)
            .bind(offset)
            .fetch_all(pool)
            .await?
        };

        Ok(rows.into_iter().map(|r| r.into_model()).collect())
    }

    pub async fn restore_record(
        &self,
        pool: &SqlitePool,
        id: Uuid,
        restored_by: Uuid,
    ) -> Result<ArchivedRecord> {
        let now = Utc::now();

        sqlx::query("UPDATE archived_records SET restored_at = ?, restored_by = ? WHERE id = ?")
            .bind(now.to_rfc3339())
            .bind(restored_by.to_string())
            .bind(id.to_string())
            .execute(pool)
            .await?;

        self.get_archived_record(pool, id).await
    }

    pub async fn delete_archived_record(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM archived_records WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn purge_expired_records(&self, pool: &SqlitePool) -> Result<u64> {
        let now = Utc::now();
        let result = sqlx::query("DELETE FROM archived_records WHERE retention_until IS NOT NULL AND retention_until < ? AND restored_at IS NULL")
            .bind(now.to_rfc3339())
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn get_stats(&self, pool: &SqlitePool) -> Result<ArchiveStats> {
        let total_archived: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM archived_records WHERE restored_at IS NULL")
            .fetch_one(pool)
            .await?;

        let rows: Vec<EntityArchiveCountRow> = sqlx::query_as(
            "SELECT entity_type, COUNT(*) as count FROM archived_records WHERE restored_at IS NULL GROUP BY entity_type ORDER BY count DESC"
        )
        .fetch_all(pool)
        .await?;
        
        let by_entity_type: Vec<EntityArchiveCount> = rows.into_iter().map(|r| EntityArchiveCount {
            entity_type: r.entity_type,
            count: r.count,
        }).collect();

        let pending_deletion: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM archived_records WHERE retention_until IS NOT NULL AND retention_until < ? AND restored_at IS NULL"
        )
        .bind(Utc::now().to_rfc3339())
        .fetch_one(pool)
        .await?;

        Ok(ArchiveStats {
            total_archived,
            by_entity_type,
            pending_deletion,
        })
    }
}

#[derive(Debug, sqlx::FromRow)]
struct ArchivedRecordRow {
    id: String,
    entity_type: String,
    entity_id: String,
    original_data: String,
    archived_at: String,
    archived_by: String,
    retention_until: Option<String>,
    restored_at: Option<String>,
    restored_by: Option<String>,
}

impl ArchivedRecordRow {
    fn into_model(self) -> ArchivedRecord {
        ArchivedRecord {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            entity_type: self.entity_type,
            entity_id: self.entity_id,
            original_data: serde_json::from_str(&self.original_data).unwrap_or(serde_json::json!(null)),
            archived_at: DateTime::parse_from_rfc3339(&self.archived_at).ok().map(|d| d.with_timezone(&Utc)).unwrap_or_else(Utc::now),
            archived_by: Uuid::parse_str(&self.archived_by).unwrap_or_default(),
            retention_until: self.retention_until.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            restored_at: self.restored_at.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            restored_by: self.restored_by.and_then(|id| Uuid::parse_str(&id).ok()),
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
struct RetentionPolicyRow {
    id: String,
    entity_type: String,
    retention_days: i32,
    archive_after_days: i32,
    delete_after_archive_days: Option<i32>,
    is_active: i32,
    created_at: String,
    updated_at: String,
}

impl RetentionPolicyRow {
    fn into_model(self) -> DataRetentionPolicy {
        DataRetentionPolicy {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            entity_type: self.entity_type,
            retention_days: self.retention_days,
            archive_after_days: self.archive_after_days,
            delete_after_archive_days: self.delete_after_archive_days,
            is_active: self.is_active != 0,
            created_at: DateTime::parse_from_rfc3339(&self.created_at).ok().map(|d| d.with_timezone(&Utc)).unwrap_or_else(Utc::now),
            updated_at: DateTime::parse_from_rfc3339(&self.updated_at).ok().map(|d| d.with_timezone(&Utc)).unwrap_or_else(Utc::now),
        }
    }
}
