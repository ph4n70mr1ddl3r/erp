use crate::models::*;
use crate::repository::{PredictiveRepository, SqlitePredictiveRepository};
use chrono::{NaiveDate, Utc};
use erp_core::{BaseEntity, Result};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct PredictiveService { repo: SqlitePredictiveRepository }
impl PredictiveService {
    pub fn new(pool: SqlitePool) -> Self { Self { repo: SqlitePredictiveRepository::new(pool) } }

    pub async fn register_sensor(&self, pool: &SqlitePool, req: RegisterSensorRequest) -> Result<AssetSensor> {
        let sensor = AssetSensor {
            base: BaseEntity::new(),
            sensor_number: format!("SNS-{}", Uuid::new_v4()),
            asset_id: req.asset_id,
            name: req.name,
            sensor_type: req.sensor_type,
            manufacturer: req.manufacturer,
            model: req.model,
            serial_number: req.serial_number,
            location: req.location,
            measurement_unit: req.measurement_unit,
            sampling_interval_seconds: req.sampling_interval_seconds.unwrap_or(60),
            data_source: req.data_source,
            connection_type: req.connection_type,
            last_reading: None,
            last_reading_at: None,
            min_threshold: req.min_threshold,
            max_threshold: req.max_threshold,
            alert_threshold_low: req.alert_threshold_low,
            alert_threshold_high: req.alert_threshold_high,
            status: erp_core::Status::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_sensor(&sensor).await
    }

    pub async fn record_reading(&self, pool: &SqlitePool, sensor_id: Uuid, value: f64, unit: String) -> Result<SensorReading> {
        let reading = SensorReading {
            id: Uuid::new_v4(),
            sensor_id,
            reading_timestamp: Utc::now(),
            value,
            unit,
            quality: ReadingQuality::Good,
            raw_value: Some(value),
            is_anomaly: false,
            anomaly_score: None,
            created_at: Utc::now(),
        };
        self.repo.create_reading(&reading).await
    }

    pub async fn create_model(&self, pool: &SqlitePool, req: CreateModelRequest) -> Result<PredictiveModel> {
        let model = PredictiveModel {
            base: BaseEntity::new(),
            model_number: format!("MDL-{}", Uuid::new_v4()),
            name: req.name,
            description: req.description,
            model_type: req.model_type,
            algorithm: req.algorithm,
            version: "1.0.0".to_string(),
            asset_type_id: req.asset_type_id,
            target_variable: req.target_variable,
            features: req.features,
            training_data_start: None,
            training_data_end: None,
            training_samples: 0,
            accuracy: 0.0,
            precision: 0.0,
            recall: 0.0,
            f1_score: 0.0,
            auc_roc: None,
            confusion_matrix: None,
            feature_importance: None,
            model_path: None,
            hyperparameters: None,
            retraining_frequency_days: req.retraining_frequency_days.unwrap_or(90),
            last_trained_at: None,
            status: ModelStatus::Development,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_model(&model).await
    }

    pub async fn calculate_health_score(&self, pool: &SqlitePool, asset_id: Uuid) -> Result<AssetHealthScore> {
        let prev = self.repo.get_latest_health_score(asset_id).await?;
        let prev_score = prev.as_ref().map(|p| p.overall_score);
        let score = AssetHealthScore {
            base: BaseEntity::new(),
            asset_id,
            score_date: Utc::now().date_naive(),
            overall_score: 85.0,
            previous_score: prev_score,
            score_change: prev_score.map(|p| 85.0 - p),
            trend: HealthTrend::Stable,
            reliability_score: 90.0,
            performance_score: 85.0,
            maintenance_score: 80.0,
            component_scores: None,
            risk_level: RiskLevel::Low,
            days_to_failure: Some(180),
            recommended_action: Some("Continue regular maintenance schedule".to_string()),
            created_at: Utc::now(),
        };
        self.repo.create_health_score(&score).await
    }

    pub async fn predict_failure(&self, pool: &SqlitePool, req: PredictFailureRequest) -> Result<FailurePrediction> {
        let prediction = FailurePrediction {
            base: BaseEntity::new(),
            prediction_number: format!("PRD-{}", Uuid::new_v4()),
            asset_id: req.asset_id,
            model_id: req.model_id,
            prediction_type: PredictionType::Failure,
            prediction_date: Utc::now(),
            predicted_failure_date: req.predicted_failure_date,
            confidence: req.confidence,
            failure_mode: req.failure_mode,
            failure_probability: req.failure_probability,
            remaining_useful_life_days: req.remaining_useful_life_days,
            health_score_at_prediction: 75.0,
            contributing_factors: req.contributing_factors,
            recommended_actions: req.recommended_actions,
            priority: req.priority.unwrap_or(5),
            estimated_repair_cost: req.estimated_repair_cost,
            currency: req.currency.unwrap_or_else(|| "USD".to_string()),
            status: PredictionStatus::Active,
            actual_failure_date: None,
            work_order_id: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_prediction(&prediction).await
    }

    pub async fn schedule_maintenance(&self, pool: &SqlitePool, req: ScheduleMaintenanceRequest) -> Result<MaintenanceSchedule> {
        let schedule = MaintenanceSchedule {
            base: BaseEntity::new(),
            schedule_number: format!("MS-{}", Uuid::new_v4()),
            asset_id: req.asset_id,
            maintenance_type: req.maintenance_type,
            scheduled_date: req.scheduled_date,
            estimated_duration_hours: req.estimated_duration_hours.unwrap_or(4.0),
            estimated_cost: req.estimated_cost.unwrap_or(0),
            currency: req.currency.unwrap_or_else(|| "USD".to_string()),
            priority: req.priority.unwrap_or(5),
            prediction_id: req.prediction_id,
            description: req.description,
            tasks: req.tasks,
            parts_required: req.parts_required,
            assigned_to: req.assigned_to,
            status: ScheduleStatus::Scheduled,
            completed_date: None,
            actual_cost: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_schedule(&schedule).await
    }

    pub async fn detect_anomaly(&self, pool: &SqlitePool, req: DetectAnomalyRequest) -> Result<AnomalyDetection> {
        let anomaly = AnomalyDetection {
            base: BaseEntity::new(),
            detection_number: format!("ANM-{}", Uuid::new_v4()),
            asset_id: req.asset_id,
            sensor_id: req.sensor_id,
            detection_date: Utc::now(),
            anomaly_type: req.anomaly_type,
            severity: req.severity,
            description: req.description,
            measured_value: req.measured_value,
            expected_value: req.expected_value,
            deviation_percent: req.deviation_percent,
            detection_method: req.detection_method.unwrap_or_else(|| "statistical".to_string()),
            model_id: None,
            acknowledged: false,
            acknowledged_by: None,
            acknowledged_at: None,
            resolution: None,
            resolved_at: None,
            created_at: Utc::now(),
        };
        self.repo.create_anomaly(&anomaly).await
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct RegisterSensorRequest {
    pub asset_id: Uuid,
    pub name: String,
    pub sensor_type: SensorType,
    pub manufacturer: Option<String>,
    pub model: Option<String>,
    pub serial_number: Option<String>,
    pub location: Option<String>,
    pub measurement_unit: String,
    pub sampling_interval_seconds: Option<i32>,
    pub data_source: String,
    pub connection_type: ConnectionType,
    pub min_threshold: Option<f64>,
    pub max_threshold: Option<f64>,
    pub alert_threshold_low: Option<f64>,
    pub alert_threshold_high: Option<f64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateModelRequest {
    pub name: String,
    pub description: Option<String>,
    pub model_type: ModelType,
    pub algorithm: String,
    pub asset_type_id: Option<Uuid>,
    pub target_variable: String,
    pub features: String,
    pub retraining_frequency_days: Option<i32>,
}

#[derive(Debug, serde::Deserialize)]
pub struct PredictFailureRequest {
    pub asset_id: Uuid,
    pub model_id: Option<Uuid>,
    pub predicted_failure_date: NaiveDate,
    pub confidence: f64,
    pub failure_mode: Option<String>,
    pub failure_probability: f64,
    pub remaining_useful_life_days: Option<i32>,
    pub contributing_factors: Option<String>,
    pub recommended_actions: Option<String>,
    pub priority: Option<i32>,
    pub estimated_repair_cost: Option<i64>,
    pub currency: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct ScheduleMaintenanceRequest {
    pub asset_id: Uuid,
    pub maintenance_type: MaintenanceStrategy,
    pub scheduled_date: NaiveDate,
    pub estimated_duration_hours: Option<f64>,
    pub estimated_cost: Option<i64>,
    pub currency: Option<String>,
    pub priority: Option<i32>,
    pub prediction_id: Option<Uuid>,
    pub description: Option<String>,
    pub tasks: Option<String>,
    pub parts_required: Option<String>,
    pub assigned_to: Option<Uuid>,
}

#[derive(Debug, serde::Deserialize)]
pub struct DetectAnomalyRequest {
    pub asset_id: Uuid,
    pub sensor_id: Option<Uuid>,
    pub anomaly_type: AnomalyType,
    pub severity: AnomalySeverity,
    pub description: String,
    pub measured_value: f64,
    pub expected_value: Option<f64>,
    pub deviation_percent: Option<f64>,
    pub detection_method: Option<String>,
}
