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
use erp_inventory_adjustment::{
    AdjustmentService,
    InventoryAdjustment, InventoryAdjustmentLine, InventoryAdjustmentWithLines, AdjustmentAnalytics,
    CreateAdjustmentRequest, CreateAdjustmentLineRequest,
    AdjustmentType, AdjustmentStatus,
};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Deserialize)]
pub struct AdjustmentFilterQuery {
    pub warehouse_id: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct AdjustmentResponse {
    pub id: String,
    pub adjustment_number: String,
    pub warehouse_id: String,
    pub adjustment_type: String,
    pub reason: String,
    pub status: String,
    pub total_value_change: i64,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<InventoryAdjustment> for AdjustmentResponse {
    fn from(a: InventoryAdjustment) -> Self {
        Self {
            id: a.base.id.to_string(),
            adjustment_number: a.adjustment_number,
            warehouse_id: a.warehouse_id.to_string(),
            adjustment_type: format!("{:?}", a.adjustment_type),
            reason: a.reason,
            status: format!("{:?}", a.status),
            total_value_change: a.total_value_change,
            approved_by: a.approved_by.map(|id| id.to_string()),
            approved_at: a.approved_at.map(|d| d.to_rfc3339()),
            notes: a.notes,
            created_at: a.base.created_at.to_rfc3339(),
            updated_at: a.base.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct AdjustmentLineResponse {
    pub id: String,
    pub adjustment_id: String,
    pub product_id: String,
    pub location_id: String,
    pub system_quantity: i64,
    pub counted_quantity: i64,
    pub adjustment_quantity: i64,
    pub unit_cost: i64,
    pub total_value_change: i64,
    pub lot_number: Option<String>,
    pub serial_number: Option<String>,
    pub reason_code: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}

impl From<InventoryAdjustmentLine> for AdjustmentLineResponse {
    fn from(l: InventoryAdjustmentLine) -> Self {
        Self {
            id: l.id.to_string(),
            adjustment_id: l.adjustment_id.to_string(),
            product_id: l.product_id.to_string(),
            location_id: l.location_id.to_string(),
            system_quantity: l.system_quantity,
            counted_quantity: l.counted_quantity,
            adjustment_quantity: l.adjustment_quantity,
            unit_cost: l.unit_cost,
            total_value_change: l.total_value_change,
            lot_number: l.lot_number,
            serial_number: l.serial_number,
            reason_code: l.reason_code,
            notes: l.notes,
            created_at: l.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct AdjustmentWithLinesResponse {
    #[serde(flatten)]
    pub adjustment: AdjustmentResponse,
    pub lines: Vec<AdjustmentLineResponse>,
}

impl From<InventoryAdjustmentWithLines> for AdjustmentWithLinesResponse {
    fn from(a: InventoryAdjustmentWithLines) -> Self {
        Self {
            adjustment: AdjustmentResponse::from(a.adjustment),
            lines: a.lines.into_iter().map(AdjustmentLineResponse::from).collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateAdjustmentRequestDto {
    pub warehouse_id: Uuid,
    pub adjustment_type: String,
    pub reason: String,
    pub notes: Option<String>,
    pub lines: Vec<CreateAdjustmentLineRequestDto>,
}

#[derive(Deserialize)]
pub struct CreateAdjustmentLineRequestDto {
    pub product_id: Uuid,
    pub location_id: Uuid,
    pub system_quantity: i64,
    pub counted_quantity: i64,
    pub unit_cost: i64,
    pub lot_number: Option<String>,
    pub serial_number: Option<String>,
    pub reason_code: Option<String>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct RejectRequestDto {
    pub reason: String,
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", axum::routing::get(list_adjustments).post(create_adjustment))
        .route("/analytics", axum::routing::get(get_analytics))
        .route("/:id", axum::routing::get(get_adjustment).delete(delete_adjustment))
        .route("/:id/submit", axum::routing::post(submit_adjustment))
        .route("/:id/approve", axum::routing::post(approve_adjustment))
        .route("/:id/reject", axum::routing::post(reject_adjustment))
        .route("/:id/complete", axum::routing::post(complete_adjustment))
        .route("/:id/cancel", axum::routing::post(cancel_adjustment))
        .route("/:id/lines", axum::routing::get(list_lines).post(add_line))
}

fn parse_adjustment_type(s: &str) -> AdjustmentType {
    match s {
        "CountVariance" => AdjustmentType::CountVariance,
        "Damage" => AdjustmentType::Damage,
        "Theft" => AdjustmentType::Theft,
        "Expired" => AdjustmentType::Expired,
        "Obsolete" => AdjustmentType::Obsolete,
        "Found" => AdjustmentType::Found,
        "TransferCorrection" => AdjustmentType::TransferCorrection,
        _ => AdjustmentType::Other,
    }
}

fn parse_status(s: &str) -> Option<AdjustmentStatus> {
    match s {
        "Draft" => Some(AdjustmentStatus::Draft),
        "Pending" => Some(AdjustmentStatus::Pending),
        "Approved" => Some(AdjustmentStatus::Approved),
        "Rejected" => Some(AdjustmentStatus::Rejected),
        "Completed" => Some(AdjustmentStatus::Completed),
        "Cancelled" => Some(AdjustmentStatus::Cancelled),
        _ => None,
    }
}

pub async fn create_adjustment(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Json(req): Json<CreateAdjustmentRequestDto>,
) -> ApiResult<Json<AdjustmentWithLinesResponse>> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id).ok();
    let svc = AdjustmentService::new(state.pool);
    
    let lines: Vec<CreateAdjustmentLineRequest> = req.lines.into_iter().map(|l| CreateAdjustmentLineRequest {
        product_id: l.product_id,
        location_id: l.location_id,
        system_quantity: l.system_quantity,
        counted_quantity: l.counted_quantity,
        unit_cost: l.unit_cost,
        lot_number: l.lot_number,
        serial_number: l.serial_number,
        reason_code: l.reason_code,
        notes: l.notes,
    }).collect();
    
    let result = svc.create(CreateAdjustmentRequest {
        warehouse_id: req.warehouse_id,
        adjustment_type: parse_adjustment_type(&req.adjustment_type),
        reason: req.reason,
        notes: req.notes,
        lines,
    }, user_id).await.map_err(|e| {
        tracing::error!("Failed to create adjustment: {:?}", e);
        e
    })?;
    
    Ok(Json(AdjustmentWithLinesResponse::from(result)))
}

pub async fn get_adjustment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<AdjustmentWithLinesResponse>> {
    let svc = AdjustmentService::new(state.pool);
    
    let result = svc.get(id).await?
        .ok_or_else(|| anyhow::anyhow!("Adjustment not found"))?;
    
    Ok(Json(AdjustmentWithLinesResponse::from(result)))
}

pub async fn list_adjustments(
    State(state): State<AppState>,
    Query(query): Query<AdjustmentFilterQuery>,
) -> ApiResult<Json<ApiResponse<Vec<AdjustmentResponse>>>> {
    let svc = AdjustmentService::new(state.pool);
    
    let status = query.status.and_then(|s| parse_status(&s));
    let adjustments = svc.list(query.warehouse_id, status).await?;
    let items: Vec<AdjustmentResponse> = adjustments.into_iter().map(AdjustmentResponse::from).collect();
    
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn submit_adjustment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<AdjustmentResponse>> {
    let svc = AdjustmentService::new(state.pool);
    let adjustment = svc.submit(id).await?;
    Ok(Json(AdjustmentResponse::from(adjustment)))
}

pub async fn approve_adjustment(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<AdjustmentResponse>> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    
    let svc = AdjustmentService::new(state.pool);
    let adjustment = svc.approve(id, user_id).await?;
    Ok(Json(AdjustmentResponse::from(adjustment)))
}

pub async fn reject_adjustment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<RejectRequestDto>,
) -> ApiResult<Json<AdjustmentResponse>> {
    let svc = AdjustmentService::new(state.pool);
    let adjustment = svc.reject(id, req.reason).await?;
    Ok(Json(AdjustmentResponse::from(adjustment)))
}

pub async fn complete_adjustment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<AdjustmentResponse>> {
    let svc = AdjustmentService::new(state.pool);
    let adjustment = svc.complete(id).await?;
    Ok(Json(AdjustmentResponse::from(adjustment)))
}

pub async fn cancel_adjustment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<AdjustmentResponse>> {
    let svc = AdjustmentService::new(state.pool);
    let adjustment = svc.cancel(id).await?;
    Ok(Json(AdjustmentResponse::from(adjustment)))
}

pub async fn delete_adjustment(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let svc = AdjustmentService::new(state.pool);
    svc.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_lines(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<Vec<AdjustmentLineResponse>>>> {
    let svc = AdjustmentService::new(state.pool);
    let lines = svc.get_lines(id).await?;
    let items: Vec<AdjustmentLineResponse> = lines.into_iter().map(AdjustmentLineResponse::from).collect();
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn add_line(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateAdjustmentLineRequestDto>,
) -> ApiResult<Json<AdjustmentLineResponse>> {
    let svc = AdjustmentService::new(state.pool);
    
    let line = svc.add_line(id, CreateAdjustmentLineRequest {
        product_id: req.product_id,
        location_id: req.location_id,
        system_quantity: req.system_quantity,
        counted_quantity: req.counted_quantity,
        unit_cost: req.unit_cost,
        lot_number: req.lot_number,
        serial_number: req.serial_number,
        reason_code: req.reason_code,
        notes: req.notes,
    }).await?;
    
    Ok(Json(AdjustmentLineResponse::from(line)))
}

pub async fn get_analytics(
    State(state): State<AppState>,
) -> ApiResult<Json<AdjustmentAnalytics>> {
    let svc = AdjustmentService::new(state.pool);
    let analytics = svc.get_analytics().await?;
    Ok(Json(analytics))
}
