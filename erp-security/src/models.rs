use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoFactorSetup {
    pub id: Uuid,
    pub user_id: Uuid,
    pub secret: String,
    pub backup_codes: Vec<String>,
    pub enabled: bool,
    pub verified_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoFactorSetupRequest {
    pub user_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoFactorSetupResponse {
    pub secret: String,
    pub qr_code_url: String,
    pub backup_codes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoFactorVerifyRequest {
    pub user_id: Uuid,
    pub code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TwoFactorDisableRequest {
    pub user_id: Uuid,
    pub password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthProvider {
    pub id: Uuid,
    pub name: String,
    pub display_name: String,
    pub client_id: String,
    pub client_secret: String,
    pub authorize_url: String,
    pub token_url: String,
    pub userinfo_url: String,
    pub scope: String,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOAuthProviderRequest {
    pub name: String,
    pub display_name: String,
    pub client_id: String,
    pub client_secret: String,
    pub authorize_url: String,
    pub token_url: String,
    pub userinfo_url: String,
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthAuthorizeUrl {
    pub url: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthCallbackRequest {
    pub provider: String,
    pub code: String,
    pub state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthUserInfo {
    pub provider_user_id: String,
    pub email: String,
    pub name: Option<String>,
    pub picture: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserOAuthConnection {
    pub id: Uuid,
    pub user_id: Uuid,
    pub provider_id: Uuid,
    pub provider_user_id: String,
    pub access_token: Option<String>,
    pub refresh_token: Option<String>,
    pub token_expires_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub device_type: Option<String>,
    pub is_current: bool,
    pub last_activity: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiRateLimit {
    pub id: Uuid,
    pub identifier: String,
    pub identifier_type: String,
    pub endpoint_pattern: Option<String>,
    pub requests_per_minute: i32,
    pub requests_per_hour: i32,
    pub requests_per_day: i32,
    pub current_minute_count: i32,
    pub current_hour_count: i32,
    pub current_day_count: i32,
    pub minute_window_start: Option<DateTime<Utc>>,
    pub hour_window_start: Option<DateTime<Utc>>,
    pub day_window_start: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimitCheck {
    pub allowed: bool,
    pub retry_after_seconds: Option<i64>,
    pub remaining_minute: i32,
    pub remaining_hour: i32,
    pub remaining_day: i32,
}
