use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ConsignmentStatus {
    InTransit,
    AtCustomer,
    Consumed,
    Returned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsignmentStock {
    pub id: Uuid,
    pub product_id: Uuid,
    pub customer_id: Uuid,
    pub quantity: u32,
    pub unit_cost: f64,
    pub status: ConsignmentStatus,
    pub last_audit_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

impl ConsignmentStock {
    pub fn new(product_id: Uuid, customer_id: Uuid, quantity: u32, unit_cost: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            product_id,
            customer_id,
            quantity,
            unit_cost,
            status: ConsignmentStatus::InTransit,
            last_audit_date: None,
            created_at: Utc::now(),
        }
    }

    pub fn mark_delivered(&mut self) -> Result<(), &'static str> {
        if self.status != ConsignmentStatus::InTransit {
            return Err("Stock must be in transit to be marked as delivered");
        }
        self.status = ConsignmentStatus::AtCustomer;
        Ok(())
    }

    pub fn consume(&mut self, quantity_to_consume: u32) -> Result<(), &'static str> {
        if self.status != ConsignmentStatus::AtCustomer {
            return Err("Stock must be at customer to be consumed");
        }
        if quantity_to_consume > self.quantity {
            return Err("Insufficient quantity in consignment stock");
        }
        
        self.quantity -= quantity_to_consume;
        if self.quantity == 0 {
            self.status = ConsignmentStatus::Consumed;
        }
        Ok(())
    }

    pub fn return_to_warehouse(&mut self) -> Result<(), &'static str> {
        if self.status != ConsignmentStatus::AtCustomer {
            return Err("Only stock at customer can be returned");
        }
        self.status = ConsignmentStatus::Returned;
        self.quantity = 0;
        Ok(())
    }

    pub fn audit(&mut self) {
        self.last_audit_date = Some(Utc::now());
    }

    pub fn get_total_value(&self) -> f64 {
        self.quantity as f64 * self.unit_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consignment_lifecycle() {
        let product_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let mut stock = ConsignmentStock::new(product_id, customer_id, 100, 50.0);

        assert_eq!(stock.status, ConsignmentStatus::InTransit);
        assert_eq!(stock.get_total_value(), 5000.0);

        // Mark as delivered
        stock.mark_delivered().expect("Delivery failed");
        assert_eq!(stock.status, ConsignmentStatus::AtCustomer);

        // Consume some
        stock.consume(30).expect("Consumption failed");
        assert_eq!(stock.quantity, 70);
        assert_eq!(stock.get_total_value(), 3500.0);

        // Audit
        stock.audit();
        assert!(stock.last_audit_date.is_some());

        // Return remaining
        stock.return_to_warehouse().expect("Return failed");
        assert_eq!(stock.status, ConsignmentStatus::Returned);
        assert_eq!(stock.quantity, 0);
    }

    #[test]
    fn test_insufficient_quantity() {
        let product_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let mut stock = ConsignmentStock::new(product_id, customer_id, 10, 10.0);
        stock.mark_delivered().unwrap();

        assert!(stock.consume(15).is_err());
    }

    #[test]
    fn test_full_consumption() {
        let product_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        let mut stock = ConsignmentStock::new(product_id, customer_id, 10, 10.0);
        stock.mark_delivered().unwrap();

        stock.consume(10).unwrap();
        assert_eq!(stock.status, ConsignmentStatus::Consumed);
        assert_eq!(stock.quantity, 0);
    }
}
