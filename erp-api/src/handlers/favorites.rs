use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::AppState;
use crate::error::ApiResult;
use crate::handlers::auth::AuthUser;
use erp_favorites::{CreateFavoriteRequest, Favorite, FavoriteService, FavoriteType};

#[derive(Debug, Deserialize)]
pub struct FavoritesQuery {
    pub favorite_type: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct FavoriteResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub favorite_type: String,
    pub entity_id: Option<Uuid>,
    pub entity_name: String,
    pub entity_code: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}

impl From<Favorite> for FavoriteResponse {
    fn from(fav: Favorite) -> Self {
        Self {
            id: fav.base.id,
            user_id: fav.user_id,
            favorite_type: fav.favorite_type.to_string(),
            entity_id: fav.entity_id,
            entity_name: fav.entity_name,
            entity_code: fav.entity_code,
            notes: fav.notes,
            created_at: fav.base.created_at.to_rfc3339(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateFavoriteBody {
    pub favorite_type: String,
    pub entity_id: Option<Uuid>,
    pub entity_name: String,
    pub entity_code: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ToggleFavoriteBody {
    pub favorite_type: String,
    pub entity_id: Uuid,
    pub entity_name: String,
    pub entity_code: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ToggleFavoriteResponse {
    pub favorite: Option<FavoriteResponse>,
    pub is_favorited: bool,
}

#[derive(Debug, Serialize)]
pub struct FavoriteListResponse {
    pub items: Vec<FavoriteResponse>,
    pub total: i64,
}

pub async fn list_favorites(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Query(query): Query<FavoritesQuery>,
) -> ApiResult<Json<FavoriteListResponse>> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).unwrap_or_default();
    let svc = FavoriteService::new();

    if let Some(ft) = &query.favorite_type {
        let favorite_type: FavoriteType = ft.parse().unwrap_or(FavoriteType::Page);
        let items = svc.list_for_user_by_type(&state.pool, user_id, &favorite_type).await?;
        let total = items.len() as i64;
        return Ok(Json(FavoriteListResponse {
            items: items.into_iter().map(FavoriteResponse::from).collect(),
            total,
        }));
    }

    let result = svc.list_for_user(&state.pool, user_id).await?;
    Ok(Json(FavoriteListResponse {
        items: result.items.into_iter().map(FavoriteResponse::from).collect(),
        total: result.total,
    }))
}

pub async fn create_favorite(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(body): Json<CreateFavoriteBody>,
) -> ApiResult<Json<FavoriteResponse>> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).unwrap_or_default();
    let svc = FavoriteService::new();

    let req = CreateFavoriteRequest {
        favorite_type: body.favorite_type,
        entity_id: body.entity_id,
        entity_name: body.entity_name,
        entity_code: body.entity_code,
        notes: body.notes,
    };

    let favorite = svc.create(&state.pool, req, user_id).await?;
    Ok(Json(FavoriteResponse::from(favorite)))
}

pub async fn get_favorite(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<FavoriteResponse>> {
    let svc = FavoriteService::new();
    let favorite = svc.get(&state.pool, id).await?;
    Ok(Json(FavoriteResponse::from(favorite)))
}

pub async fn delete_favorite(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).unwrap_or_default();
    let svc = FavoriteService::new();
    svc.delete(&state.pool, id, user_id).await?;
    Ok(Json(serde_json::json!({ "status": "deleted" })))
}

pub async fn toggle_favorite(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Json(body): Json<ToggleFavoriteBody>,
) -> ApiResult<Json<ToggleFavoriteResponse>> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).unwrap_or_default();
    let svc = FavoriteService::new();

    let favorite_type: FavoriteType = body.favorite_type.parse()
        .map_err(|e: String| erp_core::Error::validation(&e))?;

    let (favorite, is_favorited) = svc.toggle_favorite(
        &state.pool,
        user_id,
        &favorite_type,
        body.entity_id,
        body.entity_name,
        body.entity_code,
    ).await?;

    Ok(Json(ToggleFavoriteResponse {
        favorite: if is_favorited { Some(FavoriteResponse::from(favorite)) } else { None },
        is_favorited,
    }))
}

#[derive(Debug, Serialize)]
pub struct IsFavoriteResponse {
    pub is_favorited: bool,
}

pub async fn is_favorite(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
    Path((favorite_type, entity_id)): Path<(String, Uuid)>,
) -> ApiResult<Json<IsFavoriteResponse>> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).unwrap_or_default();
    let svc = FavoriteService::new();

    let fav_type: FavoriteType = favorite_type.parse()
        .map_err(|e: String| erp_core::Error::validation(&e))?;

    let is_favorited = svc.is_favorite(&state.pool, user_id, &fav_type, entity_id).await?;
    Ok(Json(IsFavoriteResponse { is_favorited }))
}

#[derive(Debug, Serialize)]
pub struct FavoriteCountResponse {
    pub count: i64,
}

pub async fn favorite_count(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> ApiResult<Json<FavoriteCountResponse>> {
    let user_id = Uuid::parse_str(&auth_user.0.user_id).unwrap_or_default();
    let svc = FavoriteService::new();
    let count = svc.count_for_user(&state.pool, user_id).await?;
    Ok(Json(FavoriteCountResponse { count }))
}

pub fn routes() -> axum::Router<crate::db::AppState> {
    axum::Router::new()
        .route("/", axum::routing::get(list_favorites).post(create_favorite))
        .route("/count", axum::routing::get(favorite_count))
        .route("/toggle", axum::routing::post(toggle_favorite))
        .route("/:id", axum::routing::get(get_favorite).delete(delete_favorite))
        .route("/check/:favorite_type/:entity_id", axum::routing::get(is_favorite))
}
