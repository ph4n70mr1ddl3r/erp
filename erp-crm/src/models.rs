use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LeadStatus {
    New,
    Contacted,
    Qualified,
    Lost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lead {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub company: String,
    pub email: String,
    pub status: LeadStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Lead {
    pub fn new(first_name: &str, last_name: &str, company: &str, email: &str) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            company: company.to_string(),
            email: email.to_string(),
            status: LeadStatus::New,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn convert_to_contact(&self) -> Result<Contact, &'static str> {
        if self.status != LeadStatus::Qualified {
            return Err("Lead must be qualified before conversion to a contact.");
        }
        Ok(Contact::new(&self.first_name, &self.last_name, &self.email, &self.company, self.id))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub company: String,
    pub lead_source_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Contact {
    pub fn new(first_name: &str, last_name: &str, email: &str, company: &str, lead_source_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            email: email.to_string(),
            company: company.to_string(),
            lead_source_id: Some(lead_source_id),
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OpportunityStage {
    Prospecting,
    Proposal,
    Negotiation,
    ClosedWon,
    ClosedLost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opportunity {
    pub id: Uuid,
    pub contact_id: Uuid,
    pub name: String,
    pub amount: f64,
    pub stage: OpportunityStage,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Opportunity {
    pub fn new(contact_id: Uuid, name: &str, amount: f64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            contact_id,
            name: name.to_string(),
            amount,
            stage: OpportunityStage::Prospecting,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn advance_stage(&mut self, new_stage: OpportunityStage) {
        self.stage = new_stage;
        self.updated_at = Utc::now();
    }
}
