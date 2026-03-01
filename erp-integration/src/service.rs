use base64::{engine::general_purpose::STANDARD as BASE64, Engine};
use chrono::Utc;
use erp_core::BaseEntity;
use rand::Rng;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct APIKeyService {
    api_key_repo: SqliteAPIKeyRepository,
}

impl Default for APIKeyService {
    fn default() -> Self {
        Self::new()
    }
}

impl APIKeyService {
    pub fn new() -> Self {
        Self {
            api_key_repo: SqliteAPIKeyRepository,
        }
    }

    pub async fn create(
        &self,
        pool: &SqlitePool,
        name: String,
        description: Option<String>,
        user_id: Option<Uuid>,
        scopes: Vec<String>,
        created_by: Uuid,
        expires_at: Option<chrono::DateTime<Utc>>,
    ) -> anyhow::Result<(APIKey, String)> {
        let raw_key = generate_api_key();
        let key_hash = hash_key(&raw_key);
        let key_prefix = raw_key[..8].to_string();
        
        let api_key = APIKey {
            base: BaseEntity::new(),
            name,
            description,
            key_hash,
            key_prefix,
            user_id,
            scopes,
            rate_limit_per_minute: Some(60),
            rate_limit_per_hour: Some(1000),
            rate_limit_per_day: Some(10000),
            allowed_ips: None,
            allowed_origins: None,
            expires_at,
            last_used_at: None,
            usage_count: 0,
            status: APIKeyStatus::Active,
            created_by,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        let created = self.api_key_repo.create(pool, &api_key).await?;
        Ok((created, raw_key))
    }

    pub async fn validate(&self, pool: &SqlitePool, raw_key: &str) -> anyhow::Result<Option<APIKey>> {
        let prefix = &raw_key[..8.min(raw_key.len())];
        
        if let Some(api_key) = self.api_key_repo.get_by_prefix(pool, prefix).await? {
            if api_key.status != APIKeyStatus::Active {
                return Ok(None);
            }
            
            if let Some(expires) = api_key.expires_at {
                if expires < Utc::now() {
                    return Ok(None);
                }
            }
            
            let hash = hash_key(raw_key);
            if hash == api_key.key_hash {
                return Ok(Some(api_key));
            }
        }
        
        Ok(None)
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<APIKey>> {
        self.api_key_repo.get_by_id(pool, id).await
    }

    pub async fn list(&self, pool: &SqlitePool, user_id: Option<Uuid>) -> anyhow::Result<Vec<APIKey>> {
        self.api_key_repo.list(pool, user_id).await
    }

    pub async fn revoke(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        let mut api_key = self.api_key_repo.get_by_id(pool, id).await?
            .ok_or_else(|| anyhow::anyhow!("API key not found"))?;
        
        api_key.status = APIKeyStatus::Revoked;
        self.api_key_repo.update(pool, &api_key).await
    }

    pub async fn record_usage(
        &self,
        pool: &SqlitePool,
        api_key_id: Uuid,
        endpoint: String,
        method: String,
        status_code: i32,
        response_time_ms: i64,
        request_size: i64,
        response_size: i64,
        ip_address: Option<String>,
        user_agent: Option<String>,
    ) -> anyhow::Result<()> {
        let usage = APIKeyUsage {
            id: Uuid::new_v4(),
            api_key_id,
            timestamp: Utc::now(),
            endpoint,
            method,
            status_code,
            response_time_ms,
            request_size,
            response_size,
            ip_address,
            user_agent,
            error_message: None,
        };
        
        self.api_key_repo.record_usage(pool, &usage).await
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.api_key_repo.delete(pool, id).await
    }
}

fn generate_api_key() -> String {
    const CHARSET: &[u8] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut rng = rand::thread_rng();
    
    let prefix: String = (0..8)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    
    let secret: String = (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    
    format!("erp_{}_{}", prefix, secret)
}

fn hash_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    let result = hasher.finalize();
    BASE64.encode(result)
}

pub struct ExternalConnectionService {
    connection_repo: SqliteExternalConnectionRepository,
}

impl Default for ExternalConnectionService {
    fn default() -> Self {
        Self::new()
    }
}

impl ExternalConnectionService {
    pub fn new() -> Self {
        Self {
            connection_repo: SqliteExternalConnectionRepository,
        }
    }

    pub async fn create(
        &self,
        pool: &SqlitePool,
        name: String,
        code: String,
        connection_type: ConnectionType,
        endpoint_url: Option<String>,
        configuration: Option<serde_json::Value>,
        auth_type: AuthType,
        auth_config: Option<serde_json::Value>,
        created_by: Uuid,
    ) -> anyhow::Result<ExternalConnection> {
        let conn = ExternalConnection {
            base: BaseEntity::new(),
            name,
            code,
            connection_type,
            description: None,
            endpoint_url,
            configuration,
            credentials_encrypted: None,
            auth_type,
            auth_config,
            status: ConnectionStatus::Pending,
            last_sync_at: None,
            last_sync_status: None,
            last_error: None,
            sync_interval_minutes: None,
            auto_sync: false,
            created_by,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.connection_repo.create(pool, &conn).await
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ExternalConnection>> {
        self.connection_repo.get_by_id(pool, id).await
    }

    pub async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> anyhow::Result<Option<ExternalConnection>> {
        self.connection_repo.get_by_code(pool, code).await
    }

    pub async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<ExternalConnection>> {
        self.connection_repo.list(pool).await
    }

    pub async fn update(&self, pool: &SqlitePool, conn: &ExternalConnection) -> anyhow::Result<()> {
        self.connection_repo.update(pool, conn).await
    }

    pub async fn test_connection(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<bool> {
        let conn = self.connection_repo.get_by_id(pool, id).await?
            .ok_or_else(|| anyhow::anyhow!("Connection not found"))?;
        
        match &conn.connection_type {
            ConnectionType::REST | ConnectionType::GraphQL => {
                if let Some(url) = &conn.endpoint_url {
                    let client = reqwest::Client::new();
                    let response = client.get(url).timeout(std::time::Duration::from_secs(10)).send().await?;
                    return Ok(response.status().is_success() || response.status().is_redirection());
                }
            }
            _ => {}
        }
        
        Ok(false)
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.connection_repo.delete(pool, id).await
    }
}

pub struct IntegrationFlowService {
    flow_repo: SqliteIntegrationFlowRepository,
}

impl Default for IntegrationFlowService {
    fn default() -> Self {
        Self::new()
    }
}

impl IntegrationFlowService {
    pub fn new() -> Self {
        Self {
            flow_repo: SqliteIntegrationFlowRepository,
        }
    }

    pub async fn create(
        &self,
        pool: &SqlitePool,
        name: String,
        code: String,
        description: Option<String>,
        trigger_type: TriggerType,
        trigger_config: Option<serde_json::Value>,
        steps: serde_json::Value,
        created_by: Uuid,
    ) -> anyhow::Result<IntegrationFlow> {
        let flow = IntegrationFlow {
            base: BaseEntity::new(),
            name,
            code,
            description,
            trigger_type,
            trigger_config,
            steps,
            error_handling: ErrorHandlingStrategy::StopOnError,
            retry_policy: None,
            enabled: true,
            execution_count: 0,
            success_count: 0,
            failure_count: 0,
            last_execution_at: None,
            created_by,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.flow_repo.create(pool, &flow).await
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<IntegrationFlow>> {
        self.flow_repo.get_by_id(pool, id).await
    }

    pub async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<IntegrationFlow>> {
        self.flow_repo.list(pool).await
    }

    pub async fn execute(&self, pool: &SqlitePool, id: Uuid, trigger_data: Option<serde_json::Value>) -> anyhow::Result<FlowExecution> {
        let flow = self.flow_repo.get_by_id(pool, id).await?
            .ok_or_else(|| anyhow::anyhow!("Flow not found"))?;
        
        let execution = FlowExecution {
            base: BaseEntity::new(),
            flow_id: flow.base.id,
            trigger_type: flow.trigger_type.clone(),
            trigger_data,
            status: FlowExecutionStatus::Running,
            started_at: Utc::now(),
            completed_at: None,
            current_step: Some(0),
            total_steps: 0,
            step_results: None,
            error_message: None,
            error_step: None,
            created_at: Utc::now(),
        };
        
        Ok(execution)
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        self.flow_repo.delete(pool, id).await
    }
}
