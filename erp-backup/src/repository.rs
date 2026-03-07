use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::Result;
use crate::models::*;

#[async_trait]
pub trait BackupRepository: Send + Sync {
    async fn create_schedule(&self, schedule: BackupSchedule) -> Result<BackupSchedule>;
    async fn get_schedule(&self, id: Uuid) -> Result<Option<BackupSchedule>>;
    async fn list_schedules(&self) -> Result<Vec<BackupSchedule>>;
    async fn update_schedule(&self, schedule: BackupSchedule) -> Result<BackupSchedule>;
    async fn delete_schedule(&self, id: Uuid) -> Result<()>;
    async fn create_backup(&self, backup: BackupRecord) -> Result<BackupRecord>;
    async fn get_backup(&self, id: Uuid) -> Result<Option<BackupRecord>>;
    async fn list_backups(&self, limit: i32) -> Result<Vec<BackupRecord>>;
    async fn update_backup(&self, backup: BackupRecord) -> Result<BackupRecord>;
    async fn delete_backup(&self, id: Uuid) -> Result<()>;
    async fn create_restore(&self, restore: RestoreOperation) -> Result<RestoreOperation>;
    async fn update_restore(&self, restore: RestoreOperation) -> Result<RestoreOperation>;
    async fn list_restores(&self, limit: i32) -> Result<Vec<RestoreOperation>>;
    async fn create_verification(&self, verification: BackupVerification) -> Result<BackupVerification>;
    async fn get_latest_verification(&self, backup_id: Uuid) -> Result<Option<BackupVerification>>;
    async fn get_storage_stats(&self) -> Result<BackupStorageStats>;
}

pub struct SqliteBackupRepository {
    pool: SqlitePool,
}

impl SqliteBackupRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl BackupRepository for SqliteBackupRepository {
    async fn create_schedule(&self, schedule: BackupSchedule) -> Result<BackupSchedule> {
        sqlx::query(
            r#"INSERT INTO backup_schedules (id, name, backup_type, schedule_cron, retention_days,
               max_backups, compression, encryption_enabled, encryption_key_id, storage_type,
               storage_path, include_attachments, is_active, last_run, next_run, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(schedule.base.id)
        .bind(&schedule.name)
        .bind(&schedule.backup_type)
        .bind(&schedule.schedule_cron)
        .bind(schedule.retention_days)
        .bind(schedule.max_backups)
        .bind(schedule.compression)
        .bind(schedule.encryption_enabled)
        .bind(schedule.encryption_key_id)
        .bind(&schedule.storage_type)
        .bind(&schedule.storage_path)
        .bind(schedule.include_attachments)
        .bind(schedule.is_active)
        .bind(schedule.last_run)
        .bind(schedule.next_run)
        .bind(schedule.base.created_at)
        .bind(schedule.base.updated_at)
        .bind(schedule.base.created_by)
        .bind(schedule.base.updated_by)
        .execute(&self.pool).await?;
        Ok(schedule)
    }

    async fn get_schedule(&self, id: Uuid) -> Result<Option<BackupSchedule>> {
        sqlx::query_as::<_, BackupSchedule>(
            r#"SELECT id, name, backup_type, schedule_cron, retention_days, max_backups, compression,
               encryption_enabled, encryption_key_id, storage_type, storage_path, include_attachments,
               is_active, last_run, next_run, created_at, updated_at, created_by, updated_by
               FROM backup_schedules WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool).await
        .map_err(Into::into)
    }

    async fn list_schedules(&self) -> Result<Vec<BackupSchedule>> {
        sqlx::query_as::<_, BackupSchedule>(
            r#"SELECT id, name, backup_type, schedule_cron, retention_days, max_backups, compression,
               encryption_enabled, encryption_key_id, storage_type, storage_path, include_attachments,
               is_active, last_run, next_run, created_at, updated_at, created_by, updated_by
               FROM backup_schedules ORDER BY name"#
        )
        .fetch_all(&self.pool).await
        .map_err(Into::into)
    }

    async fn update_schedule(&self, schedule: BackupSchedule) -> Result<BackupSchedule> {
        sqlx::query(
            r#"UPDATE backup_schedules SET name = ?, backup_type = ?, schedule_cron = ?, retention_days = ?,
               max_backups = ?, compression = ?, encryption_enabled = ?, storage_type = ?, storage_path = ?,
               include_attachments = ?, is_active = ?, last_run = ?, next_run = ?, updated_at = ?, updated_by = ?
               WHERE id = ?"#,
        )
        .bind(&schedule.name)
        .bind(&schedule.backup_type)
        .bind(&schedule.schedule_cron)
        .bind(schedule.retention_days)
        .bind(schedule.max_backups)
        .bind(schedule.compression)
        .bind(schedule.encryption_enabled)
        .bind(&schedule.storage_type)
        .bind(&schedule.storage_path)
        .bind(schedule.include_attachments)
        .bind(schedule.is_active)
        .bind(schedule.last_run)
        .bind(schedule.next_run)
        .bind(schedule.base.updated_at)
        .bind(schedule.base.updated_by)
        .bind(schedule.base.id)
        .execute(&self.pool).await?;
        Ok(schedule)
    }

    async fn delete_schedule(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM backup_schedules WHERE id = ?")
            .bind(id)
            .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_backup(&self, backup: BackupRecord) -> Result<BackupRecord> {
        sqlx::query(
            r#"INSERT INTO backup_records (id, schedule_id, backup_type, status, file_path, file_size_bytes,
               compressed_size_bytes, checksum, checksum_algorithm, started_at, completed_at, duration_seconds,
               tables_included, records_count, error_message, verification_status, verified_at, is_restorable,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(backup.base.id)
        .bind(backup.schedule_id)
        .bind(&backup.backup_type)
        .bind(&backup.status)
        .bind(&backup.file_path)
        .bind(backup.file_size_bytes)
        .bind(backup.compressed_size_bytes)
        .bind(&backup.checksum)
        .bind(&backup.checksum_algorithm)
        .bind(backup.started_at)
        .bind(backup.completed_at)
        .bind(backup.duration_seconds)
        .bind(&backup.tables_included)
        .bind(backup.records_count)
        .bind(&backup.error_message)
        .bind(&backup.verification_status)
        .bind(backup.verified_at)
        .bind(backup.is_restorable)
        .bind(backup.base.created_at)
        .bind(backup.base.updated_at)
        .bind(backup.base.created_by)
        .bind(backup.base.updated_by)
        .execute(&self.pool).await?;
        Ok(backup)
    }

    async fn get_backup(&self, id: Uuid) -> Result<Option<BackupRecord>> {
        sqlx::query_as::<_, BackupRecord>(
            r#"SELECT id, schedule_id, backup_type, status, file_path, file_size_bytes, compressed_size_bytes,
               checksum, checksum_algorithm, started_at, completed_at, duration_seconds, tables_included,
               records_count, error_message, verification_status, verified_at, is_restorable, created_at, updated_at, created_by, updated_by
               FROM backup_records WHERE id = ?"#,
        )
        .bind(id)
        .fetch_optional(&self.pool).await
        .map_err(Into::into)
    }

    async fn list_backups(&self, limit: i32) -> Result<Vec<BackupRecord>> {
        sqlx::query_as::<_, BackupRecord>(
            r#"SELECT id, schedule_id, backup_type, status, file_path, file_size_bytes, compressed_size_bytes,
               checksum, checksum_algorithm, started_at, completed_at, duration_seconds, tables_included,
               records_count, error_message, verification_status, verified_at, is_restorable, created_at, updated_at, created_by, updated_by
               FROM backup_records ORDER BY started_at DESC LIMIT ?"#,
        )
        .bind(limit)
        .fetch_all(&self.pool).await
        .map_err(Into::into)
    }

    async fn update_backup(&self, backup: BackupRecord) -> Result<BackupRecord> {
        sqlx::query(
            r#"UPDATE backup_records SET status = ?, completed_at = ?, duration_seconds = ?, records_count = ?,
               error_message = ?, verification_status = ?, verified_at = ?, is_restorable = ?, updated_at = ?, updated_by = ?
               WHERE id = ?"#,
        )
        .bind(&backup.status)
        .bind(backup.completed_at)
        .bind(backup.duration_seconds)
        .bind(backup.records_count)
        .bind(&backup.error_message)
        .bind(&backup.verification_status)
        .bind(backup.verified_at)
        .bind(backup.is_restorable)
        .bind(backup.base.updated_at)
        .bind(backup.base.updated_by)
        .bind(backup.base.id)
        .execute(&self.pool).await?;
        Ok(backup)
    }

    async fn delete_backup(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM backup_records WHERE id = ?")
            .bind(id)
            .execute(&self.pool).await?;
        Ok(())
    }

    async fn create_restore(&self, restore: RestoreOperation) -> Result<RestoreOperation> {
        sqlx::query(
            r#"INSERT INTO restore_operations (id, backup_id, status, restore_type, target_tables,
               started_at, completed_at, duration_seconds, records_restored, error_message, initiated_by,
               backup_before_restore, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(restore.base.id)
        .bind(restore.backup_id)
        .bind(&restore.status)
        .bind(&restore.restore_type)
        .bind(&restore.target_tables)
        .bind(restore.started_at)
        .bind(restore.completed_at)
        .bind(restore.duration_seconds)
        .bind(restore.records_restored)
        .bind(&restore.error_message)
        .bind(restore.initiated_by)
        .bind(restore.backup_before_restore)
        .bind(restore.base.created_at)
        .bind(restore.base.updated_at)
        .bind(restore.base.created_by)
        .bind(restore.base.updated_by)
        .execute(&self.pool).await?;
        Ok(restore)
    }

    async fn update_restore(&self, restore: RestoreOperation) -> Result<RestoreOperation> {
        sqlx::query(
            r#"UPDATE restore_operations SET status = ?, completed_at = ?, duration_seconds = ?,
               records_restored = ?, error_message = ?, updated_at = ?, updated_by = ? WHERE id = ?"#,
        )
        .bind(&restore.status)
        .bind(restore.completed_at)
        .bind(restore.duration_seconds)
        .bind(restore.records_restored)
        .bind(&restore.error_message)
        .bind(restore.base.updated_at)
        .bind(restore.base.updated_by)
        .bind(restore.base.id)
        .execute(&self.pool).await?;
        Ok(restore)
    }

    async fn list_restores(&self, limit: i32) -> Result<Vec<RestoreOperation>> {
        sqlx::query_as::<_, RestoreOperation>(
            r#"SELECT id, backup_id, status, restore_type, target_tables, started_at, completed_at,
               duration_seconds, records_restored, error_message, initiated_by, backup_before_restore, 
               created_at, updated_at, created_by, updated_by
               FROM restore_operations ORDER BY started_at DESC LIMIT ?"#,
        )
        .bind(limit)
        .fetch_all(&self.pool).await
        .map_err(Into::into)
    }

    async fn create_verification(&self, verification: BackupVerification) -> Result<BackupVerification> {
        sqlx::query(
            r#"INSERT INTO backup_verifications (id, backup_id, status, checked_at, checksum_valid,
               file_readable, schema_valid, sample_data_valid, error_details, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
        )
        .bind(verification.base.id)
        .bind(verification.backup_id)
        .bind(&verification.status)
        .bind(verification.checked_at)
        .bind(verification.checksum_valid)
        .bind(verification.file_readable)
        .bind(verification.schema_valid)
        .bind(verification.sample_data_valid)
        .bind(&verification.error_details)
        .bind(verification.base.created_at)
        .bind(verification.base.updated_at)
        .bind(verification.base.created_by)
        .bind(verification.base.updated_by)
        .execute(&self.pool).await?;
        Ok(verification)
    }

    async fn get_latest_verification(&self, backup_id: Uuid) -> Result<Option<BackupVerification>> {
        sqlx::query_as::<_, BackupVerification>(
            r#"SELECT id, backup_id, status, checked_at, checksum_valid, file_readable, schema_valid,
               sample_data_valid, error_details, created_at, updated_at, created_by, updated_by
               FROM backup_verifications WHERE backup_id = ? ORDER BY checked_at DESC LIMIT 1"#,
        )
        .bind(backup_id)
        .fetch_optional(&self.pool).await
        .map_err(Into::into)
    }

    async fn get_storage_stats(&self) -> Result<BackupStorageStats> {
        let count: i64 = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM backup_records")
            .fetch_one(&self.pool).await?;
        
        let total_size: i64 = sqlx::query_scalar::<_, i64>("SELECT COALESCE(SUM(file_size_bytes), 0) FROM backup_records")
            .fetch_one(&self.pool).await?;
        
        Ok(BackupStorageStats {
            base: erp_core::BaseEntity::new(),
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
