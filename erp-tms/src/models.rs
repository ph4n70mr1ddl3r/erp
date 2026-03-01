use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum LoadStatus {
    Planned,
    Building,
    Ready,
    InTransit,
    Delivered,
    Cancelled,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum RouteStatus {
    Planned,
    Active,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum FreightAuditStatus {
    Pending,
    Validated,
    Disputed,
    Approved,
    Paid,
    Rejected,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum DriverStatus {
    Available,
    OnDuty,
    Driving,
    OffDuty,
    Resting,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum VehicleStatus {
    Available,
    InUse,
    Maintenance,
    OutOfService,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Vehicle {
    pub id: Uuid,
    pub vehicle_number: String,
    pub vehicle_type: String,
    pub license_plate: String,
    pub vin: Option<String>,
    pub make: Option<String>,
    pub model: Option<String>,
    pub year: Option<i32>,
    pub capacity_weight: f64,
    pub capacity_volume: f64,
    pub capacity_unit: String,
    pub fuel_type: String,
    pub status: VehicleStatus,
    pub current_location_lat: Option<f64>,
    pub current_location_lng: Option<f64>,
    pub odometer: f64,
    pub last_maintenance: Option<DateTime<Utc>>,
    pub next_maintenance: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Driver {
    pub id: Uuid,
    pub employee_id: Option<Uuid>,
    pub driver_code: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: Option<String>,
    pub license_number: String,
    pub license_class: String,
    pub license_expiry: DateTime<Utc>,
    pub status: DriverStatus,
    pub current_vehicle_id: Option<Uuid>,
    pub current_location_lat: Option<f64>,
    pub current_location_lng: Option<f64>,
    pub hours_driven_today: f64,
    pub hours_available: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct CarrierContract {
    pub id: Uuid,
    pub carrier_id: Uuid,
    pub contract_number: String,
    pub contract_name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub payment_terms: String,
    pub fuel_surcharge_percent: f64,
    pub accessorial_rates: serde_json::Value,
    pub volume_commitment: Option<f64>,
    pub volume_discount_tiers: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FreightRate {
    pub id: Uuid,
    pub contract_id: Uuid,
    pub origin_zone: String,
    pub destination_zone: String,
    pub service_type: String,
    pub rate_type: String,
    pub base_rate: i64,
    pub per_mile_rate: i64,
    pub per_kg_rate: i64,
    pub per_pallet_rate: i64,
    pub min_charge: i64,
    pub max_charge: Option<i64>,
    pub currency: String,
    pub effective_date: DateTime<Utc>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Load {
    pub id: Uuid,
    pub load_number: String,
    pub carrier_id: Option<Uuid>,
    pub driver_id: Option<Uuid>,
    pub vehicle_id: Option<Uuid>,
    pub status: LoadStatus,
    pub origin_name: String,
    pub origin_street: String,
    pub origin_city: String,
    pub origin_state: Option<String>,
    pub origin_postal_code: String,
    pub origin_country: String,
    pub origin_lat: Option<f64>,
    pub origin_lng: Option<f64>,
    pub destination_name: String,
    pub destination_street: String,
    pub destination_city: String,
    pub destination_state: Option<String>,
    pub destination_postal_code: String,
    pub destination_country: String,
    pub destination_lat: Option<f64>,
    pub destination_lng: Option<f64>,
    pub planned_pickup: DateTime<Utc>,
    pub actual_pickup: Option<DateTime<Utc>>,
    pub planned_delivery: DateTime<Utc>,
    pub actual_delivery: Option<DateTime<Utc>>,
    pub total_weight: f64,
    pub total_volume: f64,
    pub total_pallets: i32,
    pub total_pieces: i32,
    pub freight_charge: i64,
    pub fuel_surcharge: i64,
    pub accessorial_charges: i64,
    pub total_charge: i64,
    pub currency: String,
    pub distance_miles: Option<f64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LoadStop {
    pub id: Uuid,
    pub load_id: Uuid,
    pub stop_number: i32,
    pub stop_type: String,
    pub location_name: String,
    pub street: String,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub lat: Option<f64>,
    pub lng: Option<f64>,
    pub planned_arrival: DateTime<Utc>,
    pub actual_arrival: Option<DateTime<Utc>>,
    pub planned_departure: DateTime<Utc>,
    pub actual_departure: Option<DateTime<Utc>>,
    pub appointment_number: Option<String>,
    pub instructions: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LoadItem {
    pub id: Uuid,
    pub load_id: Uuid,
    pub stop_id: Uuid,
    pub shipment_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub description: String,
    pub quantity: i32,
    pub weight: f64,
    pub volume: f64,
    pub pallets: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Route {
    pub id: Uuid,
    pub route_number: String,
    pub route_name: String,
    pub driver_id: Option<Uuid>,
    pub vehicle_id: Option<Uuid>,
    pub status: RouteStatus,
    pub planned_start: DateTime<Utc>,
    pub actual_start: Option<DateTime<Utc>>,
    pub planned_end: DateTime<Utc>,
    pub actual_end: Option<DateTime<Utc>>,
    pub total_distance: f64,
    pub total_duration_minutes: i32,
    pub total_stops: i32,
    pub completed_stops: i32,
    pub total_weight: f64,
    pub optimization_score: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct RouteStop {
    pub id: Uuid,
    pub route_id: Uuid,
    pub stop_sequence: i32,
    pub stop_type: String,
    pub load_id: Option<Uuid>,
    pub location_name: String,
    pub street: String,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub lat: f64,
    pub lng: f64,
    pub planned_arrival: DateTime<Utc>,
    pub actual_arrival: Option<DateTime<Utc>>,
    pub planned_duration_minutes: i32,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FreightInvoice {
    pub id: Uuid,
    pub invoice_number: String,
    pub carrier_id: Uuid,
    pub load_id: Option<Uuid>,
    pub invoice_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
    pub status: FreightAuditStatus,
    pub base_freight: i64,
    pub fuel_surcharge: i64,
    pub accessorial_charges: i64,
    pub tax: i64,
    pub total_amount: i64,
    pub currency: String,
    pub expected_amount: i64,
    pub variance_amount: i64,
    pub variance_reason: Option<String>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub paid_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FreightInvoiceLine {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub line_number: i32,
    pub charge_type: String,
    pub description: String,
    pub quantity: f64,
    pub unit: String,
    pub rate: i64,
    pub amount: i64,
    pub expected_amount: i64,
    pub variance: i64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FuelPurchase {
    pub id: Uuid,
    pub vehicle_id: Option<Uuid>,
    pub driver_id: Option<Uuid>,
    pub purchase_date: DateTime<Utc>,
    pub vendor_name: String,
    pub location: String,
    pub fuel_type: String,
    pub quantity: f64,
    pub unit: String,
    pub price_per_unit: i64,
    pub total_amount: i64,
    pub currency: String,
    pub odometer: f64,
    pub receipt_number: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccessorialCharge {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub default_rate: i64,
    pub currency: String,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteOptimizationRequest {
    pub origin_lat: f64,
    pub origin_lng: f64,
    pub stops: Vec<OptimizationStop>,
    pub vehicle_capacity_weight: Option<f64>,
    pub vehicle_capacity_volume: Option<f64>,
    pub max_driving_hours: Option<f64>,
    pub optimize_for: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationStop {
    pub id: Uuid,
    pub lat: f64,
    pub lng: f64,
    pub stop_type: String,
    pub weight: f64,
    pub volume: f64,
    pub duration_minutes: i32,
    pub time_window_start: Option<DateTime<Utc>>,
    pub time_window_end: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteOptimizationResult {
    pub total_distance: f64,
    pub total_duration_minutes: i32,
    pub optimized_stops: Vec<OptimizedStop>,
    pub total_weight: f64,
    pub total_volume: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizedStop {
    pub stop_id: Uuid,
    pub sequence: i32,
    pub arrival_time: DateTime<Utc>,
    pub departure_time: DateTime<Utc>,
    pub distance_from_previous: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateLoadRequest {
    pub carrier_id: Option<Uuid>,
    pub origin_name: String,
    pub origin_street: String,
    pub origin_city: String,
    pub origin_state: Option<String>,
    pub origin_postal_code: String,
    pub origin_country: String,
    pub destination_name: String,
    pub destination_street: String,
    pub destination_city: String,
    pub destination_state: Option<String>,
    pub destination_postal_code: String,
    pub destination_country: String,
    pub planned_pickup: DateTime<Utc>,
    pub planned_delivery: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateVehicleRequest {
    pub vehicle_number: String,
    pub vehicle_type: String,
    pub license_plate: String,
    pub vin: Option<String>,
    pub make: Option<String>,
    pub model: Option<String>,
    pub year: Option<i32>,
    pub capacity_weight: f64,
    pub capacity_volume: f64,
    pub capacity_unit: String,
    pub fuel_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDriverRequest {
    pub employee_id: Option<Uuid>,
    pub driver_code: String,
    pub first_name: String,
    pub last_name: String,
    pub phone: String,
    pub email: Option<String>,
    pub license_number: String,
    pub license_class: String,
    pub license_expiry: DateTime<Utc>,
}
