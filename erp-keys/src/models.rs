use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionKey {
    pub base: BaseEntity,
    pub key_id: String,
    pub key_type: KeyType,
    pub algorithm: String,
    pub key_version: i32,
    pub public_key: Option<String>,
    pub encrypted_private_key: Option<String>,
    pub key_derivation_info: Option<String>,
    pub is_active: bool,
    pub is_primary: bool,
    pub rotation_days: Option<i32>,
    pub last_rotated: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub max_usage_count: Option<i64>,
    pub current_usage_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum KeyType {
    Symmetric,
    Asymmetric,
    Hmac,
    DataEncryption,
    Aes256Gcm,
    Aes256Cbc,
    Rsa2048,
    Rsa4096,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyRotation {
    pub base: BaseEntity,
    pub key_id: Uuid,
    pub from_version: i32,
    pub to_version: i32,
    pub rotation_type: RotationType,
    pub status: RotationStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub re_encrypted_count: i64,
    pub error_message: Option<String>,
    pub initiated_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RotationType {
    Scheduled,
    Manual,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RotationStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
    RolledBack,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedData {
    pub base: BaseEntity,
    pub entity_type: String,
    pub entity_id: String,
    pub field_name: String,
    pub key_id: Uuid,
    pub key_version: i32,
    pub iv: String,
    pub auth_tag: Option<String>,
    pub encrypted_value: String,
    pub encryption_algorithm: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyUsageLog {
    pub base: BaseEntity,
    pub key_id: Uuid,
    pub operation: KeyOperation,
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
    pub performed_at: DateTime<Utc>,
    pub performed_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum KeyOperation {
    Encrypt,
    Decrypt,
    Sign,
    Verify,
    Rotate,
    Export,
    Import,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPolicy {
    pub base: BaseEntity,
    pub name: String,
    pub key_type: KeyType,
    pub algorithm: String,
    pub key_size_bits: i32,
    pub rotation_days: i32,
    pub max_usage_count: Option<i64>,
    pub require_hsm: bool,
    pub allow_export: bool,
    pub allowed_operations: String,
    pub is_active: bool,
}
