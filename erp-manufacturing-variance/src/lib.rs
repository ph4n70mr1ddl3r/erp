use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VarianceReport {
    pub work_order_id: Uuid,
    pub material_variances: Vec<MaterialVariance>,
    pub labor_variances: Vec<LaborVariance>,
    pub total_variance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialVariance {
    pub product_id: Uuid,
    pub standard_quantity: f64,
    pub actual_quantity: f64,
    pub standard_price: f64,
    /// (Actual Quantity - Standard Quantity) * Standard Price
    pub quantity_variance: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LaborVariance {
    pub work_center_id: Uuid,
    pub standard_hours: f64,
    pub actual_hours: f64,
    pub standard_rate: f64,
    /// (Actual Hours - Standard Hours) * Standard Rate
    pub efficiency_variance: f64,
}

pub struct VarianceEngine;

impl VarianceEngine {
    /// Calculates variances for a production run.
    pub fn calculate_variances(
        work_order_id: Uuid,
        materials: Vec<MaterialActuals>,
        labor: Vec<LaborActuals>,
    ) -> VarianceReport {
        let mut material_variances = Vec::new();
        let mut labor_variances = Vec::new();
        let mut total_variance = 0.0;

        for mat in materials {
            let variance = (mat.actual_quantity - mat.standard_quantity) * mat.standard_price;
            total_variance += variance;
            material_variances.push(MaterialVariance {
                product_id: mat.product_id,
                standard_quantity: mat.standard_quantity,
                actual_quantity: mat.actual_quantity,
                standard_price: mat.standard_price,
                quantity_variance: variance,
            });
        }

        for lab in labor {
            let variance = (lab.actual_hours - lab.standard_hours) * lab.standard_rate;
            total_variance += variance;
            labor_variances.push(LaborVariance {
                work_center_id: lab.work_center_id,
                standard_hours: lab.standard_hours,
                actual_hours: lab.actual_hours,
                standard_rate: lab.standard_rate,
                efficiency_variance: variance,
            });
        }

        VarianceReport {
            work_order_id,
            material_variances,
            labor_variances,
            total_variance,
        }
    }
}

#[derive(Debug, Clone)]
pub struct MaterialActuals {
    pub product_id: Uuid,
    pub standard_quantity: f64,
    pub actual_quantity: f64,
    pub standard_price: f64,
}

#[derive(Debug, Clone)]
pub struct LaborActuals {
    pub work_center_id: Uuid,
    pub standard_hours: f64,
    pub actual_hours: f64,
    pub standard_rate: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manufacturing_variance_calculation() {
        let work_order_id = Uuid::new_v4();
        let steel_id = Uuid::new_v4();
        let assembly_id = Uuid::new_v4();

        // Scenario:
        // Material: Standard 10kg, Actual 12kg, Price $5/kg -> $10 Unfavorable Variance
        let materials = vec![
            MaterialActuals {
                product_id: steel_id,
                standard_quantity: 10.0,
                actual_quantity: 12.0,
                standard_price: 5.0,
            }
        ];

        // Labor: Standard 5h, Actual 4h, Rate $20/h -> -$20 Favorable Variance
        let labor = vec![
            LaborActuals {
                work_center_id: assembly_id,
                standard_hours: 5.0,
                actual_hours: 4.0,
                standard_rate: 20.0,
            }
        ];

        let report = VarianceEngine::calculate_variances(work_order_id, materials, labor);

        assert_eq!(report.material_variances[0].quantity_variance, 10.0);
        assert_eq!(report.labor_variances[0].efficiency_variance, -20.0);
        assert_eq!(report.total_variance, -10.0); // Overall favorable
    }

    #[test]
    fn test_zero_variance() {
        let work_order_id = Uuid::new_v4();
        let materials = vec![
            MaterialActuals {
                product_id: Uuid::new_v4(),
                standard_quantity: 100.0,
                actual_quantity: 100.0,
                standard_price: 1.0,
            }
        ];
        let labor = vec![];

        let report = VarianceEngine::calculate_variances(work_order_id, materials, labor);
        assert_eq!(report.total_variance, 0.0);
    }
}
