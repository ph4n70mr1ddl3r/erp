use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status};
use crate::models::*;
use crate::repository::*;

pub struct ProductService {
    repo: SqliteProductRepository,
}

impl ProductService {
    pub fn new() -> Self {
        Self { repo: SqliteProductRepository }
    }

    pub async fn get_product(&self, pool: &SqlitePool, id: Uuid) -> Result<Product> {
        self.repo.find_by_id(pool, id).await
    }

    pub async fn get_product_by_sku(&self, pool: &SqlitePool, sku: &str) -> Result<Product> {
        self.repo.find_by_sku(pool, sku).await
    }

    pub async fn create_product(&self, pool: &SqlitePool, mut product: Product) -> Result<Product> {
        self.validate_product(&product)?;
        
        if self.repo.find_by_sku(pool, &product.sku).await.is_ok() {
            return Err(Error::Conflict(format!("Product with SKU '{}' already exists", product.sku)));
        }
        
        product.base = BaseEntity::new();
        product.status = Status::Active;
        self.repo.create(pool, product).await
    }

    pub async fn update_product(&self, pool: &SqlitePool, product: Product) -> Result<Product> {
        self.validate_product(&product)?;
        self.repo.update(pool, product).await
    }

    pub async fn list_products(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Product>> {
        self.repo.find_all(pool, pagination).await
    }

    pub async fn delete_product(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete(pool, id).await
    }

    fn validate_product(&self, product: &Product) -> Result<()> {
        if product.sku.is_empty() {
            return Err(Error::validation("SKU is required"));
        }
        if product.name.is_empty() {
            return Err(Error::validation("Product name is required"));
        }
        if product.unit_of_measure.is_empty() {
            return Err(Error::validation("Unit of measure is required"));
        }
        Ok(())
    }
}

pub struct WarehouseService {
    repo: SqliteWarehouseRepository,
}

impl WarehouseService {
    pub fn new() -> Self {
        Self { repo: SqliteWarehouseRepository }
    }

    pub async fn get_warehouse(&self, pool: &SqlitePool, id: Uuid) -> Result<Warehouse> {
        self.repo.find_by_id(pool, id).await
    }

    pub async fn list_warehouses(&self, pool: &SqlitePool) -> Result<Vec<Warehouse>> {
        self.repo.find_all(pool).await
    }

    pub async fn create_warehouse(&self, pool: &SqlitePool, mut warehouse: Warehouse) -> Result<Warehouse> {
        if warehouse.code.is_empty() {
            return Err(Error::validation("Warehouse code is required"));
        }
        if warehouse.name.is_empty() {
            return Err(Error::validation("Warehouse name is required"));
        }
        
        warehouse.base = BaseEntity::new();
        warehouse.status = Status::Active;
        self.repo.create(pool, warehouse).await
    }

    pub async fn update_warehouse(&self, pool: &SqlitePool, warehouse: Warehouse) -> Result<Warehouse> {
        self.repo.update(pool, warehouse).await
    }
}

pub struct StockService {
    repo: SqliteStockMovementRepository,
}

impl StockService {
    pub fn new() -> Self {
        Self { repo: SqliteStockMovementRepository }
    }

    pub async fn get_stock_level(&self, pool: &SqlitePool, product_id: Uuid, location_id: Uuid) -> Result<StockLevel> {
        self.repo.get_stock_level(pool, product_id, location_id).await
    }

    pub async fn record_movement(&self, pool: &SqlitePool, mut movement: StockMovement) -> Result<StockMovement> {
        if movement.quantity <= 0 {
            return Err(Error::validation("Movement quantity must be positive"));
        }
        
        movement.base = BaseEntity::new();
        movement.movement_number = self.generate_movement_number();
        movement.date = Utc::now();
        
        self.repo.record(pool, movement).await
    }

    pub async fn get_product_stock(&self, pool: &SqlitePool, product_id: Uuid) -> Result<Vec<StockLevel>> {
        self.repo.get_product_stock(pool, product_id).await
    }

    fn generate_movement_number(&self) -> String {
        format!("SM-{}", chrono::Local::now().format("%Y%m%d%H%M%S"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use erp_core::Address;
    
    fn create_test_product() -> Product {
        Product {
            base: BaseEntity::new(),
            sku: "SKU-001".to_string(),
            name: "Test Product".to_string(),
            description: None,
            product_type: ProductType::Goods,
            category_id: None,
            unit_of_measure: "PCS".to_string(),
            status: Status::Active,
        }
    }
    
    fn create_test_warehouse() -> Warehouse {
        Warehouse {
            base: BaseEntity::new(),
            code: "WH-001".to_string(),
            name: "Test Warehouse".to_string(),
            address: Address {
                street: "123 Main St".to_string(),
                city: "Test City".to_string(),
                state: Some("TS".to_string()),
                postal_code: "12345".to_string(),
                country: "US".to_string(),
            },
            status: Status::Active,
        }
    }
    
    #[test]
    fn test_validate_product_valid() {
        let svc = ProductService::new();
        let product = create_test_product();
        
        let result = svc.validate_product(&product);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_product_empty_sku() {
        let svc = ProductService::new();
        let mut product = create_test_product();
        product.sku = String::new();
        
        let result = svc.validate_product(&product);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("SKU"));
    }
    
    #[test]
    fn test_validate_product_empty_name() {
        let svc = ProductService::new();
        let mut product = create_test_product();
        product.name = String::new();
        
        let result = svc.validate_product(&product);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("name"));
    }
    
    #[test]
    fn test_validate_product_empty_unit() {
        let svc = ProductService::new();
        let mut product = create_test_product();
        product.unit_of_measure = String::new();
        
        let result = svc.validate_product(&product);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unit of measure"));
    }
    
    #[test]
    fn test_validate_warehouse_valid() {
        let warehouse = create_test_warehouse();
        
        let result = validate_warehouse(&warehouse);
        assert!(result.is_ok());
    }
    
    #[test]
    fn test_validate_warehouse_empty_code() {
        let mut warehouse = create_test_warehouse();
        warehouse.code = String::new();
        
        let result = validate_warehouse(&warehouse);
        assert!(result.is_err());
    }
    
    fn validate_warehouse(warehouse: &Warehouse) -> Result<()> {
        if warehouse.code.is_empty() {
            return Err(Error::validation("Warehouse code is required"));
        }
        if warehouse.name.is_empty() {
            return Err(Error::validation("Warehouse name is required"));
        }
        Ok(())
    }
    
    #[test]
    fn test_movement_number_format() {
        let svc = StockService::new();
        let number = svc.generate_movement_number();
        
        assert!(number.starts_with("SM-"));
        assert_eq!(number.len(), 17);
    }
}

pub struct LotService;

impl LotService {
    pub fn new() -> Self { Self }

    pub async fn create_lot(
        pool: &SqlitePool,
        lot_number: &str,
        product_id: Uuid,
        serial_number: Option<&str>,
        manufacture_date: Option<String>,
        expiry_date: Option<String>,
        quantity: i64,
        notes: Option<&str>,
    ) -> Result<Lot> {
        let now = chrono::Utc::now();
        let lot = Lot {
            id: Uuid::new_v4(),
            lot_number: lot_number.to_string(),
            product_id,
            serial_number: serial_number.map(|s| s.to_string()),
            manufacture_date: manufacture_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            expiry_date: expiry_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            quantity,
            status: LotStatus::Active,
            notes: notes.map(|s| s.to_string()),
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO lots (id, lot_number, product_id, serial_number, manufacture_date, expiry_date, quantity, status, notes, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(lot.id.to_string())
        .bind(&lot.lot_number)
        .bind(lot.product_id.to_string())
        .bind(&lot.serial_number)
        .bind(lot.manufacture_date.map(|d| d.to_rfc3339()))
        .bind(lot.expiry_date.map(|d| d.to_rfc3339()))
        .bind(lot.quantity)
        .bind("Active")
        .bind(&lot.notes)
        .bind(lot.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(lot)
    }

    pub async fn get_lot(pool: &SqlitePool, id: Uuid) -> Result<Lot> {
        let row = sqlx::query_as::<_, LotRow>(
            "SELECT id, lot_number, product_id, serial_number, manufacture_date, expiry_date, quantity, status, notes, created_at
             FROM lots WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?
        .ok_or_else(|| Error::not_found("Lot", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn list_lots_for_product(pool: &SqlitePool, product_id: Uuid) -> Result<Vec<Lot>> {
        let rows = sqlx::query_as::<_, LotRow>(
            "SELECT id, lot_number, product_id, serial_number, manufacture_date, expiry_date, quantity, status, notes, created_at
             FROM lots WHERE product_id = ? ORDER BY created_at DESC"
        )
        .bind(product_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn record_transaction(
        pool: &SqlitePool,
        lot_id: Uuid,
        transaction_type: LotTransactionType,
        quantity: i64,
        reference_type: Option<&str>,
        reference_id: Option<&str>,
    ) -> Result<LotTransaction> {
        let now = chrono::Utc::now();
        let tx = LotTransaction {
            id: Uuid::new_v4(),
            lot_id,
            transaction_type: transaction_type.clone(),
            quantity,
            reference_type: reference_type.map(|s| s.to_string()),
            reference_id: reference_id.map(|s| s.to_string()),
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO lot_transactions (id, lot_id, transaction_type, quantity, reference_type, reference_id, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(tx.id.to_string())
        .bind(tx.lot_id.to_string())
        .bind(format!("{:?}", tx.transaction_type))
        .bind(tx.quantity)
        .bind(&tx.reference_type)
        .bind(&tx.reference_id)
        .bind(tx.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        let delta = match transaction_type {
            LotTransactionType::Receipt => quantity,
            LotTransactionType::Issue => -quantity,
            LotTransactionType::Transfer => 0,
            LotTransactionType::Adjustment => quantity,
            LotTransactionType::Expiry => -quantity,
        };
        
        sqlx::query(
            "UPDATE lots SET quantity = quantity + ? WHERE id = ?"
        )
        .bind(delta)
        .bind(lot_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(tx)
    }
}

#[derive(sqlx::FromRow)]
struct LotRow {
    id: String,
    lot_number: String,
    product_id: String,
    serial_number: Option<String>,
    manufacture_date: Option<String>,
    expiry_date: Option<String>,
    quantity: i64,
    status: String,
    notes: Option<String>,
    created_at: String,
}

impl From<LotRow> for Lot {
    fn from(r: LotRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            lot_number: r.lot_number,
            product_id: Uuid::parse_str(&r.product_id).unwrap_or_default(),
            serial_number: r.serial_number,
            manufacture_date: r.manufacture_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            expiry_date: r.expiry_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            quantity: r.quantity,
            status: match r.status.as_str() {
                "Expired" => LotStatus::Expired,
                "Quarantined" => LotStatus::Quarantined,
                "Depleted" => LotStatus::Depleted,
                _ => LotStatus::Active,
            },
            notes: r.notes,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}
