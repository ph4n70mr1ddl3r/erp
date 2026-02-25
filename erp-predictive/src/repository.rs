use crate::models::*;
use async_trait::async_trait;
use erp_core::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait PredictiveRepository: Send + Sync {
    async fn create_sensor(&self, sensor: &AssetSensor) -> Result<AssetSensor>;
    async fn get_sensor(&self, id: Uuid) -> Result<Option<AssetSensor>>;
    async fn list_sensors(&self, asset_id: Uuid) -> Result<Vec<AssetSensor>>;
    async fn create_reading(&self, reading: &SensorReading) -> Result<SensorReading>;
    async fn list_readings(&self, sensor_id: Uuid, limit: i32) -> Result<Vec<SensorReading>>;
    async fn create_model(&self, model: &PredictiveModel) -> Result<PredictiveModel>;
    async fn create_health_score(&self, score: &AssetHealthScore) -> Result<AssetHealthScore>;
    async fn get_latest_health_score(&self, asset_id: Uuid) -> Result<Option<AssetHealthScore>>;
    async fn create_prediction(&self, prediction: &FailurePrediction) -> Result<FailurePrediction>;
    async fn list_predictions(&self, asset_id: Uuid) -> Result<Vec<FailurePrediction>>;
    async fn create_schedule(&self, schedule: &MaintenanceSchedule) -> Result<MaintenanceSchedule>;
    async fn create_anomaly(&self, anomaly: &AnomalyDetection) -> Result<AnomalyDetection>;
    async fn create_cost_analysis(&self, analysis: &MaintenanceCostAnalysis) -> Result<MaintenanceCostAnalysis>;
}

pub struct SqlitePredictiveRepository { pool: SqlitePool }
impl SqlitePredictiveRepository { pub fn new(pool: SqlitePool) -> Self { Self { pool } } }

#[async_trait]
impl PredictiveRepository for SqlitePredictiveRepository {
    async fn create_sensor(&self, sensor: &AssetSensor) -> Result<AssetSensor> {
        let s = sensor.clone();
        sqlx::query!(r#"INSERT INTO asset_sensors (id, sensor_number, asset_id, name, sensor_type,
            manufacturer, model, serial_number, location, measurement_unit, sampling_interval_seconds,
            data_source, connection_type, last_reading, last_reading_at, min_threshold, max_threshold,
            alert_threshold_low, alert_threshold_high, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            s.base.id, s.sensor_number, s.asset_id, s.name, s.sensor_type, s.manufacturer, s.model,
            s.serial_number, s.location, s.measurement_unit, s.sampling_interval_seconds, s.data_source,
            s.connection_type, s.last_reading, s.last_reading_at, s.min_threshold, s.max_threshold,
            s.alert_threshold_low, s.alert_threshold_high, s.status, s.created_at, s.updated_at).execute(&self.pool).await?;
        Ok(s)
    }
    async fn get_sensor(&self, _id: Uuid) -> Result<Option<AssetSensor>> { Ok(None) }
    async fn list_sensors(&self, _asset_id: Uuid) -> Result<Vec<AssetSensor>> { Ok(vec![]) }
    async fn create_reading(&self, reading: &SensorReading) -> Result<SensorReading> {
        let r = reading.clone();
        sqlx::query!(r#"INSERT INTO sensor_readings (id, sensor_id, reading_timestamp, value, unit,
            quality, raw_value, is_anomaly, anomaly_score, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            r.id, r.sensor_id, r.reading_timestamp, r.value, r.unit, r.quality, r.raw_value,
            r.is_anomaly, r.anomaly_score, r.created_at).execute(&self.pool).await?;
        Ok(r)
    }
    async fn list_readings(&self, _sensor_id: Uuid, _limit: i32) -> Result<Vec<SensorReading>> { Ok(vec![]) }
    async fn create_model(&self, model: &PredictiveModel) -> Result<PredictiveModel> {
        let m = model.clone();
        sqlx::query!(r#"INSERT INTO predictive_models (id, model_number, name, description, model_type,
            algorithm, version, asset_type_id, target_variable, features, training_data_start,
            training_data_end, training_samples, accuracy, precision, recall, f1_score, auc_roc,
            confusion_matrix, feature_importance, model_path, hyperparameters, retraining_frequency_days,
            last_trained_at, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            m.base.id, m.model_number, m.name, m.description, m.model_type, m.algorithm, m.version,
            m.asset_type_id, m.target_variable, m.features, m.training_data_start, m.training_data_end,
            m.training_samples, m.accuracy, m.precision, m.recall, m.f1_score, m.auc_roc,
            m.confusion_matrix, m.feature_importance, m.model_path, m.hyperparameters, m.retraining_frequency_days,
            m.last_trained_at, m.status, m.created_at, m.updated_at).execute(&self.pool).await?;
        Ok(m)
    }
    async fn create_health_score(&self, score: &AssetHealthScore) -> Result<AssetHealthScore> {
        let s = score.clone();
        sqlx::query!(r#"INSERT INTO asset_health_scores (id, asset_id, score_date, overall_score,
            previous_score, score_change, trend, reliability_score, performance_score, maintenance_score,
            component_scores, risk_level, days_to_failure, recommended_action, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            s.base.id, s.asset_id, s.score_date, s.overall_score, s.previous_score, s.score_change,
            s.trend, s.reliability_score, s.performance_score, s.maintenance_score, s.component_scores,
            s.risk_level, s.days_to_failure, s.recommended_action, s.created_at).execute(&self.pool).await?;
        Ok(s)
    }
    async fn get_latest_health_score(&self, _asset_id: Uuid) -> Result<Option<AssetHealthScore>> { Ok(None) }
    async fn create_prediction(&self, prediction: &FailurePrediction) -> Result<FailurePrediction> {
        let p = prediction.clone();
        sqlx::query!(r#"INSERT INTO failure_predictions (id, prediction_number, asset_id, model_id,
            prediction_type, prediction_date, predicted_failure_date, confidence, failure_mode,
            failure_probability, remaining_useful_life_days, health_score_at_prediction,
            contributing_factors, recommended_actions, priority, estimated_repair_cost, currency,
            status, actual_failure_date, work_order_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            p.base.id, p.prediction_number, p.asset_id, p.model_id, p.prediction_type, p.prediction_date,
            p.predicted_failure_date, p.confidence, p.failure_mode, p.failure_probability,
            p.remaining_useful_life_days, p.health_score_at_prediction, p.contributing_factors,
            p.recommended_actions, p.priority, p.estimated_repair_cost, p.currency, p.status,
            p.actual_failure_date, p.work_order_id, p.created_at, p.updated_at).execute(&self.pool).await?;
        Ok(p)
    }
    async fn list_predictions(&self, _asset_id: Uuid) -> Result<Vec<FailurePrediction>> { Ok(vec![]) }
    async fn create_schedule(&self, schedule: &MaintenanceSchedule) -> Result<MaintenanceSchedule> {
        let s = schedule.clone();
        sqlx::query!(r#"INSERT INTO maintenance_schedules (id, schedule_number, asset_id,
            maintenance_type, scheduled_date, estimated_duration_hours, estimated_cost, currency,
            priority, prediction_id, description, tasks, parts_required, assigned_to, status,
            completed_date, actual_cost, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            s.base.id, s.schedule_number, s.asset_id, s.maintenance_type, s.scheduled_date,
            s.estimated_duration_hours, s.estimated_cost, s.currency, s.priority, s.prediction_id,
            s.description, s.tasks, s.parts_required, s.assigned_to, s.status, s.completed_date,
            s.actual_cost, s.created_at, s.updated_at).execute(&self.pool).await?;
        Ok(s)
    }
    async fn create_anomaly(&self, anomaly: &AnomalyDetection) -> Result<AnomalyDetection> {
        let a = anomaly.clone();
        sqlx::query!(r#"INSERT INTO anomaly_detections (id, detection_number, asset_id, sensor_id,
            detection_date, anomaly_type, severity, description, measured_value, expected_value,
            deviation_percent, detection_method, model_id, acknowledged, acknowledged_by,
            acknowledged_at, resolution, resolved_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            a.base.id, a.detection_number, a.asset_id, a.sensor_id, a.detection_date, a.anomaly_type,
            a.severity, a.description, a.measured_value, a.expected_value, a.deviation_percent,
            a.detection_method, a.model_id, a.acknowledged, a.acknowledged_by, a.acknowledged_at,
            a.resolution, a.resolved_at, a.created_at).execute(&self.pool).await?;
        Ok(a)
    }
    async fn create_cost_analysis(&self, analysis: &MaintenanceCostAnalysis) -> Result<MaintenanceCostAnalysis> {
        let a = analysis.clone();
        sqlx::query!(r#"INSERT INTO maintenance_cost_analyses (id, asset_id, period_start, period_end,
            total_maintenance_cost, preventive_cost, predictive_cost, corrective_cost, downtime_cost,
            total_downtime_hours, unplanned_downtime_hours, mtbf_hours, mttr_hours, availability_percent,
            oee_percent, savings_from_predictions, currency, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            a.base.id, a.asset_id, a.period_start, a.period_end, a.total_maintenance_cost,
            a.preventive_cost, a.predictive_cost, a.corrective_cost, a.downtime_cost,
            a.total_downtime_hours, a.unplanned_downtime_hours, a.mtbf_hours, a.mttr_hours,
            a.availability_percent, a.oee_percent, a.savings_from_predictions, a.currency, a.created_at).execute(&self.pool).await?;
        Ok(a)
    }
}
