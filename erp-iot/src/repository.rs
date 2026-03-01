use async_trait::async_trait;
use sqlx::SqlitePool;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity};
use crate::models::*;
use uuid::Uuid;
use chrono::Utc;

#[async_trait]
pub trait IoTDeviceRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<IoTDevice>;
    async fn find_by_device_id(&self, pool: &SqlitePool, device_id: &str) -> Result<IoTDevice>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<IoTDevice>>;
    async fn create(&self, pool: &SqlitePool, device: IoTDevice) -> Result<IoTDevice>;
    async fn update(&self, pool: &SqlitePool, device: &IoTDevice) -> Result<()>;
    async fn update_heartbeat(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteIoTDeviceRepository;

#[async_trait]
impl IoTDeviceRepository for SqliteIoTDeviceRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<IoTDevice> {
        let row = sqlx::query_as::<_, IoTDeviceRow>(
            "SELECT * FROM iot_devices WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("IoTDevice", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn find_by_device_id(&self, pool: &SqlitePool, device_id: &str) -> Result<IoTDevice> {
        let row = sqlx::query_as::<_, IoTDeviceRow>(
            "SELECT * FROM iot_devices WHERE device_id = ?"
        )
        .bind(device_id)
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("IoTDevice", device_id))?;
        
        Ok(row.into())
    }
    
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<IoTDevice>> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM iot_devices WHERE status = 'Active'")
            .fetch_one(pool)
            .await
            .map_err(Error::Database)?;
        
        let rows = sqlx::query_as::<_, IoTDeviceRow>(
            "SELECT * FROM iot_devices WHERE status = 'Active' ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit())
        .bind(pagination.offset())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(Paginated::new(rows.into_iter().map(|r| r.into()).collect(), count as u64, pagination))
    }
    
    async fn create(&self, pool: &SqlitePool, device: IoTDevice) -> Result<IoTDevice> {
        sqlx::query(
            r#"INSERT INTO iot_devices (id, device_id, name, description, device_type, manufacturer,
               model, serial_number, firmware_version, status, connectivity_type, ip_address,
               mac_address, location_id, warehouse_id, heartbeat_interval_seconds, metadata, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(device.base.id.to_string())
        .bind(&device.device_id)
        .bind(&device.name)
        .bind(&device.description)
        .bind(format!("{:?}", device.device_type))
        .bind(&device.manufacturer)
        .bind(&device.model)
        .bind(&device.serial_number)
        .bind(&device.firmware_version)
        .bind(format!("{:?}", device.status))
        .bind(format!("{:?}", device.connectivity_type))
        .bind(&device.ip_address)
        .bind(&device.mac_address)
        .bind(device.location_id.map(|id| id.to_string()))
        .bind(device.warehouse_id.map(|id| id.to_string()))
        .bind(device.heartbeat_interval_seconds)
        .bind(&device.metadata)
        .bind(device.created_at.to_rfc3339())
        .bind(device.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(device)
    }
    
    async fn update(&self, pool: &SqlitePool, device: &IoTDevice) -> Result<()> {
        sqlx::query(
            r#"UPDATE iot_devices SET name = ?, description = ?, firmware_version = ?, 
               status = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(&device.name)
        .bind(&device.description)
        .bind(&device.firmware_version)
        .bind(format!("{:?}", device.status))
        .bind(device.updated_at.to_rfc3339())
        .bind(device.base.id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        Ok(())
    }
    
    async fn update_heartbeat(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE iot_devices SET last_heartbeat_at = ?, status = 'Online', updated_at = ? WHERE id = ?"
        )
        .bind(Utc::now().to_rfc3339())
        .bind(Utc::now().to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        Ok(())
    }
    
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE iot_devices SET status = 'Disabled' WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct IoTDeviceRow {
    id: String,
    device_id: String,
    name: String,
    description: Option<String>,
    device_type: String,
    manufacturer: Option<String>,
    model: Option<String>,
    serial_number: Option<String>,
    firmware_version: Option<String>,
    status: String,
    connectivity_type: String,
    ip_address: Option<String>,
    mac_address: Option<String>,
    location_id: Option<String>,
    warehouse_id: Option<String>,
    heartbeat_interval_seconds: i32,
    last_heartbeat_at: Option<String>,
    metadata: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<IoTDeviceRow> for IoTDevice {
    fn from(r: IoTDeviceRow) -> Self {
        use chrono::{DateTime, Utc};
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            device_id: r.device_id,
            name: r.name,
            description: r.description,
            device_type: match r.device_type.as_str() {
                "Actuator" => DeviceType::Actuator,
                "Gateway" => DeviceType::Gateway,
                "Controller" => DeviceType::Controller,
                "SmartMeter" => DeviceType::SmartMeter,
                "Camera" => DeviceType::Camera,
                "RFIDReader" => DeviceType::RFIDReader,
                "BarcodeScanner" => DeviceType::BarcodeScanner,
                "Scale" => DeviceType::Scale,
                "Thermostat" => DeviceType::Thermostat,
                "HumiditySensor" => DeviceType::HumiditySensor,
                "MotionDetector" => DeviceType::MotionDetector,
                "GPSTracker" => DeviceType::GPSTracker,
                "Beacon" => DeviceType::Beacon,
                "PLC" => DeviceType::PLC,
                "EdgeDevice" => DeviceType::EdgeDevice,
                "Other" => DeviceType::Other,
                _ => DeviceType::Sensor,
            },
            manufacturer: r.manufacturer,
            model: r.model,
            serial_number: r.serial_number,
            firmware_version: r.firmware_version,
            hardware_version: None,
            status: match r.status.as_str() {
                "Offline" => DeviceStatus::Offline,
                "Maintenance" => DeviceStatus::Maintenance,
                "Error" => DeviceStatus::Error,
                "Disabled" => DeviceStatus::Disabled,
                "Provisioning" => DeviceStatus::Provisioning,
                "Unknown" => DeviceStatus::Unknown,
                _ => DeviceStatus::Online,
            },
            connectivity_type: match r.connectivity_type.as_str() {
                "Ethernet" => ConnectivityType::Ethernet,
                "Cellular" => ConnectivityType::Cellular,
                "LoRaWAN" => ConnectivityType::LoRaWAN,
                "Bluetooth" => ConnectivityType::Bluetooth,
                "Zigbee" => ConnectivityType::Zigbee,
                "MQTT" => ConnectivityType::MQTT,
                "CoAP" => ConnectivityType::CoAP,
                "HTTP" => ConnectivityType::HTTP,
                "Modbus" => ConnectivityType::Modbus,
                "OPCUA" => ConnectivityType::OPCUA,
                "CANBus" => ConnectivityType::CANBus,
                "RS232" => ConnectivityType::RS232,
                "RS485" => ConnectivityType::RS485,
                "USB" => ConnectivityType::USB,
                _ => ConnectivityType::WiFi,
            },
            ip_address: r.ip_address,
            mac_address: r.mac_address,
            port: None,
            protocol_config: None,
            gateway_id: None,
            location_id: r.location_id.and_then(|id| Uuid::parse_str(&id).ok()),
            warehouse_id: r.warehouse_id.and_then(|id| Uuid::parse_str(&id).ok()),
            zone: None,
            latitude: None,
            longitude: None,
            geofence_enabled: false,
            geofence_radius_meters: None,
            last_seen_at: None,
            last_heartbeat_at: r.last_heartbeat_at.and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            heartbeat_interval_seconds: r.heartbeat_interval_seconds,
            battery_level: None,
            signal_strength: None,
            temperature_celsius: None,
            humidity_percent: None,
            data_format: None,
            data_schema: None,
            transforms: None,
            alert_rules: None,
            metadata: r.metadata,
            tags: None,
            owner_id: None,
            installed_at: None,
            maintenance_due_at: None,
            created_at: DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            updated_at: DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[async_trait]
pub trait TelemetryRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, data: TelemetryData) -> Result<TelemetryData>;
    async fn find_by_device(&self, pool: &SqlitePool, device_id: Uuid, limit: i64) -> Result<Vec<TelemetryData>>;
    async fn find_latest(&self, pool: &SqlitePool, device_id: Uuid, metric_name: &str) -> Result<Option<TelemetryData>>;
}

pub struct SqliteTelemetryRepository;

#[async_trait]
impl TelemetryRepository for SqliteTelemetryRepository {
    async fn create(&self, pool: &SqlitePool, data: TelemetryData) -> Result<TelemetryData> {
        sqlx::query(
            r#"INSERT INTO telemetry_data (id, device_id, timestamp, metric_name, metric_type,
               value_numeric, value_string, value_boolean, unit, quality, raw_value, received_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(data.id.to_string())
        .bind(data.device_id.to_string())
        .bind(data.timestamp.to_rfc3339())
        .bind(&data.metric_name)
        .bind(format!("{:?}", data.metric_type))
        .bind(data.value_numeric)
        .bind(&data.value_string)
        .bind(data.value_boolean.map(|b| b as i32))
        .bind(&data.unit)
        .bind(format!("{:?}", data.quality))
        .bind(&data.raw_value)
        .bind(data.received_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(data)
    }
    
    async fn find_by_device(&self, pool: &SqlitePool, device_id: Uuid, limit: i64) -> Result<Vec<TelemetryData>> {
        let rows = sqlx::query_as::<_, TelemetryRow>(
            "SELECT * FROM telemetry_data WHERE device_id = ? ORDER BY timestamp DESC LIMIT ?"
        )
        .bind(device_id.to_string())
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    async fn find_latest(&self, pool: &SqlitePool, device_id: Uuid, metric_name: &str) -> Result<Option<TelemetryData>> {
        let row = sqlx::query_as::<_, TelemetryRow>(
            "SELECT * FROM telemetry_data WHERE device_id = ? AND metric_name = ? ORDER BY timestamp DESC LIMIT 1"
        )
        .bind(device_id.to_string())
        .bind(metric_name)
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(row.map(|r| r.into()))
    }
}

#[derive(sqlx::FromRow)]
struct TelemetryRow {
    id: String,
    device_id: String,
    timestamp: String,
    metric_name: String,
    metric_type: String,
    value_numeric: Option<f64>,
    value_string: Option<String>,
    value_boolean: Option<i32>,
    unit: Option<String>,
    quality: String,
    raw_value: Option<String>,
    received_at: String,
}

impl From<TelemetryRow> for TelemetryData {
    fn from(r: TelemetryRow) -> Self {
        use chrono::{DateTime, Utc};
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            device_id: Uuid::parse_str(&r.device_id).unwrap_or_default(),
            timestamp: DateTime::parse_from_rfc3339(&r.timestamp)
                .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            metric_name: r.metric_name,
            metric_type: match r.metric_type.as_str() {
                "Gauge" => MetricType::Gauge,
                "Histogram" => MetricType::Histogram,
                "Summary" => MetricType::Summary,
                "String" => MetricType::String,
                "Boolean" => MetricType::Boolean,
                "Location" => MetricType::Location,
                _ => MetricType::Counter,
            },
            value_numeric: r.value_numeric,
            value_string: r.value_string,
            value_boolean: r.value_boolean.map(|b| b != 0),
            unit: r.unit,
            quality: match r.quality.as_str() {
                "Uncertain" => DataQuality::Uncertain,
                "Bad" => DataQuality::Bad,
                "Unknown" => DataQuality::Unknown,
                _ => DataQuality::Good,
            },
            raw_value: r.raw_value,
            transformation_applied: None,
            metadata: None,
            received_at: DateTime::parse_from_rfc3339(&r.received_at)
                .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            processed_at: None,
        }
    }
}

#[async_trait]
pub trait IoTAlertRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, alert: IoTAlert) -> Result<IoTAlert>;
    async fn find_unresolved(&self, pool: &SqlitePool, limit: i64) -> Result<Vec<IoTAlert>>;
    async fn acknowledge(&self, pool: &SqlitePool, id: Uuid, user_id: Uuid) -> Result<()>;
    async fn resolve(&self, pool: &SqlitePool, id: Uuid, notes: Option<String>) -> Result<()>;
}

pub struct SqliteIoTAlertRepository;

#[async_trait]
impl IoTAlertRepository for SqliteIoTAlertRepository {
    async fn create(&self, pool: &SqlitePool, alert: IoTAlert) -> Result<IoTAlert> {
        sqlx::query(
            r#"INSERT INTO iot_alerts (id, device_id, alert_rule_id, alert_type, severity,
               title, message, metric_name, threshold_value, actual_value, trigger_condition,
               context_data, status, triggered_at, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(alert.base.id.to_string())
        .bind(alert.device_id.to_string())
        .bind(alert.alert_rule_id.map(|id| id.to_string()))
        .bind(format!("{:?}", alert.alert_type))
        .bind(format!("{:?}", alert.severity))
        .bind(&alert.title)
        .bind(&alert.message)
        .bind(&alert.metric_name)
        .bind(alert.threshold_value)
        .bind(alert.actual_value)
        .bind(&alert.trigger_condition)
        .bind(&alert.context_data)
        .bind(format!("{:?}", alert.status))
        .bind(alert.triggered_at.to_rfc3339())
        .bind(alert.base.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(alert)
    }
    
    async fn find_unresolved(&self, pool: &SqlitePool, limit: i64) -> Result<Vec<IoTAlert>> {
        let rows = sqlx::query_as::<_, IoTAlertRow>(
            "SELECT * FROM iot_alerts WHERE status IN ('New', 'Acknowledged', 'Investigating') ORDER BY triggered_at DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    async fn acknowledge(&self, pool: &SqlitePool, id: Uuid, user_id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE iot_alerts SET status = 'Acknowledged', acknowledged_at = ?, acknowledged_by = ? WHERE id = ?"
        )
        .bind(Utc::now().to_rfc3339())
        .bind(user_id.to_string())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        Ok(())
    }
    
    async fn resolve(&self, pool: &SqlitePool, id: Uuid, notes: Option<String>) -> Result<()> {
        sqlx::query(
            "UPDATE iot_alerts SET status = 'Resolved', resolved_at = ?, resolution_notes = ? WHERE id = ?"
        )
        .bind(Utc::now().to_rfc3339())
        .bind(&notes)
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct IoTAlertRow {
    id: String,
    device_id: String,
    alert_rule_id: Option<String>,
    alert_type: String,
    severity: String,
    title: String,
    message: String,
    metric_name: Option<String>,
    threshold_value: Option<f64>,
    actual_value: Option<f64>,
    trigger_condition: Option<String>,
    context_data: Option<String>,
    status: String,
    triggered_at: String,
    acknowledged_at: Option<String>,
    acknowledged_by: Option<String>,
    resolved_at: Option<String>,
    resolution_notes: Option<String>,
    created_at: String,
}

impl From<IoTAlertRow> for IoTAlert {
    fn from(r: IoTAlertRow) -> Self {
        use chrono::{DateTime, Utc};
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: Utc::now(),
                created_by: None,
                updated_by: None,
            },
            device_id: Uuid::parse_str(&r.device_id).unwrap_or_default(),
            alert_rule_id: r.alert_rule_id.and_then(|id| Uuid::parse_str(&id).ok()),
            alert_type: match r.alert_type.as_str() {
                "DeviceOffline" => IoTAlertType::DeviceOffline,
                "DeviceError" => IoTAlertType::DeviceError,
                "ThresholdExceeded" => IoTAlertType::ThresholdExceeded,
                "AnomalyDetected" => IoTAlertType::AnomalyDetected,
                "BatteryLow" => IoTAlertType::BatteryLow,
                "SignalWeak" => IoTAlertType::SignalWeak,
                "GeofenceBreach" => IoTAlertType::GeofenceBreach,
                "DataQuality" => IoTAlertType::DataQuality,
                "FirmwareUpdate" => IoTAlertType::FirmwareUpdate,
                "MaintenanceDue" => IoTAlertType::MaintenanceDue,
                "CommunicationError" => IoTAlertType::CommunicationError,
                "ConfigurationChange" => IoTAlertType::ConfigurationChange,
                _ => IoTAlertType::DeviceError,
            },
            severity: match r.severity.as_str() {
                "Warning" => IoTAlertSeverity::Warning,
                "Error" => IoTAlertSeverity::Error,
                "Critical" => IoTAlertSeverity::Critical,
                _ => IoTAlertSeverity::Info,
            },
            title: r.title,
            message: r.message,
            metric_name: r.metric_name,
            threshold_value: r.threshold_value,
            actual_value: r.actual_value,
            trigger_condition: r.trigger_condition,
            context_data: r.context_data,
            status: match r.status.as_str() {
                "Acknowledged" => IoTAlertStatus::Acknowledged,
                "Investigating" => IoTAlertStatus::Investigating,
                "Resolved" => IoTAlertStatus::Resolved,
                "Suppressed" => IoTAlertStatus::Suppressed,
                _ => IoTAlertStatus::New,
            },
            triggered_at: DateTime::parse_from_rfc3339(&r.triggered_at)
                .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            acknowledged_at: r.acknowledged_at.and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            acknowledged_by: r.acknowledged_by.and_then(|id| Uuid::parse_str(&id).ok()),
            resolved_at: r.resolved_at.and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            resolution_notes: r.resolution_notes,
            auto_resolved: false,
            notification_sent: false,
            escalation_level: 0,
            escalated_at: None,
        }
    }
}
