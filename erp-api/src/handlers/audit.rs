use axum::{
    extract::{Query, State},
    Json,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{AuditLog, get_audit_logs, Pagination};

#[derive(Debug, Deserialize)]
pub struct AuditQuery {
    pub entity_type: Option<String>,
    pub entity_id: Option<String>,
    pub page: Option<u32>,
    pub per_page: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct AuditLogResponse {
    pub id: Uuid,
    pub entity_type: String,
    pub entity_id: String,
    pub action: String,
    pub old_values: Option<serde_json::Value>,
    pub new_values: Option<serde_json::Value>,
    pub user_id: Option<Uuid>,
    pub username: Option<String>,
    pub created_at: String,
}

impl From<AuditLog> for AuditLogResponse {
    fn from(log: AuditLog) -> Self {
        Self {
            id: log.id,
            entity_type: log.entity_type,
            entity_id: log.entity_id,
            action: format!("{:?}", log.action),
            old_values: log.old_values.and_then(|v| serde_json::from_str(&v).ok()),
            new_values: log.new_values.and_then(|v| serde_json::from_str(&v).ok()),
            user_id: log.user_id,
            username: log.username,
            created_at: log.created_at.to_rfc3339(),
        }
    }
}

pub async fn list_audit_logs(
    State(state): State<AppState>,
    Query(query): Query<AuditQuery>,
) -> ApiResult<Json<erp_core::Paginated<AuditLogResponse>>> {
    let pagination = Pagination {
        page: query.page.unwrap_or(1),
        per_page: query.per_page.unwrap_or(50),
    };
    
    let result = get_audit_logs(
        &state.pool,
        query.entity_type.as_deref(),
        query.entity_id.as_deref(),
        pagination.page,
        pagination.per_page,
    ).await?;
    
    Ok(Json(erp_core::Paginated::new(
        result.items.into_iter().map(AuditLogResponse::from).collect(),
        result.total,
        pagination,
    )))
}
