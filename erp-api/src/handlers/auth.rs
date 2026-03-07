use axum::{
    extract::State,
    Json,
    http::{Request, StatusCode, header::AUTHORIZATION},
    response::Response,
    body::Body,
};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_auth::{AuthService, LoginRequest, RegisterRequest, AuthResponse, UserInfo};
use validator::Validate;

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> ApiResult<Json<AuthResponse>> {
    req.validate().map_err(|e| erp_core::Error::Validation(e.to_string()))?;
    let res = state.auth_svc.register(req).await?;
    Ok(Json(res))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> ApiResult<Json<AuthResponse>> {
    req.validate().map_err(|e| erp_core::Error::Validation(e.to_string()))?;
    let res = state.auth_svc.login(req).await?;
    Ok(Json(res))
}

pub async fn me(
    State(state): State<AppState>,
    axum::Extension(AuthUser(user)): axum::Extension<AuthUser>,
) -> ApiResult<Json<UserInfo>> {
    let id = uuid::Uuid::parse_str(&user.user_id).map_err(|_| erp_core::Error::Unauthorized)?;
    let user = state.auth_svc.get_user(id).await?;
    Ok(Json(UserInfo {
        id: user.base.id,
        username: user.username,
        email: user.email,
        full_name: user.full_name,
        role: user.role.as_str().to_string(),
    }))
}

#[derive(Clone, Debug)]
pub struct AuthUser(pub erp_auth::jwt::TokenData);

impl AuthUser {
    pub fn user_id(&self) -> uuid::Uuid {
        uuid::Uuid::parse_str(&self.0.user_id).unwrap_or_default()
    }
}

fn extract_token(req: &Request<Body>) -> Option<String> {
    let auth = req.headers().get(AUTHORIZATION)?.to_str().ok()?;
    if auth.starts_with("Bearer ") {
        Some(auth[7..].to_string())
    } else {
        None
    }
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    req: Request<Body>,
    next: axum::middleware::Next,
) -> Result<Response, StatusCode> {
    let token = extract_token(&req).ok_or(StatusCode::UNAUTHORIZED)?;
    
    let token_data = state.auth_svc.validate_token(&token).map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    let mut req = req;
    req.extensions_mut().insert(AuthUser(token_data));
    Ok(next.run(req).await)
}

pub async fn optional_auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: axum::middleware::Next,
) -> Response {
    if let Some(token) = extract_token(&req) {
        if let Ok(token_data) = state.auth_svc.validate_token(&token) {
            req.extensions_mut().insert(AuthUser(token_data));
        }
    }
    next.run(req).await
}
