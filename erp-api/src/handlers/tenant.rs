use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ApiResult;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    page: Option<i32>,
    page_size: Option<i32>,
    status: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    items: Vec<T>,
    total: i64,
    page: i32,
    page_size: i32,
}

pub async fn list_tenants(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> ApiResult<Json<Vec<erp_tenant::Tenant>>> {
    let status = query.status.and_then(|s| match s.as_str() {
        "Trial" => Some(erp_tenant::TenantStatus::Trial),
        "Active" => Some(erp_tenant::TenantStatus::Active),
        "Suspended" => Some(erp_tenant::TenantStatus::Suspended),
        "Cancelled" => Some(erp_tenant::TenantStatus::Cancelled),
        _ => None,
    });
    let tenants = erp_tenant::TenantService::new(erp_tenant::SqliteTenantRepository::new(state.pool.clone()))
        .list_tenants(status)
        .await?;
    Ok(Json(tenants))
}

pub async fn create_tenant(
    State(state): State<AppState>,
    Json(req): Json<erp_tenant::CreateTenantRequest>,
) -> ApiResult<Json<erp_tenant::Tenant>> {
    let tenant = erp_tenant::TenantService::new(erp_tenant::SqliteTenantRepository::new(state.pool.clone()))
        .create_tenant(req)
        .await?;
    Ok(Json(tenant))
}

pub async fn get_tenant(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_tenant::Tenant>> {
    let tenant = erp_tenant::TenantService::new(erp_tenant::SqliteTenantRepository::new(state.pool.clone()))
        .get_tenant(id)
        .await?
        .ok_or_else(|| crate::error::ApiError::NotFound("Tenant not found".into()))?;
    Ok(Json(tenant))
}

#[derive(Debug, Deserialize)]
pub struct InviteUserRequest {
    pub tenant_id: Uuid,
    pub email: String,
    pub role: erp_tenant::TenantRole,
    pub invited_by: Uuid,
}

pub async fn invite_user(
    State(state): State<AppState>,
    Json(req): Json<InviteUserRequest>,
) -> ApiResult<Json<erp_tenant::TenantInvitation>> {
    let invitation = erp_tenant::TenantService::new(erp_tenant::SqliteTenantRepository::new(state.pool.clone()))
        .invite_user(req.tenant_id, erp_tenant::InviteUserRequest {
            email: req.email,
            role: req.role,
        }, req.invited_by)
        .await?;
    Ok(Json(invitation))
}

#[derive(Debug, Deserialize)]
pub struct AcceptInvitationRequest {
    pub token: String,
    pub user_id: Uuid,
}

pub async fn accept_invitation(
    State(state): State<AppState>,
    Json(req): Json<AcceptInvitationRequest>,
) -> ApiResult<Json<()>> {
    erp_tenant::TenantService::new(erp_tenant::SqliteTenantRepository::new(state.pool.clone()))
        .accept_invitation(&req.token, req.user_id)
        .await?;
    Ok(Json(()))
}

#[derive(Debug, Deserialize)]
pub struct SetFeatureRequest {
    pub tenant_id: Uuid,
    pub feature_key: String,
    pub enabled: bool,
    pub settings: Option<serde_json::Value>,
}

pub async fn set_feature(
    State(state): State<AppState>,
    Json(req): Json<SetFeatureRequest>,
) -> ApiResult<Json<erp_tenant::TenantFeature>> {
    let feature = erp_tenant::TenantService::new(erp_tenant::SqliteTenantRepository::new(state.pool.clone()))
        .set_feature(req.tenant_id, req.feature_key, req.enabled, req.settings)
        .await?;
    Ok(Json(feature))
}

pub async fn get_stats(
    State(state): State<AppState>,
) -> ApiResult<Json<erp_tenant::TenantStats>> {
    let stats = erp_tenant::TenantService::new(erp_tenant::SqliteTenantRepository::new(state.pool.clone()))
        .get_stats()
        .await?;
    Ok(Json(stats))
}

pub fn routes() -> axum::Router<crate::state::AppState> {
    axum::Router::new()
        .route("/", axum::routing::get(list_tenants).post(create_tenant))
        .route("/:id", axum::routing::get(get_tenant))
        .route("/invite", axum::routing::post(invite_user))
        .route("/accept-invitation", axum::routing::post(accept_invitation))
        .route("/features", axum::routing::post(set_feature))
        .route("/stats", axum::routing::get(get_stats))
}
