use sqlx::{SqlitePool, Row};
use uuid::Uuid;
use chrono::Utc;
use crate::models::*;
use erp_core::Result;

pub struct I18nService;

impl Default for I18nService {
    fn default() -> Self {
        Self::new()
    }
}

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
        let row = sqlx::query(
            "SELECT id, code, name, native_name, language_code, country_code, is_rtl, date_format, time_format, number_format, currency_symbol, currency_position, decimal_separator, thousand_separator, status, is_default, created_at, updated_at FROM i18n_locales WHERE code = ?"
        )
        .bind(code)
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| Locale {
            id: r.get::<String, _>("id").parse().unwrap_or_default(),
            code: r.get("code"),
            name: r.get("name"),
            native_name: r.get("native_name"),
            language_code: r.get("language_code"),
            country_code: r.get("country_code"),
            is_rtl: r.get::<i32, _>("is_rtl") != 0,
            date_format: r.get("date_format"),
            time_format: r.get("time_format"),
            number_format: r.get("number_format"),
            currency_symbol: r.get("currency_symbol"),
            currency_position: r.get("currency_position"),
            decimal_separator: r.get("decimal_separator"),
            thousand_separator: r.get("thousand_separator"),
            status: r.get("status"),
            is_default: r.get::<i32, _>("is_default") != 0,
            created_at: r.get::<String, _>("created_at").parse().unwrap_or_default(),
            updated_at: r.get::<String, _>("updated_at").parse().unwrap_or_default(),
        }))
    }

    pub async fn list_locales(&self, pool: &SqlitePool, active_only: bool) -> Result<Vec<Locale>> {
        let query = if active_only {
            "SELECT id, code, name, native_name, language_code, country_code, is_rtl, date_format, time_format, number_format, currency_symbol, currency_position, decimal_separator, thousand_separator, status, is_default, created_at, updated_at FROM i18n_locales WHERE status = 'Active'"
        } else {
            "SELECT id, code, name, native_name, language_code, country_code, is_rtl, date_format, time_format, number_format, currency_symbol, currency_position, decimal_separator, thousand_separator, status, is_default, created_at, updated_at FROM i18n_locales"
        };
        
        let rows = sqlx::query(query)
            .fetch_all(pool)
            .await?;

        Ok(rows.into_iter().map(|r| Locale {
            id: r.get::<String, _>("id").parse().unwrap_or_default(),
            code: r.get("code"),
            name: r.get("name"),
            native_name: r.get("native_name"),
            language_code: r.get("language_code"),
            country_code: r.get("country_code"),
            is_rtl: r.get::<i32, _>("is_rtl") != 0,
            date_format: r.get("date_format"),
            time_format: r.get("time_format"),
            number_format: r.get("number_format"),
            currency_symbol: r.get("currency_symbol"),
            currency_position: r.get("currency_position"),
            decimal_separator: r.get("decimal_separator"),
            thousand_separator: r.get("thousand_separator"),
            status: r.get("status"),
            is_default: r.get::<i32, _>("is_default") != 0,
            created_at: r.get::<String, _>("created_at").parse().unwrap_or_default(),
            updated_at: r.get::<String, _>("updated_at").parse().unwrap_or_default(),
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
        let rows = sqlx::query(
            "SELECT id, locale_code, namespace, key, value, plural_form, context, is_approved, translated_by, reviewed_by, reviewed_at, created_at, updated_at FROM i18n_translations WHERE locale_code = ? AND namespace = ?"
        )
        .bind(locale_code)
        .bind(namespace)
        .fetch_all(pool)
        .await?;

        Ok(rows.into_iter().map(|r| Translation {
            id: r.get::<String, _>("id").parse().unwrap_or_default(),
            locale_code: r.get("locale_code"),
            namespace: r.get("namespace"),
            key: r.get("key"),
            value: r.get("value"),
            plural_form: r.get("plural_form"),
            context: r.get("context"),
            is_approved: r.get::<i32, _>("is_approved") != 0,
            translated_by: r.get::<Option<String>, _>("translated_by").and_then(|s| s.parse().ok()),
            reviewed_by: r.get::<Option<String>, _>("reviewed_by").and_then(|s| s.parse().ok()),
            reviewed_at: r.get::<Option<String>, _>("reviewed_at").and_then(|s| s.parse().ok()),
            created_at: r.get::<String, _>("created_at").parse().unwrap_or_default(),
            updated_at: r.get::<String, _>("updated_at").parse().unwrap_or_default(),
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
        let row = sqlx::query(
            "SELECT id, user_id, locale_code, timezone, date_format_override, time_format_override, number_format_override, created_at, updated_at FROM i18n_user_preferences WHERE user_id = ?"
        )
        .bind(user_id.to_string())
        .fetch_optional(pool)
        .await?;

        Ok(row.map(|r| UserLocalePreference {
            id: r.get::<String, _>("id").parse().unwrap_or_default(),
            user_id: r.get::<String, _>("user_id").parse().unwrap_or_default(),
            locale_code: r.get("locale_code"),
            timezone: r.get("timezone"),
            date_format_override: r.get("date_format_override"),
            time_format_override: r.get("time_format_override"),
            number_format_override: r.get("number_format_override"),
            created_at: r.get::<String, _>("created_at").parse().unwrap_or_default(),
            updated_at: r.get::<String, _>("updated_at").parse().unwrap_or_default(),
        }))
    }
}
