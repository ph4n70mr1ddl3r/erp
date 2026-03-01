use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use erp_core::Pagination;
use crate::db::AppState;
use erp_iot::{IoTDeviceService, TelemetryService, IoTAlertService, DeviceCommandService};
use erp_iot::{IoTDevice, DeviceType, DeviceStatus, ConnectivityType, MetricType};

#[derive(Deserialize)]
pub struct RegisterDeviceRequest {
    pub device_id: String,
    pub name: String,
    pub device_type: String,
    pub connectivity_type: String,
    pub description: Option<String>,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub warehouse_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct IngestTelemetryRequest {
    pub device_id: Uuid,
    pub metric_name: String,
    pub value: f64,
    pub unit: Option<String>,
}

#[derive(Deserialize)]
pub struct IngestBatchRequest {
    pub device_id: Uuid,
    pub readings: Vec<(String, f64, Option<String>)>,
}

#[derive(Deserialize)]
pub struct SendCommandRequest {
    pub command_type: String,
    pub payload: Option<String>,
}

#[derive(Serialize)]
pub struct DeviceResponse {
    pub id: Uuid,
    pub device_id: String,
    pub name: String,
    pub device_type: String,
    pub status: String,
    pub last_heartbeat_at: Option<String>,
}

impl From<IoTDevice> for DeviceResponse {
    fn from(d: IoTDevice) -> Self {
        Self {
            id: d.base.id,
            device_id: d.device_id,
            name: d.name,
            device_type: format!("{:?}", d.device_type),
            status: format!("{:?}", d.status),
            last_heartbeat_at: d.last_heartbeat_at.map(|t| t.to_rfc3339()),
        }
    }
}

pub async fn list_devices(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = IoTDeviceService::new();
    let result = service.list(&state.pool, pagination).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "items": result.items.into_iter().map(DeviceResponse::from).collect::<Vec<_>>(),
        "total": result.total
    })))
}

pub async fn get_device(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<DeviceResponse>, (StatusCode, String)> {
    let service = IoTDeviceService::new();
    let device = service.get(&state.pool, id).await
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;
    Ok(Json(DeviceResponse::from(device)))
}

pub async fn register_device(
    State(state): State<AppState>,
    Json(req): Json<RegisterDeviceRequest>,
) -> Result<Json<DeviceResponse>, (StatusCode, String)> {
    let service = IoTDeviceService::new();
    
    let now = chrono::Utc::now();
    let device = IoTDevice {
        base: erp_core::BaseEntity::new(),
        device_id: req.device_id,
        name: req.name,
        description: req.description,
        device_type: match req.device_type.as_str() {
            "Actuator" => DeviceType::Actuator,
            "Gateway" => DeviceType::Gateway,
            "SmartMeter" => DeviceType::SmartMeter,
            "Camera" => DeviceType::Camera,
            _ => DeviceType::Sensor,
        },
        manufacturer: req.manufacturer,
        model: req.model,
        serial_number: None,
        firmware_version: None,
        hardware_version: None,
        status: DeviceStatus::Provisioning,
        connectivity_type: match req.connectivity_type.as_str() {
            "Ethernet" => ConnectivityType::Ethernet,
            "Cellular" => ConnectivityType::Cellular,
            "LoRaWAN" => ConnectivityType::LoRaWAN,
            "MQTT" => ConnectivityType::MQTT,
            _ => ConnectivityType::WiFi,
        },
        ip_address: None,
        mac_address: None,
        port: None,
        protocol_config: None,
        gateway_id: None,
        location_id: None,
        warehouse_id: req.warehouse_id,
        zone: None,
        latitude: None,
        longitude: None,
        geofence_enabled: false,
        geofence_radius_meters: None,
        last_seen_at: None,
        last_heartbeat_at: None,
        heartbeat_interval_seconds: 60,
        battery_level: None,
        signal_strength: None,
        temperature_celsius: None,
        humidity_percent: None,
        data_format: None,
        data_schema: None,
        transforms: None,
        alert_rules: None,
        metadata: None,
        tags: None,
        owner_id: None,
        installed_at: None,
        maintenance_due_at: None,
        created_at: now,
        updated_at: now,
    };
    
    let device = service.register(&state.pool, device).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    Ok(Json(DeviceResponse::from(device)))
}

pub async fn device_heartbeat(
    State(state): State<AppState>,
    Path(device_id): Path<String>,
    Json(payload): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = IoTDeviceService::new();
    
    let battery_level = payload.get("battery_level").and_then(|v| v.as_i64()).map(|v| v as i32);
    let signal_strength = payload.get("signal_strength").and_then(|v| v.as_i64()).map(|v| v as i32);
    let temperature = payload.get("temperature").and_then(|v| v.as_f64());
    let humidity = payload.get("humidity").and_then(|v| v.as_f64());
    
    service.heartbeat(&state.pool, &device_id, battery_level, signal_strength, temperature, humidity).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(serde_json::json!({ "status": "ok" })))
}

pub async fn ingest_telemetry(
    State(state): State<AppState>,
    Json(req): Json<IngestTelemetryRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = TelemetryService::new();
    let data = service.ingest(&state.pool, req.device_id, req.metric_name, MetricType::Gauge, Some(req.value), None, None, req.unit).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "id": data.id,
        "device_id": data.device_id,
        "metric_name": data.metric_name,
        "value": data.value_numeric,
        "timestamp": data.timestamp
    })))
}

pub async fn ingest_batch(
    State(state): State<AppState>,
    Json(req): Json<IngestBatchRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = TelemetryService::new();
    let results = service.ingest_batch(&state.pool, req.device_id, req.readings).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "count": results.len(),
        "device_id": req.device_id
    })))
}

pub async fn get_telemetry(
    State(state): State<AppState>,
    Path(device_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = TelemetryService::new();
    let data = service.get_device_history(&state.pool, device_id, 100).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "device_id": device_id,
        "readings": data.into_iter().map(|d| serde_json::json!({
            "metric_name": d.metric_name,
            "value": d.value_numeric,
            "timestamp": d.timestamp,
            "unit": d.unit
        })).collect::<Vec<_>>()
    })))
}

pub async fn list_alerts(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = IoTAlertService::new();
    let alerts = service.list_unresolved(&state.pool, 100).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "alerts": alerts.into_iter().map(|a| serde_json::json!({
            "id": a.base.id,
            "device_id": a.device_id,
            "alert_type": format!("{:?}", a.alert_type),
            "severity": format!("{:?}", a.severity),
            "title": a.title,
            "message": a.message,
            "status": format!("{:?}", a.status),
            "triggered_at": a.triggered_at
        })).collect::<Vec<_>>()
    })))
}

pub async fn acknowledge_alert(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = IoTAlertService::new();
    service.acknowledge(&state.pool, id, Uuid::nil()).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn resolve_alert(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = IoTAlertService::new();
    service.resolve(&state.pool, id, Some("Resolved via API".to_string())).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn send_command(
    State(state): State<AppState>,
    Path(device_id): Path<Uuid>,
    Json(req): Json<SendCommandRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = DeviceCommandService::new();
    let command = service.send_command(&state.pool, device_id, req.command_type, req.payload).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "id": command.base.id,
        "device_id": command.device_id,
        "command_type": command.command_type,
        "status": format!("{:?}", command.status)
    })))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/devices", axum::routing::get(list_devices).post(register_device))
        .route("/devices/:id", axum::routing::get(get_device))
        .route("/devices/:device_id/heartbeat", axum::routing::post(device_heartbeat))
        .route("/telemetry", axum::routing::post(ingest_telemetry))
        .route("/telemetry/batch", axum::routing::post(ingest_batch))
        .route("/telemetry/:device_id", axum::routing::get(get_telemetry))
        .route("/alerts", axum::routing::get(list_alerts))
        .route("/alerts/:id/acknowledge", axum::routing::post(acknowledge_alert))
        .route("/alerts/:id/resolve", axum::routing::post(resolve_alert))
        .route("/devices/:device_id/command", axum::routing::post(send_command))
}
