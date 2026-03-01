use async_trait::async_trait;
use sqlx::{Row, SqlitePool};
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
        sqlx::query(
            r#"INSERT INTO encryption_keys (id, key_id, key_type, algorithm, key_version, public_key,
               encrypted_private_key, key_derivation_info, is_active, is_primary, rotation_days,
               last_rotated, expires_at, max_usage_count, current_usage_count, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(key.base.id.to_string())
        .bind(&key.key_id)
        .bind(format!("{:?}", key.key_type))
        .bind(&key.algorithm)
        .bind(key.key_version)
        .bind(&key.public_key)
        .bind(&key.encrypted_private_key)
        .bind(&key.key_derivation_info)
        .bind(key.is_active as i32)
        .bind(key.is_primary as i32)
        .bind(key.rotation_days)
        .bind(key.last_rotated.map(|d| d.to_rfc3339()))
        .bind(key.expires_at.map(|d| d.to_rfc3339()))
        .bind(key.max_usage_count)
        .bind(key.current_usage_count)
        .bind(key.base.created_at.to_rfc3339())
        .bind(key.base.updated_at.to_rfc3339())
        .bind(key.base.created_by.map(|id| id.to_string()))
        .bind(key.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(key)
    }

    async fn get_key(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<EncryptionKey>> {
        let row: Option<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, key_id, key_type, algorithm, key_version, public_key, encrypted_private_key,
               key_derivation_info, is_active, is_primary, rotation_days, last_rotated, expires_at,
               max_usage_count, current_usage_count, created_at, updated_at
               FROM encryption_keys WHERE id = ?"#
        )
        .bind(id.to_string())
        .fetch_optional(pool).await?;
        
        Ok(row.map(|r| EncryptionKey {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            key_id: r.get("key_id"),
            key_type: KeyType::Aes256Gcm,
            algorithm: r.get("algorithm"),
            key_version: r.get("key_version"),
            public_key: r.get("public_key"),
            encrypted_private_key: r.get("encrypted_private_key"),
            key_derivation_info: r.get("key_derivation_info"),
            is_active: r.get::<i32, _>("is_active") == 1,
            is_primary: r.get::<i32, _>("is_primary") == 1,
            rotation_days: r.get("rotation_days"),
            last_rotated: r.get::<Option<&str>, _>("last_rotated").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            expires_at: r.get::<Option<&str>, _>("expires_at").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            max_usage_count: r.get("max_usage_count"),
            current_usage_count: r.get("current_usage_count"),
        }))
    }

    async fn get_key_by_key_id(&self, pool: &SqlitePool, key_id: &str) -> Result<Option<EncryptionKey>> {
        let row: Option<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, key_id, key_type, algorithm, key_version, public_key, encrypted_private_key,
               key_derivation_info, is_active, is_primary, rotation_days, last_rotated, expires_at,
               max_usage_count, current_usage_count, created_at, updated_at
               FROM encryption_keys WHERE key_id = ?"#
        )
        .bind(key_id)
        .fetch_optional(pool).await?;
        
        Ok(row.map(|r| EncryptionKey {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            key_id: r.get("key_id"),
            key_type: KeyType::Aes256Gcm,
            algorithm: r.get("algorithm"),
            key_version: r.get("key_version"),
            public_key: r.get("public_key"),
            encrypted_private_key: r.get("encrypted_private_key"),
            key_derivation_info: r.get("key_derivation_info"),
            is_active: r.get::<i32, _>("is_active") == 1,
            is_primary: r.get::<i32, _>("is_primary") == 1,
            rotation_days: r.get("rotation_days"),
            last_rotated: r.get::<Option<&str>, _>("last_rotated").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            expires_at: r.get::<Option<&str>, _>("expires_at").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            max_usage_count: r.get("max_usage_count"),
            current_usage_count: r.get("current_usage_count"),
        }))
    }

    async fn get_primary_key(&self, pool: &SqlitePool, _key_type: KeyType) -> Result<Option<EncryptionKey>> {
        let row: Option<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, key_id, key_type, algorithm, key_version, public_key, encrypted_private_key,
               key_derivation_info, is_active, is_primary, rotation_days, last_rotated, expires_at,
               max_usage_count, current_usage_count, created_at, updated_at
               FROM encryption_keys WHERE is_primary = 1 AND is_active = 1 LIMIT 1"#
        )
        .fetch_optional(pool).await?;
        
        Ok(row.map(|r| EncryptionKey {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            key_id: r.get("key_id"),
            key_type: KeyType::Aes256Gcm,
            algorithm: r.get("algorithm"),
            key_version: r.get("key_version"),
            public_key: r.get("public_key"),
            encrypted_private_key: r.get("encrypted_private_key"),
            key_derivation_info: r.get("key_derivation_info"),
            is_active: r.get::<i32, _>("is_active") == 1,
            is_primary: r.get::<i32, _>("is_primary") == 1,
            rotation_days: r.get("rotation_days"),
            last_rotated: r.get::<Option<&str>, _>("last_rotated").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            expires_at: r.get::<Option<&str>, _>("expires_at").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            max_usage_count: r.get("max_usage_count"),
            current_usage_count: r.get("current_usage_count"),
        }))
    }

    async fn list_keys(&self, pool: &SqlitePool) -> Result<Vec<EncryptionKey>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, key_id, key_type, algorithm, key_version, public_key, encrypted_private_key,
               key_derivation_info, is_active, is_primary, rotation_days, last_rotated, expires_at,
               max_usage_count, current_usage_count, created_at, updated_at
               FROM encryption_keys ORDER BY created_at DESC"#
        )
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| EncryptionKey {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            key_id: r.get("key_id"),
            key_type: KeyType::Aes256Gcm,
            algorithm: r.get("algorithm"),
            key_version: r.get("key_version"),
            public_key: r.get("public_key"),
            encrypted_private_key: r.get("encrypted_private_key"),
            key_derivation_info: r.get("key_derivation_info"),
            is_active: r.get::<i32, _>("is_active") == 1,
            is_primary: r.get::<i32, _>("is_primary") == 1,
            rotation_days: r.get("rotation_days"),
            last_rotated: r.get::<Option<&str>, _>("last_rotated").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            expires_at: r.get::<Option<&str>, _>("expires_at").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            max_usage_count: r.get("max_usage_count"),
            current_usage_count: r.get("current_usage_count"),
        }).collect())
    }

    async fn update_key(&self, pool: &SqlitePool, key: EncryptionKey) -> Result<EncryptionKey> {
        sqlx::query(
            r#"UPDATE encryption_keys SET is_primary = ?, is_active = ?, rotation_days = ?,
               last_rotated = ?, current_usage_count = ?, updated_at = ?, updated_by = ?
               WHERE id = ?"#
        )
        .bind(key.is_primary as i32)
        .bind(key.is_active as i32)
        .bind(key.rotation_days)
        .bind(key.last_rotated.map(|d| d.to_rfc3339()))
        .bind(key.current_usage_count)
        .bind(key.base.updated_at.to_rfc3339())
        .bind(key.base.updated_by.map(|id| id.to_string()))
        .bind(key.base.id.to_string())
        .execute(pool).await?;
        Ok(key)
    }

    async fn delete_key(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE encryption_keys SET is_active = 0 WHERE id = ?")
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn create_rotation(&self, pool: &SqlitePool, rotation: KeyRotation) -> Result<KeyRotation> {
        sqlx::query(
            r#"INSERT INTO key_rotations (id, key_id, from_version, to_version, rotation_type, status,
               started_at, completed_at, re_encrypted_count, error_message, initiated_by, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(rotation.base.id.to_string())
        .bind(rotation.key_id.to_string())
        .bind(rotation.from_version)
        .bind(rotation.to_version)
        .bind(format!("{:?}", rotation.rotation_type))
        .bind(format!("{:?}", rotation.status))
        .bind(rotation.started_at.to_rfc3339())
        .bind(rotation.completed_at.map(|d| d.to_rfc3339()))
        .bind(rotation.re_encrypted_count)
        .bind(&rotation.error_message)
        .bind(rotation.initiated_by.map(|id| id.to_string()))
        .bind(rotation.base.created_at.to_rfc3339())
        .bind(rotation.base.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(rotation)
    }

    async fn update_rotation(&self, pool: &SqlitePool, rotation: KeyRotation) -> Result<KeyRotation> {
        sqlx::query(
            r#"UPDATE key_rotations SET status = ?, completed_at = ?, re_encrypted_count = ?,
               error_message = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(format!("{:?}", rotation.status))
        .bind(rotation.completed_at.map(|d| d.to_rfc3339()))
        .bind(rotation.re_encrypted_count)
        .bind(&rotation.error_message)
        .bind(rotation.base.updated_at.to_rfc3339())
        .bind(rotation.base.id.to_string())
        .execute(pool).await?;
        Ok(rotation)
    }

    async fn list_rotations(&self, pool: &SqlitePool, key_id: Uuid) -> Result<Vec<KeyRotation>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, key_id, from_version, to_version, rotation_type, status, started_at,
               completed_at, re_encrypted_count, error_message, initiated_by, created_at, updated_at
               FROM key_rotations WHERE key_id = ? ORDER BY started_at DESC"#
        )
        .bind(key_id.to_string())
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| KeyRotation {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            key_id: Uuid::parse_str(r.get::<&str, _>("key_id")).unwrap(),
            from_version: r.get("from_version"),
            to_version: r.get("to_version"),
            rotation_type: RotationType::Manual,
            status: RotationStatus::Completed,
            started_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("started_at")).unwrap().with_timezone(&chrono::Utc),
            completed_at: r.get::<Option<&str>, _>("completed_at").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            re_encrypted_count: r.get("re_encrypted_count"),
            error_message: r.get("error_message"),
            initiated_by: r.get::<Option<&str>, _>("initiated_by").and_then(|id| Uuid::parse_str(id).ok()),
        }).collect())
    }

    async fn log_usage(&self, pool: &SqlitePool, log: KeyUsageLog) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO key_usage_logs (id, key_id, operation, entity_type, entity_id, success,
               error_message, performed_at, performed_by, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(log.base.id.to_string())
        .bind(log.key_id.to_string())
        .bind(format!("{:?}", log.operation))
        .bind(log.entity_type)
        .bind(log.entity_id)
        .bind(log.success as i32)
        .bind(log.error_message)
        .bind(log.performed_at.to_rfc3339())
        .bind(log.performed_by.map(|id| id.to_string()))
        .bind(log.base.created_at.to_rfc3339())
        .bind(log.base.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(())
    }

    async fn create_encrypted_data(&self, pool: &SqlitePool, data: EncryptedData) -> Result<EncryptedData> {
        sqlx::query(
            r#"INSERT INTO encrypted_data (id, entity_type, entity_id, field_name, key_id, key_version,
               iv, auth_tag, encrypted_value, encryption_algorithm, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(data.base.id.to_string())
        .bind(&data.entity_type)
        .bind(&data.entity_id)
        .bind(&data.field_name)
        .bind(data.key_id.to_string())
        .bind(data.key_version)
        .bind(&data.iv)
        .bind(&data.auth_tag)
        .bind(&data.encrypted_value)
        .bind(&data.encryption_algorithm)
        .bind(data.created_at.to_rfc3339())
        .bind(data.base.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(data)
    }

    async fn list_encrypted_data(&self, pool: &SqlitePool, entity_type: &str, entity_id: &str) -> Result<Vec<EncryptedData>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, entity_type, entity_id, field_name, key_id, key_version, iv, auth_tag,
               encrypted_value, encryption_algorithm, created_at, updated_at
               FROM encrypted_data WHERE entity_type = ? AND entity_id = ?"#
        )
        .bind(entity_type)
        .bind(entity_id)
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| EncryptedData {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            entity_type: r.get("entity_type"),
            entity_id: r.get("entity_id"),
            field_name: r.get("field_name"),
            key_id: Uuid::parse_str(r.get::<&str, _>("key_id")).unwrap(),
            key_version: r.get("key_version"),
            iv: r.get("iv"),
            auth_tag: r.get("auth_tag"),
            encrypted_value: r.get("encrypted_value"),
            encryption_algorithm: r.get("encryption_algorithm"),
            created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
        }).collect())
    }

    async fn update_encrypted_data(&self, pool: &SqlitePool, data: EncryptedData) -> Result<EncryptedData> {
        sqlx::query(
            r#"UPDATE encrypted_data SET key_id = ?, key_version = ?, iv = ?, auth_tag = ?,
               encrypted_value = ?, encryption_algorithm = ?, updated_at = ?
               WHERE id = ?"#
        )
        .bind(data.key_id.to_string())
        .bind(data.key_version)
        .bind(&data.iv)
        .bind(&data.auth_tag)
        .bind(&data.encrypted_value)
        .bind(&data.encryption_algorithm)
        .bind(data.base.updated_at.to_rfc3339())
        .bind(data.base.id.to_string())
        .execute(pool).await?;
        Ok(data)
    }

    async fn create_policy(&self, pool: &SqlitePool, policy: KeyPolicy) -> Result<KeyPolicy> {
        sqlx::query(
            r#"INSERT INTO key_policies (id, name, key_type, algorithm, key_size_bits, rotation_days,
               max_usage_count, require_hsm, allow_export, allowed_operations, is_active, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(policy.base.id.to_string())
        .bind(&policy.name)
        .bind(format!("{:?}", policy.key_type))
        .bind(&policy.algorithm)
        .bind(policy.key_size_bits)
        .bind(policy.rotation_days)
        .bind(policy.max_usage_count)
        .bind(policy.require_hsm as i32)
        .bind(policy.allow_export as i32)
        .bind(&policy.allowed_operations)
        .bind(policy.is_active as i32)
        .bind(policy.base.created_at.to_rfc3339())
        .bind(policy.base.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(policy)
    }

    async fn list_policies(&self, pool: &SqlitePool) -> Result<Vec<KeyPolicy>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, name, key_type, algorithm, key_size_bits, rotation_days, max_usage_count,
               require_hsm, allow_export, allowed_operations, is_active, created_at, updated_at
               FROM key_policies ORDER BY name"#
        )
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| KeyPolicy {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: r.get("name"),
            key_type: KeyType::Aes256Gcm,
            algorithm: r.get("algorithm"),
            key_size_bits: r.get("key_size_bits"),
            rotation_days: r.get("rotation_days"),
            max_usage_count: r.get("max_usage_count"),
            require_hsm: r.get::<i32, _>("require_hsm") == 1,
            allow_export: r.get::<i32, _>("allow_export") == 1,
            allowed_operations: r.get("allowed_operations"),
            is_active: r.get::<i32, _>("is_active") == 1,
        }).collect())
    }
}
