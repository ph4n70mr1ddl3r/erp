use crate::models::*;
use async_trait::async_trait;
use erp_core::Result;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait LoyaltyRepository: Send + Sync {
    async fn create_program(&self, program: &LoyaltyProgram) -> Result<LoyaltyProgram>;
    async fn get_program(&self, id: Uuid) -> Result<Option<LoyaltyProgram>>;
    async fn list_programs(&self) -> Result<Vec<LoyaltyProgram>>;
    async fn create_tier(&self, tier: &LoyaltyTier) -> Result<LoyaltyTier>;
    async fn list_tiers(&self, program_id: Uuid) -> Result<Vec<LoyaltyTier>>;
    async fn create_member(&self, member: &LoyaltyMember) -> Result<LoyaltyMember>;
    async fn get_member(&self, id: Uuid) -> Result<Option<LoyaltyMember>>;
    async fn get_member_by_customer(&self, customer_id: Uuid) -> Result<Option<LoyaltyMember>>;
    async fn update_member(&self, member: &LoyaltyMember) -> Result<LoyaltyMember>;
    async fn create_transaction(&self, tx: &LoyaltyTransaction) -> Result<LoyaltyTransaction>;
    async fn list_transactions(&self, member_id: Uuid) -> Result<Vec<LoyaltyTransaction>>;
    async fn create_reward(&self, reward: &LoyaltyReward) -> Result<LoyaltyReward>;
    async fn list_rewards(&self, program_id: Uuid) -> Result<Vec<LoyaltyReward>>;
    async fn create_redemption(&self, redemption: &LoyaltyRedemption) -> Result<LoyaltyRedemption>;
    async fn create_promotion(&self, promo: &LoyaltyPromotion) -> Result<LoyaltyPromotion>;
}

pub struct SqliteLoyaltyRepository { pool: SqlitePool }

impl SqliteLoyaltyRepository {
    pub fn new(pool: SqlitePool) -> Self { Self { pool } }
}

#[async_trait]
impl LoyaltyRepository for SqliteLoyaltyRepository {
    async fn create_program(&self, program: &LoyaltyProgram) -> Result<LoyaltyProgram> {
        let p = program.clone();
        sqlx::query(r#"INSERT INTO loyalty_programs (id, program_number, name, description, program_type,
            points_name, earn_rate, earn_currency, minimum_points_redemption, points_expiry_months,
            enrollment_bonus, referral_bonus, birthday_bonus, max_points_per_day, max_points_per_year,
            status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(p.base.id)
            .bind(&p.program_number)
            .bind(&p.name)
            .bind(&p.description)
            .bind(&p.program_type)
            .bind(&p.points_name)
            .bind(p.earn_rate)
            .bind(&p.earn_currency)
            .bind(p.minimum_points_redemption)
            .bind(p.points_expiry_months)
            .bind(p.enrollment_bonus)
            .bind(p.referral_bonus)
            .bind(p.birthday_bonus)
            .bind(p.max_points_per_day)
            .bind(p.max_points_per_year)
            .bind(&p.status)
            .bind(p.created_at)
            .bind(p.updated_at)
            .execute(&self.pool).await?;
        Ok(p)
    }
    async fn get_program(&self, _id: Uuid) -> Result<Option<LoyaltyProgram>> { Ok(None) }
    async fn list_programs(&self) -> Result<Vec<LoyaltyProgram>> { Ok(vec![]) }
    async fn create_tier(&self, tier: &LoyaltyTier) -> Result<LoyaltyTier> {
        let t = tier.clone();
        sqlx::query(r#"INSERT INTO loyalty_tiers (id, program_id, tier_type, name, description,
            minimum_points, maximum_points, earn_multiplier, benefits, upgrade_bonus, color_code,
            priority, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(t.base.id)
            .bind(t.program_id)
            .bind(&t.tier_type)
            .bind(&t.name)
            .bind(&t.description)
            .bind(t.minimum_points)
            .bind(t.maximum_points)
            .bind(t.earn_multiplier)
            .bind(&t.benefits)
            .bind(t.upgrade_bonus)
            .bind(&t.color_code)
            .bind(t.priority)
            .bind(&t.status)
            .bind(t.created_at)
            .bind(t.updated_at)
            .execute(&self.pool).await?;
        Ok(t)
    }
    async fn list_tiers(&self, _program_id: Uuid) -> Result<Vec<LoyaltyTier>> { Ok(vec![]) }
    async fn create_member(&self, member: &LoyaltyMember) -> Result<LoyaltyMember> {
        let m = member.clone();
        sqlx::query(r#"INSERT INTO loyalty_members (id, member_number, program_id, customer_id, tier_id,
            enrollment_date, enrollment_source, total_points_earned, total_points_redeemed, available_points,
            lifetime_points, points_expiring, points_expiring_date, total_rewards_redeemed, tier_upgrade_date,
            tier_downgrade_date, referral_code, referred_by, referral_count, status, last_activity_date,
            created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(m.base.id)
            .bind(&m.member_number)
            .bind(m.program_id)
            .bind(m.customer_id)
            .bind(m.tier_id)
            .bind(m.enrollment_date)
            .bind(&m.enrollment_source)
            .bind(m.total_points_earned)
            .bind(m.total_points_redeemed)
            .bind(m.available_points)
            .bind(m.lifetime_points)
            .bind(m.points_expiring)
            .bind(m.points_expiring_date)
            .bind(m.total_rewards_redeemed)
            .bind(m.tier_upgrade_date)
            .bind(m.tier_downgrade_date)
            .bind(&m.referral_code)
            .bind(m.referred_by)
            .bind(m.referral_count)
            .bind(&m.status)
            .bind(m.last_activity_date)
            .bind(m.created_at)
            .bind(m.updated_at)
            .execute(&self.pool).await?;
        Ok(m)
    }
    async fn get_member(&self, _id: Uuid) -> Result<Option<LoyaltyMember>> { Ok(None) }
    async fn get_member_by_customer(&self, _customer_id: Uuid) -> Result<Option<LoyaltyMember>> { Ok(None) }
    async fn update_member(&self, member: &LoyaltyMember) -> Result<LoyaltyMember> { Ok(member.clone()) }
    async fn create_transaction(&self, tx: &LoyaltyTransaction) -> Result<LoyaltyTransaction> {
        let t = tx.clone();
        sqlx::query(r#"INSERT INTO loyalty_transactions (id, transaction_number, member_id,
            transaction_type, point_type, points, running_balance, source_type, source_id, order_id,
            amount, currency, description, expiry_date, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(t.base.id)
            .bind(&t.transaction_number)
            .bind(t.member_id)
            .bind(&t.transaction_type)
            .bind(&t.point_type)
            .bind(t.points)
            .bind(t.running_balance)
            .bind(&t.source_type)
            .bind(t.source_id)
            .bind(t.order_id)
            .bind(t.amount)
            .bind(&t.currency)
            .bind(&t.description)
            .bind(t.expiry_date)
            .bind(t.created_at)
            .execute(&self.pool).await?;
        Ok(t)
    }
    async fn list_transactions(&self, _member_id: Uuid) -> Result<Vec<LoyaltyTransaction>> { Ok(vec![]) }
    async fn create_reward(&self, reward: &LoyaltyReward) -> Result<LoyaltyReward> {
        let r = reward.clone();
        sqlx::query(r#"INSERT INTO loyalty_rewards (id, reward_number, program_id, name, description,
            reward_type, points_cost, cash_value, currency, product_id, discount_percent, discount_amount,
            min_purchase_amount, max_redemptions, redemptions_count, valid_from, valid_until,
            tier_requirement, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(r.base.id)
            .bind(&r.reward_number)
            .bind(r.program_id)
            .bind(&r.name)
            .bind(&r.description)
            .bind(&r.reward_type)
            .bind(r.points_cost)
            .bind(r.cash_value)
            .bind(&r.currency)
            .bind(r.product_id)
            .bind(r.discount_percent)
            .bind(r.discount_amount)
            .bind(r.min_purchase_amount)
            .bind(r.max_redemptions)
            .bind(r.redemptions_count)
            .bind(r.valid_from)
            .bind(r.valid_until)
            .bind(r.tier_requirement)
            .bind(&r.status)
            .bind(r.created_at)
            .bind(r.updated_at)
            .execute(&self.pool).await?;
        Ok(r)
    }
    async fn list_rewards(&self, _program_id: Uuid) -> Result<Vec<LoyaltyReward>> { Ok(vec![]) }
    async fn create_redemption(&self, redemption: &LoyaltyRedemption) -> Result<LoyaltyRedemption> {
        let r = redemption.clone();
        sqlx::query(r#"INSERT INTO loyalty_redemptions (id, redemption_number, member_id, reward_id,
            points_used, cash_value, currency, order_id, redemption_date, status, voucher_code,
            voucher_expires, used_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(r.base.id)
            .bind(&r.redemption_number)
            .bind(r.member_id)
            .bind(r.reward_id)
            .bind(r.points_used)
            .bind(r.cash_value)
            .bind(&r.currency)
            .bind(r.order_id)
            .bind(r.redemption_date)
            .bind(&r.status)
            .bind(&r.voucher_code)
            .bind(r.voucher_expires)
            .bind(r.used_at)
            .bind(r.created_at)
            .execute(&self.pool).await?;
        Ok(r)
    }
    async fn create_promotion(&self, promo: &LoyaltyPromotion) -> Result<LoyaltyPromotion> {
        let p = promo.clone();
        sqlx::query(r#"INSERT INTO loyalty_promotions (id, promotion_number, program_id, name,
            description, promotion_type, earn_multiplier, bonus_points, start_date, end_date,
            product_ids, category_ids, tier_restriction, max_points_per_member, status,
            created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#)
            .bind(p.base.id)
            .bind(&p.promotion_number)
            .bind(p.program_id)
            .bind(&p.name)
            .bind(&p.description)
            .bind(&p.promotion_type)
            .bind(p.earn_multiplier)
            .bind(p.bonus_points)
            .bind(p.start_date)
            .bind(p.end_date)
            .bind(&p.product_ids)
            .bind(&p.category_ids)
            .bind(&p.tier_restriction)
            .bind(p.max_points_per_member)
            .bind(&p.status)
            .bind(p.created_at)
            .bind(p.updated_at)
            .execute(&self.pool).await?;
        Ok(p)
    }
}
