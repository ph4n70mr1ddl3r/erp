use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PriceRuleType {
    BasePrice,
    Markup,
    Margin,
    Discount,
    Surcharge,
    Bundle,
    Tiered,
    Volume,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PriceRuleScope {
    Global,
    Customer,
    CustomerGroup,
    Product,
    ProductCategory,
    Warehouse,
    Region,
    Channel,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DiscountType {
    Percentage,
    FixedAmount,
    BuyXGetY,
    FreeShipping,
    FreeItem,
    Tiered,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PromotionStatus {
    Draft,
    Scheduled,
    Active,
    Paused,
    Expired,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceBook {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub currency: String,
    pub is_default: bool,
    pub is_active: bool,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_to: Option<DateTime<Utc>>,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceBookEntry {
    pub base: BaseEntity,
    pub price_book_id: Uuid,
    pub product_id: Uuid,
    pub unit_price: i64,
    pub currency: String,
    pub min_quantity: i32,
    pub max_quantity: Option<i32>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_to: Option<DateTime<Utc>>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceRule {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub rule_type: PriceRuleType,
    pub scope: PriceRuleScope,
    pub priority: i32,
    pub value: f64,
    pub currency: Option<String>,
    pub conditions: Option<String>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_to: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub is_stackable: bool,
    pub max_applications: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceRuleAssignment {
    pub base: BaseEntity,
    pub rule_id: Uuid,
    pub entity_type: String,
    pub entity_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discount {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub discount_type: DiscountType,
    pub value: f64,
    pub max_discount: Option<i64>,
    pub min_order_value: Option<i64>,
    pub applicable_to: Option<String>,
    pub customer_groups: Option<String>,
    pub products: Option<String>,
    pub categories: Option<String>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_to: Option<DateTime<Utc>>,
    pub usage_limit: Option<i32>,
    pub usage_per_customer: Option<i32>,
    pub current_usage: i32,
    pub is_active: bool,
    pub requires_code: bool,
    pub auto_apply: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountUsage {
    pub base: BaseEntity,
    pub discount_id: Uuid,
    pub order_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub discount_amount: i64,
    pub applied_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Promotion {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub promotion_type: PromotionType,
    pub status: PromotionStatus,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub rules: String,
    pub rewards: String,
    pub target_segments: Option<String>,
    pub channels: Option<String>,
    pub budget: Option<i64>,
    pub spent: i64,
    pub usage_limit: Option<i32>,
    pub current_usage: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PromotionType {
    ProductDiscount,
    OrderDiscount,
    FreeGift,
    BundleOffer,
    LoyaltyBonus,
    ReferralBonus,
    Flash,
    Seasonal,
    Clearance,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coupon {
    pub base: BaseEntity,
    pub code: String,
    pub discount_id: Uuid,
    pub promotion_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub is_used: bool,
    pub used_at: Option<DateTime<Utc>>,
    pub order_id: Option<Uuid>,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_to: Option<DateTime<Utc>>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceTier {
    pub base: BaseEntity,
    pub price_book_entry_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub min_quantity: i32,
    pub max_quantity: Option<i32>,
    pub unit_price: i64,
    pub discount_percent: Option<f64>,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerPriceGroup {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub price_book_id: Option<Uuid>,
    pub discount_id: Option<Uuid>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerPriceGroupMember {
    pub base: BaseEntity,
    pub group_id: Uuid,
    pub customer_id: Uuid,
    pub joined_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceList {
    pub base: BaseEntity,
    pub name: String,
    pub currency: String,
    pub is_default: bool,
    pub valid_from: Option<DateTime<Utc>>,
    pub valid_to: Option<DateTime<Utc>>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceListItem {
    pub base: BaseEntity,
    pub price_list_id: Uuid,
    pub product_id: Uuid,
    pub unit_price: i64,
    pub min_quantity: i32,
    pub currency: String,
}
