use axum::{extract::{Path, Query, State}, Json};
use uuid::Uuid;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::Pagination;
use erp_assets::{ITAsset, SoftwareLicense, ITAssetService, SoftwareLicenseService, ITAssetType, ITAssetStatus, LicenseType};

#[derive(Deserialize)]
pub struct CreateAssetRequest {
    pub asset_tag: String,
    pub name: String,
    pub description: Option<String>,
    pub asset_type: Option<String>,
    pub model: Option<String>,
    pub manufacturer: Option<String>,
    pub serial_number: Option<String>,
    pub purchase_date: Option<String>,
    pub purchase_cost: i64,
    pub currency: Option<String>,
    pub warranty_expiry: Option<String>,
    pub location_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct AssetResponse {
    pub id: Uuid,
    pub asset_tag: String,
    pub name: String,
    pub description: Option<String>,
    pub asset_type: String,
    pub status: String,
    pub assigned_to: Option<Uuid>,
    pub created_at: String,
}

impl From<ITAsset> for AssetResponse {
    fn from(a: ITAsset) -> Self {
        Self {
            id: a.base.id,
            asset_tag: a.asset_tag,
            name: a.name,
            description: a.description,
            asset_type: format!("{:?}", a.asset_type),
            status: format!("{:?}", a.status),
            assigned_to: a.assigned_to,
            created_at: a.base.created_at.to_rfc3339(),
        }
    }
}

pub async fn list_assets(State(state): State<AppState>, Query(pagination): Query<Pagination>) -> ApiResult<Json<erp_core::Paginated<AssetResponse>>> {
    let svc = ITAssetService::new();
    let assets = svc.list(&state.pool, pagination.page as i64, pagination.per_page as i64).await?;
    Ok(Json(erp_core::Paginated::new(
        assets.into_iter().map(AssetResponse::from).collect(),
        0,
        pagination,
    )))
}

pub async fn get_asset(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    let svc = ITAssetService::new();
    let asset = svc.get(&state.pool, id).await?;
    Ok(Json(serde_json::to_value(asset)?))
}

pub async fn create_asset(State(state): State<AppState>, Json(req): Json<CreateAssetRequest>) -> ApiResult<Json<AssetResponse>> {
    let svc = ITAssetService::new();
    let asset_type = match req.asset_type.as_deref() {
        Some("Software") => ITAssetType::Software,
        Some("Network") => ITAssetType::Network,
        Some("Peripheral") => ITAssetType::Peripheral,
        Some("Mobile") => ITAssetType::Mobile,
        Some("Server") => ITAssetType::Server,
        Some("Storage") => ITAssetType::Storage,
        Some("Security") => ITAssetType::Security,
        _ => ITAssetType::Hardware,
    };
    let purchase_date = req.purchase_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    let warranty_expiry = req.warranty_expiry.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    let asset = svc.create(&state.pool, req.asset_tag, req.name, asset_type, req.purchase_cost, req.currency.unwrap_or_else(|| "USD".to_string()), req.description, req.model, req.manufacturer, req.serial_number, purchase_date, warranty_expiry, req.location_id).await?;
    Ok(Json(AssetResponse::from(asset)))
}

#[derive(Deserialize)]
pub struct AssignAssetRequest {
    pub user_id: Uuid,
    pub assigned_by: Uuid,
    pub expected_return: Option<String>,
}

pub async fn assign_asset(State(state): State<AppState>, Path(id): Path<Uuid>, Json(req): Json<AssignAssetRequest>) -> ApiResult<Json<AssetResponse>> {
    let svc = ITAssetService::new();
    let expected_return = req.expected_return.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    let asset = svc.assign(&state.pool, id, req.user_id, req.assigned_by, expected_return).await?;
    Ok(Json(AssetResponse::from(asset)))
}

#[derive(Deserialize)]
pub struct ReturnAssetRequest {
    pub returned_by: Uuid,
}

pub async fn return_asset(State(state): State<AppState>, Path(id): Path<Uuid>, Json(req): Json<ReturnAssetRequest>) -> ApiResult<Json<AssetResponse>> {
    let svc = ITAssetService::new();
    let asset = svc.unassign(&state.pool, id, req.returned_by).await?;
    Ok(Json(AssetResponse::from(asset)))
}

#[derive(Deserialize)]
pub struct UpdateAssetStatusRequest {
    pub status: String,
}

pub async fn update_asset_status(State(state): State<AppState>, Path(id): Path<Uuid>, Json(req): Json<UpdateAssetStatusRequest>) -> ApiResult<Json<AssetResponse>> {
    let svc = ITAssetService::new();
    let status = match req.status.as_str() {
        "InUse" => ITAssetStatus::InUse,
        "InMaintenance" => ITAssetStatus::InMaintenance,
        "Reserved" => ITAssetStatus::Reserved,
        "Retired" => ITAssetStatus::Retired,
        "Lost" => ITAssetStatus::Lost,
        "Disposed" => ITAssetStatus::Disposed,
        _ => ITAssetStatus::Available,
    };
    let asset = svc.update_status(&state.pool, id, status).await?;
    Ok(Json(AssetResponse::from(asset)))
}

pub async fn asset_stats(State(state): State<AppState>) -> ApiResult<Json<serde_json::Value>> {
    let svc = ITAssetService::new();
    let stats = svc.get_stats(&state.pool).await?;
    Ok(Json(serde_json::json!({ "by_status": stats.into_iter().collect::<std::collections::HashMap<_, _>>() })))
}

#[derive(Deserialize)]
pub struct CreateLicenseRequest {
    pub license_key: String,
    pub product_name: String,
    pub vendor: String,
    pub license_type: Option<String>,
    pub seats_purchased: i32,
    pub purchase_cost: i64,
    pub currency: Option<String>,
    pub purchase_date: String,
    pub start_date: String,
    pub expiry_date: Option<String>,
}

#[derive(Serialize)]
pub struct LicenseResponse {
    pub id: Uuid,
    pub product_name: String,
    pub vendor: String,
    pub license_type: String,
    pub seats_purchased: i32,
    pub seats_used: i32,
    pub seats_available: i32,
    pub status: String,
    pub expiry_date: Option<String>,
}

impl From<SoftwareLicense> for LicenseResponse {
    fn from(l: SoftwareLicense) -> Self {
        Self {
            id: l.id,
            product_name: l.product_name,
            vendor: l.vendor,
            license_type: format!("{:?}", l.license_type),
            seats_purchased: l.seats_purchased,
            seats_used: l.seats_used,
            seats_available: l.seats_purchased - l.seats_used,
            status: format!("{:?}", l.status),
            expiry_date: l.expiry_date.map(|d| d.to_string()),
        }
    }
}

pub async fn list_licenses(State(state): State<AppState>, Query(pagination): Query<Pagination>) -> ApiResult<Json<erp_core::Paginated<LicenseResponse>>> {
    let svc = SoftwareLicenseService::new();
    let licenses = svc.list(&state.pool, pagination.page as i64, pagination.per_page as i64).await?;
    Ok(Json(erp_core::Paginated::new(
        licenses.into_iter().map(LicenseResponse::from).collect(),
        0,
        pagination,
    )))
}

pub async fn get_license(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    let svc = SoftwareLicenseService::new();
    let license = svc.get(&state.pool, id).await?;
    Ok(Json(serde_json::to_value(license)?))
}

pub async fn create_license(State(state): State<AppState>, Json(req): Json<CreateLicenseRequest>) -> ApiResult<Json<LicenseResponse>> {
    let svc = SoftwareLicenseService::new();
    let license_type = match req.license_type.as_deref() {
        Some("Subscription") => LicenseType::Subscription,
        Some("Volume") => LicenseType::Volume,
        Some("Site") => LicenseType::Site,
        Some("Concurrent") => LicenseType::Concurrent,
        Some("NamedUser") => LicenseType::NamedUser,
        Some("Oem") => LicenseType::Oem,
        Some("Trial") => LicenseType::Trial,
        _ => LicenseType::Perpetual,
    };
    let purchase_date = NaiveDate::parse_from_str(&req.purchase_date, "%Y-%m-%d").unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
    let start_date = NaiveDate::parse_from_str(&req.start_date, "%Y-%m-%d").unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap());
    let expiry_date = req.expiry_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok());
    let license = svc.create(&state.pool, req.license_key, req.product_name, req.vendor, license_type, req.seats_purchased, req.purchase_cost, req.currency.unwrap_or_else(|| "USD".to_string()), purchase_date, start_date, expiry_date).await?;
    Ok(Json(LicenseResponse::from(license)))
}

pub async fn use_license_seat(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<LicenseResponse>> {
    let svc = SoftwareLicenseService::new();
    let license = svc.increment_usage(&state.pool, id).await?;
    Ok(Json(LicenseResponse::from(license)))
}

pub async fn release_license_seat(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<LicenseResponse>> {
    let svc = SoftwareLicenseService::new();
    let license = svc.decrement_usage(&state.pool, id).await?;
    Ok(Json(LicenseResponse::from(license)))
}

#[derive(Deserialize)]
pub struct ExpiringQuery {
    pub days: Option<i32>,
}

pub async fn expiring_licenses(State(state): State<AppState>, Query(query): Query<ExpiringQuery>) -> ApiResult<Json<Vec<LicenseResponse>>> {
    let svc = SoftwareLicenseService::new();
    let licenses = svc.get_expiring(&state.pool, query.days.unwrap_or(30)).await?;
    Ok(Json(licenses.into_iter().map(LicenseResponse::from).collect()))
}
