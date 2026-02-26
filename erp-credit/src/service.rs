use crate::models::*;
use crate::repository::{CreditRepository, SqliteCreditRepository};
use anyhow::Result;
use chrono::Utc;
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use tracing::{info, warn};
use uuid::Uuid;

pub struct CreditService {
    repo: SqliteCreditRepository,
}

impl CreditService {
    pub fn new() -> Self {
        Self {
            repo: SqliteCreditRepository,
        }
    }
    
    pub async fn check_credit(&self, pool: &SqlitePool, request: CreditCheckRequest, user_id: Option<Uuid>) -> Result<CreditCheckResponse> {
        let profile = self.repo.get_profile(pool, request.customer_id).await?;
        
        let profile = match profile {
            Some(p) => p,
            None => {
                return Ok(CreditCheckResponse {
                    customer_id: request.customer_id,
                    result: CreditCheckResult::Approved,
                    credit_limit: 0,
                    credit_used: 0,
                    available_credit: i64::MAX,
                    requested_amount: request.order_amount,
                    projected_available: i64::MAX - request.order_amount,
                    hold_id: None,
                    reason: Some("No credit profile found - customer has no credit limit set".to_string()),
                    warnings: vec!["Customer has no credit profile. Consider setting up credit management.".to_string()],
                    checked_at: Utc::now(),
                });
            }
        };
        
        let active_hold = self.repo.get_active_hold(pool, request.customer_id).await?;
        
        if let Some(hold) = active_hold {
            return Ok(CreditCheckResponse {
                customer_id: request.customer_id,
                result: CreditCheckResult::Blocked,
                credit_limit: profile.credit_limit,
                credit_used: profile.credit_used,
                available_credit: profile.available_credit,
                requested_amount: request.order_amount,
                projected_available: profile.available_credit - request.order_amount,
                hold_id: Some(hold.id),
                reason: Some(format!("Customer is on credit hold: {}", hold.reason)),
                warnings: vec![],
                checked_at: Utc::now(),
            });
        }
        
        let projected_available = profile.available_credit - request.order_amount;
        let mut warnings = Vec::new();
        let mut result = CreditCheckResult::Approved;
        
        let utilization_threshold = (profile.hold_threshold_percent as f64 / 100.0) * profile.credit_limit as f64;
        let projected_utilization = profile.credit_used + request.order_amount;
        
        if projected_available < 0 {
            result = CreditCheckResult::Blocked;
        } else if projected_utilization > utilization_threshold as i64 {
            result = CreditCheckResult::Warning;
            warnings.push(format!(
                "Order will utilize {:.1}% of credit limit",
                (projected_utilization as f64 / profile.credit_limit as f64) * 100.0
            ));
        }
        
        if profile.overdue_amount > 0 {
            warnings.push(format!("Customer has ${:.2} in overdue invoices", profile.overdue_amount as f64 / 100.0));
            if result == CreditCheckResult::Approved && profile.overdue_amount > profile.credit_limit / 4 {
                result = CreditCheckResult::Warning;
            }
        }
        
        if profile.auto_hold_enabled && result == CreditCheckResult::Blocked {
            let hold = CreditHold {
                id: Uuid::new_v4(),
                profile_id: profile.base.id,
                customer_id: request.customer_id,
                hold_type: CreditHoldType::CreditLimitExceeded,
                reason: format!("Credit limit exceeded by order of ${:.2}", request.order_amount as f64 / 100.0),
                amount_over_limit: -projected_available,
                related_order_id: request.order_id,
                related_invoice_id: None,
                status: CreditHoldStatus::Active,
                placed_by: user_id,
                placed_at: Utc::now(),
                released_by: None,
                released_at: None,
                override_reason: None,
                notes: None,
                created_at: Utc::now(),
            };
            self.repo.create_hold(pool, &hold).await?;
            info!("Auto-placed credit hold for customer {}", request.customer_id);
        }
        
        Ok(CreditCheckResponse {
            customer_id: request.customer_id,
            result,
            credit_limit: profile.credit_limit,
            credit_used: profile.credit_used,
            available_credit: profile.available_credit,
            requested_amount: request.order_amount,
            projected_available,
            hold_id: None,
            reason: None,
            warnings,
            checked_at: Utc::now(),
        })
    }
    
    pub async fn get_or_create_profile(&self, pool: &SqlitePool, customer_id: Uuid, initial_limit: i64) -> Result<CustomerCreditProfile> {
        if let Some(profile) = self.repo.get_profile(pool, customer_id).await? {
            return Ok(profile);
        }
        
        let now = Utc::now();
        let profile = CustomerCreditProfile {
            base: BaseEntity::new(),
            customer_id,
            credit_limit: initial_limit,
            credit_used: 0,
            available_credit: initial_limit,
            outstanding_invoices: 0,
            pending_orders: 0,
            overdue_amount: 0,
            overdue_days_avg: 0,
            credit_score: None,
            risk_level: RiskLevel::Low,
            payment_history_score: None,
            last_credit_review: None,
            next_review_date: None,
            auto_hold_enabled: true,
            hold_threshold_percent: 90,
            status: erp_core::Status::Active,
            created_at: now,
            updated_at: now,
        };
        
        self.repo.create_profile(pool, &profile).await?;
        Ok(profile)
    }
    
    pub async fn update_credit_limit(&self, pool: &SqlitePool, customer_id: Uuid, new_limit: i64, reason: String, user_id: Uuid) -> Result<CustomerCreditProfile> {
        let mut profile = self.repo.get_profile(pool, customer_id).await?
            .ok_or_else(|| anyhow::anyhow!("Credit profile not found"))?;
        
        let change = CreditLimitChange {
            id: Uuid::new_v4(),
            profile_id: profile.base.id,
            customer_id,
            previous_limit: profile.credit_limit,
            new_limit,
            change_reason: reason,
            approved_by: Some(user_id),
            approved_at: Some(Utc::now()),
            effective_date: Utc::now(),
            created_by: user_id,
            created_at: Utc::now(),
        };
        self.repo.create_limit_change(pool, &change).await?;
        
        let previous_used = profile.credit_used;
        profile.credit_limit = new_limit;
        profile.available_credit = new_limit - profile.credit_used;
        profile.updated_at = Utc::now();
        
        let txn = CreditTransaction {
            id: Uuid::new_v4(),
            profile_id: profile.base.id,
            customer_id,
            transaction_type: CreditTransactionType::CreditLimitChanged,
            amount: new_limit - change.previous_limit,
            previous_credit_used: previous_used,
            new_credit_used: profile.credit_used,
            reference_type: Some("CreditLimitChange".to_string()),
            reference_id: Some(change.id),
            reference_number: None,
            description: Some(format!("Credit limit changed from ${:.2} to ${:.2}", 
                change.previous_limit as f64 / 100.0, new_limit as f64 / 100.0)),
            created_by: Some(user_id),
            created_at: Utc::now(),
        };
        self.repo.create_transaction(pool, &txn).await?;
        
        self.repo.update_profile(pool, &profile).await?;
        
        if profile.credit_used > profile.credit_limit && profile.auto_hold_enabled {
            let existing_hold = self.repo.get_active_hold(pool, customer_id).await?;
            if existing_hold.is_none() {
                let hold = CreditHold {
                    id: Uuid::new_v4(),
                    profile_id: profile.base.id,
                    customer_id,
                    hold_type: CreditHoldType::CreditLimitExceeded,
                    reason: "Credit limit reduced below current usage".to_string(),
                    amount_over_limit: profile.credit_used - profile.credit_limit,
                    related_order_id: None,
                    related_invoice_id: None,
                    status: CreditHoldStatus::Active,
                    placed_by: Some(user_id),
                    placed_at: Utc::now(),
                    released_by: None,
                    released_at: None,
                    override_reason: None,
                    notes: None,
                    created_at: Utc::now(),
                };
                self.repo.create_hold(pool, &hold).await?;
            }
        }
        
        Ok(profile)
    }
    
    pub async fn record_invoice(&self, pool: &SqlitePool, customer_id: Uuid, invoice_id: Uuid, invoice_number: String, amount: i64, user_id: Option<Uuid>) -> Result<CustomerCreditProfile> {
        let mut profile = self.get_or_create_profile(pool, customer_id, 0).await?;
        
        let previous_used = profile.credit_used;
        profile.credit_used += amount;
        profile.outstanding_invoices += amount;
        profile.available_credit = profile.credit_limit - profile.credit_used;
        profile.pending_orders = (profile.pending_orders - amount).max(0);
        profile.updated_at = Utc::now();
        
        self.update_risk_level(&mut profile);
        
        let txn = CreditTransaction {
            id: Uuid::new_v4(),
            profile_id: profile.base.id,
            customer_id,
            transaction_type: CreditTransactionType::InvoiceCreated,
            amount,
            previous_credit_used: previous_used,
            new_credit_used: profile.credit_used,
            reference_type: Some("Invoice".to_string()),
            reference_id: Some(invoice_id),
            reference_number: Some(invoice_number),
            description: Some(format!("Invoice created for ${:.2}", amount as f64 / 100.0)),
            created_by: user_id,
            created_at: Utc::now(),
        };
        self.repo.create_transaction(pool, &txn).await?;
        
        self.repo.update_profile(pool, &profile).await?;
        
        if profile.credit_used > profile.credit_limit && profile.auto_hold_enabled {
            let existing_hold = self.repo.get_active_hold(pool, customer_id).await?;
            if existing_hold.is_none() {
                let hold = CreditHold {
                    id: Uuid::new_v4(),
                    profile_id: profile.base.id,
                    customer_id,
                    hold_type: CreditHoldType::CreditLimitExceeded,
                    reason: format!("Credit limit exceeded after invoice: ${:.2} over limit", 
                        (profile.credit_used - profile.credit_limit) as f64 / 100.0),
                    amount_over_limit: profile.credit_used - profile.credit_limit,
                    related_order_id: None,
                    related_invoice_id: Some(invoice_id),
                    status: CreditHoldStatus::Active,
                    placed_by: user_id,
                    placed_at: Utc::now(),
                    released_by: None,
                    released_at: None,
                    override_reason: None,
                    notes: None,
                    created_at: Utc::now(),
                };
                self.repo.create_hold(pool, &hold).await?;
                self.create_alert(pool, &profile, CreditAlertType::LimitExceeded, AlertSeverity::Critical,
                    format!("Credit limit exceeded by ${:.2}", (profile.credit_used - profile.credit_limit) as f64 / 100.0)).await?;
            }
        } else if profile.available_credit < profile.credit_limit / 10 {
            self.create_alert(pool, &profile, CreditAlertType::ApproachingLimit, AlertSeverity::Warning,
                format!("Only ${:.2} credit available", profile.available_credit as f64 / 100.0)).await?;
        }
        
        Ok(profile)
    }
    
    pub async fn record_payment(&self, pool: &SqlitePool, customer_id: Uuid, invoice_id: Option<Uuid>, amount: i64, user_id: Option<Uuid>) -> Result<CustomerCreditProfile> {
        let mut profile = self.repo.get_profile(pool, customer_id).await?
            .ok_or_else(|| anyhow::anyhow!("Credit profile not found"))?;
        
        let previous_used = profile.credit_used;
        profile.credit_used = (profile.credit_used - amount).max(0);
        profile.outstanding_invoices = (profile.outstanding_invoices - amount).max(0);
        profile.available_credit = profile.credit_limit - profile.credit_used;
        profile.updated_at = Utc::now();
        
        let txn = CreditTransaction {
            id: Uuid::new_v4(),
            profile_id: profile.base.id,
            customer_id,
            transaction_type: CreditTransactionType::InvoicePaid,
            amount,
            previous_credit_used: previous_used,
            new_credit_used: profile.credit_used,
            reference_type: Some("Payment".to_string()),
            reference_id: invoice_id,
            reference_number: None,
            description: Some(format!("Payment received: ${:.2}", amount as f64 / 100.0)),
            created_by: user_id,
            created_at: Utc::now(),
        };
        self.repo.create_transaction(pool, &txn).await?;
        
        if let Some(hold) = self.repo.get_active_hold(pool, customer_id).await? {
            if profile.credit_used <= profile.credit_limit {
                self.repo.release_hold(pool, hold.id, user_id, Some("Payment received - credit within limit".to_string())).await?;
                self.create_alert(pool, &profile, CreditAlertType::HoldReleased, AlertSeverity::Info,
                    "Credit hold released after payment".to_string()).await?;
            }
        }
        
        self.update_risk_level(&mut profile);
        self.repo.update_profile(pool, &profile).await?;
        
        Ok(profile)
    }
    
    pub async fn record_order(&self, pool: &SqlitePool, customer_id: Uuid, order_id: Uuid, order_number: String, amount: i64, user_id: Option<Uuid>) -> Result<()> {
        let profile = match self.repo.get_profile(pool, customer_id).await? {
            Some(p) => p,
            None => return Ok(()),
        };
        
        let mut updated = profile.clone();
        updated.pending_orders += amount;
        updated.updated_at = Utc::now();
        
        let txn = CreditTransaction {
            id: Uuid::new_v4(),
            profile_id: profile.base.id,
            customer_id,
            transaction_type: CreditTransactionType::OrderPlaced,
            amount,
            previous_credit_used: profile.credit_used,
            new_credit_used: profile.credit_used,
            reference_type: Some("SalesOrder".to_string()),
            reference_id: Some(order_id),
            reference_number: Some(order_number),
            description: Some(format!("Order placed for ${:.2}", amount as f64 / 100.0)),
            created_by: user_id,
            created_at: Utc::now(),
        };
        self.repo.create_transaction(pool, &txn).await?;
        
        self.repo.update_profile(pool, &updated).await?;
        Ok(())
    }
    
    pub async fn place_manual_hold(&self, pool: &SqlitePool, customer_id: Uuid, reason: String, user_id: Uuid) -> Result<CreditHold> {
        let profile = self.repo.get_profile(pool, customer_id).await?
            .ok_or_else(|| anyhow::anyhow!("Credit profile not found"))?;
        
        if let Some(existing) = self.repo.get_active_hold(pool, customer_id).await? {
            return Err(anyhow::anyhow!("Customer already has an active credit hold"));
        }
        
        let hold = CreditHold {
            id: Uuid::new_v4(),
            profile_id: profile.base.id,
            customer_id,
            hold_type: CreditHoldType::ManualHold,
            reason,
            amount_over_limit: 0,
            related_order_id: None,
            related_invoice_id: None,
            status: CreditHoldStatus::Active,
            placed_by: Some(user_id),
            placed_at: Utc::now(),
            released_by: None,
            released_at: None,
            override_reason: None,
            notes: None,
            created_at: Utc::now(),
        };
        
        self.repo.create_hold(pool, &hold).await?;
        
        let txn = CreditTransaction {
            id: Uuid::new_v4(),
            profile_id: profile.base.id,
            customer_id,
            transaction_type: CreditTransactionType::CreditHoldPlaced,
            amount: 0,
            previous_credit_used: profile.credit_used,
            new_credit_used: profile.credit_used,
            reference_type: Some("CreditHold".to_string()),
            reference_id: Some(hold.id),
            reference_number: None,
            description: Some("Manual credit hold placed".to_string()),
            created_by: Some(user_id),
            created_at: Utc::now(),
        };
        self.repo.create_transaction(pool, &txn).await?;
        
        self.create_alert(pool, &profile, CreditAlertType::HoldPlaced, AlertSeverity::Warning,
            "Manual credit hold placed".to_string()).await?;
        
        Ok(hold)
    }
    
    pub async fn release_hold(&self, pool: &SqlitePool, customer_id: Uuid, override_reason: String, user_id: Uuid) -> Result<()> {
        let profile = self.repo.get_profile(pool, customer_id).await?
            .ok_or_else(|| anyhow::anyhow!("Credit profile not found"))?;
        
        let hold = self.repo.get_active_hold(pool, customer_id).await?
            .ok_or_else(|| anyhow::anyhow!("No active credit hold found"))?;
        
        self.repo.release_hold(pool, hold.id, Some(user_id), Some(override_reason)).await?;
        
        let txn = CreditTransaction {
            id: Uuid::new_v4(),
            profile_id: profile.base.id,
            customer_id,
            transaction_type: CreditTransactionType::CreditHoldReleased,
            amount: 0,
            previous_credit_used: profile.credit_used,
            new_credit_used: profile.credit_used,
            reference_type: Some("CreditHold".to_string()),
            reference_id: Some(hold.id),
            reference_number: None,
            description: Some("Credit hold released".to_string()),
            created_by: Some(user_id),
            created_at: Utc::now(),
        };
        self.repo.create_transaction(pool, &txn).await?;
        
        Ok(())
    }
    
    pub async fn get_profile(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Option<CustomerCreditProfile>> {
        self.repo.get_profile(pool, customer_id).await
    }
    
    pub async fn list_profiles(&self, pool: &SqlitePool, page: i64, limit: i64) -> Result<Vec<CustomerCreditProfile>> {
        self.repo.list_profiles(pool, page, limit).await
    }
    
    pub async fn list_on_hold(&self, pool: &SqlitePool) -> Result<Vec<CustomerCreditProfile>> {
        self.repo.list_on_hold(pool).await
    }
    
    pub async fn list_high_risk(&self, pool: &SqlitePool) -> Result<Vec<CustomerCreditProfile>> {
        self.repo.list_high_risk(pool).await
    }
    
    pub async fn list_transactions(&self, pool: &SqlitePool, customer_id: Uuid, limit: i64) -> Result<Vec<CreditTransaction>> {
        self.repo.list_transactions(pool, customer_id, limit).await
    }
    
    pub async fn list_holds(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<CreditHold>> {
        self.repo.list_holds(pool, customer_id).await
    }
    
    pub async fn list_limit_changes(&self, pool: &SqlitePool, customer_id: Uuid) -> Result<Vec<CreditLimitChange>> {
        self.repo.list_limit_changes(pool, customer_id).await
    }
    
    pub async fn get_summary(&self, pool: &SqlitePool) -> Result<CreditSummary> {
        self.repo.get_summary(pool).await
    }
    
    pub async fn list_unread_alerts(&self, pool: &SqlitePool) -> Result<Vec<CreditAlert>> {
        self.repo.list_unread_alerts(pool).await
    }
    
    pub async fn acknowledge_alert(&self, pool: &SqlitePool, alert_id: Uuid, user_id: Uuid) -> Result<()> {
        self.repo.acknowledge_alert(pool, alert_id, user_id).await
    }
    
    fn update_risk_level(&self, profile: &mut CustomerCreditProfile) {
        let utilization = if profile.credit_limit > 0 {
            profile.credit_used as f64 / profile.credit_limit as f64
        } else {
            0.0
        };
        
        let overdue_ratio = if profile.outstanding_invoices > 0 {
            profile.overdue_amount as f64 / profile.outstanding_invoices as f64
        } else {
            0.0
        };
        
        profile.risk_level = if utilization > 0.95 || overdue_ratio > 0.5 {
            RiskLevel::Critical
        } else if utilization > 0.8 || overdue_ratio > 0.3 {
            RiskLevel::High
        } else if utilization > 0.6 || overdue_ratio > 0.1 {
            RiskLevel::Medium
        } else {
            RiskLevel::Low
        };
    }
    
    async fn create_alert(&self, pool: &SqlitePool, profile: &CustomerCreditProfile, 
                          alert_type: CreditAlertType, severity: AlertSeverity, message: String) -> Result<()> {
        let alert = CreditAlert {
            id: Uuid::new_v4(),
            profile_id: profile.base.id,
            customer_id: profile.customer_id,
            alert_type,
            severity,
            message,
            threshold_value: profile.credit_limit,
            actual_value: profile.credit_used,
            is_read: false,
            acknowledged_by: None,
            acknowledged_at: None,
            created_at: Utc::now(),
        };
        self.repo.create_alert(pool, &alert).await
    }
}
