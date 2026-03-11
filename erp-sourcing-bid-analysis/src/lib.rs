use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationCriteria {
    /// Relative weight of price (e.g., 0.50 for 50%)
    pub price_weight: f64,
    /// Relative weight of lead time (e.g., 0.20 for 20%)
    pub lead_time_weight: f64,
    /// Relative weight of technical/quality score (e.g., 0.30 for 30%)
    pub technical_weight: f64,
}

impl Default for EvaluationCriteria {
    fn default() -> Self {
        Self {
            price_weight: 0.60,
            lead_time_weight: 0.20,
            technical_weight: 0.20,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidResponse {
    pub vendor_id: Uuid,
    pub bid_amount: f64,
    pub lead_time_days: u32,
    /// Score from 0 to 100
    pub technical_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidRanking {
    pub vendor_id: Uuid,
    pub weighted_score: f64,
    pub price_score: f64,
    pub lead_time_score: f64,
    pub technical_score: f64,
    pub rank: usize,
}

pub struct BidEvaluationEngine {
    criteria: EvaluationCriteria,
}

impl BidEvaluationEngine {
    pub fn new(criteria: EvaluationCriteria) -> Self {
        Self { criteria }
    }

    /// Evaluates multiple bids and returns a ranked list.
    /// Lower price and lower lead time are considered better.
    pub fn evaluate_bids(&self, bids: &[BidResponse]) -> Vec<BidRanking> {
        if bids.is_empty() {
            return Vec::new();
        }

        // Find min/max for normalization
        let min_price = bids.iter().map(|b| b.bid_amount).fold(f64::INFINITY, f64::min);
        let min_lead_time = bids.iter().map(|b| b.lead_time_days).min().unwrap_or(1) as f64;

        let mut rankings: Vec<BidRanking> = bids.iter().map(|bid| {
            // Price Score (Normalized: Min Price / Current Price * 100)
            let p_score = (min_price / bid.bid_amount) * 100.0;
            
            // Lead Time Score (Normalized: Min Lead Time / Current Lead Time * 100)
            let l_score = (min_lead_time / bid.lead_time_days as f64) * 100.0;
            
            // Technical Score is already 0-100
            let t_score = bid.technical_score;

            let weighted_score = (p_score * self.criteria.price_weight) +
                                 (l_score * self.criteria.lead_time_weight) +
                                 (t_score * self.criteria.technical_weight);

            BidRanking {
                vendor_id: bid.vendor_id,
                weighted_score,
                price_score: p_score,
                lead_time_score: l_score,
                technical_score: t_score,
                rank: 0, // Assigned below
            }
        }).collect();

        // Sort by weighted score descending
        rankings.sort_by(|a, b| b.weighted_score.partial_cmp(&a.weighted_score).unwrap());

        // Assign ranks
        for (i, ranking) in rankings.iter_mut().enumerate() {
            ranking.rank = i + 1;
        }

        rankings
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bid_evaluation_logic() {
        let criteria = EvaluationCriteria {
            price_weight: 0.50,
            lead_time_weight: 0.25,
            technical_weight: 0.25,
        };
        let engine = BidEvaluationEngine::new(criteria);

        let v1 = Uuid::new_v4(); // Cheap but slow/low quality
        let v2 = Uuid::new_v4(); // Expensive but fast/high quality
        let v3 = Uuid::new_v4(); // Balanced

        let bids = vec![
            BidResponse { vendor_id: v1, bid_amount: 1000.0, lead_time_days: 30, technical_score: 60.0 },
            BidResponse { vendor_id: v2, bid_amount: 2000.0, lead_time_days: 10, technical_score: 95.0 },
            BidResponse { vendor_id: v3, bid_amount: 1200.0, lead_time_days: 15, technical_score: 85.0 },
        ];

        let results = engine.evaluate_bids(&bids);

        assert_eq!(results.len(), 3);
        
        // v3 should be rank 1 due to balance
        assert_eq!(results[0].vendor_id, v3);
        assert_eq!(results[0].rank, 1);

        // v1 should have the best price score (100.0)
        let r1 = results.iter().find(|r| r.vendor_id == v1).unwrap();
        assert_eq!(r1.price_score, 100.0);

        // v2 should have the best lead time score (100.0)
        let r2 = results.iter().find(|r| r.vendor_id == v2).unwrap();
        assert_eq!(r2.lead_time_score, 100.0);
    }
}
