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
        sqlx::query!(r#"INSERT INTO loyalty_programs (id, program_number, name, description, program_type,
            points_name, earn_rate, earn_currency, minimum_points_redemption, points_expiry_months,
            enrollment_bonus, referral_bonus, birthday_bonus, max_points_per_day, max_points_per_year,
            status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            p.base.id, p.program_number, p.name, p.description, p.program_type, p.points_name,
            p.earn_rate, p.earn_currency, p.minimum_points_redemption, p.points_expiry_months,
            p.enrollment_bonus, p.referral_bonus, p.birthday_bonus, p.max_points_per_day, p.max_points_per_year,
            p.status, p.created_at, p.updated_at).execute(&self.pool).await?;
        Ok(p)
    }
    async fn get_program(&self, _id: Uuid) -> Result<Option<LoyaltyProgram>> { Ok(None) }
    async fn list_programs(&self) -> Result<Vec<LoyaltyProgram>> { Ok(vec![]) }
    async fn create_tier(&self, tier: &LoyaltyTier) -> Result<LoyaltyTier> {
        let t = tier.clone();
        sqlx::query!(r#"INSERT INTO loyalty_tiers (id, program_id, tier_type, name, description,
            minimum_points, maximum_points, earn_multiplier, benefits, upgrade_bonus, color_code,
            priority, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            t.base.id, t.program_id, t.tier_type, t.name, t.description, t.minimum_points, t.maximum_points,
            t.earn_multiplier, t.benefits, t.upgrade_bonus, t.color_code, t.priority, t.status,
            t.created_at, t.updated_at).execute(&self.pool).await?;
        Ok(t)
    }
    async fn list_tiers(&self, _program_id: Uuid) -> Result<Vec<LoyaltyTier>> { Ok(vec![]) }
    async fn create_member(&self, member: &LoyaltyMember) -> Result<LoyaltyMember> {
        let m = member.clone();
        sqlx::query!(r#"INSERT INTO loyalty_members (id, member_number, program_id, customer_id, tier_id,
            enrollment_date, enrollment_source, total_points_earned, total_points_redeemed, available_points,
            lifetime_points, points_expiring, points_expiring_date, total_rewards_redeemed, tier_upgrade_date,
            tier_downgrade_date, referral_code, referred_by, referral_count, status, last_activity_date,
            created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            m.base.id, m.member_number, m.program_id, m.customer_id, m.tier_id, m.enrollment_date,
            m.enrollment_source, m.total_points_earned, m.total_points_redeemed, m.available_points,
            m.lifetime_points, m.points_expiring, m.points_expiring_date, m.total_rewards_redeemed,
            m.tier_upgrade_date, m.tier_downgrade_date, m.referral_code, m.referred_by, m.referral_count,
            m.status, m.last_activity_date, m.created_at, m.updated_at).execute(&self.pool).await?;
        Ok(m)
    }
    async fn get_member(&self, _id: Uuid) -> Result<Option<LoyaltyMember>> { Ok(None) }
    async fn get_member_by_customer(&self, _customer_id: Uuid) -> Result<Option<LoyaltyMember>> { Ok(None) }
    async fn update_member(&self, member: &LoyaltyMember) -> Result<LoyaltyMember> { Ok(member.clone()) }
    async fn create_transaction(&self, tx: &LoyaltyTransaction) -> Result<LoyaltyTransaction> {
        let t = tx.clone();
        sqlx::query!(r#"INSERT INTO loyalty_transactions (id, transaction_number, member_id,
            transaction_type, point_type, points, running_balance, source_type, source_id, order_id,
            amount, currency, description, expiry_date, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            t.base.id, t.transaction_number, t.member_id, t.transaction_type, t.point_type, t.points,
            t.running_balance, t.source_type, t.source_id, t.order_id, t.amount, t.currency,
            t.description, t.expiry_date, t.created_at).execute(&self.pool).await?;
        Ok(t)
    }
    async fn list_transactions(&self, _member_id: Uuid) -> Result<Vec<LoyaltyTransaction>> { Ok(vec![]) }
    async fn create_reward(&self, reward: &LoyaltyReward) -> Result<LoyaltyReward> {
        let r = reward.clone();
        sqlx::query!(r#"INSERT INTO loyalty_rewards (id, reward_number, program_id, name, description,
            reward_type, points_cost, cash_value, currency, product_id, discount_percent, discount_amount,
            min_purchase_amount, max_redemptions, redemptions_count, valid_from, valid_until,
            tier_requirement, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            r.base.id, r.reward_number, r.program_id, r.name, r.description, r.reward_type,
            r.points_cost, r.cash_value, r.currency, r.product_id, r.discount_percent, r.discount_amount,
            r.min_purchase_amount, r.max_redemptions, r.redemptions_count, r.valid_from, r.valid_until,
            r.tier_requirement, r.status, r.created_at, r.updated_at).execute(&self.pool).await?;
        Ok(r)
    }
    async fn list_rewards(&self, _program_id: Uuid) -> Result<Vec<LoyaltyReward>> { Ok(vec![]) }
    async fn create_redemption(&self, redemption: &LoyaltyRedemption) -> Result<LoyaltyRedemption> {
        let r = redemption.clone();
        sqlx::query!(r#"INSERT INTO loyalty_redemptions (id, redemption_number, member_id, reward_id,
            points_used, cash_value, currency, order_id, redemption_date, status, voucher_code,
            voucher_expires, used_at, created_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            r.base.id, r.redemption_number, r.member_id, r.reward_id, r.points_used, r.cash_value,
            r.currency, r.order_id, r.redemption_date, r.status, r.voucher_code, r.voucher_expires,
            r.used_at, r.created_at).execute(&self.pool).await?;
        Ok(r)
    }
    async fn create_promotion(&self, promo: &LoyaltyPromotion) -> Result<LoyaltyPromotion> {
        let p = promo.clone();
        sqlx::query!(r#"INSERT INTO loyalty_promotions (id, promotion_number, program_id, name,
            description, promotion_type, earn_multiplier, bonus_points, start_date, end_date,
            product_ids, category_ids, tier_restriction, max_points_per_member, status,
            created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            p.base.id, p.promotion_number, p.program_id, p.name, p.description, p.promotion_type,
            p.earn_multiplier, p.bonus_points, p.start_date, p.end_date, p.product_ids, p.category_ids,
            p.tier_restriction, p.max_points_per_member, p.status, p.created_at, p.updated_at).execute(&self.pool).await?;
        Ok(p)
    }
}
