use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScoringFactors {
    pub customer_id: Uuid,
    /// Percentage of invoices paid on time (0.0 to 1.0)
    pub on_time_payment_rate: f64,
    /// Current credit utilization (Credit Used / Credit Limit)
    pub credit_utilization: f64,
    /// Average days overdue for late payments
    pub avg_days_overdue: f64,
    /// Total number of years as a customer
    pub customer_tenure_years: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditScoreResult {
    pub customer_id: Uuid,
    /// Score from 0 to 100
    pub numerical_score: i32,
    pub risk_level: RiskLevel,
    pub recommended_limit_multiplier: f64,
}

pub struct CreditScoringEngine {
    /// Weight for on-time payment (e.g., 0.40)
    pub payment_weight: f64,
    /// Weight for utilization (e.g., 0.30 - lower is better)
    pub utilization_weight: f64,
    /// Weight for overdue days (e.g., 0.20 - lower is better)
    pub overdue_weight: f64,
    /// Weight for tenure (e.g., 0.10)
    pub tenure_weight: f64,
}

impl Default for CreditScoringEngine {
    fn default() -> Self {
        Self {
            payment_weight: 0.40,
            utilization_weight: 0.30,
            overdue_weight: 0.20,
            tenure_weight: 0.10,
        }
    }
}

impl CreditScoringEngine {
    pub fn calculate_score(&self, factors: &ScoringFactors) -> CreditScoreResult {
        // 1. Payment Score (0-100): on_time_rate * 100
        let payment_score = factors.on_time_payment_rate * 100.0;

        // 2. Utilization Score (0-100): 100 - (utilization * 100)
        // High utilization is risky
        let utilization_score = (100.0 - (factors.credit_utilization * 100.0)).max(0.0);

        // 3. Overdue Score (0-100):
        // 0 days = 100 points, 30+ days = 0 points
        let overdue_score = (100.0 - (factors.avg_days_overdue * 3.33)).max(0.0);

        // 4. Tenure Score (0-100):
        // 10+ years = 100 points
        let tenure_score = (factors.customer_tenure_years * 10.0).min(100.0);

        let total_score = (payment_score * self.payment_weight) +
                          (utilization_score * self.utilization_weight) +
                          (overdue_score * self.overdue_weight) +
                          (tenure_score * self.tenure_weight);

        let numerical_score = total_score.round() as i32;

        let (risk_level, multiplier) = if numerical_score >= 80 {
            (RiskLevel::Low, 1.5)
        } else if numerical_score >= 60 {
            (RiskLevel::Medium, 1.0)
        } else if numerical_score >= 40 {
            (RiskLevel::High, 0.5)
        } else {
            (RiskLevel::Critical, 0.0)
        };

        CreditScoreResult {
            customer_id: factors.customer_id,
            numerical_score,
            risk_level,
            recommended_limit_multiplier: multiplier,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_credit_score() {
        let customer_id = Uuid::new_v4();
        let engine = CreditScoringEngine::default();
        
        let factors = ScoringFactors {
            customer_id,
            on_time_payment_rate: 1.0, // 100% on time
            credit_utilization: 0.0,   // No debt
            avg_days_overdue: 0.0,     // Never late
            customer_tenure_years: 10.0, // Veteran customer
        };

        let result = engine.calculate_score(&factors);
        assert_eq!(result.numerical_score, 100);
        assert_eq!(result.risk_level, RiskLevel::Low);
        assert_eq!(result.recommended_limit_multiplier, 1.5);
    }

    #[test]
    fn test_risky_credit_score() {
        let customer_id = Uuid::new_v4();
        let engine = CreditScoringEngine::default();
        
        let factors = ScoringFactors {
            customer_id,
            on_time_payment_rate: 0.5, // 50% on time
            credit_utilization: 0.9,   // Maxed out
            avg_days_overdue: 20.0,    // Often 3 weeks late
            customer_tenure_years: 1.0, // New customer
        };

        let result = engine.calculate_score(&factors);
        // (50 * 0.4) + (10 * 0.3) + (33.4 * 0.2) + (10 * 0.1)
        // 20 + 3 + 6.68 + 1 = 30.68 -> 31
        assert_eq!(result.numerical_score, 31);
        assert_eq!(result.risk_level, RiskLevel::Critical);
        assert_eq!(result.recommended_limit_multiplier, 0.0);
    }

    #[test]
    fn test_medium_risk_score() {
        let customer_id = Uuid::new_v4();
        let engine = CreditScoringEngine::default();
        
        let factors = ScoringFactors {
            customer_id,
            on_time_payment_rate: 0.85,
            credit_utilization: 0.4,
            avg_days_overdue: 5.0,
            customer_tenure_years: 3.0,
        };

        let result = engine.calculate_score(&factors);
        assert_eq!(result.risk_level, RiskLevel::Medium);
        assert!(result.numerical_score >= 60 && result.numerical_score < 80);
    }
}
