use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupSchedule {
    pub base: BaseEntity,
    pub name: String,
    pub backup_type: BackupType,
    pub schedule_cron: String,
    pub retention_days: i32,
    pub max_backups: i32,
    pub compression: bool,
    pub encryption_enabled: bool,
    pub encryption_key_id: Option<Uuid>,
    pub storage_type: BackupStorageType,
    pub storage_path: String,
    pub include_attachments: bool,
    pub is_active: bool,
    pub last_run: Option<DateTime<Utc>>,
    pub next_run: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BackupType {
    Full,
    Incremental,
    Differential,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BackupStorageType {
    Local,
    S3,
    AzureBlob,
    GCS,
    SFTP,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupRecord {
    pub base: BaseEntity,
    pub schedule_id: Option<Uuid>,
    pub backup_type: BackupType,
    pub status: BackupStatus,
    pub file_path: String,
    pub file_size_bytes: i64,
    pub compressed_size_bytes: Option<i64>,
    pub checksum: Option<String>,
    pub checksum_algorithm: Option<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
    pub tables_included: Option<String>,
    pub records_count: Option<i64>,
    pub error_message: Option<String>,
    pub verification_status: Option<VerificationStatus>,
    pub verified_at: Option<DateTime<Utc>>,
    pub is_restorable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BackupStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum VerificationStatus {
    Pending,
    Verified,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreOperation {
    pub base: BaseEntity,
    pub backup_id: Uuid,
    pub status: RestoreStatus,
    pub restore_type: RestoreType,
    pub target_tables: Option<String>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
    pub records_restored: Option<i64>,
    pub error_message: Option<String>,
    pub initiated_by: Option<Uuid>,
    pub backup_before_restore: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RestoreStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    RolledBack,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RestoreType {
    Full,
    Partial,
    PointInTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupVerification {
    pub base: BaseEntity,
    pub backup_id: Uuid,
    pub status: VerificationStatus,
    pub checked_at: DateTime<Utc>,
    pub checksum_valid: bool,
    pub file_readable: bool,
    pub schema_valid: bool,
    pub sample_data_valid: bool,
    pub error_details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupStorageStats {
    pub base: BaseEntity,
    pub storage_type: BackupStorageType,
    pub total_size_bytes: i64,
    pub backup_count: i32,
    pub oldest_backup: Option<DateTime<Utc>>,
    pub newest_backup: Option<DateTime<Utc>>,
    pub available_space_bytes: Option<i64>,
    pub calculated_at: DateTime<Utc>,
}
