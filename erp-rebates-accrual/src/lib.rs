use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RebateAccrual {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub agreement_id: Uuid,
    pub accrual_date: DateTime<Utc>,
    pub base_amount: i64,
    pub accrual_rate: f64,
    pub accrual_amount: i64,
    pub status: AccrualStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AccrualStatus {
    Pending,
    Posted,
    Settled,
    Reversed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub partner_id: Uuid,
    pub amount: i64,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccrualPolicy {
    pub agreement_id: Uuid,
    pub partner_id: Uuid,
    /// Expected rebate percentage to accrue (e.g., 0.03 for 3%)
    /// This is often a conservative estimate of which tier will be reached.
    pub target_accrual_rate: f64,
    pub is_active: bool,
}

pub struct AccrualEngine {
    policies: Vec<AccrualPolicy>,
}

impl AccrualEngine {
    pub fn new(policies: Vec<AccrualPolicy>) -> Self {
        Self { policies }
    }

    /// Processes a transaction and generates a rebate accrual if a matching policy exists.
    pub fn process_transaction(&self, tx: &Transaction) -> Option<RebateAccrual> {
        // Find active policy for this partner
        let policy = self.policies.iter()
            .find(|p| p.partner_id == tx.partner_id && p.is_active)?;

        let accrual_amount = (tx.amount as f64 * policy.target_accrual_rate).round() as i64;

        Some(RebateAccrual {
            id: Uuid::new_v4(),
            transaction_id: tx.id,
            agreement_id: policy.agreement_id,
            accrual_date: Utc::now(),
            base_amount: tx.amount,
            accrual_rate: policy.target_accrual_rate,
            accrual_amount,
            status: AccrualStatus::Pending,
        })
    }

    /// Adjusts accruals when an agreement period ends and final volume is known.
    pub fn calculate_adjustment(&self, accruals: &[RebateAccrual], final_rebate_amount: i64) -> i64 {
        let total_accrued: i64 = accruals.iter()
            .filter(|a| a.status != AccrualStatus::Reversed)
            .map(|a| a.accrual_amount)
            .sum();

        final_rebate_amount - total_accrued
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rebate_accrual_generation() {
        let partner_id = Uuid::new_v4();
        let agreement_id = Uuid::new_v4();
        
        let policy = AccrualPolicy {
            agreement_id,
            partner_id,
            target_accrual_rate: 0.05, // Accrue 5%
            is_active: true,
        };

        let engine = AccrualEngine::new(vec![policy]);

        let tx = Transaction {
            id: Uuid::new_v4(),
            partner_id,
            amount: 10000, // $100.00
            date: Utc::now(),
        };

        let accrual = engine.process_transaction(&tx).expect("Should generate accrual");
        
        assert_eq!(accrual.agreement_id, agreement_id);
        assert_eq!(accrual.accrual_amount, 500); // 5% of 10000
        assert_eq!(accrual.status, AccrualStatus::Pending);
    }

    #[test]
    fn test_accrual_adjustment() {
        let engine = AccrualEngine::new(vec![]);
        
        let accruals = vec![
            RebateAccrual {
                id: Uuid::new_v4(),
                transaction_id: Uuid::new_v4(),
                agreement_id: Uuid::new_v4(),
                accrual_date: Utc::now(),
                base_amount: 10000,
                accrual_rate: 0.02,
                accrual_amount: 200,
                status: AccrualStatus::Posted,
            },
            RebateAccrual {
                id: Uuid::new_v4(),
                transaction_id: Uuid::new_v4(),
                agreement_id: Uuid::new_v4(),
                accrual_date: Utc::now(),
                base_amount: 20000,
                accrual_rate: 0.02,
                accrual_amount: 400,
                status: AccrualStatus::Posted,
            },
        ];

        // Total accrued = 600
        // If final rebate calculated is 750 (maybe a higher tier was hit)
        let adjustment = engine.calculate_adjustment(&accruals, 750);
        assert_eq!(adjustment, 150);

        // If final rebate is 500 (lower tier)
        let adjustment2 = engine.calculate_adjustment(&accruals, 500);
        assert_eq!(adjustment2, -100);
    }
}
