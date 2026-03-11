use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationFactors {
    pub product_id: Uuid,
    /// Average daily demand
    pub avg_daily_demand: f64,
    /// Standard deviation of daily demand
    pub std_dev_demand: f64,
    /// Average lead time in days
    pub avg_lead_time_days: f64,
    /// Standard deviation of lead time in days
    pub std_dev_lead_time: f64,
    /// Desired service level (e.g., 0.95 for 95%)
    pub target_service_level: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyStockResult {
    pub product_id: Uuid,
    pub safety_stock: f64,
    pub reorder_point: f64,
    pub z_score: f64,
}

pub struct SafetyStockEngine;

impl SafetyStockEngine {
    /// Maps service level to Z-score (Simplified normal distribution lookup)
    pub fn get_z_score(service_level: f64) -> f64 {
        if service_level >= 0.999 { return 3.09; }
        if service_level >= 0.99 { return 2.33; }
        if service_level >= 0.98 { return 2.05; }
        if service_level >= 0.95 { return 1.65; }
        if service_level >= 0.90 { return 1.28; }
        if service_level >= 0.85 { return 1.04; }
        if service_level >= 0.80 { return 0.84; }
        1.0 // Default fallback
    }

    /// Calculates safety stock using the formula:
    /// SS = Z * sqrt( (Avg LT * std_dev_demand^2) + (Avg Demand^2 * std_dev_LT^2) )
    pub fn calculate(factors: &OptimizationFactors) -> SafetyStockResult {
        let z = Self::get_z_score(factors.target_service_level);
        
        let demand_variance_during_lt = factors.avg_lead_time_days * factors.std_dev_demand.powi(2);
        let lt_variance_impact = factors.avg_daily_demand.powi(2) * factors.std_dev_lead_time.powi(2);
        
        let combined_std_dev = (demand_variance_during_lt + lt_variance_impact).sqrt();
        let safety_stock = z * combined_std_dev;
        
        let reorder_point = (factors.avg_daily_demand * factors.avg_lead_time_days) + safety_stock;

        SafetyStockResult {
            product_id: factors.product_id,
            safety_stock,
            reorder_point,
            z_score: z,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safety_stock_calculation() {
        let product_id = Uuid::new_v4();
        
        // Scenario:
        // Demand: 100 units/day, std dev 10
        // Lead Time: 5 days, std dev 1
        // Service Level: 95% (Z=1.65)
        let factors = OptimizationFactors {
            product_id,
            avg_daily_demand: 100.0,
            std_dev_demand: 10.0,
            avg_lead_time_days: 5.0,
            std_dev_lead_time: 1.0,
            target_service_level: 0.95,
        };

        let result = SafetyStockEngine::calculate(&factors);

        // demand_variance_during_lt = 5 * (10^2) = 500
        // lt_variance_impact = (100^2) * (1^2) = 10000
        // combined_std_dev = sqrt(500 + 10000) = sqrt(10500) = ~102.47
        // SS = 1.65 * 102.47 = ~169.07
        assert!((result.safety_stock - 169.07).abs() < 1.0);
        
        // ROP = (100 * 5) + 169.07 = 669.07
        assert!((result.reorder_point - 669.07).abs() < 1.0);
    }

    #[test]
    fn test_zero_variability() {
        let factors = OptimizationFactors {
            product_id: Uuid::new_v4(),
            avg_daily_demand: 50.0,
            std_dev_demand: 0.0,
            avg_lead_time_days: 10.0,
            std_dev_lead_time: 0.0,
            target_service_level: 0.99,
        };

        let result = SafetyStockEngine::calculate(&factors);
        assert_eq!(result.safety_stock, 0.0);
        assert_eq!(result.reorder_point, 500.0); // Pure lead time demand
    }
}
