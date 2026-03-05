use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::ApiResult;
use crate::db::AppState;
use crate::handlers::auth::AuthUser;
use erp_quality::{
    QualityService,
    QualityInspection, InspectionItem, QualityInspectionWithItems, QualityAnalytics,
    CreateInspectionRequest, CreateInspectionItemRequest, UpdateInspectionItemRequest,
    InspectionType, InspectionStatus,
    NonConformanceReport, CreateNCRRequest, UpdateNCRRequest,
    NCRSource, NCRSeverity, NCRStatus,
};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Deserialize)]
pub struct InspectionFilterQuery {
    pub status: Option<String>,
    pub inspection_type: Option<String>,
}

#[derive(Serialize)]
pub struct InspectionResponse {
    pub id: String,
    pub inspection_number: String,
    pub inspection_type: String,
    pub entity_type: String,
    pub entity_id: String,
    pub inspector_id: Option<String>,
    pub inspection_date: String,
    pub status: String,
    pub result: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}

impl From<QualityInspection> for InspectionResponse {
    fn from(i: QualityInspection) -> Self {
        Self {
            id: i.base.id.to_string(),
            inspection_number: i.inspection_number,
            inspection_type: format!("{:?}", i.inspection_type),
            entity_type: i.entity_type,
            entity_id: i.entity_id.to_string(),
            inspector_id: i.inspector_id.map(|id| id.to_string()),
            inspection_date: i.inspection_date.to_string(),
            status: format!("{:?}", i.status),
            result: i.result.map(|r| format!("{:?}", r)),
            notes: i.notes,
            created_at: i.base.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct InspectionItemResponse {
    pub id: String,
    pub inspection_id: String,
    pub criterion: String,
    pub expected_value: Option<String>,
    pub actual_value: Option<String>,
    pub pass_fail: Option<bool>,
    pub notes: Option<String>,
    pub created_at: String,
}

impl From<InspectionItem> for InspectionItemResponse {
    fn from(i: InspectionItem) -> Self {
        Self {
            id: i.id.to_string(),
            inspection_id: i.inspection_id.to_string(),
            criterion: i.criterion,
            expected_value: i.expected_value,
            actual_value: i.actual_value,
            pass_fail: i.pass_fail,
            notes: i.notes,
            created_at: i.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct InspectionWithItemsResponse {
    #[serde(flatten)]
    pub inspection: InspectionResponse,
    pub items: Vec<InspectionItemResponse>,
}

impl From<QualityInspectionWithItems> for InspectionWithItemsResponse {
    fn from(i: QualityInspectionWithItems) -> Self {
        Self {
            inspection: InspectionResponse::from(i.inspection),
            items: i.items.into_iter().map(InspectionItemResponse::from).collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateInspectionRequestDto {
    pub inspection_type: String,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub inspector_id: Option<Uuid>,
    pub inspection_date: String,
    pub notes: Option<String>,
    pub items: Vec<CreateInspectionItemRequestDto>,
}

#[derive(Deserialize)]
pub struct CreateInspectionItemRequestDto {
    pub criterion: String,
    pub expected_value: Option<String>,
    pub actual_value: Option<String>,
    pub pass_fail: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateInspectionItemRequestDto {
    pub actual_value: Option<String>,
    pub pass_fail: Option<bool>,
    pub notes: Option<String>,
}

#[derive(Serialize)]
pub struct NCRResponse {
    pub id: String,
    pub ncr_number: String,
    pub source_type: String,
    pub source_id: Option<String>,
    pub product_id: Option<String>,
    pub description: String,
    pub severity: String,
    pub status: String,
    pub assigned_to: Option<String>,
    pub root_cause: Option<String>,
    pub corrective_action: Option<String>,
    pub preventive_action: Option<String>,
    pub resolution_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<NonConformanceReport> for NCRResponse {
    fn from(n: NonConformanceReport) -> Self {
        Self {
            id: n.base.id.to_string(),
            ncr_number: n.ncr_number,
            source_type: format!("{:?}", n.source_type),
            source_id: n.source_id.map(|id| id.to_string()),
            product_id: n.product_id.map(|id| id.to_string()),
            description: n.description,
            severity: format!("{:?}", n.severity),
            status: format!("{:?}", n.status),
            assigned_to: n.assigned_to.map(|id| id.to_string()),
            root_cause: n.root_cause,
            corrective_action: n.corrective_action,
            preventive_action: n.preventive_action,
            resolution_date: n.resolution_date.map(|d| d.to_string()),
            created_at: n.base.created_at.to_rfc3339(),
            updated_at: n.base.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateNCRRequestDto {
    pub source_type: String,
    pub source_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub description: String,
    pub severity: String,
    pub assigned_to: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct UpdateNCRRequestDto {
    pub root_cause: Option<String>,
    pub corrective_action: Option<String>,
    pub preventive_action: Option<String>,
    pub status: Option<String>,
}

#[derive(Deserialize)]
pub struct NCRFilterQuery {
    pub status: Option<String>,
    pub severity: Option<String>,
}

fn parse_inspection_type(s: &str) -> InspectionType {
    match s {
        "Incoming" => InspectionType::Incoming,
        "InProcess" => InspectionType::InProcess,
        "Final" => InspectionType::Final,
        "Outgoing" => InspectionType::Outgoing,
        "Supplier" => InspectionType::Supplier,
        "Customer" => InspectionType::Customer,
        _ => InspectionType::Incoming,
    }
}

fn parse_inspection_status(s: &str) -> Option<InspectionStatus> {
    match s {
        "Pending" => Some(InspectionStatus::Pending),
        "InProgress" => Some(InspectionStatus::InProgress),
        "Passed" => Some(InspectionStatus::Passed),
        "Failed" => Some(InspectionStatus::Failed),
        "Partial" => Some(InspectionStatus::Partial),
        "Cancelled" => Some(InspectionStatus::Cancelled),
        _ => None,
    }
}

fn parse_ncr_source(s: &str) -> NCRSource {
    match s {
        "IncomingInspection" => NCRSource::IncomingInspection,
        "InProcessInspection" => NCRSource::InProcessInspection,
        "FinalInspection" => NCRSource::FinalInspection,
        "CustomerComplaint" => NCRSource::CustomerComplaint,
        "InternalAudit" => NCRSource::InternalAudit,
        "SupplierIssue" => NCRSource::SupplierIssue,
        "ProductionIssue" => NCRSource::ProductionIssue,
        _ => NCRSource::Other,
    }
}

fn parse_ncr_severity(s: &str) -> NCRSeverity {
    match s {
        "Minor" => NCRSeverity::Minor,
        "Major" => NCRSeverity::Major,
        "Critical" => NCRSeverity::Critical,
        _ => NCRSeverity::Minor,
    }
}

fn parse_ncr_status(s: &str) -> Option<NCRStatus> {
    match s {
        "Open" => Some(NCRStatus::Open),
        "UnderInvestigation" => Some(NCRStatus::UnderInvestigation),
        "CorrectiveAction" => Some(NCRStatus::CorrectiveAction),
        "Verification" => Some(NCRStatus::Verification),
        "Closed" => Some(NCRStatus::Closed),
        "Cancelled" => Some(NCRStatus::Cancelled),
        _ => None,
    }
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/inspections", axum::routing::get(list_inspections).post(create_inspection))
        .route("/inspections/analytics", axum::routing::get(get_analytics))
        .route("/inspections/:id", axum::routing::get(get_inspection).delete(delete_inspection))
        .route("/inspections/:id/start", axum::routing::post(start_inspection))
        .route("/inspections/:id/complete", axum::routing::post(complete_inspection))
        .route("/inspections/:id/cancel", axum::routing::post(cancel_inspection))
        .route("/inspections/:id/items", axum::routing::get(list_items).post(add_item))
        .route("/inspections/:id/items/:item_id", axum::routing::put(update_item))
        .route("/ncrs", axum::routing::get(list_ncrs).post(create_ncr))
        .route("/ncrs/:id", axum::routing::get(get_ncr).put(update_ncr).delete(delete_ncr))
        .route("/ncrs/:id/close", axum::routing::post(close_ncr))
}

pub async fn create_inspection(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Json(req): Json<CreateInspectionRequestDto>,
) -> ApiResult<Json<InspectionWithItemsResponse>> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id).ok();
    let svc = QualityService::new(state.pool);

    let inspection_date = chrono::NaiveDate::parse_from_str(&req.inspection_date, "%Y-%m-%d")
        .map_err(|_| anyhow::anyhow!("Invalid date format"))?;

    let items: Vec<CreateInspectionItemRequest> = req.items.into_iter().map(|l| CreateInspectionItemRequest {
        criterion: l.criterion,
        expected_value: l.expected_value,
        actual_value: l.actual_value,
        pass_fail: l.pass_fail,
        notes: l.notes,
    }).collect();

    let result = svc.create_inspection(CreateInspectionRequest {
        inspection_type: parse_inspection_type(&req.inspection_type),
        entity_type: req.entity_type,
        entity_id: req.entity_id,
        inspector_id: req.inspector_id,
        inspection_date,
        notes: req.notes,
        items,
    }, user_id).await.map_err(|e| {
        tracing::error!("Failed to create inspection: {:?}", e);
        e
    })?;

    Ok(Json(InspectionWithItemsResponse::from(result)))
}

pub async fn get_inspection(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<InspectionWithItemsResponse>> {
    let svc = QualityService::new(state.pool);

    let result = svc.get_inspection(id).await?
        .ok_or_else(|| anyhow::anyhow!("Inspection not found"))?;

    Ok(Json(InspectionWithItemsResponse::from(result)))
}

pub async fn list_inspections(
    State(state): State<AppState>,
    Query(query): Query<InspectionFilterQuery>,
) -> ApiResult<Json<ApiResponse<Vec<InspectionResponse>>>> {
    let svc = QualityService::new(state.pool);

    let status = query.status.and_then(|s| parse_inspection_status(&s));
    let inspection_type = query.inspection_type.map(|t| parse_inspection_type(&t));
    let inspections = svc.list_inspections(status, inspection_type).await?;
    let items: Vec<InspectionResponse> = inspections.into_iter().map(InspectionResponse::from).collect();

    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn start_inspection(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<InspectionResponse>> {
    let svc = QualityService::new(state.pool);
    let inspection = svc.start_inspection(id).await?;
    Ok(Json(InspectionResponse::from(inspection)))
}

pub async fn complete_inspection(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<InspectionResponse>> {
    let svc = QualityService::new(state.pool);
    let inspection = svc.complete_inspection(id).await?;
    Ok(Json(InspectionResponse::from(inspection)))
}

pub async fn cancel_inspection(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<InspectionResponse>> {
    let svc = QualityService::new(state.pool);
    let inspection = svc.cancel_inspection(id).await?;
    Ok(Json(InspectionResponse::from(inspection)))
}

pub async fn delete_inspection(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let svc = QualityService::new(state.pool);
    svc.delete_inspection(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_items(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<Vec<InspectionItemResponse>>>> {
    let svc = QualityService::new(state.pool);
    let items = svc.get_inspection_items(id).await?;
    let response: Vec<InspectionItemResponse> = items.into_iter().map(InspectionItemResponse::from).collect();
    Ok(Json(ApiResponse { success: true, data: response }))
}

pub async fn add_item(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateInspectionItemRequestDto>,
) -> ApiResult<Json<InspectionItemResponse>> {
    let svc = QualityService::new(state.pool);

    let item = svc.add_inspection_item(id, CreateInspectionItemRequest {
        criterion: req.criterion,
        expected_value: req.expected_value,
        actual_value: req.actual_value,
        pass_fail: req.pass_fail,
        notes: req.notes,
    }).await?;

    Ok(Json(InspectionItemResponse::from(item)))
}

pub async fn update_item(
    State(state): State<AppState>,
    Path((inspection_id, item_id)): Path<(Uuid, Uuid)>,
    Json(req): Json<UpdateInspectionItemRequestDto>,
) -> ApiResult<Json<InspectionItemResponse>> {
    let svc = QualityService::new(state.pool);

    let item = svc.update_inspection_item(inspection_id, item_id, UpdateInspectionItemRequest {
        actual_value: req.actual_value,
        pass_fail: req.pass_fail,
        notes: req.notes,
    }).await?;

    Ok(Json(InspectionItemResponse::from(item)))
}

pub async fn create_ncr(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Json(req): Json<CreateNCRRequestDto>,
) -> ApiResult<Json<NCRResponse>> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id).ok();
    let svc = QualityService::new(state.pool);

    let ncr = svc.create_ncr(CreateNCRRequest {
        source_type: parse_ncr_source(&req.source_type),
        source_id: req.source_id,
        product_id: req.product_id,
        description: req.description,
        severity: parse_ncr_severity(&req.severity),
        assigned_to: req.assigned_to,
    }, user_id).await.map_err(|e| {
        tracing::error!("Failed to create NCR: {:?}", e);
        e
    })?;

    Ok(Json(NCRResponse::from(ncr)))
}

pub async fn get_ncr(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<NCRResponse>> {
    let svc = QualityService::new(state.pool);

    let ncr = svc.get_ncr(id).await?
        .ok_or_else(|| anyhow::anyhow!("NCR not found"))?;

    Ok(Json(NCRResponse::from(ncr)))
}

pub async fn list_ncrs(
    State(state): State<AppState>,
    Query(query): Query<NCRFilterQuery>,
) -> ApiResult<Json<ApiResponse<Vec<NCRResponse>>>> {
    let svc = QualityService::new(state.pool);

    let status = query.status.and_then(|s| parse_ncr_status(&s));
    let severity = query.severity.map(|s| parse_ncr_severity(&s));
    let ncrs = svc.list_ncrs(status, severity).await?;
    let items: Vec<NCRResponse> = ncrs.into_iter().map(NCRResponse::from).collect();

    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn update_ncr(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateNCRRequestDto>,
) -> ApiResult<Json<NCRResponse>> {
    let svc = QualityService::new(state.pool);

    let ncr = svc.update_ncr(id, UpdateNCRRequest {
        root_cause: req.root_cause,
        corrective_action: req.corrective_action,
        preventive_action: req.preventive_action,
        status: req.status.and_then(|s| parse_ncr_status(&s)),
    }).await?;

    Ok(Json(NCRResponse::from(ncr)))
}

pub async fn close_ncr(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<NCRResponse>> {
    let svc = QualityService::new(state.pool);
    let ncr = svc.close_ncr(id).await?;
    Ok(Json(NCRResponse::from(ncr)))
}

pub async fn delete_ncr(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let svc = QualityService::new(state.pool);
    svc.delete_ncr(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn get_analytics(
    State(state): State<AppState>,
) -> ApiResult<Json<QualityAnalytics>> {
    let svc = QualityService::new(state.pool);
    let analytics = svc.get_analytics().await?;
    Ok(Json(analytics))
}
