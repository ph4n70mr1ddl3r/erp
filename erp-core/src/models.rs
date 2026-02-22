use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BaseEntity {
    pub id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub created_by: Option<Uuid>,
    pub updated_by: Option<Uuid>,
}

impl BaseEntity {
    pub fn new() -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            created_at: now,
            updated_at: now,
            created_by: None,
            updated_by: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum Status {
    Active,
    Inactive,
    Draft,
    Pending,
    Approved,
    Rejected,
    Completed,
    Cancelled,
}

impl Default for Status {
    fn default() -> Self {
        Status::Active
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Money {
    pub amount: i64,
    pub currency: Currency,
}

impl Money {
    pub fn new(amount: i64, currency: Currency) -> Self {
        Self { amount, currency }
    }

    pub fn zero(currency: Currency) -> Self {
        Self {
            amount: 0,
            currency,
        }
    }

    pub fn from_decimal(amount: f64, currency: Currency) -> Self {
        Self {
            amount: (amount * 100.0) as i64,
            currency,
        }
    }

    pub fn to_decimal(&self) -> f64 {
        self.amount as f64 / 100.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum Currency {
    USD,
    EUR,
    GBP,
    JPY,
    CNY,
    CAD,
    AUD,
    CHF,
    INR,
    MXN,
}

impl Default for Currency {
    fn default() -> Self {
        Currency::USD
    }
}

impl std::fmt::Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Currency::USD => write!(f, "USD"),
            Currency::EUR => write!(f, "EUR"),
            Currency::GBP => write!(f, "GBP"),
            Currency::JPY => write!(f, "JPY"),
            Currency::CNY => write!(f, "CNY"),
            Currency::CAD => write!(f, "CAD"),
            Currency::AUD => write!(f, "AUD"),
            Currency::CHF => write!(f, "CHF"),
            Currency::INR => write!(f, "INR"),
            Currency::MXN => write!(f, "MXN"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Address {
    pub street: String,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContactInfo {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub website: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldDefinition {
    pub id: Uuid,
    pub entity_type: String,
    pub field_name: String,
    pub field_label: String,
    pub field_type: CustomFieldType,
    pub required: bool,
    pub options: Option<String>,
    pub sort_order: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CustomFieldType {
    Text,
    Number,
    Date,
    Boolean,
    Select,
    MultiSelect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldValue {
    pub id: Uuid,
    pub definition_id: Uuid,
    pub entity_id: Uuid,
    pub value: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
