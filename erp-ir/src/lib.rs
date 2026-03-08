use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InvestorType {
    Individual,
    Institutional,
    VentureCapital,
    PrivateEquity,
    Employee,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Investor {
    pub id: Uuid,
    pub name: String,
    pub investor_type: InvestorType,
    pub tax_id: Option<String>,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct StockClass {
    pub id: Uuid,
    pub name: String, // e.g., "Class A Common", "Preferred Series B"
    pub symbol: String,
    pub par_value: f64,
    pub voting_rights_per_share: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Shareholding {
    pub id: Uuid,
    pub investor_id: Uuid,
    pub stock_class_id: Uuid,
    pub quantity: u64,
    pub acquired_date: NaiveDate,
    pub cost_basis: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DividendDeclaration {
    pub id: Uuid,
    pub stock_class_id: Uuid,
    pub amount_per_share: f64,
    pub record_date: NaiveDate,
    pub payment_date: NaiveDate,
    pub currency: String,
    pub status: String, // e.g., "Declared", "Paid"
}

impl Investor {
    pub fn new(name: String, investor_type: InvestorType, email: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            investor_type,
            tax_id: None,
            email,
            created_at: Utc::now(),
        }
    }
}

pub struct InvestorRelationsService {}

impl InvestorRelationsService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate_total_shares(&self, holdings: &[Shareholding], stock_class_id: Uuid) -> u64 {
        holdings.iter()
            .filter(|h| h.stock_class_id == stock_class_id)
            .map(|h| h.quantity)
            .sum()
    }

    pub fn calculate_dividend_payout(&self, holding: &Shareholding, declaration: &DividendDeclaration) -> f64 {
        if holding.stock_class_id == declaration.stock_class_id {
            holding.quantity as f64 * declaration.amount_per_share
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_investor_creation() {
        let investor = Investor::new(
            "Global Wealth Fund".to_string(),
            InvestorType::Institutional,
            "ir@globalwealth.com".to_string(),
        );
        assert_eq!(investor.investor_type, InvestorType::Institutional);
    }

    #[test]
    fn test_dividend_calculation() {
        let stock_class_id = Uuid::new_v4();
        let investor_id = Uuid::new_v4();
        
        let holding = Shareholding {
            id: Uuid::new_v4(),
            investor_id,
            stock_class_id,
            quantity: 1000,
            acquired_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
            cost_basis: 10.0,
        };

        let declaration = DividendDeclaration {
            id: Uuid::new_v4(),
            stock_class_id,
            amount_per_share: 0.25,
            record_date: NaiveDate::from_ymd_opt(2026, 3, 1).unwrap(),
            payment_date: NaiveDate::from_ymd_opt(2026, 3, 15).unwrap(),
            currency: "USD".to_string(),
            status: "Declared".to_string(),
        };

        let service = InvestorRelationsService::new();
        let payout = service.calculate_dividend_payout(&holding, &declaration);
        
        assert_eq!(payout, 250.0); // 1000 * 0.25
    }

    #[test]
    fn test_total_shares_calculation() {
        let class_a = Uuid::new_v4();
        let class_b = Uuid::new_v4();
        let inv_id = Uuid::new_v4();

        let holdings = vec![
            Shareholding {
                id: Uuid::new_v4(),
                investor_id: inv_id,
                stock_class_id: class_a,
                quantity: 500,
                acquired_date: NaiveDate::from_ymd_opt(2025, 1, 1).unwrap(),
                cost_basis: 5.0,
            },
            Shareholding {
                id: Uuid::new_v4(),
                investor_id: inv_id,
                stock_class_id: class_a,
                quantity: 300,
                acquired_date: NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
                cost_basis: 6.0,
            },
            Shareholding {
                id: Uuid::new_v4(),
                investor_id: inv_id,
                stock_class_id: class_b,
                quantity: 100,
                acquired_date: NaiveDate::from_ymd_opt(2025, 6, 1).unwrap(),
                cost_basis: 100.0,
            },
        ];

        let service = InvestorRelationsService::new();
        assert_eq!(service.calculate_total_shares(&holdings, class_a), 800);
        assert_eq!(service.calculate_total_shares(&holdings, class_b), 100);
    }
}
