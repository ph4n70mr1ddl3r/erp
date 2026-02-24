use axum::{
    extract::{Path, Query, State},
    Json,
};
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::BaseEntity;
use erp_config::{
    ConfigService, SystemConfig, CompanySetting, NumberSequence, EmailConfig,
    StorageConfig, PaymentGateway, ShippingProvider, Localization, AuditSetting,
    IntegrationConfig, ConfigValueType, StorageType,
};

#[derive(Deserialize)]
pub struct SetConfigRequest {
    pub category: String,
    pub key: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct ConfigResponse {
    pub id: Uuid,
    pub category: String,
    pub key: String,
    pub value: String,
}

pub async fn list_configs(
    State(state): State<AppState>,
    Query(query): Query<CategoryQuery>,
) -> ApiResult<Json<Vec<ConfigResponse>>> {
    let service = ConfigService::new();
    let configs = service.list(&state.pool, query.category.as_deref()).await?;
    Ok(Json(configs.into_iter().map(|c| ConfigResponse {
        id: c.base.id,
        category: c.category,
        key: c.key,
        value: c.value,
    }).collect()))
}

#[derive(Deserialize)]
pub struct CategoryQuery {
    pub category: Option<String>,
}

pub async fn set_config(
    State(state): State<AppState>,
    Json(req): Json<SetConfigRequest>,
) -> ApiResult<Json<ConfigResponse>> {
    let service = ConfigService::new();
    let config = service.set(&state.pool, req.category, req.key, req.value).await?;
    Ok(Json(ConfigResponse {
        id: config.base.id,
        category: config.category,
        key: config.key,
        value: config.value,
    }))
}

pub async fn get_config(
    State(state): State<AppState>,
    Path((category, key)): Path<(String, String)>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = ConfigService::new();
    let value = service.get(&state.pool, &category, &key).await?;
    Ok(Json(serde_json::json!({
        "category": category,
        "key": key,
        "value": value
    })))
}

pub async fn delete_config(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = ConfigService::new();
    service.delete(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "deleted" })))
}

#[derive(Serialize)]
pub struct CompanySettingsResponse {
    pub company_name: String,
    pub currency: String,
    pub timezone: String,
}

pub async fn get_company_settings(
    State(state): State<AppState>,
) -> ApiResult<Json<Option<CompanySettingsResponse>>> {
    let service = ConfigService::new();
    let settings = service.get_company_settings(&state.pool).await?;
    Ok(Json(settings.map(|s| CompanySettingsResponse {
        company_name: s.company_name,
        currency: s.currency,
        timezone: s.timezone,
    })))
}

#[derive(Deserialize)]
pub struct UpdateCompanySettingsRequest {
    pub company_name: String,
    pub currency: String,
    pub timezone: String,
}

pub async fn update_company_settings(
    State(state): State<AppState>,
    Json(req): Json<UpdateCompanySettingsRequest>,
) -> ApiResult<Json<CompanySettingsResponse>> {
    let service = ConfigService::new();
    let existing = service.get_company_settings(&state.pool).await?;
    let settings = CompanySetting {
        base: existing.map(|e| e.base).unwrap_or_else(BaseEntity::new),
        company_name: req.company_name,
        legal_name: None,
        tax_id: None,
        registration_number: None,
        logo_url: None,
        favicon_url: None,
        primary_color: None,
        secondary_color: None,
        timezone: req.timezone,
        date_format: "YYYY-MM-DD".to_string(),
        time_format: "HH:mm".to_string(),
        currency: req.currency,
        language: "en".to_string(),
        fiscal_year_start: 1,
        week_start: 1,
        address: None,
        city: None,
        state: None,
        country: None,
        postal_code: None,
        phone: None,
        email: None,
        website: None,
    };
    let updated = service.update_company_settings(&state.pool, settings).await?;
    Ok(Json(CompanySettingsResponse {
        company_name: updated.company_name,
        currency: updated.currency,
        timezone: updated.timezone,
    }))
}

#[derive(Deserialize)]
pub struct CreateSequenceRequest {
    pub name: String,
    pub code: String,
    pub prefix: Option<String>,
    pub padding: i32,
}

pub async fn create_sequence(
    State(state): State<AppState>,
    Json(req): Json<CreateSequenceRequest>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = ConfigService::new();
    service.create_number_sequence(&state.pool, req.name, req.code, req.prefix, req.padding).await?;
    Ok(Json(serde_json::json!({ "status": "created" })))
}

pub async fn get_next_number(
    State(state): State<AppState>,
    Path(code): Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = ConfigService::new();
    let number = service.get_next_number(&state.pool, &code).await?;
    Ok(Json(serde_json::json!({ "number": number })))
}

#[derive(Serialize)]
pub struct AuditSettingsResponse {
    pub log_retention_days: i32,
    pub max_login_attempts: i32,
    pub require_mfa: bool,
}

pub async fn get_audit_settings(
    State(state): State<AppState>,
) -> ApiResult<Json<Option<AuditSettingsResponse>>> {
    let service = ConfigService::new();
    let settings = service.get_audit_settings(&state.pool).await?;
    Ok(Json(settings.map(|s| AuditSettingsResponse {
        log_retention_days: s.log_retention_days,
        max_login_attempts: s.max_login_attempts,
        require_mfa: s.require_mfa,
    })))
}

#[derive(Deserialize)]
pub struct UpdateAuditSettingsRequest {
    pub log_retention_days: i32,
    pub max_login_attempts: i32,
    pub require_mfa: bool,
}

pub async fn update_audit_settings(
    State(state): State<AppState>,
    Json(req): Json<UpdateAuditSettingsRequest>,
) -> ApiResult<Json<AuditSettingsResponse>> {
    let service = ConfigService::new();
    let existing = service.get_audit_settings(&state.pool).await?;
    let settings = AuditSetting {
        base: existing.map(|e| e.base).unwrap_or_else(BaseEntity::new),
        log_retention_days: req.log_retention_days,
        log_sensitive_data: false,
        log_login_attempts: true,
        log_data_changes: true,
        log_api_requests: true,
        alert_on_suspicious: true,
        max_login_attempts: req.max_login_attempts,
        lockout_duration_minutes: 30,
        password_expiry_days: None,
        require_mfa: req.require_mfa,
        session_timeout_minutes: 60,
    };
    let updated = service.update_audit_settings(&state.pool, settings).await?;
    Ok(Json(AuditSettingsResponse {
        log_retention_days: updated.log_retention_days,
        max_login_attempts: updated.max_login_attempts,
        require_mfa: updated.require_mfa,
    }))
}

#[derive(Serialize)]
pub struct IntegrationResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub integration_type: String,
    pub is_active: bool,
}

pub async fn list_integrations(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<IntegrationResponse>>> {
    let service = ConfigService::new();
    let integrations = service.list_integrations(&state.pool).await?;
    Ok(Json(integrations.into_iter().map(|i| IntegrationResponse {
        id: i.base.id,
        name: i.name,
        code: i.code,
        integration_type: i.integration_type,
        is_active: i.is_active,
    }).collect()))
}

pub fn routes() -> axum::Router<crate::db::AppState> {
    axum::Router::new()
        .route("/configs", axum::routing::get(list_configs).post(set_config))
        .route("/configs/:category/:key", axum::routing::get(get_config))
        .route("/configs/:id", axum::routing::delete(delete_config))
        .route("/company", axum::routing::get(get_company_settings).post(update_company_settings))
        .route("/sequences", axum::routing::post(create_sequence))
        .route("/sequences/:code/next", axum::routing::get(get_next_number))
        .route("/audit", axum::routing::get(get_audit_settings).post(update_audit_settings))
        .route("/integrations", axum::routing::get(list_integrations))
}
