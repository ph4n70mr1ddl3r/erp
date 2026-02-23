use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum DeviceType {
    Sensor,
    Actuator,
    Gateway,
    Controller,
    SmartMeter,
    Camera,
    RFIDReader,
    BarcodeScanner,
    Scale,
    Thermostat,
    HumiditySensor,
    MotionDetector,
    GPSTracker,
    Beacon,
    PLC,
    EdgeDevice,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum DeviceStatus {
    Online,
    Offline,
    Maintenance,
    Error,
    Disabled,
    Provisioning,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ConnectivityType {
    WiFi,
    Ethernet,
    Cellular,
    LoRaWAN,
    Bluetooth,
    Zigbee,
    MQTT,
    CoAP,
    HTTP,
    Modbus,
    OPCUA,
    CANBus,
    RS232,
    RS485,
    USB,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTDevice {
    pub base: BaseEntity,
    pub device_id: String,
    pub name: String,
    pub description: Option<String>,
    pub device_type: DeviceType,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub firmware_version: Option<String>,
    pub hardware_version: Option<String>,
    pub status: DeviceStatus,
    pub connectivity_type: ConnectivityType,
    pub ip_address: Option<String>,
    pub mac_address: Option<String>,
    pub port: Option<i32>,
    pub protocol_config: Option<String>,
    pub gateway_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub warehouse_id: Option<Uuid>,
    pub zone: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub geofence_enabled: bool,
    pub geofence_radius_meters: Option<i32>,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub last_heartbeat_at: Option<DateTime<Utc>>,
    pub heartbeat_interval_seconds: i32,
    pub battery_level: Option<i32>,
    pub signal_strength: Option<i32>,
    pub temperature_celsius: Option<f64>,
    pub humidity_percent: Option<f64>,
    pub data_format: Option<String>,
    pub data_schema: Option<String>,
    pub transforms: Option<String>,
    pub alert_rules: Option<String>,
    pub metadata: Option<String>,
    pub tags: Option<String>,
    pub owner_id: Option<Uuid>,
    pub installed_at: Option<DateTime<Utc>>,
    pub maintenance_due_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTDeviceGroup {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub group_type: String,
    pub parent_id: Option<Uuid>,
    pub device_count: i32,
    pub alert_rules: Option<String>,
    pub metadata: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceGroupMembership {
    pub id: Uuid,
    pub group_id: Uuid,
    pub device_id: Uuid,
    pub added_at: DateTime<Utc>,
    pub added_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryData {
    pub id: Uuid,
    pub device_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub metric_name: String,
    pub metric_type: MetricType,
    pub value_numeric: Option<f64>,
    pub value_string: Option<String>,
    pub value_boolean: Option<bool>,
    pub unit: Option<String>,
    pub quality: DataQuality,
    pub raw_value: Option<String>,
    pub transformation_applied: Option<String>,
    pub metadata: Option<String>,
    pub received_at: DateTime<Utc>,
    pub processed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum MetricType {
    Counter,
    Gauge,
    Histogram,
    Summary,
    String,
    Boolean,
    Location,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum DataQuality {
    Good,
    Uncertain,
    Bad,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryAggregate {
    pub id: Uuid,
    pub device_id: Uuid,
    pub metric_name: String,
    pub aggregation_type: AggregationType,
    pub period: AggregatePeriod,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub value_min: Option<f64>,
    pub value_max: Option<f64>,
    pub value_avg: Option<f64>,
    pub value_sum: Option<f64>,
    pub value_count: i64,
    pub value_stddev: Option<f64>,
    pub quality_score: f64,
    pub computed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AggregationType {
    Min,
    Max,
    Avg,
    Sum,
    Count,
    StdDev,
    Percentile,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AggregatePeriod {
    Minute,
    Hour,
    Day,
    Week,
    Month,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTAlert {
    pub base: BaseEntity,
    pub device_id: Uuid,
    pub alert_rule_id: Option<Uuid>,
    pub alert_type: IoTAlertType,
    pub severity: IoTAlertSeverity,
    pub title: String,
    pub message: String,
    pub metric_name: Option<String>,
    pub threshold_value: Option<f64>,
    pub actual_value: Option<f64>,
    pub trigger_condition: Option<String>,
    pub context_data: Option<String>,
    pub status: IoTAlertStatus,
    pub triggered_at: DateTime<Utc>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub acknowledged_by: Option<Uuid>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution_notes: Option<String>,
    pub auto_resolved: bool,
    pub notification_sent: bool,
    pub escalation_level: i32,
    pub escalated_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum IoTAlertType {
    DeviceOffline,
    DeviceError,
    ThresholdExceeded,
    AnomalyDetected,
    BatteryLow,
    SignalWeak,
    GeofenceBreach,
    DataQuality,
    FirmwareUpdate,
    MaintenanceDue,
    CommunicationError,
    ConfigurationChange,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum IoTAlertSeverity {
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum IoTAlertStatus {
    New,
    Acknowledged,
    Investigating,
    Resolved,
    Suppressed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTAlertRule {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub device_type: Option<DeviceType>,
    pub device_group_id: Option<Uuid>,
    pub metric_name: String,
    pub condition_type: AlertConditionType,
    pub operator: ComparisonOperator,
    pub threshold_value: f64,
    pub threshold_value_secondary: Option<f64>,
    pub duration_seconds: i32,
    pub aggregation_window_seconds: i32,
    pub aggregation_type: Option<AggregationType>,
    pub severity: IoTAlertSeverity,
    pub cooldown_seconds: i32,
    pub auto_resolve: bool,
    pub auto_resolve_after_seconds: Option<i32>,
    pub notification_channels: Option<String>,
    pub escalation_config: Option<String>,
    pub suppress_duplicates: bool,
    pub suppression_window_seconds: i32,
    pub status: Status,
    pub last_triggered_at: Option<DateTime<Utc>>,
    pub trigger_count: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AlertConditionType {
    Threshold,
    RateOfChange,
    MissingData,
    Anomaly,
    Compound,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ComparisonOperator {
    Equal,
    NotEqual,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Between,
    Outside,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceCommand {
    pub base: BaseEntity,
    pub device_id: Uuid,
    pub command_type: String,
    pub command_payload: Option<String>,
    pub priority: i32,
    pub status: CommandStatus,
    pub retry_count: i32,
    pub max_retries: i32,
    pub timeout_seconds: i32,
    pub response_payload: Option<String>,
    pub error_message: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum CommandStatus {
    Pending,
    Sent,
    Acknowledged,
    Completed,
    Failed,
    Timeout,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirmwareVersion {
    pub id: Uuid,
    pub device_type: DeviceType,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub version: String,
    pub file_path: String,
    pub file_size_bytes: i64,
    pub checksum: String,
    pub checksum_type: String,
    pub release_notes: Option<String>,
    pub is_critical: bool,
    pub rollout_strategy: RolloutStrategy,
    pub rollout_percentage: i32,
    pub target_device_group_id: Option<Uuid>,
    pub status: FirmwareStatus,
    pub scheduled_at: Option<DateTime<Utc>>,
    pub released_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum RolloutStrategy {
    Immediate,
    Scheduled,
    Staged,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum FirmwareStatus {
    Draft,
    Testing,
    Released,
    Deprecated,
    Retired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirmwareUpdateJob {
    pub base: BaseEntity,
    pub firmware_id: Uuid,
    pub device_id: Uuid,
    pub status: FirmwareUpdateStatus,
    pub previous_version: Option<String>,
    pub target_version: String,
    pub progress_percent: i32,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
    pub retry_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum FirmwareUpdateStatus {
    Pending,
    Downloading,
    Installing,
    Verifying,
    Completed,
    Failed,
    Rollback,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IoTDataExport {
    pub base: BaseEntity,
    pub name: String,
    pub export_type: IoTExportType,
    pub device_ids: Option<String>,
    pub device_group_ids: Option<String>,
    pub metric_names: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub format: ExportFormat,
    pub compression: ExportCompression,
    pub file_path: Option<String>,
    pub file_size_bytes: Option<i64>,
    pub row_count: Option<i64>,
    pub status: ExportStatus,
    pub error_message: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub expires_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum IoTExportType {
    Telemetry,
    Alerts,
    Devices,
    Aggregates,
    Events,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ExportFormat {
    CSV,
    JSON,
    Parquet,
    Excel,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ExportCompression {
    None,
    Gzip,
    Zip,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ExportStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DigitalTwin {
    pub base: BaseEntity,
    pub name: String,
    pub twin_type: String,
    pub physical_device_id: Option<Uuid>,
    pub model_reference: Option<String>,
    pub properties: Option<String>,
    pub relationships: Option<String>,
    pub simulation_config: Option<String>,
    pub last_synced_at: Option<DateTime<Utc>>,
    pub sync_status: SyncStatus,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum SyncStatus {
    Synced,
    OutOfSync,
    Syncing,
    Error,
}
