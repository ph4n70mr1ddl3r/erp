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

pub async fn register(
    State(state): State<AppState>,
    Json(req): Json<RegisterRequest>,
) -> ApiResult<Json<AuthResponse>> {
    let svc = AuthService::new();
    let res = svc.register(&state.pool, req).await?;
    Ok(Json(res))
}

pub async fn login(
    State(state): State<AppState>,
    Json(req): Json<LoginRequest>,
) -> ApiResult<Json<AuthResponse>> {
    let svc = AuthService::new();
    let res = svc.login(&state.pool, req).await?;
    Ok(Json(res))
}

pub async fn me(
    State(state): State<AppState>,
    axum::Extension(AuthUser(user)): axum::Extension<AuthUser>,
) -> ApiResult<Json<UserInfo>> {
    let svc = AuthService::new();
    let id = uuid::Uuid::parse_str(&user.user_id).map_err(|_| erp_core::Error::Unauthorized)?;
    let user = svc.get_user(&state.pool, id).await?;
    Ok(Json(UserInfo {
        id: user.id,
        username: user.username,
        email: user.email,
        full_name: user.full_name,
        role: user.role.as_str().to_string(),
    }))
}

#[derive(Clone)]
pub struct AuthUser(pub erp_auth::jwt::TokenData);

fn extract_token(req: &Request<Body>) -> Option<String> {
    let auth = req.headers().get(AUTHORIZATION)?.to_str().ok()?;
    auth.strip_prefix("Bearer ").map(|s| s.to_string())
}

pub async fn auth_middleware(
    State(_state): State<AppState>,
    req: Request<Body>,
    next: axum::middleware::Next,
) -> Result<Response, StatusCode> {
    let token = extract_token(&req).ok_or(StatusCode::UNAUTHORIZED)?;
    
    let svc = AuthService::new();
    let token_data = svc.validate_token(&token).map_err(|_| StatusCode::UNAUTHORIZED)?;
    
    let mut req = req;
    req.extensions_mut().insert(AuthUser(token_data));
    Ok(next.run(req).await)
}

pub async fn optional_auth_middleware(
    State(_state): State<AppState>,
    mut req: Request<Body>,
    next: axum::middleware::Next,
) -> Response {
    if let Some(token) = extract_token(&req) {
        let svc = AuthService::new();
        if let Ok(token_data) = svc.validate_token(&token) {
            req.extensions_mut().insert(AuthUser(token_data));
        }
    }
    next.run(req).await
}
