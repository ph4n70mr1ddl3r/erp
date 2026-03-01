use async_trait::async_trait;
use sqlx::{SqlitePool, Row};
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
        let row = sqlx::query(
            r#"SELECT id, category, key, value, value_type, description, is_encrypted, is_system,
               is_public, default_value, validation_regex, group_name, sort_order, created_at, updated_at
               FROM system_configs WHERE category = ? AND key = ?"#
        )
        .bind(category)
        .bind(key)
        .fetch_optional(pool).await?;
        
        Ok(row.map(|r| SystemConfig {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            category: r.get("category"),
            key: r.get("key"),
            value: r.get("value"),
            value_type: ConfigValueType::String,
            description: r.get("description"),
            is_encrypted: r.get::<i32, _>("is_encrypted") == 1,
            is_system: r.get::<i32, _>("is_system") == 1,
            is_public: r.get::<i32, _>("is_public") == 1,
            default_value: r.get("default_value"),
            validation_regex: r.get("validation_regex"),
            group_name: r.get("group_name"),
            sort_order: r.get("sort_order"),
        }))
    }

    async fn set_config(&self, pool: &SqlitePool, config: SystemConfig) -> Result<SystemConfig> {
        sqlx::query(
            r#"INSERT INTO system_configs (id, category, key, value, value_type, description,
               is_encrypted, is_system, is_public, default_value, validation_regex, group_name,
               sort_order, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               ON CONFLICT(category, key) DO UPDATE SET value = ?, updated_at = ?, updated_by = ?"#
        )
        .bind(config.base.id.to_string())
        .bind(&config.category)
        .bind(&config.key)
        .bind(&config.value)
        .bind(format!("{:?}", config.value_type))
        .bind(&config.description)
        .bind(config.is_encrypted as i32)
        .bind(config.is_system as i32)
        .bind(config.is_public as i32)
        .bind(&config.default_value)
        .bind(&config.validation_regex)
        .bind(&config.group_name)
        .bind(config.sort_order)
        .bind(config.base.created_at.to_rfc3339())
        .bind(config.base.updated_at.to_rfc3339())
        .bind(config.base.created_by.map(|id| id.to_string()))
        .bind(config.base.updated_by.map(|id| id.to_string()))
        .bind(&config.value)
        .bind(config.base.updated_at.to_rfc3339())
        .bind(config.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(config)
    }

    async fn list_configs(&self, pool: &SqlitePool, category: Option<&str>) -> Result<Vec<SystemConfig>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = if let Some(cat) = category {
            sqlx::query(
                r#"SELECT id, category, key, value, value_type, description, is_encrypted, is_system,
                   is_public, default_value, validation_regex, group_name, sort_order, created_at, updated_at
                   FROM system_configs WHERE category = ? ORDER BY sort_order"#
            )
            .bind(cat)
            .fetch_all(pool).await?
        } else {
            sqlx::query(
                r#"SELECT id, category, key, value, value_type, description, is_encrypted, is_system,
                   is_public, default_value, validation_regex, group_name, sort_order, created_at, updated_at
                   FROM system_configs ORDER BY category, sort_order"#
            )
            .fetch_all(pool).await?
        };
        
        Ok(rows.into_iter().map(|r| SystemConfig {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            category: r.get("category"),
            key: r.get("key"),
            value: r.get("value"),
            value_type: ConfigValueType::String,
            description: r.get("description"),
            is_encrypted: r.get::<i32, _>("is_encrypted") == 1,
            is_system: r.get::<i32, _>("is_system") == 1,
            is_public: r.get::<i32, _>("is_public") == 1,
            default_value: r.get("default_value"),
            validation_regex: r.get("validation_regex"),
            group_name: r.get("group_name"),
            sort_order: r.get("sort_order"),
        }).collect())
    }

    async fn delete_config(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM system_configs WHERE id = ? AND is_system = 0")
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn get_company_settings(&self, pool: &SqlitePool) -> Result<Option<CompanySetting>> {
        let row = sqlx::query(
            r#"SELECT id, company_name, legal_name, tax_id, registration_number, logo_url, favicon_url,
               primary_color, secondary_color, timezone, date_format, time_format, currency, language,
               fiscal_year_start, week_start, address, city, state, country, postal_code, phone, email, website,
               created_at, updated_at FROM company_settings LIMIT 1"#
        )
        .fetch_optional(pool).await?;
        
        Ok(row.map(|r| CompanySetting {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            company_name: r.get("company_name"),
            legal_name: r.get("legal_name"),
            tax_id: r.get("tax_id"),
            registration_number: r.get("registration_number"),
            logo_url: r.get("logo_url"),
            favicon_url: r.get("favicon_url"),
            primary_color: r.get("primary_color"),
            secondary_color: r.get("secondary_color"),
            timezone: r.get("timezone"),
            date_format: r.get("date_format"),
            time_format: r.get("time_format"),
            currency: r.get("currency"),
            language: r.get("language"),
            fiscal_year_start: r.get("fiscal_year_start"),
            week_start: r.get("week_start"),
            address: r.get("address"),
            city: r.get("city"),
            state: r.get("state"),
            country: r.get("country"),
            postal_code: r.get("postal_code"),
            phone: r.get("phone"),
            email: r.get("email"),
            website: r.get("website"),
        }))
    }

    async fn update_company_settings(&self, pool: &SqlitePool, settings: CompanySetting) -> Result<CompanySetting> {
        sqlx::query(
            r#"INSERT INTO company_settings (id, company_name, legal_name, tax_id, registration_number,
               logo_url, favicon_url, primary_color, secondary_color, timezone, date_format, time_format,
               currency, language, fiscal_year_start, week_start, address, city, state, country,
               postal_code, phone, email, website, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               ON CONFLICT DO UPDATE SET company_name = ?, legal_name = ?, tax_id = ?, updated_at = ?"#
        )
        .bind(settings.base.id.to_string())
        .bind(&settings.company_name)
        .bind(&settings.legal_name)
        .bind(&settings.tax_id)
        .bind(&settings.registration_number)
        .bind(&settings.logo_url)
        .bind(&settings.favicon_url)
        .bind(&settings.primary_color)
        .bind(&settings.secondary_color)
        .bind(&settings.timezone)
        .bind(&settings.date_format)
        .bind(&settings.time_format)
        .bind(&settings.currency)
        .bind(&settings.language)
        .bind(settings.fiscal_year_start)
        .bind(settings.week_start)
        .bind(&settings.address)
        .bind(&settings.city)
        .bind(&settings.state)
        .bind(&settings.country)
        .bind(&settings.postal_code)
        .bind(&settings.phone)
        .bind(&settings.email)
        .bind(&settings.website)
        .bind(settings.base.created_at.to_rfc3339())
        .bind(settings.base.updated_at.to_rfc3339())
        .bind(settings.base.created_by.map(|id| id.to_string()))
        .bind(settings.base.updated_by.map(|id| id.to_string()))
        .bind(&settings.company_name)
        .bind(&settings.legal_name)
        .bind(&settings.tax_id)
        .bind(settings.base.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(settings)
    }

    async fn create_number_sequence(&self, pool: &SqlitePool, seq: NumberSequence) -> Result<NumberSequence> {
        sqlx::query(
            r#"INSERT INTO number_sequences (id, name, code, prefix, suffix, current_value, increment,
               padding, reset_period, last_reset, format, is_active, created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(seq.base.id.to_string())
        .bind(&seq.name)
        .bind(&seq.code)
        .bind(&seq.prefix)
        .bind(&seq.suffix)
        .bind(seq.current_value)
        .bind(seq.increment)
        .bind(seq.padding)
        .bind(seq.reset_period.as_ref().map(|p| format!("{:?}", p)))
        .bind(seq.last_reset.map(|d| d.to_rfc3339()))
        .bind(&seq.format)
        .bind(seq.is_active as i32)
        .bind(seq.base.created_at.to_rfc3339())
        .bind(seq.base.updated_at.to_rfc3339())
        .bind(seq.base.created_by.map(|id| id.to_string()))
        .bind(seq.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(seq)
    }

    async fn get_next_number(&self, pool: &SqlitePool, code: &str) -> Result<String> {
        let row = sqlx::query(
            r#"SELECT id, name, code, prefix, suffix, current_value, increment, padding, format
               FROM number_sequences WHERE code = ? AND is_active = 1"#
        )
        .bind(code)
        .fetch_one(pool).await?;
        
        let current_value: i64 = row.get("current_value");
        let increment: i32 = row.get("increment");
        let next_value = current_value + increment as i64;
        let padding: i32 = row.get("padding");
        let number_str = format!("{:0>width$}", next_value, width = padding as usize);
        
        let result = if let Ok(fmt) = row.try_get::<&str, _>("format") {
            fmt.replace("{number}", &number_str)
                .replace("{prefix}", row.get::<Option<&str>, _>("prefix").unwrap_or(""))
                .replace("{suffix}", row.get::<Option<&str>, _>("suffix").unwrap_or(""))
        } else {
            format!("{}{}{}", row.get::<Option<&str>, _>("prefix").unwrap_or_default(), number_str, row.get::<Option<&str>, _>("suffix").unwrap_or_default())
        };
        
        sqlx::query(
            r#"UPDATE number_sequences SET current_value = ? WHERE code = ?"#
        )
        .bind(next_value)
        .bind(code)
        .execute(pool).await?;
        
        Ok(result)
    }

    async fn create_email_config(&self, pool: &SqlitePool, config: EmailConfig) -> Result<EmailConfig> {
        sqlx::query(
            r#"INSERT INTO email_configs (id, smtp_host, smtp_port, smtp_user, smtp_password, use_tls,
               use_ssl, from_address, from_name, reply_to, is_default, is_active, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(config.base.id.to_string())
        .bind(&config.smtp_host)
        .bind(config.smtp_port)
        .bind(&config.smtp_user)
        .bind(&config.smtp_password)
        .bind(config.use_tls as i32)
        .bind(config.use_ssl as i32)
        .bind(&config.from_address)
        .bind(&config.from_name)
        .bind(&config.reply_to)
        .bind(config.is_default as i32)
        .bind(config.is_active as i32)
        .bind(config.base.created_at.to_rfc3339())
        .bind(config.base.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(config)
    }

    async fn get_email_config(&self, pool: &SqlitePool) -> Result<Option<EmailConfig>> {
        let row = sqlx::query(
            r#"SELECT id, smtp_host, smtp_port, smtp_user, smtp_password, use_tls, use_ssl,
               from_address, from_name, reply_to, is_default, is_active, created_at, updated_at
               FROM email_configs WHERE is_default = 1 AND is_active = 1 LIMIT 1"#
        )
        .fetch_optional(pool).await?;
        
        Ok(row.map(|r| EmailConfig {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            smtp_host: r.get("smtp_host"),
            smtp_port: r.get("smtp_port"),
            smtp_user: r.get("smtp_user"),
            smtp_password: r.get("smtp_password"),
            use_tls: r.get::<i32, _>("use_tls") == 1,
            use_ssl: r.get::<i32, _>("use_ssl") == 1,
            from_address: r.get("from_address"),
            from_name: r.get("from_name"),
            reply_to: r.get("reply_to"),
            is_default: r.get::<i32, _>("is_default") == 1,
            is_active: r.get::<i32, _>("is_active") == 1,
        }))
    }

    async fn create_storage_config(&self, pool: &SqlitePool, config: StorageConfig) -> Result<StorageConfig> {
        sqlx::query(
            r#"INSERT INTO storage_configs (id, storage_type, name, endpoint, bucket, region,
               access_key, secret_key, base_path, max_file_size, allowed_types, is_default, is_active,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(config.base.id.to_string())
        .bind(format!("{:?}", config.storage_type))
        .bind(&config.name)
        .bind(&config.endpoint)
        .bind(&config.bucket)
        .bind(&config.region)
        .bind(&config.access_key)
        .bind(&config.secret_key)
        .bind(&config.base_path)
        .bind(config.max_file_size)
        .bind(&config.allowed_types)
        .bind(config.is_default as i32)
        .bind(config.is_active as i32)
        .bind(config.base.created_at.to_rfc3339())
        .bind(config.base.updated_at.to_rfc3339())
        .bind(config.base.created_by.map(|id| id.to_string()))
        .bind(config.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(config)
    }

    async fn list_storage_configs(&self, pool: &SqlitePool) -> Result<Vec<StorageConfig>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, storage_type, name, endpoint, bucket, region, access_key, secret_key,
               base_path, max_file_size, allowed_types, is_default, is_active, created_at, updated_at
               FROM storage_configs WHERE is_active = 1"#
        )
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| StorageConfig {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            storage_type: StorageType::Local,
            name: r.get("name"),
            endpoint: r.get("endpoint"),
            bucket: r.get("bucket"),
            region: r.get("region"),
            access_key: r.get("access_key"),
            secret_key: r.get("secret_key"),
            base_path: r.get("base_path"),
            max_file_size: r.get("max_file_size"),
            allowed_types: r.get("allowed_types"),
            is_default: r.get::<i32, _>("is_default") == 1,
            is_active: r.get::<i32, _>("is_active") == 1,
        }).collect())
    }

    async fn create_payment_gateway(&self, pool: &SqlitePool, gateway: PaymentGateway) -> Result<PaymentGateway> {
        sqlx::query(
            r#"INSERT INTO payment_gateways (id, name, code, gateway_type, api_key, api_secret,
               merchant_id, endpoint_url, webhook_url, supported_currencies, supported_methods,
               fee_percent, fee_fixed, is_sandbox, is_default, is_active, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(gateway.base.id.to_string())
        .bind(&gateway.name)
        .bind(&gateway.code)
        .bind(&gateway.gateway_type)
        .bind(&gateway.api_key)
        .bind(&gateway.api_secret)
        .bind(&gateway.merchant_id)
        .bind(&gateway.endpoint_url)
        .bind(&gateway.webhook_url)
        .bind(&gateway.supported_currencies)
        .bind(&gateway.supported_methods)
        .bind(gateway.fee_percent)
        .bind(gateway.fee_fixed)
        .bind(gateway.is_sandbox as i32)
        .bind(gateway.is_default as i32)
        .bind(gateway.is_active as i32)
        .bind(gateway.base.created_at.to_rfc3339())
        .bind(gateway.base.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(gateway)
    }

    async fn list_payment_gateways(&self, pool: &SqlitePool) -> Result<Vec<PaymentGateway>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, name, code, gateway_type, api_key, api_secret, merchant_id, endpoint_url,
               webhook_url, supported_currencies, supported_methods, fee_percent, fee_fixed,
               is_sandbox, is_default, is_active, created_at, updated_at
               FROM payment_gateways WHERE is_active = 1"#
        )
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| PaymentGateway {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: r.get("name"),
            code: r.get("code"),
            gateway_type: r.get("gateway_type"),
            api_key: r.get("api_key"),
            api_secret: r.get("api_secret"),
            merchant_id: r.get("merchant_id"),
            endpoint_url: r.get("endpoint_url"),
            webhook_url: r.get("webhook_url"),
            supported_currencies: r.get("supported_currencies"),
            supported_methods: r.get("supported_methods"),
            fee_percent: r.get("fee_percent"),
            fee_fixed: r.get("fee_fixed"),
            is_sandbox: r.get::<i32, _>("is_sandbox") == 1,
            is_default: r.get::<i32, _>("is_default") == 1,
            is_active: r.get::<i32, _>("is_active") == 1,
        }).collect())
    }

    async fn create_shipping_provider(&self, pool: &SqlitePool, provider: ShippingProvider) -> Result<ShippingProvider> {
        sqlx::query(
            r#"INSERT INTO shipping_providers (id, name, code, api_key, api_secret, account_number,
               endpoint_url, tracking_url, supported_services, supported_countries, is_default, is_active,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(provider.base.id.to_string())
        .bind(&provider.name)
        .bind(&provider.code)
        .bind(&provider.api_key)
        .bind(&provider.api_secret)
        .bind(&provider.account_number)
        .bind(&provider.endpoint_url)
        .bind(&provider.tracking_url)
        .bind(&provider.supported_services)
        .bind(&provider.supported_countries)
        .bind(provider.is_default as i32)
        .bind(provider.is_active as i32)
        .bind(provider.base.created_at.to_rfc3339())
        .bind(provider.base.updated_at.to_rfc3339())
        .bind(provider.base.created_by.map(|id| id.to_string()))
        .bind(provider.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(provider)
    }

    async fn list_shipping_providers(&self, pool: &SqlitePool) -> Result<Vec<ShippingProvider>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, name, code, api_key, api_secret, account_number, endpoint_url, tracking_url,
               supported_services, supported_countries, is_default, is_active, created_at, updated_at
               FROM shipping_providers WHERE is_active = 1"#
        )
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| ShippingProvider {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: r.get("name"),
            code: r.get("code"),
            api_key: r.get("api_key"),
            api_secret: r.get("api_secret"),
            account_number: r.get("account_number"),
            endpoint_url: r.get("endpoint_url"),
            tracking_url: r.get("tracking_url"),
            supported_services: r.get("supported_services"),
            supported_countries: r.get("supported_countries"),
            is_default: r.get::<i32, _>("is_default") == 1,
            is_active: r.get::<i32, _>("is_active") == 1,
        }).collect())
    }

    async fn create_localization(&self, pool: &SqlitePool, loc: Localization) -> Result<Localization> {
        sqlx::query(
            r#"INSERT INTO localizations (id, language_code, locale, name, native_name, date_format,
               time_format, number_format, currency_symbol, currency_position, decimal_separator,
               thousand_separator, is_rtl, is_default, is_active, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(loc.base.id.to_string())
        .bind(&loc.language_code)
        .bind(&loc.locale)
        .bind(&loc.name)
        .bind(&loc.native_name)
        .bind(&loc.date_format)
        .bind(&loc.time_format)
        .bind(&loc.number_format)
        .bind(&loc.currency_symbol)
        .bind(&loc.currency_position)
        .bind(&loc.decimal_separator)
        .bind(&loc.thousand_separator)
        .bind(loc.is_rtl as i32)
        .bind(loc.is_default as i32)
        .bind(loc.is_active as i32)
        .bind(loc.base.created_at.to_rfc3339())
        .bind(loc.base.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(loc)
    }

    async fn list_localizations(&self, pool: &SqlitePool) -> Result<Vec<Localization>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, language_code, locale, name, native_name, date_format, time_format,
               number_format, currency_symbol, currency_position, decimal_separator, thousand_separator,
               is_rtl, is_default, is_active, created_at, updated_at
               FROM localizations WHERE is_active = 1"#
        )
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| Localization {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            language_code: r.get("language_code"),
            locale: r.get("locale"),
            name: r.get("name"),
            native_name: r.get("native_name"),
            date_format: r.get("date_format"),
            time_format: r.get("time_format"),
            number_format: r.get("number_format"),
            currency_symbol: r.get("currency_symbol"),
            currency_position: r.get("currency_position"),
            decimal_separator: r.get("decimal_separator"),
            thousand_separator: r.get("thousand_separator"),
            is_rtl: r.get::<i32, _>("is_rtl") == 1,
            is_default: r.get::<i32, _>("is_default") == 1,
            is_active: r.get::<i32, _>("is_active") == 1,
        }).collect())
    }

    async fn get_audit_settings(&self, pool: &SqlitePool) -> Result<Option<AuditSetting>> {
        let row = sqlx::query(
            r#"SELECT id, log_retention_days, log_sensitive_data, log_login_attempts, log_data_changes,
               log_api_requests, alert_on_suspicious, max_login_attempts, lockout_duration_minutes,
               password_expiry_days, require_mfa, session_timeout_minutes, created_at, updated_at
               FROM audit_settings LIMIT 1"#
        )
        .fetch_optional(pool).await?;
        
        Ok(row.map(|r| AuditSetting {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            log_retention_days: r.get("log_retention_days"),
            log_sensitive_data: r.get::<i32, _>("log_sensitive_data") == 1,
            log_login_attempts: r.get::<i32, _>("log_login_attempts") == 1,
            log_data_changes: r.get::<i32, _>("log_data_changes") == 1,
            log_api_requests: r.get::<i32, _>("log_api_requests") == 1,
            alert_on_suspicious: r.get::<i32, _>("alert_on_suspicious") == 1,
            max_login_attempts: r.get("max_login_attempts"),
            lockout_duration_minutes: r.get("lockout_duration_minutes"),
            password_expiry_days: r.get("password_expiry_days"),
            require_mfa: r.get::<i32, _>("require_mfa") == 1,
            session_timeout_minutes: r.get("session_timeout_minutes"),
        }))
    }

    async fn update_audit_settings(&self, pool: &SqlitePool, settings: AuditSetting) -> Result<AuditSetting> {
        sqlx::query(
            r#"INSERT INTO audit_settings (id, log_retention_days, log_sensitive_data, log_login_attempts,
               log_data_changes, log_api_requests, alert_on_suspicious, max_login_attempts,
               lockout_duration_minutes, password_expiry_days, require_mfa, session_timeout_minutes,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
               ON CONFLICT DO UPDATE SET log_retention_days = ?, max_login_attempts = ?, updated_at = ?"#
        )
        .bind(settings.base.id.to_string())
        .bind(settings.log_retention_days)
        .bind(settings.log_sensitive_data as i32)
        .bind(settings.log_login_attempts as i32)
        .bind(settings.log_data_changes as i32)
        .bind(settings.log_api_requests as i32)
        .bind(settings.alert_on_suspicious as i32)
        .bind(settings.max_login_attempts)
        .bind(settings.lockout_duration_minutes)
        .bind(settings.password_expiry_days)
        .bind(settings.require_mfa as i32)
        .bind(settings.session_timeout_minutes)
        .bind(settings.base.created_at.to_rfc3339())
        .bind(settings.base.updated_at.to_rfc3339())
        .bind(settings.base.created_by.map(|id| id.to_string()))
        .bind(settings.base.updated_by.map(|id| id.to_string()))
        .bind(settings.log_retention_days)
        .bind(settings.max_login_attempts)
        .bind(settings.base.updated_at.to_rfc3339())
        .execute(pool).await?;
        Ok(settings)
    }

    async fn create_integration(&self, pool: &SqlitePool, config: IntegrationConfig) -> Result<IntegrationConfig> {
        sqlx::query(
            r#"INSERT INTO integration_configs (id, name, code, integration_type, api_endpoint, api_key,
               api_secret, config_json, sync_enabled, sync_frequency, last_sync, sync_status, is_active,
               created_at, updated_at, created_by, updated_by)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(config.base.id.to_string())
        .bind(&config.name)
        .bind(&config.code)
        .bind(&config.integration_type)
        .bind(&config.api_endpoint)
        .bind(&config.api_key)
        .bind(&config.api_secret)
        .bind(&config.config_json)
        .bind(config.sync_enabled as i32)
        .bind(&config.sync_frequency)
        .bind(config.last_sync.map(|d| d.to_rfc3339()))
        .bind(&config.sync_status)
        .bind(config.is_active as i32)
        .bind(config.base.created_at.to_rfc3339())
        .bind(config.base.updated_at.to_rfc3339())
        .bind(config.base.created_by.map(|id| id.to_string()))
        .bind(config.base.updated_by.map(|id| id.to_string()))
        .execute(pool).await?;
        Ok(config)
    }

    async fn list_integrations(&self, pool: &SqlitePool) -> Result<Vec<IntegrationConfig>> {
        let rows: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
            r#"SELECT id, name, code, integration_type, api_endpoint, api_key, api_secret, config_json,
               sync_enabled, sync_frequency, last_sync, sync_status, is_active, created_at, updated_at
               FROM integration_configs WHERE is_active = 1"#
        )
        .fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| IntegrationConfig {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).unwrap().with_timezone(&chrono::Utc),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).unwrap().with_timezone(&chrono::Utc),
                created_by: None,
                updated_by: None,
            },
            name: r.get("name"),
            code: r.get("code"),
            integration_type: r.get("integration_type"),
            api_endpoint: r.get("api_endpoint"),
            api_key: r.get("api_key"),
            api_secret: r.get("api_secret"),
            config_json: r.get("config_json"),
            sync_enabled: r.get::<i32, _>("sync_enabled") == 1,
            sync_frequency: r.get("sync_frequency"),
            last_sync: r.get::<Option<&str>, _>("last_sync").and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok().map(|d| d.with_timezone(&chrono::Utc))),
            sync_status: r.get("sync_status"),
            is_active: r.get::<i32, _>("is_active") == 1,
        }).collect())
    }
}
