use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_enterprise::{SearchService, SearchRequest, SearchResponse, IndexRequest, SearchStats};

#[derive(Debug, Deserialize)]
pub struct SearchQuery {
    pub q: String,
    pub entity_types: Option<String>,
    pub limit: Option<i32>,
    pub offset: Option<i32>,
}

pub async fn search(
    State(state): State<AppState>,
    Query(query): Query<SearchQuery>,
) -> ApiResult<Json<SearchResponse>> {
    let svc = SearchService::new();
    let entity_types = query.entity_types.as_ref().map(|s| {
        s.split(',').map(|s| s.trim().to_string()).collect()
    });
    
    let request = SearchRequest {
        query: query.q,
        entity_types,
        limit: query.limit,
        offset: query.offset,
    };
    
    let results = svc.search(&state.pool, request).await?;
    Ok(Json(results))
}

pub async fn index_entity(
    State(state): State<AppState>,
    Json(req): Json<IndexRequest>,
) -> ApiResult<Json<erp_enterprise::SearchIndexEntry>> {
    let svc = SearchService::new();
    let entry = svc.index(&state.pool, req).await?;
    Ok(Json(entry))
}

#[derive(Debug, Deserialize)]
pub struct RemoveFromIndexPath {
    pub entity_type: String,
    pub entity_id: String,
}

pub async fn remove_from_index(
    State(state): State<AppState>,
    Path(path): Path<RemoveFromIndexPath>,
) -> ApiResult<Json<serde_json::Value>> {
    let svc = SearchService::new();
    svc.remove_from_index(&state.pool, &path.entity_type, &path.entity_id).await?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn rebuild_index(
    State(state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let svc = SearchService::new();
    let count = svc.rebuild_index(&state.pool).await?;
    Ok(Json(serde_json::json!({ "deleted_count": count })))
}

pub async fn search_stats(
    State(state): State<AppState>,
) -> ApiResult<Json<SearchStats>> {
    let svc = SearchService::new();
    let stats = svc.get_stats(&state.pool).await?;
    Ok(Json(stats))
}
