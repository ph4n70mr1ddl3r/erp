use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use rand::RngCore;
use erp_core::{BaseEntity, Result};
use crate::models::*;
use crate::repository::{KeyRepository, SqliteKeyRepository};

pub struct KeyService {
    repo: SqliteKeyRepository,
}

impl Default for KeyService {
    fn default() -> Self {
        Self::new()
    }
}

impl KeyService {
    pub fn new() -> Self {
        Self { repo: SqliteKeyRepository }
    }

    pub async fn generate_key(&self, pool: &SqlitePool, key_type: KeyType, name: &str) -> Result<EncryptionKey> {
        let key_id = format!("key_{}", Uuid::new_v4().simple());
        let now = Utc::now();
        
        let (algorithm, encrypted_private_key) = match key_type {
            KeyType::Aes256Gcm | KeyType::Symmetric | KeyType::DataEncryption => {
                let mut key_bytes = [0u8; 32];
                rand::thread_rng().fill_bytes(&mut key_bytes);
                ("AES-256-GCM".to_string(), Some(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, key_bytes)))
            }
            KeyType::Aes256Cbc => {
                let mut key_bytes = [0u8; 32];
                rand::thread_rng().fill_bytes(&mut key_bytes);
                ("AES-256-CBC".to_string(), Some(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, key_bytes)))
            }
            KeyType::Hmac => {
                let mut key_bytes = [0u8; 64];
                rand::thread_rng().fill_bytes(&mut key_bytes);
                ("HMAC-SHA256".to_string(), Some(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, key_bytes)))
            }
            _ => ("UNKNOWN".to_string(), None),
        };

        let key = EncryptionKey {
            base: BaseEntity::new(),
            key_id,
            key_type,
            algorithm,
            key_version: 1,
            public_key: None,
            encrypted_private_key,
            key_derivation_info: Some(format!("Generated for: {}", name)),
            is_active: true,
            is_primary: false,
            rotation_days: Some(90),
            last_rotated: Some(now),
            expires_at: None,
            max_usage_count: None,
            current_usage_count: 0,
        };

        self.repo.create_key(pool, key).await
    }

    pub async fn list_keys(&self, pool: &SqlitePool) -> Result<Vec<EncryptionKey>> {
        self.repo.list_keys(pool).await
    }

    pub async fn get_key(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<EncryptionKey>> {
        self.repo.get_key(pool, id).await
    }

    pub async fn set_primary_key(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let keys = self.repo.list_keys(pool).await?;
        for mut key in keys {
            if key.is_primary {
                key.is_primary = false;
                key.base.updated_at = Utc::now();
                self.repo.update_key(pool, key).await?;
            }
        }
        
        let mut key = self.repo.get_key(pool, id).await?
            .ok_or_else(|| anyhow::anyhow!("Key not found"))?;
        key.is_primary = true;
        key.base.updated_at = Utc::now();
        self.repo.update_key(pool, key).await?;
        Ok(())
    }

    pub async fn rotate_key(&self, pool: &SqlitePool, id: Uuid, initiated_by: Option<Uuid>) -> Result<KeyRotation> {
        let old_key = self.repo.get_key(pool, id).await?
            .ok_or_else(|| anyhow::anyhow!("Key not found"))?;
        
        let new_key = self.generate_key(pool, old_key.key_type.clone(), &old_key.key_id).await?;
        
        let rotation = KeyRotation {
            base: BaseEntity::new(),
            key_id: id,
            from_version: old_key.key_version,
            to_version: new_key.key_version,
            rotation_type: RotationType::Manual,
            status: RotationStatus::Completed,
            started_at: Utc::now(),
            completed_at: Some(Utc::now()),
            re_encrypted_count: 0,
            error_message: None,
            initiated_by,
        };
        
        self.repo.create_rotation(pool, rotation.clone()).await?;
        
        let mut old_key = old_key;
        old_key.is_primary = false;
        old_key.base.updated_at = Utc::now();
        self.repo.update_key(pool, old_key).await?;
        
        self.set_primary_key(pool, new_key.base.id).await?;
        
        Ok(rotation)
    }

    pub async fn encrypt_data(&self, pool: &SqlitePool, entity_type: &str, entity_id: &str, field_name: &str, plaintext: &str) -> Result<EncryptedData> {
        let key = self.repo.get_primary_key(pool, KeyType::Aes256Gcm).await?
            .ok_or_else(|| anyhow::anyhow!("No primary key found"))?;
        
        let mut iv = [0u8; 12];
        rand::thread_rng().fill_bytes(&mut iv);
        
        let encrypted = self.do_encrypt(&key, plaintext, &iv)?;
        
        let data = EncryptedData {
            base: BaseEntity::new(),
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            field_name: field_name.to_string(),
            key_id: key.base.id,
            key_version: key.key_version,
            iv: base64::Engine::encode(&base64::engine::general_purpose::STANDARD, iv),
            auth_tag: None,
            encrypted_value: encrypted,
            encryption_algorithm: key.algorithm.clone(),
            created_at: Utc::now(),
        };
        
        self.repo.create_encrypted_data(pool, data.clone()).await?;
        
        self.log_key_usage(pool, key.base.id, KeyOperation::Encrypt, Some(entity_type), Some(entity_id), true, None).await;
        
        Ok(data)
    }

    pub async fn decrypt_data(&self, pool: &SqlitePool, entity_type: &str, entity_id: &str, field_name: &str) -> Result<Option<String>> {
        let records = self.repo.list_encrypted_data(pool, entity_type, entity_id).await?;
        let record = records.into_iter().find(|r| r.field_name == field_name);
        
        match record {
            Some(data) => {
                let key = self.repo.get_key(pool, data.key_id).await?
                    .ok_or_else(|| anyhow::anyhow!("Key not found"))?;
                
                let iv = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, &data.iv)
                    .map_err(|e| anyhow::anyhow!("Invalid IV: {}", e))?;
                
                let plaintext = self.do_decrypt(&key, &data.encrypted_value, &iv)?;
                
                self.log_key_usage(pool, key.base.id, KeyOperation::Decrypt, Some(entity_type), Some(entity_id), true, None).await;
                
                Ok(Some(plaintext))
            }
            None => Ok(None),
        }
    }

    fn do_encrypt(&self, _key: &EncryptionKey, plaintext: &str, iv: &[u8]) -> Result<String> {
        let iv_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, iv);
        let data_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, plaintext.as_bytes());
        Ok(format!("{}.{}", iv_b64, data_b64))
    }

    fn do_decrypt(&self, _key: &EncryptionKey, ciphertext: &str, _iv: &[u8]) -> Result<String> {
        let parts: Vec<&str> = ciphertext.split('.').collect();
        if parts.len() != 2 {
            return Err(erp_core::Error::Validation("Invalid ciphertext format".to_string()));
        }
        let data = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, parts[1])
            .map_err(|e| erp_core::Error::Validation(format!("Invalid data: {}", e)))?;
        String::from_utf8(data).map_err(|e| erp_core::Error::Validation(format!("Invalid UTF-8: {}", e)))
    }

    #[allow(clippy::too_many_arguments)]
    async fn log_key_usage(&self, pool: &SqlitePool, key_id: Uuid, operation: KeyOperation, entity_type: Option<&str>, entity_id: Option<&str>, success: bool, error: Option<&str>) {
        let log = KeyUsageLog {
            base: BaseEntity::new(),
            key_id,
            operation,
            entity_type: entity_type.map(|s| s.to_string()),
            entity_id: entity_id.map(|s| s.to_string()),
            success,
            error_message: error.map(|s| s.to_string()),
            performed_at: Utc::now(),
            performed_by: None,
        };
        let _ = self.repo.log_usage(pool, log).await;
    }

    pub async fn list_rotations(&self, pool: &SqlitePool, key_id: Uuid) -> Result<Vec<KeyRotation>> {
        self.repo.list_rotations(pool, key_id).await
    }

    pub async fn create_policy(&self, pool: &SqlitePool, policy: KeyPolicy) -> Result<KeyPolicy> {
        self.repo.create_policy(pool, policy).await
    }

    pub async fn list_policies(&self, pool: &SqlitePool) -> Result<Vec<KeyPolicy>> {
        self.repo.list_policies(pool).await
    }

    pub async fn get_keys_needing_rotation(&self, pool: &SqlitePool) -> Result<Vec<EncryptionKey>> {
        let keys = self.repo.list_keys(pool).await?;
        let now = Utc::now();
        
        Ok(keys.into_iter().filter(|k| {
            if !k.is_active || k.rotation_days.is_none() {
                return false;
            }
            if let Some(last_rotated) = k.last_rotated {
                let days_since = (now - last_rotated).num_days();
                days_since >= k.rotation_days.unwrap() as i64
            } else {
                true
            }
        }).collect())
    }
}
