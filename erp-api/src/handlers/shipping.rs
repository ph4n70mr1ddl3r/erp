use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

use crate::db::AppState;
use erp_shipping::{CarrierService, ShipmentService, CarrierType, CreateShipmentRequest, GetRatesRequest, ShipmentItemRequest, ShipmentStatus};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/carriers", get(list_carriers).post(create_carrier))
        .route("/shipments", get(list_pending_shipments).post(create_shipment))
        .route("/shipments/:id", get(get_shipment))
        .route("/shipments/:id/status", post(update_shipment_status))
        .route("/shipments/:id/label", post(generate_label))
        .route("/shipments/:id/tracking", post(add_tracking_event))
        .route("/rates", post(get_rates))
}

#[derive(Deserialize)]
pub struct CreateCarrierBody {
    pub code: String,
    pub name: String,
    #[serde(default)]
    pub carrier_type: String,
    pub tracking_url_template: Option<String>,
}

async fn create_carrier(
    State(state): State<AppState>,
    Json(body): Json<CreateCarrierBody>,
) -> Json<serde_json::Value> {
    let carrier_type = match body.carrier_type.as_str() {
        "FedEx" => CarrierType::FedEx,
        "UPS" => CarrierType::UPS,
        "DHL" => CarrierType::DHL,
        "USPS" => CarrierType::USPS,
        "CanadaPost" => CarrierType::CanadaPost,
        "RoyalMail" => CarrierType::RoyalMail,
        "DPD" => CarrierType::DPD,
        _ => CarrierType::Other,
    };
    match CarrierService::create(&state.pool, body.code, body.name, carrier_type, body.tracking_url_template).await {
        Ok(carrier) => Json(json!({
            "id": carrier.id,
            "code": carrier.code,
            "name": carrier.name,
            "carrier_type": carrier.carrier_type
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn list_carriers(State(state): State<AppState>) -> Json<serde_json::Value> {
    match CarrierService::list_active(&state.pool).await {
        Ok(carriers) => Json(json!({
            "items": carriers.iter().map(|c| json!({
                "id": c.id,
                "code": c.code,
                "name": c.name,
                "carrier_type": c.carrier_type
            })).collect::<Vec<_>>()
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct CreateShipmentBody {
    pub order_id: Option<Uuid>,
    pub carrier_id: Uuid,
    pub carrier_service_id: Uuid,
    pub ship_to_name: String,
    pub ship_to_street: String,
    pub ship_to_city: String,
    pub ship_to_state: Option<String>,
    pub ship_to_postal_code: String,
    pub ship_to_country: String,
    pub ship_to_phone: Option<String>,
    pub weight: f64,
    #[serde(default = "default_weight_unit")]
    pub weight_unit: String,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    #[serde(default = "default_dim_unit")]
    pub dimension_unit: String,
    pub items: Vec<ShipmentItemBody>,
    pub notes: Option<String>,
}

fn default_weight_unit() -> String { "lb".to_string() }
fn default_dim_unit() -> String { "in".to_string() }

#[derive(Deserialize)]
pub struct ShipmentItemBody {
    pub product_id: Uuid,
    pub quantity: i32,
    pub weight: f64,
    pub declared_value: i64,
}

async fn create_shipment(
    State(state): State<AppState>,
    Json(body): Json<CreateShipmentBody>,
) -> Json<serde_json::Value> {
    let req = CreateShipmentRequest {
        order_id: body.order_id,
        carrier_id: body.carrier_id,
        carrier_service_id: body.carrier_service_id,
        ship_to_name: body.ship_to_name,
        ship_to_street: body.ship_to_street,
        ship_to_city: body.ship_to_city,
        ship_to_state: body.ship_to_state,
        ship_to_postal_code: body.ship_to_postal_code,
        ship_to_country: body.ship_to_country,
        ship_to_phone: body.ship_to_phone,
        weight: body.weight,
        weight_unit: body.weight_unit,
        length: body.length,
        width: body.width,
        height: body.height,
        dimension_unit: body.dimension_unit,
        items: body.items.iter().map(|i| ShipmentItemRequest {
            product_id: i.product_id,
            quantity: i.quantity,
            weight: i.weight,
            declared_value: i.declared_value,
        }).collect(),
        notes: body.notes,
    };
    match ShipmentService::create(&state.pool, req).await {
        Ok(shipment) => Json(json!({
            "id": shipment.id,
            "shipment_number": shipment.shipment_number,
            "status": shipment.status,
            "shipping_cost": shipment.shipping_cost,
            "estimated_delivery": shipment.estimated_delivery
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn get_shipment(State(state): State<AppState>, Path(id): Path<Uuid>) -> Json<serde_json::Value> {
    match ShipmentService::get(&state.pool, id).await {
        Ok(Some(shipment)) => Json(json!({
            "id": shipment.id,
            "shipment_number": shipment.shipment_number,
            "status": shipment.status,
            "tracking_number": shipment.tracking_number,
            "shipping_cost": shipment.shipping_cost
        })),
        Ok(None) => Json(json!({ "error": "Shipment not found" })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn list_pending_shipments(State(state): State<AppState>) -> Json<serde_json::Value> {
    match ShipmentService::list_pending(&state.pool).await {
        Ok(shipments) => Json(json!({
            "items": shipments.iter().map(|s| json!({
                "id": s.id,
                "shipment_number": s.shipment_number,
                "status": s.status,
                "ship_to_name": s.ship_to_name
            })).collect::<Vec<_>>()
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct UpdateStatusBody {
    pub status: String,
}

async fn update_shipment_status(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<UpdateStatusBody>,
) -> Json<serde_json::Value> {
    let status = match body.status.as_str() {
        "Picked" => ShipmentStatus::Picked,
        "Packed" => ShipmentStatus::Packed,
        "Shipped" => ShipmentStatus::Shipped,
        "InTransit" => ShipmentStatus::InTransit,
        "OutForDelivery" => ShipmentStatus::OutForDelivery,
        "Delivered" => ShipmentStatus::Delivered,
        "Returned" => ShipmentStatus::Returned,
        "Cancelled" => ShipmentStatus::Cancelled,
        _ => ShipmentStatus::Pending,
    };
    match ShipmentService::update_status(&state.pool, id, status).await {
        Ok(shipment) => Json(json!({
            "id": shipment.id,
            "status": shipment.status
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn generate_label(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    match ShipmentService::generate_label(&state.pool, id).await {
        Ok(label_url) => Json(json!({ "label_url": label_url })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct AddTrackingBody {
    pub event_type: String,
    pub description: String,
    pub location: Option<String>,
}

async fn add_tracking_event(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<AddTrackingBody>,
) -> Json<serde_json::Value> {
    match ShipmentService::add_tracking_event(&state.pool, id, body.event_type, body.description, body.location).await {
        Ok(event) => Json(json!({
            "id": event.id,
            "event_type": event.event_type,
            "occurred_at": event.occurred_at
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct GetRatesBody {
    pub ship_from_postal_code: String,
    pub ship_from_country: String,
    pub ship_to_postal_code: String,
    pub ship_to_country: String,
    pub weight: f64,
    pub weight_unit: String,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub dimension_unit: String,
    pub declared_value: i64,
    pub currency: String,
}

async fn get_rates(
    State(state): State<AppState>,
    Json(body): Json<GetRatesBody>,
) -> Json<serde_json::Value> {
    let req = GetRatesRequest {
        ship_from_postal_code: body.ship_from_postal_code,
        ship_from_country: body.ship_from_country,
        ship_to_postal_code: body.ship_to_postal_code,
        ship_to_country: body.ship_to_country,
        weight: body.weight,
        weight_unit: body.weight_unit,
        length: body.length,
        width: body.width,
        height: body.height,
        dimension_unit: body.dimension_unit,
        declared_value: body.declared_value,
        currency: body.currency,
    };
    match ShipmentService::get_rates(&state.pool, req).await {
        Ok(quotes) => Json(json!({
            "rates": quotes.iter().map(|q| json!({
                "carrier_id": q.carrier_id,
                "carrier_name": q.carrier_name,
                "service_name": q.service_name,
                "total_charge": q.total_charge,
                "currency": q.currency,
                "estimated_days": q.estimated_days
            })).collect::<Vec<_>>()
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}
