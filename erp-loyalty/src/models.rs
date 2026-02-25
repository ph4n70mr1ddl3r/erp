use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TierType {
    Bronze,
    Silver,
    Gold,
    Platinum,
    Diamond,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PointType {
    Base,
    Bonus,
    Promotional,
    Expiring,
    Lifetime,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RewardType {
    Discount,
    FreeProduct,
    Cashback,
    GiftCard,
    Experience,
    Upgrade,
    PartnerOffer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgram {
    pub base: BaseEntity,
    pub program_number: String,
    pub name: String,
    pub description: Option<String>,
    pub program_type: ProgramType,
    pub points_name: String,
    pub earn_rate: f64,
    earn_currency: String,
    pub minimum_points_redemption: i64,
    pub points_expiry_months: Option<i32>,
    pub enrollment_bonus: i64,
    pub referral_bonus: i64,
    pub birthday_bonus: i64,
    pub max_points_per_day: Option<i64>,
    pub max_points_per_year: Option<i64>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ProgramType {
    Points,
    Tiered,
    Cashback,
    Hybrid,
    Subscription,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyTier {
    pub base: BaseEntity,
    pub program_id: Uuid,
    pub tier_type: TierType,
    pub name: String,
    pub description: Option<String>,
    pub minimum_points: i64,
    pub maximum_points: Option<i64>,
    pub earn_multiplier: f64,
    pub benefits: Option<String>,
    pub upgrade_bonus: i64,
    pub color_code: Option<String>,
    pub priority: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyMember {
    pub base: BaseEntity,
    pub member_number: String,
    pub program_id: Uuid,
    pub customer_id: Uuid,
    pub tier_id: Option<Uuid>,
    pub enrollment_date: NaiveDate,
    pub enrollment_source: Option<String>,
    pub total_points_earned: i64,
    pub total_points_redeemed: i64,
    pub available_points: i64,
    pub lifetime_points: i64,
    pub points_expiring: i64,
    pub points_expiring_date: Option<NaiveDate>,
    pub total_rewards_redeemed: i64,
    pub tier_upgrade_date: Option<NaiveDate>,
    pub tier_downgrade_date: Option<NaiveDate>,
    pub referral_code: String,
    pub referred_by: Option<Uuid>,
    pub referral_count: i32,
    pub status: MemberStatus,
    pub last_activity_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MemberStatus {
    Active,
    Inactive,
    Suspended,
    OptedOut,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyTransaction {
    pub base: BaseEntity,
    pub transaction_number: String,
    pub member_id: Uuid,
    pub transaction_type: LoyaltyTransactionType,
    pub point_type: PointType,
    pub points: i64,
    pub running_balance: i64,
    pub source_type: Option<String>,
    pub source_id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub amount: Option<i64>,
    pub currency: Option<String>,
    pub description: Option<String>,
    pub expiry_date: Option<NaiveDate>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LoyaltyTransactionType {
    Earn,
    Redeem,
    Bonus,
    Refund,
    Expiry,
    Adjustment,
    Transfer,
    Purchase,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyReward {
    pub base: BaseEntity,
    pub reward_number: String,
    pub program_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub reward_type: RewardType,
    pub points_cost: i64,
    pub cash_value: Option<i64>,
    pub currency: String,
    pub product_id: Option<Uuid>,
    pub discount_percent: Option<f64>,
    pub discount_amount: Option<i64>,
    pub min_purchase_amount: Option<i64>,
    pub max_redemptions: Option<i32>,
    pub redemptions_count: i32,
    pub valid_from: Option<NaiveDate>,
    pub valid_until: Option<NaiveDate>,
    pub tier_requirement: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyRedemption {
    pub base: BaseEntity,
    pub redemption_number: String,
    pub member_id: Uuid,
    pub reward_id: Uuid,
    pub points_used: i64,
    pub cash_value: i64,
    pub currency: String,
    pub order_id: Option<Uuid>,
    pub redemption_date: NaiveDate,
    pub status: RedemptionStatus,
    pub voucher_code: Option<String>,
    pub voucher_expires: Option<NaiveDate>,
    pub used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RedemptionStatus {
    Pending,
    Confirmed,
    Used,
    Expired,
    Cancelled,
    Refunded,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyPromotion {
    pub base: BaseEntity,
    pub promotion_number: String,
    pub program_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub promotion_type: PromotionType,
    pub earn_multiplier: f64,
    pub bonus_points: i64,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub product_ids: Option<String>,
    pub category_ids: Option<String>,
    pub tier_restriction: Option<String>,
    pub max_points_per_member: Option<i64>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PromotionType {
    PointMultiplier,
    BonusPoints,
    DoublePoints,
    CategoryBonus,
    ProductBonus,
    Seasonal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyPartner {
    pub base: BaseEntity,
    pub partner_number: String,
    pub program_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub logo_url: Option<String>,
    pub website_url: Option<String>,
    pub earn_rate: f64,
    pub redemption_rate: f64,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyPointsTransfer {
    pub base: BaseEntity,
    pub transfer_number: String,
    pub program_id: Uuid,
    pub from_member_id: Uuid,
    pub to_member_id: Uuid,
    pub points: i64,
    pub fee_points: i64,
    pub status: TransferStatus,
    pub transfer_date: NaiveDate,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TransferStatus {
    Pending,
    Completed,
    Failed,
    Cancelled,
}
