use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Promotion {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub promotion_type: PromotionType,
    pub discount_type: DiscountType,
    pub discount_value: i64,
    pub max_discount: Option<i64>,
    pub min_order_amount: Option<i64>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub usage_limit: Option<i32>,
    pub usage_count: i32,
    pub per_customer_limit: Option<i32>,
    pub applies_to: PromotionAppliesTo,
    pub stackable: bool,
    pub auto_apply: bool,
    pub priority: i32,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PromotionType {
    OrderDiscount,
    LineItemDiscount,
    BuyXGetY,
    FreeShipping,
    BundleDiscount,
    LoyaltyPoints,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DiscountType {
    Percentage,
    FixedAmount,
    FixedPrice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionAppliesTo {
    pub product_ids: Vec<Uuid>,
    pub category_ids: Vec<Uuid>,
    pub customer_group_ids: Vec<Uuid>,
    pub exclude_product_ids: Vec<Uuid>,
    pub exclude_category_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coupon {
    pub base: BaseEntity,
    pub code: String,
    pub promotion_id: Uuid,
    pub coupon_type: CouponType,
    pub discount_type: DiscountType,
    pub discount_value: i64,
    pub max_discount: Option<i64>,
    pub min_order_amount: Option<i64>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub usage_limit: Option<i32>,
    pub usage_count: i32,
    pub per_customer_limit: Option<i32>,
    pub customer_email: Option<String>,
    pub first_time_only: bool,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CouponType {
    SingleUse,
    MultiUse,
    Unlimited,
    Referral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponBatch {
    pub id: Uuid,
    pub promotion_id: Uuid,
    pub prefix: String,
    pub quantity: i32,
    pub length: i32,
    pub created_count: i32,
    pub used_count: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionRule {
    pub id: Uuid,
    pub promotion_id: Uuid,
    pub rule_type: RuleType,
    pub condition_type: ConditionType,
    pub condition_field: String,
    pub operator: ConditionOperator,
    pub value: String,
    pub priority: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RuleType {
    Eligibility,
    Trigger,
    Benefit,
    Exclusion,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ConditionType {
    Customer,
    Product,
    Order,
    Cart,
    Time,
    Location,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ConditionOperator {
    Equals,
    NotEquals,
    GreaterThan,
    GreaterThanOrEqual,
    LessThan,
    LessThanOrEqual,
    Contains,
    NotContains,
    In,
    NotIn,
    Between,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionUsage {
    pub id: Uuid,
    pub promotion_id: Option<Uuid>,
    pub coupon_id: Option<Uuid>,
    pub order_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub customer_email: Option<String>,
    pub discount_amount: i64,
    pub original_amount: i64,
    pub final_amount: i64,
    pub used_at: DateTime<Utc>,
    pub status: PromotionUsageStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PromotionUsageStatus {
    Applied,
    Reverted,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuyXGetYRule {
    pub id: Uuid,
    pub promotion_id: Uuid,
    pub buy_quantity: i32,
    pub get_quantity: i32,
    pub buy_product_ids: Vec<Uuid>,
    pub get_product_ids: Vec<Uuid>,
    pub discount_percent: i32,
    pub max_free_items: Option<i32>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromotionReport {
    pub promotion_id: Uuid,
    pub promotion_code: String,
    pub promotion_name: String,
    pub total_usage: i32,
    pub total_discount: i64,
    pub total_revenue: i64,
    pub unique_customers: i32,
    pub avg_order_value: i64,
    pub conversion_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CouponValidation {
    pub valid: bool,
    pub coupon: Option<Coupon>,
    pub promotion: Option<Promotion>,
    pub error_message: Option<String>,
    pub discount_preview: Option<DiscountPreview>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountPreview {
    pub original_amount: i64,
    pub discount_amount: i64,
    pub final_amount: i64,
    pub applied_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReferralProgram {
    pub id: Uuid,
    pub name: String,
    pub referrer_reward_type: DiscountType,
    pub referrer_reward_value: i64,
    pub referee_reward_type: DiscountType,
    pub referee_reward_value: i64,
    pub min_referee_purchase: i64,
    pub max_referrals_per_user: Option<i32>,
    pub total_referrals: i32,
    pub total_rewards_given: i64,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Referral {
    pub id: Uuid,
    pub program_id: Uuid,
    pub referrer_customer_id: Uuid,
    pub referrer_email: String,
    pub referee_email: String,
    pub referral_code: String,
    pub coupon_id: Option<Uuid>,
    pub status: ReferralStatus,
    pub referred_at: DateTime<Utc>,
    pub converted_at: Option<DateTime<Utc>>,
    pub reward_given_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReferralStatus {
    Pending,
    Registered,
    Converted,
    Rewarded,
    Expired,
}
