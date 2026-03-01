use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/runs", post(create_run).get(list_runs))
        .route("/runs/:id", get(get_run))
        .route("/runs/:id/execute", post(execute_run))
        .route("/runs/:id/suggestions", get(list_suggestions))
        .route("/parameters", post(create_parameter).get(list_parameters))
        .route("/parameters/:id", get(get_parameter).put(update_parameter))
        .route("/forecasts", post(create_forecast).get(list_forecasts))
        .route("/forecasts/:id", get(get_forecast))
        .route("/forecasts/:id/lines", post(add_forecast_line).get(list_forecast_lines))
        .route("/planned-orders", post(create_planned_order).get(list_planned_orders))
        .route("/planned-orders/:id", get(get_planned_order))
        .route("/planned-orders/:id/firm", post(firm_planned_order))
        .route("/planned-orders/:id/convert", post(convert_planned_order))
        .route("/exceptions", get(list_exceptions))
        .route("/exceptions/:id/acknowledge", post(acknowledge_exception))
}

#[derive(Serialize)]
pub struct RunResponse {
    pub id: Uuid,
    pub run_number: String,
    pub name: String,
    pub status: String,
    pub total_items_planned: i32,
    pub total_suggestions: i32,
}

#[derive(Deserialize)]
pub struct CreateRunRequest {
    pub name: String,
    pub planning_horizon_days: i32,
    pub include_forecasts: bool,
    pub include_sales_orders: bool,
}

pub async fn create_run(State(_state): State<AppState>, Json(_req): Json<CreateRunRequest>) -> Json<RunResponse> {
    Json(RunResponse {
        id: Uuid::new_v4(),
        run_number: format!("MRP-{}", chrono::Utc::now().format("%Y%m%d%H%M%S")),
        name: "MRP Run".to_string(),
        status: "Draft".to_string(),
        total_items_planned: 0,
        total_suggestions: 0,
    })
}

pub async fn list_runs(State(_state): State<AppState>) -> Json<Vec<RunResponse>> { Json(vec![]) }

pub async fn get_run(State(_state): State<AppState>) -> Json<RunResponse> {
    Json(RunResponse {
        id: Uuid::new_v4(),
        run_number: "MRP-20240101000000".to_string(),
        name: "Weekly MRP".to_string(),
        status: "Completed".to_string(),
        total_items_planned: 150,
        total_suggestions: 25,
    })
}

#[derive(Serialize)]
pub struct ExecuteResponse { pub status: String, pub items_processed: i32 }
pub async fn execute_run(State(_state): State<AppState>) -> Json<ExecuteResponse> {
    Json(ExecuteResponse { status: "Completed".to_string(), items_processed: 150 })
}

#[derive(Serialize)]
pub struct SuggestionResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub action_type: String,
    pub quantity: i64,
    pub required_date: String,
    pub priority: i32,
    pub reason: String,
}
pub async fn list_suggestions(State(_state): State<AppState>) -> Json<Vec<SuggestionResponse>> { Json(vec![]) }

#[derive(Serialize)]
pub struct ParameterResponse {
    pub id: Uuid,
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub planning_method: String,
    pub lead_time_days: i32,
    pub safety_stock: i64,
}

#[derive(Deserialize)]
pub struct CreateParameterRequest {
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub planning_method: String,
    pub lead_time_days: i32,
    pub safety_stock: i64,
}

pub async fn create_parameter(State(_state): State<AppState>, Json(_req): Json<CreateParameterRequest>) -> Json<ParameterResponse> {
    Json(ParameterResponse {
        id: Uuid::new_v4(),
        product_id: Uuid::new_v4(),
        warehouse_id: Uuid::new_v4(),
        planning_method: "MRP".to_string(),
        lead_time_days: 14,
        safety_stock: 100,
    })
}

pub async fn list_parameters(State(_state): State<AppState>) -> Json<Vec<ParameterResponse>> { Json(vec![]) }
pub async fn get_parameter(State(_state): State<AppState>) -> Json<ParameterResponse> {
    Json(ParameterResponse {
        id: Uuid::new_v4(),
        product_id: Uuid::new_v4(),
        warehouse_id: Uuid::new_v4(),
        planning_method: "MRP".to_string(),
        lead_time_days: 14,
        safety_stock: 100,
    })
}
pub async fn update_parameter(State(_state): State<AppState>, Json(_req): Json<CreateParameterRequest>) -> Json<ParameterResponse> {
    Json(ParameterResponse {
        id: Uuid::new_v4(),
        product_id: Uuid::new_v4(),
        warehouse_id: Uuid::new_v4(),
        planning_method: "MRP".to_string(),
        lead_time_days: 14,
        safety_stock: 100,
    })
}

#[derive(Serialize)]
pub struct ForecastResponse { pub id: Uuid, pub name: String, pub status: String, pub period: String }

#[derive(Deserialize)]
pub struct CreateForecastRequest { pub name: String, pub start_date: String, pub end_date: String, pub method: String }

pub async fn create_forecast(State(_state): State<AppState>, Json(_req): Json<CreateForecastRequest>) -> Json<ForecastResponse> {
    Json(ForecastResponse { id: Uuid::new_v4(), name: "Q1 Forecast".to_string(), status: "Draft".to_string(), period: "2024-Q1".to_string() })
}

pub async fn list_forecasts(State(_state): State<AppState>) -> Json<Vec<ForecastResponse>> { Json(vec![]) }
pub async fn get_forecast(State(_state): State<AppState>) -> Json<ForecastResponse> {
    Json(ForecastResponse { id: Uuid::new_v4(), name: "Q1 Forecast".to_string(), status: "Active".to_string(), period: "2024-Q1".to_string() })
}

#[derive(Serialize)]
pub struct ForecastLineResponse { pub id: Uuid, pub product_id: Uuid, pub quantity: i64, pub period: String }
pub async fn add_forecast_line(State(_state): State<AppState>) -> Json<ForecastLineResponse> {
    Json(ForecastLineResponse { id: Uuid::new_v4(), product_id: Uuid::new_v4(), quantity: 1000, period: "2024-01".to_string() })
}
pub async fn list_forecast_lines(State(_state): State<AppState>) -> Json<Vec<ForecastLineResponse>> { Json(vec![]) }

#[derive(Serialize)]
pub struct PlannedOrderResponse {
    pub id: Uuid,
    pub order_number: String,
    pub product_id: Uuid,
    pub order_type: String,
    pub quantity: i64,
    pub due_date: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct CreatePlannedOrderRequest {
    pub product_id: Uuid,
    pub warehouse_id: Uuid,
    pub order_type: String,
    pub quantity: i64,
    pub due_date: String,
}

pub async fn create_planned_order(State(_state): State<AppState>, Json(_req): Json<CreatePlannedOrderRequest>) -> Json<PlannedOrderResponse> {
    let due_date = chrono::Utc::now() + chrono::Duration::days(14);
    Json(PlannedOrderResponse {
        id: Uuid::new_v4(),
        order_number: format!("PLN-{}", chrono::Utc::now().format("%Y%m%d%H%M%S")),
        product_id: Uuid::new_v4(),
        order_type: "Purchase".to_string(),
        quantity: 500,
        due_date: due_date.format("%Y-%m-%d").to_string(),
        status: "Open".to_string(),
    })
}

pub async fn list_planned_orders(State(_state): State<AppState>) -> Json<Vec<PlannedOrderResponse>> { Json(vec![]) }
pub async fn get_planned_order(State(_state): State<AppState>) -> Json<PlannedOrderResponse> {
    let due_date = chrono::Utc::now() + chrono::Duration::days(21);
    Json(PlannedOrderResponse {
        id: Uuid::new_v4(),
        order_number: "PLN-20240101000000".to_string(),
        product_id: Uuid::new_v4(),
        order_type: "Manufacture".to_string(),
        quantity: 200,
        due_date: due_date.format("%Y-%m-%d").to_string(),
        status: "Firmed".to_string(),
    })
}

pub async fn firm_planned_order(State(_state): State<AppState>) -> Json<PlannedOrderResponse> {
    let due_date = chrono::Utc::now() + chrono::Duration::days(14);
    Json(PlannedOrderResponse {
        id: Uuid::new_v4(),
        order_number: "PLN-20240101000000".to_string(),
        product_id: Uuid::new_v4(),
        order_type: "Purchase".to_string(),
        quantity: 500,
        due_date: due_date.format("%Y-%m-%d").to_string(),
        status: "Firmed".to_string(),
    })
}

#[derive(Serialize)]
pub struct ConvertResponse { pub planned_order_id: Uuid, pub converted_type: String, pub converted_id: Uuid }
pub async fn convert_planned_order(State(_state): State<AppState>) -> Json<ConvertResponse> {
    Json(ConvertResponse {
        planned_order_id: Uuid::new_v4(),
        converted_type: "PurchaseOrder".to_string(),
        converted_id: Uuid::new_v4(),
    })
}

#[derive(Serialize)]
pub struct ExceptionResponse { pub id: Uuid, pub exception_type: String, pub severity: String, pub message: String }
pub async fn list_exceptions(State(_state): State<AppState>) -> Json<Vec<ExceptionResponse>> { Json(vec![]) }
pub async fn acknowledge_exception(State(_state): State<AppState>) -> Json<ExceptionResponse> {
    Json(ExceptionResponse {
        id: Uuid::new_v4(),
        exception_type: "Shortage".to_string(),
        severity: "Warning".to_string(),
        message: "Shortage acknowledged".to_string(),
    })
}
