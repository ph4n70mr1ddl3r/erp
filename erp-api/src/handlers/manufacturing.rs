use axum::{extract::{Path, Query, State}, Json};
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{BaseEntity, Status, Pagination};
use erp_manufacturing::{BillOfMaterial, BomComponent, WorkOrder, BillOfMaterialService, WorkOrderService};

#[derive(Deserialize)] pub struct CreateBomRequest { pub product_id: Uuid, pub name: String, pub quantity: i64, pub components: Vec<BomComponentRequest> }
#[derive(Deserialize)] pub struct BomComponentRequest { pub product_id: Uuid, pub quantity: i64, pub unit: String }

#[derive(Serialize)] pub struct BomResponse { pub id: Uuid, pub product_id: Uuid, pub name: String, pub quantity: i64, pub status: String, pub components: Vec<BomComponentResponse> }
#[derive(Serialize)] pub struct BomComponentResponse { pub product_id: Uuid, pub quantity: i64, pub unit: String }

impl From<BillOfMaterial> for BomResponse {
    fn from(b: BillOfMaterial) -> Self { Self { id: b.base.id, product_id: b.product_id, name: b.name, quantity: b.quantity, status: format!("{:?}", b.status),
        components: b.components.into_iter().map(|c| BomComponentResponse { product_id: c.product_id, quantity: c.quantity, unit: c.unit }).collect() }
    }
}

pub async fn list_boms(State(state): State<AppState>, Query(pagination): Query<Pagination>) -> ApiResult<Json<erp_core::Paginated<BomResponse>>> {
    let svc = BillOfMaterialService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(res.items.into_iter().map(BomResponse::from).collect(), res.total, Pagination { page: res.page, per_page: res.per_page })))
}

pub async fn get_bom(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<BomResponse>> {
    Ok(Json(BomResponse::from(BillOfMaterialService::new().get(&state.pool, id).await?)))
}

pub async fn create_bom(State(state): State<AppState>, Json(req): Json<CreateBomRequest>) -> ApiResult<Json<BomResponse>> {
    let svc = BillOfMaterialService::new();
    let bom = BillOfMaterial {
        base: BaseEntity::new(), product_id: req.product_id, name: req.name, version: "1.0".to_string(), quantity: req.quantity,
        components: req.components.into_iter().map(|c| BomComponent { id: Uuid::nil(), product_id: c.product_id, quantity: c.quantity, unit: c.unit, scrap_percent: 0.0 }).collect(),
        operations: vec![], status: Status::Draft,
    };
    Ok(Json(BomResponse::from(svc.create(&state.pool, bom).await?)))
}

#[derive(Deserialize)] pub struct CreateWORequest { pub product_id: Uuid, pub bom_id: Uuid, pub quantity: i64, pub planned_start: String, pub planned_end: String }

#[derive(Serialize)] pub struct WOResponse { pub id: Uuid, pub order_number: String, pub product_id: Uuid, pub quantity: i64, pub status: String }
impl From<WorkOrder> for WOResponse {
    fn from(wo: WorkOrder) -> Self { Self { id: wo.base.id, order_number: wo.order_number, product_id: wo.product_id, quantity: wo.quantity, status: format!("{:?}", wo.status) } }
}

pub async fn list_work_orders(State(state): State<AppState>, Query(pagination): Query<Pagination>) -> ApiResult<Json<erp_core::Paginated<WOResponse>>> {
    let svc = WorkOrderService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(res.items.into_iter().map(WOResponse::from).collect(), res.total, Pagination { page: res.page, per_page: res.per_page })))
}

pub async fn create_work_order(State(state): State<AppState>, Json(req): Json<CreateWORequest>) -> ApiResult<Json<WOResponse>> {
    let svc = WorkOrderService::new();
    let wo = WorkOrder {
        base: BaseEntity::new(), order_number: String::new(), product_id: req.product_id, bom_id: req.bom_id, quantity: req.quantity,
        planned_start: chrono::DateTime::parse_from_rfc3339(&req.planned_start).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        planned_end: chrono::DateTime::parse_from_rfc3339(&req.planned_end).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        actual_start: None, actual_end: None, status: Status::Draft,
    };
    Ok(Json(WOResponse::from(svc.create(&state.pool, wo).await?)))
}

pub async fn start_work_order(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    WorkOrderService::new().start(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "in_progress" })))
}

pub async fn complete_work_order(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    WorkOrderService::new().complete(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "completed" })))
}
