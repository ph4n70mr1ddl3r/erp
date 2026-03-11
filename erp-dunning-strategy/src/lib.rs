use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DunningAction {
    EmailNotification,
    LetterPost,
    PhoneCall,
    CreditHold,
    LegalAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DunningStep {
    pub level: u32,
    pub min_days_overdue: u32,
    pub min_amount_threshold: i64,
    pub action: DunningAction,
    pub fee_to_apply: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverdueInvoice {
    pub id: Uuid,
    pub customer_id: Uuid,
    pub amount: i64,
    pub days_overdue: u32,
    pub current_dunning_level: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DunningRecommendation {
    pub invoice_id: Uuid,
    pub next_level: u32,
    pub action: DunningAction,
    pub reason: String,
}

pub struct DunningStrategyEngine {
    steps: Vec<DunningStep>,
}

impl DunningStrategyEngine {
    pub fn new(mut steps: Vec<DunningStep>) -> Self {
        // Ensure steps are sorted by level
        steps.sort_by_key(|s| s.level);
        Self { steps }
    }

    /// Evaluates an invoice and recommends the next dunning step.
    pub fn evaluate(&self, invoice: &OverdueInvoice) -> Option<DunningRecommendation> {
        // Find the highest step that meets the criteria and is greater than current level
        let next_step = self.steps.iter()
            .filter(|s| s.level > invoice.current_dunning_level)
            .filter(|s| invoice.days_overdue >= s.min_days_overdue)
            .filter(|s| invoice.amount >= s.min_amount_threshold)
            .last();

        next_step.map(|step| DunningRecommendation {
            invoice_id: invoice.id,
            next_level: step.level,
            action: step.action.clone(),
            reason: format!(
                "Invoice is {} days overdue with amount {}, matching Step {} criteria",
                invoice.days_overdue, invoice.amount, step.level
            ),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dunning_progression() {
        let steps = vec![
            DunningStep {
                level: 1,
                min_days_overdue: 7,
                min_amount_threshold: 100,
                action: DunningAction::EmailNotification,
                fee_to_apply: 0,
            },
            DunningStep {
                level: 2,
                min_days_overdue: 30,
                min_amount_threshold: 500,
                action: DunningAction::PhoneCall,
                fee_to_apply: 2500, // $25.00
            },
            DunningStep {
                level: 3,
                min_days_overdue: 60,
                min_amount_threshold: 1000,
                action: DunningAction::CreditHold,
                fee_to_apply: 5000,
            },
        ];

        let engine = DunningStrategyEngine::new(steps);

        let invoice = OverdueInvoice {
            id: Uuid::new_v4(),
            customer_id: Uuid::new_v4(),
            amount: 2000,
            days_overdue: 45,
            current_dunning_level: 1,
        };

        // Should recommend Level 2 (Phone Call) because it's > 30 days and level 1 is done
        let rec = engine.evaluate(&invoice).unwrap();
        assert_eq!(rec.next_level, 2);
        assert_eq!(rec.action, DunningAction::PhoneCall);

        // Advance invoice to level 2 and 65 days
        let mut invoice_advanced = invoice.clone();
        invoice_advanced.current_dunning_level = 2;
        invoice_advanced.days_overdue = 65;

        // Should recommend Level 3 (Credit Hold)
        let rec2 = engine.evaluate(&invoice_advanced).unwrap();
        assert_eq!(rec2.next_level, 3);
        assert_eq!(rec2.action, DunningAction::CreditHold);
    }

    #[test]
    fn test_dunning_threshold_gate() {
        let steps = vec![
            DunningStep {
                level: 1,
                min_days_overdue: 15,
                min_amount_threshold: 5000, // Only dun for large amounts
                action: DunningAction::EmailNotification,
                fee_to_apply: 0,
            },
        ];

        let engine = DunningStrategyEngine::new(steps);

        let invoice = OverdueInvoice {
            id: Uuid::new_v4(),
            customer_id: Uuid::new_v4(),
            amount: 1000, // Below threshold
            days_overdue: 20,
            current_dunning_level: 0,
        };

        // Should NOT recommend anything due to amount threshold
        assert!(engine.evaluate(&invoice).is_none());
    }
}
