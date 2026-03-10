use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ExpenseType {
    Travel,
    Meals,
    Lodging,
    Entertainment,
    OfficeSupplies,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseLine {
    pub id: Uuid,
    pub expense_type: ExpenseType,
    pub amount: i64, // in minor units
    pub has_receipt: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpensePolicy {
    pub expense_type: ExpenseType,
    pub max_per_transaction: Option<i64>,
    pub receipt_required_above: Option<i64>,
    pub is_prohibited: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ViolationSeverity {
    Warning,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyViolation {
    pub line_id: Uuid,
    pub severity: ViolationSeverity,
    pub message: String,
}

pub struct ExpensePolicyEngine {
    policies: Vec<ExpensePolicy>,
}

impl ExpensePolicyEngine {
    pub fn new(policies: Vec<ExpensePolicy>) -> Self {
        Self { policies }
    }

    pub fn validate_report(&self, lines: &[ExpenseLine]) -> Vec<PolicyViolation> {
        let mut violations = Vec::new();

        for line in lines {
            if let Some(policy) = self.policies.iter().find(|p| p.expense_type == line.expense_type) {
                // 1. Check if category is prohibited
                if policy.is_prohibited {
                    violations.push(PolicyViolation {
                        line_id: line.id,
                        severity: ViolationSeverity::Critical,
                        message: format!("Expense type {:?} is prohibited by policy", line.expense_type),
                    });
                    continue; // No need to check other rules if prohibited
                }

                // 2. Check per-transaction limit
                if let Some(limit) = policy.max_per_transaction {
                    if line.amount > limit {
                        violations.push(PolicyViolation {
                            line_id: line.id,
                            severity: ViolationSeverity::Critical,
                            message: format!(
                                "Expense amount {} exceeds the maximum allowed limit of {} for {:?}",
                                line.amount, limit, line.expense_type
                            ),
                        });
                    }
                }

                // 3. Check receipt requirement
                if let Some(threshold) = policy.receipt_required_above {
                    if line.amount > threshold && !line.has_receipt {
                        violations.push(PolicyViolation {
                            line_id: line.id,
                            severity: ViolationSeverity::Warning,
                            message: format!(
                                "Receipt is required for {:?} expenses above {}",
                                line.expense_type, threshold
                            ),
                        });
                    }
                }
            }
        }

        violations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expense_policy_validation() {
        let policies = vec![
            ExpensePolicy {
                expense_type: ExpenseType::Meals,
                max_per_transaction: Some(5000), // $50.00
                receipt_required_above: Some(2500), // $25.00
                is_prohibited: false,
            },
            ExpensePolicy {
                expense_type: ExpenseType::Entertainment,
                max_per_transaction: None,
                receipt_required_above: None,
                is_prohibited: true,
            },
        ];

        let engine = ExpensePolicyEngine::new(policies);

            let l1 = Uuid::new_v4(); // Valid meal
            let l2 = Uuid::new_v4(); // Meal missing receipt
            let l3 = Uuid::new_v4(); // Meal exceeding limit
            let l4 = Uuid::new_v4(); // Prohibited entertainment

        let lines = vec![
            ExpenseLine { id: l1, expense_type: ExpenseType::Meals, amount: 2000, has_receipt: false },
            ExpenseLine { id: l2, expense_type: ExpenseType::Meals, amount: 3000, has_receipt: false },
            ExpenseLine { id: l3, expense_type: ExpenseType::Meals, amount: 6000, has_receipt: true },
            ExpenseLine { id: l4, expense_type: ExpenseType::Entertainment, amount: 1000, has_receipt: true },
        ];

        let violations = engine.validate_report(&lines);

        assert_eq!(violations.len(), 3);

        // Check L2 violation (Warning)
        let v_l2 = violations.iter().find(|v| v.line_id == l2).unwrap();
        assert_eq!(v_l2.severity, ViolationSeverity::Warning);
        assert!(v_l2.message.contains("Receipt is required"));

        // Check L3 violation (Critical)
        let v_l3 = violations.iter().find(|v| v.line_id == l3).unwrap();
        assert_eq!(v_l3.severity, ViolationSeverity::Critical);
        assert!(v_l3.message.contains("exceeds the maximum allowed limit"));

        // Check L4 violation (Critical)
        let v_l4 = violations.iter().find(|v| v.line_id == l4).unwrap();
        assert_eq!(v_l4.severity, ViolationSeverity::Critical);
        assert!(v_l4.message.contains("prohibited"));
    }
}
