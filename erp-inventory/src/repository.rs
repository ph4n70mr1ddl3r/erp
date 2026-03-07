use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status};
use crate::models::*;

#[async_trait]
impl ProductRepository for SqliteProductRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Product> {
        let row = sqlx::query_as::<_, ProductRow>(
            "SELECT id, sku, name, description, product_type, category_id, 
                    unit_of_measure, status, created_at, updated_at, created_by, updated_by
             FROM products WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("Product", &id.to_string()))?;
        
        Ok(row.into_product()?)
    }

    async fn find_by_sku(&self, pool: &SqlitePool, sku: &str) -> Result<Product> {
        let row = sqlx::query_as::<_, ProductRow>(
            "SELECT id, sku, name, description, product_type, category_id, 
                    unit_of_measure, status, created_at, updated_at, created_by, updated_by
             FROM products WHERE sku = ?"
        )
        .bind(sku)
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("Product", sku))?;
        
        Ok(row.into_product()?)
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Product>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products WHERE status != 'Deleted'")
            .fetch_one(pool)
            .await?;
        
        let rows = sqlx::query_as::<_, ProductRow>(
            "SELECT id, sku, name, description, product_type, category_id, 
                    unit_of_measure, status, created_at, updated_at, created_by, updated_by
             FROM products 
             WHERE status != 'Deleted'
             ORDER BY created_at DESC 
             LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await?;
        
        let items: Result<Vec<Product>> = rows.into_iter().map(|r| r.into_product()).collect();
        Ok(Paginated::new(items?, count.0 as u64, pagination))
    }

    async fn create(&self, pool: &SqlitePool, product: Product) -> Result<Product> {
        let now = Utc::now();
        sqlx::query("INSERT INTO products (id, sku, name, description, product_type, category_id, 
             unit_of_measure, status, created_at, updated_at, created_by, updated_by)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(product.base.id.to_string())
        .bind(&product.sku)
        .bind(&product.name)
        .bind(&product.description)
        .bind(format!("{:?}", product.product_type))
        .bind(product.category_id.map(|id| id.to_string()))
        .bind(&product.unit_of_measure)
        .bind(format!("{:?}", product.status))
        .bind(product.base.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(product.base.created_by.map(|id| id.to_string()))
        .bind(product.base.updated_by.map(|id| id.to_string()))
        .execute(pool)
        .await?;
        
        Ok(product)
    }

    async fn update(&self, pool: &SqlitePool, product: Product) -> Result<Product> {
        let now = Utc::now();
        let rows = sqlx::query("UPDATE products SET sku = ?, name = ?, description = ?, product_type = ?, 
             category_id = ?, unit_of_measure = ?, status = ?, updated_at = ?, updated_by = ?
             WHERE id = ?")
        .bind(&product.sku)
        .bind(&product.name)
        .bind(&product.description)
        .bind(format!("{:?}", product.product_type))
        .bind(product.category_id.map(|id| id.to_string()))
        .bind(&product.unit_of_measure)
        .bind(format!("{:?}", product.status))
        .bind(now.to_rfc3339())
        .bind(product.base.updated_by.map(|id| id.to_string()))
        .bind(product.base.id.to_string())
        .execute(pool)
        .await?;
        
        if rows.rows_affected() == 0 {
            return Err(Error::not_found("Product", &product.base.id.to_string()));
        }
        
        Ok(product)
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let rows = sqlx::query("UPDATE products SET status = 'Deleted', updated_at = ? WHERE id = ?")
        .bind(Utc::now().to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await?;
        
        if rows.rows_affected() == 0 {
            return Err(Error::not_found("Product", &id.to_string()));
        }
        
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct ProductRow {
    id: String,
    sku: String,
    name: String,
    description: Option<String>,
    product_type: String,
    category_id: Option<String>,
    unit_of_measure: String,
    status: String,
    created_at: String,
    updated_at: String,
    created_by: Option<String>,
    updated_by: Option<String>,
}

impl ProductRow {
    fn into_product(self) -> Result<Product> {
        let id = Uuid::parse_str(&self.id)
            .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid UUID '{}': {}", self.id, e)))?;
        
        Ok(Product {
            base: BaseEntity {
                id,
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
                    .map(|d| d.with_timezone(&Utc))
                    .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid datetime: {}", e)))?,
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at)
                    .map(|d| d.with_timezone(&Utc))
                    .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid datetime: {}", e)))?,
                created_by: self.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: self.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            sku: self.sku,
            name: self.name,
            description: self.description,
            product_type: match self.product_type.as_str() {
                "Service" => ProductType::Service,
                "Digital" => ProductType::Digital,
                _ => ProductType::Goods,
            },
            category_id: self.category_id.and_then(|s| Uuid::parse_str(&s).ok()),
            unit_of_measure: self.unit_of_measure,
            status: match self.status.as_str() {
                "Inactive" => Status::Inactive,
                _ => Status::Active,
            },
        })
    }
}

pub struct SqliteProductRepository;

#[async_trait]
pub trait ProductRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Product>;
    async fn find_by_sku(&self, pool: &SqlitePool, sku: &str) -> Result<Product>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Product>>;
    async fn create(&self, pool: &SqlitePool, product: Product) -> Result<Product>;
    async fn update(&self, pool: &SqlitePool, product: Product) -> Result<Product>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

#[async_trait]
pub trait WarehouseRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Warehouse>;
    async fn find_all(&self, pool: &SqlitePool) -> Result<Vec<Warehouse>>;
    async fn create(&self, pool: &SqlitePool, warehouse: Warehouse) -> Result<Warehouse>;
    async fn update(&self, pool: &SqlitePool, warehouse: Warehouse) -> Result<Warehouse>;
}

#[async_trait]
pub trait StockMovementRepository: Send + Sync {
    async fn record(&self, pool: &SqlitePool, movement: StockMovement) -> Result<StockMovement>;
    async fn get_stock_level(&self, pool: &SqlitePool, product_id: Uuid, location_id: Uuid) -> Result<StockLevel>;
    async fn get_product_stock(&self, pool: &SqlitePool, product_id: Uuid) -> Result<Vec<StockLevel>>;
}

pub struct SqliteWarehouseRepository;

#[derive(sqlx::FromRow)]
struct WarehouseRow {
    id: String,
    code: String,
    name: String,
    address_street: Option<String>,
    address_city: Option<String>,
    address_state: Option<String>,
    address_postal_code: Option<String>,
    address_country: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
}

impl WarehouseRow {
    fn into_warehouse(self) -> Result<Warehouse> {
        use erp_core::Address;
        let id = Uuid::parse_str(&self.id)
            .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid UUID '{}': {}", self.id, e)))?;
        
        Ok(Warehouse {
            base: BaseEntity {
                id,
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
                    .map(|d| d.with_timezone(&Utc))
                    .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid datetime: {}", e)))?,
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at)
                    .map(|d| d.with_timezone(&Utc))
                    .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid datetime: {}", e)))?,
                created_by: None,
                updated_by: None,
            },
            code: self.code,
            name: self.name,
            address: Address {
                street: self.address_street.unwrap_or_default(),
                city: self.address_city.unwrap_or_default(),
                state: self.address_state,
                postal_code: self.address_postal_code.unwrap_or_default(),
                country: self.address_country.unwrap_or_default(),
            },
            status: match self.status.as_str() {
                "Inactive" => Status::Inactive,
                _ => Status::Active,
            },
        })
    }
}

#[async_trait]
impl WarehouseRepository for SqliteWarehouseRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Warehouse> {
        let row = sqlx::query_as::<_, WarehouseRow>(
            "SELECT id, code, name, address_street, address_city, address_state, 
                    address_postal_code, address_country, status, created_at, updated_at
             FROM warehouses WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("Warehouse", &id.to_string()))?;
        
        Ok(row.into_warehouse()?)
    }

    async fn find_all(&self, pool: &SqlitePool) -> Result<Vec<Warehouse>> {
        let rows = sqlx::query_as::<_, WarehouseRow>(
            "SELECT id, code, name, address_street, address_city, address_state, 
                    address_postal_code, address_country, status, created_at, updated_at
             FROM warehouses WHERE status = 'Active' ORDER BY code"
        )
        .fetch_all(pool)
        .await?;
        
        Ok(rows.into_iter().map(|r| r.into_warehouse()).collect::<Result<Vec<_>>>()?)
    }

    async fn create(&self, pool: &SqlitePool, warehouse: Warehouse) -> Result<Warehouse> {
        let now = Utc::now();
        sqlx::query("INSERT INTO warehouses (id, code, name, address_street, address_city, 
             address_state, address_postal_code, address_country, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(warehouse.base.id.to_string())
        .bind(&warehouse.code)
        .bind(&warehouse.name)
        .bind(&warehouse.address.street)
        .bind(&warehouse.address.city)
        .bind(&warehouse.address.state)
        .bind(&warehouse.address.postal_code)
        .bind(&warehouse.address.country)
        .bind(format!("{:?}", warehouse.status))
        .bind(warehouse.base.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;
        
        Ok(warehouse)
    }

    async fn update(&self, pool: &SqlitePool, warehouse: Warehouse) -> Result<Warehouse> {
        let now = Utc::now();
        let rows = sqlx::query("UPDATE warehouses SET code = ?, name = ?, address_street = ?, address_city = ?, 
             address_state = ?, address_postal_code = ?, address_country = ?, status = ?, updated_at = ?
             WHERE id = ?")
        .bind(&warehouse.code)
        .bind(&warehouse.name)
        .bind(&warehouse.address.street)
        .bind(&warehouse.address.city)
        .bind(&warehouse.address.state)
        .bind(&warehouse.address.postal_code)
        .bind(&warehouse.address.country)
        .bind(format!("{:?}", warehouse.status))
        .bind(now.to_rfc3339())
        .bind(warehouse.base.id.to_string())
        .execute(pool)
        .await?;
        
        if rows.rows_affected() == 0 {
            return Err(Error::not_found("Warehouse", &warehouse.base.id.to_string()));
        }
        
        Ok(warehouse)
    }
}

pub struct SqliteStockMovementRepository;

#[derive(sqlx::FromRow)]
struct StockLevelRow {
    id: String,
    product_id: String,
    location_id: String,
    quantity: i64,
    reserved_quantity: i64,
    available_quantity: i64,
}

impl StockLevelRow {
    fn into_stock_level(self) -> Result<StockLevel> {
        let id = Uuid::parse_str(&self.id)
            .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid UUID '{}': {}", self.id, e)))?;
        let product_id = Uuid::parse_str(&self.product_id)
            .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid UUID '{}': {}", self.product_id, e)))?;
        let location_id = Uuid::parse_str(&self.location_id)
            .map_err(|e| Error::Internal(anyhow::anyhow!("Invalid UUID '{}': {}", self.location_id, e)))?;
        
        Ok(StockLevel {
            id,
            product_id,
            location_id,
            quantity: self.quantity,
            reserved_quantity: self.reserved_quantity,
            available_quantity: self.available_quantity,
        })
    }
}

#[async_trait]
impl StockMovementRepository for SqliteStockMovementRepository {
    async fn record(&self, pool: &SqlitePool, movement: StockMovement) -> Result<StockMovement> {
        let now = Utc::now();
        let mut tx = pool.begin().await?;
        
        sqlx::query("INSERT INTO stock_movements (id, movement_number, movement_type, product_id, 
             from_location_id, to_location_id, quantity, reference, movement_date, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
        .bind(movement.base.id.to_string())
        .bind(&movement.movement_number)
        .bind(format!("{:?}", movement.movement_type))
        .bind(movement.product_id.to_string())
        .bind(movement.from_location_id.map(|id| id.to_string()))
        .bind(movement.to_location_id.to_string())
        .bind(movement.quantity)
        .bind(&movement.reference)
        .bind(movement.date.to_rfc3339())
        .bind(movement.base.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&mut *tx)
        .await?;
        
        let existing = sqlx::query_as::<_, StockLevelRow>(
            "SELECT id, product_id, location_id, quantity, reserved_quantity, available_quantity
             FROM stock_levels WHERE product_id = ? AND location_id = ?"
        )
        .bind(movement.product_id.to_string())
        .bind(movement.to_location_id.to_string())
        .fetch_optional(&mut *tx)
        .await?;
        
        match existing {
            Some(_) => {
                sqlx::query("UPDATE stock_levels SET quantity = quantity + ?, available_quantity = available_quantity + ?, reserved_quantity = reserved_quantity
                     WHERE product_id = ? AND location_id = ?")
                .bind(movement.quantity)
                .bind(movement.quantity)
                .bind(movement.product_id.to_string())
                .bind(movement.to_location_id.to_string())
                .execute(&mut *tx)
                .await?;
            }
            None => {
                sqlx::query("INSERT INTO stock_levels (id, product_id, location_id, quantity, reserved_quantity, available_quantity)
                     VALUES (?, ?, ?, ?, 0, ?)")
                .bind(Uuid::new_v4().to_string())
                .bind(movement.product_id.to_string())
                .bind(movement.to_location_id.to_string())
                .bind(movement.quantity)
                .bind(movement.quantity)
                .execute(&mut *tx)
                .await?;
            }
        }
        
        tx.commit().await?;
        Ok(movement)
    }

    async fn get_stock_level(&self, pool: &SqlitePool, product_id: Uuid, location_id: Uuid) -> Result<StockLevel> {
        let row = sqlx::query_as::<_, StockLevelRow>(
            "SELECT id, product_id, location_id, quantity, reserved_quantity, available_quantity
             FROM stock_levels WHERE product_id = ? AND location_id = ?"
        )
        .bind(product_id.to_string())
        .bind(location_id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("StockLevel", &format!("{}/{}", product_id, location_id)))?;
        
        Ok(row.into_stock_level()?)
    }

    async fn get_product_stock(&self, pool: &SqlitePool, product_id: Uuid) -> Result<Vec<StockLevel>> {
        let rows = sqlx::query_as::<_, StockLevelRow>(
            "SELECT id, product_id, location_id, quantity, reserved_quantity, available_quantity
             FROM stock_levels WHERE product_id = ?"
        )
        .bind(product_id.to_string())
        .fetch_all(pool)
        .await?;
        
        Ok(rows.into_iter().map(|r| r.into_stock_level()).collect::<Result<Vec<_>>>()?)
    }
}

#[async_trait]
pub trait CycleCountRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<CycleCount>;
    async fn find_all(&self, pool: &SqlitePool, warehouse_id: Option<Uuid>) -> Result<Vec<CycleCount>>;
    async fn create(&self, pool: &SqlitePool, count: CycleCount) -> Result<CycleCount>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: CycleCountStatus, completed_at: Option<DateTime<Utc>>) -> Result<()>;
    
    async fn find_lines(&self, pool: &SqlitePool, cycle_count_id: Uuid) -> Result<Vec<CycleCountLine>>;
    async fn find_line_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<CycleCountLine>;
    async fn create_line(&self, pool: &SqlitePool, line: CycleCountLine) -> Result<CycleCountLine>;
    async fn update_line_count(&self, pool: &SqlitePool, id: Uuid, actual_qty: i64, adjustment_qty: i64, notes: Option<String>) -> Result<()>;
    async fn update_line_status(&self, pool: &SqlitePool, id: Uuid, status: CycleCountLineStatus) -> Result<()>;
}

#[async_trait]
pub trait ValuationRepository: Send + Sync {
    async fn get_valuation(&self, pool: &SqlitePool, product_id: Uuid, warehouse_id: Uuid) -> Result<ProductValuation>;
    async fn update_valuation(&self, pool: &SqlitePool, valuation: ProductValuation) -> Result<()>;
    async fn add_cost_layer(&self, pool: &SqlitePool, layer: InventoryCostLayer) -> Result<()>;
    async fn get_cost_layers(&self, pool: &SqlitePool, product_id: Uuid, warehouse_id: Uuid) -> Result<Vec<InventoryCostLayer>>;
    async fn create_cost_adjustment(&self, pool: &SqlitePool, adj: CostAdjustment, lines: Vec<CostAdjustmentLine>) -> Result<()>;
}

pub struct SqliteCycleCountRepository;

#[async_trait]
impl CycleCountRepository for SqliteCycleCountRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<CycleCount> {
        let row = sqlx::query_as::<_, CycleCountRow>(
            "SELECT id, count_number, warehouse_id, name, status, planned_date, completed_at, created_by, created_at
             FROM cycle_counts WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("CycleCount", &id.to_string()))?;
        
        Ok(row.into_cycle_count()?)
    }

    async fn find_all(&self, pool: &SqlitePool, warehouse_id: Option<Uuid>) -> Result<Vec<CycleCount>> {
        let rows = match warehouse_id {
            Some(wid) => sqlx::query_as::<_, CycleCountRow>(
                "SELECT id, count_number, warehouse_id, name, status, planned_date, completed_at, created_by, created_at
                 FROM cycle_counts WHERE warehouse_id = ? ORDER BY created_at DESC"
            )
            .bind(wid.to_string())
            .fetch_all(pool)
            .await?,
            None => sqlx::query_as::<_, CycleCountRow>(
                "SELECT id, count_number, warehouse_id, name, status, planned_date, completed_at, created_by, created_at
                 FROM cycle_counts ORDER BY created_at DESC"
            )
            .fetch_all(pool)
            .await?,
        };
        
        Ok(rows.into_iter().map(|r| r.into_cycle_count()).collect::<Result<Vec<_>>>()?)
    }

    async fn create(&self, pool: &SqlitePool, count: CycleCount) -> Result<CycleCount> {
        sqlx::query(
            "INSERT INTO cycle_counts (id, count_number, warehouse_id, name, status, planned_date, completed_at, created_by, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(count.id.to_string())
        .bind(&count.count_number)
        .bind(count.warehouse_id.to_string())
        .bind(&count.name)
        .bind(format!("{:?}", count.status))
        .bind(count.planned_date.to_rfc3339())
        .bind(count.completed_at.map(|d| d.to_rfc3339()))
        .bind(count.created_by.map(|id| id.to_string()))
        .bind(count.created_at.to_rfc3339())
        .execute(pool)
        .await?;
        
        Ok(count)
    }

    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: CycleCountStatus, completed_at: Option<DateTime<Utc>>) -> Result<()> {
        sqlx::query("UPDATE cycle_counts SET status = ?, completed_at = ? WHERE id = ?")
            .bind(format!("{:?}", status))
            .bind(completed_at.map(|d: DateTime<Utc>| d.to_rfc3339()))
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn find_lines(&self, pool: &SqlitePool, cycle_count_id: Uuid) -> Result<Vec<CycleCountLine>> {
        let rows = sqlx::query_as::<_, CycleCountLineRow>(
            "SELECT id, cycle_count_id, product_id, location_id, expected_quantity, actual_quantity, adjustment_qty, status, notes
             FROM cycle_count_lines WHERE cycle_count_id = ?"
        )
        .bind(cycle_count_id.to_string())
        .fetch_all(pool)
        .await?;
        
        Ok(rows.into_iter().map(|r| r.into_line()).collect::<Result<Vec<_>>>()?)
    }

    async fn find_line_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<CycleCountLine> {
        let row = sqlx::query_as::<_, CycleCountLineRow>(
            "SELECT id, cycle_count_id, product_id, location_id, expected_quantity, actual_quantity, adjustment_qty, status, notes
             FROM cycle_count_lines WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("CycleCountLine", &id.to_string()))?;
        
        Ok(row.into_line()?)
    }

    async fn create_line(&self, pool: &SqlitePool, line: CycleCountLine) -> Result<CycleCountLine> {
        sqlx::query(
            "INSERT INTO cycle_count_lines (id, cycle_count_id, product_id, location_id, expected_quantity, actual_quantity, adjustment_qty, status, notes)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(line.id.to_string())
        .bind(line.cycle_count_id.to_string())
        .bind(line.product_id.to_string())
        .bind(line.location_id.to_string())
        .bind(line.expected_quantity)
        .bind(line.actual_quantity)
        .bind(line.adjustment_qty)
        .bind(format!("{:?}", line.status))
        .bind(&line.notes)
        .execute(pool)
        .await?;
        
        Ok(line)
    }

    async fn update_line_count(&self, pool: &SqlitePool, id: Uuid, actual_qty: i64, adjustment_qty: i64, notes: Option<String>) -> Result<()> {
        sqlx::query("UPDATE cycle_count_lines SET actual_quantity = ?, adjustment_qty = ?, status = 'Counted', notes = ? WHERE id = ?")
            .bind(actual_qty)
            .bind(adjustment_qty)
            .bind(notes)
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn update_line_status(&self, pool: &SqlitePool, id: Uuid, status: CycleCountLineStatus) -> Result<()> {
        sqlx::query("UPDATE cycle_count_lines SET status = ? WHERE id = ?")
            .bind(format!("{:?}", status))
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }
}

impl CycleCountRow {
    fn into_cycle_count(self) -> Result<CycleCount> {
        Ok(CycleCount {
            id: Uuid::parse_str(&self.id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.id, e)))?,
            count_number: self.count_number,
            warehouse_id: Uuid::parse_str(&self.warehouse_id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.warehouse_id, e)))?,
            name: self.name,
            status: match self.status.as_str() {
                "InProgress" => CycleCountStatus::InProgress,
                "Completed" => CycleCountStatus::Completed,
                "Adjusted" => CycleCountStatus::Adjusted,
                "Cancelled" => CycleCountStatus::Cancelled,
                _ => CycleCountStatus::Draft,
            },
            planned_date: chrono::DateTime::parse_from_rfc3339(&self.planned_date).map(|d| d.with_timezone(&Utc)).map_err(|e| Error::internal(format!("Invalid date {}: {}", self.planned_date, e)))?,
            completed_at: self.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            created_by: self.created_by.and_then(|id| Uuid::parse_str(&id).ok()),
            created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at).map(|d| d.with_timezone(&Utc)).map_err(|e| Error::internal(format!("Invalid date {}: {}", self.created_at, e)))?,
        })
    }
}

impl CycleCountLineRow {
    fn into_line(self) -> Result<CycleCountLine> {
        Ok(CycleCountLine {
            id: Uuid::parse_str(&self.id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.id, e)))?,
            cycle_count_id: Uuid::parse_str(&self.cycle_count_id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.cycle_count_id, e)))?,
            product_id: Uuid::parse_str(&self.product_id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.product_id, e)))?,
            location_id: Uuid::parse_str(&self.location_id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.location_id, e)))?,
            expected_quantity: self.expected_quantity,
            actual_quantity: self.actual_quantity,
            adjustment_qty: self.adjustment_qty,
            status: match self.status.as_str() {
                "Counted" => CycleCountLineStatus::Counted,
                "Verified" => CycleCountLineStatus::Verified,
                "Adjusted" => CycleCountLineStatus::Adjusted,
                _ => CycleCountLineStatus::Pending,
            },
            notes: self.notes,
        })
    }
}

#[derive(sqlx::FromRow)]
struct CycleCountRow {
    id: String,
    count_number: String,
    warehouse_id: String,
    name: String,
    status: String,
    planned_date: String,
    completed_at: Option<String>,
    created_by: Option<String>,
    created_at: String,
}

#[derive(sqlx::FromRow)]
struct CycleCountLineRow {
    id: String,
    cycle_count_id: String,
    product_id: String,
    location_id: String,
    expected_quantity: i64,
    actual_quantity: Option<i64>,
    adjustment_qty: Option<i64>,
    status: String,
    notes: Option<String>,
}

pub struct SqliteValuationRepository;

#[async_trait]
impl ValuationRepository for SqliteValuationRepository {
    async fn get_valuation(&self, pool: &SqlitePool, product_id: Uuid, warehouse_id: Uuid) -> Result<ProductValuation> {
        let row = sqlx::query_as::<_, ValuationRow>(
            "SELECT id, product_id, warehouse_id, valuation_method, standard_cost, current_unit_cost, 
                    total_quantity, total_value, last_receipt_cost, last_receipt_date, 
                    last_issue_cost, last_issue_date, created_at, updated_at
             FROM product_valuations WHERE product_id = ? AND warehouse_id = ?"
        )
        .bind(product_id.to_string())
        .bind(warehouse_id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("ProductValuation", &format!("{}/{}", product_id, warehouse_id)))?;
        
        Ok(row.into_valuation()?)
    }

    async fn update_valuation(&self, pool: &SqlitePool, v: ProductValuation) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            "INSERT INTO product_valuations (id, product_id, warehouse_id, valuation_method, standard_cost, 
                    current_unit_cost, total_quantity, total_value, last_receipt_cost, last_receipt_date, 
                    last_issue_cost, last_issue_date, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(product_id, warehouse_id) DO UPDATE SET
                current_unit_cost = excluded.current_unit_cost,
                total_quantity = excluded.total_quantity,
                total_value = excluded.total_value,
                last_receipt_cost = excluded.last_receipt_cost,
                last_receipt_date = excluded.last_receipt_date,
                updated_at = excluded.updated_at"
        )
        .bind(v.id.to_string())
        .bind(v.product_id.to_string())
        .bind(v.warehouse_id.to_string())
        .bind(format!("{:?}", v.valuation_method))
        .bind(v.standard_cost)
        .bind(v.current_unit_cost)
        .bind(v.total_quantity)
        .bind(v.total_value)
        .bind(v.last_receipt_cost)
        .bind(v.last_receipt_date.map(|d| d.to_rfc3339()))
        .bind(v.last_issue_cost)
        .bind(v.last_issue_date.map(|d| d.to_rfc3339()))
        .bind(v.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await?;
        
        Ok(())
    }

    async fn add_cost_layer(&self, pool: &SqlitePool, layer: InventoryCostLayer) -> Result<()> {
        sqlx::query(
            "INSERT INTO inventory_cost_layers (id, product_id, warehouse_id, layer_date, receipt_reference, 
                    receipt_id, quantity, unit_cost, remaining_quantity, total_value, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(layer.id.to_string())
        .bind(layer.product_id.to_string())
        .bind(layer.warehouse_id.to_string())
        .bind(layer.layer_date.to_rfc3339())
        .bind(&layer.receipt_reference)
        .bind(layer.receipt_id.map(|id| id.to_string()))
        .bind(layer.quantity)
        .bind(layer.unit_cost)
        .bind(layer.remaining_quantity)
        .bind(layer.total_value)
        .bind(layer.created_at.to_rfc3339())
        .execute(pool)
        .await?;
        
        Ok(())
    }

    async fn get_cost_layers(&self, pool: &SqlitePool, product_id: Uuid, warehouse_id: Uuid) -> Result<Vec<InventoryCostLayer>> {
        let rows = sqlx::query_as::<_, CostLayerRow>(
            "SELECT id, product_id, warehouse_id, layer_date, receipt_reference, receipt_id, 
                    quantity, unit_cost, remaining_quantity, total_value, created_at
             FROM inventory_cost_layers 
             WHERE product_id = ? AND warehouse_id = ? AND remaining_quantity > 0
             ORDER BY layer_date ASC"
        )
        .bind(product_id.to_string())
        .bind(warehouse_id.to_string())
        .fetch_all(pool)
        .await?;
        
        Ok(rows.into_iter().map(|r| r.into_layer()).collect::<Result<Vec<_>>>()?)
    }

    async fn create_cost_adjustment(&self, pool: &SqlitePool, adj: CostAdjustment, lines: Vec<CostAdjustmentLine>) -> Result<()> {
        let mut tx = pool.begin().await?;
        
        sqlx::query(
            "INSERT INTO cost_adjustments (id, adjustment_number, adjustment_type, adjustment_date, reason, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(adj.id.to_string())
        .bind(&adj.adjustment_number)
        .bind(format!("{:?}", adj.adjustment_type))
        .bind(adj.adjustment_date.to_rfc3339())
        .bind(&adj.reason)
        .bind(format!("{:?}", adj.status))
        .bind(adj.created_at.to_rfc3339())
        .execute(&mut *tx)
        .await?;
        
        for line in lines {
            sqlx::query(
                "INSERT INTO cost_adjustment_lines (id, adjustment_id, product_id, warehouse_id, quantity, 
                        old_unit_cost, new_unit_cost, old_total_value, new_total_value, value_change)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
            )
            .bind(line.id.to_string())
            .bind(adj.id.to_string())
            .bind(line.product_id.to_string())
            .bind(line.warehouse_id.to_string())
            .bind(line.quantity)
            .bind(line.old_unit_cost)
            .bind(line.new_unit_cost)
            .bind(line.old_total_value)
            .bind(line.new_total_value)
            .bind(line.value_change)
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct ValuationRow {
    id: String,
    product_id: String,
    warehouse_id: String,
    valuation_method: String,
    standard_cost: i64,
    current_unit_cost: i64,
    total_quantity: i64,
    total_value: i64,
    last_receipt_cost: i64,
    last_receipt_date: Option<String>,
    last_issue_cost: i64,
    last_issue_date: Option<String>,
    created_at: String,
    updated_at: String,
}

impl ValuationRow {
    fn into_valuation(self) -> Result<ProductValuation> {
        Ok(ProductValuation {
            id: Uuid::parse_str(&self.id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.id, e)))?,
            product_id: Uuid::parse_str(&self.product_id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.product_id, e)))?,
            warehouse_id: Uuid::parse_str(&self.warehouse_id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.warehouse_id, e)))?,
            valuation_method: match self.valuation_method.as_str() {
                "FIFO" => ValuationMethod::FIFO,
                "LIFO" => ValuationMethod::LIFO,
                "WeightedAverage" => ValuationMethod::WeightedAverage,
                "StandardCost" => ValuationMethod::StandardCost,
                _ => ValuationMethod::MovingAverage,
            },
            standard_cost: self.standard_cost,
            current_unit_cost: self.current_unit_cost,
            total_quantity: self.total_quantity,
            total_value: self.total_value,
            last_receipt_cost: self.last_receipt_cost,
            last_receipt_date: self.last_receipt_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            last_issue_cost: self.last_issue_cost,
            last_issue_date: self.last_issue_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at).map(|d| d.with_timezone(&Utc)).map_err(|e| Error::internal(format!("Invalid date {}: {}", self.created_at, e)))?,
            updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at).map(|d| d.with_timezone(&Utc)).map_err(|e| Error::internal(format!("Invalid date {}: {}", self.updated_at, e)))?,
        })
    }
}

#[derive(sqlx::FromRow)]
struct CostLayerRow {
    id: String,
    product_id: String,
    warehouse_id: String,
    layer_date: String,
    receipt_reference: String,
    receipt_id: Option<String>,
    quantity: i64,
    unit_cost: i64,
    remaining_quantity: i64,
    total_value: i64,
    created_at: String,
}

impl CostLayerRow {
    fn into_layer(self) -> Result<InventoryCostLayer> {
        Ok(InventoryCostLayer {
            id: Uuid::parse_str(&self.id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.id, e)))?,
            product_id: Uuid::parse_str(&self.product_id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.product_id, e)))?,
            warehouse_id: Uuid::parse_str(&self.warehouse_id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.warehouse_id, e)))?,
            layer_date: chrono::DateTime::parse_from_rfc3339(&self.layer_date).map(|d| d.with_timezone(&Utc)).map_err(|e| Error::internal(format!("Invalid date {}: {}", self.layer_date, e)))?,
            receipt_reference: self.receipt_reference,
            receipt_id: self.receipt_id.and_then(|id| Uuid::parse_str(&id).ok()),
            quantity: self.quantity,
            unit_cost: self.unit_cost,
            remaining_quantity: self.remaining_quantity,
            total_value: self.total_value,
            created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at).map(|d| d.with_timezone(&Utc)).map_err(|e| Error::internal(format!("Invalid date {}: {}", self.created_at, e)))?,
        })
    }
}

#[async_trait]
pub trait StockTransferRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<StockTransfer>;
    async fn find_all(&self, pool: &SqlitePool) -> Result<Vec<StockTransfer>>;
    async fn create(&self, pool: &SqlitePool, transfer: StockTransfer) -> Result<StockTransfer>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: StockTransferStatus) -> Result<()>;
    async fn find_lines(&self, pool: &SqlitePool, transfer_id: Uuid) -> Result<Vec<StockTransferLine>>;
    async fn create_line(&self, pool: &SqlitePool, line: StockTransferLine) -> Result<StockTransferLine>;
}

pub struct SqliteStockTransferRepository;

#[async_trait]
impl StockTransferRepository for SqliteStockTransferRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<StockTransfer> {
        let row = sqlx::query_as::<_, StockTransferRow>(
            "SELECT id, transfer_number, from_warehouse_id, to_warehouse_id, shipment_date, expected_arrival, status, carrier, tracking_number, notes, created_at, updated_at, created_by, updated_by
             FROM stock_transfers WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await?
        .ok_or_else(|| Error::not_found("StockTransfer", &id.to_string()))?;
        
        Ok(row.into_transfer()?)
    }

    async fn find_all(&self, pool: &SqlitePool) -> Result<Vec<StockTransfer>> {
        let rows = sqlx::query_as::<_, StockTransferRow>(
            "SELECT id, transfer_number, from_warehouse_id, to_warehouse_id, shipment_date, expected_arrival, status, carrier, tracking_number, notes, created_at, updated_at, created_by, updated_by
             FROM stock_transfers ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await?;
        
        Ok(rows.into_iter().map(|r| r.into_transfer()).collect::<Result<Vec<_>>>()?)
    }

    async fn create(&self, pool: &SqlitePool, transfer: StockTransfer) -> Result<StockTransfer> {
        sqlx::query(
            "INSERT INTO stock_transfers (id, transfer_number, from_warehouse_id, to_warehouse_id, shipment_date, expected_arrival, status, carrier, tracking_number, notes, created_at, updated_at, created_by, updated_by)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(transfer.base.id.to_string())
        .bind(&transfer.transfer_number)
        .bind(transfer.from_warehouse_id.to_string())
        .bind(transfer.to_warehouse_id.to_string())
        .bind(transfer.shipment_date.map(|d| d.to_rfc3339()))
        .bind(transfer.expected_arrival.map(|d| d.to_rfc3339()))
        .bind(format!("{:?}", transfer.status))
        .bind(&transfer.carrier)
        .bind(&transfer.tracking_number)
        .bind(&transfer.notes)
        .bind(transfer.base.created_at.to_rfc3339())
        .bind(transfer.base.updated_at.to_rfc3339())
        .bind(transfer.base.created_by.map(|id| id.to_string()))
        .bind(transfer.base.updated_by.map(|id| id.to_string()))
        .execute(pool)
        .await?;
        
        Ok(transfer)
    }

    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: StockTransferStatus) -> Result<()> {
        sqlx::query("UPDATE stock_transfers SET status = ?, updated_at = ? WHERE id = ?")
            .bind(format!("{:?}", status))
            .bind(Utc::now().to_rfc3339())
            .bind(id.to_string())
            .execute(pool)
            .await?;
        Ok(())
    }

    async fn find_lines(&self, pool: &SqlitePool, transfer_id: Uuid) -> Result<Vec<StockTransferLine>> {
        let rows = sqlx::query_as::<_, StockTransferLineRow>(
            "SELECT id, transfer_id, product_id, quantity, quantity_received, lot_id, status
             FROM stock_transfer_lines WHERE transfer_id = ?"
        )
        .bind(transfer_id.to_string())
        .fetch_all(pool)
        .await?;
        
        Ok(rows.into_iter().map(|r| r.into_line()).collect::<Result<Vec<_>>>()?)
    }

    async fn create_line(&self, pool: &SqlitePool, line: StockTransferLine) -> Result<StockTransferLine> {
        sqlx::query(
            "INSERT INTO stock_transfer_lines (id, transfer_id, product_id, quantity, quantity_received, lot_id, status)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(line.id.to_string())
        .bind(line.transfer_id.to_string())
        .bind(line.product_id.to_string())
        .bind(line.quantity)
        .bind(line.quantity_received)
        .bind(line.lot_id.map(|id| id.to_string()))
        .bind(format!("{:?}", line.status))
        .execute(pool)
        .await?;
        
        Ok(line)
    }
}

#[derive(sqlx::FromRow)]
struct StockTransferRow {
    id: String,
    transfer_number: String,
    from_warehouse_id: String,
    to_warehouse_id: String,
    shipment_date: Option<String>,
    expected_arrival: Option<String>,
    status: String,
    carrier: Option<String>,
    tracking_number: Option<String>,
    notes: Option<String>,
    created_at: String,
    updated_at: String,
    created_by: Option<String>,
    updated_by: Option<String>,
}

impl StockTransferRow {
    fn into_transfer(self) -> Result<StockTransfer> {
        let id = Uuid::parse_str(&self.id)
            .map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.id, e)))?;
        
        Ok(StockTransfer {
            base: BaseEntity {
                id,
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
                    .map(|d| d.with_timezone(&Utc))
                    .map_err(|e| Error::internal(format!("Invalid date {}: {}", self.created_at, e)))?,
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at)
                    .map(|d| d.with_timezone(&Utc))
                    .map_err(|e| Error::internal(format!("Invalid date {}: {}", self.updated_at, e)))?,
                created_by: self.created_by.and_then(|s| Uuid::parse_str(&s).ok()),
                updated_by: self.updated_by.and_then(|s| Uuid::parse_str(&s).ok()),
            },
            transfer_number: self.transfer_number,
            from_warehouse_id: Uuid::parse_str(&self.from_warehouse_id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.from_warehouse_id, e)))?,
            to_warehouse_id: Uuid::parse_str(&self.to_warehouse_id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.to_warehouse_id, e)))?,
            shipment_date: self.shipment_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            expected_arrival: self.expected_arrival.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)),
            status: match self.status.as_str() {
                "Pending" => StockTransferStatus::Pending,
                "InTransit" => StockTransferStatus::InTransit,
                "PartiallyReceived" => StockTransferStatus::PartiallyReceived,
                "Received" => StockTransferStatus::Received,
                "Cancelled" => StockTransferStatus::Cancelled,
                _ => StockTransferStatus::Draft,
            },
            carrier: self.carrier,
            tracking_number: self.tracking_number,
            notes: self.notes,
        })
    }
}

#[derive(sqlx::FromRow)]
struct StockTransferLineRow {
    id: String,
    transfer_id: String,
    product_id: String,
    quantity: i64,
    quantity_received: i64,
    lot_id: Option<String>,
    status: String,
}

impl StockTransferLineRow {
    fn into_line(self) -> Result<StockTransferLine> {
        Ok(StockTransferLine {
            id: Uuid::parse_str(&self.id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.id, e)))?,
            transfer_id: Uuid::parse_str(&self.transfer_id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.transfer_id, e)))?,
            product_id: Uuid::parse_str(&self.product_id).map_err(|e| Error::internal(format!("Invalid UUID {}: {}", self.product_id, e)))?,
            quantity: self.quantity,
            quantity_received: self.quantity_received,
            lot_id: self.lot_id.and_then(|s| Uuid::parse_str(&s).ok()),
            status: match self.status.as_str() {
                "Pending" => StockTransferStatus::Pending,
                "InTransit" => StockTransferStatus::InTransit,
                "PartiallyReceived" => StockTransferStatus::PartiallyReceived,
                "Received" => StockTransferStatus::Received,
                "Cancelled" => StockTransferStatus::Cancelled,
                _ => StockTransferStatus::Draft,
            },
        })
    }
}
