use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeasurementData {
    pub product_id: Uuid,
    pub process_id: Uuid,
    pub measurements: Vec<f64>,
    pub upper_spec_limit: f64,
    pub lower_spec_limit: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SpcResult {
    pub product_id: Uuid,
    pub process_id: Uuid,
    pub mean: f64,
    pub standard_deviation: f64,
    /// Process Capability (Cp) = (USL - LSL) / (6 * Sigma)
    pub cp: f64,
    /// Process Capability Index (Cpk) = min((USL - Mean)/(3*Sigma), (Mean - LSL)/(3*Sigma))
    pub cpk: f64,
    pub status: SpcStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SpcStatus {
    Capable,      // Cpk >= 1.33
    Marginal,     // 1.0 <= Cpk < 1.33
    Incapable,    // Cpk < 1.0
}

pub struct SpcEngine;

impl SpcEngine {
    /// Performs Process Capability Analysis (Cp/Cpk)
    pub fn analyze(data: &MeasurementData) -> Option<SpcResult> {
        let n = data.measurements.len();
        if n < 2 {
            return None;
        }

        // 1. Calculate Mean
        let mean: f64 = data.measurements.iter().sum::<f64>() / n as f64;

        // 2. Calculate Standard Deviation (Sigma)
        let variance = data.measurements.iter()
            .map(|m| {
                let diff = m - mean;
                diff * diff
            })
            .sum::<f64>() / (n - 1) as f64;
        
        let sigma = variance.sqrt();
        if sigma == 0.0 {
            return None;
        }

        // 3. Calculate Cp
        let cp = (data.upper_spec_limit - data.lower_spec_limit) / (6.0 * sigma);

        // 4. Calculate Cpk
        let cpu = (data.upper_spec_limit - mean) / (3.0 * sigma);
        let cpl = (mean - data.lower_spec_limit) / (3.0 * sigma);
        let cpk = cpu.min(cpl);

        let status = if cpk >= 1.33 {
            SpcStatus::Capable
        } else if cpk >= 1.0 {
            SpcStatus::Marginal
        } else {
            SpcStatus::Incapable
        };

        Some(SpcResult {
            product_id: data.product_id,
            process_id: data.process_id,
            mean,
            standard_deviation: sigma,
            cp,
            cpk,
            status,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spc_capable_process() {
        let product_id = Uuid::new_v4();
        let process_id = Uuid::new_v4();
        
        // Spec: 10.0 +/- 0.5 (USL=10.5, LSL=9.5)
        // Data is centered and tight
        let data = MeasurementData {
            product_id,
            process_id,
            measurements: vec![10.0, 10.1, 9.9, 10.0, 10.0, 10.1, 9.9, 10.0],
            upper_spec_limit: 10.5,
            lower_spec_limit: 9.5,
        };

        let result = SpcEngine::analyze(&data).unwrap();
        
        assert!(result.cpk >= 1.33);
        assert_eq!(result.status, SpcStatus::Capable);
    }

    #[test]
    fn test_spc_incapable_process() {
        let product_id = Uuid::new_v4();
        let process_id = Uuid::new_v4();
        
        // Spec: 10.0 +/- 0.5 (USL=10.5, LSL=9.5)
        // Data is spread wide
        let data = MeasurementData {
            product_id,
            process_id,
            measurements: vec![10.0, 10.6, 9.4, 10.0, 11.0, 9.0],
            upper_spec_limit: 10.5,
            lower_spec_limit: 9.5,
        };

        let result = SpcEngine::analyze(&data).unwrap();
        
        assert!(result.cpk < 1.0);
        assert_eq!(result.status, SpcStatus::Incapable);
    }

    #[test]
    fn test_spc_shifted_process() {
        let product_id = Uuid::new_v4();
        let process_id = Uuid::new_v4();
        
        // Spec: 10.0 +/- 0.5 (USL=10.5, LSL=9.5)
        // Data is tight but shifted towards USL
        let data = MeasurementData {
            product_id,
            process_id,
            measurements: vec![10.4, 10.4, 10.5, 10.4, 10.4],
            upper_spec_limit: 10.5,
            lower_spec_limit: 9.5,
        };

        let result = SpcEngine::analyze(&data).unwrap();
        
        // Cp might be high (tight spread), but Cpk will be low (off center)
        assert!(result.cp > result.cpk);
        assert!(result.cpk < 1.0);
    }
}
