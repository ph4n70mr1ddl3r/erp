use crate::models::*;
use anyhow::{Result, Context};
use sqlx::{SqlitePool, Row};
use uuid::Uuid;

pub struct SubscriptionPlanRepository;

impl SubscriptionPlanRepository {
    pub async fn create(pool: &SqlitePool, plan: &SubscriptionPlan) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO subscription_plans (id, code, name, description, price, currency, billing_interval, interval_count, trial_days, features, max_users, max_transactions, is_active, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(plan.id.to_string())
        .bind(&plan.code)
        .bind(&plan.name)
        .bind(&plan.description)
        .bind(plan.price)
        .bind(&plan.currency)
        .bind(format!("{:?}", plan.billing_interval))
        .bind(plan.interval_count)
        .bind(plan.trial_days)
        .bind(&plan.features)
        .bind(plan.max_users)
        .bind(plan.max_transactions)
        .bind(plan.is_active)
        .bind(plan.created_at.to_rfc3339())
        .bind(plan.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(())
    }

    pub async fn list_active(pool: &SqlitePool) -> Result<Vec<SubscriptionPlan>> {
        let rows = sqlx::query(
            r#"SELECT id, code, name, description, price, currency, billing_interval, interval_count, trial_days, features, max_users, max_transactions, is_active, created_at, updated_at FROM subscription_plans WHERE is_active = 1 ORDER BY price"#
        )
        .fetch_all(pool).await?;
        rows.iter().map(Self::row_to_plan).collect()
    }

    fn row_to_plan(r: &sqlx::sqlite::SqliteRow) -> Result<SubscriptionPlan> {
        Ok(SubscriptionPlan {
            id: Uuid::parse_str(r.get::<String, _>("id").as_str())
                .context("Failed to parse plan id")?,
            code: r.get("code"),
            name: r.get("name"),
            description: r.get("description"),
            price: r.get("price"),
            currency: r.get("currency"),
            billing_interval: match r.get::<String, _>("billing_interval").as_str() {
                "Daily" => BillingInterval::Daily,
                "Weekly" => BillingInterval::Weekly,
                "Monthly" => BillingInterval::Monthly,
                "Quarterly" => BillingInterval::Quarterly,
                "SemiAnnual" => BillingInterval::SemiAnnual,
                "Annual" => BillingInterval::Annual,
                _ => BillingInterval::Custom,
            },
            interval_count: r.get("interval_count"),
            trial_days: r.get("trial_days"),
            features: r.get("features"),
            max_users: r.get("max_users"),
            max_transactions: r.get("max_transactions"),
            is_active: r.get("is_active"),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at"))
                .context("Failed to parse created_at")?
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at"))
                .context("Failed to parse updated_at")?
                .with_timezone(&chrono::Utc),
        })
    }
}

pub struct SubscriptionRepository;

impl SubscriptionRepository {
    pub async fn create(pool: &SqlitePool, sub: &Subscription) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO subscriptions (id, customer_id, plan_id, status, quantity, price_override, current_period_start, current_period_end, trial_start, trial_end, cancelled_at, cancel_at_period_end, metadata, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(sub.id.to_string())
        .bind(sub.customer_id.to_string())
        .bind(sub.plan_id.to_string())
        .bind(format!("{:?}", sub.status))
        .bind(sub.quantity)
        .bind(sub.price_override)
        .bind(sub.current_period_start.to_rfc3339())
        .bind(sub.current_period_end.to_rfc3339())
        .bind(sub.trial_start.map(|d| d.to_rfc3339()))
        .bind(sub.trial_end.map(|d| d.to_rfc3339()))
        .bind(sub.cancelled_at.map(|d| d.to_rfc3339()))
        .bind(sub.cancel_at_period_end)
        .bind(&sub.metadata)
        .bind(sub.created_at.to_rfc3339())
        .bind(sub.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(())
    }

    pub async fn list_by_customer(pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<Subscription>> {
        let rows = sqlx::query(
            r#"SELECT id, customer_id, plan_id, status, quantity, price_override, current_period_start, current_period_end, trial_start, trial_end, cancelled_at, cancel_at_period_end, metadata, created_at, updated_at FROM subscriptions WHERE customer_id = ? ORDER BY created_at DESC"#
        )
        .bind(customer_id.to_string())
        .fetch_all(pool).await?;
        rows.iter().map(Self::row_to_sub).collect()
    }

    pub fn row_to_sub(r: &sqlx::sqlite::SqliteRow) -> Result<Subscription> {
        Ok(Subscription {
            id: Uuid::parse_str(r.get::<String, _>("id").as_str())
                .context("Failed to parse subscription id")?,
            customer_id: Uuid::parse_str(r.get::<String, _>("customer_id").as_str())
                .context("Failed to parse customer_id")?,
            plan_id: Uuid::parse_str(r.get::<String, _>("plan_id").as_str())
                .context("Failed to parse plan_id")?,
            status: match r.get::<String, _>("status").as_str() {
                "Active" => SubscriptionStatus::Active,
                "Paused" => SubscriptionStatus::Paused,
                "Cancelled" => SubscriptionStatus::Cancelled,
                "Expired" => SubscriptionStatus::Expired,
                "Trial" => SubscriptionStatus::Trial,
                _ => SubscriptionStatus::Pending,
            },
            quantity: r.get("quantity"),
            price_override: r.get("price_override"),
            current_period_start: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("current_period_start"))
                .context("Failed to parse current_period_start")?
                .with_timezone(&chrono::Utc),
            current_period_end: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("current_period_end"))
                .context("Failed to parse current_period_end")?
                .with_timezone(&chrono::Utc),
            trial_start: r.get::<Option<String>, _>("trial_start").and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&chrono::Utc))),
            trial_end: r.get::<Option<String>, _>("trial_end").and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&chrono::Utc))),
            cancelled_at: r.get::<Option<String>, _>("cancelled_at").and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&chrono::Utc))),
            cancel_at_period_end: r.get("cancel_at_period_end"),
            metadata: r.get("metadata"),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("created_at"))
                .context("Failed to parse created_at")?
                .with_timezone(&chrono::Utc),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.get::<String, _>("updated_at"))
                .context("Failed to parse updated_at")?
                .with_timezone(&chrono::Utc),
        })
    }
}
