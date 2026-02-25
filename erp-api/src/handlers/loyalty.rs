use axum::{extract::State, routing::{get, post}, Json, Router};
use serde::Serialize;
use uuid::Uuid;
use crate::db::AppState;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/programs", post(create_program).get(list_programs))
        .route("/programs/:id", get(get_program))
        .route("/members", post(enroll_member).get(list_members))
        .route("/members/:id", get(get_member))
        .route("/members/:id/earn", post(earn_points))
        .route("/members/:id/redeem", post(redeem_points))
        .route("/rewards", post(create_reward).get(list_rewards))
        .route("/promotions", post(create_promotion).get(list_promotions))
}

#[derive(Serialize)]
pub struct ProgramResponse { pub id: Uuid, pub name: String, pub status: String }
pub async fn create_program(State(_state): State<AppState>) -> Json<ProgramResponse> {
    Json(ProgramResponse { id: Uuid::new_v4(), name: "Program".to_string(), status: "Active".to_string() })
}
pub async fn list_programs(State(_state): State<AppState>) -> Json<Vec<ProgramResponse>> { Json(vec![]) }
pub async fn get_program(State(_state): State<AppState>) -> Json<ProgramResponse> {
    Json(ProgramResponse { id: Uuid::new_v4(), name: "Program".to_string(), status: "Active".to_string() })
}

#[derive(Serialize)]
pub struct MemberResponse { pub id: Uuid, pub member_number: String, pub points: i64 }
pub async fn enroll_member(State(_state): State<AppState>) -> Json<MemberResponse> {
    Json(MemberResponse { id: Uuid::new_v4(), member_number: "MEM-001".to_string(), points: 0 })
}
pub async fn list_members(State(_state): State<AppState>) -> Json<Vec<MemberResponse>> { Json(vec![]) }
pub async fn get_member(State(_state): State<AppState>) -> Json<MemberResponse> {
    Json(MemberResponse { id: Uuid::new_v4(), member_number: "MEM-001".to_string(), points: 100 })
}
pub async fn earn_points(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Points earned"}))
}
pub async fn redeem_points(State(_state): State<AppState>) -> Json<serde_json::Value> {
    Json(serde_json::json!({"message": "Points redeemed"}))
}

#[derive(Serialize)]
pub struct RewardResponse { pub id: Uuid, pub name: String, pub points_cost: i64 }
pub async fn create_reward(State(_state): State<AppState>) -> Json<RewardResponse> {
    Json(RewardResponse { id: Uuid::new_v4(), name: "Reward".to_string(), points_cost: 100 })
}
pub async fn list_rewards(State(_state): State<AppState>) -> Json<Vec<RewardResponse>> { Json(vec![]) }

#[derive(Serialize)]
pub struct PromotionResponse { pub id: Uuid, pub name: String }
pub async fn create_promotion(State(_state): State<AppState>) -> Json<PromotionResponse> {
    Json(PromotionResponse { id: Uuid::new_v4(), name: "Promotion".to_string() })
}
pub async fn list_promotions(State(_state): State<AppState>) -> Json<Vec<PromotionResponse>> { Json(vec![]) }
