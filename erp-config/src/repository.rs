use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use erp_core::Result;
use crate::models::*;

#[async_trait]
pub trait ConfigRepository: Send + Sync {
    async fn get_config(&self, pool: &SqlitePool, category: &str, key: &str) -> Result<Option<SystemConfig>>;
    async fn set_config(&self, pool: &SqlitePool, config: SystemConfig) -> Result<SystemConfig>;
    async fn list_configs(&self, pool: &SqlitePool, category: Option<&str>) -> Result<Vec<SystemConfig>>;
    async fn delete_config(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn get_company_settings(&self, pool: &SqlitePool) -> Result<Option<CompanySetting>>;
    async fn update_company_settings(&self, pool: &SqlitePool, settings: CompanySetting) -> Result<CompanySetting>;
    async fn create_number_sequence(&self, pool: &SqlitePool, seq: NumberSequence) -> Result<NumberSequence>;
    async fn get_next_number(&self, pool: &SqlitePool, code: &str) -> Result<String>;
    async fn create_email_config(&self, pool: &SqlitePool, config: EmailConfig) -> Result<EmailConfig>;
    async fn get_email_config(&self, pool: &SqlitePool) -> Result<Option<EmailConfig>>;
    async fn create_storage_config(&self, pool: &SqlitePool, config: StorageConfig) -> Result<StorageConfig>;
    async fn list_storage_configs(&self, pool: &SqlitePool) -> Result<Vec<StorageConfig>>;
    async fn create_payment_gateway(&self, pool: &SqlitePool, gateway: PaymentGateway) -> Result<PaymentGateway>;
    async fn list_payment_gateways(&self, pool: &SqlitePool) -> Result<Vec<PaymentGateway>>;
    async fn create_shipping_provider(&self, pool: &SqlitePool, provider: ShippingProvider) -> Result<ShippingProvider>;
    async fn list_shipping_providers(&self, pool: &SqlitePool) -> Result<Vec<ShippingProvider>>;
    async fn create_localization(&self, pool: &SqlitePool, loc: Localization) -> Result<Localization>;
    async fn list_localizations(&self, pool: &SqlitePool) -> Result<Vec<Localization>>;
    async fn get_audit_settings(&self, pool: &SqlitePool) -> Result<Option<AuditSetting>>;
    async fn update_audit_settings(&self, pool: &SqlitePool, settings: AuditSetting) -> Result<AuditSetting>;
    async fn create_integration(&self, pool: &SqlitePool, config: IntegrationConfig) -> Result<IntegrationConfig>;
    async fn list_integrations(&self, pool: &SqlitePool) -> Result<Vec<IntegrationConfig>>;
}

pub struct SqliteConfigRepository;

#[async_trait]
impl ConfigRepository for SqliteConfigRepository {
    async fn get_config(&self, pool: &SqlitePool, category: &str, key: &str) -> Result<Option<SystemConfig>> {
        let row = sqlx::query!(
            r#"SELECT id, category, key, value, value_type, description, is_encrypted, is_system,
               is_public, default_value, validation_regex, group_name, sort_order, created_at, updated_at
               FROM system_configs WHERE category = ? AND key = ?"#,
            category, key
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|r| SystemConfig {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            category: r.category,
            key: r.key,
            value: r.value,
            value_type: ConfigValueType::String,
            description: r.description,
            is_encrypted: r.is_encrypted == 1,
            is_system: r.is_system == 1,
            is_public: r.is_public == 1,
            default_value: r.default_value,
            validation_regex: r.validation_regex,
            group_name: r.group_name,
            sort_order: r.sort_order,
        }))
    }

    async fn set_config(&self, pool: &SqlitePool, config: SystemConfig) -> Result<SystemConfig> {
        sqlx::query!(
            r#"INSERT INTO system_configs (id, category, key, value, value_type, description,
               is_encrypted, is_system, is_public, default_value, validation_regex, group_name,
               sort_order, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               ON CONFLICT(category, key) DO UPDATE SET value = ?, updated_at = ?, updated_by = ?"#,
            config.base.id.to_string(),
            config.category,
            config.key,
            config.value,
            format!("{:?}", config.value_type),
            config.description,
            config.is_encrypted as i32,
            config.is_system as i32,
            config.is_public as i32,
            config.default_value,
            config.validation_regex,
            config.group_name,
            config.sort_order,
            config.base.created_at.to_rfc3339(),
            config.base.updated_at.to_rfc3339(),
            config.base.created_by.map(|id| id.to_string()),
            config.base.updated_by.map(|id| id.to_string()),
            config.value,
            config.base.updated_at.to_rfc3339(),
            config.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(config)
    }

    async fn list_configs(&self, pool: &SqlitePool, category: Option<&str>) -> Result<Vec<SystemConfig>> {
        let rows = if let Some(cat) = category {
            sqlx::query!(
                r#"SELECT id, category, key, value, value_type, description, is_encrypted, is_system,
                   is_public, default_value, validation_regex, group_name, sort_order, created_at, updated_at
                   FROM system_configs WHERE category = ? ORDER BY sort_order"#,
                cat
            ).fetch_all(pool).await?
        } else {
            sqlx::query!(
                r#"SELECT id, category, key, value, value_type, description, is_encrypted, is_system,
                   is_public, default_value, validation_regex, group_name, sort_order, created_at, updated_at
                   FROM system_configs ORDER BY category, sort_order"#
            ).fetch_all(pool).await?
        };
        
        Ok(rows.into_iter().map(|r| SystemConfig {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            category: r.category,
            key: r.key,
            value: r.value,
            value_type: ConfigValueType::String,
            description: r.description,
            is_encrypted: r.is_encrypted == 1,
            is_system: r.is_system == 1,
            is_public: r.is_public == 1,
            default_value: r.default_value,
            validation_regex: r.validation_regex,
            group_name: r.group_name,
            sort_order: r.sort_order,
        }).collect())
    }

    async fn delete_config(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query!("DELETE FROM system_configs WHERE id = ? AND is_system = 0", id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn get_company_settings(&self, pool: &SqlitePool) -> Result<Option<CompanySetting>> {
        let row = sqlx::query!(
            r#"SELECT id, company_name, legal_name, tax_id, registration_number, logo_url, favicon_url,
               primary_color, secondary_color, timezone, date_format, time_format, currency, language,
               fiscal_year_start, week_start, address, city, state, country, postal_code, phone, email, website,
               created_at, updated_at FROM company_settings LIMIT 1"#
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|r| CompanySetting {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            company_name: r.company_name,
            legal_name: r.legal_name,
            tax_id: r.tax_id,
            registration_number: r.registration_number,
            logo_url: r.logo_url,
            favicon_url: r.favicon_url,
            primary_color: r.primary_color,
            secondary_color: r.secondary_color,
            timezone: r.timezone,
            date_format: r.date_format,
            time_format: r.time_format,
            currency: r.currency,
            language: r.language,
            fiscal_year_start: r.fiscal_year_start,
            week_start: r.week_start,
            address: r.address,
            city: r.city,
            state: r.state,
            country: r.country,
            postal_code: r.postal_code,
            phone: r.phone,
            email: r.email,
            website: r.website,
        }))
    }

    async fn update_company_settings(&self, pool: &SqlitePool, settings: CompanySetting) -> Result<CompanySetting> {
        sqlx::query!(
            r#"INSERT INTO company_settings (id, company_name, legal_name, tax_id, registration_number,
               logo_url, favicon_url, primary_color, secondary_color, timezone, date_format, time_format,
               currency, language, fiscal_year_start, week_start, address, city, state, country,
               postal_code, phone, email, website, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               ON CONFLICT DO UPDATE SET company_name = ?, legal_name = ?, tax_id = ?, updated_at = ?"#,
            settings.base.id.to_string(),
            settings.company_name,
            settings.legal_name,
            settings.tax_id,
            settings.registration_number,
            settings.logo_url,
            settings.favicon_url,
            settings.primary_color,
            settings.secondary_color,
            settings.timezone,
            settings.date_format,
            settings.time_format,
            settings.currency,
            settings.language,
            settings.fiscal_year_start,
            settings.week_start,
            settings.address,
            settings.city,
            settings.state,
            settings.country,
            settings.postal_code,
            settings.phone,
            settings.email,
            settings.website,
            settings.base.created_at.to_rfc3339(),
            settings.base.updated_at.to_rfc3339(),
            settings.base.created_by.map(|id| id.to_string()),
            settings.base.updated_by.map(|id| id.to_string()),
            settings.company_name,
            settings.legal_name,
            settings.tax_id,
            settings.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(settings)
    }

    async fn create_number_sequence(&self, pool: &SqlitePool, seq: NumberSequence) -> Result<NumberSequence> {
        sqlx::query!(
            r#"INSERT INTO number_sequences (id, name, code, prefix, suffix, current_value, increment,
               padding, reset_period, last_reset, format, is_active, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            seq.base.id.to_string(),
            seq.name,
            seq.code,
            seq.prefix,
            seq.suffix,
            seq.current_value,
            seq.increment,
            seq.padding,
            seq.reset_period.map(|p| format!("{:?}", p)),
            seq.last_reset.map(|d| d.to_rfc3339()),
            seq.format,
            seq.is_active as i32,
            seq.base.created_at.to_rfc3339(),
            seq.base.updated_at.to_rfc3339(),
            seq.base.created_by.map(|id| id.to_string()),
            seq.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(seq)
    }

    async fn get_next_number(&self, pool: &SqlitePool, code: &str) -> Result<String> {
        let row = sqlx::query!(
            r#"SELECT id, name, code, prefix, suffix, current_value, increment, padding, format
               FROM number_sequences WHERE code = ? AND is_active = 1"#,
            code
        ).fetch_one(pool).await?;
        
        let next_value = row.current_value + row.increment as i64;
        let number_str = format!("{:0>width$}", next_value, width = row.padding as usize);
        
        let result = if let Some(fmt) = row.format {
            fmt.replace("{number}", &number_str)
                .replace("{prefix}", row.prefix.as_deref().unwrap_or(""))
                .replace("{suffix}", row.suffix.as_deref().unwrap_or(""))
        } else {
            format!("{}{}{}", row.prefix.unwrap_or_default(), number_str, row.suffix.unwrap_or_default())
        };
        
        sqlx::query!(
            r#"UPDATE number_sequences SET current_value = ? WHERE code = ?"#,
            next_value, code
        ).execute(pool).await?;
        
        Ok(result)
    }

    async fn create_email_config(&self, pool: &SqlitePool, config: EmailConfig) -> Result<EmailConfig> {
        sqlx::query!(
            r#"INSERT INTO email_configs (id, smtp_host, smtp_port, smtp_user, smtp_password, use_tls,
               use_ssl, from_address, from_name, reply_to, is_default, is_active, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            config.base.id.to_string(),
            config.smtp_host,
            config.smtp_port,
            config.smtp_user,
            config.smtp_password,
            config.use_tls as i32,
            config.use_ssl as i32,
            config.from_address,
            config.from_name,
            config.reply_to,
            config.is_default as i32,
            config.is_active as i32,
            config.base.created_at.to_rfc3339(),
            config.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(config)
    }

    async fn get_email_config(&self, pool: &SqlitePool) -> Result<Option<EmailConfig>> {
        let row = sqlx::query!(
            r#"SELECT id, smtp_host, smtp_port, smtp_user, smtp_password, use_tls, use_ssl,
               from_address, from_name, reply_to, is_default, is_active, created_at, updated_at
               FROM email_configs WHERE is_default = 1 AND is_active = 1 LIMIT 1"#
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|r| EmailConfig {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            smtp_host: r.smtp_host,
            smtp_port: r.smtp_port,
            smtp_user: r.smtp_user,
            smtp_password: r.smtp_password,
            use_tls: r.use_tls == 1,
            use_ssl: r.use_ssl == 1,
            from_address: r.from_address,
            from_name: r.from_name,
            reply_to: r.reply_to,
            is_default: r.is_default == 1,
            is_active: r.is_active == 1,
        }))
    }

    async fn create_storage_config(&self, pool: &SqlitePool, config: StorageConfig) -> Result<StorageConfig> {
        sqlx::query!(
            r#"INSERT INTO storage_configs (id, storage_type, name, endpoint, bucket, region,
               access_key, secret_key, base_path, max_file_size, allowed_types, is_default, is_active,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            config.base.id.to_string(),
            format!("{:?}", config.storage_type),
            config.name,
            config.endpoint,
            config.bucket,
            config.region,
            config.access_key,
            config.secret_key,
            config.base_path,
            config.max_file_size,
            config.allowed_types,
            config.is_default as i32,
            config.is_active as i32,
            config.base.created_at.to_rfc3339(),
            config.base.updated_at.to_rfc3339(),
            config.base.created_by.map(|id| id.to_string()),
            config.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(config)
    }

    async fn list_storage_configs(&self, pool: &SqlitePool) -> Result<Vec<StorageConfig>> {
        let rows = sqlx::query!(
            r#"SELECT id, storage_type, name, endpoint, bucket, region, access_key, secret_key,
               base_path, max_file_size, allowed_types, is_default, is_active, created_at, updated_at
               FROM storage_configs WHERE is_active = 1"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| StorageConfig {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            storage_type: StorageType::Local,
            name: r.name,
            endpoint: r.endpoint,
            bucket: r.bucket,
            region: r.region,
            access_key: r.access_key,
            secret_key: r.secret_key,
            base_path: r.base_path,
            max_file_size: r.max_file_size,
            allowed_types: r.allowed_types,
            is_default: r.is_default == 1,
            is_active: r.is_active == 1,
        }).collect())
    }

    async fn create_payment_gateway(&self, pool: &SqlitePool, gateway: PaymentGateway) -> Result<PaymentGateway> {
        sqlx::query!(
            r#"INSERT INTO payment_gateways (id, name, code, gateway_type, api_key, api_secret,
               merchant_id, endpoint_url, webhook_url, supported_currencies, supported_methods,
               fee_percent, fee_fixed, is_sandbox, is_default, is_active, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            gateway.base.id.to_string(),
            gateway.name,
            gateway.code,
            gateway.gateway_type,
            gateway.api_key,
            gateway.api_secret,
            gateway.merchant_id,
            gateway.endpoint_url,
            gateway.webhook_url,
            gateway.supported_currencies,
            gateway.supported_methods,
            gateway.fee_percent,
            gateway.fee_fixed,
            gateway.is_sandbox as i32,
            gateway.is_default as i32,
            gateway.is_active as i32,
            gateway.base.created_at.to_rfc3339(),
            gateway.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(gateway)
    }

    async fn list_payment_gateways(&self, pool: &SqlitePool) -> Result<Vec<PaymentGateway>> {
        let rows = sqlx::query!(
            r#"SELECT id, name, code, gateway_type, api_key, api_secret, merchant_id, endpoint_url,
               webhook_url, supported_currencies, supported_methods, fee_percent, fee_fixed,
               is_sandbox, is_default, is_active, created_at, updated_at
               FROM payment_gateways WHERE is_active = 1"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| PaymentGateway {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: r.name,
            code: r.code,
            gateway_type: r.gateway_type,
            api_key: r.api_key,
            api_secret: r.api_secret,
            merchant_id: r.merchant_id,
            endpoint_url: r.endpoint_url,
            webhook_url: r.webhook_url,
            supported_currencies: r.supported_currencies,
            supported_methods: r.supported_methods,
            fee_percent: r.fee_percent,
            fee_fixed: r.fee_fixed,
            is_sandbox: r.is_sandbox == 1,
            is_default: r.is_default == 1,
            is_active: r.is_active == 1,
        }).collect())
    }

    async fn create_shipping_provider(&self, pool: &SqlitePool, provider: ShippingProvider) -> Result<ShippingProvider> {
        sqlx::query!(
            r#"INSERT INTO shipping_providers (id, name, code, api_key, api_secret, account_number,
               endpoint_url, tracking_url, supported_services, supported_countries, is_default, is_active,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            provider.base.id.to_string(),
            provider.name,
            provider.code,
            provider.api_key,
            provider.api_secret,
            provider.account_number,
            provider.endpoint_url,
            provider.tracking_url,
            provider.supported_services,
            provider.supported_countries,
            provider.is_default as i32,
            provider.is_active as i32,
            provider.base.created_at.to_rfc3339(),
            provider.base.updated_at.to_rfc3339(),
            provider.base.created_by.map(|id| id.to_string()),
            provider.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(provider)
    }

    async fn list_shipping_providers(&self, pool: &SqlitePool) -> Result<Vec<ShippingProvider>> {
        let rows = sqlx::query!(
            r#"SELECT id, name, code, api_key, api_secret, account_number, endpoint_url, tracking_url,
               supported_services, supported_countries, is_default, is_active, created_at, updated_at
               FROM shipping_providers WHERE is_active = 1"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| ShippingProvider {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: r.name,
            code: r.code,
            api_key: r.api_key,
            api_secret: r.api_secret,
            account_number: r.account_number,
            endpoint_url: r.endpoint_url,
            tracking_url: r.tracking_url,
            supported_services: r.supported_services,
            supported_countries: r.supported_countries,
            is_default: r.is_default == 1,
            is_active: r.is_active == 1,
        }).collect())
    }

    async fn create_localization(&self, pool: &SqlitePool, loc: Localization) -> Result<Localization> {
        sqlx::query!(
            r#"INSERT INTO localizations (id, language_code, locale, name, native_name, date_format,
               time_format, number_format, currency_symbol, currency_position, decimal_separator,
               thousand_separator, is_rtl, is_default, is_active, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            loc.base.id.to_string(),
            loc.language_code,
            loc.locale,
            loc.name,
            loc.native_name,
            loc.date_format,
            loc.time_format,
            loc.number_format,
            loc.currency_symbol,
            loc.currency_position,
            loc.decimal_separator,
            loc.thousand_separator,
            loc.is_rtl as i32,
            loc.is_default as i32,
            loc.is_active as i32,
            loc.base.created_at.to_rfc3339(),
            loc.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(loc)
    }

    async fn list_localizations(&self, pool: &SqlitePool) -> Result<Vec<Localization>> {
        let rows = sqlx::query!(
            r#"SELECT id, language_code, locale, name, native_name, date_format, time_format,
               number_format, currency_symbol, currency_position, decimal_separator, thousand_separator,
               is_rtl, is_default, is_active, created_at, updated_at
               FROM localizations WHERE is_active = 1"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| Localization {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            language_code: r.language_code,
            locale: r.locale,
            name: r.name,
            native_name: r.native_name,
            date_format: r.date_format,
            time_format: r.time_format,
            number_format: r.number_format,
            currency_symbol: r.currency_symbol,
            currency_position: r.currency_position,
            decimal_separator: r.decimal_separator,
            thousand_separator: r.thousand_separator,
            is_rtl: r.is_rtl == 1,
            is_default: r.is_default == 1,
            is_active: r.is_active == 1,
        }).collect())
    }

    async fn get_audit_settings(&self, pool: &SqlitePool) -> Result<Option<AuditSetting>> {
        let row = sqlx::query!(
            r#"SELECT id, log_retention_days, log_sensitive_data, log_login_attempts, log_data_changes,
               log_api_requests, alert_on_suspicious, max_login_attempts, lockout_duration_minutes,
               password_expiry_days, require_mfa, session_timeout_minutes, created_at, updated_at
               FROM audit_settings LIMIT 1"#
        ).fetch_optional(pool).await?;
        
        Ok(row.map(|r| AuditSetting {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            log_retention_days: r.log_retention_days,
            log_sensitive_data: r.log_sensitive_data == 1,
            log_login_attempts: r.log_login_attempts == 1,
            log_data_changes: r.log_data_changes == 1,
            log_api_requests: r.log_api_requests == 1,
            alert_on_suspicious: r.alert_on_suspicious == 1,
            max_login_attempts: r.max_login_attempts,
            lockout_duration_minutes: r.lockout_duration_minutes,
            password_expiry_days: r.password_expiry_days,
            require_mfa: r.require_mfa == 1,
            session_timeout_minutes: r.session_timeout_minutes,
        }))
    }

    async fn update_audit_settings(&self, pool: &SqlitePool, settings: AuditSetting) -> Result<AuditSetting> {
        sqlx::query!(
            r#"INSERT INTO audit_settings (id, log_retention_days, log_sensitive_data, log_login_attempts,
               log_data_changes, log_api_requests, alert_on_suspicious, max_login_attempts,
               lockout_duration_minutes, password_expiry_days, require_mfa, session_timeout_minutes,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               ON CONFLICT DO UPDATE SET log_retention_days = ?, max_login_attempts = ?, updated_at = ?"#,
            settings.base.id.to_string(),
            settings.log_retention_days,
            settings.log_sensitive_data as i32,
            settings.log_login_attempts as i32,
            settings.log_data_changes as i32,
            settings.log_api_requests as i32,
            settings.alert_on_suspicious as i32,
            settings.max_login_attempts,
            settings.lockout_duration_minutes,
            settings.password_expiry_days,
            settings.require_mfa as i32,
            settings.session_timeout_minutes,
            settings.base.created_at.to_rfc3339(),
            settings.base.updated_at.to_rfc3339(),
            settings.base.created_by.map(|id| id.to_string()),
            settings.base.updated_by.map(|id| id.to_string()),
            settings.log_retention_days,
            settings.max_login_attempts,
            settings.base.updated_at.to_rfc3339(),
        ).execute(pool).await?;
        Ok(settings)
    }

    async fn create_integration(&self, pool: &SqlitePool, config: IntegrationConfig) -> Result<IntegrationConfig> {
        sqlx::query!(
            r#"INSERT INTO integration_configs (id, name, code, integration_type, api_endpoint, api_key,
               api_secret, config_json, sync_enabled, sync_frequency, last_sync, sync_status, is_active,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#,
            config.base.id.to_string(),
            config.name,
            config.code,
            config.integration_type,
            config.api_endpoint,
            config.api_key,
            config.api_secret,
            config.config_json,
            config.sync_enabled as i32,
            config.sync_frequency,
            config.last_sync.map(|d| d.to_rfc3339()),
            config.sync_status,
            config.is_active as i32,
            config.base.created_at.to_rfc3339(),
            config.base.updated_at.to_rfc3339(),
            config.base.created_by.map(|id| id.to_string()),
            config.base.updated_by.map(|id| id.to_string()),
        ).execute(pool).await?;
        Ok(config)
    }

    async fn list_integrations(&self, pool: &SqlitePool) -> Result<Vec<IntegrationConfig>> {
        let rows = sqlx::query!(
            r#"SELECT id, name, code, integration_type, api_endpoint, api_key, api_secret, config_json,
               sync_enabled, sync_frequency, last_sync, sync_status, is_active, created_at, updated_at
               FROM integration_configs WHERE is_active = 1"#
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| IntegrationConfig {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: r.name,
            code: r.code,
            integration_type: r.integration_type,
            api_endpoint: r.api_endpoint,
            api_key: r.api_key,
            api_secret: r.api_secret,
            config_json: r.config_json,
            sync_enabled: r.sync_enabled == 1,
            sync_frequency: r.sync_frequency,
            last_sync: r.last_sync.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            sync_status: r.sync_status,
            is_active: r.is_active == 1,
        }).collect())
    }
}
