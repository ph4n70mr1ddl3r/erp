use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SupplyDemandType {
    OnHand,
    PurchaseOrder,
    ProductionOrder,
    SalesOrder,
    TransferOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryEvent {
    pub id: Uuid,
    pub product_id: Uuid,
    pub event_type: SupplyDemandType,
    pub quantity: i64, // Positive for supply, negative for demand
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtpCheckRequest {
    pub product_id: Uuid,
    pub requested_quantity: i64,
    pub requested_date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtpCheckResult {
    pub available_quantity: i64,
    pub is_fulfilled: bool,
    pub suggested_date: Option<DateTime<Utc>>,
}

pub struct AtpEngine {
    events: Vec<InventoryEvent>,
}

impl AtpEngine {
    pub fn new(events: Vec<InventoryEvent>) -> Self {
        let mut sorted_events = events;
        // Sort events by date to calculate cumulative availability
        sorted_events.sort_by(|a, b| a.date.cmp(&b.date));
        Self { events: sorted_events }
    }

    /// Checks if the requested quantity is available on the requested date.
    /// If not, suggests the earliest date when it will be available.
    pub fn check_availability(&self, request: AtpCheckRequest) -> AtpCheckResult {
        let mut cumulative_qty = 0;
        let mut available_on_requested_date = 0;
        let mut suggested_date = None;

        for event in &self.events {
            if event.product_id != request.product_id {
                continue;
            }

            cumulative_qty += event.quantity;

            if event.date <= request.requested_date {
                available_on_requested_date = cumulative_qty;
            }

            if suggested_date.is_none() && cumulative_qty >= request.requested_quantity {
                suggested_date = Some(event.date);
            }
        }

        let is_fulfilled = available_on_requested_date >= request.requested_quantity;

        AtpCheckResult {
            available_quantity: available_on_requested_date,
            is_fulfilled,
            suggested_date: if is_fulfilled { None } else { suggested_date },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_atp_basic_check() {
        let product_id = Uuid::new_v4();
        let now = Utc::now();
        let tomorrow = now + chrono::Duration::days(1);
        let next_week = now + chrono::Duration::days(7);

        let events = vec![
            InventoryEvent {
                id: Uuid::new_v4(),
                product_id,
                event_type: SupplyDemandType::OnHand,
                quantity: 10,
                date: now,
            },
            InventoryEvent {
                id: Uuid::new_v4(),
                product_id,
                event_type: SupplyDemandType::PurchaseOrder,
                quantity: 50,
                date: next_week,
            },
            InventoryEvent {
                id: Uuid::new_v4(),
                product_id,
                event_type: SupplyDemandType::SalesOrder,
                quantity: -5,
                date: tomorrow,
            },
        ];

        let engine = AtpEngine::new(events);

        // Check availability for tomorrow (Should have 10 - 5 = 5)
        let res1 = engine.check_availability(AtpCheckRequest {
            product_id,
            requested_quantity: 5,
            requested_date: tomorrow,
        });
        assert!(res1.is_fulfilled);
        assert_eq!(res1.available_quantity, 5);

        // Check availability for tomorrow for 20 (Should fail, only 5 available)
        let res2 = engine.check_availability(AtpCheckRequest {
            product_id,
            requested_quantity: 20,
            requested_date: tomorrow,
        });
        assert!(!res2.is_fulfilled);
        assert_eq!(res2.suggested_date, Some(next_week));
    }

    #[test]
    fn test_atp_cumulative_demand() {
        let product_id = Uuid::new_v4();
        let d1 = Utc.with_ymd_and_hms(2026, 3, 10, 0, 0, 0).unwrap();
        let d2 = d1 + chrono::Duration::days(1);
        let d3 = d1 + chrono::Duration::days(2);

        let events = vec![
            InventoryEvent { id: Uuid::new_v4(), product_id, event_type: SupplyDemandType::OnHand, quantity: 100, date: d1 },
            InventoryEvent { id: Uuid::new_v4(), product_id, event_type: SupplyDemandType::SalesOrder, quantity: -60, date: d2 },
            InventoryEvent { id: Uuid::new_v4(), product_id, event_type: SupplyDemandType::SalesOrder, quantity: -50, date: d3 },
        ];

        let engine = AtpEngine::new(events);

        // On day 3, we have 100 - 60 - 50 = -10 (Shortage!)
        let res = engine.check_availability(AtpCheckRequest {
            product_id,
            requested_quantity: 1,
            requested_date: d3,
        });
        assert!(!res.is_fulfilled);
        assert_eq!(res.available_quantity, -10);
    }
}
