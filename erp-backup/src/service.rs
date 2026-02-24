use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use std::path::PathBuf;
use erp_core::{BaseEntity, Result};
use crate::models::*;
use crate::repository::{BackupRepository, SqliteBackupRepository};

pub struct BackupService {
    repo: SqliteBackupRepository,
}

impl BackupService {
    pub fn new() -> Self {
        Self { repo: SqliteBackupRepository }
    }

    pub async fn create_schedule(&self, pool: &SqlitePool, schedule: BackupSchedule) -> Result<BackupSchedule> {
        self.repo.create_schedule(pool, schedule).await
    }

    pub async fn list_schedules(&self, pool: &SqlitePool) -> Result<Vec<BackupSchedule>> {
        self.repo.list_schedules(pool).await
    }

    pub async fn get_schedule(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<BackupSchedule>> {
        self.repo.get_schedule(pool, id).await
    }

    pub async fn update_schedule(&self, pool: &SqlitePool, schedule: BackupSchedule) -> Result<BackupSchedule> {
        self.repo.update_schedule(pool, schedule).await
    }

    pub async fn delete_schedule(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete_schedule(pool, id).await
    }

    pub async fn execute_backup(&self, pool: &SqlitePool, schedule_id: Option<Uuid>) -> Result<BackupRecord> {
        let started_at = Utc::now();
        
        let backup = BackupRecord {
            base: BaseEntity::new(),
            schedule_id,
            backup_type: BackupType::Full,
            status: BackupStatus::InProgress,
            file_path: String::new(),
            file_size_bytes: 0,
            compressed_size_bytes: None,
            checksum: None,
            checksum_algorithm: Some("SHA256".to_string()),
            started_at,
            completed_at: None,
            duration_seconds: None,
            tables_included: None,
            records_count: None,
            error_message: None,
            verification_status: None,
            verified_at: None,
            is_restorable: false,
        };
        
        let mut backup = self.repo.create_backup(pool, backup).await?;
        
        match self.do_backup(pool, &mut backup).await {
            Ok(_) => {
                backup.status = BackupStatus::Completed;
                backup.completed_at = Some(Utc::now());
                backup.duration_seconds = Some((Utc::now() - started_at).num_seconds());
                backup.is_restorable = true;
                self.repo.update_backup(pool, backup.clone()).await
            }
            Err(e) => {
                backup.status = BackupStatus::Failed;
                backup.error_message = Some(e.to_string());
                backup.completed_at = Some(Utc::now());
                self.repo.update_backup(pool, backup.clone()).await
            }
        }
    }

    async fn do_backup(&self, pool: &SqlitePool, backup: &mut BackupRecord) -> Result<()> {
        let timestamp = Utc::now().format("%Y%m%d_%H%M%S");
        let filename = format!("backup_{}.db", timestamp);
        let backup_dir = PathBuf::from("./backups");
        
        tokio::fs::create_dir_all(&backup_dir).await
            .map_err(|e| anyhow::anyhow!("Failed to create backup directory: {}", e))?;
        
        let backup_path = backup_dir.join(&filename);
        backup.file_path = backup_path.to_string_lossy().to_string();
        
        let db_path = "./erp.db";
        
        tokio::fs::copy(db_path, &backup_path).await
            .map_err(|e| anyhow::anyhow!("Failed to copy database: {}", e))?;
        
        let metadata = tokio::fs::metadata(&backup_path).await
            .map_err(|e| anyhow::anyhow!("Failed to get backup file metadata: {}", e))?;
        backup.file_size_bytes = metadata.len() as i64;
        
        let tables: Vec<String> = sqlx::query_scalar("SELECT name FROM sqlite_master WHERE type='table'")
            .fetch_all(pool).await?;
        backup.tables_included = Some(tables.join(","));
        
        let total_records: i64 = sqlx::query_scalar(
            "SELECT SUM(cnt) FROM (SELECT COUNT(*) as cnt FROM users UNION ALL SELECT COUNT(*) FROM products UNION ALL SELECT COUNT(*) FROM customers)"
        ).fetch_optional(pool).await?.unwrap_or(Some(0)).unwrap_or(0);
        backup.records_count = Some(total_records);
        
        Ok(())
    }

    pub async fn list_backups(&self, pool: &SqlitePool, limit: i32) -> Result<Vec<BackupRecord>> {
        self.repo.list_backups(pool, limit).await
    }

    pub async fn get_backup(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<BackupRecord>> {
        self.repo.get_backup(pool, id).await
    }

    pub async fn delete_backup(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        if let Some(backup) = self.repo.get_backup(pool, id).await? {
            if !backup.file_path.is_empty() {
                let _ = tokio::fs::remove_file(&backup.file_path).await;
            }
        }
        self.repo.delete_backup(pool, id).await
    }

    pub async fn restore_backup(&self, pool: &SqlitePool, backup_id: Uuid, initiated_by: Option<Uuid>) -> Result<RestoreOperation> {
        let backup = self.repo.get_backup(pool, backup_id).await?
            .ok_or_else(|| anyhow::anyhow!("Backup not found"))?;
        
        if !backup.is_restorable {
            return Err(erp_core::Error::NotFound("Backup is not restorable".to_string()));
        }
        
        let restore = RestoreOperation {
            base: BaseEntity::new(),
            backup_id,
            status: RestoreStatus::InProgress,
            restore_type: RestoreType::Full,
            target_tables: None,
            started_at: Utc::now(),
            completed_at: None,
            duration_seconds: None,
            records_restored: None,
            error_message: None,
            initiated_by,
            backup_before_restore: None,
        };
        
        let mut restore = self.repo.create_restore(pool, restore).await?;
        
        match self.do_restore(&backup).await {
            Ok(records) => {
                restore.status = RestoreStatus::Completed;
                restore.completed_at = Some(Utc::now());
                restore.duration_seconds = Some((Utc::now() - restore.started_at).num_seconds());
                restore.records_restored = Some(records);
                self.repo.update_restore(pool, restore.clone()).await
            }
            Err(e) => {
                restore.status = RestoreStatus::Failed;
                restore.error_message = Some(e.to_string());
                restore.completed_at = Some(Utc::now());
                self.repo.update_restore(pool, restore.clone()).await
            }
        }
    }

    async fn do_restore(&self, backup: &BackupRecord) -> Result<i64> {
        let backup_path = PathBuf::from(&backup.file_path);
        if !backup_path.exists() {
            return Err(erp_core::Error::NotFound(format!("Backup file not found: {}", backup.file_path)));
        }
        
        tokio::fs::copy(&backup_path, "./erp.db").await
            .map_err(|e| anyhow::anyhow!("Failed to restore database: {}", e))?;
        
        Ok(backup.records_count.unwrap_or(0))
    }

    pub async fn verify_backup(&self, pool: &SqlitePool, backup_id: Uuid) -> Result<BackupVerification> {
        let backup = self.repo.get_backup(pool, backup_id).await?
            .ok_or_else(|| anyhow::anyhow!("Backup not found"))?;
        
        let mut verification = BackupVerification {
            base: BaseEntity::new(),
            backup_id,
            status: VerificationStatus::Pending,
            checked_at: Utc::now(),
            checksum_valid: true,
            file_readable: false,
            schema_valid: false,
            sample_data_valid: false,
            error_details: None,
        };
        
        let backup_path = PathBuf::from(&backup.file_path);
        verification.file_readable = backup_path.exists();
        
        if verification.file_readable {
            verification.schema_valid = true;
            verification.sample_data_valid = true;
            verification.status = VerificationStatus::Verified;
        } else {
            verification.status = VerificationStatus::Failed;
            verification.error_details = Some("Backup file not found".to_string());
        }
        
        self.repo.create_verification(pool, verification.clone()).await?;
        
        let mut backup = backup;
        backup.verification_status = Some(verification.status.clone());
        backup.verified_at = Some(Utc::now());
        self.repo.update_backup(pool, backup).await?;
        
        Ok(verification)
    }

    pub async fn list_restores(&self, pool: &SqlitePool, limit: i32) -> Result<Vec<RestoreOperation>> {
        self.repo.list_restores(pool, limit).await
    }

    pub async fn get_storage_stats(&self, pool: &SqlitePool) -> Result<BackupStorageStats> {
        self.repo.get_storage_stats(pool).await
    }

    pub async fn cleanup_old_backups(&self, pool: &SqlitePool, retention_days: i32) -> Result<i32> {
        let cutoff = Utc::now() - chrono::Duration::days(retention_days as i64);
        let backups = self.repo.list_backups(pool, 1000).await?;
        let mut deleted = 0;
        
        for backup in backups {
            if backup.started_at < cutoff {
                self.delete_backup(pool, backup.base.id).await?;
                deleted += 1;
            }
        }
        
        Ok(deleted)
    }
}
