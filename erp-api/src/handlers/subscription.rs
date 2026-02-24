use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::db::AppState;
use erp_subscription::{SubscriptionPlanService, SubscriptionService, CreatePlanRequest, CreateSubscriptionRequest, BillingInterval};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/plans", get(list_plans).post(create_plan))
        .route("/subscriptions", post(create_subscription))
        .route("/subscriptions/:id", get(get_subscription))
        .route("/subscriptions/customer/:customer_id", get(list_customer_subscriptions))
        .route("/subscriptions/:id/cancel", post(cancel_subscription))
        .route("/subscriptions/:id/renew", post(renew_subscription))
        .route("/subscriptions/expiring/:days", get(get_expiring))
        .route("/subscriptions/:id/usage", post(record_usage))
}

#[derive(Deserialize)]
pub struct CreatePlanBody {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    #[serde(default = "default_currency")]
    pub currency: String,
    #[serde(default)]
    pub billing_interval: String,
    #[serde(default = "default_interval_count")]
    pub interval_count: i32,
    #[serde(default)]
    pub trial_days: i32,
    #[serde(default)]
    pub features: String,
    pub max_users: Option<i32>,
    pub max_transactions: Option<i64>,
}

fn default_currency() -> String { "USD".to_string() }
fn default_interval_count() -> i32 { 1 }

async fn create_plan(
    State(state): State<AppState>,
    Json(body): Json<CreatePlanBody>,
) -> Json<serde_json::Value> {
    let req = CreatePlanRequest {
        code: body.code,
        name: body.name,
        description: body.description,
        price: body.price,
        currency: body.currency,
        billing_interval: match body.billing_interval.as_str() {
            "Daily" => BillingInterval::Daily,
            "Weekly" => BillingInterval::Weekly,
            "Quarterly" => BillingInterval::Quarterly,
            "SemiAnnual" => BillingInterval::SemiAnnual,
            "Annual" => BillingInterval::Annual,
            "Custom" => BillingInterval::Custom,
            _ => BillingInterval::Monthly,
        },
        interval_count: body.interval_count,
        trial_days: body.trial_days,
        features: body.features,
        max_users: body.max_users,
        max_transactions: body.max_transactions,
    };
    match SubscriptionPlanService::create(&state.pool, req).await {
        Ok(plan) => Json(json!({
            "id": plan.id,
            "code": plan.code,
            "name": plan.name,
            "price": plan.price,
            "currency": plan.currency,
            "billing_interval": plan.billing_interval
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn list_plans(State(state): State<AppState>) -> Json<serde_json::Value> {
    match SubscriptionPlanService::list_active(&state.pool).await {
        Ok(plans) => Json(json!({
            "items": plans.iter().map(|p| json!({
                "id": p.id,
                "code": p.code,
                "name": p.name,
                "price": p.price,
                "currency": p.currency,
                "billing_interval": p.billing_interval
            })).collect::<Vec<_>>()
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct CreateSubscriptionBody {
    pub customer_id: Uuid,
    pub plan_id: Uuid,
    #[serde(default = "default_quantity")]
    pub quantity: i32,
    pub price_override: Option<i64>,
    pub trial_days: Option<i32>,
    pub metadata: Option<String>,
}

fn default_quantity() -> i32 { 1 }

async fn create_subscription(
    State(state): State<AppState>,
    Json(body): Json<CreateSubscriptionBody>,
) -> Json<serde_json::Value> {
    let req = CreateSubscriptionRequest {
        customer_id: body.customer_id,
        plan_id: body.plan_id,
        quantity: body.quantity,
        price_override: body.price_override,
        trial_days: body.trial_days,
        metadata: body.metadata,
    };
    match SubscriptionService::create(&state.pool, req).await {
        Ok(sub) => Json(json!({
            "id": sub.id,
            "customer_id": sub.customer_id,
            "plan_id": sub.plan_id,
            "status": sub.status,
            "current_period_start": sub.current_period_start,
            "current_period_end": sub.current_period_end
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn get_subscription(State(state): State<AppState>, Path(id): Path<Uuid>) -> Json<serde_json::Value> {
    Json(json!({ "error": "Use customer subscriptions endpoint" }))
}

async fn list_customer_subscriptions(
    State(state): State<AppState>,
    Path(customer_id): Path<Uuid>,
) -> Json<serde_json::Value> {
    match SubscriptionService::list_by_customer(&state.pool, customer_id).await {
        Ok(subs) => Json(json!({
            "items": subs.iter().map(|s| json!({
                "id": s.id,
                "plan_id": s.plan_id,
                "status": s.status,
                "current_period_end": s.current_period_end
            })).collect::<Vec<_>>()
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct CancelSubscriptionBody {
    #[serde(default)]
    pub at_period_end: bool,
}

async fn cancel_subscription(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<CancelSubscriptionBody>,
) -> Json<serde_json::Value> {
    match SubscriptionService::cancel(&state.pool, id, body.at_period_end).await {
        Ok(sub) => Json(json!({
            "id": sub.id,
            "status": sub.status,
            "cancel_at_period_end": sub.cancel_at_period_end
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn renew_subscription(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    match SubscriptionService::renew(&state.pool, id).await {
        Ok(sub) => Json(json!({
            "id": sub.id,
            "status": sub.status,
            "current_period_end": sub.current_period_end
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn get_expiring(
    State(state): State<AppState>,
    Path(days): Path<i32>,
) -> Json<serde_json::Value> {
    match SubscriptionService::get_expiring(&state.pool, days).await {
        Ok(subs) => Json(json!({
            "items": subs.iter().map(|s| json!({
                "id": s.id,
                "customer_id": s.customer_id,
                "current_period_end": s.current_period_end
            })).collect::<Vec<_>>()
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct RecordUsageBody {
    pub usage_type: String,
    pub quantity: i64,
    pub unit: String,
}

async fn record_usage(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(body): Json<RecordUsageBody>,
) -> Json<serde_json::Value> {
    match SubscriptionService::record_usage(&state.pool, id, body.usage_type, body.quantity, body.unit).await {
        Ok(usage) => Json(json!({
            "id": usage.id,
            "usage_type": usage.usage_type,
            "quantity": usage.quantity
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}
