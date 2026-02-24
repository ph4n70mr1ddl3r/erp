use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_security::{SecurityService, TwoFactorSetupRequest, TwoFactorVerifyRequest, TwoFactorSetupResponse};
use erp_auth::AuthUser;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct TwoFactorSetupBody {
    pub email: String,
    pub issuer: Option<String>,
}

pub async fn setup_two_factor(
    State(state): State<AppState>,
    axum::Extension(AuthUser(user)): axum::Extension<AuthUser>,
    Json(body): Json<TwoFactorSetupBody>,
) -> ApiResult<Json<TwoFactorSetupResponse>> {
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = SecurityService::new();
    let response = svc.setup_two_factor(
        &state.pool,
        user_id,
        &body.email,
        body.issuer.as_deref().unwrap_or("ERP System"),
    ).await?;
    Ok(Json(response))
}

#[derive(Debug, Deserialize)]
pub struct TwoFactorVerifyBody {
    pub code: String,
}

pub async fn verify_two_factor(
    State(state): State<AppState>,
    axum::Extension(AuthUser(user)): axum::Extension<AuthUser>,
    Json(body): Json<TwoFactorVerifyBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = SecurityService::new();
    let valid = svc.verify_two_factor(&state.pool, user_id, &body.code).await?;
    Ok(Json(serde_json::json!({ "valid": valid })))
}

pub async fn get_two_factor_status(
    State(state): State<AppState>,
    axum::Extension(AuthUser(user)): axum::Extension<AuthUser>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = SecurityService::new();
    let status = svc.get_two_factor_status(&state.pool, user_id).await?;
    Ok(Json(serde_json::json!({
        "enabled": status.map(|s| s.enabled).unwrap_or(false)
    })))
}

pub async fn disable_two_factor(
    State(state): State<AppState>,
    axum::Extension(AuthUser(user)): axum::Extension<AuthUser>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = SecurityService::new();
    svc.disable_two_factor(&state.pool, user_id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn regenerate_backup_codes(
    State(state): State<AppState>,
    axum::Extension(AuthUser(user)): axum::Extension<AuthUser>,
) -> ApiResult<Json<Vec<String>>> {
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = SecurityService::new();
    let codes = svc.regenerate_backup_codes(&state.pool, user_id).await?;
    Ok(Json(codes))
}

#[derive(Debug, Deserialize)]
pub struct OAuthAuthorizeQuery {
    pub provider: String,
    pub redirect_uri: String,
}

pub async fn get_oauth_authorize_url(
    State(state): State<AppState>,
    Query(query): Query<OAuthAuthorizeQuery>,
) -> ApiResult<Json<erp_security::OAuthAuthorizeUrl>> {
    let svc = SecurityService::new();
    let url = svc.get_oauth_authorize_url(&state.pool, &query.provider, &query.redirect_uri).await?;
    Ok(Json(url))
}

#[derive(Debug, Deserialize)]
pub struct OAuthCallbackQuery {
    pub provider: String,
    pub code: String,
    pub state: String,
    pub redirect_uri: String,
}

#[derive(Debug, Serialize)]
pub struct OAuthCallbackResponse {
    pub user_info: erp_security::OAuthUserInfo,
    pub linked_user_id: Option<Uuid>,
    pub requires_link: bool,
}

pub async fn oauth_callback(
    State(state): State<AppState>,
    Query(query): Query<OAuthCallbackQuery>,
) -> ApiResult<Json<OAuthCallbackResponse>> {
    let svc = SecurityService::new();
    let (user_info, linked_user_id) = svc.handle_oauth_callback(
        &state.pool,
        &query.provider,
        &query.code,
        &query.state,
        &query.redirect_uri,
    ).await?;
    
    Ok(Json(OAuthCallbackResponse {
        user_info,
        linked_user_id,
        requires_link: linked_user_id.is_none(),
    }))
}

#[derive(Debug, Deserialize)]
pub struct LinkOAuthBody {
    pub provider: String,
    pub provider_user_id: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in_seconds: Option<i64>,
}

pub async fn link_oauth_account(
    State(state): State<AppState>,
    axum::Extension(AuthUser(user)): axum::Extension<AuthUser>,
    Json(body): Json<LinkOAuthBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = SecurityService::new();
    
    let expires_at = body.expires_in_seconds.map(|secs| {
        chrono::Utc::now() + chrono::Duration::seconds(secs)
    });
    
    svc.link_oauth_account(
        &state.pool,
        user_id,
        &body.provider,
        &body.provider_user_id,
        &body.access_token,
        body.refresh_token.as_deref(),
        expires_at,
    ).await?;
    
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn get_oauth_connections(
    State(state): State<AppState>,
    axum::Extension(AuthUser(user)): axum::Extension<AuthUser>,
) -> ApiResult<Json<Vec<erp_security::UserOAuthConnection>>> {
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = SecurityService::new();
    let connections = svc.get_user_oauth_connections(&state.pool, user_id).await?;
    Ok(Json(connections))
}

pub async fn unlink_oauth_account(
    State(state): State<AppState>,
    axum::Extension(AuthUser(user)): axum::Extension<AuthUser>,
    Path(connection_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let svc = SecurityService::new();
    svc.unlink_oauth_account(&state.pool, connection_id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn list_oauth_providers(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<erp_security::OAuthProvider>>> {
    let svc = SecurityService::new();
    let providers = svc.list_oauth_providers(&state.pool).await?;
    Ok(Json(providers))
}

pub async fn get_user_sessions(
    State(state): State<AppState>,
    axum::Extension(AuthUser(user)): axum::Extension<AuthUser>,
) -> ApiResult<Json<Vec<erp_security::UserSession>>> {
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = SecurityService::new();
    let sessions = svc.get_user_sessions(&state.pool, user_id).await?;
    Ok(Json(sessions))
}

pub async fn revoke_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let svc = SecurityService::new();
    svc.revoke_session(&state.pool, session_id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn revoke_all_sessions(
    State(state): State<AppState>,
    axum::Extension(AuthUser(user)): axum::Extension<AuthUser>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = Uuid::parse_str(&user.user_id).map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = SecurityService::new();
    svc.revoke_all_sessions(&state.pool, user_id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}
