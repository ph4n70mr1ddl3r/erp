use erp_core::Result;
use sqlx::SqlitePool;

pub struct SqliteLocaleRepository;

impl SqliteLocaleRepository {
    pub async fn create(&self, pool: &SqlitePool, locale: &crate::models::Locale) -> Result<crate::models::Locale> {
        sqlx::query(r#"
            INSERT INTO i18n_locales (id, code, name, native_name, language_code, country_code, 
                is_rtl, date_format, time_format, number_format, currency_symbol, currency_position, 
                decimal_separator, thousand_separator, status, is_default, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        "#)
        .bind(locale.id.to_string())
        .bind(&locale.code)
        .bind(&locale.name)
        .bind(&locale.native_name)
        .bind(&locale.language_code)
        .bind(&locale.country_code)
        .bind(locale.is_rtl)
        .bind(&locale.date_format)
        .bind(&locale.time_format)
        .bind(&locale.number_format)
        .bind(&locale.currency_symbol)
        .bind(&locale.currency_position)
        .bind(&locale.decimal_separator)
        .bind(&locale.thousand_separator)
        .bind(&locale.status)
        .bind(locale.is_default)
        .bind(locale.created_at.to_rfc3339())
        .bind(locale.updated_at.to_rfc3339())
        .execute(pool)
        .await?;
        Ok(locale.clone())
    }
}
