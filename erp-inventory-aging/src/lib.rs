use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AgingBucket {
    Current,      // 0-30 days
    OneMonth,     // 31-60 days
    TwoMonths,    // 61-90 days
    ThreeMonths,  // 91-180 days
    SixMonths,    // 181-365 days
    Obsolete,     // > 365 days
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockItem {
    pub product_id: Uuid,
    pub quantity: i64,
    pub unit_cost: f64,
    pub last_received_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgingReportLine {
    pub bucket: AgingBucket,
    pub total_quantity: i64,
    pub total_value: f64,
}

pub struct InventoryAgingEngine;

impl InventoryAgingEngine {
    pub fn get_bucket(as_of: DateTime<Utc>, receipt_date: DateTime<Utc>) -> AgingBucket {
        let days = (as_of - receipt_date).num_days();
        
        if days <= 30 {
            AgingBucket::Current
        } else if days <= 60 {
            AgingBucket::OneMonth
        } else if days <= 90 {
            AgingBucket::TwoMonths
        } else if days <= 180 {
            AgingBucket::ThreeMonths
        } else if days <= 365 {
            AgingBucket::SixMonths
        } else {
            AgingBucket::Obsolete
        }
    }

    pub fn generate_report(as_of: DateTime<Utc>, items: &[StockItem]) -> Vec<AgingReportLine> {
        let mut buckets = std::collections::HashMap::new();

        for item in items {
            let bucket = Self::get_bucket(as_of, item.last_received_at);
            let (q, v) = buckets.entry(bucket).or_insert((0i64, 0.0f64));
            *q += item.quantity;
            *v += item.quantity as f64 * item.unit_cost;
        }

        let mut results = Vec::new();
        for bucket in [
            AgingBucket::Current,
            AgingBucket::OneMonth,
            AgingBucket::TwoMonths,
            AgingBucket::ThreeMonths,
            AgingBucket::SixMonths,
            AgingBucket::Obsolete,
        ] {
            let (q, v) = buckets.get(&bucket).cloned().unwrap_or((0, 0.0));
            results.push(AgingReportLine {
                bucket,
                total_quantity: q,
                total_value: v,
            });
        }

        results
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_aging_buckets() {
        let now = Utc::now();
        
        assert_eq!(InventoryAgingEngine::get_bucket(now, now - Duration::days(10)), AgingBucket::Current);
        assert_eq!(InventoryAgingEngine::get_bucket(now, now - Duration::days(45)), AgingBucket::OneMonth);
        assert_eq!(InventoryAgingEngine::get_bucket(now, now - Duration::days(400)), AgingBucket::Obsolete);
    }

    #[test]
    fn test_aging_report() {
        let now = Utc::now();
        let p1 = Uuid::new_v4();
        let p2 = Uuid::new_v4();

        let items = vec![
            StockItem {
                product_id: p1,
                quantity: 100,
                unit_cost: 10.0,
                last_received_at: now - Duration::days(5), // Current
            },
            StockItem {
                product_id: p2,
                quantity: 50,
                unit_cost: 20.0,
                last_received_at: now - Duration::days(100), // 3 months
            },
        ];

        let report = InventoryAgingEngine::generate_report(now, &items);

        assert_eq!(report.len(), 6);
        
        let current = report.iter().find(|r| r.bucket == AgingBucket::Current).unwrap();
        assert_eq!(current.total_quantity, 100);
        assert_eq!(current.total_value, 1000.0);

        let three_months = report.iter().find(|r| r.bucket == AgingBucket::ThreeMonths).unwrap();
        assert_eq!(three_months.total_quantity, 50);
        assert_eq!(three_months.total_value, 1000.0);
    }
}
