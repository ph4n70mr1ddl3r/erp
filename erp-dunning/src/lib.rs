use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DunningStatus {
    Open,
    Sent,
    Resolved,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DunningLevel {
    pub id: Uuid,
    pub level: u32,
    pub days_overdue: u32,
    pub fee_amount: f64,
    pub message_template: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DunningNotice {
    pub id: Uuid,
    pub invoice_id: Uuid,
    pub customer_id: Uuid,
    pub level: u32,
    pub status: DunningStatus,
    pub issue_date: DateTime<Utc>,
    pub fee_applied: f64,
    pub updated_at: DateTime<Utc>,
}

impl DunningNotice {
    pub fn new(invoice_id: Uuid, customer_id: Uuid, dunning_level: &DunningLevel) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            invoice_id,
            customer_id,
            level: dunning_level.level,
            status: DunningStatus::Open,
            issue_date: now,
            fee_applied: dunning_level.fee_amount,
            updated_at: now,
        }
    }

    pub fn mark_sent(&mut self) {
        if self.status == DunningStatus::Open {
            self.status = DunningStatus::Sent;
            self.updated_at = Utc::now();
        }
    }

    pub fn resolve(&mut self) {
        self.status = DunningStatus::Resolved;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dunning_notice_lifecycle() {
        let invoice_id = Uuid::new_v4();
        let customer_id = Uuid::new_v4();
        
        let level1 = DunningLevel {
            id: Uuid::new_v4(),
            level: 1,
            days_overdue: 15,
            fee_amount: 10.0,
            message_template: "Your invoice is 15 days overdue. Please remit payment.".to_string(),
        };

        let mut notice = DunningNotice::new(invoice_id, customer_id, &level1);
        
        assert_eq!(notice.level, 1);
        assert_eq!(notice.fee_applied, 10.0);
        assert_eq!(notice.status, DunningStatus::Open);
        
        notice.mark_sent();
        assert_eq!(notice.status, DunningStatus::Sent);
        
        notice.resolve();
        assert_eq!(notice.status, DunningStatus::Resolved);
    }
}
