use async_trait::async_trait;
use sqlx::SqlitePool;
use erp_core::{Error, Result, Pagination, Paginated};
use crate::models::*;
use uuid::Uuid;

#[async_trait]
pub trait AIModelRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<AIModel>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<AIModel>>;
    async fn find_by_type(&self, pool: &SqlitePool, model_type: ModelType) -> Result<Vec<AIModel>>;
    async fn create(&self, pool: &SqlitePool, model: AIModel) -> Result<AIModel>;
    async fn update(&self, pool: &SqlitePool, model: &AIModel) -> Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteAIModelRepository;

#[async_trait]
impl AIModelRepository for SqliteAIModelRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<AIModel> {
        let row = sqlx::query_as::<_, AIModelRow>(
            "SELECT * FROM ai_models WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("AIModel", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<AIModel>> {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ai_models WHERE status = 'Active'")
            .fetch_one(pool)
            .await
            .map_err(Error::Database)?;
        
        let rows = sqlx::query_as::<_, AIModelRow>(
            "SELECT * FROM ai_models WHERE status = 'Active' ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit())
        .bind(pagination.offset())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(Paginated::new(rows.into_iter().map(|r| r.into()).collect(), count as u64, pagination))
    }
    
    async fn find_by_type(&self, pool: &SqlitePool, model_type: ModelType) -> Result<Vec<AIModel>> {
        let rows = sqlx::query_as::<_, AIModelRow>(
            "SELECT * FROM ai_models WHERE model_type = ? AND status = 'Active'"
        )
        .bind(format!("{:?}", model_type))
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    async fn create(&self, pool: &SqlitePool, model: AIModel) -> Result<AIModel> {
        sqlx::query(
            r#"INSERT INTO ai_models (id, name, code, model_type, algorithm, description, version, 
               parameters, features, target_variable, training_data_source, training_config, 
               validation_split, cross_validation_folds, hyperparameters, training_status,
               accuracy_score, deployment_status, auto_retrain, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(model.base.id.to_string())
        .bind(&model.name)
        .bind(&model.code)
        .bind(format!("{:?}", model.model_type))
        .bind(format!("{:?}", model.algorithm))
        .bind(&model.description)
        .bind(&model.version)
        .bind(&model.parameters)
        .bind(&model.features)
        .bind(&model.target_variable)
        .bind(&model.training_data_source)
        .bind(&model.training_config)
        .bind(model.validation_split)
        .bind(model.cross_validation_folds)
        .bind(&model.hyperparameters)
        .bind(format!("{:?}", model.training_status))
        .bind(model.accuracy_score)
        .bind(format!("{:?}", model.deployment_status))
        .bind(model.auto_retrain as i32)
        .bind(format!("{:?}", model.status))
        .bind(model.base.created_at.to_rfc3339())
        .bind(model.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(model)
    }
    
    async fn update(&self, pool: &SqlitePool, model: &AIModel) -> Result<()> {
        sqlx::query(
            r#"UPDATE ai_models SET name = ?, version = ?, training_status = ?, 
               accuracy_score = ?, deployment_status = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(&model.name)
        .bind(&model.version)
        .bind(format!("{:?}", model.training_status))
        .bind(model.accuracy_score)
        .bind(format!("{:?}", model.deployment_status))
        .bind(model.base.updated_at.to_rfc3339())
        .bind(model.base.id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
    
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE ai_models SET status = 'Inactive' WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct AIModelRow {
    id: String,
    name: String,
    code: String,
    model_type: String,
    algorithm: String,
    description: Option<String>,
    version: String,
    parameters: Option<String>,
    features: Option<String>,
    target_variable: Option<String>,
    training_data_source: Option<String>,
    training_config: Option<String>,
    validation_split: f64,
    cross_validation_folds: i32,
    hyperparameters: Option<String>,
    training_status: String,
    accuracy_score: Option<f64>,
    deployment_status: String,
    auto_retrain: i32,
    status: String,
    created_at: String,
    updated_at: String,
}

impl From<AIModelRow> for AIModel {
    fn from(r: AIModelRow) -> Self {
        use chrono::{DateTime, Utc};
        Self {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            name: r.name,
            code: r.code,
            model_type: match r.model_type.as_str() {
                "SalesPrediction" => ModelType::SalesPrediction,
                "InventoryOptimization" => ModelType::InventoryOptimization,
                "PriceOptimization" => ModelType::PriceOptimization,
                "CustomerSegmentation" => ModelType::CustomerSegmentation,
                "FraudDetection" => ModelType::FraudDetection,
                "AnomalyDetection" => ModelType::AnomalyDetection,
                "ChurnPrediction" => ModelType::ChurnPrediction,
                "LeadScoring" => ModelType::LeadScoring,
                "SentimentAnalysis" => ModelType::SentimentAnalysis,
                "Recommendation" => ModelType::Recommendation,
                "CapacityPlanning" => ModelType::CapacityPlanning,
                _ => ModelType::DemandForecast,
            },
            algorithm: match r.algorithm.as_str() {
                "RandomForest" => AlgorithmType::RandomForest,
                "GradientBoosting" => AlgorithmType::GradientBoosting,
                "NeuralNetwork" => AlgorithmType::NeuralNetwork,
                "ARIMA" => AlgorithmType::ARIMA,
                "Prophet" => AlgorithmType::Prophet,
                "XGBoost" => AlgorithmType::XGBoost,
                "LightGBM" => AlgorithmType::LightGBM,
                "LSTM" => AlgorithmType::LSTM,
                "Transformer" => AlgorithmType::Transformer,
                "KMeans" => AlgorithmType::KMeans,
                "DBSCAN" => AlgorithmType::DBSCAN,
                "IsolationForest" => AlgorithmType::IsolationForest,
                "OneClassSVM" => AlgorithmType::OneClassSVM,
                _ => AlgorithmType::LinearRegression,
            },
            description: r.description,
            version: r.version,
            parameters: r.parameters,
            features: r.features,
            target_variable: r.target_variable,
            training_data_source: r.training_data_source,
            training_config: r.training_config,
            validation_split: r.validation_split,
            cross_validation_folds: r.cross_validation_folds,
            hyperparameters: r.hyperparameters,
            training_status: match r.training_status.as_str() {
                "Training" => ModelStatus::Training,
                "Trained" => ModelStatus::Trained,
                "Deployed" => ModelStatus::Deployed,
                "Deprecated" => ModelStatus::Deprecated,
                "Failed" => ModelStatus::Failed,
                _ => ModelStatus::Draft,
            },
            training_started_at: None,
            training_completed_at: None,
            training_duration_seconds: None,
            training_samples: None,
            validation_metrics: None,
            test_metrics: None,
            accuracy_score: r.accuracy_score,
            precision_score: None,
            recall_score: None,
            f1_score: None,
            rmse: None,
            mae: None,
            mape: None,
            r2_score: None,
            auc_roc: None,
            feature_importance: None,
            model_artifact_path: None,
            deployment_endpoint: None,
            deployment_status: match r.deployment_status.as_str() {
                "Draft" => ModelStatus::Draft,
                "Training" => ModelStatus::Training,
                "Trained" => ModelStatus::Trained,
                "Deprecated" => ModelStatus::Deprecated,
                "Failed" => ModelStatus::Failed,
                _ => ModelStatus::Deployed,
            },
            deployed_at: None,
            inference_count: 0,
            last_inference_at: None,
            drift_detected: false,
            drift_score: None,
            retraining_required: false,
            auto_retrain: r.auto_retrain != 0,
            retrain_threshold: None,
            retrain_schedule_cron: None,
            last_retrain_at: None,
            owner_id: None,
            tags: None,
            status: match r.status.as_str() {
                "Inactive" => erp_core::Status::Inactive,
                _ => erp_core::Status::Active,
            },
        }
    }
}

#[async_trait]
pub trait PredictionRequestRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, request: PredictionRequest) -> Result<PredictionRequest>;
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<PredictionRequest>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: PredictionStatus, result: Option<String>) -> Result<()>;
}

pub struct SqlitePredictionRequestRepository;

#[async_trait]
impl PredictionRequestRepository for SqlitePredictionRequestRepository {
    async fn create(&self, pool: &SqlitePool, request: PredictionRequest) -> Result<PredictionRequest> {
        sqlx::query(
            r#"INSERT INTO prediction_requests (id, model_id, request_id, input_data, 
               batch_mode, priority, status, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(request.base.id.to_string())
        .bind(request.model_id.to_string())
        .bind(&request.request_id)
        .bind(&request.input_data)
        .bind(request.batch_mode as i32)
        .bind(request.priority)
        .bind(format!("{:?}", request.status))
        .bind(request.base.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(request)
    }
    
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<PredictionRequest> {
        let row = sqlx::query_as::<_, PredictionRequestRow>(
            "SELECT * FROM prediction_requests WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("PredictionRequest", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: PredictionStatus, result: Option<String>) -> Result<()> {
        sqlx::query(
            "UPDATE prediction_requests SET status = ?, result = ?, processed_at = ? WHERE id = ?"
        )
        .bind(format!("{:?}", status))
        .bind(&result)
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct PredictionRequestRow {
    id: String,
    model_id: String,
    request_id: String,
    input_data: String,
    batch_mode: i32,
    priority: i32,
    status: String,
    result: Option<String>,
    created_at: String,
    processed_at: Option<String>,
}

impl From<PredictionRequestRow> for PredictionRequest {
    fn from(r: PredictionRequestRow) -> Self {
        use chrono::{DateTime, Utc};
        Self {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: Utc::now(),
                created_by: None,
                updated_by: None,
            },
            model_id: Uuid::parse_str(&r.model_id).unwrap_or_default(),
            request_id: r.request_id,
            input_data: r.input_data,
            input_features: None,
            batch_mode: r.batch_mode != 0,
            batch_size: None,
            priority: r.priority,
            status: match r.status.as_str() {
                "Processing" => PredictionStatus::Processing,
                "Completed" => PredictionStatus::Completed,
                "Failed" => PredictionStatus::Failed,
                _ => PredictionStatus::Pending,
            },
            result: r.result,
            predictions: None,
            confidence_scores: None,
            explanation: None,
            feature_contributions: None,
            processing_time_ms: None,
            created_by: None,
            processed_at: r.processed_at.and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            error_message: None,
        }
    }
}

#[async_trait]
pub trait AnomalyDetectionRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, anomaly: AnomalyDetection) -> Result<AnomalyDetection>;
    async fn find_unresolved(&self, pool: &SqlitePool, limit: i64) -> Result<Vec<AnomalyDetection>>;
    async fn acknowledge(&self, pool: &SqlitePool, id: Uuid, user_id: Uuid) -> Result<()>;
    async fn resolve(&self, pool: &SqlitePool, id: Uuid, notes: Option<String>) -> Result<()>;
}

pub struct SqliteAnomalyDetectionRepository;

#[async_trait]
impl AnomalyDetectionRepository for SqliteAnomalyDetectionRepository {
    async fn create(&self, pool: &SqlitePool, anomaly: AnomalyDetection) -> Result<AnomalyDetection> {
        sqlx::query(
            r#"INSERT INTO anomaly_detections (id, model_id, entity_type, entity_id, anomaly_type,
               severity, detected_at, metric_name, expected_value, actual_value, deviation_percent,
               confidence_score, detection_method, status, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(anomaly.base.id.to_string())
        .bind(anomaly.model_id.map(|id| id.to_string()))
        .bind(&anomaly.entity_type)
        .bind(anomaly.entity_id.to_string())
        .bind(format!("{:?}", anomaly.anomaly_type))
        .bind(format!("{:?}", anomaly.severity))
        .bind(anomaly.detected_at.to_rfc3339())
        .bind(&anomaly.metric_name)
        .bind(anomaly.expected_value)
        .bind(anomaly.actual_value)
        .bind(anomaly.deviation_percent)
        .bind(anomaly.confidence_score)
        .bind(&anomaly.detection_method)
        .bind(format!("{:?}", anomaly.status))
        .bind(anomaly.base.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(anomaly)
    }
    
    async fn find_unresolved(&self, pool: &SqlitePool, limit: i64) -> Result<Vec<AnomalyDetection>> {
        let rows = sqlx::query_as::<_, AnomalyRow>(
            "SELECT * FROM anomaly_detections WHERE status IN ('New', 'Investigating', 'Acknowledged') ORDER BY detected_at DESC LIMIT ?"
        )
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    async fn acknowledge(&self, pool: &SqlitePool, id: Uuid, user_id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE anomaly_detections SET status = 'Acknowledged', acknowledged_at = ?, acknowledged_by = ? WHERE id = ?"
        )
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(user_id.to_string())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        Ok(())
    }
    
    async fn resolve(&self, pool: &SqlitePool, id: Uuid, notes: Option<String>) -> Result<()> {
        sqlx::query(
            "UPDATE anomaly_detections SET status = 'Resolved', resolved_at = ?, resolution_notes = ? WHERE id = ?"
        )
        .bind(chrono::Utc::now().to_rfc3339())
        .bind(&notes)
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct AnomalyRow {
    id: String,
    model_id: Option<String>,
    entity_type: String,
    entity_id: String,
    anomaly_type: String,
    severity: String,
    detected_at: String,
    metric_name: String,
    expected_value: f64,
    actual_value: f64,
    deviation_percent: f64,
    confidence_score: f64,
    detection_method: String,
    status: String,
    created_at: String,
}

impl From<AnomalyRow> for AnomalyDetection {
    fn from(r: AnomalyRow) -> Self {
        use chrono::{DateTime, Utc};
        Self {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: Utc::now(),
                created_by: None,
                updated_by: None,
            },
            model_id: r.model_id.and_then(|id| Uuid::parse_str(&id).ok()),
            entity_type: r.entity_type,
            entity_id: Uuid::parse_str(&r.entity_id).unwrap_or_default(),
            anomaly_type: match r.anomaly_type.as_str() {
                "Contextual" => AnomalyType::Contextual,
                "Collective" => AnomalyType::Collective,
                "Trend" => AnomalyType::Trend,
                "SeasonalityBreak" => AnomalyType::SeasonalityBreak,
                "Outlier" => AnomalyType::Outlier,
                "Drift" => AnomalyType::Drift,
                _ => AnomalyType::Point,
            },
            severity: match r.severity.as_str() {
                "Medium" => AnomalySeverity::Medium,
                "High" => AnomalySeverity::High,
                "Critical" => AnomalySeverity::Critical,
                _ => AnomalySeverity::Low,
            },
            detected_at: DateTime::parse_from_rfc3339(&r.detected_at)
                .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            metric_name: r.metric_name,
            expected_value: r.expected_value,
            actual_value: r.actual_value,
            deviation_percent: r.deviation_percent,
            z_score: None,
            confidence_score: r.confidence_score,
            detection_method: r.detection_method,
            context_data: None,
            root_cause_analysis: None,
            related_anomalies: None,
            status: match r.status.as_str() {
                "Investigating" => AnomalyStatus::Investigating,
                "Acknowledged" => AnomalyStatus::Acknowledged,
                "Resolved" => AnomalyStatus::Resolved,
                "FalsePositive" => AnomalyStatus::FalsePositive,
                _ => AnomalyStatus::New,
            },
            acknowledged_at: None,
            acknowledged_by: None,
            resolution_notes: None,
            resolved_at: None,
            false_positive: false,
        }
    }
}
