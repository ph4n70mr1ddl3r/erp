use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::AppState;
use crate::ApiResult;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/locales", get(list_locales).post(create_locale))
        .route("/locales/:code", get(get_locale))
        .route("/translations", post(set_translation))
        .route("/translations/:locale/:namespace", get(get_translations))
        .route("/user-preferences", post(set_user_preference).get(get_user_preference))
        .route("/missing", get(get_missing_translations))
}

#[derive(Deserialize)]
pub struct CreateLocaleRequest {
    pub code: String,
    pub name: String,
    pub native_name: String,
    pub language_code: String,
}

#[derive(Serialize)]
pub struct LocaleResponse {
    pub id: String,
    pub code: String,
    pub name: String,
    pub native_name: String,
    pub is_default: bool,
}

pub async fn create_locale(
    State(state): State<AppState>,
    Json(req): Json<CreateLocaleRequest>,
) -> ApiResult<Json<LocaleResponse>> {
    let service = erp_i18n::I18nService::new();
    let locale = service.create_locale(&state.pool, req.code.clone(), req.name, req.native_name, req.language_code).await?;

    Ok(Json(LocaleResponse {
        id: locale.base.id.to_string(),
        code: locale.code,
        name: locale.name,
        native_name: locale.native_name,
        is_default: locale.is_default,
    }))
}

pub async fn list_locales(State(state): State<AppState>) -> ApiResult<Json<Vec<LocaleResponse>>> {
    let service = erp_i18n::I18nService::new();
    let locales = service.list_locales(&state.pool, true).await?;

    Ok(Json(locales.into_iter().map(|l| LocaleResponse {
        id: l.base.id.to_string(),
        code: l.code,
        name: l.name,
        native_name: l.native_name,
        is_default: l.is_default,
    }).collect()))
}

pub async fn get_locale(
    State(state): State<AppState>,
    axum::extract::Path(code): axum::extract::Path<String>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = erp_i18n::I18nService::new();
    let locale = service.get_locale(&state.pool, &code).await?.ok_or_else(|| anyhow::anyhow!("Locale not found"))?;

    Ok(Json(serde_json::json!({
        "id": locale.base.id.to_string(),
        "code": locale.code,
        "name": locale.name,
        "native_name": locale.native_name,
        "language_code": locale.language_code,
        "country_code": locale.country_code,
        "is_rtl": locale.is_rtl,
        "date_format": locale.date_format,
        "time_format": locale.time_format,
        "number_format": locale.number_format,
        "currency_symbol": locale.currency_symbol,
        "is_default": locale.is_default
    })))
}

#[derive(Deserialize)]
pub struct SetTranslationRequest {
    pub locale_code: String,
    pub namespace: String,
    pub key: String,
    pub value: String,
}

#[derive(Serialize)]
pub struct TranslationResponse {
    pub id: String,
    pub locale_code: String,
    pub namespace: String,
    pub key: String,
    pub value: String,
    pub is_approved: bool,
}

pub async fn set_translation(
    State(state): State<AppState>,
    Json(req): Json<SetTranslationRequest>,
) -> ApiResult<Json<TranslationResponse>> {
    let service = erp_i18n::I18nService::new();
    let translation = service.set_translation(&state.pool, req.locale_code, req.namespace, req.key, req.value).await?;

    Ok(Json(TranslationResponse {
        id: translation.base.id.to_string(),
        locale_code: translation.locale_code,
        namespace: translation.namespace,
        key: translation.key,
        value: translation.value,
        is_approved: translation.is_approved,
    }))
}

pub async fn get_translations(
    State(state): State<AppState>,
    axum::extract::Path((locale, namespace)): axum::extract::Path<(String, String)>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = erp_i18n::I18nService::new();
    let map = service.get_translations_map(&state.pool, &locale, &namespace).await?;

    Ok(Json(map))
}

#[derive(Deserialize)]
pub struct SetUserPreferenceRequest {
    pub user_id: String,
    pub locale_code: String,
    pub timezone: String,
}

#[derive(Serialize)]
pub struct UserPreferenceResponse {
    pub user_id: String,
    pub locale_code: String,
    pub timezone: String,
}

pub async fn set_user_preference(
    State(state): State<AppState>,
    Json(req): Json<SetUserPreferenceRequest>,
) -> ApiResult<Json<UserPreferenceResponse>> {
    let user_id = Uuid::parse_str(&req.user_id)?;
    let service = erp_i18n::I18nService::new();
    let pref = service.set_user_preference(&state.pool, user_id, req.locale_code, req.timezone).await?;

    Ok(Json(UserPreferenceResponse {
        user_id: pref.user_id.to_string(),
        locale_code: pref.locale_code,
        timezone: pref.timezone,
    }))
}

pub async fn get_user_preference(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> ApiResult<Json<Option<UserPreferenceResponse>>> {
    let user_id = params.get("user_id")
        .ok_or_else(|| anyhow::anyhow!("user_id required"))?;
    let user_id = Uuid::parse_str(user_id)?;

    let service = erp_i18n::I18nService::new();
    let pref = service.get_user_preference(&state.pool, user_id).await?;

    Ok(Json(pref.map(|p| UserPreferenceResponse {
        user_id: p.user_id.to_string(),
        locale_code: p.locale_code,
        timezone: p.timezone,
    })))
}

#[derive(Serialize)]
pub struct MissingTranslationResponse {
    pub locale_code: String,
    pub namespace: String,
    pub key: String,
    pub usage_count: i32,
    pub priority: String,
}

pub async fn get_missing_translations(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<MissingTranslationResponse>>> {
    let service = erp_i18n::I18nService::new();
    let missing = service.detect_missing_translations(&state.pool).await?;

    Ok(Json(missing.into_iter().map(|m| MissingTranslationResponse {
        locale_code: m.locale_code,
        namespace: m.namespace,
        key: m.key,
        usage_count: m.usage_count,
        priority: m.priority,
    }).collect()))
}
