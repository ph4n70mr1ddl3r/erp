use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::db::AppState;
use crate::error::ApiResult;

#[derive(Debug, Deserialize)]
pub struct CreateAPIKeyRequest {
    pub name: String,
    pub description: Option<String>,
    pub scopes: Vec<String>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct APIKeyResponse {
    pub id: Uuid,
    pub name: String,
    pub key_prefix: String,
    pub status: String,
    pub scopes: Vec<String>,
    pub usage_count: i64,
    pub created_at: String,
}

#[derive(Debug, Serialize)]
pub struct APIKeyWithSecretResponse {
    pub id: Uuid,
    pub name: String,
    pub key: String,
    pub key_prefix: String,
    pub status: String,
    pub scopes: Vec<String>,
    pub created_at: String,
}

pub async fn create_api_key(
    State(state): State<AppState>,
    Json(req): Json<CreateAPIKeyRequest>,
) -> ApiResult<Json<APIKeyWithSecretResponse>> {
    let created_by = Uuid::nil();
    let expires_at = req.expires_at.as_deref()
        .map(|s| chrono::DateTime::parse_from_rfc3339(s))
        .transpose()
        .map_err(|e| erp_core::Error::validation(format!("Invalid expires_at: {}", e)))?
        .map(|dt| dt.with_timezone(&chrono::Utc));
    
    let service = erp_integration::APIKeyService::new();
    let (api_key, secret) = service.create(
        &state.pool,
        req.name,
        req.description,
        None,
        req.scopes,
        created_by,
        expires_at,
    ).await?;
    
    Ok(Json(APIKeyWithSecretResponse {
        id: api_key.base.id,
        name: api_key.name,
        key: secret,
        key_prefix: api_key.key_prefix,
        status: format!("{:?}", api_key.status),
        scopes: api_key.scopes,
        created_at: api_key.created_at.to_rfc3339(),
    }))
}

pub async fn list_api_keys(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<APIKeyResponse>>> {
    let service = erp_integration::APIKeyService::new();
    let keys = service.list(&state.pool, None).await?;
    
    let response: Vec<APIKeyResponse> = keys.into_iter().map(|k| APIKeyResponse {
        id: k.base.id,
        name: k.name,
        key_prefix: k.key_prefix,
        status: format!("{:?}", k.status),
        scopes: k.scopes,
        usage_count: k.usage_count,
        created_at: k.created_at.to_rfc3339(),
    }).collect();
    
    Ok(Json(response))
}

pub async fn revoke_api_key(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let service = erp_integration::APIKeyService::new();
    service.revoke(&state.pool, id).await?;
    Ok(StatusCode::OK)
}

pub async fn delete_api_key(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let service = erp_integration::APIKeyService::new();
    service.delete(&state.pool, id).await?;
    Ok(StatusCode::OK)
}

#[derive(Debug, Deserialize)]
pub struct CreateConnectionRequest {
    pub name: String,
    pub code: String,
    pub connection_type: String,
    pub endpoint_url: Option<String>,
    pub configuration: Option<serde_json::Value>,
    pub auth_type: String,
    pub auth_config: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ConnectionResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub connection_type: String,
    pub status: String,
    pub last_sync_at: Option<String>,
    pub created_at: String,
}

pub async fn create_connection(
    State(state): State<AppState>,
    Json(req): Json<CreateConnectionRequest>,
) -> ApiResult<Json<ConnectionResponse>> {
    let created_by = Uuid::nil();
    let connection_type = parse_connection_type(&req.connection_type)?;
    let auth_type = parse_auth_type(&req.auth_type)?;
    
    let service = erp_integration::ExternalConnectionService::new();
    let conn = service.create(
        &state.pool,
        req.name,
        req.code,
        connection_type,
        req.endpoint_url,
        req.configuration,
        auth_type,
        req.auth_config,
        created_by,
    ).await?;
    
    Ok(Json(ConnectionResponse {
        id: conn.base.id,
        name: conn.name,
        code: conn.code,
        connection_type: format!("{:?}", conn.connection_type),
        status: format!("{:?}", conn.status),
        last_sync_at: conn.last_sync_at.map(|dt| dt.to_rfc3339()),
        created_at: conn.created_at.to_rfc3339(),
    }))
}

pub async fn list_connections(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<ConnectionResponse>>> {
    let service = erp_integration::ExternalConnectionService::new();
    let connections = service.list(&state.pool).await?;
    
    let response: Vec<ConnectionResponse> = connections.into_iter().map(|c| ConnectionResponse {
        id: c.base.id,
        name: c.name,
        code: c.code,
        connection_type: format!("{:?}", c.connection_type),
        status: format!("{:?}", c.status),
        last_sync_at: c.last_sync_at.map(|dt| dt.to_rfc3339()),
        created_at: c.created_at.to_rfc3339(),
    }).collect();
    
    Ok(Json(response))
}

pub async fn test_connection(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = erp_integration::ExternalConnectionService::new();
    let success = service.test_connection(&state.pool, id).await?;
    
    Ok(Json(serde_json::json!({
        "success": success,
        "message": if success { "Connection successful" } else { "Connection failed" }
    })))
}

pub async fn delete_connection(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let service = erp_integration::ExternalConnectionService::new();
    service.delete(&state.pool, id).await?;
    Ok(StatusCode::OK)
}

fn parse_connection_type(s: &str) -> anyhow::Result<erp_integration::ConnectionType> {
    match s {
        "Database" => Ok(erp_integration::ConnectionType::Database),
        "REST" => Ok(erp_integration::ConnectionType::REST),
        "GraphQL" => Ok(erp_integration::ConnectionType::GraphQL),
        "SOAP" => Ok(erp_integration::ConnectionType::SOAP),
        "FTP" => Ok(erp_integration::ConnectionType::FTP),
        "SFTP" => Ok(erp_integration::ConnectionType::SFTP),
        "Email" => Ok(erp_integration::ConnectionType::Email),
        "OAuth2" => Ok(erp_integration::ConnectionType::OAuth2),
        "Webhook" => Ok(erp_integration::ConnectionType::Webhook),
        "PaymentGateway" => Ok(erp_integration::ConnectionType::PaymentGateway),
        "ShippingProvider" => Ok(erp_integration::ConnectionType::ShippingProvider),
        "CRM" => Ok(erp_integration::ConnectionType::CRM),
        "ERP" => Ok(erp_integration::ConnectionType::ERP),
        "Accounting" => Ok(erp_integration::ConnectionType::Accounting),
        "ECommerce" => Ok(erp_integration::ConnectionType::ECommerce),
        "Custom" => Ok(erp_integration::ConnectionType::Custom),
        _ => Err(anyhow::anyhow!("Invalid connection type: {}", s)),
    }
}

fn parse_auth_type(s: &str) -> anyhow::Result<erp_integration::AuthType> {
    match s {
        "None" => Ok(erp_integration::AuthType::None),
        "Basic" => Ok(erp_integration::AuthType::Basic),
        "Bearer" => Ok(erp_integration::AuthType::Bearer),
        "APIKey" => Ok(erp_integration::AuthType::APIKey),
        "OAuth2" => Ok(erp_integration::AuthType::OAuth2),
        "OAuth2ClientCredentials" => Ok(erp_integration::AuthType::OAuth2ClientCredentials),
        "MutualTLS" => Ok(erp_integration::AuthType::MutualTLS),
        "Custom" => Ok(erp_integration::AuthType::Custom),
        _ => Err(anyhow::anyhow!("Invalid auth type: {}", s)),
    }
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/api-keys", axum::routing::post(create_api_key).get(list_api_keys))
        .route("/api-keys/:id/revoke", axum::routing::post(revoke_api_key))
        .route("/api-keys/:id", axum::routing::delete(delete_api_key))
        .route("/connections", axum::routing::post(create_connection).get(list_connections))
        .route("/connections/:id/test", axum::routing::post(test_connection))
        .route("/connections/:id", axum::routing::delete(delete_connection))
}
