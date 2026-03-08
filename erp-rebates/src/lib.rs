use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RebateType {
    Customer,
    Vendor,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CalculationMethod {
    PercentageOfSales,
    FixedAmountPerUnit,
    FlatAmount,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RebateTier {
    pub id: Uuid,
    pub agreement_id: Uuid,
    pub threshold_value: f64, // e.g. Min sales volume/amount
    pub rebate_value: f64,    // e.g. Percentage or amount
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AgreementStatus {
    Draft,
    Active,
    Expired,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RebateAgreement {
    pub id: Uuid,
    pub title: String,
    pub partner_id: Uuid, // Customer or Vendor ID
    pub rebate_type: RebateType,
    pub method: CalculationMethod,
    pub status: AgreementStatus,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub tiers: Vec<RebateTier>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ClaimStatus {
    Pending,
    Calculated,
    Approved,
    Settled,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RebateClaim {
    pub id: Uuid,
    pub agreement_id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub total_base_value: f64, // Total sales/purchases in period
    pub calculated_rebate_amount: f64,
    pub status: ClaimStatus,
    pub approved_by: Option<Uuid>,
    pub settled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl RebateAgreement {
    pub fn new(
        title: String,
        partner_id: Uuid,
        rebate_type: RebateType,
        method: CalculationMethod,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            partner_id,
            rebate_type,
            method,
            status: AgreementStatus::Draft,
            start_date,
            end_date,
            tiers: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_tier(&mut self, threshold: f64, value: f64) {
        self.tiers.push(RebateTier {
            id: Uuid::new_v4(),
            agreement_id: self.id,
            threshold_value: threshold,
            rebate_value: value,
        });
        // Keep tiers sorted by threshold
        self.tiers.sort_by(|a, b| a.threshold_value.partial_cmp(&b.threshold_value).unwrap());
        self.updated_at = Utc::now();
    }

    pub fn calculate_rebate(&self, total_value: f64) -> f64 {
        if self.status != AgreementStatus::Active {
            return 0.0;
        }

        // Find the highest tier met
        let active_tier = self.tiers.iter().rev().find(|t| total_value >= t.threshold_value);

        match (active_tier, &self.method) {
            (Some(tier), CalculationMethod::PercentageOfSales) => {
                (total_value * tier.rebate_value) / 100.0
            }
            (Some(tier), CalculationMethod::FixedAmountPerUnit) => {
                // In this simplified model, total_value is assumed to be quantity if method is PerUnit
                total_value * tier.rebate_value
            }
            (Some(tier), CalculationMethod::FlatAmount) => {
                tier.rebate_value
            }
            _ => 0.0,
        }
    }

    pub fn activate(&mut self) {
        self.status = AgreementStatus::Active;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rebate_calculation_flow() {
        let partner_id = Uuid::new_v4();
        let start = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();
        
        let mut agreement = RebateAgreement::new(
            "2026 Volume Rebate".to_string(),
            partner_id,
            RebateType::Customer,
            CalculationMethod::PercentageOfSales,
            start,
            end,
        );

        // Tier 1: > $10,000 -> 2%
        agreement.add_tier(10000.0, 2.0);
        // Tier 2: > $50,000 -> 5%
        agreement.add_tier(50000.0, 5.0);

        // Calculation should be 0 while in Draft
        assert_eq!(agreement.calculate_rebate(60000.0), 0.0);

        agreement.activate();

        // Under first tier
        assert_eq!(agreement.calculate_rebate(5000.0), 0.0);

        // Tier 1
        assert_eq!(agreement.calculate_rebate(20000.0), 400.0); // 2% of 20000

        // Tier 2
        assert_eq!(agreement.calculate_rebate(100000.0), 5000.0); // 5% of 100000
    }

    #[test]
    fn test_fixed_amount_rebate() {
        let partner_id = Uuid::new_v4();
        let mut agreement = RebateAgreement::new(
            "Fixed Rebate".to_string(),
            partner_id,
            RebateType::Vendor,
            CalculationMethod::FlatAmount,
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 12, 31).unwrap(),
        );

        agreement.add_tier(1000.0, 500.0);
        agreement.activate();

        assert_eq!(agreement.calculate_rebate(1500.0), 500.0);
        assert_eq!(agreement.calculate_rebate(500.0), 0.0);
    }
}
