use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, BaseEntity, Paginated};
use crate::models::*;
use crate::repository::*;

pub struct IoTDeviceService { repo: SqliteIoTDeviceRepository }
impl Default for IoTDeviceService {
    fn default() -> Self {
        Self::new()
    }
}

impl IoTDeviceService {
    pub fn new() -> Self { Self { repo: SqliteIoTDeviceRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<IoTDevice> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn get_by_device_id(&self, pool: &SqlitePool, device_id: &str) -> Result<IoTDevice> {
        self.repo.find_by_device_id(pool, device_id).await
    }
    
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<IoTDevice>> {
        self.repo.find_all(pool, pagination).await
    }
    
    pub async fn register(&self, pool: &SqlitePool, mut device: IoTDevice) -> Result<IoTDevice> {
        if device.device_id.is_empty() {
            return Err(Error::validation("Device ID is required"));
        }
        if device.name.is_empty() {
            return Err(Error::validation("Device name is required"));
        }
        
        if self.repo.find_by_device_id(pool, &device.device_id).await.is_ok() {
            return Err(Error::validation("Device ID already registered"));
        }
        
        device.base = BaseEntity::new();
        device.status = DeviceStatus::Provisioning;
        device.created_at = Utc::now();
        device.updated_at = Utc::now();
        device.heartbeat_interval_seconds = 60;
        
        self.repo.create(pool, device).await
    }
    
    pub async fn update(&self, pool: &SqlitePool, mut device: IoTDevice) -> Result<IoTDevice> {
        device.updated_at = Utc::now();
        self.repo.update(pool, &device).await?;
        Ok(device)
    }
    
    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete(pool, id).await
    }
    
    pub async fn heartbeat(&self, pool: &SqlitePool, device_id: &str, battery_level: Option<i32>, signal_strength: Option<i32>, temperature: Option<f64>, humidity: Option<f64>) -> Result<()> {
        let mut device = self.repo.find_by_device_id(pool, device_id).await?;
        device.last_heartbeat_at = Some(Utc::now());
        device.battery_level = battery_level;
        device.signal_strength = signal_strength;
        device.temperature_celsius = temperature;
        device.humidity_percent = humidity;
        device.status = DeviceStatus::Online;
        device.updated_at = Utc::now();
        self.repo.update(pool, &device).await?;
        self.repo.update_heartbeat(pool, device.base.id).await
    }
    
    pub async fn check_offline_devices(&self, pool: &SqlitePool) -> Result<Vec<IoTDevice>> {
        let all = self.repo.find_all(pool, Pagination::new(1, 1000)).await?;
        let now = Utc::now();
        let offline: Vec<IoTDevice> = all.items.into_iter().filter(|d| {
            if let Some(last) = d.last_heartbeat_at {
                let elapsed = (now - last).num_seconds();
                elapsed > (d.heartbeat_interval_seconds * 3) as i64
            } else {
                d.status != DeviceStatus::Provisioning
            }
        }).collect();
        
        Ok(offline)
    }
}

pub struct TelemetryService { repo: SqliteTelemetryRepository }
impl Default for TelemetryService {
    fn default() -> Self {
        Self::new()
    }
}

impl TelemetryService {
    pub fn new() -> Self { Self { repo: SqliteTelemetryRepository } }
    
    pub async fn ingest(&self, pool: &SqlitePool, device_id: Uuid, metric_name: String, metric_type: MetricType, value_numeric: Option<f64>, value_string: Option<String>, value_boolean: Option<bool>, unit: Option<String>) -> Result<TelemetryData> {
        let data = TelemetryData {
            id: Uuid::new_v4(),
            device_id,
            timestamp: Utc::now(),
            metric_name,
            metric_type,
            value_numeric,
            value_string,
            value_boolean,
            unit,
            quality: DataQuality::Good,
            raw_value: None,
            transformation_applied: None,
            metadata: None,
            received_at: Utc::now(),
            processed_at: None,
        };
        
        self.repo.create(pool, data).await
    }
    
    pub async fn ingest_batch(&self, pool: &SqlitePool, device_id: Uuid, readings: Vec<(String, f64, Option<String>)>) -> Result<Vec<TelemetryData>> {
        let mut results = Vec::new();
        for (metric_name, value, unit) in readings {
            let data = self.ingest(pool, device_id, metric_name, MetricType::Gauge, Some(value), None, None, unit).await?;
            results.push(data);
        }
        Ok(results)
    }
    
    pub async fn get_device_history(&self, pool: &SqlitePool, device_id: Uuid, limit: i64) -> Result<Vec<TelemetryData>> {
        self.repo.find_by_device(pool, device_id, limit).await
    }
    
    pub async fn get_latest(&self, pool: &SqlitePool, device_id: Uuid, metric_name: &str) -> Result<Option<TelemetryData>> {
        self.repo.find_latest(pool, device_id, metric_name).await
    }
}

pub struct IoTAlertService { repo: SqliteIoTAlertRepository }
impl Default for IoTAlertService {
    fn default() -> Self {
        Self::new()
    }
}

impl IoTAlertService {
    pub fn new() -> Self { Self { repo: SqliteIoTAlertRepository } }
    
    pub async fn create(&self, pool: &SqlitePool, alert: IoTAlert) -> Result<IoTAlert> {
        self.repo.create(pool, alert).await
    }
    
    pub async fn list_unresolved(&self, pool: &SqlitePool, limit: i64) -> Result<Vec<IoTAlert>> {
        self.repo.find_unresolved(pool, limit).await
    }
    
    pub async fn acknowledge(&self, pool: &SqlitePool, id: Uuid, user_id: Uuid) -> Result<()> {
        self.repo.acknowledge(pool, id, user_id).await
    }
    
    pub async fn resolve(&self, pool: &SqlitePool, id: Uuid, notes: Option<String>) -> Result<()> {
        self.repo.resolve(pool, id, notes).await
    }
    
    pub async fn check_threshold(&self, pool: &SqlitePool, device_id: Uuid, metric_name: &str, value: f64, threshold: f64, severity: IoTAlertSeverity) -> Result<Option<IoTAlert>> {
        if value > threshold {
            let alert = IoTAlert {
                base: BaseEntity::new(),
                device_id,
                alert_rule_id: None,
                alert_type: IoTAlertType::ThresholdExceeded,
                severity,
                title: format!("{} threshold exceeded", metric_name),
                message: format!("{} value {} exceeds threshold {}", metric_name, value, threshold),
                metric_name: Some(metric_name.to_string()),
                threshold_value: Some(threshold),
                actual_value: Some(value),
                trigger_condition: Some(format!("{} > {}", metric_name, threshold)),
                context_data: None,
                status: IoTAlertStatus::New,
                triggered_at: Utc::now(),
                acknowledged_at: None,
                acknowledged_by: None,
                resolved_at: None,
                resolution_notes: None,
                auto_resolved: false,
                notification_sent: false,
                escalation_level: 0,
                escalated_at: None,
            };
            return Ok(Some(self.repo.create(pool, alert).await?));
        }
        Ok(None)
    }
}

pub struct DeviceCommandService;
impl Default for DeviceCommandService {
    fn default() -> Self {
        Self::new()
    }
}

impl DeviceCommandService {
    pub fn new() -> Self { Self }
    
    pub async fn send_command(&self, pool: &SqlitePool, device_id: Uuid, command_type: String, payload: Option<String>) -> Result<DeviceCommand> {
        let _ = pool;
        Ok(DeviceCommand {
            base: BaseEntity::new(),
            device_id,
            command_type,
            command_payload: payload,
            priority: 5,
            status: CommandStatus::Pending,
            retry_count: 0,
            max_retries: 3,
            timeout_seconds: 30,
            response_payload: None,
            error_message: None,
            sent_at: None,
            acknowledged_at: None,
            completed_at: None,
            created_by: None,
            created_at: Utc::now(),
        })
    }
    
    pub async fn get_status(&self, pool: &SqlitePool, command_id: Uuid) -> Result<DeviceCommand> {
        let _ = pool;
        Err(Error::not_found("DeviceCommand", &command_id.to_string()))
    }
}

pub struct FirmwareService;
impl Default for FirmwareService {
    fn default() -> Self {
        Self::new()
    }
}

impl FirmwareService {
    pub fn new() -> Self { Self }
    
    pub async fn schedule_update(&self, pool: &SqlitePool, firmware_id: Uuid, device_ids: Vec<Uuid>) -> Result<Vec<FirmwareUpdateJob>> {
        let _ = pool;
        let mut jobs = Vec::new();
        for device_id in device_ids {
            jobs.push(FirmwareUpdateJob {
                base: BaseEntity::new(),
                firmware_id,
                device_id,
                status: FirmwareUpdateStatus::Pending,
                previous_version: None,
                target_version: "2.0.0".to_string(),
                progress_percent: 0,
                started_at: None,
                completed_at: None,
                error_message: None,
                retry_count: 0,
                created_at: Utc::now(),
            });
        }
        Ok(jobs)
    }
    
    pub async fn get_update_status(&self, pool: &SqlitePool, job_id: Uuid) -> Result<FirmwareUpdateJob> {
        let _ = pool;
        Err(Error::not_found("FirmwareUpdateJob", &job_id.to_string()))
    }
}

pub struct DigitalTwinService;
impl Default for DigitalTwinService {
    fn default() -> Self {
        Self::new()
    }
}

impl DigitalTwinService {
    pub fn new() -> Self { Self }
    
    pub async fn create(&self, pool: &SqlitePool, name: String, twin_type: String, physical_device_id: Option<Uuid>) -> Result<DigitalTwin> {
        let _ = pool;
        Ok(DigitalTwin {
            base: BaseEntity::new(),
            name,
            twin_type,
            physical_device_id,
            model_reference: None,
            properties: None,
            relationships: None,
            simulation_config: None,
            last_synced_at: None,
            sync_status: SyncStatus::Error,
            status: erp_core::Status::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        })
    }
    
    pub async fn sync(&self, pool: &SqlitePool, twin_id: Uuid) -> Result<DigitalTwin> {
        let _ = pool;
        Err(Error::not_found("DigitalTwin", &twin_id.to_string()))
    }
}

pub struct IoTDataExportService;
impl Default for IoTDataExportService {
    fn default() -> Self {
        Self::new()
    }
}

impl IoTDataExportService {
    pub fn new() -> Self { Self }
    
    pub async fn create_export(&self, pool: &SqlitePool, name: String, export_type: IoTExportType, device_ids: Vec<Uuid>, start_time: chrono::DateTime<Utc>, end_time: chrono::DateTime<Utc>) -> Result<IoTDataExport> {
        let _ = pool;
        Ok(IoTDataExport {
            base: BaseEntity::new(),
            name,
            export_type,
            device_ids: Some(serde_json::to_string(&device_ids).unwrap_or_default()),
            device_group_ids: None,
            metric_names: None,
            start_time,
            end_time,
            format: ExportFormat::CSV,
            compression: ExportCompression::Gzip,
            file_path: None,
            file_size_bytes: None,
            row_count: None,
            status: ExportStatus::Pending,
            error_message: None,
            started_at: None,
            completed_at: None,
            expires_at: None,
            created_by: None,
            created_at: Utc::now(),
        })
    }
    
    pub async fn get_export(&self, pool: &SqlitePool, id: Uuid) -> Result<IoTDataExport> {
        let _ = pool;
        Err(Error::not_found("IoTDataExport", &id.to_string()))
    }
}
