use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::ApiResult;
use crate::db::AppState;
use crate::handlers::auth::AuthUser;
use erp_stock_transfer::{
    TransferService, StockTransfer, StockTransferLine, StockTransferWithLines, TransferAnalytics,
    CreateTransferRequest, CreateTransferLineRequest, ShipTransferRequest, ReceiveTransferRequest,
    TransferStatus, TransferPriority,
};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Deserialize)]
pub struct TransferFilterQuery {
    pub from_warehouse_id: Option<Uuid>,
    pub to_warehouse_id: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct TransferResponse {
    pub id: String,
    pub transfer_number: String,
    pub from_warehouse_id: String,
    pub to_warehouse_id: String,
    pub status: String,
    pub priority: String,
    pub requested_date: Option<String>,
    pub expected_date: Option<String>,
    pub shipped_date: Option<String>,
    pub received_date: Option<String>,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub shipped_by: Option<String>,
    pub received_by: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<StockTransfer> for TransferResponse {
    fn from(t: StockTransfer) -> Self {
        Self {
            id: t.base.id.to_string(),
            transfer_number: t.transfer_number,
            from_warehouse_id: t.from_warehouse_id.to_string(),
            to_warehouse_id: t.to_warehouse_id.to_string(),
            status: format!("{:?}", t.status),
            priority: format!("{:?}", t.priority),
            requested_date: t.requested_date.map(|d| d.to_rfc3339()),
            expected_date: t.expected_date.map(|d| d.to_rfc3339()),
            shipped_date: t.shipped_date.map(|d| d.to_rfc3339()),
            received_date: t.received_date.map(|d| d.to_rfc3339()),
            approved_by: t.approved_by.map(|id| id.to_string()),
            approved_at: t.approved_at.map(|d| d.to_rfc3339()),
            shipped_by: t.shipped_by.map(|id| id.to_string()),
            received_by: t.received_by.map(|id| id.to_string()),
            notes: t.notes,
            created_at: t.base.created_at.to_rfc3339(),
            updated_at: t.base.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct TransferLineResponse {
    pub id: String,
    pub transfer_id: String,
    pub product_id: String,
    pub requested_quantity: i64,
    pub shipped_quantity: i64,
    pub received_quantity: i64,
    pub unit_cost: i64,
    pub notes: Option<String>,
    pub created_at: String,
}

impl From<StockTransferLine> for TransferLineResponse {
    fn from(l: StockTransferLine) -> Self {
        Self {
            id: l.id.to_string(),
            transfer_id: l.transfer_id.to_string(),
            product_id: l.product_id.to_string(),
            requested_quantity: l.requested_quantity,
            shipped_quantity: l.shipped_quantity,
            received_quantity: l.received_quantity,
            unit_cost: l.unit_cost,
            notes: l.notes,
            created_at: l.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct TransferWithLinesResponse {
    #[serde(flatten)]
    pub transfer: TransferResponse,
    pub lines: Vec<TransferLineResponse>,
}

impl From<StockTransferWithLines> for TransferWithLinesResponse {
    fn from(t: StockTransferWithLines) -> Self {
        Self {
            transfer: TransferResponse::from(t.transfer),
            lines: t.lines.into_iter().map(TransferLineResponse::from).collect(),
        }
    }
}

#[derive(Deserialize)]
pub struct CreateTransferRequestDto {
    pub from_warehouse_id: Uuid,
    pub to_warehouse_id: Uuid,
    pub priority: String,
    pub requested_date: Option<String>,
    pub expected_date: Option<String>,
    pub notes: Option<String>,
    pub lines: Vec<CreateTransferLineRequestDto>,
}

#[derive(Deserialize)]
pub struct CreateTransferLineRequestDto {
    pub product_id: Uuid,
    pub requested_quantity: i64,
    pub unit_cost: i64,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct ShipRequestDto {
    pub lines: Vec<ShipLineDto>,
}

#[derive(Deserialize)]
pub struct ShipLineDto {
    pub product_id: Uuid,
    pub shipped_quantity: i64,
}

#[derive(Deserialize)]
pub struct ReceiveRequestDto {
    pub lines: Vec<ReceiveLineDto>,
}

#[derive(Deserialize)]
pub struct ReceiveLineDto {
    pub product_id: Uuid,
    pub received_quantity: i64,
}

#[derive(Deserialize)]
pub struct RejectRequestDto {
    pub reason: String,
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/", axum::routing::get(list_transfers).post(create_transfer))
        .route("/analytics", axum::routing::get(get_analytics))
        .route("/:id", axum::routing::get(get_transfer).delete(delete_transfer))
        .route("/:id/submit", axum::routing::post(submit_transfer))
        .route("/:id/approve", axum::routing::post(approve_transfer))
        .route("/:id/reject", axum::routing::post(reject_transfer))
        .route("/:id/ship", axum::routing::post(ship_transfer))
        .route("/:id/receive", axum::routing::post(receive_transfer))
        .route("/:id/cancel", axum::routing::post(cancel_transfer))
        .route("/:id/lines", axum::routing::get(list_lines).post(add_line))
}

fn parse_priority(s: &str) -> TransferPriority {
    match s {
        "Low" => TransferPriority::Low,
        "High" => TransferPriority::High,
        "Urgent" => TransferPriority::Urgent,
        _ => TransferPriority::Normal,
    }
}

fn parse_status(s: &str) -> Option<TransferStatus> {
    match s {
        "Draft" => Some(TransferStatus::Draft),
        "Pending" => Some(TransferStatus::Pending),
        "Approved" => Some(TransferStatus::Approved),
        "InTransit" => Some(TransferStatus::InTransit),
        "Received" => Some(TransferStatus::Received),
        "PartiallyReceived" => Some(TransferStatus::PartiallyReceived),
        "Cancelled" => Some(TransferStatus::Cancelled),
        _ => None,
    }
}

pub async fn create_transfer(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Json(req): Json<CreateTransferRequestDto>,
) -> ApiResult<Json<TransferWithLinesResponse>> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id).ok();
    let svc = TransferService::new(state.pool);
    
    let lines: Vec<CreateTransferLineRequest> = req.lines.into_iter().map(|l| CreateTransferLineRequest {
        product_id: l.product_id,
        requested_quantity: l.requested_quantity,
        unit_cost: l.unit_cost,
        notes: l.notes,
    }).collect();
    
    let requested_date = req.requested_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&chrono::Utc));
    let expected_date = req.expected_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&chrono::Utc));
    
    let result = svc.create(CreateTransferRequest {
        from_warehouse_id: req.from_warehouse_id,
        to_warehouse_id: req.to_warehouse_id,
        priority: parse_priority(&req.priority),
        requested_date,
        expected_date,
        notes: req.notes,
        lines,
    }, user_id).await.map_err(|e| {
        tracing::error!("Failed to create transfer: {:?}", e);
        e
    })?;
    
    Ok(Json(TransferWithLinesResponse::from(result)))
}

pub async fn get_transfer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<TransferWithLinesResponse>> {
    let svc = TransferService::new(state.pool);
    
    let result = svc.get(id).await?
        .ok_or_else(|| anyhow::anyhow!("Transfer not found"))?;
    
    Ok(Json(TransferWithLinesResponse::from(result)))
}

pub async fn list_transfers(
    State(state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<TransferFilterQuery>,
) -> ApiResult<Json<ApiResponse<Vec<TransferResponse>>>> {
    let svc = TransferService::new(state.pool);
    
    let status = query.status.and_then(|s| parse_status(&s));
    let transfers = svc.list(query.from_warehouse_id, query.to_warehouse_id, status).await?;
    let items: Vec<TransferResponse> = transfers.into_iter().map(TransferResponse::from).collect();
    
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn submit_transfer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<TransferResponse>> {
    let svc = TransferService::new(state.pool);
    let transfer = svc.submit(id).await?;
    Ok(Json(TransferResponse::from(transfer)))
}

pub async fn approve_transfer(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<TransferResponse>> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    
    let svc = TransferService::new(state.pool);
    let transfer = svc.approve(id, user_id).await?;
    Ok(Json(TransferResponse::from(transfer)))
}

pub async fn reject_transfer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<RejectRequestDto>,
) -> ApiResult<Json<TransferResponse>> {
    let svc = TransferService::new(state.pool);
    let transfer = svc.reject(id, req.reason).await?;
    Ok(Json(TransferResponse::from(transfer)))
}

pub async fn ship_transfer(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(req): Json<ShipRequestDto>,
) -> ApiResult<Json<TransferResponse>> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    
    let svc = TransferService::new(state.pool);
    let ship_req = ShipTransferRequest {
        lines: req.lines.into_iter().map(|l| erp_stock_transfer::ShipLineRequest {
            product_id: l.product_id,
            shipped_quantity: l.shipped_quantity,
        }).collect(),
    };
    let transfer = svc.ship(id, user_id, ship_req).await?;
    Ok(Json(TransferResponse::from(transfer)))
}

pub async fn receive_transfer(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(req): Json<ReceiveRequestDto>,
) -> ApiResult<Json<TransferResponse>> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    
    let svc = TransferService::new(state.pool);
    let recv_req = ReceiveTransferRequest {
        lines: req.lines.into_iter().map(|l| erp_stock_transfer::ReceiveLineRequest {
            product_id: l.product_id,
            received_quantity: l.received_quantity,
        }).collect(),
    };
    let transfer = svc.receive(id, user_id, recv_req).await?;
    Ok(Json(TransferResponse::from(transfer)))
}

pub async fn cancel_transfer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<TransferResponse>> {
    let svc = TransferService::new(state.pool);
    let transfer = svc.cancel(id).await?;
    Ok(Json(TransferResponse::from(transfer)))
}

pub async fn delete_transfer(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let svc = TransferService::new(state.pool);
    svc.delete(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn list_lines(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<Vec<TransferLineResponse>>>> {
    let svc = TransferService::new(state.pool);
    let lines = svc.get_lines(id).await?;
    let items: Vec<TransferLineResponse> = lines.into_iter().map(TransferLineResponse::from).collect();
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn add_line(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<CreateTransferLineRequestDto>,
) -> ApiResult<Json<TransferLineResponse>> {
    let svc = TransferService::new(state.pool);
    
    let line = svc.add_line(id, CreateTransferLineRequest {
        product_id: req.product_id,
        requested_quantity: req.requested_quantity,
        unit_cost: req.unit_cost,
        notes: req.notes,
    }).await?;
    
    Ok(Json(TransferLineResponse::from(line)))
}

pub async fn get_analytics(
    State(state): State<AppState>,
) -> ApiResult<Json<TransferAnalytics>> {
    let svc = TransferService::new(state.pool);
    let analytics = svc.get_analytics().await?;
    Ok(Json(analytics))
}
