use crate::models::*;
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait CpqRepository: Send + Sync {
    async fn create_template(&self, template: &ConfigurationTemplate) -> Result<()>;
    async fn get_template(&self, id: uuid::Uuid) -> Result<Option<ConfigurationTemplate>>;
    async fn list_templates(&self) -> Result<Vec<ConfigurationTemplate>>;
    
    async fn create_configuration(&self, config: &ProductConfiguration) -> Result<()>;
    async fn get_configuration(&self, id: uuid::Uuid) -> Result<Option<ProductConfiguration>>;
    async fn list_configurations(&self) -> Result<Vec<ProductConfiguration>>;
    
    async fn create_configured_quote(&self, quote: &ConfiguredQuote) -> Result<()>;
    async fn get_configured_quote(&self, id: uuid::Uuid) -> Result<Option<ConfiguredQuote>>;
    async fn list_configured_quotes(&self) -> Result<Vec<ConfiguredQuote>>;
}

pub struct SqliteCpqRepository;

impl Default for SqliteCpqRepository {
    fn default() -> Self {
        Self::new()
    }
}

impl SqliteCpqRepository {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl CpqRepository for SqliteCpqRepository {
    async fn create_template(&self, _template: &ConfigurationTemplate) -> Result<()> {
        Ok(())
    }
    
    async fn get_template(&self, _id: uuid::Uuid) -> Result<Option<ConfigurationTemplate>> {
        Ok(None)
    }
    
    async fn list_templates(&self) -> Result<Vec<ConfigurationTemplate>> {
        Ok(Vec::new())
    }
    
    async fn create_configuration(&self, _config: &ProductConfiguration) -> Result<()> {
        Ok(())
    }
    
    async fn get_configuration(&self, _id: uuid::Uuid) -> Result<Option<ProductConfiguration>> {
        Ok(None)
    }
    
    async fn list_configurations(&self) -> Result<Vec<ProductConfiguration>> {
        Ok(Vec::new())
    }
    
    async fn create_configured_quote(&self, _quote: &ConfiguredQuote) -> Result<()> {
        Ok(())
    }
    
    async fn get_configured_quote(&self, _id: uuid::Uuid) -> Result<Option<ConfiguredQuote>> {
        Ok(None)
    }
    
    async fn list_configured_quotes(&self) -> Result<Vec<ConfiguredQuote>> {
        Ok(Vec::new())
    }
}
