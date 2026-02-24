use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::Result;
use crate::models::*;

#[async_trait]
pub trait BackupRepository: Send + Sync {
    async fn create_schedule(&self, pool: &SqlitePool, schedule: BackupSchedule) -> Result<BackupSchedule>;
    async fn get_schedule(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<BackupSchedule>>;
    async fn list_schedules(&self, pool: &SqlitePool) -> Result<Vec<BackupSchedule>>;
    async fn update_schedule(&self, pool: &SqlitePool, schedule: BackupSchedule) -> Result<BackupSchedule>;
    async fn delete_schedule(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn create_backup(&self, pool: &SqlitePool, backup: BackupRecord) -> Result<BackupRecord>;
    async fn get_backup(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<BackupRecord>>;
    async fn list_backups(&self, pool: &SqlitePool, limit: i32) -> Result<Vec<BackupRecord>>;
    async fn update_backup(&self, pool: &SqlitePool, backup: BackupRecord) -> Result<BackupRecord>;
    async fn delete_backup(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn create_restore(&self, pool: &SqlitePool, restore: RestoreOperation) -> Result<RestoreOperation>;
    async fn update_restore(&self, pool: &SqlitePool, restore: RestoreOperation) -> Result<RestoreOperation>;
    async fn list_restores(&self, pool: &SqlitePool, limit: i32) -> Result<Vec<RestoreOperation>>;
    async fn create_verification(&self, pool: &SqlitePool, verification: BackupVerification) -> Result<BackupVerification>;
    async fn get_latest_verification(&self, pool: &SqlitePool, backup_id: Uuid) -> Result<Option<BackupVerification>>;
    async fn get_storage_stats(&self, pool: &SqlitePool) -> Result<BackupStorageStats>;
}

pub struct SqliteBackupRepository;

#[async_trait]
impl BackupRepository for SqliteBackupRepository {
    async fn create_schedule(&self, pool: &SqlitePool, schedule: BackupSchedule) -> Result<BackupSchedule> {
        sqlx::query!(
            r#"INSERT INTO backup_schedules (id, name, backup_type, schedule_cron, retention_days,
               max_backups, compression, encryption_enabled, encryption_key_id, storage_type,
               storage_path, include_attachments, is_active, last_run, next_run, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            schedule.base.id.to_string(),
            schedule.name,
            format!("{:?}", schedule.backup_type),
            schedule.schedule_cron,
            schedule.retention_days,
            schedule.max_backups,
            schedule.compression as i32,
            schedule.encryption_enabled as i32,
            schedule.encryption_key_id.map(|id| id.to_string()),
            format!("{:?}", schedule.storage_type),
            schedule.storage_path,
            schedule.include_attachments as i32,
            schedule.is_active as i32,
            schedule.last_run.map(|d| d.to_rfc3339()),
            schedule.next_run.map(|d| d.to_rfc3339()),
            schedule.base.created_at.to_rfc3339(),
            schedule.base.updated_at.to_rfc3339(),
            schedule.base.created_by.map(|id| id.to_string()),
            schedule.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(schedule)
    }

    async fn get_schedule(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<BackupSchedule>> {
        let row = sqlx::query!(
            r#"SELECT id, name, backup_type, schedule_cron, retention_days, max_backups, compression,
               encryption_enabled, encryption_key_id, storage_type, storage_path, include_attachments,
               is_active, last_run, next_run, created_at, updated_at
               FROM backup_schedules WHERE id = ?"#,
            id.to_string()
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|r| BackupSchedule {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: r.name,
            backup_type: BackupType::Full,
            schedule_cron: r.schedule_cron,
            retention_days: r.retention_days,
            max_backups: r.max_backups,
            compression: r.compression == 1,
            encryption_enabled: r.encryption_enabled == 1,
            encryption_key_id: r.encryption_key_id.and_then(|id| Uuid::parse_str(&id).ok()),
            storage_type: BackupStorageType::Local,
            storage_path: r.storage_path,
            include_attachments: r.include_attachments == 1,
            is_active: r.is_active == 1,
            last_run: r.last_run.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            next_run: r.next_run.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
        }))
    }

    async fn list_schedules(&self, pool: &SqlitePool) -> Result<Vec<BackupSchedule>> {
        let rows = sqlx::query!(
            r#"SELECT id, name, backup_type, schedule_cron, retention_days, max_backups, compression,
               encryption_enabled, encryption_key_id, storage_type, storage_path, include_attachments,
               is_active, last_run, next_run, created_at, updated_at
               FROM backup_schedules ORDER BY name"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| BackupSchedule {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: r.name,
            backup_type: BackupType::Full,
            schedule_cron: r.schedule_cron,
            retention_days: r.retention_days,
            max_backups: r.max_backups,
            compression: r.compression == 1,
            encryption_enabled: r.encryption_enabled == 1,
            encryption_key_id: r.encryption_key_id.and_then(|id| Uuid::parse_str(&id).ok()),
            storage_type: BackupStorageType::Local,
            storage_path: r.storage_path,
            include_attachments: r.include_attachments == 1,
            is_active: r.is_active == 1,
            last_run: r.last_run.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            next_run: r.next_run.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
        }).collect())
    }

    async fn update_schedule(&self, pool: &SqlitePool, schedule: BackupSchedule) -> Result<BackupSchedule> {
        sqlx::query!(
            r#"UPDATE backup_schedules SET name = ?, backup_type = ?, schedule_cron = ?, retention_days = ?,
               max_backups = ?, compression = ?, encryption_enabled = ?, storage_type = ?, storage_path = ?,
               include_attachments = ?, is_active = ?, last_run = ?, next_run = ?, updated_at = ?, updated_by = ?
               WHERE id = ?"#,
            schedule.name,
            format!("{:?}", schedule.backup_type),
            schedule.schedule_cron,
            schedule.retention_days,
            schedule.max_backups,
            schedule.compression as i32,
            schedule.encryption_enabled as i32,
            format!("{:?}", schedule.storage_type),
            schedule.storage_path,
            schedule.include_attachments as i32,
            schedule.is_active as i32,
            schedule.last_run.map(|d| d.to_rfc3339()),
            schedule.next_run.map(|d| d.to_rfc3339()),
            schedule.base.updated_at.to_rfc3339(),
            schedule.base.updated_by.map(|id| id.to_string()),
            schedule.base.id.to_string(),
        ).execute(pool).await?;
        Ok(schedule)
    }

    async fn delete_schedule(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM backup_schedules WHERE id = ?", id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn create_backup(&self, pool: &SqlitePool, backup: BackupRecord) -> Result<BackupRecord> {
        sqlx::query!(
            r#"INSERT INTO backup_records (id, schedule_id, backup_type, status, file_path, file_size_bytes,
               compressed_size_bytes, checksum, checksum_algorithm, started_at, completed_at, duration_seconds,
               tables_included, records_count, error_message, verification_status, verified_at, is_restorable,
               created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            backup.base.id.to_string(),
            backup.schedule_id.map(|id| id.to_string()),
            format!("{:?}", backup.backup_type),
            format!("{:?}", backup.status),
            backup.file_path,
            backup.file_size_bytes,
            backup.compressed_size_bytes,
            backup.checksum,
            backup.checksum_algorithm,
            backup.started_at.to_rfc3339(),
            backup.completed_at.map(|d| d.to_rfc3339()),
            backup.duration_seconds,
            backup.tables_included,
            backup.records_count,
            backup.error_message,
            backup.verification_status.map(|s| format!("{:?}", s)),
            backup.verified_at.map(|d| d.to_rfc3339()),
            backup.is_restorable as i32,
            backup.base.created_at.to_rfc3339(),
            backup.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(backup)
    }

    async fn get_backup(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<BackupRecord>> {
        let row = sqlx::query!(
            r#"SELECT id, schedule_id, backup_type, status, file_path, file_size_bytes, compressed_size_bytes,
               checksum, checksum_algorithm, started_at, completed_at, duration_seconds, tables_included,
               records_count, error_message, verification_status, verified_at, is_restorable, created_at, updated_at
               FROM backup_records WHERE id = ?"#,
            id.to_string()
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|r| BackupRecord {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            schedule_id: r.schedule_id.and_then(|id| Uuid::parse_str(&id).ok()),
            backup_type: BackupType::Full,
            status: BackupStatus::Completed,
            file_path: r.file_path,
            file_size_bytes: r.file_size_bytes,
            compressed_size_bytes: r.compressed_size_bytes,
            checksum: r.checksum,
            checksum_algorithm: r.checksum_algorithm,
            started_at: chrono::DateTime::parse_from_rfc3339(&r.started_at).unwrap().with_timezone(&chrono::Utc),
            completed_at: r.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            duration_seconds: r.duration_seconds,
            tables_included: r.tables_included,
            records_count: r.records_count,
            error_message: r.error_message,
            verification_status: r.verification_status.map(|_| VerificationStatus::Pending),
            verified_at: r.verified_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            is_restorable: r.is_restorable == 1,
        }))
    }

    async fn list_backups(&self, pool: &SqlitePool, limit: i32) -> Result<Vec<BackupRecord>> {
        let rows = sqlx::query!(
            r#"SELECT id, schedule_id, backup_type, status, file_path, file_size_bytes, compressed_size_bytes,
               checksum, checksum_algorithm, started_at, completed_at, duration_seconds, tables_included,
               records_count, error_message, verification_status, verified_at, is_restorable, created_at, updated_at
               FROM backup_records ORDER BY started_at DESC LIMIT ?"#,
            limit
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| BackupRecord {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            schedule_id: r.schedule_id.and_then(|id| Uuid::parse_str(&id).ok()),
            backup_type: BackupType::Full,
            status: BackupStatus::Completed,
            file_path: r.file_path,
            file_size_bytes: r.file_size_bytes,
            compressed_size_bytes: r.compressed_size_bytes,
            checksum: r.checksum,
            checksum_algorithm: r.checksum_algorithm,
            started_at: chrono::DateTime::parse_from_rfc3339(&r.started_at).unwrap().with_timezone(&chrono::Utc),
            completed_at: r.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            duration_seconds: r.duration_seconds,
            tables_included: r.tables_included,
            records_count: r.records_count,
            error_message: r.error_message,
            verification_status: r.verification_status.map(|_| VerificationStatus::Pending),
            verified_at: r.verified_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            is_restorable: r.is_restorable == 1,
        }).collect())
    }

    async fn update_backup(&self, pool: &SqlitePool, backup: BackupRecord) -> Result<BackupRecord> {
        sqlx::query!(
            r#"UPDATE backup_records SET status = ?, completed_at = ?, duration_seconds = ?, records_count = ?,
               error_message = ?, verification_status = ?, verified_at = ?, is_restorable = ?, updated_at = ?
               WHERE id = ?"#,
            format!("{:?}", backup.status),
            backup.completed_at.map(|d| d.to_rfc3339()),
            backup.duration_seconds,
            backup.records_count,
            backup.error_message,
            backup.verification_status.map(|s| format!("{:?}", s)),
            backup.verified_at.map(|d| d.to_rfc3339()),
            backup.is_restorable as i32,
            backup.base.updated_at.to_rfc3339(),
            backup.base.id.to_string(),
        ).execute(pool).await?;
        Ok(backup)
    }

    async fn delete_backup(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM backup_records WHERE id = ?", id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn create_restore(&self, pool: &SqlitePool, restore: RestoreOperation) -> Result<RestoreOperation> {
        sqlx::query!(
            r#"INSERT INTO restore_operations (id, backup_id, status, restore_type, target_tables,
               started_at, completed_at, duration_seconds, records_restored, error_message, initiated_by,
               backup_before_restore, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            restore.base.id.to_string(),
            restore.backup_id.to_string(),
            format!("{:?}", restore.status),
            format!("{:?}", restore.restore_type),
            restore.target_tables,
            restore.started_at.to_rfc3339(),
            restore.completed_at.map(|d| d.to_rfc3339()),
            restore.duration_seconds,
            restore.records_restored,
            restore.error_message,
            restore.initiated_by.map(|id| id.to_string()),
            restore.backup_before_restore.map(|id| id.to_string()),
            restore.base.created_at.to_rfc3339(),
            restore.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(restore)
    }

    async fn update_restore(&self, pool: &SqlitePool, restore: RestoreOperation) -> Result<RestoreOperation> {
        sqlx::query!(
            r#"UPDATE restore_operations SET status = ?, completed_at = ?, duration_seconds = ?,
               records_restored = ?, error_message = ?, updated_at = ? WHERE id = ?"#,
            format!("{:?}", restore.status),
            restore.completed_at.map(|d| d.to_rfc3339()),
            restore.duration_seconds,
            restore.records_restored,
            restore.error_message,
            restore.base.updated_at.to_rfc3339(),
            restore.base.id.to_string(),
        ).execute(pool).await?;
        Ok(restore)
    }

    async fn list_restores(&self, pool: &SqlitePool, limit: i32) -> Result<Vec<RestoreOperation>> {
        let rows = sqlx::query!(
            r#"SELECT id, backup_id, status, restore_type, target_tables, started_at, completed_at,
               duration_seconds, records_restored, error_message, initiated_by, backup_before_restore, created_at, updated_at
               FROM restore_operations ORDER BY started_at DESC LIMIT ?"#,
            limit
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| RestoreOperation {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            backup_id: Uuid::parse_str(&r.backup_id).unwrap(),
            status: RestoreStatus::Completed,
            restore_type: RestoreType::Full,
            target_tables: r.target_tables,
            started_at: chrono::DateTime::parse_from_rfc3339(&r.started_at).unwrap().with_timezone(&chrono::Utc),
            completed_at: r.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            duration_seconds: r.duration_seconds,
            records_restored: r.records_restored,
            error_message: r.error_message,
            initiated_by: r.initiated_by.and_then(|id| Uuid::parse_str(&id).ok()),
            backup_before_restore: r.backup_before_restore.and_then(|id| Uuid::parse_str(&id).ok()),
        }).collect())
    }

    async fn create_verification(&self, pool: &SqlitePool, verification: BackupVerification) -> Result<BackupVerification> {
        sqlx::query!(
            r#"INSERT INTO backup_verifications (id, backup_id, status, checked_at, checksum_valid,
               file_readable, schema_valid, sample_data_valid, error_details, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            verification.base.id.to_string(),
            verification.backup_id.to_string(),
            format!("{:?}", verification.status),
            verification.checked_at.to_rfc3339(),
            verification.checksum_valid as i32,
            verification.file_readable as i32,
            verification.schema_valid as i32,
            verification.sample_data_valid as i32,
            verification.error_details,
            verification.base.created_at.to_rfc3339(),
            verification.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(verification)
    }

    async fn get_latest_verification(&self, pool: &SqlitePool, backup_id: Uuid) -> Result<Option<BackupVerification>> {
        let row = sqlx::query!(
            r#"SELECT id, backup_id, status, checked_at, checksum_valid, file_readable, schema_valid,
               sample_data_valid, error_details, created_at, updated_at
               FROM backup_verifications WHERE backup_id = ? ORDER BY checked_at DESC LIMIT 1"#,
            backup_id.to_string()
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|r| BackupVerification {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            backup_id: Uuid::parse_str(&r.backup_id).unwrap(),
            status: VerificationStatus::Verified,
            checked_at: chrono::DateTime::parse_from_rfc3339(&r.checked_at).unwrap().with_timezone(&chrono::Utc),
            checksum_valid: r.checksum_valid == 1,
            file_readable: r.file_readable == 1,
            schema_valid: r.schema_valid == 1,
            sample_data_valid: r.sample_data_valid == 1,
            error_details: r.error_details,
        }))
    }

    async fn get_storage_stats(&self, pool: &SqlitePool) -> Result<BackupStorageStats> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM backup_records")
            .fetch_one(pool).await?.unwrap_or(0);
        
        let total_size: i64 = sqlx::query_scalar("SELECT COALESCE(SUM(file_size_bytes), 0) FROM backup_records")
            .fetch_one(pool).await?.unwrap_or(0);
        
        Ok(BackupStorageStats {
            base: BaseEntity::new(),
            storage_type: BackupStorageType::Local,
            total_size_bytes: total_size,
            backup_count: count as i32,
            oldest_backup: None,
            newest_backup: None,
            available_space_bytes: None,
            calculated_at: chrono::Utc::now(),
        })
    }
}

use erp_core::BaseEntity;
