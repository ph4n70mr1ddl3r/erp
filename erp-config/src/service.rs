use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::{Result, BaseEntity, Status};
use crate::models::*;
use crate::repository::*;

pub struct ConfigService {
    repo: SqliteConfigRepository,
}

impl ConfigService {
    pub fn new() -> Self {
        Self { repo: SqliteConfigRepository }
    }

    pub async fn get(&self, pool: &SqlitePool, category: &str, key: &str) -> Result<Option<String>> {
        let config = self.repo.get_config(pool, category, key).await?;
        Ok(config.map(|c| c.value))
    }

    pub async fn set(&self, pool: &SqlitePool, category: String, key: String, value: String) -> Result<SystemConfig> {
        let config = SystemConfig {
            base: BaseEntity::new(),
            category,
            key,
            value,
            value_type: ConfigValueType::String,
            description: None,
            is_encrypted: false,
            is_system: false,
            is_public: false,
            default_value: None,
            validation_regex: None,
            group_name: None,
            sort_order: 0,
        };
        self.repo.set_config(pool, config).await
    }

    pub async fn list(&self, pool: &SqlitePool, category: Option<&str>) -> Result<Vec<SystemConfig>> {
        self.repo.list_configs(pool, category).await
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete_config(pool, id).await
    }

    pub async fn get_company_settings(&self, pool: &SqlitePool) -> Result<Option<CompanySetting>> {
        self.repo.get_company_settings(pool).await
    }

    pub async fn update_company_settings(&self, pool: &SqlitePool, settings: CompanySetting) -> Result<CompanySetting> {
        self.repo.update_company_settings(pool, settings).await
    }

    pub async fn create_number_sequence(&self, pool: &SqlitePool, name: String, code: String, prefix: Option<String>, padding: i32) -> Result<NumberSequence> {
        let seq = NumberSequence {
            base: BaseEntity::new(),
            name,
            code,
            prefix,
            suffix: None,
            current_value: 0,
            increment: 1,
            padding,
            reset_period: None,
            last_reset: None,
            format: None,
            is_active: true,
        };
        self.repo.create_number_sequence(pool, seq).await
    }

    pub async fn get_next_number(&self, pool: &SqlitePool, code: &str) -> Result<String> {
        self.repo.get_next_number(pool, code).await
    }

    pub async fn create_email_config(&self, pool: &SqlitePool, config: EmailConfig) -> Result<EmailConfig> {
        self.repo.create_email_config(pool, config).await
    }

    pub async fn get_email_config(&self, pool: &SqlitePool) -> Result<Option<EmailConfig>> {
        self.repo.get_email_config(pool).await
    }

    pub async fn create_storage_config(&self, pool: &SqlitePool, config: StorageConfig) -> Result<StorageConfig> {
        self.repo.create_storage_config(pool, config).await
    }

    pub async fn list_storage_configs(&self, pool: &SqlitePool) -> Result<Vec<StorageConfig>> {
        self.repo.list_storage_configs(pool).await
    }

    pub async fn create_payment_gateway(&self, pool: &SqlitePool, gateway: PaymentGateway) -> Result<PaymentGateway> {
        self.repo.create_payment_gateway(pool, gateway).await
    }

    pub async fn list_payment_gateways(&self, pool: &SqlitePool) -> Result<Vec<PaymentGateway>> {
        self.repo.list_payment_gateways(pool).await
    }

    pub async fn create_shipping_provider(&self, pool: &SqlitePool, provider: ShippingProvider) -> Result<ShippingProvider> {
        self.repo.create_shipping_provider(pool, provider).await
    }

    pub async fn list_shipping_providers(&self, pool: &SqlitePool) -> Result<Vec<ShippingProvider>> {
        self.repo.list_shipping_providers(pool).await
    }

    pub async fn create_localization(&self, pool: &SqlitePool, loc: Localization) -> Result<Localization> {
        self.repo.create_localization(pool, loc).await
    }

    pub async fn list_localizations(&self, pool: &SqlitePool) -> Result<Vec<Localization>> {
        self.repo.list_localizations(pool).await
    }

    pub async fn get_audit_settings(&self, pool: &SqlitePool) -> Result<Option<AuditSetting>> {
        self.repo.get_audit_settings(pool).await
    }

    pub async fn update_audit_settings(&self, pool: &SqlitePool, settings: AuditSetting) -> Result<AuditSetting> {
        self.repo.update_audit_settings(pool, settings).await
    }

    pub async fn create_integration(&self, pool: &SqlitePool, config: IntegrationConfig) -> Result<IntegrationConfig> {
        self.repo.create_integration(pool, config).await
    }

    pub async fn list_integrations(&self, pool: &SqlitePool) -> Result<Vec<IntegrationConfig>> {
        self.repo.list_integrations(pool).await
    }
}
