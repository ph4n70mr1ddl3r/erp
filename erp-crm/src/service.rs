use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::{Result, Pagination, Paginated};
use crate::models::{Lead, Opportunity, Contact};
use crate::repository::{LeadRepository, OpportunityRepository, ContactRepository};
use std::sync::Arc;

pub struct CrmService {
    pool: SqlitePool,
    lead_repo: Arc<dyn LeadRepository>,
    opportunity_repo: Arc<dyn OpportunityRepository>,
    contact_repo: Arc<dyn ContactRepository>,
}

impl CrmService {
    pub fn new(
        pool: SqlitePool,
        lead_repo: Arc<dyn LeadRepository>,
        opportunity_repo: Arc<dyn OpportunityRepository>,
        contact_repo: Arc<dyn ContactRepository>,
    ) -> Self {
        Self {
            pool,
            lead_repo,
            opportunity_repo,
            contact_repo,
        }
    }

    pub async fn create_lead(&self, lead: Lead) -> Result<Lead> {
        self.lead_repo.create(&self.pool, lead).await
    }

    pub async fn get_lead(&self, id: Uuid) -> Result<Lead> {
        self.lead_repo.find_by_id(&self.pool, id).await
    }

    pub async fn update_lead(&self, lead: Lead) -> Result<Lead> {
        self.lead_repo.update(&self.pool, lead).await
    }

    pub async fn list_leads(&self, pagination: &Pagination) -> Result<Paginated<Lead>> {
        self.lead_repo.list(&self.pool, pagination).await
    }

    pub async fn create_opportunity(&self, opp: Opportunity) -> Result<Opportunity> {
        self.opportunity_repo.create(&self.pool, opp).await
    }

    pub async fn get_opportunity(&self, id: Uuid) -> Result<Opportunity> {
        self.opportunity_repo.find_by_id(&self.pool, id).await
    }

    pub async fn list_opportunities(&self, customer_id: Uuid, pagination: &Pagination) -> Result<Paginated<Opportunity>> {
        self.opportunity_repo.list(&self.pool, customer_id, pagination).await
    }

    pub async fn create_contact(&self, contact: Contact) -> Result<Contact> {
        self.contact_repo.create(&self.pool, contact).await
    }

    pub async fn get_contact(&self, id: Uuid) -> Result<Contact> {
        self.contact_repo.find_by_id(&self.pool, id).await
    }
}
