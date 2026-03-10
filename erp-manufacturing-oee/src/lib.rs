use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OeeResult {
    pub machine_id: Uuid,
    pub availability: f64,
    pub performance: f64,
    pub quality: f64,
    pub oee: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionData {
    pub machine_id: Uuid,
    /// Total time the machine was planned to run (e.g., in minutes)
    pub planned_production_time: f64,
    /// Actual time the machine was running (excluding downtime)
    pub run_time: f64,
    /// Total parts produced during the run time
    pub total_count: u32,
    /// Number of good parts produced
    pub good_count: u32,
    /// The fastest possible time to produce one part (e.g., in minutes/part)
    pub ideal_cycle_time: f64,
}

pub struct OeeCalculator;

impl OeeCalculator {
    /// Calculates OEE based on availability, performance, and quality.
    /// OEE = Availability * Performance * Quality
    pub fn calculate(data: &ProductionData) -> OeeResult {
        // 1. Availability = Run Time / Planned Production Time
        let availability = if data.planned_production_time > 0.0 {
            data.run_time / data.planned_production_time
        } else {
            0.0
        };

        // 2. Performance = (Ideal Cycle Time * Total Count) / Run Time
        let performance = if data.run_time > 0.0 {
            (data.ideal_cycle_time * data.total_count as f64) / data.run_time
        } else {
            0.0
        };

        // 3. Quality = Good Count / Total Count
        let quality = if data.total_count > 0 {
            data.good_count as f64 / data.total_count as f64
        } else {
            0.0
        };

        let oee = availability * performance * quality;

        OeeResult {
            machine_id: data.machine_id,
            availability: availability.clamp(0.0, 1.0),
            performance: performance.clamp(0.0, 1.0),
            quality: quality.clamp(0.0, 1.0),
            oee: oee.clamp(0.0, 1.0),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oee_calculation() {
        let machine_id = Uuid::new_v4();
        
        // Scenario:
        // Shift: 8 hours (480 min)
        // Breaks: 2 x 15 min (30 min)
        // Planned Production Time: 450 min
        // Downtime: 50 min
        // Run Time: 400 min
        // Total Count: 3000 parts
        // Good Count: 2910 parts
        // Ideal Cycle Time: 0.1 min/part (600 parts/hour)
        
        let data = ProductionData {
            machine_id,
            planned_production_time: 450.0,
            run_time: 400.0,
            total_count: 3000,
            good_count: 2910,
            ideal_cycle_time: 0.1,
        };

        let result = OeeCalculator::calculate(&data);

        // Availability = 400 / 450 = 0.8888...
        assert!((result.availability - 0.88888888).abs() < 1e-6);

        // Performance = (0.1 * 3000) / 400 = 300 / 400 = 0.75
        assert_eq!(result.performance, 0.75);

        // Quality = 2910 / 3000 = 0.97
        assert_eq!(result.quality, 0.97);

        // OEE = 0.8888 * 0.75 * 0.97 = 0.6466...
        assert!((result.oee - 0.64666666).abs() < 1e-6);
    }

    #[test]
    fn test_perfect_oee() {
        let machine_id = Uuid::new_v4();
        let data = ProductionData {
            machine_id,
            planned_production_time: 100.0,
            run_time: 100.0,
            total_count: 100,
            good_count: 100,
            ideal_cycle_time: 1.0,
        };

        let result = OeeCalculator::calculate(&data);
        assert_eq!(result.availability, 1.0);
        assert_eq!(result.performance, 1.0);
        assert_eq!(result.quality, 1.0);
        assert_eq!(result.oee, 1.0);
    }
}
