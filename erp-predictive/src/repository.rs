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
        let id = s.base.id.to_string();
        let asset_id = s.asset_id.to_string();
        let created_at = s.created_at.to_rfc3339();
        let updated_at = s.updated_at.to_rfc3339();
        let last_reading_at = s.last_reading_at.map(|d| d.to_rfc3339());
        sqlx::query(r#"INSERT INTO asset_sensors (id, sensor_number, asset_id, name, sensor_type,
            manufacturer, model, serial_number, location, measurement_unit, sampling_interval_seconds,
            data_source, connection_type, last_reading, last_reading_at, min_threshold, max_threshold,
            alert_threshold_low, alert_threshold_high, status, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(&id)
            .bind(&s.sensor_number)
            .bind(&asset_id)
            .bind(&s.name)
            .bind(&s.sensor_type)
            .bind(&s.manufacturer)
            .bind(&s.model)
            .bind(&s.serial_number)
            .bind(&s.location)
            .bind(&s.measurement_unit)
            .bind(s.sampling_interval_seconds)
            .bind(&s.data_source)
            .bind(&s.connection_type)
            .bind(s.last_reading)
            .bind(&last_reading_at)
            .bind(s.min_threshold)
            .bind(s.max_threshold)
            .bind(s.alert_threshold_low)
            .bind(s.alert_threshold_high)
            .bind(&s.status)
            .bind(&created_at)
            .bind(&updated_at)
            .execute(&self.pool).await?;
        Ok(s)
    }
    async fn get_sensor(&self, _id: Uuid) -> Result<Option<AssetSensor>> { Ok(None) }
    async fn list_sensors(&self, _asset_id: Uuid) -> Result<Vec<AssetSensor>> { Ok(vec![]) }
    async fn create_reading(&self, reading: &SensorReading) -> Result<SensorReading> {
        let r = reading.clone();
        let id = r.id.to_string();
        let sensor_id = r.sensor_id.to_string();
        let reading_timestamp = r.reading_timestamp.to_rfc3339();
        let created_at = r.created_at.to_rfc3339();
        sqlx::query(r#"INSERT INTO sensor_readings (id, sensor_id, reading_timestamp, value, unit,
            quality, raw_value, is_anomaly, anomaly_score, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(&id)
            .bind(&sensor_id)
            .bind(&reading_timestamp)
            .bind(r.value)
            .bind(&r.unit)
            .bind(&r.quality)
            .bind(r.raw_value)
            .bind(r.is_anomaly)
            .bind(r.anomaly_score)
            .bind(&created_at)
            .execute(&self.pool).await?;
        Ok(r)
    }
    async fn list_readings(&self, _sensor_id: Uuid, _limit: i32) -> Result<Vec<SensorReading>> { Ok(vec![]) }
    async fn create_model(&self, model: &PredictiveModel) -> Result<PredictiveModel> {
        let m = model.clone();
        let id = m.base.id.to_string();
        let asset_type_id = m.asset_type_id.map(|u| u.to_string());
        let training_data_start = m.training_data_start.map(|d| d.to_string());
        let training_data_end = m.training_data_end.map(|d| d.to_string());
        let last_trained_at = m.last_trained_at.map(|d| d.to_rfc3339());
        let created_at = m.created_at.to_rfc3339();
        let updated_at = m.updated_at.to_rfc3339();
        sqlx::query(r#"INSERT INTO predictive_models (id, model_number, name, description, model_type,
            algorithm, version, asset_type_id, target_variable, features, training_data_start,
            training_data_end, training_samples, accuracy, precision, recall, f1_score, auc_roc,
            confusion_matrix, feature_importance, model_path, hyperparameters, retraining_frequency_days,
            last_trained_at, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(&id)
            .bind(&m.model_number)
            .bind(&m.name)
            .bind(&m.description)
            .bind(&m.model_type)
            .bind(&m.algorithm)
            .bind(&m.version)
            .bind(&asset_type_id)
            .bind(&m.target_variable)
            .bind(&m.features)
            .bind(&training_data_start)
            .bind(&training_data_end)
            .bind(m.training_samples)
            .bind(m.accuracy)
            .bind(m.precision)
            .bind(m.recall)
            .bind(m.f1_score)
            .bind(m.auc_roc)
            .bind(&m.confusion_matrix)
            .bind(&m.feature_importance)
            .bind(&m.model_path)
            .bind(&m.hyperparameters)
            .bind(m.retraining_frequency_days)
            .bind(&last_trained_at)
            .bind(&m.status)
            .bind(&created_at)
            .bind(&updated_at)
            .execute(&self.pool).await?;
        Ok(m)
    }
    async fn create_health_score(&self, score: &AssetHealthScore) -> Result<AssetHealthScore> {
        let s = score.clone();
        let id = s.base.id.to_string();
        let asset_id = s.asset_id.to_string();
        let score_date = s.score_date.to_string();
        let created_at = s.created_at.to_rfc3339();
        sqlx::query(r#"INSERT INTO asset_health_scores (id, asset_id, score_date, overall_score,
            previous_score, score_change, trend, reliability_score, performance_score, maintenance_score,
            component_scores, risk_level, days_to_failure, recommended_action, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(&id)
            .bind(&asset_id)
            .bind(&score_date)
            .bind(s.overall_score)
            .bind(s.previous_score)
            .bind(s.score_change)
            .bind(&s.trend)
            .bind(s.reliability_score)
            .bind(s.performance_score)
            .bind(s.maintenance_score)
            .bind(&s.component_scores)
            .bind(&s.risk_level)
            .bind(s.days_to_failure)
            .bind(&s.recommended_action)
            .bind(&created_at)
            .execute(&self.pool).await?;
        Ok(s)
    }
    async fn get_latest_health_score(&self, _asset_id: Uuid) -> Result<Option<AssetHealthScore>> { Ok(None) }
    async fn create_prediction(&self, prediction: &FailurePrediction) -> Result<FailurePrediction> {
        let p = prediction.clone();
        let id = p.base.id.to_string();
        let asset_id = p.asset_id.to_string();
        let model_id = p.model_id.map(|u| u.to_string());
        let prediction_date = p.prediction_date.to_rfc3339();
        let predicted_failure_date = p.predicted_failure_date.to_string();
        let actual_failure_date = p.actual_failure_date.map(|d| d.to_string());
        let work_order_id = p.work_order_id.map(|u| u.to_string());
        let created_at = p.created_at.to_rfc3339();
        let updated_at = p.updated_at.to_rfc3339();
        sqlx::query(r#"INSERT INTO failure_predictions (id, prediction_number, asset_id, model_id,
            prediction_type, prediction_date, predicted_failure_date, confidence, failure_mode,
            failure_probability, remaining_useful_life_days, health_score_at_prediction,
            contributing_factors, recommended_actions, priority, estimated_repair_cost, currency,
            status, actual_failure_date, work_order_id, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(&id)
            .bind(&p.prediction_number)
            .bind(&asset_id)
            .bind(&model_id)
            .bind(&p.prediction_type)
            .bind(&prediction_date)
            .bind(&predicted_failure_date)
            .bind(p.confidence)
            .bind(&p.failure_mode)
            .bind(p.failure_probability)
            .bind(p.remaining_useful_life_days)
            .bind(p.health_score_at_prediction)
            .bind(&p.contributing_factors)
            .bind(&p.recommended_actions)
            .bind(p.priority)
            .bind(p.estimated_repair_cost)
            .bind(&p.currency)
            .bind(&p.status)
            .bind(&actual_failure_date)
            .bind(&work_order_id)
            .bind(&created_at)
            .bind(&updated_at)
            .execute(&self.pool).await?;
        Ok(p)
    }
    async fn list_predictions(&self, _asset_id: Uuid) -> Result<Vec<FailurePrediction>> { Ok(vec![]) }
    async fn create_schedule(&self, schedule: &MaintenanceSchedule) -> Result<MaintenanceSchedule> {
        let s = schedule.clone();
        let id = s.base.id.to_string();
        let asset_id = s.asset_id.to_string();
        let scheduled_date = s.scheduled_date.to_string();
        let prediction_id = s.prediction_id.map(|u| u.to_string());
        let assigned_to = s.assigned_to.map(|u| u.to_string());
        let completed_date = s.completed_date.map(|d| d.to_string());
        let created_at = s.created_at.to_rfc3339();
        let updated_at = s.updated_at.to_rfc3339();
        sqlx::query(r#"INSERT INTO maintenance_schedules (id, schedule_number, asset_id,
            maintenance_type, scheduled_date, estimated_duration_hours, estimated_cost, currency,
            priority, prediction_id, description, tasks, parts_required, assigned_to, status,
            completed_date, actual_cost, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(&id)
            .bind(&s.schedule_number)
            .bind(&asset_id)
            .bind(&s.maintenance_type)
            .bind(&scheduled_date)
            .bind(s.estimated_duration_hours)
            .bind(s.estimated_cost)
            .bind(&s.currency)
            .bind(s.priority)
            .bind(&prediction_id)
            .bind(&s.description)
            .bind(&s.tasks)
            .bind(&s.parts_required)
            .bind(&assigned_to)
            .bind(&s.status)
            .bind(&completed_date)
            .bind(s.actual_cost)
            .bind(&created_at)
            .bind(&updated_at)
            .execute(&self.pool).await?;
        Ok(s)
    }
    async fn create_anomaly(&self, anomaly: &AnomalyDetection) -> Result<AnomalyDetection> {
        let a = anomaly.clone();
        let id = a.base.id.to_string();
        let asset_id = a.asset_id.to_string();
        let sensor_id = a.sensor_id.map(|u| u.to_string());
        let detection_date = a.detection_date.to_rfc3339();
        let model_id = a.model_id.map(|u| u.to_string());
        let acknowledged_by = a.acknowledged_by.map(|u| u.to_string());
        let acknowledged_at = a.acknowledged_at.map(|d| d.to_rfc3339());
        let resolved_at = a.resolved_at.map(|d| d.to_rfc3339());
        let created_at = a.created_at.to_rfc3339();
        sqlx::query(r#"INSERT INTO anomaly_detections (id, detection_number, asset_id, sensor_id,
            detection_date, anomaly_type, severity, description, measured_value, expected_value,
            deviation_percent, detection_method, model_id, acknowledged, acknowledged_by,
            acknowledged_at, resolution, resolved_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(&id)
            .bind(&a.detection_number)
            .bind(&asset_id)
            .bind(&sensor_id)
            .bind(&detection_date)
            .bind(&a.anomaly_type)
            .bind(&a.severity)
            .bind(&a.description)
            .bind(a.measured_value)
            .bind(a.expected_value)
            .bind(a.deviation_percent)
            .bind(&a.detection_method)
            .bind(&model_id)
            .bind(a.acknowledged)
            .bind(&acknowledged_by)
            .bind(&acknowledged_at)
            .bind(&a.resolution)
            .bind(&resolved_at)
            .bind(&created_at)
            .execute(&self.pool).await?;
        Ok(a)
    }
    async fn create_cost_analysis(&self, analysis: &MaintenanceCostAnalysis) -> Result<MaintenanceCostAnalysis> {
        let a = analysis.clone();
        let id = a.base.id.to_string();
        let asset_id = a.asset_id.to_string();
        let period_start = a.period_start.to_string();
        let period_end = a.period_end.to_string();
        let created_at = a.created_at.to_rfc3339();
        sqlx::query(r#"INSERT INTO maintenance_cost_analyses (id, asset_id, period_start, period_end,
            total_maintenance_cost, preventive_cost, predictive_cost, corrective_cost, downtime_cost,
            total_downtime_hours, unplanned_downtime_hours, mtbf_hours, mttr_hours, availability_percent,
            oee_percent, savings_from_predictions, currency, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(&id)
            .bind(&asset_id)
            .bind(&period_start)
            .bind(&period_end)
            .bind(a.total_maintenance_cost)
            .bind(a.preventive_cost)
            .bind(a.predictive_cost)
            .bind(a.corrective_cost)
            .bind(a.downtime_cost)
            .bind(a.total_downtime_hours)
            .bind(a.unplanned_downtime_hours)
            .bind(a.mtbf_hours)
            .bind(a.mttr_hours)
            .bind(a.availability_percent)
            .bind(a.oee_percent)
            .bind(a.savings_from_predictions)
            .bind(&a.currency)
            .bind(&created_at)
            .execute(&self.pool).await?;
        Ok(a)
    }
}
