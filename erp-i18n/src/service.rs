use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::*;
use erp_core::Result;

pub struct I18nService;

impl I18nService {
    pub fn new() -> Self { Self }

    pub async fn create_locale(&self, pool: &SqlitePool, code: String, name: String, 
        native_name: String, language_code: String) -> Result<Locale> {
        let id = Uuid::new_v4();
        let now = Utc::now();
        sqlx::query(r#"
            INSERT INTO i18n_locales (id, code, name, native_name, language_code, country_code, 
                is_rtl, date_format, time_format, number_format, currency_symbol, currency_position, 
                decimal_separator, thousand_separator, status, is_default, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(id.to_string())
        .bind(&code)
        .bind(&name)
        .bind(&native_name)
        .bind(&language_code)
        .bind(Option::<String>::None)
        .bind(false)
        .bind("YYYY-MM-DD")
        .bind("HH:mm:ss")
        .bind("#,##0.00")
        .bind("$")
        .bind("before")
        .bind(".")
        .bind(",")
        .bind("Active")
        .bind(false)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(Locale {
            id,
            code,
            name,
            native_name,
            language_code,
            country_code: None,
            is_rtl: false,
            date_format: "YYYY-MM-DD".to_string(),
            time_format: "HH:mm:ss".to_string(),
            number_format: "#,##0.00".to_string(),
            currency_symbol: "$".to_string(),
            currency_position: "before".to_string(),
            decimal_separator: ".".to_string(),
            thousand_separator: ",".to_string(),
            status: "Active".to_string(),
            is_default: false,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_locale(&self, pool: &SqlitePool, code: &str) -> Result<Option<Locale>> {
        let row = sqlx::query_as::<_, (String, String, String, String, String, Option<String>, bool, String, String, String, String, String, String, String, String, bool, String, String)>(
            "SELECT id, code, name, native_name, language_code, country_code, is_rtl, date_format, time_format, number_format, currency_symbol, currency_position, decimal_separator, thousand_separator, status, is_default, created_at, updated_at FROM i18n_locales WHERE code = ?"
        )
        .bind(code)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| Locale {
            id: r.0.parse().unwrap_or_default(),
            code: r.1,
            name: r.2,
            native_name: r.3,
            language_code: r.4,
            country_code: r.5,
            is_rtl: r.6,
            date_format: r.7,
            time_format: r.8,
            number_format: r.9,
            currency_symbol: r.10,
            currency_position: r.11,
            decimal_separator: r.12,
            thousand_separator: r.13,
            status: r.14,
            is_default: r.15,
            created_at: r.16.parse().unwrap_or_default(),
            updated_at: r.17.parse().unwrap_or_default(),
        }))
    }

    pub async fn list_locales(&self, pool: &SqlitePool, active_only: bool) -> Result<Vec<Locale>> {
        let query = if active_only {
            "SELECT id, code, name, native_name, language_code, country_code, is_rtl, date_format, time_format, number_format, currency_symbol, currency_position, decimal_separator, thousand_separator, status, is_default, created_at, updated_at FROM i18n_locales WHERE status = 'Active'"
        } else {
            "SELECT id, code, name, native_name, language_code, country_code, is_rtl, date_format, time_format, number_format, currency_symbol, currency_position, decimal_separator, thousand_separator, status, is_default, created_at, updated_at FROM i18n_locales"
        };
        
        let rows = sqlx::query_as::<_, (String, String, String, String, String, Option<String>, bool, String, String, String, String, String, String, String, String, bool, String, String)>(query)
            .fetch_all(pool)
            .await?;

        Ok(rows.into_iter().map(|r| Locale {
            id: r.0.parse().unwrap_or_default(),
            code: r.1,
            name: r.2,
            native_name: r.3,
            language_code: r.4,
            country_code: r.5,
            is_rtl: r.6,
            date_format: r.7,
            time_format: r.8,
            number_format: r.9,
            currency_symbol: r.10,
            currency_position: r.11,
            decimal_separator: r.12,
            thousand_separator: r.13,
            status: r.14,
            is_default: r.15,
            created_at: r.16.parse().unwrap_or_default(),
            updated_at: r.17.parse().unwrap_or_default(),
        }).collect())
    }

    pub async fn set_translation(&self, pool: &SqlitePool, locale_code: String, namespace: String, 
        key: String, value: String) -> Result<Translation> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(r#"
            INSERT INTO i18n_translations (id, locale_code, namespace, key, value, plural_form, 
                context, is_approved, translated_by, reviewed_by, reviewed_at, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(id.to_string())
        .bind(&locale_code)
        .bind(&namespace)
        .bind(&key)
        .bind(&value)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(true)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(Translation {
            id,
            locale_code,
            namespace,
            key,
            value,
            plural_form: None,
            context: None,
            is_approved: true,
            translated_by: None,
            reviewed_by: None,
            reviewed_at: Some(now),
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_translations(&self, pool: &SqlitePool, locale_code: &str, namespace: &str) -> Result<Vec<Translation>> {
        let rows = sqlx::query_as::<_, (String, String, String, String, String, Option<String>, Option<String>, bool, Option<String>, Option<String>, Option<String>, String, String)>(
            "SELECT id, locale_code, namespace, key, value, plural_form, context, is_approved, translated_by, reviewed_by, reviewed_at, created_at, updated_at FROM i18n_translations WHERE locale_code = ? AND namespace = ?"
        )
        .bind(locale_code)
        .bind(namespace)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| Translation {
            id: r.0.parse().unwrap_or_default(),
            locale_code: r.1,
            namespace: r.2,
            key: r.3,
            value: r.4,
            plural_form: r.5,
            context: r.6,
            is_approved: r.7,
            translated_by: r.8.and_then(|s| s.parse().ok()),
            reviewed_by: r.9.and_then(|s| s.parse().ok()),
            reviewed_at: r.10.and_then(|s| s.parse().ok()),
            created_at: r.11.parse().unwrap_or_default(),
            updated_at: r.12.parse().unwrap_or_default(),
        }).collect())
    }

    pub async fn get_translations_map(&self, pool: &SqlitePool, locale_code: &str, namespace: &str) -> Result<serde_json::Value> {
        let translations = self.get_translations(pool, locale_code, namespace).await?;
        let map: std::collections::HashMap<String, String> = translations
            .into_iter()
            .map(|t| (t.key, t.value))
            .collect();
        Ok(serde_json::to_value(map).unwrap_or(serde_json::json!({})))
    }

    pub async fn set_user_preference(&self, pool: &SqlitePool, user_id: Uuid, locale_code: String, 
        timezone: String) -> Result<UserLocalePreference> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(r#"
            INSERT INTO i18n_user_preferences (id, user_id, locale_code, timezone, 
                date_format_override, time_format_override, number_format_override, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            ON CONFLICT(user_id) DO UPDATE SET locale_code = excluded.locale_code, 
                timezone = excluded.timezone, updated_at = excluded.updated_at
        "#)
        .bind(id.to_string())
        .bind(user_id.to_string())
        .bind(&locale_code)
        .bind(&timezone)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(Option::<String>::None)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;

        Ok(UserLocalePreference {
            id,
            user_id,
            locale_code,
            timezone,
            date_format_override: None,
            time_format_override: None,
            number_format_override: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn get_user_preference(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Option<UserLocalePreference>> {
        let row = sqlx::query_as::<_, (String, String, String, String, Option<String>, Option<String>, Option<String>, String, String)>(
            "SELECT id, user_id, locale_code, timezone, date_format_override, time_format_override, number_format_override, created_at, updated_at FROM i18n_user_preferences WHERE user_id = ?"
        )
        .bind(user_id.to_string())
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| UserLocalePreference {
            id: r.0.parse().unwrap_or_default(),
            user_id: r.1.parse().unwrap_or_default(),
            locale_code: r.2,
            timezone: r.3,
            date_format_override: r.4,
            time_format_override: r.5,
            number_format_override: r.6,
            created_at: r.7.parse().unwrap_or_default(),
            updated_at: r.8.parse().unwrap_or_default(),
        }))
    }
}
