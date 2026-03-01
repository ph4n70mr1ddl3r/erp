use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use erp_core::Pagination;
use crate::db::AppState;
use erp_ai::{AIModelService, PredictionService, AnomalyService, ForecastService, RecommendationService, CustomerInsightService};
use erp_ai::{AIModel, ModelType, ModelStatus};

#[derive(Deserialize)]
pub struct CreateModelRequest {
    pub name: String,
    pub code: String,
    pub model_type: String,
    pub algorithm: String,
    pub description: Option<String>,
    pub target_variable: Option<String>,
    pub training_data_source: Option<String>,
}

#[derive(Serialize)]
pub struct ModelResponse {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub model_type: String,
    pub algorithm: String,
    pub status: String,
    pub training_status: String,
    pub accuracy_score: Option<f64>,
}

impl From<AIModel> for ModelResponse {
    fn from(m: AIModel) -> Self {
        Self {
            id: m.base.id,
            name: m.name,
            code: m.code,
            model_type: format!("{:?}", m.model_type),
            algorithm: format!("{:?}", m.algorithm),
            status: format!("{:?}", m.status),
            training_status: format!("{:?}", m.training_status),
            accuracy_score: m.accuracy_score,
        }
    }
}

#[derive(Deserialize)]
pub struct PredictRequest {
    pub model_id: Uuid,
    pub input_data: String,
    pub batch_mode: Option<bool>,
}

fn new_ai_model() -> AIModel {
    AIModel {
        base: erp_core::BaseEntity::new(),
        name: String::new(),
        code: String::new(),
        model_type: ModelType::DemandForecast,
        algorithm: erp_ai::AlgorithmType::LinearRegression,
        description: None,
        version: "1.0.0".to_string(),
        parameters: None,
        features: None,
        target_variable: None,
        training_data_source: None,
        training_config: None,
        validation_split: 0.2,
        cross_validation_folds: 5,
        hyperparameters: None,
        training_status: ModelStatus::Draft,
        training_started_at: None,
        training_completed_at: None,
        training_duration_seconds: None,
        training_samples: None,
        validation_metrics: None,
        test_metrics: None,
        accuracy_score: None,
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
        deployment_status: ModelStatus::Draft,
        deployed_at: None,
        inference_count: 0,
        last_inference_at: None,
        drift_detected: false,
        drift_score: None,
        retraining_required: false,
        auto_retrain: false,
        retrain_threshold: None,
        retrain_schedule_cron: None,
        last_retrain_at: None,
        owner_id: None,
        tags: None,
        status: erp_core::Status::Active,
    }
}

#[derive(Deserialize)]
pub struct AnomalyCheckRequest {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub metric_name: String,
    pub value: f64,
}

#[derive(Deserialize)]
pub struct ForecastRequest {
    pub product_id: Uuid,
    pub horizon_days: i32,
}

pub async fn list_models(
    State(state): State<AppState>,
    Query(pagination): Query<Pagination>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = AIModelService::new();
    let result = service.list(&state.pool, pagination).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "items": result.items.into_iter().map(ModelResponse::from).collect::<Vec<_>>(),
        "total": result.total,
        "page": result.page,
        "per_page": result.per_page
    })))
}

pub async fn get_model(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ModelResponse>, (StatusCode, String)> {
    let service = AIModelService::new();
    let model = service.get(&state.pool, id).await
        .map_err(|e| (StatusCode::NOT_FOUND, e.to_string()))?;
    Ok(Json(ModelResponse::from(model)))
}

pub async fn create_model(
    State(state): State<AppState>,
    Json(req): Json<CreateModelRequest>,
) -> Result<Json<ModelResponse>, (StatusCode, String)> {
    let service = AIModelService::new();
    
    let model = AIModel {
        base: erp_core::BaseEntity::new(),
        name: req.name,
        code: req.code,
        model_type: match req.model_type.as_str() {
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
        algorithm: erp_ai::AlgorithmType::RandomForest,
        description: req.description,
        version: "1.0.0".to_string(),
        target_variable: req.target_variable,
        training_data_source: req.training_data_source,
        status: erp_core::Status::Active,
        training_status: ModelStatus::Draft,
        deployment_status: ModelStatus::Draft,
        ..new_ai_model()
    };
    
    let model = service.create(&state.pool, model).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    Ok(Json(ModelResponse::from(model)))
}

pub async fn deploy_model(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<ModelResponse>, (StatusCode, String)> {
    let service = AIModelService::new();
    let model = service.deploy(&state.pool, id).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    Ok(Json(ModelResponse::from(model)))
}

pub async fn predict(
    State(state): State<AppState>,
    Json(req): Json<PredictRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = PredictionService::new();
    let request = service.create_request(&state.pool, req.model_id, req.input_data, req.batch_mode.unwrap_or(false)).await
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    
    let request = service.process_request(&state.pool, request.base.id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "request_id": request.base.id,
        "status": format!("{:?}", request.status),
        "result": request.result,
        "processing_time_ms": request.processing_time_ms
    })))
}

pub async fn forecast_demand(
    State(state): State<AppState>,
    Json(req): Json<ForecastRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = ForecastService::new();
    let forecast = service.generate_demand_forecast(&state.pool, req.product_id, req.horizon_days).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "id": forecast.base.id,
        "product_id": forecast.product_id,
        "horizon_days": forecast.horizon_days,
        "forecasts": forecast.forecasts,
        "generated_at": forecast.generated_at
    })))
}

pub async fn list_anomalies(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = AnomalyService::new();
    let anomalies = service.list_unresolved(&state.pool, 100).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "anomalies": anomalies.into_iter().map(|a| serde_json::json!({
            "id": a.base.id,
            "entity_type": a.entity_type,
            "entity_id": a.entity_id,
            "severity": format!("{:?}", a.severity),
            "metric_name": a.metric_name,
            "actual_value": a.actual_value,
            "expected_value": a.expected_value,
            "detected_at": a.detected_at
        })).collect::<Vec<_>>()
    })))
}

pub async fn acknowledge_anomaly(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = AnomalyService::new();
    service.acknowledge(&state.pool, id, Uuid::nil()).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn resolve_anomaly(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = AnomalyService::new();
    service.resolve(&state.pool, id, Some("Resolved via API".to_string())).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    Ok(Json(serde_json::json!({ "success": true })))
}

pub async fn get_recommendations(
    State(state): State<AppState>,
    Path(customer_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = RecommendationService::new();
    let recommendations = service.get_product_recommendations(&state.pool, customer_id, 10).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "recommendations": recommendations.into_iter().map(|r| serde_json::json!({
            "customer_id": r.customer_id,
            "items": r.recommendations
        })).collect::<Vec<_>>()
    })))
}

pub async fn get_customer_insights(
    State(state): State<AppState>,
    Path(customer_id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, (StatusCode, String)> {
    let service = CustomerInsightService::new();
    let insights = service.calculate_insights(&state.pool, customer_id).await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    
    Ok(Json(serde_json::json!({
        "customer_id": insights.customer_id,
        "segment": insights.segment,
        "lifetime_value": insights.lifetime_value,
        "churn_probability": insights.churn_probability,
        "engagement_score": insights.engagement_score,
        "next_best_action": insights.next_best_action
    })))
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/models", axum::routing::get(list_models).post(create_model))
        .route("/models/:id", axum::routing::get(get_model))
        .route("/models/:id/deploy", axum::routing::post(deploy_model))
        .route("/predict", axum::routing::post(predict))
        .route("/forecast/demand", axum::routing::post(forecast_demand))
        .route("/anomalies", axum::routing::get(list_anomalies))
        .route("/anomalies/:id/acknowledge", axum::routing::post(acknowledge_anomaly))
        .route("/anomalies/:id/resolve", axum::routing::post(resolve_anomaly))
        .route("/recommendations/:customer_id", axum::routing::get(get_recommendations))
        .route("/insights/:customer_id", axum::routing::get(get_customer_insights))
}
