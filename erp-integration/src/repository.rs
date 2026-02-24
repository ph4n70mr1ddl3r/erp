use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait APIKeyRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, api_key: &APIKey) -> anyhow::Result<APIKey>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<APIKey>>;
    async fn get_by_prefix(&self, pool: &SqlitePool, prefix: &str) -> anyhow::Result<Option<APIKey>>;
    async fn list(&self, pool: &SqlitePool, user_id: Option<Uuid>) -> anyhow::Result<Vec<APIKey>>;
    async fn update(&self, pool: &SqlitePool, api_key: &APIKey) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
    async fn record_usage(&self, pool: &SqlitePool, usage: &APIKeyUsage) -> anyhow::Result<()>;
}

pub struct SqliteAPIKeyRepository;

#[async_trait]
impl APIKeyRepository for SqliteAPIKeyRepository {
    async fn create(&self, pool: &SqlitePool, api_key: &APIKey) -> anyhow::Result<APIKey> {
        let scopes_json = serde_json::to_string(&api_key.scopes)?;
        let now = Utc::now();
        
        sqlx::query_as::<_, APIKey>(
            r#"
            INSERT INTO api_keys (
                id, name, description, key_hash, key_prefix, user_id, scopes,
                rate_limit_per_minute, rate_limit_per_hour, rate_limit_per_day,
                allowed_ips, allowed_origins, expires_at, last_used_at, usage_count,
                status, created_by, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(api_key.base.id)
        .bind(&api_key.name)
        .bind(&api_key.description)
        .bind(&api_key.key_hash)
        .bind(&api_key.key_prefix)
        .bind(api_key.user_id)
        .bind(&scopes_json)
        .bind(api_key.rate_limit_per_minute)
        .bind(api_key.rate_limit_per_hour)
        .bind(api_key.rate_limit_per_day)
        .bind(&api_key.allowed_ips)
        .bind(&api_key.allowed_origins)
        .bind(api_key.expires_at)
        .bind(api_key.last_used_at)
        .bind(api_key.usage_count)
        .bind(&api_key.status)
        .bind(api_key.created_by)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<APIKey>> {
        sqlx::query_as::<_, APIKey>("SELECT * FROM api_keys WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn get_by_prefix(&self, pool: &SqlitePool, prefix: &str) -> anyhow::Result<Option<APIKey>> {
        sqlx::query_as::<_, APIKey>("SELECT * FROM api_keys WHERE key_prefix = ?")
            .bind(prefix)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list(&self, pool: &SqlitePool, user_id: Option<Uuid>) -> anyhow::Result<Vec<APIKey>> {
        match user_id {
            Some(uid) => sqlx::query_as::<_, APIKey>("SELECT * FROM api_keys WHERE user_id = ? ORDER BY created_at DESC")
                .bind(uid)
                .fetch_all(pool)
                .await
                .map_err(Into::into),
            None => sqlx::query_as::<_, APIKey>("SELECT * FROM api_keys ORDER BY created_at DESC")
                .fetch_all(pool)
                .await
                .map_err(Into::into),
        }
    }

    async fn update(&self, pool: &SqlitePool, api_key: &APIKey) -> anyhow::Result<()> {
        let scopes_json = serde_json::to_string(&api_key.scopes)?;
        let now = Utc::now();
        
        sqlx::query(
            r#"
            UPDATE api_keys SET
                name = ?, description = ?, scopes = ?, rate_limit_per_minute = ?,
                rate_limit_per_hour = ?, rate_limit_per_day = ?, allowed_ips = ?,
                allowed_origins = ?, expires_at = ?, status = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&api_key.name)
        .bind(&api_key.description)
        .bind(&scopes_json)
        .bind(api_key.rate_limit_per_minute)
        .bind(api_key.rate_limit_per_hour)
        .bind(api_key.rate_limit_per_day)
        .bind(&api_key.allowed_ips)
        .bind(&api_key.allowed_origins)
        .bind(api_key.expires_at)
        .bind(&api_key.status)
        .bind(now)
        .bind(api_key.base.id)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM api_keys WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn record_usage(&self, pool: &SqlitePool, usage: &APIKeyUsage) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO api_key_usage (
                id, api_key_id, timestamp, endpoint, method, status_code,
                response_time_ms, request_size, response_size, ip_address,
                user_agent, error_message
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(usage.id)
        .bind(usage.api_key_id)
        .bind(usage.timestamp)
        .bind(&usage.endpoint)
        .bind(&usage.method)
        .bind(usage.status_code)
        .bind(usage.response_time_ms)
        .bind(usage.request_size)
        .bind(usage.response_size)
        .bind(&usage.ip_address)
        .bind(&usage.user_agent)
        .bind(&usage.error_message)
        .execute(pool)
        .await?;
        
        sqlx::query(
            "UPDATE api_keys SET usage_count = usage_count + 1, last_used_at = ? WHERE id = ?"
        )
        .bind(usage.timestamp)
        .bind(usage.api_key_id)
        .execute(pool)
        .await?;
        
        Ok(())
    }
}

#[async_trait]
pub trait ExternalConnectionRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, conn: &ExternalConnection) -> anyhow::Result<ExternalConnection>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ExternalConnection>>;
    async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> anyhow::Result<Option<ExternalConnection>>;
    async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<ExternalConnection>>;
    async fn update(&self, pool: &SqlitePool, conn: &ExternalConnection) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteExternalConnectionRepository;

#[async_trait]
impl ExternalConnectionRepository for SqliteExternalConnectionRepository {
    async fn create(&self, pool: &SqlitePool, conn: &ExternalConnection) -> anyhow::Result<ExternalConnection> {
        let now = Utc::now();
        sqlx::query_as::<_, ExternalConnection>(
            r#"
            INSERT INTO external_connections (
                id, name, code, connection_type, description, endpoint_url,
                configuration, credentials_encrypted, auth_type, auth_config,
                status, last_sync_at, last_sync_status, last_error,
                sync_interval_minutes, auto_sync, created_by, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(conn.base.id)
        .bind(&conn.name)
        .bind(&conn.code)
        .bind(&conn.connection_type)
        .bind(&conn.description)
        .bind(&conn.endpoint_url)
        .bind(&conn.configuration)
        .bind(&conn.credentials_encrypted)
        .bind(&conn.auth_type)
        .bind(&conn.auth_config)
        .bind(&conn.status)
        .bind(conn.last_sync_at)
        .bind(&conn.last_sync_status)
        .bind(&conn.last_error)
        .bind(conn.sync_interval_minutes)
        .bind(conn.auto_sync)
        .bind(conn.created_by)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<ExternalConnection>> {
        sqlx::query_as::<_, ExternalConnection>("SELECT * FROM external_connections WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn get_by_code(&self, pool: &SqlitePool, code: &str) -> anyhow::Result<Option<ExternalConnection>> {
        sqlx::query_as::<_, ExternalConnection>("SELECT * FROM external_connections WHERE code = ?")
            .bind(code)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<ExternalConnection>> {
        sqlx::query_as::<_, ExternalConnection>("SELECT * FROM external_connections ORDER BY name")
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, conn: &ExternalConnection) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query(
            r#"
            UPDATE external_connections SET
                name = ?, description = ?, endpoint_url = ?, configuration = ?,
                credentials_encrypted = ?, auth_type = ?, auth_config = ?, status = ?,
                last_sync_at = ?, last_sync_status = ?, last_error = ?,
                sync_interval_minutes = ?, auto_sync = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&conn.name)
        .bind(&conn.description)
        .bind(&conn.endpoint_url)
        .bind(&conn.configuration)
        .bind(&conn.credentials_encrypted)
        .bind(&conn.auth_type)
        .bind(&conn.auth_config)
        .bind(&conn.status)
        .bind(conn.last_sync_at)
        .bind(&conn.last_sync_status)
        .bind(&conn.last_error)
        .bind(conn.sync_interval_minutes)
        .bind(conn.auto_sync)
        .bind(now)
        .bind(conn.base.id)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM external_connections WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait IntegrationFlowRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, flow: &IntegrationFlow) -> anyhow::Result<IntegrationFlow>;
    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<IntegrationFlow>>;
    async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<IntegrationFlow>>;
    async fn update(&self, pool: &SqlitePool, flow: &IntegrationFlow) -> anyhow::Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()>;
}

pub struct SqliteIntegrationFlowRepository;

#[async_trait]
impl IntegrationFlowRepository for SqliteIntegrationFlowRepository {
    async fn create(&self, pool: &SqlitePool, flow: &IntegrationFlow) -> anyhow::Result<IntegrationFlow> {
        let now = Utc::now();
        sqlx::query_as::<_, IntegrationFlow>(
            r#"
            INSERT INTO integration_flows (
                id, name, code, description, trigger_type, trigger_config,
                steps, error_handling, retry_policy, enabled, execution_count,
                success_count, failure_count, last_execution_at, created_by, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            RETURNING *
            "#,
        )
        .bind(flow.base.id)
        .bind(&flow.name)
        .bind(&flow.code)
        .bind(&flow.description)
        .bind(&flow.trigger_type)
        .bind(&flow.trigger_config)
        .bind(&flow.steps)
        .bind(&flow.error_handling)
        .bind(&flow.retry_policy)
        .bind(flow.enabled)
        .bind(flow.execution_count)
        .bind(flow.success_count)
        .bind(flow.failure_count)
        .bind(flow.last_execution_at)
        .bind(flow.created_by)
        .bind(now)
        .bind(now)
        .fetch_one(pool)
        .await
        .map_err(Into::into)
    }

    async fn get_by_id(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<Option<IntegrationFlow>> {
        sqlx::query_as::<_, IntegrationFlow>("SELECT * FROM integration_flows WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await
            .map_err(Into::into)
    }

    async fn list(&self, pool: &SqlitePool) -> anyhow::Result<Vec<IntegrationFlow>> {
        sqlx::query_as::<_, IntegrationFlow>("SELECT * FROM integration_flows ORDER BY name")
            .fetch_all(pool)
            .await
            .map_err(Into::into)
    }

    async fn update(&self, pool: &SqlitePool, flow: &IntegrationFlow) -> anyhow::Result<()> {
        let now = Utc::now();
        sqlx::query(
            r#"
            UPDATE integration_flows SET
                name = ?, description = ?, trigger_type = ?, trigger_config = ?,
                steps = ?, error_handling = ?, retry_policy = ?, enabled = ?,
                execution_count = ?, success_count = ?, failure_count = ?,
                last_execution_at = ?, updated_at = ?
            WHERE id = ?
            "#,
        )
        .bind(&flow.name)
        .bind(&flow.description)
        .bind(&flow.trigger_type)
        .bind(&flow.trigger_config)
        .bind(&flow.steps)
        .bind(&flow.error_handling)
        .bind(&flow.retry_policy)
        .bind(flow.enabled)
        .bind(flow.execution_count)
        .bind(flow.success_count)
        .bind(flow.failure_count)
        .bind(flow.last_execution_at)
        .bind(now)
        .bind(flow.base.id)
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM integration_flows WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
