use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaseAmortizationLine {
    pub period: u32,
    pub opening_liability: f64,
    pub payment: f64,
    pub interest_expense: f64,
    pub principal_reduction: f64,
    pub closing_liability: f64,
    pub depreciation_expense: f64,
    pub net_book_value: f64,
}

pub struct LeaseValuationEngine {
    pub lease_id: Uuid,
    pub annual_discount_rate: f64,
    pub lease_term_months: u32,
    pub monthly_payment: f64,
    pub initial_direct_costs: f64,
}

impl LeaseValuationEngine {
    pub fn new(lease_id: Uuid, rate: f64, term: u32, payment: f64, costs: f64) -> Self {
        Self {
            lease_id,
            annual_discount_rate: rate,
            lease_term_months: term,
            monthly_payment: payment,
            initial_direct_costs: costs,
        }
    }

    /// Calculates the Present Value (PV) of lease payments at commencement.
    pub fn calculate_present_value(&self) -> f64 {
        let monthly_rate = self.annual_discount_rate / 12.0;
        let mut pv = 0.0;

        for t in 1..=self.lease_term_months {
            pv += self.monthly_payment / (1.0 + monthly_rate).powi(t as i32);
        }

        pv
    }

    /// Generates the full amortization schedule for the lease.
    pub fn generate_amortization_schedule(&self) -> Vec<LeaseAmortizationLine> {
        let pv_liability = self.calculate_present_value();
        let initial_rou_asset = pv_liability + self.initial_direct_costs;
        let monthly_depreciation = initial_rou_asset / self.lease_term_months as f64;
        let monthly_rate = self.annual_discount_rate / 12.0;

        let mut schedule = Vec::new();
        let mut current_liability = pv_liability;
        let mut current_nbv = initial_rou_asset;

        for period in 1..=self.lease_term_months {
            let interest = current_liability * monthly_rate;
            let principal_reduction = self.monthly_payment - interest;
            let closing_liability = current_liability - principal_reduction;
            
            let depreciation = if period == self.lease_term_months {
                current_nbv // Final adjustment to zero out NBV
            } else {
                monthly_depreciation
            };
            
            let closing_nbv = current_nbv - depreciation;

            schedule.push(LeaseAmortizationLine {
                period,
                opening_liability: current_liability,
                payment: self.monthly_payment,
                interest_expense: interest,
                principal_reduction,
                closing_liability,
                depreciation_expense: depreciation,
                net_book_value: closing_nbv,
            });

            current_liability = closing_liability;
            current_nbv = closing_nbv;
        }

        schedule
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lease_valuation_and_amortization() {
        let lease_id = Uuid::new_v4();
        // $1,000 monthly payment, 12 months, 6% annual discount rate, $500 direct costs
        let engine = LeaseValuationEngine::new(
            lease_id,
            0.06,
            12,
            1000.0,
            500.0,
        );

        let pv = engine.calculate_present_value();
        // PV of $1000 for 12 months at 0.5% monthly is ~$11,618.93
        assert!((pv - 11618.93).abs() < 1.0);

        let schedule = engine.generate_amortization_schedule();
        assert_eq!(schedule.len(), 12);

        // Final liability should be zero
        assert!((schedule[11].closing_liability).abs() < 0.01);
        
        // Final NBV should be zero
        assert!((schedule[11].net_book_value).abs() < 0.01);

        // Period 1 check
        let line1 = &schedule[0];
        assert_eq!(line1.period, 1);
        assert!((line1.interest_expense - (pv * 0.005)).abs() < 0.01);
    }
}
