use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub vendor_id: Uuid,
    /// Percentage of orders delivered on time (0.0 to 1.0)
    pub on_time_delivery_rate: f64,
    /// Percentage of parts that passed quality inspection (0.0 to 1.0)
    pub quality_pass_rate: f64,
    /// Price variance (Actual Price / Standard Price). Lower is better.
    /// E.g., 1.0 means no variance, 1.1 means 10% over budget.
    pub price_variance_ratio: f64,
    /// Average days to respond to inquiries
    pub avg_response_days: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScorecardConfig {
    pub delivery_weight: f64,
    pub quality_weight: f64,
    pub price_weight: f64,
    pub responsiveness_weight: f64,
}

impl Default for ScorecardConfig {
    fn default() -> Self {
        Self {
            delivery_weight: 0.40,
            quality_weight: 0.35,
            price_weight: 0.15,
            responsiveness_weight: 0.10,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorScore {
    pub vendor_id: Uuid,
    pub total_score: f64, // 0 to 100
    pub delivery_score: f64,
    pub quality_score: f64,
    pub price_score: f64,
    pub responsiveness_score: f64,
    pub classification: VendorClassification,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VendorClassification {
    StrategicPartner, // > 90
    Preferred,        // 75-90
    Conditional,      // 50-75
    AtRisk,           // < 50
}

pub struct ScorecardEngine {
    config: ScorecardConfig,
}

impl ScorecardEngine {
    pub fn new(config: ScorecardConfig) -> Self {
        Self { config }
    }

    pub fn calculate_score(&self, metrics: &PerformanceMetrics) -> VendorScore {
        // 1. Delivery (0-100)
        let delivery_score = metrics.on_time_delivery_rate * 100.0;

        // 2. Quality (0-100)
        let quality_score = metrics.quality_pass_rate * 100.0;

        // 3. Price (0-100)
        // 1.0 variance = 100 points. 1.5+ variance = 0 points.
        let price_score = (100.0 - ((metrics.price_variance_ratio - 1.0) * 200.0)).max(0.0);

        // 4. Responsiveness (0-100)
        // 0 days = 100 points. 5+ days = 0 points.
        let responsiveness_score = (100.0 - (metrics.avg_response_days * 20.0)).max(0.0);

        let total_score = (delivery_score * self.config.delivery_weight) +
                          (quality_score * self.config.quality_weight) +
                          (price_score * self.config.price_weight) +
                          (responsiveness_score * self.config.responsiveness_weight);

        let classification = if total_score >= 90.0 {
            VendorClassification::StrategicPartner
        } else if total_score >= 75.0 {
            VendorClassification::Preferred
        } else if total_score >= 50.0 {
            VendorClassification::Conditional
        } else {
            VendorClassification::AtRisk
        };

        VendorScore {
            vendor_id: metrics.vendor_id,
            total_score,
            delivery_score,
            quality_score,
            price_score,
            responsiveness_score,
            classification,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategic_vendor_score() {
        let vendor_id = Uuid::new_v4();
        let engine = ScorecardEngine::new(ScorecardConfig::default());

        let metrics = PerformanceMetrics {
            vendor_id,
            on_time_delivery_rate: 0.98,
            quality_pass_rate: 0.99,
            price_variance_ratio: 1.0,
            avg_response_days: 0.5,
        };

        let score = engine.calculate_score(&metrics);
        assert!(score.total_score >= 90.0);
        assert_eq!(score.classification, VendorClassification::StrategicPartner);
    }

    #[test]
    fn test_at_risk_vendor_score() {
        let vendor_id = Uuid::new_v4();
        let engine = ScorecardEngine::new(ScorecardConfig::default());

        let metrics = PerformanceMetrics {
            vendor_id,
            on_time_delivery_rate: 0.40,
            quality_pass_rate: 0.60,
            price_variance_ratio: 1.3,
            avg_response_days: 4.0,
        };

        let score = engine.calculate_score(&metrics);
        assert!(score.total_score < 50.0);
        assert_eq!(score.classification, VendorClassification::AtRisk);
    }
}
