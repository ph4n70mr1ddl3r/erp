use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use sqlx::Row;
use crate::db::AppState;
use crate::error::{ApiError, ApiResult};
use erp_auth::get_default_permissions;

fn parse_uuid(s: &str) -> ApiResult<Uuid> {
    Uuid::parse_str(s).map_err(|e| ApiError(erp_core::Error::Validation(format!("Invalid UUID: {}", e))))
}

#[derive(Serialize)]
pub struct RoleResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub is_system: bool,
    pub is_active: bool,
}

#[derive(Deserialize)]
pub struct CreateRoleRequest {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub parent_role_id: Option<Uuid>,
}

pub async fn list_roles(State(state): State<AppState>) -> ApiResult<Json<Vec<RoleResponse>>> {
    let rows = sqlx::query(
        "SELECT id, name, code, description, is_system, is_active FROM custom_roles WHERE is_active = 1 ORDER BY name"
    ).fetch_all(&state.pool).await?;
    
    let mut roles = Vec::new();
    for r in rows {
        roles.push(RoleResponse {
            id: parse_uuid(r.get::<String, _>("id").as_str())?,
            name: r.get::<String, _>("name"),
            code: r.get::<String, _>("code"),
            description: r.get::<Option<String>, _>("description"),
            is_system: r.get::<i32, _>("is_system") == 1,
            is_active: r.get::<i32, _>("is_active") == 1,
        });
    }
    Ok(Json(roles))
}

pub async fn create_role(
    State(state): State<AppState>,
    Json(req): Json<CreateRoleRequest>,
) -> ApiResult<Json<RoleResponse>> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        "INSERT INTO custom_roles (id, name, code, description, parent_role_id, is_system, is_active, created_at, updated_at) VALUES (?, ?, ?, ?, ?, 0, 1, ?, ?)"
    )
    .bind(id.to_string())
    .bind(&req.name)
    .bind(&req.code)
    .bind(&req.description)
    .bind(req.parent_role_id.map(|id| id.to_string()))
    .bind(&now)
    .bind(&now)
    .execute(&state.pool).await?;
    
    Ok(Json(RoleResponse {
        id,
        name: req.name,
        code: req.code,
        description: req.description,
        is_system: false,
        is_active: true,
    }))
}

pub async fn delete_role(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    sqlx::query("UPDATE custom_roles SET is_active = 0 WHERE id = ? AND is_system = 0")
        .bind(id.to_string())
        .execute(&state.pool).await?;
    Ok(Json(serde_json::json!({ "status": "deleted" })))
}

#[derive(Serialize)]
pub struct PermissionResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub module: String,
    pub resource: String,
    pub action: String,
}

pub async fn list_permissions(State(state): State<AppState>) -> ApiResult<Json<Vec<PermissionResponse>>> {
    let rows = sqlx::query(
        "SELECT id, code, name, module, resource, action FROM permissions ORDER BY module, resource, action"
    ).fetch_all(&state.pool).await?;
    
    if rows.is_empty() {
        let default_perms = get_default_permissions();
        for perm in &default_perms {
            let id = Uuid::new_v4();
            let now = chrono::Utc::now().to_rfc3339();
            let _ = sqlx::query(
                "INSERT OR IGNORE INTO permissions (id, code, name, description, module, resource, action, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(id.to_string())
            .bind(&perm.code)
            .bind(&perm.name)
            .bind(&perm.description)
            .bind(&perm.module)
            .bind(&perm.resource)
            .bind(&perm.action)
            .bind(&now)
            .execute(&state.pool).await;
        }
        
        let rows = sqlx::query(
            "SELECT id, code, name, module, resource, action FROM permissions ORDER BY module, resource, action"
        ).fetch_all(&state.pool).await?;
        
        let mut perms = Vec::new();
        for r in rows {
            perms.push(PermissionResponse {
                id: parse_uuid(r.get::<String, _>("id").as_str())?,
                code: r.get::<String, _>("code"),
                name: r.get::<String, _>("name"),
                module: r.get::<String, _>("module"),
                resource: r.get::<String, _>("resource"),
                action: r.get::<String, _>("action"),
            });
        }
        return Ok(Json(perms));
    }
    
    let mut perms = Vec::new();
    for r in rows {
        perms.push(PermissionResponse {
            id: parse_uuid(r.get::<String, _>("id").as_str())?,
            code: r.get::<String, _>("code"),
            name: r.get::<String, _>("name"),
            module: r.get::<String, _>("module"),
            resource: r.get::<String, _>("resource"),
            action: r.get::<String, _>("action"),
        });
    }
    Ok(Json(perms))
}

#[derive(Deserialize)]
pub struct AssignPermissionRequest {
    pub permission_id: Uuid,
}

pub async fn assign_permission(
    State(state): State<AppState>,
    Path(role_id): Path<Uuid>,
    Json(req): Json<AssignPermissionRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        "INSERT OR IGNORE INTO role_permissions (id, role_id, permission_id, granted_at) VALUES (?, ?, ?, ?)"
    )
    .bind(id.to_string())
    .bind(role_id.to_string())
    .bind(req.permission_id.to_string())
    .bind(&now)
    .execute(&state.pool).await?;
    
    Ok(Json(serde_json::json!({ "status": "assigned" })))
}

pub async fn revoke_permission(
    State(state): State<AppState>,
    Path((role_id, permission_id)): Path<(Uuid, Uuid)>,
) -> ApiResult<Json<serde_json::Value>> {
    sqlx::query("DELETE FROM role_permissions WHERE role_id = ? AND permission_id = ?")
        .bind(role_id.to_string())
        .bind(permission_id.to_string())
        .execute(&state.pool).await?;
    
    Ok(Json(serde_json::json!({ "status": "revoked" })))
}

#[derive(Serialize)]
pub struct RolePermissionResponse {
    pub permission_id: Uuid,
    pub permission_code: String,
    pub permission_name: String,
}

pub async fn list_role_permissions(
    State(state): State<AppState>,
    Path(role_id): Path<Uuid>,
) -> ApiResult<Json<Vec<RolePermissionResponse>>> {
    let rows = sqlx::query(
        r#"SELECT rp.permission_id, p.code, p.name 
           FROM role_permissions rp 
           JOIN permissions p ON rp.permission_id = p.id 
           WHERE rp.role_id = ?"#
    )
    .bind(role_id.to_string())
    .fetch_all(&state.pool).await?;
    
    let mut perms = Vec::new();
    for r in rows {
        perms.push(RolePermissionResponse {
            permission_id: parse_uuid(r.get::<String, _>("permission_id").as_str())?,
            permission_code: r.get::<String, _>("code"),
            permission_name: r.get::<String, _>("name"),
        });
    }
    Ok(Json(perms))
}

#[derive(Deserialize)]
pub struct AssignRoleRequest {
    pub user_id: Uuid,
    pub expires_at: Option<String>,
}

pub async fn assign_role_to_user(
    State(state): State<AppState>,
    Path(role_id): Path<Uuid>,
    Json(req): Json<AssignRoleRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        "INSERT OR REPLACE INTO user_role_assignments (id, user_id, role_id, assigned_at, expires_at) VALUES (?, ?, ?, ?, ?)"
    )
    .bind(id.to_string())
    .bind(req.user_id.to_string())
    .bind(role_id.to_string())
    .bind(&now)
    .bind(&req.expires_at)
    .execute(&state.pool).await?;
    
    Ok(Json(serde_json::json!({ "status": "assigned" })))
}

pub async fn revoke_role_from_user(
    State(state): State<AppState>,
    Path((user_id, role_id)): Path<(Uuid, Uuid)>,
) -> ApiResult<Json<serde_json::Value>> {
    sqlx::query("DELETE FROM user_role_assignments WHERE user_id = ? AND role_id = ?")
        .bind(user_id.to_string())
        .bind(role_id.to_string())
        .execute(&state.pool).await?;
    
    Ok(Json(serde_json::json!({ "status": "revoked" })))
}

#[derive(Serialize)]
pub struct UserRolesResponse {
    pub role_id: Uuid,
    pub role_name: String,
    pub role_code: String,
}

pub async fn list_user_roles(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<Json<Vec<UserRolesResponse>>> {
    let rows = sqlx::query(
        r#"SELECT r.id, r.name, r.code 
           FROM user_role_assignments ura 
           JOIN custom_roles r ON ura.role_id = r.id 
           WHERE ura.user_id = ? AND r.is_active = 1"#
    )
    .bind(user_id.to_string())
    .fetch_all(&state.pool).await?;
    
    let mut roles = Vec::new();
    for r in rows {
        roles.push(UserRolesResponse {
            role_id: parse_uuid(r.get::<String, _>("id").as_str())?,
            role_name: r.get::<String, _>("name"),
            role_code: r.get::<String, _>("code"),
        });
    }
    Ok(Json(roles))
}

#[derive(Serialize)]
pub struct UserEffectivePermissionsResponse {
    pub user_id: Uuid,
    pub permissions: Vec<String>,
}

pub async fn get_user_effective_permissions(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> ApiResult<Json<UserEffectivePermissionsResponse>> {
    let rows = sqlx::query(
        r#"SELECT DISTINCT p.code 
           FROM user_role_assignments ura 
           JOIN role_permissions rp ON ura.role_id = rp.role_id 
           JOIN permissions p ON rp.permission_id = p.id 
           WHERE ura.user_id = ? AND (ura.expires_at IS NULL OR ura.expires_at > datetime('now'))"#
    )
    .bind(user_id.to_string())
    .fetch_all(&state.pool).await?;
    
    Ok(Json(UserEffectivePermissionsResponse {
        user_id,
        permissions: rows.into_iter().map(|r| r.get::<String, _>("code")).collect(),
    }))
}

#[derive(Deserialize)]
pub struct SetDataPermissionRequest {
    pub resource: String,
    pub filter_type: String,
    pub filter_value: String,
}

pub async fn set_data_permission(
    State(state): State<AppState>,
    Path(role_id): Path<Uuid>,
    Json(req): Json<SetDataPermissionRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        "INSERT OR REPLACE INTO data_permissions (id, role_id, resource, filter_type, filter_value, created_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(id.to_string())
    .bind(role_id.to_string())
    .bind(&req.resource)
    .bind(&req.filter_type)
    .bind(&req.filter_value)
    .bind(&now)
    .execute(&state.pool).await?;
    
    Ok(Json(serde_json::json!({ "status": "set" })))
}

#[derive(Deserialize)]
pub struct SetFieldPermissionRequest {
    pub resource: String,
    pub field_name: String,
    pub can_read: bool,
    pub can_write: bool,
    pub can_create: bool,
}

pub async fn set_field_permission(
    State(state): State<AppState>,
    Path(role_id): Path<Uuid>,
    Json(req): Json<SetFieldPermissionRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let id = Uuid::new_v4();
    let now = chrono::Utc::now().to_rfc3339();
    
    sqlx::query(
        "INSERT OR REPLACE INTO field_permissions (id, role_id, resource, field_name, can_read, can_write, can_create, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
    )
    .bind(id.to_string())
    .bind(role_id.to_string())
    .bind(&req.resource)
    .bind(&req.field_name)
    .bind(req.can_read as i32)
    .bind(req.can_write as i32)
    .bind(req.can_create as i32)
    .bind(&now)
    .execute(&state.pool).await?;
    
    Ok(Json(serde_json::json!({ "status": "set" })))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/roles", get(list_roles).post(create_role))
        .route("/roles/:id", delete(delete_role))
        .route("/permissions", get(list_permissions))
        .route("/roles/:role_id/permissions", get(list_role_permissions).post(assign_permission))
        .route("/roles/:role_id/permissions/:permission_id", delete(revoke_permission))
        .route("/roles/:role_id/users", post(assign_role_to_user))
        .route("/users/:user_id/roles", get(list_user_roles))
        .route("/users/:user_id/roles/:role_id", delete(revoke_role_from_user))
        .route("/users/:user_id/effective-permissions", get(get_user_effective_permissions))
        .route("/roles/:role_id/data-permissions", post(set_data_permission))
        .route("/roles/:role_id/field-permissions", post(set_field_permission))
}
