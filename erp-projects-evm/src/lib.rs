use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvmSnapshot {
    pub project_id: Uuid,
    /// Planned Value (PV) - Budgeted Cost of Work Scheduled (BCWS)
    pub planned_value: f64,
    /// Earned Value (EV) - Budgeted Cost of Work Performed (BCWP)
    pub earned_value: f64,
    /// Actual Cost (AC) - Actual Cost of Work Performed (ACWP)
    pub actual_cost: f64,
    /// Budget at Completion (BAC)
    pub budget_at_completion: f64,
}

impl EvmSnapshot {
    pub fn new(project_id: Uuid, pv: f64, ev: f64, ac: f64, bac: f64) -> Self {
        Self {
            project_id,
            planned_value: pv,
            earned_value: ev,
            actual_cost: ac,
            budget_at_completion: bac,
        }
    }

    /// Cost Variance (CV) = EV - AC
    pub fn cost_variance(&self) -> f64 {
        self.earned_value - self.actual_cost
    }

    /// Schedule Variance (SV) = EV - PV
    pub fn schedule_variance(&self) -> f64 {
        self.earned_value - self.planned_value
    }

    /// Cost Performance Index (CPI) = EV / AC
    pub fn cpi(&self) -> f64 {
        if self.actual_cost == 0.0 {
            return 1.0;
        }
        self.earned_value / self.actual_cost
    }

    /// Schedule Performance Index (SPI) = EV / PV
    pub fn spi(&self) -> f64 {
        if self.planned_value == 0.0 {
            return 1.0;
        }
        self.earned_value / self.planned_value
    }

    /// Estimate at Completion (EAC) = BAC / CPI
    pub fn estimate_at_completion(&self) -> f64 {
        let cpi = self.cpi();
        if cpi == 0.0 {
            return self.budget_at_completion;
        }
        self.budget_at_completion / cpi
    }

    /// Estimate to Complete (ETC) = EAC - AC
    pub fn estimate_to_complete(&self) -> f64 {
        self.estimate_at_completion() - self.actual_cost
    }

    /// Variance at Completion (VAC) = BAC - EAC
    pub fn variance_at_completion(&self) -> f64 {
        self.budget_at_completion - self.estimate_at_completion()
    }

    /// To-Complete Performance Index (TCPI) = (BAC - EV) / (BAC - AC)
    pub fn tcpi(&self) -> f64 {
        let remaining_work = self.budget_at_completion - self.earned_value;
        let remaining_funds = self.budget_at_completion - self.actual_cost;
        if remaining_funds <= 0.0 {
            return 0.0;
        }
        remaining_work / remaining_funds
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evm_metrics() {
        let project_id = Uuid::new_v4();
        // Project with BAC of $100,000
        // Currently:
        // - Should have done $50,000 worth of work (PV)
        // - Actually did $40,000 worth of work (EV)
        // - Spent $45,000 (AC)
        let snapshot = EvmSnapshot::new(
            project_id,
            50000.0,
            40000.0,
            45000.0,
            100000.0,
        );

        // Variance checks
        assert_eq!(snapshot.cost_variance(), -5000.0); // Over budget
        assert_eq!(snapshot.schedule_variance(), -10000.0); // Behind schedule

        // Index checks
        assert!((snapshot.cpi() - 0.8888888888888888).abs() < 1e-10);
        assert!((snapshot.spi() - 0.8).abs() < 1e-10);

        // Forecasting
        let eac = snapshot.estimate_at_completion();
        assert_eq!(eac, 112500.0); // 100000 / 0.888...
        
        let vac = snapshot.variance_at_completion();
        assert_eq!(vac, -12500.0); // Project will be $12,500 over budget
    }

    #[test]
    fn test_perfect_project() {
        let project_id = Uuid::new_v4();
        let snapshot = EvmSnapshot::new(
            project_id,
            5000.0,
            5000.0,
            5000.0,
            10000.0,
        );

        assert_eq!(snapshot.cpi(), 1.0);
        assert_eq!(snapshot.spi(), 1.0);
        assert_eq!(snapshot.estimate_at_completion(), 10000.0);
        assert_eq!(snapshot.tcpi(), 1.0);
    }
}
