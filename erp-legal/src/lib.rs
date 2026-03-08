use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MatterStatus {
    Open,
    InProgress,
    OnHold,
    Settled,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MatterCategory {
    Litigation,
    Corporate,
    IntellectualProperty,
    Employment,
    Regulatory,
    Contract,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatterNote {
    pub id: Uuid,
    pub matter_id: Uuid,
    pub author_id: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MatterExpense {
    pub id: Uuid,
    pub matter_id: Uuid,
    pub date: DateTime<Utc>,
    pub description: String,
    pub amount: f64,
    pub currency: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LegalMatter {
    pub id: Uuid,
    pub case_number: String,
    pub title: String,
    pub description: String,
    pub category: MatterCategory,
    pub status: MatterStatus,
    pub internal_owner_id: Uuid, // Employee ID
    pub external_counsel: Option<String>,
    pub notes: Vec<MatterNote>,
    pub expenses: Vec<MatterExpense>,
    pub opened_at: DateTime<Utc>,
    pub closed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl LegalMatter {
    pub fn new(
        case_number: String,
        title: String,
        description: String,
        category: MatterCategory,
        internal_owner_id: Uuid,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            case_number,
            title,
            description,
            category,
            status: MatterStatus::Open,
            internal_owner_id,
            external_counsel: None,
            notes: Vec::new(),
            expenses: Vec::new(),
            opened_at: now,
            closed_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_note(&mut self, author_id: Uuid, content: String) {
        let now = Utc::now();
        self.notes.push(MatterNote {
            id: Uuid::new_v4(),
            matter_id: self.id,
            author_id,
            content,
            created_at: now,
            updated_at: now,
        });
        self.updated_at = now;
    }

    pub fn add_expense(&mut self, description: String, amount: f64, currency: String) {
        let now = Utc::now();
        self.expenses.push(MatterExpense {
            id: Uuid::new_v4(),
            matter_id: self.id,
            date: now,
            description,
            amount,
            currency,
            created_at: now,
        });
        self.updated_at = now;
    }

    pub fn update_status(&mut self, new_status: MatterStatus) {
        let now = Utc::now();
        self.status = new_status;
        if self.status == MatterStatus::Closed || self.status == MatterStatus::Settled {
            self.closed_at = Some(now);
        }
        self.updated_at = now;
    }

    pub fn total_legal_spend(&self, currency: &str) -> f64 {
        self.expenses.iter()
            .filter(|e| e.currency == currency)
            .map(|e| e.amount)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legal_matter_lifecycle() {
        let owner_id = Uuid::new_v4();
        let mut matter = LegalMatter::new(
            "LEG-2026-001".to_string(),
            "Patent Dispute: WidgetX".to_string(),
            "Litigation regarding infringement of patent XYZ.".to_string(),
            MatterCategory::IntellectualProperty,
            owner_id,
        );

        assert_eq!(matter.status, MatterStatus::Open);
        assert_eq!(matter.notes.len(), 0);

        matter.add_note(owner_id, "Met with external counsel to review the claim.".to_string());
        assert_eq!(matter.notes.len(), 1);

        matter.add_expense("Filing fees".to_string(), 500.0, "USD".to_string());
        matter.add_expense("External counsel retainer".to_string(), 5000.0, "USD".to_string());
        
        assert_eq!(matter.total_legal_spend("USD"), 5500.0);

        matter.update_status(MatterStatus::InProgress);
        assert_eq!(matter.status, MatterStatus::InProgress);

        matter.update_status(MatterStatus::Settled);
        assert_eq!(matter.status, MatterStatus::Settled);
        assert!(matter.closed_at.is_some());
    }
}
