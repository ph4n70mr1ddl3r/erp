use crate::models::*;
use crate::repository::*;
use anyhow::Result;
use chrono::{Duration, Utc};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct SubscriptionPlanService;

impl SubscriptionPlanService {
    pub async fn create(pool: &SqlitePool, req: CreatePlanRequest) -> Result<SubscriptionPlan> {
        let now = Utc::now();
        let plan = SubscriptionPlan {
            id: Uuid::new_v4(),
            code: req.code,
            name: req.name,
            description: req.description,
            price: req.price,
            currency: req.currency,
            billing_interval: req.billing_interval,
            interval_count: req.interval_count,
            trial_days: req.trial_days,
            features: req.features,
            max_users: req.max_users,
            max_transactions: req.max_transactions,
            is_active: true,
            created_at: now,
            updated_at: now,
        };
        SubscriptionPlanRepository::create(pool, &plan).await?;
        Ok(plan)
    }

    pub async fn list_active(pool: &SqlitePool) -> Result<Vec<SubscriptionPlan>> {
        SubscriptionPlanRepository::list_active(pool).await
    }
}

pub struct SubscriptionService;

impl SubscriptionService {
    pub async fn create(pool: &SqlitePool, req: CreateSubscriptionRequest) -> Result<Subscription> {
        let plans = SubscriptionPlanRepository::list_active(pool).await?;
        let plan = plans.iter().find(|p| p.id == req.plan_id)
            .ok_or_else(|| anyhow::anyhow!("Plan not found"))?;
        
        let now = Utc::now();
        let trial_days = req.trial_days.unwrap_or(plan.trial_days);
        let (trial_start, trial_end, period_start, period_end, status) = if trial_days > 0 {
            let ts = now;
            let te = ts + Duration::days(trial_days as i64);
            (Some(ts), Some(te), ts, te, SubscriptionStatus::Trial)
        } else {
            let ps = now;
            let pe = Self::calculate_period_end(&ps, &plan.billing_interval, plan.interval_count);
            (None, None, ps, pe, SubscriptionStatus::Active)
        };
        
        let sub = Subscription {
            id: Uuid::new_v4(),
            customer_id: req.customer_id,
            plan_id: req.plan_id,
            status,
            quantity: req.quantity,
            price_override: req.price_override,
            current_period_start: period_start,
            current_period_end: period_end,
            trial_start,
            trial_end,
            cancelled_at: None,
            cancel_at_period_end: false,
            metadata: req.metadata,
            created_at: now,
            updated_at: now,
        };
        SubscriptionRepository::create(pool, &sub).await?;
        Ok(sub)
    }

    fn calculate_period_end(start: &chrono::DateTime<Utc>, interval: &BillingInterval, count: i32) -> chrono::DateTime<Utc> {
        match interval {
            BillingInterval::Daily => *start + Duration::days(count as i64),
            BillingInterval::Weekly => *start + Duration::weeks(count as i64),
            BillingInterval::Monthly => *start + Duration::days(30 * count as i64),
            BillingInterval::Quarterly => *start + Duration::days(90 * count as i64),
            BillingInterval::SemiAnnual => *start + Duration::days(180 * count as i64),
            BillingInterval::Annual => *start + Duration::days(365 * count as i64),
            BillingInterval::Custom => *start + Duration::days(30),
        }
    }

    pub async fn list_by_customer(pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<Subscription>> {
        SubscriptionRepository::list_by_customer(pool, customer_id).await
    }

    pub async fn cancel(pool: &SqlitePool, id: Uuid, at_period_end: bool) -> Result<Subscription> {
        let now = Utc::now();
        sqlx::query(
            r#"UPDATE subscriptions SET cancel_at_period_end = ?, cancelled_at = ?, status = 'Cancelled', updated_at = ? WHERE id = ?"#
        )
        .bind(at_period_end)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool).await?;
        
        let rows = sqlx::query("SELECT id, customer_id, plan_id, status, quantity, price_override, current_period_start, current_period_end, trial_start, trial_end, cancelled_at, cancel_at_period_end, metadata, created_at, updated_at FROM subscriptions WHERE id = ?")
            .bind(id.to_string())
            .fetch_all(pool).await?;
        SubscriptionRepository::row_to_sub(&rows[0])
    }

    pub async fn renew(pool: &SqlitePool, id: Uuid) -> Result<Subscription> {
        let rows = sqlx::query("SELECT id, customer_id, plan_id, status, quantity, price_override, current_period_start, current_period_end, trial_start, trial_end, cancelled_at, cancel_at_period_end, metadata, created_at, updated_at FROM subscriptions WHERE id = ?")
            .bind(id.to_string())
            .fetch_all(pool).await?;
        let sub = SubscriptionRepository::row_to_sub(&rows[0])?;
        
        let plans = SubscriptionPlanRepository::list_active(pool).await?;
        let plan = plans.iter().find(|p| p.id == sub.plan_id)
            .ok_or_else(|| anyhow::anyhow!("Plan not found for subscription"))?;
        
        let new_start = sub.current_period_end;
        let new_end = Self::calculate_period_end(&new_start, &plan.billing_interval, plan.interval_count);
        let now = Utc::now();
        
        sqlx::query(
            r#"UPDATE subscriptions SET current_period_start = ?, current_period_end = ?, status = 'Active', updated_at = ? WHERE id = ?"#
        )
        .bind(new_start.to_rfc3339())
        .bind(new_end.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool).await?;
        
        let rows = sqlx::query("SELECT id, customer_id, plan_id, status, quantity, price_override, current_period_start, current_period_end, trial_start, trial_end, cancelled_at, cancel_at_period_end, metadata, created_at, updated_at FROM subscriptions WHERE id = ?")
            .bind(id.to_string())
            .fetch_all(pool).await?;
        SubscriptionRepository::row_to_sub(&rows[0])
    }

    pub async fn get_expiring(pool: &SqlitePool, days: i32) -> Result<Vec<Subscription>> {
        let threshold = Utc::now() + Duration::days(days as i64);
        let rows = sqlx::query(
            r#"SELECT id, customer_id, plan_id, status, quantity, price_override, current_period_start, current_period_end, trial_start, trial_end, cancelled_at, cancel_at_period_end, metadata, created_at, updated_at FROM subscriptions WHERE status = 'Active' AND current_period_end <= ?"#
        )
        .bind(threshold.to_rfc3339())
        .fetch_all(pool).await?;
        rows.iter().map(SubscriptionRepository::row_to_sub).collect()
    }

    pub async fn record_usage(pool: &SqlitePool, subscription_id: Uuid, usage_type: String, quantity: i64, unit: String) -> Result<SubscriptionUsage> {
        let now = Utc::now();
        let usage = SubscriptionUsage {
            id: Uuid::new_v4(),
            subscription_id,
            usage_type,
            quantity,
            unit,
            recorded_at: now,
            created_at: now,
        };
        sqlx::query(
            r#"INSERT INTO subscription_usage (id, subscription_id, usage_type, quantity, unit, recorded_at, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(usage.id.to_string())
        .bind(usage.subscription_id.to_string())
        .bind(&usage.usage_type)
        .bind(usage.quantity)
        .bind(&usage.unit)
        .bind(usage.recorded_at.to_rfc3339())
        .bind(usage.created_at.to_rfc3339())
        .execute(pool).await?;
        Ok(usage)
    }
}
