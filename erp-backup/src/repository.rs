use async_trait::async_trait;
use sqlx::{Row, SqlitePool};
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
        let id = schedule.base.id.to_string();
        let backup_type = format!("{:?}", schedule.backup_type);
        let encryption_key_id = schedule.encryption_key_id.map(|id| id.to_string());
        let storage_type = format!("{:?}", schedule.storage_type);
        let last_run = schedule.last_run.map(|d| d.to_rfc3339());
        let next_run = schedule.next_run.map(|d| d.to_rfc3339());
        let created_at = schedule.base.created_at.to_rfc3339();
        let updated_at = schedule.base.updated_at.to_rfc3339();
        let created_by = schedule.base.created_by.map(|id| id.to_string());
        let updated_by = schedule.base.updated_by.map(|id| id.to_string());
        sqlx::query(
            r#"INSERT INTO backup_schedules (id, name, backup_type, schedule_cron, retention_days,
               max_backups, compression, encryption_enabled, encryption_key_id, storage_type,
               storage_path, include_attachments, is_active, last_run, next_run, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(&schedule.name)
        .bind(&backup_type)
        .bind(&schedule.schedule_cron)
        .bind(schedule.retention_days)
        .bind(schedule.max_backups)
        .bind(schedule.compression as i32)
        .bind(schedule.encryption_enabled as i32)
        .bind(&encryption_key_id)
        .bind(&storage_type)
        .bind(&schedule.storage_path)
        .bind(schedule.include_attachments as i32)
        .bind(schedule.is_active as i32)
        .bind(&last_run)
        .bind(&next_run)
        .bind(&created_at)
        .bind(&updated_at)
        .bind(&created_by)
        .bind(&updated_by)
        .execute(pool).await?;
        Ok(schedule)
    }

    async fn get_schedule(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<BackupSchedule>> {
        let row = sqlx::query(
            r#"SELECT id, name, backup_type, schedule_cron, retention_days, max_backups, compression,
               encryption_enabled, encryption_key_id, storage_type, storage_path, include_attachments,
               is_active, last_run, next_run, created_at, updated_at
               FROM backup_schedules WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(pool).await?;
        
        Ok(row.map(|r| BackupSchedule {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.try_get::<String, _>("id").unwrap().as_str()).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.try_get::<String, _>("created_at").unwrap().as_str()).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.try_get::<String, _>("updated_at").unwrap().as_str()).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: r.try_get("name").unwrap(),
            backup_type: BackupType::Full,
            schedule_cron: r.try_get("schedule_cron").unwrap(),
            retention_days: r.try_get("retention_days").unwrap(),
            max_backups: r.try_get("max_backups").unwrap(),
            compression: r.try_get::<i32, _>("compression").unwrap() == 1,
            encryption_enabled: r.try_get::<i32, _>("encryption_enabled").unwrap() == 1,
            encryption_key_id: r.try_get::<Option<String>, _>("encryption_key_id").unwrap().as_deref().and_then(|id| Uuid::parse_str(id).ok()),
            storage_type: BackupStorageType::Local,
            storage_path: r.try_get("storage_path").unwrap(),
            include_attachments: r.try_get::<i32, _>("include_attachments").unwrap() == 1,
            is_active: r.try_get::<i32, _>("is_active").unwrap() == 1,
            last_run: r.try_get::<Option<String>, _>("last_run").unwrap().as_deref().and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            next_run: r.try_get::<Option<String>, _>("next_run").unwrap().as_deref().and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
        }))
    }

    async fn list_schedules(&self, pool: &SqlitePool) -> Result<Vec<BackupSchedule>> {
        let rows = sqlx::query(
            r#"SELECT id, name, backup_type, schedule_cron, retention_days, max_backups, compression,
               encryption_enabled, encryption_key_id, storage_type, storage_path, include_attachments,
               is_active, last_run, next_run, created_at, updated_at
               FROM backup_schedules ORDER BY name"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| BackupSchedule {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.try_get::<String, _>("id").unwrap().as_str()).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.try_get::<String, _>("created_at").unwrap().as_str()).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.try_get::<String, _>("updated_at").unwrap().as_str()).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: r.try_get("name").unwrap(),
            backup_type: BackupType::Full,
            schedule_cron: r.try_get("schedule_cron").unwrap(),
            retention_days: r.try_get("retention_days").unwrap(),
            max_backups: r.try_get("max_backups").unwrap(),
            compression: r.try_get::<i32, _>("compression").unwrap() == 1,
            encryption_enabled: r.try_get::<i32, _>("encryption_enabled").unwrap() == 1,
            encryption_key_id: r.try_get::<Option<String>, _>("encryption_key_id").unwrap().as_deref().and_then(|id| Uuid::parse_str(id).ok()),
            storage_type: BackupStorageType::Local,
            storage_path: r.try_get("storage_path").unwrap(),
            include_attachments: r.try_get::<i32, _>("include_attachments").unwrap() == 1,
            is_active: r.try_get::<i32, _>("is_active").unwrap() == 1,
            last_run: r.try_get::<Option<String>, _>("last_run").unwrap().as_deref().and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            next_run: r.try_get::<Option<String>, _>("next_run").unwrap().as_deref().and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
        }).collect())
    }

    async fn update_schedule(&self, pool: &SqlitePool, schedule: BackupSchedule) -> Result<BackupSchedule> {
        let backup_type = format!("{:?}", schedule.backup_type);
        let storage_type = format!("{:?}", schedule.storage_type);
        let last_run = schedule.last_run.map(|d| d.to_rfc3339());
        let next_run = schedule.next_run.map(|d| d.to_rfc3339());
        let updated_at = schedule.base.updated_at.to_rfc3339();
        let updated_by = schedule.base.updated_by.map(|id| id.to_string());
        let id = schedule.base.id.to_string();
        sqlx::query(
            r#"UPDATE backup_schedules SET name = ?, backup_type = ?, schedule_cron = ?, retention_days = ?,
               max_backups = ?, compression = ?, encryption_enabled = ?, storage_type = ?, storage_path = ?,
               include_attachments = ?, is_active = ?, last_run = ?, next_run = ?, updated_at = ?, updated_by = ?
               WHERE id = ?"#,
        )
        .bind(&schedule.name)
        .bind(&backup_type)
        .bind(&schedule.schedule_cron)
        .bind(schedule.retention_days)
        .bind(schedule.max_backups)
        .bind(schedule.compression as i32)
        .bind(schedule.encryption_enabled as i32)
        .bind(&storage_type)
        .bind(&schedule.storage_path)
        .bind(schedule.include_attachments as i32)
        .bind(schedule.is_active as i32)
        .bind(&last_run)
        .bind(&next_run)
        .bind(&updated_at)
        .bind(&updated_by)
        .bind(&id)
        .execute(pool).await?;
        Ok(schedule)
    }

    async fn delete_schedule(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let id_str = id.to_string();
        sqlx::query("DELETE FROM backup_schedules WHERE id = ?")
            .bind(&id_str)
            .execute(pool).await?;
        Ok(())
    }

    async fn create_backup(&self, pool: &SqlitePool, backup: BackupRecord) -> Result<BackupRecord> {
        let id = backup.base.id.to_string();
        let schedule_id = backup.schedule_id.map(|id| id.to_string());
        let backup_type = format!("{:?}", backup.backup_type);
        let status = format!("{:?}", backup.status);
        let started_at = backup.started_at.to_rfc3339();
        let completed_at = backup.completed_at.map(|d| d.to_rfc3339());
        let verification_status = backup.verification_status.as_ref().map(|s| format!("{:?}", s));
        let verified_at = backup.verified_at.map(|d| d.to_rfc3339());
        let created_at = backup.base.created_at.to_rfc3339();
        let updated_at = backup.base.updated_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO backup_records (id, schedule_id, backup_type, status, file_path, file_size_bytes,
               compressed_size_bytes, checksum, checksum_algorithm, started_at, completed_at, duration_seconds,
               tables_included, records_count, error_message, verification_status, verified_at, is_restorable,
               created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(&schedule_id)
        .bind(&backup_type)
        .bind(&status)
        .bind(&backup.file_path)
        .bind(backup.file_size_bytes)
        .bind(backup.compressed_size_bytes)
        .bind(&backup.checksum)
        .bind(&backup.checksum_algorithm)
        .bind(&started_at)
        .bind(&completed_at)
        .bind(backup.duration_seconds)
        .bind(&backup.tables_included)
        .bind(backup.records_count)
        .bind(&backup.error_message)
        .bind(&verification_status)
        .bind(&verified_at)
        .bind(backup.is_restorable as i32)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(pool).await?;
        Ok(backup)
    }

    async fn get_backup(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<BackupRecord>> {
        let row = sqlx::query(
            r#"SELECT id, schedule_id, backup_type, status, file_path, file_size_bytes, compressed_size_bytes,
               checksum, checksum_algorithm, started_at, completed_at, duration_seconds, tables_included,
               records_count, error_message, verification_status, verified_at, is_restorable, created_at, updated_at
               FROM backup_records WHERE id = ?"#,
        )
        .bind(id.to_string())
        .fetch_optional(pool).await?;
        
        Ok(row.map(|r| BackupRecord {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.try_get::<String, _>("id").unwrap().as_str()).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.try_get::<String, _>("created_at").unwrap().as_str()).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.try_get::<String, _>("updated_at").unwrap().as_str()).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            schedule_id: r.try_get::<Option<String>, _>("schedule_id").unwrap().as_deref().and_then(|id| Uuid::parse_str(id).ok()),
            backup_type: BackupType::Full,
            status: BackupStatus::Completed,
            file_path: r.try_get("file_path").unwrap(),
            file_size_bytes: r.try_get("file_size_bytes").unwrap(),
            compressed_size_bytes: r.try_get("compressed_size_bytes").unwrap(),
            checksum: r.try_get("checksum").unwrap(),
            checksum_algorithm: r.try_get("checksum_algorithm").unwrap(),
            started_at: chrono::DateTime::parse_from_rfc3339(r.try_get::<String, _>("started_at").unwrap().as_str()).unwrap().with_timezone(&chrono::Utc),
            completed_at: r.try_get::<Option<String>, _>("completed_at").unwrap().as_deref().and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            duration_seconds: r.try_get("duration_seconds").unwrap(),
            tables_included: r.try_get("tables_included").unwrap(),
            records_count: r.try_get("records_count").unwrap(),
            error_message: r.try_get("error_message").unwrap(),
            verification_status: r.try_get::<Option<String>, _>("verification_status").unwrap().as_ref().map(|_| VerificationStatus::Pending),
            verified_at: r.try_get::<Option<String>, _>("verified_at").unwrap().as_deref().and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            is_restorable: r.try_get::<i32, _>("is_restorable").unwrap() == 1,
        }))
    }

    async fn list_backups(&self, pool: &SqlitePool, limit: i32) -> Result<Vec<BackupRecord>> {
        let rows = sqlx::query(
            r#"SELECT id, schedule_id, backup_type, status, file_path, file_size_bytes, compressed_size_bytes,
               checksum, checksum_algorithm, started_at, completed_at, duration_seconds, tables_included,
               records_count, error_message, verification_status, verified_at, is_restorable, created_at, updated_at
               FROM backup_records ORDER BY started_at DESC LIMIT ?"#,
        )
        .bind(limit)
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| BackupRecord {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.try_get::<String, _>("id").unwrap().as_str()).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.try_get::<String, _>("created_at").unwrap().as_str()).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.try_get::<String, _>("updated_at").unwrap().as_str()).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            schedule_id: r.try_get::<Option<String>, _>("schedule_id").unwrap().as_deref().and_then(|id| Uuid::parse_str(id).ok()),
            backup_type: BackupType::Full,
            status: BackupStatus::Completed,
            file_path: r.try_get("file_path").unwrap(),
            file_size_bytes: r.try_get("file_size_bytes").unwrap(),
            compressed_size_bytes: r.try_get("compressed_size_bytes").unwrap(),
            checksum: r.try_get("checksum").unwrap(),
            checksum_algorithm: r.try_get("checksum_algorithm").unwrap(),
            started_at: chrono::DateTime::parse_from_rfc3339(r.try_get::<String, _>("started_at").unwrap().as_str()).unwrap().with_timezone(&chrono::Utc),
            completed_at: r.try_get::<Option<String>, _>("completed_at").unwrap().as_deref().and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            duration_seconds: r.try_get("duration_seconds").unwrap(),
            tables_included: r.try_get("tables_included").unwrap(),
            records_count: r.try_get("records_count").unwrap(),
            error_message: r.try_get("error_message").unwrap(),
            verification_status: r.try_get::<Option<String>, _>("verification_status").unwrap().as_ref().map(|_| VerificationStatus::Pending),
            verified_at: r.try_get::<Option<String>, _>("verified_at").unwrap().as_deref().and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            is_restorable: r.try_get::<i32, _>("is_restorable").unwrap() == 1,
        }).collect())
    }

    async fn update_backup(&self, pool: &SqlitePool, backup: BackupRecord) -> Result<BackupRecord> {
        let status = format!("{:?}", backup.status);
        let completed_at = backup.completed_at.map(|d| d.to_rfc3339());
        let verification_status = backup.verification_status.as_ref().map(|s| format!("{:?}", s));
        let verified_at = backup.verified_at.map(|d| d.to_rfc3339());
        let updated_at = backup.base.updated_at.to_rfc3339();
        let id = backup.base.id.to_string();
        sqlx::query(
            r#"UPDATE backup_records SET status = ?, completed_at = ?, duration_seconds = ?, records_count = ?,
               error_message = ?, verification_status = ?, verified_at = ?, is_restorable = ?, updated_at = ?
               WHERE id = ?"#,
        )
        .bind(&status)
        .bind(&completed_at)
        .bind(backup.duration_seconds)
        .bind(backup.records_count)
        .bind(&backup.error_message)
        .bind(&verification_status)
        .bind(&verified_at)
        .bind(backup.is_restorable as i32)
        .bind(&updated_at)
        .bind(&id)
        .execute(pool).await?;
        Ok(backup)
    }

    async fn delete_backup(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let id_str = id.to_string();
        sqlx::query("DELETE FROM backup_records WHERE id = ?")
            .bind(&id_str)
            .execute(pool).await?;
        Ok(())
    }

    async fn create_restore(&self, pool: &SqlitePool, restore: RestoreOperation) -> Result<RestoreOperation> {
        let id = restore.base.id.to_string();
        let backup_id = restore.backup_id.to_string();
        let status = format!("{:?}", restore.status);
        let restore_type = format!("{:?}", restore.restore_type);
        let started_at = restore.started_at.to_rfc3339();
        let completed_at = restore.completed_at.map(|d| d.to_rfc3339());
        let initiated_by = restore.initiated_by.map(|id| id.to_string());
        let backup_before_restore = restore.backup_before_restore.map(|id| id.to_string());
        let created_at = restore.base.created_at.to_rfc3339();
        let updated_at = restore.base.updated_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO restore_operations (id, backup_id, status, restore_type, target_tables,
               started_at, completed_at, duration_seconds, records_restored, error_message, initiated_by,
               backup_before_restore, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(&backup_id)
        .bind(&status)
        .bind(&restore_type)
        .bind(&restore.target_tables)
        .bind(&started_at)
        .bind(&completed_at)
        .bind(restore.duration_seconds)
        .bind(restore.records_restored)
        .bind(&restore.error_message)
        .bind(&initiated_by)
        .bind(&backup_before_restore)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(pool).await?;
        Ok(restore)
    }

    async fn update_restore(&self, pool: &SqlitePool, restore: RestoreOperation) -> Result<RestoreOperation> {
        let status = format!("{:?}", restore.status);
        let completed_at = restore.completed_at.map(|d| d.to_rfc3339());
        let updated_at = restore.base.updated_at.to_rfc3339();
        let id = restore.base.id.to_string();
        sqlx::query(
            r#"UPDATE restore_operations SET status = ?, completed_at = ?, duration_seconds = ?,
               records_restored = ?, error_message = ?, updated_at = ? WHERE id = ?"#,
        )
        .bind(&status)
        .bind(&completed_at)
        .bind(restore.duration_seconds)
        .bind(restore.records_restored)
        .bind(&restore.error_message)
        .bind(&updated_at)
        .bind(&id)
        .execute(pool).await?;
        Ok(restore)
    }

    async fn list_restores(&self, pool: &SqlitePool, limit: i32) -> Result<Vec<RestoreOperation>> {
        let rows = sqlx::query(
            r#"SELECT id, backup_id, status, restore_type, target_tables, started_at, completed_at,
               duration_seconds, records_restored, error_message, initiated_by, backup_before_restore, created_at, updated_at
               FROM restore_operations ORDER BY started_at DESC LIMIT ?"#,
        )
        .bind(limit)
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| RestoreOperation {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.try_get::<String, _>("id").unwrap().as_str()).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.try_get::<String, _>("created_at").unwrap().as_str()).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.try_get::<String, _>("updated_at").unwrap().as_str()).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            backup_id: Uuid::parse_str(r.try_get::<String, _>("backup_id").unwrap().as_str()).unwrap(),
            status: RestoreStatus::Completed,
            restore_type: RestoreType::Full,
            target_tables: r.try_get("target_tables").unwrap(),
            started_at: chrono::DateTime::parse_from_rfc3339(r.try_get::<String, _>("started_at").unwrap().as_str()).unwrap().with_timezone(&chrono::Utc),
            completed_at: r.try_get::<Option<String>, _>("completed_at").unwrap().as_deref().and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            duration_seconds: r.try_get("duration_seconds").unwrap(),
            records_restored: r.try_get("records_restored").unwrap(),
            error_message: r.try_get("error_message").unwrap(),
            initiated_by: r.try_get::<Option<String>, _>("initiated_by").unwrap().as_deref().and_then(|id| Uuid::parse_str(id).ok()),
            backup_before_restore: r.try_get::<Option<String>, _>("backup_before_restore").unwrap().as_deref().and_then(|id| Uuid::parse_str(id).ok()),
        }).collect())
    }

    async fn create_verification(&self, pool: &SqlitePool, verification: BackupVerification) -> Result<BackupVerification> {
        let id = verification.base.id.to_string();
        let backup_id = verification.backup_id.to_string();
        let status = format!("{:?}", verification.status);
        let checked_at = verification.checked_at.to_rfc3339();
        let created_at = verification.base.created_at.to_rfc3339();
        let updated_at = verification.base.updated_at.to_rfc3339();
        sqlx::query(
            r#"INSERT INTO backup_verifications (id, backup_id, status, checked_at, checksum_valid,
               file_readable, schema_valid, sample_data_valid, error_details, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(&id)
        .bind(&backup_id)
        .bind(&status)
        .bind(&checked_at)
        .bind(verification.checksum_valid as i32)
        .bind(verification.file_readable as i32)
        .bind(verification.schema_valid as i32)
        .bind(verification.sample_data_valid as i32)
        .bind(&verification.error_details)
        .bind(&created_at)
        .bind(&updated_at)
        .execute(pool).await?;
        Ok(verification)
    }

    async fn get_latest_verification(&self, pool: &SqlitePool, backup_id: Uuid) -> Result<Option<BackupVerification>> {
        let backup_id_str = backup_id.to_string();
        let row = sqlx::query(
            r#"SELECT id, backup_id, status, checked_at, checksum_valid, file_readable, schema_valid,
               sample_data_valid, error_details, created_at, updated_at
               FROM backup_verifications WHERE backup_id = ? ORDER BY checked_at DESC LIMIT 1"#,
        )
        .bind(&backup_id_str)
        .fetch_optional(pool).await?;
        
        Ok(row.map(|r| BackupVerification {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.try_get::<String, _>("id").unwrap().as_str()).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.try_get::<String, _>("created_at").unwrap().as_str()).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.try_get::<String, _>("updated_at").unwrap().as_str()).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            backup_id: Uuid::parse_str(r.try_get::<String, _>("backup_id").unwrap().as_str()).unwrap(),
            status: VerificationStatus::Verified,
            checked_at: chrono::DateTime::parse_from_rfc3339(r.try_get::<String, _>("checked_at").unwrap().as_str()).unwrap().with_timezone(&chrono::Utc),
            checksum_valid: r.try_get::<i32, _>("checksum_valid").unwrap() == 1,
            file_readable: r.try_get::<i32, _>("file_readable").unwrap() == 1,
            schema_valid: r.try_get::<i32, _>("schema_valid").unwrap() == 1,
            sample_data_valid: r.try_get::<i32, _>("sample_data_valid").unwrap() == 1,
            error_details: r.try_get("error_details").unwrap(),
        }))
    }

    async fn get_storage_stats(&self, pool: &SqlitePool) -> Result<BackupStorageStats> {
        let count: i64 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM backup_records")
            .fetch_one(pool).await?;
        
        let total_size: i64 = sqlx::query_scalar::<_, i64>("SELECT COALESCE(SUM(file_size_bytes), 0) FROM backup_records")
            .fetch_one(pool).await?;
        
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
