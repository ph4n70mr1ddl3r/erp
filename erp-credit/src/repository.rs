use crate::models::*;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use uuid::Uuid;
use anyhow::Result;

#[async_trait]
pub trait CreditRepository: Send + Sync {
    async fn get_profile(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Option<CustomerCreditProfile>>;
    async fn get_profile_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<CustomerCreditProfile>>;
    async fn create_profile(&self, pool: &SqlitePool, profile: &CustomerCreditProfile) -> Result<()>;
    async fn update_profile(&self, pool: &SqlitePool, profile: &CustomerCreditProfile) -> Result<()>;
    async fn list_profiles(&self, pool: &SqlitePool, page: i64, limit: i64) -> Result<Vec<CustomerCreditProfile>>;
    async fn list_on_hold(&self, pool: &SqlitePool) -> Result<Vec<CustomerCreditProfile>>;
    async fn list_high_risk(&self, pool: &SqlitePool) -> Result<Vec<CustomerCreditProfile>>;
    
    async fn create_transaction(&self, pool: &SqlitePool, txn: &CreditTransaction) -> Result<()>;
    async fn list_transactions(&self, pool: &SqlitePool, customer_id: Uuid, limit: i64) -> Result<Vec<CreditTransaction>>;
    
    async fn create_hold(&self, pool: &SqlitePool, hold: &CreditHold) -> Result<()>;
    async fn get_active_hold(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Option<CreditHold>>;
    async fn release_hold(&self, pool: &SqlitePool, hold_id: Uuid, released_by: Option<Uuid>, override_reason: Option<String>) -> Result<()>;
    async fn list_holds(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<CreditHold>>;
    
    async fn create_limit_change(&self, pool: &SqlitePool, change: &CreditLimitChange) -> Result<()>;
    async fn list_limit_changes(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<CreditLimitChange>>;
    
    async fn create_alert(&self, pool: &SqlitePool, alert: &CreditAlert) -> Result<()>;
    async fn list_unread_alerts(&self, pool: &SqlitePool) -> Result<Vec<CreditAlert>>;
    async fn acknowledge_alert(&self, pool: &SqlitePool, alert_id: Uuid, user_id: Uuid) -> Result<()>;
    
    async fn get_summary(&self, pool: &SqlitePool) -> Result<CreditSummary>;
}

pub struct SqliteCreditRepository;

impl SqliteCreditRepository {
    fn row_to_profile(row: &sqlx::sqlite::SqliteRow) -> Result<CustomerCreditProfile> {
        use sqlx::Row;
        Ok(CustomerCreditProfile {
            base: BaseEntity::new_with_id(Uuid::parse_str(row.get::<String, _>("id").as_str())?),
            customer_id: Uuid::parse_str(row.get::<String, _>("customer_id").as_str())?,
            credit_limit: row.get("credit_limit"),
            credit_used: row.get("credit_used"),
            available_credit: row.get("available_credit"),
            outstanding_invoices: row.get("outstanding_invoices"),
            pending_orders: row.get("pending_orders"),
            overdue_amount: row.get("overdue_amount"),
            overdue_days_avg: row.get("overdue_days_avg"),
            credit_score: row.get("credit_score"),
            risk_level: serde_json::from_str(&row.get::<String, _>("risk_level"))?,
            payment_history_score: row.get("payment_history_score"),
            last_credit_review: row.get::<Option<String>, _>("last_credit_review")
                .map(|d| DateTime::parse_from_rfc3339(&d).map(|dt| dt.with_timezone(&Utc)))
                .transpose()?,
            next_review_date: row.get::<Option<String>, _>("next_review_date")
                .map(|d| DateTime::parse_from_rfc3339(&d).map(|dt| dt.with_timezone(&Utc)))
                .transpose()?,
            auto_hold_enabled: row.get("auto_hold_enabled"),
            hold_threshold_percent: row.get("hold_threshold_percent"),
            status: serde_json::from_str(&row.get::<String, _>("status"))?,
            created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
            updated_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("updated_at"))?.with_timezone(&Utc),
        })
    }
    
    fn row_to_transaction(row: &sqlx::sqlite::SqliteRow) -> Result<CreditTransaction> {
        use sqlx::Row;
        Ok(CreditTransaction {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str())?,
            profile_id: Uuid::parse_str(row.get::<String, _>("profile_id").as_str())?,
            customer_id: Uuid::parse_str(row.get::<String, _>("customer_id").as_str())?,
            transaction_type: serde_json::from_str(&row.get::<String, _>("transaction_type"))?,
            amount: row.get("amount"),
            previous_credit_used: row.get("previous_credit_used"),
            new_credit_used: row.get("new_credit_used"),
            reference_type: row.get("reference_type"),
            reference_id: row.get::<Option<String>, _>("reference_id").map(|id| Uuid::parse_str(&id)).transpose()?,
            reference_number: row.get("reference_number"),
            description: row.get("description"),
            created_by: row.get::<Option<String>, _>("created_by").map(|id| Uuid::parse_str(&id)).transpose()?,
            created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
        })
    }
    
    fn row_to_hold(row: &sqlx::sqlite::SqliteRow) -> Result<CreditHold> {
        use sqlx::Row;
        Ok(CreditHold {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str())?,
            profile_id: Uuid::parse_str(row.get::<String, _>("profile_id").as_str())?,
            customer_id: Uuid::parse_str(row.get::<String, _>("customer_id").as_str())?,
            hold_type: serde_json::from_str(&row.get::<String, _>("hold_type"))?,
            reason: row.get("reason"),
            amount_over_limit: row.get("amount_over_limit"),
            related_order_id: row.get::<Option<String>, _>("related_order_id").map(|id| Uuid::parse_str(&id)).transpose()?,
            related_invoice_id: row.get::<Option<String>, _>("related_invoice_id").map(|id| Uuid::parse_str(&id)).transpose()?,
            status: serde_json::from_str(&row.get::<String, _>("status"))?,
            placed_by: row.get::<Option<String>, _>("placed_by").map(|id| Uuid::parse_str(&id)).transpose()?,
            placed_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("placed_at"))?.with_timezone(&Utc),
            released_by: row.get::<Option<String>, _>("released_by").map(|id| Uuid::parse_str(&id)).transpose()?,
            released_at: row.get::<Option<String>, _>("released_at")
                .map(|d| DateTime::parse_from_rfc3339(&d).map(|dt| dt.with_timezone(&Utc)))
                .transpose()?,
            override_reason: row.get("override_reason"),
            notes: row.get("notes"),
            created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
        })
    }
    
    fn row_to_limit_change(row: &sqlx::sqlite::SqliteRow) -> Result<CreditLimitChange> {
        use sqlx::Row;
        Ok(CreditLimitChange {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str())?,
            profile_id: Uuid::parse_str(row.get::<String, _>("profile_id").as_str())?,
            customer_id: Uuid::parse_str(row.get::<String, _>("customer_id").as_str())?,
            previous_limit: row.get("previous_limit"),
            new_limit: row.get("new_limit"),
            change_reason: row.get("change_reason"),
            approved_by: row.get::<Option<String>, _>("approved_by").map(|id| Uuid::parse_str(&id)).transpose()?,
            approved_at: row.get::<Option<String>, _>("approved_at")
                .map(|d| DateTime::parse_from_rfc3339(&d).map(|dt| dt.with_timezone(&Utc)))
                .transpose()?,
            effective_date: DateTime::parse_from_rfc3339(&row.get::<String, _>("effective_date"))?.with_timezone(&Utc),
            created_by: Uuid::parse_str(row.get::<String, _>("created_by").as_str())?,
            created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
        })
    }
    
    fn row_to_alert(row: &sqlx::sqlite::SqliteRow) -> Result<CreditAlert> {
        use sqlx::Row;
        Ok(CreditAlert {
            id: Uuid::parse_str(row.get::<String, _>("id").as_str())?,
            profile_id: Uuid::parse_str(row.get::<String, _>("profile_id").as_str())?,
            customer_id: Uuid::parse_str(row.get::<String, _>("customer_id").as_str())?,
            alert_type: serde_json::from_str(&row.get::<String, _>("alert_type"))?,
            severity: serde_json::from_str(&row.get::<String, _>("severity"))?,
            message: row.get("message"),
            threshold_value: row.get("threshold_value"),
            actual_value: row.get("actual_value"),
            is_read: row.get("is_read"),
            acknowledged_by: row.get::<Option<String>, _>("acknowledged_by").map(|id| Uuid::parse_str(&id)).transpose()?,
            acknowledged_at: row.get::<Option<String>, _>("acknowledged_at")
                .map(|d| DateTime::parse_from_rfc3339(&d).map(|dt| dt.with_timezone(&Utc)))
                .transpose()?,
            created_at: DateTime::parse_from_rfc3339(&row.get::<String, _>("created_at"))?.with_timezone(&Utc),
        })
    }
}

#[async_trait]
impl CreditRepository for SqliteCreditRepository {
    async fn get_profile(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Option<CustomerCreditProfile>> {
        let row = sqlx::query(
            "SELECT * FROM customer_credit_profiles WHERE customer_id = ?"
        )
        .bind(customer_id.to_string())
        .fetch_optional(pool)
        .await?;
        
        match row {
            Some(r) => Ok(Some(Self::row_to_profile(&r)?)),
            None => Ok(None),
        }
    }
    
    async fn get_profile_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<CustomerCreditProfile>> {
        let row = sqlx::query(
            "SELECT * FROM customer_credit_profiles WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?;
        
        match row {
            Some(r) => Ok(Some(Self::row_to_profile(&r)?)),
            None => Ok(None),
        }
    }
    
    async fn create_profile(&self, pool: &SqlitePool, profile: &CustomerCreditProfile) -> Result<()> {
        sqlx::query(
            "INSERT INTO customer_credit_profiles 
             (id, customer_id, credit_limit, credit_used, available_credit, outstanding_invoices, 
              pending_orders, overdue_amount, overdue_days_avg, credit_score, risk_level, 
              payment_history_score, last_credit_review, next_review_date, auto_hold_enabled, 
              hold_threshold_percent, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(profile.base.id.to_string())
        .bind(profile.customer_id.to_string())
        .bind(profile.credit_limit)
        .bind(profile.credit_used)
        .bind(profile.available_credit)
        .bind(profile.outstanding_invoices)
        .bind(profile.pending_orders)
        .bind(profile.overdue_amount)
        .bind(profile.overdue_days_avg)
        .bind(profile.credit_score)
        .bind(serde_json::to_string(&profile.risk_level)?)
        .bind(profile.payment_history_score)
        .bind(profile.last_credit_review.map(|d| d.to_rfc3339()))
        .bind(profile.next_review_date.map(|d| d.to_rfc3339()))
        .bind(profile.auto_hold_enabled)
        .bind(profile.hold_threshold_percent)
        .bind(serde_json::to_string(&profile.status)?)
        .bind(profile.created_at.to_rfc3339())
        .bind(profile.updated_at.to_rfc3339())
        .execute(pool)
        .await?;
        Ok(())
    }
    
    async fn update_profile(&self, pool: &SqlitePool, profile: &CustomerCreditProfile) -> Result<()> {
        sqlx::query(
            "UPDATE customer_credit_profiles SET 
             credit_limit = ?, credit_used = ?, available_credit = ?, outstanding_invoices = ?,
             pending_orders = ?, overdue_amount = ?, overdue_days_avg = ?, credit_score = ?,
             risk_level = ?, payment_history_score = ?, last_credit_review = ?, next_review_date = ?,
             auto_hold_enabled = ?, hold_threshold_percent = ?, status = ?, updated_at = ?
             WHERE id = ?"
        )
        .bind(profile.credit_limit)
        .bind(profile.credit_used)
        .bind(profile.available_credit)
        .bind(profile.outstanding_invoices)
        .bind(profile.pending_orders)
        .bind(profile.overdue_amount)
        .bind(profile.overdue_days_avg)
        .bind(profile.credit_score)
        .bind(serde_json::to_string(&profile.risk_level)?)
        .bind(profile.payment_history_score)
        .bind(profile.last_credit_review.map(|d| d.to_rfc3339()))
        .bind(profile.next_review_date.map(|d| d.to_rfc3339()))
        .bind(profile.auto_hold_enabled)
        .bind(profile.hold_threshold_percent)
        .bind(serde_json::to_string(&profile.status)?)
        .bind(Utc::now().to_rfc3339())
        .bind(profile.base.id.to_string())
        .execute(pool)
        .await?;
        Ok(())
    }
    
    async fn list_profiles(&self, pool: &SqlitePool, page: i64, limit: i64) -> Result<Vec<CustomerCreditProfile>> {
        let offset = (page - 1) * limit;
        let rows = sqlx::query(
            "SELECT * FROM customer_credit_profiles ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await?;
        
        rows.iter().map(Self::row_to_profile).collect()
    }
    
    async fn list_on_hold(&self, pool: &SqlitePool) -> Result<Vec<CustomerCreditProfile>> {
        let rows = sqlx::query(
            "SELECT p.* FROM customer_credit_profiles p
             INNER JOIN credit_holds h ON p.id = h.profile_id AND h.status = 'Active'
             ORDER BY p.updated_at DESC"
        )
        .fetch_all(pool)
        .await?;
        
        rows.iter().map(Self::row_to_profile).collect()
    }
    
    async fn list_high_risk(&self, pool: &SqlitePool) -> Result<Vec<CustomerCreditProfile>> {
        let rows = sqlx::query(
            "SELECT * FROM customer_credit_profiles 
             WHERE risk_level IN ('\"High\"', '\"Critical\"')
             ORDER BY overdue_amount DESC"
        )
        .fetch_all(pool)
        .await?;
        
        rows.iter().map(Self::row_to_profile).collect()
    }
    
    async fn create_transaction(&self, pool: &SqlitePool, txn: &CreditTransaction) -> Result<()> {
        sqlx::query(
            "INSERT INTO credit_transactions 
             (id, profile_id, customer_id, transaction_type, amount, previous_credit_used, 
              new_credit_used, reference_type, reference_id, reference_number, description, created_by, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(txn.id.to_string())
        .bind(txn.profile_id.to_string())
        .bind(txn.customer_id.to_string())
        .bind(serde_json::to_string(&txn.transaction_type)?)
        .bind(txn.amount)
        .bind(txn.previous_credit_used)
        .bind(txn.new_credit_used)
        .bind(&txn.reference_type)
        .bind(txn.reference_id.map(|id| id.to_string()))
        .bind(&txn.reference_number)
        .bind(&txn.description)
        .bind(txn.created_by.map(|id| id.to_string()))
        .bind(txn.created_at.to_rfc3339())
        .execute(pool)
        .await?;
        Ok(())
    }
    
    async fn list_transactions(&self, pool: &SqlitePool, customer_id: Uuid, limit: i64) -> Result<Vec<CreditTransaction>> {
        let rows = sqlx::query(
            "SELECT * FROM credit_transactions WHERE customer_id = ? ORDER BY created_at DESC LIMIT ?"
        )
        .bind(customer_id.to_string())
        .bind(limit)
        .fetch_all(pool)
        .await?;
        
        rows.iter().map(Self::row_to_transaction).collect()
    }
    
    async fn create_hold(&self, pool: &SqlitePool, hold: &CreditHold) -> Result<()> {
        sqlx::query(
            "INSERT INTO credit_holds 
             (id, profile_id, customer_id, hold_type, reason, amount_over_limit, 
              related_order_id, related_invoice_id, status, placed_by, placed_at, 
              released_by, released_at, override_reason, notes, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(hold.id.to_string())
        .bind(hold.profile_id.to_string())
        .bind(hold.customer_id.to_string())
        .bind(serde_json::to_string(&hold.hold_type)?)
        .bind(&hold.reason)
        .bind(hold.amount_over_limit)
        .bind(hold.related_order_id.map(|id| id.to_string()))
        .bind(hold.related_invoice_id.map(|id| id.to_string()))
        .bind(serde_json::to_string(&hold.status)?)
        .bind(hold.placed_by.map(|id| id.to_string()))
        .bind(hold.placed_at.to_rfc3339())
        .bind(hold.released_by.map(|id| id.to_string()))
        .bind(hold.released_at.map(|d| d.to_rfc3339()))
        .bind(&hold.override_reason)
        .bind(&hold.notes)
        .bind(hold.created_at.to_rfc3339())
        .execute(pool)
        .await?;
        Ok(())
    }
    
    async fn get_active_hold(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Option<CreditHold>> {
        let row = sqlx::query(
            "SELECT * FROM credit_holds WHERE customer_id = ? AND status = '\"Active\"'"
        )
        .bind(customer_id.to_string())
        .fetch_optional(pool)
        .await?;
        
        match row {
            Some(r) => Ok(Some(Self::row_to_hold(&r)?)),
            None => Ok(None),
        }
    }
    
    async fn release_hold(&self, pool: &SqlitePool, hold_id: Uuid, released_by: Option<Uuid>, override_reason: Option<String>) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE credit_holds SET status = '\"Released\"', released_by = ?, released_at = ?, override_reason = ? WHERE id = ?"
        )
        .bind(released_by.map(|id| id.to_string()))
        .bind(now.to_rfc3339())
        .bind(&override_reason)
        .bind(hold_id.to_string())
        .execute(pool)
        .await?;
        Ok(())
    }
    
    async fn list_holds(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<CreditHold>> {
        let rows = sqlx::query(
            "SELECT * FROM credit_holds WHERE customer_id = ? ORDER BY created_at DESC"
        )
        .bind(customer_id.to_string())
        .fetch_all(pool)
        .await?;
        
        rows.iter().map(Self::row_to_hold).collect()
    }
    
    async fn create_limit_change(&self, pool: &SqlitePool, change: &CreditLimitChange) -> Result<()> {
        sqlx::query(
            "INSERT INTO credit_limit_changes 
             (id, profile_id, customer_id, previous_limit, new_limit, change_reason, 
              approved_by, approved_at, effective_date, created_by, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(change.id.to_string())
        .bind(change.profile_id.to_string())
        .bind(change.customer_id.to_string())
        .bind(change.previous_limit)
        .bind(change.new_limit)
        .bind(&change.change_reason)
        .bind(change.approved_by.map(|id| id.to_string()))
        .bind(change.approved_at.map(|d| d.to_rfc3339()))
        .bind(change.effective_date.to_rfc3339())
        .bind(change.created_by.to_string())
        .bind(change.created_at.to_rfc3339())
        .execute(pool)
        .await?;
        Ok(())
    }
    
    async fn list_limit_changes(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<CreditLimitChange>> {
        let rows = sqlx::query(
            "SELECT * FROM credit_limit_changes WHERE customer_id = ? ORDER BY created_at DESC"
        )
        .bind(customer_id.to_string())
        .fetch_all(pool)
        .await?;
        
        rows.iter().map(Self::row_to_limit_change).collect()
    }
    
    async fn create_alert(&self, pool: &SqlitePool, alert: &CreditAlert) -> Result<()> {
        sqlx::query(
            "INSERT INTO credit_alerts 
             (id, profile_id, customer_id, alert_type, severity, message, 
              threshold_value, actual_value, is_read, acknowledged_by, acknowledged_at, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(alert.id.to_string())
        .bind(alert.profile_id.to_string())
        .bind(alert.customer_id.to_string())
        .bind(serde_json::to_string(&alert.alert_type)?)
        .bind(serde_json::to_string(&alert.severity)?)
        .bind(&alert.message)
        .bind(alert.threshold_value)
        .bind(alert.actual_value)
        .bind(alert.is_read)
        .bind(alert.acknowledged_by.map(|id| id.to_string()))
        .bind(alert.acknowledged_at.map(|d| d.to_rfc3339()))
        .bind(alert.created_at.to_rfc3339())
        .execute(pool)
        .await?;
        Ok(())
    }
    
    async fn list_unread_alerts(&self, pool: &SqlitePool) -> Result<Vec<CreditAlert>> {
        let rows = sqlx::query(
            "SELECT * FROM credit_alerts WHERE is_read = 0 ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;
        
        rows.iter().map(Self::row_to_alert).collect()
    }
    
    async fn acknowledge_alert(&self, pool: &SqlitePool, alert_id: Uuid, user_id: Uuid) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE credit_alerts SET is_read = 1, acknowledged_by = ?, acknowledged_at = ? WHERE id = ?"
        )
        .bind(user_id.to_string())
        .bind(now.to_rfc3339())
        .bind(alert_id.to_string())
        .execute(pool)
        .await?;
        Ok(())
    }
    
    async fn get_summary(&self, pool: &SqlitePool) -> Result<CreditSummary> {
        use sqlx::Row;
        let row = sqlx::query(
            "SELECT 
                COUNT(*) as total_customers,
                COALESCE(SUM(credit_limit), 0) as total_credit_limit,
                COALESCE(SUM(credit_used), 0) as total_credit_used,
                COALESCE(SUM(available_credit), 0) as total_available_credit,
                COALESCE(SUM(overdue_amount), 0) as total_overdue,
                (SELECT COUNT(DISTINCT customer_id) FROM credit_holds WHERE status = '\"Active\"') as customers_on_hold,
                (SELECT COUNT(*) FROM customer_credit_profiles WHERE risk_level IN ('\"High\"', '\"Critical\"')) as high_risk_customers
             FROM customer_credit_profiles"
        )
        .fetch_one(pool)
        .await?;
        
        let total_limit: i64 = row.get("total_credit_limit");
        let total_used: i64 = row.get("total_credit_used");
        let avg_utilization = if total_limit > 0 {
            (total_used as f64 / total_limit as f64) * 100.0
        } else {
            0.0
        };
        
        Ok(CreditSummary {
            total_customers: row.get("total_customers"),
            total_credit_limit: total_limit,
            total_credit_used: total_used,
            total_available_credit: row.get("total_available_credit"),
            total_overdue: row.get("total_overdue"),
            customers_on_hold: row.get("customers_on_hold"),
            high_risk_customers: row.get("high_risk_customers"),
            avg_utilization_percent: avg_utilization,
        })
    }
}
