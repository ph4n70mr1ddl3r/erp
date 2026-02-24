use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::Result;
use crate::models::*;

#[async_trait]
pub trait KeyRepository: Send + Sync {
    async fn create_key(&self, pool: &SqlitePool, key: EncryptionKey) -> Result<EncryptionKey>;
    async fn get_key(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<EncryptionKey>>;
    async fn get_key_by_key_id(&self, pool: &SqlitePool, key_id: &str) -> Result<Option<EncryptionKey>>;
    async fn get_primary_key(&self, pool: &SqlitePool, key_type: KeyType) -> Result<Option<EncryptionKey>>;
    async fn list_keys(&self, pool: &SqlitePool) -> Result<Vec<EncryptionKey>>;
    async fn update_key(&self, pool: &SqlitePool, key: EncryptionKey) -> Result<EncryptionKey>;
    async fn delete_key(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn create_rotation(&self, pool: &SqlitePool, rotation: KeyRotation) -> Result<KeyRotation>;
    async fn update_rotation(&self, pool: &SqlitePool, rotation: KeyRotation) -> Result<KeyRotation>;
    async fn list_rotations(&self, pool: &SqlitePool, key_id: Uuid) -> Result<Vec<KeyRotation>>;
    async fn log_usage(&self, pool: &SqlitePool, log: KeyUsageLog) -> Result<()>;
    async fn create_encrypted_data(&self, pool: &SqlitePool, data: EncryptedData) -> Result<EncryptedData>;
    async fn list_encrypted_data(&self, pool: &SqlitePool, entity_type: &str, entity_id: &str) -> Result<Vec<EncryptedData>>;
    async fn update_encrypted_data(&self, pool: &SqlitePool, data: EncryptedData) -> Result<EncryptedData>;
    async fn create_policy(&self, pool: &SqlitePool, policy: KeyPolicy) -> Result<KeyPolicy>;
    async fn list_policies(&self, pool: &SqlitePool) -> Result<Vec<KeyPolicy>>;
}

pub struct SqliteKeyRepository;

#[async_trait]
impl KeyRepository for SqliteKeyRepository {
    async fn create_key(&self, pool: &SqlitePool, key: EncryptionKey) -> Result<EncryptionKey> {
        sqlx::query!(
            r#"INSERT INTO encryption_keys (id, key_id, key_type, algorithm, key_version, public_key,
               encrypted_private_key, key_derivation_info, is_active, is_primary, rotation_days,
               last_rotated, expires_at, max_usage_count, current_usage_count, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            key.base.id.to_string(),
            key.key_id,
            format!("{:?}", key.key_type),
            key.algorithm,
            key.key_version,
            key.public_key,
            key.encrypted_private_key,
            key.key_derivation_info,
            key.is_active as i32,
            key.is_primary as i32,
            key.rotation_days,
            key.last_rotated.map(|d| d.to_rfc3339()),
            key.expires_at.map(|d| d.to_rfc3339()),
            key.max_usage_count,
            key.current_usage_count,
            key.base.created_at.to_rfc3339(),
            key.base.updated_at.to_rfc3339(),
            key.base.created_by.map(|id| id.to_string()),
            key.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(key)
    }

    async fn get_key(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<EncryptionKey>> {
        let row = sqlx::query!(
            r#"SELECT id, key_id, key_type, algorithm, key_version, public_key, encrypted_private_key,
               key_derivation_info, is_active, is_primary, rotation_days, last_rotated, expires_at,
               max_usage_count, current_usage_count, created_at, updated_at
               FROM encryption_keys WHERE id = ?"#,
            id.to_string()
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|r| EncryptionKey {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            key_id: r.key_id,
            key_type: KeyType::Aes256Gcm,
            algorithm: r.algorithm,
            key_version: r.key_version,
            public_key: r.public_key,
            encrypted_private_key: r.encrypted_private_key,
            key_derivation_info: r.key_derivation_info,
            is_active: r.is_active == 1,
            is_primary: r.is_primary == 1,
            rotation_days: r.rotation_days,
            last_rotated: r.last_rotated.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            expires_at: r.expires_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            max_usage_count: r.max_usage_count,
            current_usage_count: r.current_usage_count,
        }))
    }

    async fn get_key_by_key_id(&self, pool: &SqlitePool, key_id: &str) -> Result<Option<EncryptionKey>> {
        let row = sqlx::query!(
            r#"SELECT id, key_id, key_type, algorithm, key_version, public_key, encrypted_private_key,
               key_derivation_info, is_active, is_primary, rotation_days, last_rotated, expires_at,
               max_usage_count, current_usage_count, created_at, updated_at
               FROM encryption_keys WHERE key_id = ?"#,
            key_id
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|r| EncryptionKey {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            key_id: r.key_id,
            key_type: KeyType::Aes256Gcm,
            algorithm: r.algorithm,
            key_version: r.key_version,
            public_key: r.public_key,
            encrypted_private_key: r.encrypted_private_key,
            key_derivation_info: r.key_derivation_info,
            is_active: r.is_active == 1,
            is_primary: r.is_primary == 1,
            rotation_days: r.rotation_days,
            last_rotated: r.last_rotated.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            expires_at: r.expires_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            max_usage_count: r.max_usage_count,
            current_usage_count: r.current_usage_count,
        }))
    }

    async fn get_primary_key(&self, pool: &SqlitePool, _key_type: KeyType) -> Result<Option<EncryptionKey>> {
        let row = sqlx::query!(
            r#"SELECT id, key_id, key_type, algorithm, key_version, public_key, encrypted_private_key,
               key_derivation_info, is_active, is_primary, rotation_days, last_rotated, expires_at,
               max_usage_count, current_usage_count, created_at, updated_at
               FROM encryption_keys WHERE is_primary = 1 AND is_active = 1 LIMIT 1"#
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|r| EncryptionKey {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            key_id: r.key_id,
            key_type: KeyType::Aes256Gcm,
            algorithm: r.algorithm,
            key_version: r.key_version,
            public_key: r.public_key,
            encrypted_private_key: r.encrypted_private_key,
            key_derivation_info: r.key_derivation_info,
            is_active: r.is_active == 1,
            is_primary: r.is_primary == 1,
            rotation_days: r.rotation_days,
            last_rotated: r.last_rotated.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            expires_at: r.expires_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            max_usage_count: r.max_usage_count,
            current_usage_count: r.current_usage_count,
        }))
    }

    async fn list_keys(&self, pool: &SqlitePool) -> Result<Vec<EncryptionKey>> {
        let rows = sqlx::query!(
            r#"SELECT id, key_id, key_type, algorithm, key_version, public_key, encrypted_private_key,
               key_derivation_info, is_active, is_primary, rotation_days, last_rotated, expires_at,
               max_usage_count, current_usage_count, created_at, updated_at
               FROM encryption_keys ORDER BY created_at DESC"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| EncryptionKey {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            key_id: r.key_id,
            key_type: KeyType::Aes256Gcm,
            algorithm: r.algorithm,
            key_version: r.key_version,
            public_key: r.public_key,
            encrypted_private_key: r.encrypted_private_key,
            key_derivation_info: r.key_derivation_info,
            is_active: r.is_active == 1,
            is_primary: r.is_primary == 1,
            rotation_days: r.rotation_days,
            last_rotated: r.last_rotated.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            expires_at: r.expires_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            max_usage_count: r.max_usage_count,
            current_usage_count: r.current_usage_count,
        }).collect())
    }

    async fn update_key(&self, pool: &SqlitePool, key: EncryptionKey) -> Result<EncryptionKey> {
        sqlx::query!(
            r#"UPDATE encryption_keys SET is_primary = ?, is_active = ?, rotation_days = ?,
               last_rotated = ?, current_usage_count = ?, updated_at = ?, updated_by = ?
               WHERE id = ?"#,
            key.is_primary as i32,
            key.is_active as i32,
            key.rotation_days,
            key.last_rotated.map(|d| d.to_rfc3339()),
            key.current_usage_count,
            key.base.updated_at.to_rfc3339(),
            key.base.updated_by.map(|id| id.to_string()),
            key.base.id.to_string(),
        ).execute(pool).await?;
        Ok(key)
    }

    async fn delete_key(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query!("UPDATE encryption_keys SET is_active = 0 WHERE id = ?", id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn create_rotation(&self, pool: &SqlitePool, rotation: KeyRotation) -> Result<KeyRotation> {
        sqlx::query!(
            r#"INSERT INTO key_rotations (id, key_id, from_version, to_version, rotation_type, status,
               started_at, completed_at, re_encrypted_count, error_message, initiated_by, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            rotation.base.id.to_string(),
            rotation.key_id.to_string(),
            rotation.from_version,
            rotation.to_version,
            format!("{:?}", rotation.rotation_type),
            format!("{:?}", rotation.status),
            rotation.started_at.to_rfc3339(),
            rotation.completed_at.map(|d| d.to_rfc3339()),
            rotation.re_encrypted_count,
            rotation.error_message,
            rotation.initiated_by.map(|id| id.to_string()),
            rotation.base.created_at.to_rfc3339(),
            rotation.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(rotation)
    }

    async fn update_rotation(&self, pool: &SqlitePool, rotation: KeyRotation) -> Result<KeyRotation> {
        sqlx::query!(
            r#"UPDATE key_rotations SET status = ?, completed_at = ?, re_encrypted_count = ?,
               error_message = ?, updated_at = ? WHERE id = ?"#,
            format!("{:?}", rotation.status),
            rotation.completed_at.map(|d| d.to_rfc3339()),
            rotation.re_encrypted_count,
            rotation.error_message,
            rotation.base.updated_at.to_rfc3339(),
            rotation.base.id.to_string(),
        ).execute(pool).await?;
        Ok(rotation)
    }

    async fn list_rotations(&self, pool: &SqlitePool, key_id: Uuid) -> Result<Vec<KeyRotation>> {
        let rows = sqlx::query!(
            r#"SELECT id, key_id, from_version, to_version, rotation_type, status, started_at,
               completed_at, re_encrypted_count, error_message, initiated_by, created_at, updated_at
               FROM key_rotations WHERE key_id = ? ORDER BY started_at DESC"#,
            key_id.to_string()
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| KeyRotation {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            key_id: Uuid::parse_str(&r.key_id).unwrap(),
            from_version: r.from_version,
            to_version: r.to_version,
            rotation_type: RotationType::Manual,
            status: RotationStatus::Completed,
            started_at: chrono::DateTime::parse_from_rfc3339(&r.started_at).unwrap().with_timezone(&chrono::Utc),
            completed_at: r.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            re_encrypted_count: r.re_encrypted_count,
            error_message: r.error_message,
            initiated_by: r.initiated_by.and_then(|id| Uuid::parse_str(&id).ok()),
        }).collect())
    }

    async fn log_usage(&self, pool: &SqlitePool, log: KeyUsageLog) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO key_usage_logs (id, key_id, operation, entity_type, entity_id, success,
               error_message, performed_at, performed_by, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            log.base.id.to_string(),
            log.key_id.to_string(),
            format!("{:?}", log.operation),
            log.entity_type,
            log.entity_id,
            log.success as i32,
            log.error_message,
            log.performed_at.to_rfc3339(),
            log.performed_by.map(|id| id.to_string()),
            log.base.created_at.to_rfc3339(),
            log.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(())
    }

    async fn create_encrypted_data(&self, pool: &SqlitePool, data: EncryptedData) -> Result<EncryptedData> {
        sqlx::query!(
            r#"INSERT INTO encrypted_data (id, entity_type, entity_id, field_name, key_id, key_version,
               iv, auth_tag, encrypted_value, encryption_algorithm, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            data.base.id.to_string(),
            data.entity_type,
            data.entity_id,
            data.field_name,
            data.key_id.to_string(),
            data.key_version,
            data.iv,
            data.auth_tag,
            data.encrypted_value,
            data.encryption_algorithm,
            data.created_at.to_rfc3339(),
            data.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(data)
    }

    async fn list_encrypted_data(&self, pool: &SqlitePool, entity_type: &str, entity_id: &str) -> Result<Vec<EncryptedData>> {
        let rows = sqlx::query!(
            r#"SELECT id, entity_type, entity_id, field_name, key_id, key_version, iv, auth_tag,
               encrypted_value, encryption_algorithm, created_at, updated_at
               FROM encrypted_data WHERE entity_type = ? AND entity_id = ?"#,
            entity_type, entity_id
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| EncryptedData {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            entity_type: r.entity_type,
            entity_id: r.entity_id,
            field_name: r.field_name,
            key_id: Uuid::parse_str(&r.key_id).unwrap(),
            key_version: r.key_version,
            iv: r.iv,
            auth_tag: r.auth_tag,
            encrypted_value: r.encrypted_value,
            encryption_algorithm: r.encryption_algorithm,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
        }).collect())
    }

    async fn update_encrypted_data(&self, pool: &SqlitePool, data: EncryptedData) -> Result<EncryptedData> {
        sqlx::query!(
            r#"UPDATE encrypted_data SET key_id = ?, key_version = ?, iv = ?, auth_tag = ?,
               encrypted_value = ?, encryption_algorithm = ?, updated_at = ?
               WHERE id = ?"#,
            data.key_id.to_string(),
            data.key_version,
            data.iv,
            data.auth_tag,
            data.encrypted_value,
            data.encryption_algorithm,
            data.base.updated_at.to_rfc3339(),
            data.base.id.to_string(),
        ).execute(pool).await?;
        Ok(data)
    }

    async fn create_policy(&self, pool: &SqlitePool, policy: KeyPolicy) -> Result<KeyPolicy> {
        sqlx::query!(
            r#"INSERT INTO key_policies (id, name, key_type, algorithm, key_size_bits, rotation_days,
               max_usage_count, require_hsm, allow_export, allowed_operations, is_active, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            policy.base.id.to_string(),
            policy.name,
            format!("{:?}", policy.key_type),
            policy.algorithm,
            policy.key_size_bits,
            policy.rotation_days,
            policy.max_usage_count,
            policy.require_hsm as i32,
            policy.allow_export as i32,
            policy.allowed_operations,
            policy.is_active as i32,
            policy.base.created_at.to_rfc3339(),
            policy.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(policy)
    }

    async fn list_policies(&self, pool: &SqlitePool) -> Result<Vec<KeyPolicy>> {
        let rows = sqlx::query!(
            r#"SELECT id, name, key_type, algorithm, key_size_bits, rotation_days, max_usage_count,
               require_hsm, allow_export, allowed_operations, is_active, created_at, updated_at
               FROM key_policies ORDER BY name"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| KeyPolicy {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: r.name,
            key_type: KeyType::Aes256Gcm,
            algorithm: r.algorithm,
            key_size_bits: r.key_size_bits,
            rotation_days: r.rotation_days,
            max_usage_count: r.max_usage_count,
            require_hsm: r.require_hsm == 1,
            allow_export: r.allow_export == 1,
            allowed_operations: r.allowed_operations,
            is_active: r.is_active == 1,
        }).collect())
    }
}
