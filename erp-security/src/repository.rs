use async_trait::async_trait;
use chrono::{DateTime, Utc};
use erp_core::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait TwoFactorRepository: Send + Sync {
    async fn find_by_user_id(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Option<TwoFactorSetup>>;
    async fn create(&self, pool: &SqlitePool, setup: TwoFactorSetup) -> Result<TwoFactorSetup>;
    async fn update_enabled(&self, pool: &SqlitePool, id: Uuid, enabled: bool) -> Result<()>;
    async fn update_backup_codes(&self, pool: &SqlitePool, id: Uuid, codes: Vec<String>) -> Result<()>;
    async fn delete(&self, pool: &SqlitePool, user_id: Uuid) -> Result<()>;
}

pub struct SqliteTwoFactorRepository;

#[async_trait]
impl TwoFactorRepository for SqliteTwoFactorRepository {
    async fn find_by_user_id(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Option<TwoFactorSetup>> {
        let row = sqlx::query_as::<_, TwoFactorRow>(
            "SELECT id, user_id, secret, backup_codes, enabled, verified_at, created_at FROM user_two_factor WHERE user_id = ?"
        )
        .bind(user_id.to_string())
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.into_model()))
    }

    async fn create(&self, pool: &SqlitePool, setup: TwoFactorSetup) -> Result<TwoFactorSetup> {
        sqlx::query(
            "INSERT INTO user_two_factor (id, user_id, secret, backup_codes, enabled, verified_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(setup.id.to_string())
        .bind(setup.user_id.to_string())
        .bind(&setup.secret)
        .bind(serde_json::to_string(&setup.backup_codes).unwrap_or_default())
        .bind(setup.enabled as i32)
        .bind(setup.verified_at.map(|d| d.to_rfc3339()))
        .bind(setup.created_at.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(setup)
    }

    async fn update_enabled(&self, pool: &SqlitePool, id: Uuid, enabled: bool) -> Result<()> {
        let verified_at = if enabled { Some(Utc::now().to_rfc3339()) } else { None };
        sqlx::query("UPDATE user_two_factor SET enabled = ?, verified_at = ? WHERE id = ?")
            .bind(enabled as i32)
            .bind(verified_at)
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn update_backup_codes(&self, pool: &SqlitePool, id: Uuid, codes: Vec<String>) -> Result<()> {
        sqlx::query("UPDATE user_two_factor SET backup_codes = ? WHERE id = ?")
            .bind(serde_json::to_string(&codes).unwrap_or_default())
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, user_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM user_two_factor WHERE user_id = ?")
            .bind(user_id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct TwoFactorRow {
    id: String,
    user_id: String,
    secret: String,
    backup_codes: String,
    enabled: i32,
    verified_at: Option<String>,
    created_at: String,
}

impl TwoFactorRow {
    fn into_model(self) -> TwoFactorSetup {
        TwoFactorSetup {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            user_id: Uuid::parse_str(&self.user_id).unwrap_or_default(),
            secret: self.secret,
            backup_codes: serde_json::from_str(&self.backup_codes).unwrap_or_default(),
            enabled: self.enabled != 0,
            verified_at: self.verified_at.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            created_at: DateTime::parse_from_rfc3339(&self.created_at).ok().map(|d| d.with_timezone(&Utc)).unwrap_or_else(Utc::now),
        }
    }
}

#[async_trait]
pub trait OAuthProviderRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<OAuthProvider>;
    async fn find_by_name(&self, pool: &SqlitePool, name: &str) -> Result<OAuthProvider>;
    async fn list(&self, pool: &SqlitePool) -> Result<Vec<OAuthProvider>>;
    async fn create(&self, pool: &SqlitePool, provider: OAuthProvider) -> Result<OAuthProvider>;
    async fn update(&self, pool: &SqlitePool, provider: OAuthProvider) -> Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteOAuthProviderRepository;

#[async_trait]
impl OAuthProviderRepository for SqliteOAuthProviderRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<OAuthProvider> {
        let row = sqlx::query_as::<_, OAuthProviderRow>(
            "SELECT id, name, display_name, client_id, client_secret, authorize_url, token_url, userinfo_url, scope, enabled, created_at FROM oauth_providers WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| erp_core::Error::not_found("OAuthProvider", &id.to_string()))?;

        Ok(row.into_model())
    }

    async fn find_by_name(&self, pool: &SqlitePool, name: &str) -> Result<OAuthProvider> {
        let row = sqlx::query_as::<_, OAuthProviderRow>(
            "SELECT id, name, display_name, client_id, client_secret, authorize_url, token_url, userinfo_url, scope, enabled, created_at FROM oauth_providers WHERE name = ?"
        )
        .bind(name)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| erp_core::Error::not_found("OAuthProvider", name))?;

        Ok(row.into_model())
    }

    async fn list(&self, pool: &SqlitePool) -> Result<Vec<OAuthProvider>> {
        let rows = sqlx::query_as::<_, OAuthProviderRow>(
            "SELECT id, name, display_name, client_id, client_secret, authorize_url, token_url, userinfo_url, scope, enabled, created_at FROM oauth_providers WHERE enabled = 1 ORDER BY display_name"
        )
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into_model()).collect())
    }

    async fn create(&self, pool: &SqlitePool, provider: OAuthProvider) -> Result<OAuthProvider> {
        sqlx::query(
            "INSERT INTO oauth_providers (id, name, display_name, client_id, client_secret, authorize_url, token_url, userinfo_url, scope, enabled, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(provider.id.to_string())
        .bind(&provider.name)
        .bind(&provider.display_name)
        .bind(&provider.client_id)
        .bind(&provider.client_secret)
        .bind(&provider.authorize_url)
        .bind(&provider.token_url)
        .bind(&provider.userinfo_url)
        .bind(&provider.scope)
        .bind(provider.enabled as i32)
        .bind(provider.created_at.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(provider)
    }

    async fn update(&self, pool: &SqlitePool, provider: OAuthProvider) -> Result<()> {
        sqlx::query(
            "UPDATE oauth_providers SET display_name = ?, client_id = ?, client_secret = ?, authorize_url = ?, token_url = ?, userinfo_url = ?, scope = ?, enabled = ? WHERE id = ?"
        )
        .bind(&provider.display_name)
        .bind(&provider.client_id)
        .bind(&provider.client_secret)
        .bind(&provider.authorize_url)
        .bind(&provider.token_url)
        .bind(&provider.userinfo_url)
        .bind(&provider.scope)
        .bind(provider.enabled as i32)
        .bind(provider.id.to_string())
        .execute(pool)
        .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM oauth_providers WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct OAuthProviderRow {
    id: String,
    name: String,
    display_name: String,
    client_id: String,
    client_secret: String,
    authorize_url: String,
    token_url: String,
    userinfo_url: String,
    scope: String,
    enabled: i32,
    created_at: String,
}

impl OAuthProviderRow {
    fn into_model(self) -> OAuthProvider {
        OAuthProvider {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            name: self.name,
            display_name: self.display_name,
            client_id: self.client_id,
            client_secret: self.client_secret,
            authorize_url: self.authorize_url,
            token_url: self.token_url,
            userinfo_url: self.userinfo_url,
            scope: self.scope,
            enabled: self.enabled != 0,
            created_at: DateTime::parse_from_rfc3339(&self.created_at).ok().map(|d| d.with_timezone(&Utc)).unwrap_or_else(Utc::now),
        }
    }
}

#[async_trait]
pub trait UserOAuthConnectionRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<UserOAuthConnection>>;
    async fn find_by_provider_and_id(&self, pool: &SqlitePool, provider_id: Uuid, provider_user_id: &str) -> Result<Option<UserOAuthConnection>>;
    async fn find_by_user(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Vec<UserOAuthConnection>>;
    async fn create(&self, pool: &SqlitePool, conn: UserOAuthConnection) -> Result<UserOAuthConnection>;
    async fn update_tokens(&self, pool: &SqlitePool, id: Uuid, access_token: Option<&str>, refresh_token: Option<&str>, expires_at: Option<DateTime<Utc>>) -> Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteUserOAuthConnectionRepository;

#[async_trait]
impl UserOAuthConnectionRepository for SqliteUserOAuthConnectionRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<UserOAuthConnection>> {
        let row = sqlx::query_as::<_, UserOAuthConnectionRow>(
            "SELECT id, user_id, provider_id, provider_user_id, access_token, refresh_token, token_expires_at, created_at FROM user_oauth_connections WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.into_model()))
    }

    async fn find_by_provider_and_id(&self, pool: &SqlitePool, provider_id: Uuid, provider_user_id: &str) -> Result<Option<UserOAuthConnection>> {
        let row = sqlx::query_as::<_, UserOAuthConnectionRow>(
            "SELECT id, user_id, provider_id, provider_user_id, access_token, refresh_token, token_expires_at, created_at FROM user_oauth_connections WHERE provider_id = ? AND provider_user_id = ?"
        )
        .bind(provider_id.to_string())
        .bind(provider_user_id)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.into_model()))
    }

    async fn find_by_user(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Vec<UserOAuthConnection>> {
        let rows = sqlx::query_as::<_, UserOAuthConnectionRow>(
            "SELECT id, user_id, provider_id, provider_user_id, access_token, refresh_token, token_expires_at, created_at FROM user_oauth_connections WHERE user_id = ?"
        )
        .bind(user_id.to_string())
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into_model()).collect())
    }

    async fn create(&self, pool: &SqlitePool, conn: UserOAuthConnection) -> Result<UserOAuthConnection> {
        sqlx::query(
            "INSERT INTO user_oauth_connections (id, user_id, provider_id, provider_user_id, access_token, refresh_token, token_expires_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(conn.id.to_string())
        .bind(conn.user_id.to_string())
        .bind(conn.provider_id.to_string())
        .bind(&conn.provider_user_id)
        .bind(&conn.access_token)
        .bind(&conn.refresh_token)
        .bind(conn.token_expires_at.map(|d| d.to_rfc3339()))
        .bind(conn.created_at.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(conn)
    }

    async fn update_tokens(&self, pool: &SqlitePool, id: Uuid, access_token: Option<&str>, refresh_token: Option<&str>, expires_at: Option<DateTime<Utc>>) -> Result<()> {
        sqlx::query("UPDATE user_oauth_connections SET access_token = ?, refresh_token = ?, token_expires_at = ? WHERE id = ?")
            .bind(access_token)
            .bind(refresh_token)
            .bind(expires_at.map(|d| d.to_rfc3339()))
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM user_oauth_connections WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct UserOAuthConnectionRow {
    id: String,
    user_id: String,
    provider_id: String,
    provider_user_id: String,
    access_token: Option<String>,
    refresh_token: Option<String>,
    token_expires_at: Option<String>,
    created_at: String,
}

impl UserOAuthConnectionRow {
    fn into_model(self) -> UserOAuthConnection {
        UserOAuthConnection {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            user_id: Uuid::parse_str(&self.user_id).unwrap_or_default(),
            provider_id: Uuid::parse_str(&self.provider_id).unwrap_or_default(),
            provider_user_id: self.provider_user_id,
            access_token: self.access_token,
            refresh_token: self.refresh_token,
            token_expires_at: self.token_expires_at.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            created_at: DateTime::parse_from_rfc3339(&self.created_at).ok().map(|d| d.with_timezone(&Utc)).unwrap_or_else(Utc::now),
        }
    }
}

#[async_trait]
pub trait UserSessionRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, session: UserSession) -> Result<UserSession>;
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<UserSession>>;
    async fn find_by_token(&self, pool: &SqlitePool, token_hash: &str) -> Result<Option<UserSession>>;
    async fn find_by_user(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Vec<UserSession>>;
    async fn update_activity(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn delete_by_user(&self, pool: &SqlitePool, user_id: Uuid) -> Result<()>;
    async fn delete_expired(&self, pool: &SqlitePool) -> Result<u64>;
}

pub struct SqliteUserSessionRepository;

#[async_trait]
impl UserSessionRepository for SqliteUserSessionRepository {
    async fn create(&self, pool: &SqlitePool, session: UserSession) -> Result<UserSession> {
        sqlx::query(
            "INSERT INTO user_sessions (id, user_id, token_hash, ip_address, user_agent, device_type, is_current, last_activity, expires_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(session.id.to_string())
        .bind(session.user_id.to_string())
        .bind(&session.token_hash)
        .bind(&session.ip_address)
        .bind(&session.user_agent)
        .bind(&session.device_type)
        .bind(session.is_current as i32)
        .bind(session.last_activity.to_rfc3339())
        .bind(session.expires_at.to_rfc3339())
        .bind(session.created_at.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(session)
    }

    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<UserSession>> {
        let row = sqlx::query_as::<_, UserSessionRow>(
            "SELECT id, user_id, token_hash, ip_address, user_agent, device_type, is_current, last_activity, expires_at, created_at FROM user_sessions WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.into_model()))
    }

    async fn find_by_token(&self, pool: &SqlitePool, token_hash: &str) -> Result<Option<UserSession>> {
        let row = sqlx::query_as::<_, UserSessionRow>(
            "SELECT id, user_id, token_hash, ip_address, user_agent, device_type, is_current, last_activity, expires_at, created_at FROM user_sessions WHERE token_hash = ?"
        )
        .bind(token_hash)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| r.into_model()))
    }

    async fn find_by_user(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Vec<UserSession>> {
        let rows = sqlx::query_as::<_, UserSessionRow>(
            "SELECT id, user_id, token_hash, ip_address, user_agent, device_type, is_current, last_activity, expires_at, created_at FROM user_sessions WHERE user_id = ? ORDER BY last_activity DESC"
        )
        .bind(user_id.to_string())
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| r.into_model()).collect())
    }

    async fn update_activity(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE user_sessions SET last_activity = ? WHERE id = ?")
            .bind(Utc::now().to_rfc3339())
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM user_sessions WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete_by_user(&self, pool: &SqlitePool, user_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM user_sessions WHERE user_id = ?")
            .bind(user_id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn delete_expired(&self, pool: &SqlitePool) -> Result<u64> {
        let result = sqlx::query("DELETE FROM user_sessions WHERE expires_at < ?")
            .bind(Utc::now().to_rfc3339())
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}

#[derive(sqlx::FromRow)]
struct UserSessionRow {
    id: String,
    user_id: String,
    token_hash: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
    device_type: Option<String>,
    is_current: i32,
    last_activity: String,
    expires_at: String,
    created_at: String,
}

impl UserSessionRow {
    fn into_model(self) -> UserSession {
        UserSession {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            user_id: Uuid::parse_str(&self.user_id).unwrap_or_default(),
            token_hash: self.token_hash,
            ip_address: self.ip_address,
            user_agent: self.user_agent,
            device_type: self.device_type,
            is_current: self.is_current != 0,
            last_activity: DateTime::parse_from_rfc3339(&self.last_activity).ok().map(|d| d.with_timezone(&Utc)).unwrap_or_else(Utc::now),
            expires_at: DateTime::parse_from_rfc3339(&self.expires_at).ok().map(|d| d.with_timezone(&Utc)).unwrap_or_else(Utc::now),
            created_at: DateTime::parse_from_rfc3339(&self.created_at).ok().map(|d| d.with_timezone(&Utc)).unwrap_or_else(Utc::now),
        }
    }
}
