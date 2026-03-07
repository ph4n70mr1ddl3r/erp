use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, ContactInfo, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LeadStatus {
    New,
    Contacted,
    Qualified,
    Lost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lead {
    pub base: BaseEntity,
    pub first_name: String,
    pub last_name: String,
    pub company_name: Option<String>,
    pub contact_info: ContactInfo,
    pub status: LeadStatus,
    pub source: Option<String>,
    pub estimated_value: Option<Money>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OpportunityStage {
    Prospecting,
    Qualification,
    NeedsAnalysis,
    ValueProposition,
    DecisionMakers,
    PerceptionAnalysis,
    ProposalPriceQuote,
    NegotiationReview,
    ClosedWon,
    ClosedLost,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Opportunity {
    pub base: BaseEntity,
    pub name: String,
    pub customer_id: Uuid,
    pub expected_close_date: Option<DateTime<Utc>>,
    pub stage: OpportunityStage,
    pub amount: Option<Money>,
    pub probability: u8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    pub base: BaseEntity,
    pub first_name: String,
    pub last_name: String,
    pub customer_id: Uuid,
    pub contact_info: ContactInfo,
    pub role: Option<String>,
}
