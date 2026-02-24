use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_keys::{KeyService, EncryptionKey, KeyType, KeyPolicy};

#[derive(Serialize)]
pub struct KeyResponse {
    pub id: Uuid,
    pub key_id: String,
    pub key_type: String,
    pub algorithm: String,
    pub key_version: i32,
    pub is_active: bool,
    pub is_primary: bool,
    pub rotation_days: Option<i32>,
}

impl From<EncryptionKey> for KeyResponse {
    fn from(k: EncryptionKey) -> Self {
        Self {
            id: k.base.id,
            key_id: k.key_id,
            key_type: format!("{:?}", k.key_type),
            algorithm: k.algorithm,
            key_version: k.key_version,
            is_active: k.is_active,
            is_primary: k.is_primary,
            rotation_days: k.rotation_days,
        }
    }
}

#[derive(Deserialize)]
pub struct GenerateKeyRequest {
    pub key_type: String,
    pub name: String,
}

pub async fn list_keys(State(state): State<AppState>) -> ApiResult<Json<Vec<KeyResponse>>> {
    let service = KeyService::new();
    let keys = service.list_keys(&state.pool).await?;
    Ok(Json(keys.into_iter().map(KeyResponse::from).collect()))
}

pub async fn generate_key(
    State(state): State<AppState>,
    Json(req): Json<GenerateKeyRequest>,
) -> ApiResult<Json<KeyResponse>> {
    let service = KeyService::new();
    let key_type = match req.key_type.as_str() {
        "Aes256Gcm" => KeyType::Aes256Gcm,
        "Aes256Cbc" => KeyType::Aes256Cbc,
        "Hmac" => KeyType::Hmac,
        "Symmetric" => KeyType::Symmetric,
        _ => KeyType::Aes256Gcm,
    };
    let key = service.generate_key(&state.pool, key_type, &req.name).await?;
    Ok(Json(KeyResponse::from(key)))
}

pub async fn get_key(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<KeyResponse>> {
    let service = KeyService::new();
    let key = service.get_key(&state.pool, id).await?
        .ok_or_else(|| anyhow::anyhow!("Key not found"))?;
    Ok(Json(KeyResponse::from(key)))
}

pub async fn set_primary_key(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = KeyService::new();
    service.set_primary_key(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "updated" })))
}

pub async fn rotate_key(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = KeyService::new();
    let rotation = service.rotate_key(&state.pool, id, None).await?;
    Ok(Json(serde_json::json!({
        "status": "rotated",
        "from_version": rotation.from_version,
        "to_version": rotation.to_version
    })))
}

pub async fn keys_needing_rotation(State(state): State<AppState>) -> ApiResult<Json<Vec<KeyResponse>>> {
    let service = KeyService::new();
    let keys = service.get_keys_needing_rotation(&state.pool).await?;
    Ok(Json(keys.into_iter().map(KeyResponse::from).collect()))
}

#[derive(Deserialize)]
pub struct EncryptRequest {
    pub entity_type: String,
    pub entity_id: String,
    pub field_name: String,
    pub plaintext: String,
}

#[derive(Serialize)]
pub struct EncryptResponse {
    pub encrypted: bool,
}

pub async fn encrypt_data(
    State(state): State<AppState>,
    Json(req): Json<EncryptRequest>,
) -> ApiResult<Json<EncryptResponse>> {
    let service = KeyService::new();
    service.encrypt_data(&state.pool, &req.entity_type, &req.entity_id, &req.field_name, &req.plaintext).await?;
    Ok(Json(EncryptResponse { encrypted: true }))
}

#[derive(Deserialize)]
pub struct DecryptRequest {
    pub entity_type: String,
    pub entity_id: String,
    pub field_name: String,
}

#[derive(Serialize)]
pub struct DecryptResponse {
    pub plaintext: Option<String>,
}

pub async fn decrypt_data(
    State(state): State<AppState>,
    Json(req): Json<DecryptRequest>,
) -> ApiResult<Json<DecryptResponse>> {
    let service = KeyService::new();
    let plaintext = service.decrypt_data(&state.pool, &req.entity_type, &req.entity_id, &req.field_name).await?;
    Ok(Json(DecryptResponse { plaintext }))
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", get(list_keys).post(generate_key))
        .route("/:id", get(get_key))
        .route("/:id/primary", post(set_primary_key))
        .route("/:id/rotate", post(rotate_key))
        .route("/needing-rotation", get(keys_needing_rotation))
        .route("/encrypt", post(encrypt_data))
        .route("/decrypt", post(decrypt_data))
}
