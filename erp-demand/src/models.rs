use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandForecast {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub forecast_date: NaiveDate,
    pub period_type: PeriodType,
    pub forecast_qty: f64,
    pub lower_bound: f64,
    pub upper_bound: f64,
    pub confidence: f64,
    pub method: ForecastMethod,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PeriodType {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForecastMethod {
    MovingAverage,
    ExponentialSmoothing,
    HoltWinters,
    ARIMA,
    SeasonalNaive,
    MachineLearning,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastModel {
    pub id: Uuid,
    pub name: String,
    pub method: ForecastMethod,
    pub parameters: serde_json::Value,
    pub accuracy_mape: Option<f64>,
    pub accuracy_mse: Option<f64>,
    pub last_trained: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandPlan {
    pub id: Uuid,
    pub plan_name: String,
    pub plan_type: PlanType,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: PlanStatus,
    pub version: i32,
    pub baseline_id: Option<Uuid>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlanType {
    Sales,
    Production,
    Inventory,
    SAndOP,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlanStatus {
    Draft,
    Submitted,
    Approved,
    Active,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlanLine {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub product_id: Uuid,
    pub location_id: Option<Uuid>,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub forecast_qty: f64,
    pub adjusted_qty: f64,
    pub final_qty: f64,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyStock {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub safety_qty: i64,
    pub reorder_point: i64,
    pub service_level: f64,
    pub lead_time_days: i32,
    pub demand_variability: f64,
    pub last_calculated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandHistory {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub date: NaiveDate,
    pub actual_qty: i64,
    pub forecast_qty: Option<f64>,
    pub variance: Option<f64>,
    pub source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionImpact {
    pub id: Uuid,
    pub promotion_id: Uuid,
    pub product_id: Uuid,
    pub lift_factor: f64,
    pub cannibalization: Vec<Uuid>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DemandSensingSignal {
    pub id: Uuid,
    pub signal_type: SignalType,
    pub source: String,
    pub value: f64,
    pub weight: f64,
    pub timestamp: DateTime<Utc>,
    pub product_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SignalType {
    POSData,
    WebTraffic,
    SocialMedia,
    Weather,
    Economic,
    Competitor,
    MarketTrend,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateForecastRequest {
    pub product_id: Uuid,
    pub warehouse_id: Option<Uuid>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub period_type: PeriodType,
    pub method: ForecastMethod,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDemandPlanRequest {
    pub plan_name: String,
    pub plan_type: PlanType,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub product_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculateSafetyStockRequest {
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub service_level: f64,
    pub lead_time_days: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastAccuracy {
    pub product_id: Uuid,
    pub period: String,
    pub mape: f64,
    pub mad: f64,
    pub mse: f64,
    pub bias: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunForecastRequest {
    pub model_id: Option<Uuid>,
    pub product_ids: Option<Vec<Uuid>>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub retrain: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastResult {
    pub model_id: Uuid,
    pub forecasts: Vec<DemandForecast>,
    pub accuracy_metrics: ForecastAccuracyMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForecastAccuracyMetrics {
    pub mape: f64,
    pub mad: f64,
    pub mse: f64,
    pub rmse: f64,
}
