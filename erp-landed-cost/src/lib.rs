use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum CostType {
    Freight,
    Insurance,
    CustomsDuty,
    Taxes,
    Handling,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdditionalCost {
    pub id: Uuid,
    pub cost_type: CostType,
    pub amount: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LandedCostRecord {
    pub id: Uuid,
    pub shipment_id: Uuid,
    pub purchase_price: f64,
    pub additional_costs: Vec<AdditionalCost>,
}

impl LandedCostRecord {
    pub fn new(shipment_id: Uuid, purchase_price: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            shipment_id,
            purchase_price,
            additional_costs: Vec::new(),
        }
    }

    pub fn add_cost(&mut self, cost_type: CostType, amount: f64, description: String) {
        self.additional_costs.push(AdditionalCost {
            id: Uuid::new_v4(),
            cost_type,
            amount,
            description,
        });
    }

    pub fn total_additional_costs(&self) -> f64 {
        self.additional_costs.iter().map(|c| c.amount).sum()
    }

    pub fn total_landed_cost(&self) -> f64 {
        self.purchase_price + self.total_additional_costs()
    }

    pub fn get_cost_breakdown(&self) -> std::collections::HashMap<CostType, f64> {
        let mut breakdown = std::collections::HashMap::new();
        for cost in &self.additional_costs {
            *breakdown.entry(cost.cost_type.clone()).or_insert(0.0) += cost.amount;
        }
        breakdown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_landed_cost_calculation() {
        let shipment_id = Uuid::new_v4();
        let mut record = LandedCostRecord::new(shipment_id, 1000.0);

        record.add_cost(CostType::Freight, 150.0, "Ocean freight".to_string());
        record.add_cost(CostType::CustomsDuty, 50.0, "Import duty".to_string());
        record.add_cost(CostType::Insurance, 25.0, "Transit insurance".to_string());

        assert_eq!(record.total_additional_costs(), 225.0);
        assert_eq!(record.total_landed_cost(), 1225.0);

        let breakdown = record.get_cost_breakdown();
        assert_eq!(breakdown.get(&CostType::Freight), Some(&150.0));
        assert_eq!(breakdown.get(&CostType::CustomsDuty), Some(&50.0));
        assert_eq!(breakdown.get(&CostType::Insurance), Some(&25.0));
    }

    #[test]
    fn test_empty_landed_cost() {
        let shipment_id = Uuid::new_v4();
        let record = LandedCostRecord::new(shipment_id, 500.0);

        assert_eq!(record.total_additional_costs(), 0.0);
        assert_eq!(record.total_landed_cost(), 500.0);
        assert!(record.get_cost_breakdown().is_empty());
    }
}
