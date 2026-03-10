use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum AQLValue {
    AQL0_65,
    AQL1_0,
    AQL1_5,
    AQL2_5,
    AQL4_0,
    AQL6_5,
}

impl AQLValue {
    pub fn as_f64(&self) -> f64 {
        match self {
            Self::AQL0_65 => 0.65,
            Self::AQL1_0 => 1.0,
            Self::AQL1_5 => 1.5,
            Self::AQL2_5 => 2.5,
            Self::AQL4_0 => 4.0,
            Self::AQL6_5 => 6.5,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SamplingPlan {
    pub sample_size: u32,
    pub acceptance_number: u32, // Ac
    pub rejection_number: u32,  // Re
}

/// Simplified ISO 2859-1 Table for Normal Inspection (Level II)
pub fn get_sampling_plan(lot_size: u32, aql: AQLValue) -> SamplingPlan {
    // Determine Code Letter (Simplified)
    // 2-8: A, 9-15: B, 16-25: C, 26-50: D, 51-90: E, 91-150: F, 151-280: G, 281-500: H, 501-1200: J, 1201-3200: K...
    let (sample_size, index) = if lot_size <= 8 {
        (2, 0)
    } else if lot_size <= 15 {
        (3, 1)
    } else if lot_size <= 25 {
        (5, 2)
    } else if lot_size <= 50 {
        (8, 3)
    } else if lot_size <= 90 {
        (13, 4)
    } else if lot_size <= 150 {
        (20, 5)
    } else if lot_size <= 280 {
        (32, 6)
    } else if lot_size <= 500 {
        (50, 7)
    } else if lot_size <= 1200 {
        (80, 8)
    } else if lot_size <= 3200 {
        (125, 9)
    } else if lot_size <= 10000 {
        (200, 10)
    } else {
        (315, 11)
    };

    // Acceptance/Rejection numbers (Simplified lookup for Normal Level II)
    // Rows: A-L (indices 0-11)
    // Columns: AQL levels [0.65, 1.0, 1.5, 2.5, 4.0, 6.5]
    // Values are (Ac, Re)
    let aql_matrix = [
        // AQL: 0.65, 1.0, 1.5, 2.5, 4.0, 6.5
        /* A */ [(0, 1), (0, 1), (0, 1), (0, 1), (0, 1), (1, 2)],
        /* B */ [(0, 1), (0, 1), (0, 1), (0, 1), (1, 2), (1, 2)],
        /* C */ [(0, 1), (0, 1), (0, 1), (1, 2), (1, 2), (2, 3)],
        /* D */ [(0, 1), (0, 1), (1, 2), (1, 2), (2, 3), (3, 4)],
        /* E */ [(0, 1), (1, 2), (1, 2), (2, 3), (3, 4), (5, 6)],
        /* F */ [(1, 2), (1, 2), (2, 3), (3, 4), (5, 6), (7, 8)],
        /* G */ [(1, 2), (2, 3), (3, 4), (5, 6), (7, 8), (10, 11)],
        /* H */ [(2, 3), (3, 4), (5, 6), (7, 8), (10, 11), (14, 15)],
        /* J */ [(3, 4), (5, 6), (7, 8), (10, 11), (14, 15), (21, 22)],
        /* K */ [(5, 6), (7, 8), (10, 11), (14, 15), (21, 22), (21, 22)],
        /* L */ [(7, 8), (10, 11), (14, 15), (21, 22), (21, 22), (21, 22)],
        /* M */ [(10, 11), (14, 15), (21, 22), (21, 22), (21, 22), (21, 22)],
    ];

    let aql_idx = match aql {
        AQLValue::AQL0_65 => 0,
        AQLValue::AQL1_0 => 1,
        AQLValue::AQL1_5 => 2,
        AQLValue::AQL2_5 => 3,
        AQLValue::AQL4_0 => 4,
        AQLValue::AQL6_5 => 5,
    };

    let (ac, re) = aql_matrix[index][aql_idx];

    SamplingPlan {
        sample_size: sample_size.min(lot_size),
        acceptance_number: ac,
        rejection_number: re,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspectionResult {
    pub plan: SamplingPlan,
    pub defects_found: u32,
    pub passed: bool,
}

impl InspectionResult {
    pub fn evaluate(plan: SamplingPlan, defects_found: u32) -> Self {
        let passed = defects_found <= plan.acceptance_number;
        Self {
            plan,
            defects_found,
            passed,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sampling_plan_lookup() {
        // Lot size 500, AQL 2.5 -> Sample size 50, Ac 7, Re 8 (ISO 2859-1 Normal Level II)
        let plan = get_sampling_plan(500, AQLValue::AQL2_5);
        assert_eq!(plan.sample_size, 50);
        assert_eq!(plan.acceptance_number, 7);
        assert_eq!(plan.rejection_number, 8);

        // Lot size 50, AQL 1.5 -> Sample size 8, Ac 1, Re 2
        let plan2 = get_sampling_plan(50, AQLValue::AQL1_5);
        assert_eq!(plan2.sample_size, 8);
        assert_eq!(plan2.acceptance_number, 1);
        assert_eq!(plan2.rejection_number, 2);
    }

    #[test]
    fn test_inspection_evaluation() {
        let plan = SamplingPlan {
            sample_size: 50,
            acceptance_number: 7,
            rejection_number: 8,
        };

        // Pass
        let result = InspectionResult::evaluate(plan.clone(), 5);
        assert!(result.passed);

        // Borderline Pass
        let result = InspectionResult::evaluate(plan.clone(), 7);
        assert!(result.passed);

        // Fail
        let result = InspectionResult::evaluate(plan.clone(), 8);
        assert!(!result.passed);

        // Clearly Fail
        let result = InspectionResult::evaluate(plan.clone(), 15);
        assert!(!result.passed);
    }
}
