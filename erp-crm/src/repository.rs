use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::{Result, Pagination, Paginated};
use crate::models::{Lead, Opportunity, Contact};

#[async_trait]
pub trait LeadRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Lead>;
    async fn create(&self, pool: &SqlitePool, lead: Lead) -> Result<Lead>;
    async fn update(&self, pool: &SqlitePool, lead: Lead) -> Result<Lead>;
    async fn list(&self, pool: &SqlitePool, pagination: &Pagination) -> Result<Paginated<Lead>>;
}

#[async_trait]
pub trait OpportunityRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Opportunity>;
    async fn create(&self, pool: &SqlitePool, opportunity: Opportunity) -> Result<Opportunity>;
    async fn update(&self, pool: &SqlitePool, opportunity: Opportunity) -> Result<Opportunity>;
    async fn list(&self, pool: &SqlitePool, customer_id: Uuid, pagination: &Pagination) -> Result<Paginated<Opportunity>>;
}

#[async_trait]
pub trait ContactRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Contact>;
    async fn create(&self, pool: &SqlitePool, contact: Contact) -> Result<Contact>;
    async fn update(&self, pool: &SqlitePool, contact: Contact) -> Result<Contact>;
    async fn list(&self, pool: &SqlitePool, customer_id: Uuid, pagination: &Pagination) -> Result<Paginated<Contact>>;
}
