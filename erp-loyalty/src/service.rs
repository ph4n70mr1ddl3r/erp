use crate::models::*;
use crate::repository::{LoyaltyRepository, SqliteLoyaltyRepository};
use chrono::{NaiveDate, Utc};
use erp_core::{BaseEntity, Result};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct LoyaltyService { repo: SqliteLoyaltyRepository }

impl LoyaltyService {
    pub fn new(pool: SqlitePool) -> Self { Self { repo: SqliteLoyaltyRepository::new(pool) } }

    pub async fn create_program(&self, pool: &SqlitePool, req: CreateProgramRequest) -> Result<LoyaltyProgram> {
        let program = LoyaltyProgram {
            base: BaseEntity::new(),
            program_number: format!("LP-{}", Uuid::new_v4()),
            name: req.name,
            description: req.description,
            program_type: req.program_type,
            points_name: req.points_name.unwrap_or_else(|| "Points".to_string()),
            earn_rate: req.earn_rate.unwrap_or(1.0),
            earn_currency: req.earn_currency.unwrap_or_else(|| "USD".to_string()),
            minimum_points_redemption: req.minimum_points_redemption.unwrap_or(100),
            points_expiry_months: req.points_expiry_months,
            enrollment_bonus: req.enrollment_bonus.unwrap_or(0),
            referral_bonus: req.referral_bonus.unwrap_or(0),
            birthday_bonus: req.birthday_bonus.unwrap_or(0),
            max_points_per_day: req.max_points_per_day,
            max_points_per_year: req.max_points_per_year,
            status: erp_core::Status::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_program(&program).await
    }

    pub async fn enroll_member(&self, pool: &SqlitePool, req: EnrollMemberRequest) -> Result<LoyaltyMember> {
        let member = LoyaltyMember {
            base: BaseEntity::new(),
            member_number: format!("MEM-{}", Uuid::new_v4()),
            program_id: req.program_id,
            customer_id: req.customer_id,
            tier_id: None,
            enrollment_date: Utc::now().date_naive(),
            enrollment_source: req.enrollment_source,
            total_points_earned: 0,
            total_points_redeemed: 0,
            available_points: 0,
            lifetime_points: 0,
            points_expiring: 0,
            points_expiring_date: None,
            total_rewards_redeemed: 0,
            tier_upgrade_date: None,
            tier_downgrade_date: None,
            referral_code: generate_referral_code(),
            referred_by: req.referred_by,
            referral_count: 0,
            status: MemberStatus::Active,
            last_activity_date: Some(Utc::now().date_naive()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_member(&member).await
    }

    pub async fn earn_points(&self, pool: &SqlitePool, member_id: Uuid, points: i64, source_type: String, source_id: Option<Uuid>, amount: Option<i64>) -> Result<LoyaltyTransaction> {
        let tx = LoyaltyTransaction {
            base: BaseEntity::new(),
            transaction_number: format!("TXN-{}", Uuid::new_v4()),
            member_id,
            transaction_type: LoyaltyTransactionType::Earn,
            point_type: PointType::Base,
            points,
            running_balance: 0,
            source_type: Some(source_type),
            source_id,
            order_id: None,
            amount,
            currency: Some("USD".to_string()),
            description: Some("Points earned".to_string()),
            expiry_date: None,
            created_at: Utc::now(),
        };
        self.repo.create_transaction(&tx).await
    }

    pub async fn redeem_points(&self, pool: &SqlitePool, member_id: Uuid, reward_id: Uuid, points: i64) -> Result<LoyaltyRedemption> {
        let redemption = LoyaltyRedemption {
            base: BaseEntity::new(),
            redemption_number: format!("RED-{}", Uuid::new_v4()),
            member_id,
            reward_id,
            points_used: points,
            cash_value: 0,
            currency: "USD".to_string(),
            order_id: None,
            redemption_date: Utc::now().date_naive(),
            status: RedemptionStatus::Pending,
            voucher_code: Some(generate_voucher_code()),
            voucher_expires: Some(Utc::now().date_naive() + chrono::Duration::days(30)),
            used_at: None,
            created_at: Utc::now(),
        };
        self.repo.create_redemption(&redemption).await
    }

    pub async fn create_reward(&self, pool: &SqlitePool, req: CreateRewardRequest) -> Result<LoyaltyReward> {
        let reward = LoyaltyReward {
            base: BaseEntity::new(),
            reward_number: format!("RWD-{}", Uuid::new_v4()),
            program_id: req.program_id,
            name: req.name,
            description: req.description,
            reward_type: req.reward_type,
            points_cost: req.points_cost,
            cash_value: req.cash_value,
            currency: req.currency,
            product_id: req.product_id,
            discount_percent: req.discount_percent,
            discount_amount: req.discount_amount,
            min_purchase_amount: req.min_purchase_amount,
            max_redemptions: req.max_redemptions,
            redemptions_count: 0,
            valid_from: req.valid_from,
            valid_until: req.valid_until,
            tier_requirement: req.tier_requirement,
            status: erp_core::Status::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_reward(&reward).await
    }
}

fn generate_referral_code() -> String {
    format!("REF{}", &Uuid::new_v4().to_string()[..8].to_uppercase())
}

fn generate_voucher_code() -> String {
    format!("VCH{}", &Uuid::new_v4().to_string()[..8].to_uppercase())
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateProgramRequest {
    pub name: String,
    pub description: Option<String>,
    pub program_type: ProgramType,
    pub points_name: Option<String>,
    pub earn_rate: Option<f64>,
    pub earn_currency: Option<String>,
    pub minimum_points_redemption: Option<i64>,
    pub points_expiry_months: Option<i32>,
    pub enrollment_bonus: Option<i64>,
    pub referral_bonus: Option<i64>,
    pub birthday_bonus: Option<i64>,
    pub max_points_per_day: Option<i64>,
    pub max_points_per_year: Option<i64>,
}

#[derive(Debug, serde::Deserialize)]
pub struct EnrollMemberRequest {
    pub program_id: Uuid,
    pub customer_id: Uuid,
    pub enrollment_source: Option<String>,
    pub referred_by: Option<Uuid>,
}

#[derive(Debug, serde::Deserialize)]
pub struct CreateRewardRequest {
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
    pub valid_from: Option<NaiveDate>,
    pub valid_until: Option<NaiveDate>,
    pub tier_requirement: Option<Uuid>,
}
