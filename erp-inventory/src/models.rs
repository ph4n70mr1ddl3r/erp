use chrono::{DateTime, Utc};
use erp_core::{Address, BaseEntity, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub base: BaseEntity,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub product_type: ProductType,
    pub category_id: Option<Uuid>,
    pub unit_of_measure: String,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ProductType {
    Goods,
    Service,
    Digital,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductCategory {
    pub base: BaseEntity,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Warehouse {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub address: Address,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockLocation {
    pub base: BaseEntity,
    pub warehouse_id: Uuid,
    pub code: String,
    pub name: String,
    pub location_type: LocationType,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LocationType {
    Receiving,
    Storage,
    Picking,
    Shipping,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockLevel {
    pub id: Uuid,
    pub product_id: Uuid,
    pub location_id: Uuid,
    pub quantity: i64,
    pub reserved_quantity: i64,
    pub available_quantity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockMovement {
    pub base: BaseEntity,
    pub movement_number: String,
    pub movement_type: MovementType,
    pub product_id: Uuid,
    pub from_location_id: Option<Uuid>,
    pub to_location_id: Uuid,
    pub quantity: i64,
    pub reference: Option<String>,
    pub date: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum MovementType {
    Receipt,
    Issue,
    Transfer,
    Adjustment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceList {
    pub base: BaseEntity,
    pub name: String,
    pub currency: String,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriceListItem {
    pub id: Uuid,
    pub price_list_id: Uuid,
    pub product_id: Uuid,
    pub price: Money,
    pub min_quantity: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lot {
    pub id: Uuid,
    pub lot_number: String,
    pub product_id: Uuid,
    pub serial_number: Option<String>,
    pub manufacture_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub quantity: i64,
    pub status: LotStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LotStatus {
    Active,
    Expired,
    Quarantined,
    Depleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LotTransaction {
    pub id: Uuid,
    pub lot_id: Uuid,
    pub transaction_type: LotTransactionType,
    pub quantity: i64,
    pub reference_type: Option<String>,
    pub reference_id: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LotTransactionType {
    Receipt,
    Issue,
    Transfer,
    Adjustment,
    Expiry,
}
