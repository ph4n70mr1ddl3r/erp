use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Locale {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub native_name: String,
    pub language_code: String,
    pub country_code: Option<String>,
    pub is_rtl: bool,
    pub date_format: String,
    pub time_format: String,
    pub number_format: String,
    pub currency_symbol: String,
    pub currency_position: String,
    pub decimal_separator: String,
    pub thousand_separator: String,
    pub status: String,
    pub is_default: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Translation {
    pub id: Uuid,
    pub locale_code: String,
    pub namespace: String,
    pub key: String,
    pub value: String,
    pub plural_form: Option<String>,
    pub context: Option<String>,
    pub is_approved: bool,
    pub translated_by: Option<Uuid>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserLocalePreference {
    pub id: Uuid,
    pub user_id: Uuid,
    pub locale_code: String,
    pub timezone: String,
    pub date_format_override: Option<String>,
    pub time_format_override: Option<String>,
    pub number_format_override: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissingTranslation {
    pub id: Uuid,
    pub locale_code: String,
    pub namespace: String,
    pub key: String,
    pub first_seen_at: DateTime<Utc>,
    pub usage_count: i32,
    pub last_used_at: Option<DateTime<Utc>>,
    pub priority: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
