use axum::{
    extract::{Path, State},
    routing::{delete, get, post, put},
    Json, Router,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::BaseEntity;
use erp_features::{FeatureFlagService, FeatureFlag, FeatureFlagOverride, FlagTargetType, FlagEvaluationContext};

#[derive(Serialize)]
pub struct FeatureFlagResponse {
    pub id: Uuid,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub rollout_percentage: i32,
}

impl From<FeatureFlag> for FeatureFlagResponse {
    fn from(f: FeatureFlag) -> Self {
        Self {
            id: f.base.id,
            key: f.key,
            name: f.name,
            description: f.description,
            enabled: f.enabled,
            rollout_percentage: f.rollout_percentage,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateFeatureFlagRequest {
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub rollout_percentage: Option<i32>,
}

pub async fn list_flags(State(state): State<AppState>) -> ApiResult<Json<Vec<FeatureFlagResponse>>> {
    let service = FeatureFlagService::new();
    let flags = service.list_flags(&state.pool).await?;
    Ok(Json(flags.into_iter().map(FeatureFlagResponse::from).collect()))
}

pub async fn create_flag(
    State(state): State<AppState>,
    Json(req): Json<CreateFeatureFlagRequest>,
) -> ApiResult<Json<FeatureFlagResponse>> {
    let service = FeatureFlagService::new();
    let flag = FeatureFlag {
        base: BaseEntity::new(),
        key: req.key,
        name: req.name,
        description: req.description,
        enabled: true,
        rollout_percentage: req.rollout_percentage.unwrap_or(100),
        target_type: FlagTargetType::All,
        target_ids: None,
        start_time: None,
        end_time: None,
        prerequisites: None,
        variants: None,
        default_variant: None,
        is_system: false,
    };
    let created = service.create_flag(&state.pool, flag).await?;
    Ok(Json(FeatureFlagResponse::from(created)))
}

pub async fn get_flag(
    State(state): State<AppState>,
    Path(key): Path<String>,
) -> ApiResult<Json<FeatureFlagResponse>> {
    let service = FeatureFlagService::new();
    let flag = service.get_flag(&state.pool, &key).await?
        .ok_or_else(|| anyhow::anyhow!("Flag not found"))?;
    Ok(Json(FeatureFlagResponse::from(flag)))
}

#[derive(Deserialize)]
pub struct ToggleFlagRequest {
    pub enabled: bool,
}

pub async fn toggle_flag(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ToggleFlagRequest>,
) -> ApiResult<Json<FeatureFlagResponse>> {
    let service = FeatureFlagService::new();
    let flag = service.toggle_flag(&state.pool, id, req.enabled).await?;
    Ok(Json(FeatureFlagResponse::from(flag)))
}

pub async fn delete_flag(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = FeatureFlagService::new();
    service.delete_flag(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "deleted" })))
}

#[derive(Deserialize)]
pub struct EvaluateFlagRequest {
    pub user_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct EvaluateFlagResponse {
    pub enabled: bool,
}

pub async fn evaluate_flag(
    State(state): State<AppState>,
    Path(key): Path<String>,
    Json(req): Json<EvaluateFlagRequest>,
) -> ApiResult<Json<EvaluateFlagResponse>> {
    let service = FeatureFlagService::new();
    let context = FlagEvaluationContext {
        user_id: req.user_id,
        ..Default::default()
    };
    let enabled = service.is_enabled(&state.pool, &key, &context).await?;
    Ok(Json(EvaluateFlagResponse { enabled }))
}

#[derive(Deserialize)]
pub struct CreateOverrideRequest {
    pub target_type: String,
    pub target_id: String,
    pub enabled: bool,
}

pub async fn create_override(
    State(state): State<AppState>,
    Path(flag_id): Path<Uuid>,
    Json(req): Json<CreateOverrideRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = FeatureFlagService::new();
    let override_ = FeatureFlagOverride {
        base: BaseEntity::new(),
        flag_id,
        target_type: erp_features::OverrideTargetType::User,
        target_id: req.target_id,
        enabled: req.enabled,
        variant: None,
    };
    service.create_override(&state.pool, override_).await?;
    Ok(Json(serde_json::json!({ "status": "created" })))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_flags).post(create_flag))
        .route("/:key", get(get_flag))
        .route("/id/:id", delete(delete_flag))
        .route("/:id/toggle", post(toggle_flag))
        .route("/:key/evaluate", post(evaluate_flag))
        .route("/:flag_id/overrides", post(create_override))
}
