use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FavoriteType {
    Customer,
    Vendor,
    Product,
    Order,
    Invoice,
    Quote,
    PurchaseOrder,
    Employee,
    Project,
    Ticket,
    Report,
    Page,
}

impl std::fmt::Display for FavoriteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FavoriteType::Customer => write!(f, "Customer"),
            FavoriteType::Vendor => write!(f, "Vendor"),
            FavoriteType::Product => write!(f, "Product"),
            FavoriteType::Order => write!(f, "Order"),
            FavoriteType::Invoice => write!(f, "Invoice"),
            FavoriteType::Quote => write!(f, "Quote"),
            FavoriteType::PurchaseOrder => write!(f, "PurchaseOrder"),
            FavoriteType::Employee => write!(f, "Employee"),
            FavoriteType::Project => write!(f, "Project"),
            FavoriteType::Ticket => write!(f, "Ticket"),
            FavoriteType::Report => write!(f, "Report"),
            FavoriteType::Page => write!(f, "Page"),
        }
    }
}

impl std::str::FromStr for FavoriteType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "customer" => Ok(FavoriteType::Customer),
            "vendor" => Ok(FavoriteType::Vendor),
            "product" => Ok(FavoriteType::Product),
            "order" | "salesorder" => Ok(FavoriteType::Order),
            "invoice" => Ok(FavoriteType::Invoice),
            "quote" | "quotation" => Ok(FavoriteType::Quote),
            "purchaseorder" | "purchase_order" | "po" => Ok(FavoriteType::PurchaseOrder),
            "employee" => Ok(FavoriteType::Employee),
            "project" => Ok(FavoriteType::Project),
            "ticket" => Ok(FavoriteType::Ticket),
            "report" => Ok(FavoriteType::Report),
            "page" => Ok(FavoriteType::Page),
            _ => Err(format!("Unknown favorite type: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favorite {
    pub base: BaseEntity,
    pub user_id: Uuid,
    pub favorite_type: FavoriteType,
    pub entity_id: Option<Uuid>,
    pub entity_name: String,
    pub entity_code: Option<String>,
    pub notes: Option<String>,
}

impl Favorite {
    pub fn new(
        user_id: Uuid,
        favorite_type: FavoriteType,
        entity_id: Option<Uuid>,
        entity_name: String,
        entity_code: Option<String>,
    ) -> Self {
        Self {
            base: BaseEntity::new(),
            user_id,
            favorite_type,
            entity_id,
            entity_name,
            entity_code,
            notes: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFavoriteRequest {
    pub favorite_type: String,
    pub entity_id: Option<Uuid>,
    pub entity_name: String,
    pub entity_code: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteListResponse {
    pub items: Vec<Favorite>,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoritesByType {
    pub favorite_type: FavoriteType,
    pub items: Vec<Favorite>,
}
