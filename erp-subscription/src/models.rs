use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BillingInterval {
    Daily,
    Weekly,
    Monthly,
    Quarterly,
    SemiAnnual,
    Annual,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SubscriptionStatus {
    Active,
    Paused,
    Cancelled,
    Expired,
    Pending,
    Trial,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionPlan {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    pub currency: String,
    pub billing_interval: BillingInterval,
    pub interval_count: i32,
    pub trial_days: i32,
    pub features: String,
    pub max_users: Option<i32>,
    pub max_transactions: Option<i64>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub plan_id: Uuid,
    pub status: SubscriptionStatus,
    pub quantity: i32,
    pub price_override: Option<i64>,
    pub current_period_start: DateTime<Utc>,
    pub current_period_end: DateTime<Utc>,
    pub trial_start: Option<DateTime<Utc>>,
    pub trial_end: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub cancel_at_period_end: bool,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionItem {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub product_id: Uuid,
    pub quantity: i32,
    pub unit_price: i64,
    pub discount_percent: f64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionInvoice {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub invoice_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub amount: i64,
    pub currency: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionUsage {
    pub id: Uuid,
    pub subscription_id: Uuid,
    pub usage_type: String,
    pub quantity: i64,
    pub unit: String,
    pub recorded_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeteredPrice {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub meter_type: String,
    pub unit_price: i64,
    pub included_units: i64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePlanRequest {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub price: i64,
    pub currency: String,
    pub billing_interval: BillingInterval,
    pub interval_count: i32,
    pub trial_days: i32,
    pub features: String,
    pub max_users: Option<i32>,
    pub max_transactions: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    pub customer_id: Uuid,
    pub plan_id: Uuid,
    pub quantity: i32,
    pub price_override: Option<i64>,
    pub trial_days: Option<i32>,
    pub metadata: Option<String>,
}
