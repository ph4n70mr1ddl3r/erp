use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ShipmentStatus {
    Pending,
    Picked,
    Packed,
    Shipped,
    InTransit,
    OutForDelivery,
    Delivered,
    Returned,
    Cancelled,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CarrierType {
    FedEx,
    UPS,
    DHL,
    USPS,
    CanadaPost,
    RoyalMail,
    DPD,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Carrier {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub carrier_type: CarrierType,
    pub api_key: Option<String>,
    pub api_secret: Option<String>,
    pub account_number: Option<String>,
    pub tracking_url_template: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarrierServiceOption {
    pub id: Uuid,
    pub carrier_id: Uuid,
    pub code: String,
    pub name: String,
    pub service_type: String,
    pub estimated_days_min: i32,
    pub estimated_days_max: i32,
    pub base_rate: i64,
    pub currency: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Shipment {
    pub id: Uuid,
    pub shipment_number: String,
    pub order_id: Option<Uuid>,
    pub carrier_id: Uuid,
    pub carrier_service_id: Uuid,
    pub status: ShipmentStatus,
    pub ship_from_name: String,
    pub ship_from_street: String,
    pub ship_from_city: String,
    pub ship_from_state: Option<String>,
    pub ship_from_postal_code: String,
    pub ship_from_country: String,
    pub ship_to_name: String,
    pub ship_to_street: String,
    pub ship_to_city: String,
    pub ship_to_state: Option<String>,
    pub ship_to_postal_code: String,
    pub ship_to_country: String,
    pub ship_to_phone: Option<String>,
    pub weight: f64,
    pub weight_unit: String,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub dimension_unit: String,
    pub shipping_cost: i64,
    pub insurance_cost: i64,
    pub currency: String,
    pub tracking_number: Option<String>,
    pub tracking_url: Option<String>,
    pub label_url: Option<String>,
    pub shipped_at: Option<DateTime<Utc>>,
    pub estimated_delivery: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentItem {
    pub id: Uuid,
    pub shipment_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub weight: f64,
    pub declared_value: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackingEvent {
    pub id: Uuid,
    pub shipment_id: Uuid,
    pub event_type: String,
    pub description: String,
    pub location: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingZone {
    pub id: Uuid,
    pub name: String,
    pub countries: String,
    pub states: Option<String>,
    pub postal_codes: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShippingRate {
    pub id: Uuid,
    pub zone_id: Uuid,
    pub carrier_service_id: Uuid,
    pub min_weight: f64,
    pub max_weight: f64,
    pub min_value: i64,
    pub max_value: i64,
    pub rate: i64,
    pub currency: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateQuote {
    pub carrier_id: Uuid,
    pub carrier_name: String,
    pub service_code: String,
    pub service_name: String,
    pub total_charge: i64,
    pub currency: String,
    pub estimated_days: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateShipmentRequest {
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
    pub weight_unit: String,
    pub length: f64,
    pub width: f64,
    pub height: f64,
    pub dimension_unit: String,
    pub items: Vec<ShipmentItemRequest>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShipmentItemRequest {
    pub product_id: Uuid,
    pub quantity: i32,
    pub weight: f64,
    pub declared_value: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRatesRequest {
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
