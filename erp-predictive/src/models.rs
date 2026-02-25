use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PredictionType {
    Failure,
    Maintenance,
    Performance,
    Anomaly,
    RemainingUsefulLife,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PredictionStatus {
    Active,
    Triggered,
    Expired,
    False,
    Confirmed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MaintenanceStrategy {
    Reactive,
    Preventive,
    Predictive,
    ConditionBased,
    ReliabilityCentered,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetSensor {
    pub base: BaseEntity,
    pub sensor_number: String,
    pub asset_id: Uuid,
    pub name: String,
    pub sensor_type: SensorType,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub location: Option<String>,
    pub measurement_unit: String,
    pub sampling_interval_seconds: i32,
    pub data_source: String,
    pub connection_type: ConnectionType,
    pub last_reading: Option<f64>,
    pub last_reading_at: Option<DateTime<Utc>>,
    pub min_threshold: Option<f64>,
    pub max_threshold: Option<f64>,
    pub alert_threshold_low: Option<f64>,
    pub alert_threshold_high: Option<f64>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SensorType {
    Temperature,
    Vibration,
    Pressure,
    Flow,
    Level,
    Humidity,
    Current,
    Voltage,
    Power,
    Speed,
    Acoustic,
    OilAnalysis,
    Thermographic,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ConnectionType {
    Modbus,
    OPCUA,
    MQTT,
    HTTP,
    LoRaWAN,
    Bluetooth,
    WiFi,
    Wired,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SensorReading {
    pub id: Uuid,
    pub sensor_id: Uuid,
    pub reading_timestamp: DateTime<Utc>,
    pub value: f64,
    pub unit: String,
    pub quality: ReadingQuality,
    pub raw_value: Option<f64>,
    pub is_anomaly: bool,
    pub anomaly_score: Option<f64>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReadingQuality {
    Good,
    Fair,
    Poor,
    Missing,
    Suspect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictiveModel {
    pub base: BaseEntity,
    pub model_number: String,
    pub name: String,
    pub description: Option<String>,
    pub model_type: ModelType,
    pub algorithm: String,
    pub version: String,
    pub asset_type_id: Option<Uuid>,
    pub target_variable: String,
    pub features: String,
    pub training_data_start: Option<NaiveDate>,
    pub training_data_end: Option<NaiveDate>,
    pub training_samples: i64,
    pub accuracy: f64,
    pub precision: f64,
    pub recall: f64,
    pub f1_score: f64,
    pub auc_roc: Option<f64>,
    pub confusion_matrix: Option<String>,
    pub feature_importance: Option<String>,
    pub model_path: Option<String>,
    pub hyperparameters: Option<String>,
    pub retraining_frequency_days: i32,
    pub last_trained_at: Option<DateTime<Utc>>,
    pub status: ModelStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ModelType {
    Classification,
    Regression,
    TimeSeries,
    AnomalyDetection,
    Clustering,
    RemainingUsefulLife,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ModelStatus {
    Development,
    Training,
    Validating,
    Deployed,
    Deprecated,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetHealthScore {
    pub base: BaseEntity,
    pub asset_id: Uuid,
    pub score_date: NaiveDate,
    pub overall_score: f64,
    pub previous_score: Option<f64>,
    pub score_change: Option<f64>,
    pub trend: HealthTrend,
    pub reliability_score: f64,
    pub performance_score: f64,
    pub maintenance_score: f64,
    pub component_scores: Option<String>,
    pub risk_level: RiskLevel,
    pub days_to_failure: Option<i32>,
    pub recommended_action: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum HealthTrend {
    Improving,
    Stable,
    Declining,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailurePrediction {
    pub base: BaseEntity,
    pub prediction_number: String,
    pub asset_id: Uuid,
    pub model_id: Option<Uuid>,
    pub prediction_type: PredictionType,
    pub prediction_date: DateTime<Utc>,
    pub predicted_failure_date: NaiveDate,
    pub confidence: f64,
    pub failure_mode: Option<String>,
    pub failure_probability: f64,
    pub remaining_useful_life_days: Option<i32>,
    pub health_score_at_prediction: f64,
    pub contributing_factors: Option<String>,
    pub recommended_actions: Option<String>,
    pub priority: i32,
    pub estimated_repair_cost: Option<i64>,
    pub currency: String,
    pub status: PredictionStatus,
    pub actual_failure_date: Option<NaiveDate>,
    pub work_order_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceSchedule {
    pub base: BaseEntity,
    pub schedule_number: String,
    pub asset_id: Uuid,
    pub maintenance_type: MaintenanceStrategy,
    pub scheduled_date: NaiveDate,
    pub estimated_duration_hours: f64,
    pub estimated_cost: i64,
    pub currency: String,
    pub priority: i32,
    pub prediction_id: Option<Uuid>,
    pub description: Option<String>,
    pub tasks: Option<String>,
    pub parts_required: Option<String>,
    pub assigned_to: Option<Uuid>,
    pub status: ScheduleStatus,
    pub completed_date: Option<NaiveDate>,
    pub actual_cost: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ScheduleStatus {
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
    Deferred,
    Overdue,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetection {
    pub base: BaseEntity,
    pub detection_number: String,
    pub asset_id: Uuid,
    pub sensor_id: Option<Uuid>,
    pub detection_date: DateTime<Utc>,
    pub anomaly_type: AnomalyType,
    pub severity: AnomalySeverity,
    pub description: String,
    pub measured_value: f64,
    pub expected_value: Option<f64>,
    pub deviation_percent: Option<f64>,
    pub detection_method: String,
    pub model_id: Option<Uuid>,
    pub acknowledged: bool,
    pub acknowledged_by: Option<Uuid>,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub resolution: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AnomalyType {
    Point,
    Contextual,
    Collective,
    Trend,
    Seasonal,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AnomalySeverity {
    Info,
    Warning,
    Critical,
    Emergency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaintenanceCostAnalysis {
    pub base: BaseEntity,
    pub asset_id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_maintenance_cost: i64,
    pub preventive_cost: i64,
    pub predictive_cost: i64,
    pub corrective_cost: i64,
    pub downtime_cost: i64,
    pub total_downtime_hours: f64,
    pub unplanned_downtime_hours: f64,
    pub mtbf_hours: Option<f64>,
    pub mttr_hours: Option<f64>,
    pub availability_percent: f64,
    pub oee_percent: Option<f64>,
    pub savings_from_predictions: i64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}
