use axum::{extract::{Path, Query, State}, Json};
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{BaseEntity, Status, Pagination, Money, Currency, ContactInfo, Address};
use erp_purchasing::{Vendor, PurchaseOrder, PurchaseOrderLine, VendorService, PurchaseOrderService};

#[derive(Deserialize)] pub struct CreateVendorRequest { pub code: String, pub name: String, pub email: Option<String>, pub phone: Option<String>, pub payment_terms: Option<u32> }
#[derive(Serialize)] pub struct VendorResponse { pub id: Uuid, pub code: String, pub name: String, pub email: Option<String>, pub status: String }

impl From<Vendor> for VendorResponse {
    fn from(v: Vendor) -> Self { Self { id: v.base.id, code: v.code, name: v.name, email: v.contact.email, status: format!("{:?}", v.status) } }
}

pub async fn list_vendors(State(state): State<AppState>, Query(pagination): Query<Pagination>) -> ApiResult<Json<erp_core::Paginated<VendorResponse>>> {
    let svc = VendorService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(res.items.into_iter().map(VendorResponse::from).collect(), res.total, Pagination { page: res.page, per_page: res.per_page })))
}

pub async fn get_vendor(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<VendorResponse>> {
    Ok(Json(VendorResponse::from(VendorService::new().get(&state.pool, id).await?)))
}

pub async fn create_vendor(State(state): State<AppState>, Json(req): Json<CreateVendorRequest>) -> ApiResult<Json<VendorResponse>> {
    let svc = VendorService::new();
    let v = Vendor {
        base: BaseEntity::new(), code: req.code, name: req.name,
        contact: ContactInfo { email: req.email, phone: req.phone, fax: None, website: None },
        address: Address { street: String::new(), city: String::new(), state: None, postal_code: String::new(), country: String::new() },
        payment_terms: req.payment_terms.unwrap_or(30), status: Status::Active,
    };
    Ok(Json(VendorResponse::from(svc.create(&state.pool, v).await?)))
}

#[derive(Deserialize)] pub struct CreatePORequest { pub vendor_id: Uuid, pub lines: Vec<POLineRequest> }
#[derive(Deserialize)] pub struct POLineRequest { pub product_id: Uuid, pub description: String, pub quantity: i64, pub unit_price: i64 }

#[derive(Serialize)] pub struct POResponse { pub id: Uuid, pub po_number: String, pub vendor_id: Uuid, pub status: String, pub total: f64 }
impl From<PurchaseOrder> for POResponse {
    fn from(po: PurchaseOrder) -> Self { Self { id: po.base.id, po_number: po.po_number, vendor_id: po.vendor_id, status: format!("{:?}", po.status), total: po.total.to_decimal() } }
}

pub async fn list_orders(State(state): State<AppState>, Query(pagination): Query<Pagination>) -> ApiResult<Json<erp_core::Paginated<POResponse>>> {
    let svc = PurchaseOrderService::new();
    let res = svc.list(&state.pool, pagination).await?;
    Ok(Json(erp_core::Paginated::new(res.items.into_iter().map(POResponse::from).collect(), res.total, Pagination { page: res.page, per_page: res.per_page })))
}

pub async fn get_order(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<POResponse>> {
    Ok(Json(POResponse::from(PurchaseOrderService::new().get(&state.pool, id).await?)))
}

pub async fn create_order(State(state): State<AppState>, Json(req): Json<CreatePORequest>) -> ApiResult<Json<POResponse>> {
    let svc = PurchaseOrderService::new();
    let po = PurchaseOrder {
        base: BaseEntity::new(), po_number: String::new(), vendor_id: req.vendor_id, order_date: Utc::now(), expected_date: None,
        lines: req.lines.into_iter().map(|l| PurchaseOrderLine {
            id: Uuid::nil(), product_id: l.product_id, description: l.description, quantity: l.quantity,
            unit_price: Money::new(l.unit_price, Currency::USD), tax_rate: 0.0, line_total: Money::new(l.quantity * l.unit_price, Currency::USD),
        }).collect(),
        subtotal: Money::zero(Currency::USD), tax_amount: Money::zero(Currency::USD), total: Money::zero(Currency::USD), status: Status::Draft,
    };
    Ok(Json(POResponse::from(svc.create(&state.pool, po).await?)))
}

pub async fn approve_order(State(state): State<AppState>, Path(id): Path<Uuid>) -> ApiResult<Json<serde_json::Value>> {
    PurchaseOrderService::new().approve(&state.pool, id).await?;
    Ok(Json(serde_json::json!({ "status": "approved" })))
}
