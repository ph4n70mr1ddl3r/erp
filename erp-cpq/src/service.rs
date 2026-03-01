use crate::models::*;
use crate::repository::*;
use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;

pub struct CpqService {
    repo: SqliteCpqRepository,
}

impl Default for CpqService {
    fn default() -> Self {
        Self::new()
    }
}

impl CpqService {
    pub fn new() -> Self {
        Self {
            repo: SqliteCpqRepository::new(),
        }
    }
    
    pub async fn create_template(&self, _pool: &SqlitePool, template: ConfigurationTemplate) -> Result<ConfigurationTemplate> {
        self.repo.create_template(&template).await?;
        Ok(template)
    }
    
    pub async fn get_template(&self, _pool: &SqlitePool, id: Uuid) -> Result<Option<ConfigurationTemplate>> {
        self.repo.get_template(id).await
    }
    
    pub async fn list_templates(&self, _pool: &SqlitePool) -> Result<Vec<ConfigurationTemplate>> {
        self.repo.list_templates().await
    }
    
    pub async fn create_configuration(&self, _pool: &SqlitePool, mut config: ProductConfiguration) -> Result<ProductConfiguration> {
        config.id = Uuid::new_v4();
        config.configuration_number = format!("CFG-{}", chrono::Utc::now().format("%Y%m%d%H%M%S"));
        config.created_at = Utc::now();
        config.updated_at = Utc::now();
        
        let pricing_result = self.calculate_configuration_price(&config).await?;
        config.configured_price = pricing_result.configured_price;
        config.is_valid = pricing_result.warnings.is_empty();
        
        self.repo.create_configuration(&config).await?;
        Ok(config)
    }
    
    pub async fn get_configuration(&self, _pool: &SqlitePool, id: Uuid) -> Result<Option<ProductConfiguration>> {
        self.repo.get_configuration(id).await
    }
    
    pub async fn list_configurations(&self, _pool: &SqlitePool) -> Result<Vec<ProductConfiguration>> {
        self.repo.list_configurations().await
    }
    
    pub async fn calculate_configuration_price(&self, config: &ProductConfiguration) -> Result<ConfigurationPricingResult> {
        let configured_price = config.base_price;
        let mut price_breakdown = Vec::new();
        let applied_rules = Vec::new();
        let warnings = Vec::new();
        
        price_breakdown.push(PriceBreakdownItem {
            name: "Base Price".to_string(),
            description: "Starting configuration price".to_string(),
            amount: config.base_price,
        });
        
        Ok(ConfigurationPricingResult {
            base_price: config.base_price,
            configured_price,
            price_breakdown,
            applied_rules,
            warnings,
        })
    }
    
    pub async fn validate_configuration(&self, _config: &ProductConfiguration) -> Result<Vec<String>> {
        let errors = Vec::new();
        Ok(errors)
    }
    
    pub async fn create_quote(&self, _pool: &SqlitePool, quote: ConfiguredQuote) -> Result<ConfiguredQuote> {
        self.repo.create_configured_quote(&quote).await?;
        Ok(quote)
    }
    
    pub async fn get_quote(&self, _pool: &SqlitePool, id: Uuid) -> Result<Option<ConfiguredQuote>> {
        self.repo.get_configured_quote(id).await
    }
    
    pub async fn list_quotes(&self, _pool: &SqlitePool) -> Result<Vec<ConfiguredQuote>> {
        self.repo.list_configured_quotes().await
    }
}
