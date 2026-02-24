use crate::models::*;
use crate::oauth::OAuthService;
use crate::repository::*;
use crate::totp::TOTP;
use chrono::Utc;
use erp_core::Result;
use sha2::{Digest, Sha256};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SecurityService {
    two_factor_repo: SqliteTwoFactorRepository,
    oauth_provider_repo: SqliteOAuthProviderRepository,
    oauth_connection_repo: SqliteUserOAuthConnectionRepository,
    session_repo: SqliteUserSessionRepository,
    oauth_service: OAuthService,
}

impl SecurityService {
    pub fn new() -> Self {
        Self {
            two_factor_repo: SqliteTwoFactorRepository,
            oauth_provider_repo: SqliteOAuthProviderRepository,
            oauth_connection_repo: SqliteUserOAuthConnectionRepository,
            session_repo: SqliteUserSessionRepository,
            oauth_service: OAuthService::new(),
        }
    }

    pub async fn setup_two_factor(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        email: &str,
        issuer: &str,
    ) -> Result<TwoFactorSetupResponse> {
        if self.two_factor_repo.find_by_user_id(pool, user_id).await?.is_some() {
            return Err(erp_core::Error::Conflict("2FA already setup".to_string()));
        }

        let secret = TOTP::generate_secret();
        let backup_codes = TOTP::generate_backup_codes(10);
        let qr_url = TOTP::generate_qr_code_url(&secret, email, issuer);
        let secret_base32 = TOTP::secret_to_base32(&secret);

        let setup = TwoFactorSetup {
            id: Uuid::new_v4(),
            user_id,
            secret: secret_base32.clone(),
            backup_codes: backup_codes.clone(),
            enabled: false,
            verified_at: None,
            created_at: Utc::now(),
        };

        self.two_factor_repo.create(pool, setup).await?;

        Ok(TwoFactorSetupResponse {
            secret: secret_base32,
            qr_code_url: qr_url,
            backup_codes,
        })
    }

    pub async fn verify_two_factor(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        code: &str,
    ) -> Result<bool> {
        let setup = self
            .two_factor_repo
            .find_by_user_id(pool, user_id)
            .await?
            .ok_or_else(|| erp_core::Error::not_found("2FA setup", &user_id.to_string()))?;

        if setup.enabled {
            return Err(erp_core::Error::Conflict("2FA already enabled".to_string()));
        }

        let secret = TOTP::base32_to_secret(&setup.secret)
            .map_err(|e| erp_core::Error::validation(&e))?;
        let totp = TOTP::new(&secret);

        if totp.verify(code, 1) {
            self.two_factor_repo.update_enabled(pool, setup.id, true).await?;
            return Ok(true);
        }

        if setup.backup_codes.contains(&code.to_string()) {
            let mut remaining_codes: Vec<String> = setup
                .backup_codes
                .into_iter()
                .filter(|c| c != code)
                .collect();
            if remaining_codes.is_empty() {
                remaining_codes = TOTP::generate_backup_codes(10);
            }
            self.two_factor_repo.update_backup_codes(pool, setup.id, remaining_codes).await?;
            self.two_factor_repo.update_enabled(pool, setup.id, true).await?;
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn validate_two_factor_code(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        code: &str,
    ) -> Result<bool> {
        let setup = self
            .two_factor_repo
            .find_by_user_id(pool, user_id)
            .await?
            .ok_or_else(|| erp_core::Error::not_found("2FA setup", &user_id.to_string()))?;

        if !setup.enabled {
            return Ok(true);
        }

        let secret = TOTP::base32_to_secret(&setup.secret)
            .map_err(|e| erp_core::Error::validation(&e))?;
        let totp = TOTP::new(&secret);

        if totp.verify(code, 1) {
            return Ok(true);
        }

        if let Some(pos) = setup.backup_codes.iter().position(|c| c == code) {
            let mut remaining_codes = setup.backup_codes.clone();
            remaining_codes.remove(pos);
            self.two_factor_repo.update_backup_codes(pool, setup.id, remaining_codes).await?;
            return Ok(true);
        }

        Ok(false)
    }

    pub async fn disable_two_factor(&self, pool: &SqlitePool, user_id: Uuid) -> Result<()> {
        self.two_factor_repo.delete(pool, user_id).await
    }

    pub async fn regenerate_backup_codes(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Vec<String>> {
        let setup = self
            .two_factor_repo
            .find_by_user_id(pool, user_id)
            .await?
            .ok_or_else(|| erp_core::Error::not_found("2FA setup", &user_id.to_string()))?;

        let new_codes = TOTP::generate_backup_codes(10);
        self.two_factor_repo.update_backup_codes(pool, setup.id, new_codes.clone()).await?;
        Ok(new_codes)
    }

    pub async fn get_two_factor_status(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Option<TwoFactorSetup>> {
        self.two_factor_repo.find_by_user_id(pool, user_id).await
    }

    pub async fn create_oauth_provider(
        &self,
        pool: &SqlitePool,
        req: CreateOAuthProviderRequest,
    ) -> Result<OAuthProvider> {
        let provider = OAuthProvider {
            id: Uuid::new_v4(),
            name: req.name,
            display_name: req.display_name,
            client_id: req.client_id,
            client_secret: req.client_secret,
            authorize_url: req.authorize_url,
            token_url: req.token_url,
            userinfo_url: req.userinfo_url,
            scope: req.scope.unwrap_or_else(|| "openid email profile".to_string()),
            enabled: true,
            created_at: Utc::now(),
        };

        self.oauth_provider_repo.create(pool, provider.clone()).await
    }

    pub async fn get_oauth_provider(&self, pool: &SqlitePool, name: &str) -> Result<OAuthProvider> {
        self.oauth_provider_repo.find_by_name(pool, name).await
    }

    pub async fn list_oauth_providers(&self, pool: &SqlitePool) -> Result<Vec<OAuthProvider>> {
        self.oauth_provider_repo.list(pool).await
    }

    pub async fn get_oauth_authorize_url(
        &self,
        pool: &SqlitePool,
        provider_name: &str,
        redirect_uri: &str,
    ) -> Result<OAuthAuthorizeUrl> {
        let provider = self.get_oauth_provider(pool, provider_name).await?;
        Ok(self.oauth_service.get_authorize_url(&provider, redirect_uri).await)
    }

    pub async fn handle_oauth_callback(
        &self,
        pool: &SqlitePool,
        provider_name: &str,
        code: &str,
        state: &str,
        redirect_uri: &str,
    ) -> Result<(OAuthUserInfo, Option<Uuid>)> {
        let provider = self.get_oauth_provider(pool, provider_name).await?;
        
        self.oauth_service.validate_state(state).await?;

        let (access_token, refresh_token, expires_at) = self
            .oauth_service
            .exchange_code(&provider, code, redirect_uri)
            .await?;

        let user_info = self.oauth_service.get_user_info(&provider, &access_token).await?;

        let existing = self
            .oauth_connection_repo
            .find_by_provider_and_id(pool, provider.id, &user_info.provider_user_id)
            .await?;

        let connection_id = if let Some(conn) = existing {
            self.oauth_connection_repo
                .update_tokens(pool, conn.id, Some(&access_token), refresh_token.as_deref(), expires_at)
                .await?;
            Some(conn.user_id)
        } else {
            None
        };

        Ok((user_info, connection_id))
    }

    pub async fn link_oauth_account(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        provider_name: &str,
        provider_user_id: &str,
        access_token: &str,
        refresh_token: Option<&str>,
        expires_at: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<UserOAuthConnection> {
        let provider = self.get_oauth_provider(pool, provider_name).await?;

        let conn = UserOAuthConnection {
            id: Uuid::new_v4(),
            user_id,
            provider_id: provider.id,
            provider_user_id: provider_user_id.to_string(),
            access_token: Some(access_token.to_string()),
            refresh_token: refresh_token.map(|s| s.to_string()),
            token_expires_at: expires_at,
            created_at: Utc::now(),
        };

        self.oauth_connection_repo.create(pool, conn).await
    }

    pub async fn get_user_oauth_connections(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
    ) -> Result<Vec<UserOAuthConnection>> {
        self.oauth_connection_repo.find_by_user(pool, user_id).await
    }

    pub async fn unlink_oauth_account(&self, pool: &SqlitePool, connection_id: Uuid) -> Result<()> {
        self.oauth_connection_repo.delete(pool, connection_id).await
    }

    pub async fn create_session(
        &self,
        pool: &SqlitePool,
        user_id: Uuid,
        token: &str,
        ip_address: Option<&str>,
        user_agent: Option<&str>,
        expires_in_hours: i64,
    ) -> Result<UserSession> {
        let token_hash = Self::hash_token(token);
        let now = Utc::now();
        let expires_at = now + chrono::Duration::hours(expires_in_hours);

        let device_type = user_agent.as_ref().map(|ua| {
            if ua.contains("Mobile") || ua.contains("Android") || ua.contains("iPhone") {
                "Mobile"
            } else if ua.contains("Tablet") || ua.contains("iPad") {
                "Tablet"
            } else {
                "Desktop"
            }
        });

        let session = UserSession {
            id: Uuid::new_v4(),
            user_id,
            token_hash,
            ip_address: ip_address.map(|s| s.to_string()),
            user_agent: user_agent.map(|s| s.to_string()),
            device_type: device_type.map(|s| s.to_string()),
            is_current: true,
            last_activity: now,
            expires_at,
            created_at: now,
        };

        self.session_repo.create(pool, session).await
    }

    pub async fn get_user_sessions(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Vec<UserSession>> {
        self.session_repo.find_by_user(pool, user_id).await
    }

    pub async fn revoke_session(&self, pool: &SqlitePool, session_id: Uuid) -> Result<()> {
        self.session_repo.delete(pool, session_id).await
    }

    pub async fn revoke_all_sessions(&self, pool: &SqlitePool, user_id: Uuid) -> Result<()> {
        self.session_repo.delete_by_user(pool, user_id).await
    }

    pub async fn cleanup_expired_sessions(&self, pool: &SqlitePool) -> Result<u64> {
        self.session_repo.delete_expired(pool).await
    }

    fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}
