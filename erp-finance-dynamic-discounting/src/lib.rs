use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountSchedule {
    pub id: Uuid,
    /// Maximum discount percentage (e.g., 0.02 for 2%)
    pub max_discount_rate: f64,
    /// Number of days before the net due date when the max discount applies
    pub day_count_for_max_discount: i32,
    /// The net due date (e.g., Net 30)
    pub net_due_days: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoice {
    pub id: Uuid,
    pub amount: i64,
    pub invoice_date: DateTime<Utc>,
    pub due_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscountOffer {
    pub invoice_id: Uuid,
    pub payment_date: DateTime<Utc>,
    pub discount_rate: f64,
    pub discount_amount: i64,
    pub payment_amount: i64,
    pub apr_equivalent: f64,
}

pub struct DynamicDiscountEngine;

impl DynamicDiscountEngine {
    /// Calculates the dynamic discount available for a specific payment date.
    /// The discount is linear: max discount at `day_count_for_max_discount` days or more, 
    /// and 0% at the net due date.
    pub fn calculate_discount(
        invoice: &Invoice,
        schedule: &DiscountSchedule,
        payment_date: DateTime<Utc>,
    ) -> Option<DiscountOffer> {
        if payment_date >= invoice.due_date {
            return None;
        }

        let days_early = (invoice.due_date - payment_date).num_days() as i32;
        if days_early <= 0 {
            return None;
        }

        // Linear interpolation:
        // If days_early >= schedule.net_due_days, use max_discount_rate
        // Otherwise, scale linearly down to 0 at the due date.
        let applicable_rate = if days_early >= schedule.net_due_days {
            schedule.max_discount_rate
        } else {
            (days_early as f64 / schedule.net_due_days as f64) * schedule.max_discount_rate
        };

        let discount_amount = (invoice.amount as f64 * applicable_rate) as i64;
        let payment_amount = invoice.amount - discount_amount;

        // Annual Percentage Rate (APR) equivalent for the buyer
        // Formula: (Discount% / (1 - Discount%)) * (365 / Days Early)
        let apr = if applicable_rate > 0.0 && applicable_rate < 1.0 {
            (applicable_rate / (1.0 - applicable_rate)) * (365.0 / days_early as f64)
        } else {
            0.0
        };

        Some(DiscountOffer {
            invoice_id: invoice.id,
            payment_date,
            discount_rate: applicable_rate,
            discount_amount,
            payment_amount,
            apr_equivalent: apr,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_dynamic_discount_calculation() {
        let invoice_id = Uuid::new_v4();
        let now = Utc::now();
        let due_date = now + Duration::days(30);

        let invoice = Invoice {
            id: invoice_id,
            amount: 100000, // $1,000.00
            invoice_date: now,
            due_date,
        };

        let schedule = DiscountSchedule {
            id: Uuid::new_v4(),
            max_discount_rate: 0.02, // 2%
            day_count_for_max_discount: 10,
            net_due_days: 30,
        };

        // 1. Pay today (30 days early)
        // Should get the full 2% discount
        let offer1 = DynamicDiscountEngine::calculate_discount(&invoice, &schedule, now).unwrap();
        assert_eq!(offer1.discount_rate, 0.02);
        assert_eq!(offer1.discount_amount, 2000);
        assert_eq!(offer1.payment_amount, 98000);
        // APR = (0.02 / 0.98) * (365 / 30) = 0.0204 * 12.16 = ~24.8%
        assert!((offer1.apr_equivalent - 0.248).abs() < 0.01);

        // 2. Pay 15 days early (halfway)
        // Should get 1% discount (linear scale)
        let mid_payment = now + Duration::days(15);
        let offer2 = DynamicDiscountEngine::calculate_discount(&invoice, &schedule, mid_payment).unwrap();
        assert_eq!(offer2.discount_rate, 0.01);
        assert_eq!(offer2.discount_amount, 1000);

        // 3. Pay on due date
        // Should get no discount
        let offer3 = DynamicDiscountEngine::calculate_discount(&invoice, &schedule, due_date);
        assert!(offer3.is_none());
    }
}
