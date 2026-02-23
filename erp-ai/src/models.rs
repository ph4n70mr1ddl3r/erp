use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ModelType {
    DemandForecast,
    SalesPrediction,
    InventoryOptimization,
    PriceOptimization,
    CustomerSegmentation,
    FraudDetection,
    AnomalyDetection,
    ChurnPrediction,
    LeadScoring,
    SentimentAnalysis,
    Recommendation,
    CapacityPlanning,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ModelStatus {
    Draft,
    Training,
    Trained,
    Deployed,
    Deprecated,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AlgorithmType {
    LinearRegression,
    RandomForest,
    GradientBoosting,
    NeuralNetwork,
    ARIMA,
    Prophet,
    XGBoost,
    LightGBM,
    LSTM,
    Transformer,
    KMeans,
    DBSCAN,
    IsolationForest,
    OneClassSVM,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModel {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub model_type: ModelType,
    pub algorithm: AlgorithmType,
    pub description: Option<String>,
    pub version: String,
    pub parameters: Option<String>,
    pub features: Option<String>,
    pub target_variable: Option<String>,
    pub training_data_source: Option<String>,
    pub training_config: Option<String>,
    pub validation_split: f64,
    pub cross_validation_folds: i32,
    pub hyperparameters: Option<String>,
    pub training_status: ModelStatus,
    pub training_started_at: Option<DateTime<Utc>>,
    pub training_completed_at: Option<DateTime<Utc>>,
    pub training_duration_seconds: Option<i64>,
    pub training_samples: Option<i64>,
    pub validation_metrics: Option<String>,
    pub test_metrics: Option<String>,
    pub accuracy_score: Option<f64>,
    pub precision_score: Option<f64>,
    pub recall_score: Option<f64>,
    pub f1_score: Option<f64>,
    pub rmse: Option<f64>,
    pub mae: Option<f64>,
    pub mape: Option<f64>,
    pub r2_score: Option<f64>,
    pub auc_roc: Option<f64>,
    pub feature_importance: Option<String>,
    pub model_artifact_path: Option<String>,
    pub deployment_endpoint: Option<String>,
    pub deployment_status: ModelStatus,
    pub deployed_at: Option<DateTime<Utc>>,
    pub inference_count: i64,
    pub last_inference_at: Option<DateTime<Utc>>,
    pub drift_detected: bool,
    pub drift_score: Option<f64>,
    pub retraining_required: bool,
    pub auto_retrain: bool,
    pub retrain_threshold: Option<f64>,
    pub retrain_schedule_cron: Option<String>,
    pub last_retrain_at: Option<DateTime<Utc>>,
    pub owner_id: Option<Uuid>,
    pub tags: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingJob {
    pub base: BaseEntity,
    pub model_id: Uuid,
    pub job_number: String,
    pub data_source: String,
    pub data_filters: Option<String>,
    pub feature_config: Option<String>,
    pub parameter_overrides: Option<String>,
    pub training_config: Option<String>,
    pub status: TrainingJobStatus,
    pub progress_percent: i32,
    pub current_epoch: Option<i32>,
    pub total_epochs: Option<i32>,
    pub current_batch: Option<i64>,
    pub total_batches: Option<i64>,
    pub loss_value: Option<f64>,
    pub metrics_history: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
    pub samples_processed: Option<i64>,
    pub error_message: Option<String>,
    pub compute_resource: Option<String>,
    pub gpu_used: bool,
    pub memory_used_mb: Option<i64>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum TrainingJobStatus {
    Queued,
    Preparing,
    Training,
    Validating,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionRequest {
    pub base: BaseEntity,
    pub model_id: Uuid,
    pub request_id: String,
    pub input_data: String,
    pub input_features: Option<String>,
    pub batch_mode: bool,
    pub batch_size: Option<i32>,
    pub priority: i32,
    pub status: PredictionStatus,
    pub result: Option<String>,
    pub predictions: Option<String>,
    pub confidence_scores: Option<String>,
    pub explanation: Option<String>,
    pub feature_contributions: Option<String>,
    pub processing_time_ms: Option<i64>,
    pub created_by: Option<Uuid>,
    pub processed_at: Option<DateTime<Utc>>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PredictionStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandForecast {
    pub base: BaseEntity,
    pub model_id: Option<Uuid>,
    pub product_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub forecast_date: DateTime<Utc>,
    pub horizon_days: i32,
    pub forecast_type: ForecastType,
    pub granularity: ForecastGranularity,
    pub forecasts: String,
    pub confidence_intervals: Option<String>,
    pub lower_bound: Option<f64>,
    pub upper_bound: Option<f64>,
    pub actual_values: Option<String>,
    pub accuracy_metrics: Option<String>,
    pub mape: Option<f64>,
    pub mase: Option<f64>,
    pub wape: Option<f64>,
    pub factors: Option<String>,
    pub seasonality_detected: bool,
    pub trend: Option<String>,
    pub status: Status,
    pub generated_at: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ForecastType {
    Sales,
    Demand,
    Inventory,
    Revenue,
    Capacity,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ForecastGranularity {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnomalyDetection {
    pub base: BaseEntity,
    pub model_id: Option<Uuid>,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub anomaly_type: AnomalyType,
    pub severity: AnomalySeverity,
    pub detected_at: DateTime<Utc>,
    pub metric_name: String,
    pub expected_value: f64,
    pub actual_value: f64,
    pub deviation_percent: f64,
    pub z_score: Option<f64>,
    pub confidence_score: f64,
    pub detection_method: String,
    pub context_data: Option<String>,
    pub root_cause_analysis: Option<String>,
    pub related_anomalies: Option<String>,
    pub status: AnomalyStatus,
    pub acknowledged_at: Option<DateTime<Utc>>,
    pub acknowledged_by: Option<Uuid>,
    pub resolution_notes: Option<String>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub false_positive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AnomalyType {
    Point,
    Contextual,
    Collective,
    Trend,
    SeasonalityBreak,
    Outlier,
    Drift,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AnomalySeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum AnomalyStatus {
    New,
    Investigating,
    Acknowledged,
    Resolved,
    FalsePositive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerInsight {
    pub base: BaseEntity,
    pub customer_id: Uuid,
    pub insight_type: InsightType,
    pub segment: Option<String>,
    pub segment_score: Option<f64>,
    pub lifetime_value: Option<f64>,
    pub churn_probability: Option<f64>,
    pub churn_risk_level: Option<String>,
    pub next_best_action: Option<String>,
    pub recommended_products: Option<String>,
    pub cross_sell_opportunities: Option<String>,
    pub upsell_opportunities: Option<String>,
    pub purchase_propensity: Option<String>,
    pub engagement_score: Option<f64>,
    pub satisfaction_score: Option<f64>,
    pub nps_score: Option<i32>,
    pub sentiment: Option<String>,
    pub behavior_patterns: Option<String>,
    pub preferences: Option<String>,
    pub risk_factors: Option<String>,
    pub last_purchase_prediction: Option<DateTime<Utc>>,
    pub predicted_order_value: Option<f64>,
    pub model_version: Option<String>,
    pub calculated_at: DateTime<Utc>,
    pub valid_until: Option<DateTime<Utc>>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum InsightType {
    Segmentation,
    ChurnRisk,
    LifetimeValue,
    NextBestAction,
    Recommendation,
    Sentiment,
    Behavior,
    Propensity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationEngine {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub recommendation_type: RecommendationType,
    pub algorithm: RecommendationAlgorithm,
    pub description: Option<String>,
    pub model_id: Option<Uuid>,
    pub config: Option<String>,
    pub fallback_strategy: Option<String>,
    pub diversity_factor: f64,
    pub novelty_factor: f64,
    pub cold_start_strategy: Option<String>,
    pub context_features: Option<String>,
    pub ranking_strategy: Option<String>,
    pub ab_test_enabled: bool,
    pub ab_test_config: Option<String>,
    pub metrics_window_days: i32,
    pub click_through_rate: Option<f64>,
    pub conversion_rate: Option<f64>,
    pub average_rating: Option<f64>,
    pub total_recommendations: i64,
    pub total_clicks: i64,
    pub total_conversions: i64,
    pub last_optimized_at: Option<DateTime<Utc>>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum RecommendationType {
    Product,
    Content,
    Customer,
    Price,
    Promotion,
    CrossSell,
    UpSell,
    Bundle,
    Similar,
    Complementary,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum RecommendationAlgorithm {
    CollaborativeFiltering,
    ContentBased,
    MatrixFactorization,
    DeepLearning,
    Hybrid,
    AssociationRules,
    KnowledgeBased,
    DemographicBased,
    ContextAware,
    GraphBased,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationResult {
    pub id: Uuid,
    pub engine_id: Uuid,
    pub user_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub session_id: Option<String>,
    pub context: Option<String>,
    pub recommendations: String,
    pub scores: Option<String>,
    pub reasons: Option<String>,
    pub diversity_score: Option<f64>,
    pub novelty_score: Option<f64>,
    pub displayed_at: Option<DateTime<Utc>>,
    pub clicked_at: Option<DateTime<Utc>>,
    pub clicked_item_id: Option<Uuid>,
    pub clicked_position: Option<i32>,
    pub converted_at: Option<DateTime<Utc>>,
    pub converted_item_id: Option<Uuid>,
    pub conversion_value: Option<f64>,
    pub ab_test_variant: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMonitoringMetric {
    pub id: Uuid,
    pub model_id: Uuid,
    pub metric_name: String,
    pub metric_type: MonitoringMetricType,
    pub value: f64,
    pub threshold_warning: Option<f64>,
    pub threshold_critical: Option<f64>,
    pub trend: Option<String>,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub sample_size: i64,
    pub details: Option<String>,
    pub alert_triggered: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum MonitoringMetricType {
    Accuracy,
    Latency,
    Throughput,
    ErrorRate,
    DataDrift,
    ConceptDrift,
    PredictionDistribution,
    FeatureDistribution,
    InputQuality,
    OutputQuality,
    ResourceUsage,
    BiasMetric,
    FairnessMetric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureStore {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub entity_type: String,
    pub features: String,
    pub data_source: String,
    pub refresh_schedule: Option<String>,
    pub last_refresh_at: Option<DateTime<Utc>>,
    pub feature_count: i32,
    pub row_count: i64,
    pub size_bytes: i64,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureDefinition {
    pub id: Uuid,
    pub store_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub data_type: String,
    pub transformation: Option<String>,
    pub source_field: Option<String>,
    pub default_value: Option<String>,
    pub is_nullable: bool,
    pub is_primary_key: bool,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AISettings {
    pub id: Uuid,
    pub setting_key: String,
    pub setting_value: String,
    pub description: Option<String>,
    pub category: String,
    pub is_encrypted: bool,
    pub updated_by: Option<Uuid>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MLExperiment {
    pub base: BaseEntity,
    pub model_id: Uuid,
    pub experiment_name: String,
    pub run_number: i32,
    pub parameters: String,
    pub metrics: Option<String>,
    pub artifacts_path: Option<String>,
    pub status: ModelStatus,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<i64>,
    pub notes: Option<String>,
    pub created_by: Option<Uuid>,
}
