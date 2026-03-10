use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MatchStatus {
    Matched,
    Discrepancy,
    Unmatched,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ICTransaction {
    pub id: Uuid,
    pub entity_id: Uuid,
    pub partner_entity_id: Uuid,
    pub amount: i64,
    pub currency: String,
    pub reference: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ICMatchResult {
    pub source_id: Uuid,
    pub target_id: Option<Uuid>,
    pub status: MatchStatus,
    pub variance: i64,
    pub notes: String,
}

pub struct ICReconciliationEngine {
    source_transactions: Vec<ICTransaction>,
    target_transactions: Vec<ICTransaction>,
}

impl ICReconciliationEngine {
    pub fn new(source_transactions: Vec<ICTransaction>, target_transactions: Vec<ICTransaction>) -> Self {
        Self {
            source_transactions,
            target_transactions,
        }
    }

    /// Reconciles transactions between two entities based on reference and amount.
    pub fn reconcile(&self) -> Vec<ICMatchResult> {
        let mut results = Vec::new();
        let mut matched_target_ids = std::collections::HashSet::new();

        for source in &self.source_transactions {
            // Find a matching transaction in the target entity
            // In intercompany, if Entity A sells $100 to B, Entity B should have a purchase of $100 from A.
            let target_match = self.target_transactions.iter().find(|t| {
                !matched_target_ids.contains(&t.id) && 
                t.reference == source.reference &&
                t.entity_id == source.partner_entity_id &&
                t.partner_entity_id == source.entity_id
            });

            match target_match {
                Some(target) => {
                    matched_target_ids.insert(target.id);
                    let variance = source.amount - target.amount;
                    let status = if variance == 0 {
                        MatchStatus::Matched
                    } else {
                        MatchStatus::Discrepancy
                    };

                    results.push(ICMatchResult {
                        source_id: source.id,
                        target_id: Some(target.id),
                        status,
                        variance,
                        notes: if variance != 0 {
                            format!("Amount mismatch: {} vs {}", source.amount, target.amount)
                        } else {
                            "Perfect match".to_string()
                        },
                    });
                }
                None => {
                    results.push(ICMatchResult {
                        source_id: source.id,
                        target_id: None,
                        status: MatchStatus::Unmatched,
                        variance: source.amount,
                        notes: "No corresponding transaction found in partner entity".to_string(),
                    });
                }
            }
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_intercompany_reconciliation() {
        let entity_a = Uuid::new_v4();
        let entity_b = Uuid::new_v4();
        let ref1 = "INV-2026-001".to_string();
        let ref2 = "INV-2026-002".to_string();

        let source_txs = vec![
            ICTransaction {
                id: Uuid::new_v4(),
                entity_id: entity_a,
                partner_entity_id: entity_b,
                amount: 1000,
                currency: "USD".to_string(),
                reference: ref1.clone(),
            },
            ICTransaction {
                id: Uuid::new_v4(),
                entity_id: entity_a,
                partner_entity_id: entity_b,
                amount: 500,
                currency: "USD".to_string(),
                reference: ref2.clone(),
            },
        ];

        let target_txs = vec![
            ICTransaction {
                id: Uuid::new_v4(),
                entity_id: entity_b,
                partner_entity_id: entity_a,
                amount: 1000,
                currency: "USD".to_string(),
                reference: ref1.clone(),
            },
            ICTransaction {
                id: Uuid::new_v4(),
                entity_id: entity_b,
                partner_entity_id: entity_a,
                amount: 450, // Discrepancy!
                currency: "USD".to_string(),
                reference: ref2.clone(),
            },
        ];

        let engine = ICReconciliationEngine::new(source_txs, target_txs);
        let results = engine.reconcile();

        assert_eq!(results.len(), 2);

        // First transaction: Matched
        let res1 = results.iter().find(|r| r.variance == 0).unwrap();
        assert_eq!(res1.status, MatchStatus::Matched);

        // Second transaction: Discrepancy
        let res2 = results.iter().find(|r| r.variance == 50).unwrap();
        assert_eq!(res2.status, MatchStatus::Discrepancy);
    }
}
