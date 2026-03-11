use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssetRevaluation {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub revaluation_date: DateTime<Utc>,
    pub previous_book_value: f64,
    pub new_fair_market_value: f64,
    pub total_accumulated_depreciation: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RevaluationResult {
    pub asset_id: Uuid,
    /// The change in book value
    pub adjustment_amount: f64,
    /// Surplus goes to Revaluation Reserve (Equity)
    pub revaluation_surplus: f64,
    /// Loss goes to Income Statement (Expense)
    pub revaluation_loss: f64,
    pub new_book_value: f64,
}

pub struct RevaluationEngine;

impl RevaluationEngine {
    /// Revalues an asset based on its current book value and new fair market value.
    /// Following the 'Elimination Method' where accumulated depreciation is reset.
    pub fn revalue(entry: &AssetRevaluation) -> RevaluationResult {
        let adjustment = entry.new_fair_market_value - entry.previous_book_value;
        
        let mut surplus = 0.0;
        let mut loss = 0.0;

        if adjustment > 0.0 {
            surplus = adjustment;
        } else {
            loss = adjustment.abs();
        }

        RevaluationResult {
            asset_id: entry.asset_id,
            adjustment_amount: adjustment,
            revaluation_surplus: surplus,
            revaluation_loss: loss,
            new_book_value: entry.new_fair_market_value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_asset_revaluation_surplus() {
        let asset_id = Uuid::new_v4();
        let entry = AssetRevaluation {
            id: Uuid::new_v4(),
            asset_id,
            revaluation_date: Utc::now(),
            previous_book_value: 50000.0,
            new_fair_market_value: 65000.0, // Value went up
            total_accumulated_depreciation: 10000.0,
        };

        let result = RevaluationEngine::revalue(&entry);

        assert_eq!(result.adjustment_amount, 15000.0);
        assert_eq!(result.revaluation_surplus, 15000.0);
        assert_eq!(result.revaluation_loss, 0.0);
        assert_eq!(result.new_book_value, 65000.0);
    }

    #[test]
    fn test_asset_revaluation_loss() {
        let asset_id = Uuid::new_v4();
        let entry = AssetRevaluation {
            id: Uuid::new_v4(),
            asset_id,
            revaluation_date: Utc::now(),
            previous_book_value: 40000.0,
            new_fair_market_value: 32000.0, // Value went down (impairment)
            total_accumulated_depreciation: 5000.0,
        };

        let result = RevaluationEngine::revalue(&entry);

        assert_eq!(result.adjustment_amount, -8000.0);
        assert_eq!(result.revaluation_surplus, 0.0);
        assert_eq!(result.revaluation_loss, 8000.0);
        assert_eq!(result.new_book_value, 32000.0);
    }
}
