use axum::{
    extract::{Path, State},
    routing::{get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use crate::db::AppState;

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePlanRequest {
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub plan_year: i32,
    pub effective_date: String,
    pub budget_amount: i64,
    pub review_cycle: String,
    pub default_merit_budget_percent: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SetCompensationRequest {
    pub employee_id: Uuid,
    pub effective_date: String,
    pub base_salary: i64,
    pub currency: String,
    pub grade_id: Option<Uuid>,
    pub salary_range_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAdjustmentRequest {
    pub employee_id: Uuid,
    pub plan_id: Option<Uuid>,
    pub adjustment_type: String,
    pub current_base: i64,
    pub new_base: i64,
    pub effective_date: String,
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReviewRequest {
    pub plan_id: Uuid,
    pub employee_id: Uuid,
    pub reviewer_id: Uuid,
    pub current_salary: i64,
    pub proposed_salary: i64,
    pub performance_rating: Option<i32>,
}

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/plans", get(list_plans).post(create_plan))
        .route("/plans/:id", get(get_plan))
        .route("/compensations", post(set_compensation))
        .route("/compensations/:employee_id", get(get_compensation))
        .route("/adjustments", get(list_adjustments).post(create_adjustment))
        .route("/adjustments/:id/approve", post(approve_adjustment))
        .route("/reviews", get(list_reviews).post(create_review))
        .route("/reviews/:id", get(get_review))
        .route("/bonuses/:employee_id", post(calculate_bonus))
        .route("/statements/:employee_id", get(get_total_rewards))
        .route("/pay-equity", post(analyze_pay_equity))
        .route("/benchmarks/:position_id", get(get_benchmark))
        .route("/grades", get(list_grades))
        .route("/ranges", get(list_salary_ranges))
}

async fn list_plans(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "plans": [], "total": 0 }))
}

async fn create_plan(
    State(_state): State<AppState>,
    Json(_req): Json<CreatePlanRequest>,
) -> Json<serde_json::Value> {
    Json(json!({ "id": Uuid::new_v4().to_string(), "message": "Plan created" }))
}

async fn get_plan(Path(_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "plan": null }))
}

async fn set_compensation(
    State(_state): State<AppState>,
    Json(_req): Json<SetCompensationRequest>,
) -> Json<serde_json::Value> {
    Json(json!({ "id": Uuid::new_v4().to_string(), "message": "Compensation set" }))
}

async fn get_compensation(Path(_employee_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "compensation": null }))
}

async fn list_adjustments(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "adjustments": [], "total": 0 }))
}

async fn create_adjustment(
    State(_state): State<AppState>,
    Json(_req): Json<CreateAdjustmentRequest>,
) -> Json<serde_json::Value> {
    Json(json!({ "id": Uuid::new_v4().to_string(), "message": "Adjustment created" }))
}

async fn approve_adjustment(Path(_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "message": "Adjustment approved" }))
}

async fn list_reviews(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "reviews": [], "total": 0 }))
}

async fn create_review(
    State(_state): State<AppState>,
    Json(_req): Json<CreateReviewRequest>,
) -> Json<serde_json::Value> {
    Json(json!({ "id": Uuid::new_v4().to_string(), "message": "Review created" }))
}

async fn get_review(Path(_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "review": null }))
}

async fn calculate_bonus(Path(_employee_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "bonus": null }))
}

async fn get_total_rewards(Path(_employee_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "statement": null }))
}

async fn analyze_pay_equity(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "analysis": null }))
}

async fn get_benchmark(Path(_position_id): Path<Uuid>, State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "benchmark": null }))
}

async fn list_grades(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "grades": [] }))
}

async fn list_salary_ranges(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(json!({ "ranges": [] }))
}
