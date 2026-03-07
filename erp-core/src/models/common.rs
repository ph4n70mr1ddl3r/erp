use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type, Default)]
#[sqlx(type_name = "TEXT")]
pub enum Status {
    #[default]
    Active,
    Inactive,
    Draft,
    Pending,
    Approved,
    Rejected,
    Completed,
    Cancelled,
}

impl std::str::FromStr for Status {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "active" => Ok(Status::Active),
            "inactive" => Ok(Status::Inactive),
            "draft" => Ok(Status::Draft),
            "pending" => Ok(Status::Pending),
            "approved" => Ok(Status::Approved),
            "rejected" => Ok(Status::Rejected),
            "completed" => Ok(Status::Completed),
            "cancelled" | "canceled" => Ok(Status::Cancelled),
            _ => Err(format!("Invalid status: {}", s)),
        }
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

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq, Default)]
#[sqlx(type_name = "TEXT")]
pub enum Currency {
    #[default]
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

impl std::str::FromStr for Currency {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "USD" => Ok(Currency::USD),
            "EUR" => Ok(Currency::EUR),
            "GBP" => Ok(Currency::GBP),
            "JPY" => Ok(Currency::JPY),
            "CNY" => Ok(Currency::CNY),
            "CAD" => Ok(Currency::CAD),
            "AUD" => Ok(Currency::AUD),
            "CHF" => Ok(Currency::CHF),
            "INR" => Ok(Currency::INR),
            "MXN" => Ok(Currency::MXN),
            _ => Err(format!("Invalid currency: {}", s)),
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
