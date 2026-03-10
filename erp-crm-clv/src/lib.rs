use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomerMetrics {
    pub customer_id: Uuid,
    /// Average value of each order placed by the customer
    pub average_order_value: f64,
    /// Average number of orders placed per month
    pub purchase_frequency_per_month: f64,
    /// Average lifespan of the customer relationship in months
    pub customer_lifespan_months: f64,
    /// Gross margin percentage (e.g., 0.25 for 20%)
    pub gross_margin: f64,
    /// Monthly retention rate (e.g., 0.95 for 95%)
    pub retention_rate: f64,
    /// Monthly discount rate (for Net Present Value calculation)
    pub discount_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClvResult {
    pub customer_id: Uuid,
    /// Simple CLV = (Avg Order Value * Purchase Frequency * Lifespan)
    pub simple_clv: f64,
    /// Predictive CLV = (Avg Order Value * Purchase Frequency * Gross Margin) / (Churn Rate)
    pub predictive_clv: f64,
    /// Net Present Value CLV (accounting for time value of money)
    pub npv_clv: f64,
}

pub struct ClvEngine;

impl ClvEngine {
    /// Calculates various CLV metrics for a customer.
    pub fn calculate(metrics: &CustomerMetrics) -> ClvResult {
        // 1. Simple CLV
        let simple_clv = metrics.average_order_value 
            * metrics.purchase_frequency_per_month 
            * metrics.customer_lifespan_months;

        // 2. Predictive CLV (Traditional formula)
        // Churn Rate = 1 - Retention Rate
        let churn_rate = 1.0 - metrics.retention_rate;
        let predictive_clv = if churn_rate > 0.0 {
            (metrics.average_order_value * metrics.purchase_frequency_per_month * metrics.gross_margin) / churn_rate
        } else {
            simple_clv * metrics.gross_margin // Fallback if retention is 100%
        };

        // 3. NPV CLV (Sum of discounted future cash flows)
        // Formula: Margin * (Retention / (1 + Discount - Retention))
        let monthly_margin = metrics.average_order_value * metrics.purchase_frequency_per_month * metrics.gross_margin;
        let npv_clv = if (metrics.discount_rate + (1.0 - metrics.retention_rate)) > 0.0 {
            monthly_margin * (metrics.retention_rate / (1.0 + metrics.discount_rate - metrics.retention_rate))
        } else {
            predictive_clv
        };

        ClvResult {
            customer_id: metrics.customer_id,
            simple_clv,
            predictive_clv,
            npv_clv,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clv_calculation() {
        let customer_id = Uuid::new_v4();
        
        let metrics = CustomerMetrics {
            customer_id,
            average_order_value: 100.0,
            purchase_frequency_per_month: 2.0, // 2 orders/month
            customer_lifespan_months: 24.0,    // 2 years
            gross_margin: 0.30,               // 30% margin
            retention_rate: 0.90,             // 90% monthly retention (10% churn)
            discount_rate: 0.01,              // 1% monthly discount rate
        };

        let result = ClvEngine::calculate(&metrics);

        // Simple CLV = 100 * 2 * 24 = 4800
        assert_eq!(result.simple_clv, 4800.0);

        // Predictive CLV = (100 * 2 * 0.3) / 0.1 = 60 / 0.1 = 600
        assert!((result.predictive_clv - 600.0).abs() < 1e-10);

        // NPV CLV = 60 * (0.9 / (1 + 0.01 - 0.9)) = 60 * (0.9 / 0.11) = 60 * 8.1818... = 490.909...
        assert!((result.npv_clv - 490.9090909).abs() < 1e-6);
    }

    #[test]
    fn test_high_retention_clv() {
        let customer_id = Uuid::new_v4();
        let metrics = CustomerMetrics {
            customer_id,
            average_order_value: 50.0,
            purchase_frequency_per_month: 1.0,
            customer_lifespan_months: 12.0,
            gross_margin: 0.50,
            retention_rate: 0.99, // Very loyal
            discount_rate: 0.005,
        };

        let result = ClvEngine::calculate(&metrics);
        // High retention should lead to high predictive/NPV CLV relative to lifespan
        assert!(result.predictive_clv > (result.simple_clv * metrics.gross_margin));
    }
}
