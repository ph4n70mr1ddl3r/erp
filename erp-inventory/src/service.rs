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
        .map_err(|e| Error::Database(e))?;
        
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
        .map_err(|e| Error::Database(e))?
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
        .map_err(|e| Error::Database(e))?;
        
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
        .map_err(|e| Error::Database(e))?;
        
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
        .map_err(|e| Error::Database(e))?;
        
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

pub struct QualityInspectionService;

impl QualityInspectionService {
    pub fn new() -> Self { Self }

    pub async fn create_inspection(
        pool: &SqlitePool,
        inspection_type: InspectionType,
        entity_type: &str,
        entity_id: Uuid,
        inspector_id: Option<Uuid>,
    ) -> Result<QualityInspection> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        let inspection_number = format!("QI-{}", now.format("%Y%m%d%H%M%S"));
        
        let inspection = QualityInspection {
            id,
            inspection_number: inspection_number.clone(),
            inspection_type: inspection_type.clone(),
            entity_type: entity_type.to_string(),
            entity_id,
            inspector_id,
            inspection_date: now,
            status: InspectionStatus::Pending,
            result: None,
            notes: None,
            items: vec![],
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO quality_inspections (id, inspection_number, inspection_type, entity_type, entity_id, inspector_id, inspection_date, status, result, notes, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, 'Pending', NULL, NULL, ?)"
        )
        .bind(inspection.id.to_string())
        .bind(&inspection.inspection_number)
        .bind(format!("{:?}", inspection.inspection_type))
        .bind(&inspection.entity_type)
        .bind(inspection.entity_id.to_string())
        .bind(inspection.inspector_id.map(|id| id.to_string()))
        .bind(inspection.inspection_date.to_rfc3339())
        .bind(inspection.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(inspection)
    }

    pub async fn get_inspection(pool: &SqlitePool, id: Uuid) -> Result<QualityInspection> {
        let row = sqlx::query_as::<_, InspectionRow>(
            "SELECT id, inspection_number, inspection_type, entity_type, entity_id, inspector_id, inspection_date, status, result, notes, created_at
             FROM quality_inspections WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("QualityInspection", &id.to_string()))?;
        
        let items = Self::get_inspection_items(pool, id).await?;
        Ok(row.into_inspection(items))
    }

    async fn get_inspection_items(pool: &SqlitePool, inspection_id: Uuid) -> Result<Vec<InspectionItem>> {
        let rows = sqlx::query_as::<_, InspectionItemRow>(
            "SELECT id, inspection_id, criterion, expected_value, actual_value, pass_fail, notes
             FROM inspection_items WHERE inspection_id = ?"
        )
        .bind(inspection_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn add_inspection_item(
        pool: &SqlitePool,
        inspection_id: Uuid,
        criterion: &str,
        expected_value: Option<&str>,
    ) -> Result<InspectionItem> {
        let now = chrono::Utc::now();
        let item = InspectionItem {
            id: Uuid::new_v4(),
            inspection_id,
            criterion: criterion.to_string(),
            expected_value: expected_value.map(|s| s.to_string()),
            actual_value: None,
            pass_fail: None,
            notes: None,
        };
        
        sqlx::query(
            "INSERT INTO inspection_items (id, inspection_id, criterion, expected_value, actual_value, pass_fail, notes)
             VALUES (?, ?, ?, ?, NULL, NULL, NULL)"
        )
        .bind(item.id.to_string())
        .bind(item.inspection_id.to_string())
        .bind(&item.criterion)
        .bind(&item.expected_value)
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(item)
    }

    pub async fn record_result(
        pool: &SqlitePool,
        item_id: Uuid,
        actual_value: &str,
        pass_fail: PassFail,
        notes: Option<&str>,
    ) -> Result<InspectionItem> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE inspection_items SET actual_value = ?, pass_fail = ?, notes = ? WHERE id = ?"
        )
        .bind(actual_value)
        .bind(format!("{:?}", pass_fail))
        .bind(notes)
        .bind(item_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let row = sqlx::query_as::<_, InspectionItemRow>(
            "SELECT id, inspection_id, criterion, expected_value, actual_value, pass_fail, notes
             FROM inspection_items WHERE id = ?"
        )
        .bind(item_id.to_string())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(row.into())
    }

    pub async fn complete_inspection(
        pool: &SqlitePool,
        id: Uuid,
        result: InspectionResult,
        notes: Option<&str>,
    ) -> Result<QualityInspection> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE quality_inspections SET status = 'Completed', result = ?, notes = ? WHERE id = ?"
        )
        .bind(format!("{:?}", result))
        .bind(notes)
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get_inspection(pool, id).await
    }

    pub async fn list_inspections(pool: &SqlitePool, entity_type: Option<&str>, entity_id: Option<Uuid>) -> Result<Vec<QualityInspection>> {
        let rows = match (entity_type, entity_id) {
            (Some(et), Some(eid)) => {
                sqlx::query_as::<_, InspectionRow>(
                    "SELECT id, inspection_number, inspection_type, entity_type, entity_id, inspector_id, inspection_date, status, result, notes, created_at
                     FROM quality_inspections WHERE entity_type = ? AND entity_id = ? ORDER BY created_at DESC"
                )
                .bind(et)
                .bind(eid.to_string())
                .fetch_all(pool)
                .await
            }
            _ => {
                sqlx::query_as::<_, InspectionRow>(
                    "SELECT id, inspection_number, inspection_type, entity_type, entity_id, inspector_id, inspection_date, status, result, notes, created_at
                     FROM quality_inspections ORDER BY created_at DESC"
                )
                .fetch_all(pool)
                .await
            }
        }.map_err(|e| Error::Database(e))?;
        
        let mut inspections = Vec::new();
        for row in rows {
            let items = Self::get_inspection_items(pool, row.id.parse().unwrap_or_default()).await?;
            inspections.push(row.into_inspection(items));
        }
        Ok(inspections)
    }
}

#[derive(sqlx::FromRow)]
struct InspectionRow {
    id: String,
    inspection_number: String,
    inspection_type: String,
    entity_type: String,
    entity_id: String,
    inspector_id: Option<String>,
    inspection_date: String,
    status: String,
    result: Option<String>,
    notes: Option<String>,
    created_at: String,
}

impl InspectionRow {
    fn into_inspection(self, items: Vec<InspectionItem>) -> QualityInspection {
        QualityInspection {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            inspection_number: self.inspection_number,
            inspection_type: match self.inspection_type.as_str() {
                "Incoming" => InspectionType::Incoming,
                "InProcess" => InspectionType::InProcess,
                "Final" => InspectionType::Final,
                "Outgoing" => InspectionType::Outgoing,
                _ => InspectionType::Incoming,
            },
            entity_type: self.entity_type,
            entity_id: Uuid::parse_str(&self.entity_id).unwrap_or_default(),
            inspector_id: self.inspector_id.and_then(|id| Uuid::parse_str(&id).ok()),
            inspection_date: chrono::DateTime::parse_from_rfc3339(&self.inspection_date)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            status: match self.status.as_str() {
                "InProgress" => InspectionStatus::InProgress,
                "Completed" => InspectionStatus::Completed,
                "Cancelled" => InspectionStatus::Cancelled,
                _ => InspectionStatus::Pending,
            },
            result: self.result.and_then(|r| match r.as_str() {
                "Pass" => Some(InspectionResult::Pass),
                "Fail" => Some(InspectionResult::Fail),
                "Conditional" => Some(InspectionResult::Conditional),
                _ => None,
            }),
            notes: self.notes,
            items,
            created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct InspectionItemRow {
    id: String,
    inspection_id: String,
    criterion: String,
    expected_value: Option<String>,
    actual_value: Option<String>,
    pass_fail: Option<String>,
    notes: Option<String>,
}

impl From<InspectionItemRow> for InspectionItem {
    fn from(r: InspectionItemRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            inspection_id: Uuid::parse_str(&r.inspection_id).unwrap_or_default(),
            criterion: r.criterion,
            expected_value: r.expected_value,
            actual_value: r.actual_value,
            pass_fail: r.pass_fail.and_then(|pf| match pf.as_str() {
                "Pass" => Some(PassFail::Pass),
                "Fail" => Some(PassFail::Fail),
                "NotApplicable" => Some(PassFail::NotApplicable),
                _ => None,
            }),
            notes: r.notes,
        }
    }
}

pub struct NonConformanceService;

impl NonConformanceService {
    pub fn new() -> Self { Self }

    pub async fn create_ncr(
        pool: &SqlitePool,
        source_type: &str,
        source_id: Uuid,
        description: &str,
        severity: NCRSeverity,
    ) -> Result<NonConformanceReport> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        let ncr_number = format!("NCR-{}", now.format("%Y%m%d%H%M%S"));
        
        let ncr = NonConformanceReport {
            id,
            ncr_number: ncr_number.clone(),
            source_type: source_type.to_string(),
            source_id,
            description: description.to_string(),
            severity: severity.clone(),
            status: NCRStatus::Open,
            assigned_to: None,
            root_cause: None,
            corrective_action: None,
            preventive_action: None,
            resolution_date: None,
            created_at: now,
            updated_at: now,
        };
        
        sqlx::query(
            "INSERT INTO non_conformance_reports (id, ncr_number, source_type, source_id, description, severity, status, assigned_to, root_cause, corrective_action, preventive_action, resolution_date, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, 'Open', NULL, NULL, NULL, NULL, NULL, ?, ?)"
        )
        .bind(ncr.id.to_string())
        .bind(&ncr.ncr_number)
        .bind(&ncr.source_type)
        .bind(ncr.source_id.to_string())
        .bind(&ncr.description)
        .bind(format!("{:?}", ncr.severity))
        .bind(ncr.created_at.to_rfc3339())
        .bind(ncr.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(ncr)
    }

    pub async fn get_ncr(pool: &SqlitePool, id: Uuid) -> Result<NonConformanceReport> {
        let row = sqlx::query_as::<_, NCRRow>(
            "SELECT id, ncr_number, source_type, source_id, description, severity, status, assigned_to, root_cause, corrective_action, preventive_action, resolution_date, created_at, updated_at
             FROM non_conformance_reports WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("NonConformanceReport", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn resolve_ncr(
        pool: &SqlitePool,
        id: Uuid,
        root_cause: &str,
        corrective_action: &str,
        preventive_action: Option<&str>,
    ) -> Result<NonConformanceReport> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE non_conformance_reports SET status = 'Resolved', root_cause = ?, corrective_action = ?, preventive_action = ?, resolution_date = ?, updated_at = ? WHERE id = ?"
        )
        .bind(root_cause)
        .bind(corrective_action)
        .bind(preventive_action)
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get_ncr(pool, id).await
    }

    pub async fn list_ncrs(pool: &SqlitePool, status: Option<NCRStatus>) -> Result<Vec<NonConformanceReport>> {
        let rows = match status {
            Some(s) => {
                sqlx::query_as::<_, NCRRow>(
                    "SELECT id, ncr_number, source_type, source_id, description, severity, status, assigned_to, root_cause, corrective_action, preventive_action, resolution_date, created_at, updated_at
                     FROM non_conformance_reports WHERE status = ? ORDER BY created_at DESC"
                )
                .bind(format!("{:?}", s))
                .fetch_all(pool)
                .await
            }
            None => {
                sqlx::query_as::<_, NCRRow>(
                    "SELECT id, ncr_number, source_type, source_id, description, severity, status, assigned_to, root_cause, corrective_action, preventive_action, resolution_date, created_at, updated_at
                     FROM non_conformance_reports ORDER BY created_at DESC"
                )
                .fetch_all(pool)
                .await
            }
        }.map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct NCRRow {
    id: String,
    ncr_number: String,
    source_type: String,
    source_id: String,
    description: String,
    severity: String,
    status: String,
    assigned_to: Option<String>,
    root_cause: Option<String>,
    corrective_action: Option<String>,
    preventive_action: Option<String>,
    resolution_date: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<NCRRow> for NonConformanceReport {
    fn from(r: NCRRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            ncr_number: r.ncr_number,
            source_type: r.source_type,
            source_id: Uuid::parse_str(&r.source_id).unwrap_or_default(),
            description: r.description,
            severity: match r.severity.as_str() {
                "Major" => NCRSeverity::Major,
                "Critical" => NCRSeverity::Critical,
                _ => NCRSeverity::Minor,
            },
            status: match r.status.as_str() {
                "InProgress" => NCRStatus::InProgress,
                "Resolved" => NCRStatus::Resolved,
                "Closed" => NCRStatus::Closed,
                _ => NCRStatus::Open,
            },
            assigned_to: r.assigned_to.and_then(|id| Uuid::parse_str(&id).ok()),
            root_cause: r.root_cause,
            corrective_action: r.corrective_action,
            preventive_action: r.preventive_action,
            resolution_date: r.resolution_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

pub struct DemandForecastService;

impl DemandForecastService {
    pub fn new() -> Self { Self }

    pub async fn create_forecast(
        pool: &SqlitePool,
        product_id: Uuid,
        warehouse_id: Option<Uuid>,
        period_start: &str,
        period_end: &str,
        forecast_quantity: i64,
        confidence_level: i32,
        method: ForecastMethod,
    ) -> Result<DemandForecast> {
        let now = chrono::Utc::now();
        let forecast = DemandForecast {
            id: Uuid::new_v4(),
            product_id,
            warehouse_id,
            period_start: chrono::DateTime::parse_from_rfc3339(period_start)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or(now),
            period_end: chrono::DateTime::parse_from_rfc3339(period_end)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or(now),
            forecast_quantity,
            confidence_level,
            method: method.clone(),
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO demand_forecasts (id, product_id, warehouse_id, period_start, period_end, forecast_quantity, confidence_level, method, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(forecast.id.to_string())
        .bind(forecast.product_id.to_string())
        .bind(forecast.warehouse_id.map(|id| id.to_string()))
        .bind(forecast.period_start.to_rfc3339())
        .bind(forecast.period_end.to_rfc3339())
        .bind(forecast.forecast_quantity)
        .bind(forecast.confidence_level)
        .bind(format!("{:?}", forecast.method))
        .bind(forecast.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(forecast)
    }

    pub async fn get_forecasts_for_product(pool: &SqlitePool, product_id: Uuid) -> Result<Vec<DemandForecast>> {
        let rows = sqlx::query_as::<_, ForecastRow>(
            "SELECT id, product_id, warehouse_id, period_start, period_end, forecast_quantity, confidence_level, method, created_at
             FROM demand_forecasts WHERE product_id = ? ORDER BY period_start"
        )
        .bind(product_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct ForecastRow {
    id: String,
    product_id: String,
    warehouse_id: Option<String>,
    period_start: String,
    period_end: String,
    forecast_quantity: i64,
    confidence_level: i64,
    method: String,
    created_at: String,
}

impl From<ForecastRow> for DemandForecast {
    fn from(r: ForecastRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            product_id: Uuid::parse_str(&r.product_id).unwrap_or_default(),
            warehouse_id: r.warehouse_id.and_then(|id| Uuid::parse_str(&id).ok()),
            period_start: chrono::DateTime::parse_from_rfc3339(&r.period_start)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            period_end: chrono::DateTime::parse_from_rfc3339(&r.period_end)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            forecast_quantity: r.forecast_quantity,
            confidence_level: r.confidence_level as i32,
            method: match r.method.as_str() {
                "WeightedAverage" => ForecastMethod::WeightedAverage,
                "ExponentialSmoothing" => ForecastMethod::ExponentialSmoothing,
                "Seasonal" => ForecastMethod::Seasonal,
                "Manual" => ForecastMethod::Manual,
                _ => ForecastMethod::MovingAverage,
            },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

pub struct SafetyStockService;

impl SafetyStockService {
    pub fn new() -> Self { Self }

    pub async fn create_or_update(
        pool: &SqlitePool,
        product_id: Uuid,
        warehouse_id: Uuid,
        safety_stock: i64,
        reorder_point: i64,
        reorder_quantity: i64,
        lead_time_days: i32,
        service_level: i32,
    ) -> Result<SafetyStock> {
        let now = chrono::Utc::now();
        let id = Uuid::new_v4();
        
        sqlx::query(
            "INSERT INTO safety_stock (id, product_id, warehouse_id, safety_stock, reorder_point, reorder_quantity, lead_time_days, service_level, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
             ON CONFLICT(product_id, warehouse_id) DO UPDATE SET
             safety_stock = excluded.safety_stock,
             reorder_point = excluded.reorder_point,
             reorder_quantity = excluded.reorder_quantity,
             lead_time_days = excluded.lead_time_days,
             service_level = excluded.service_level,
             updated_at = excluded.updated_at"
        )
        .bind(id.to_string())
        .bind(product_id.to_string())
        .bind(warehouse_id.to_string())
        .bind(safety_stock)
        .bind(reorder_point)
        .bind(reorder_quantity)
        .bind(lead_time_days)
        .bind(service_level)
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(SafetyStock {
            id,
            product_id,
            warehouse_id,
            safety_stock,
            reorder_point,
            reorder_quantity,
            lead_time_days,
            service_level,
            updated_at: now,
        })
    }

    pub async fn get_for_product(pool: &SqlitePool, product_id: Uuid) -> Result<Vec<SafetyStock>> {
        let rows = sqlx::query_as::<_, SafetyStockRow>(
            "SELECT id, product_id, warehouse_id, safety_stock, reorder_point, reorder_quantity, lead_time_days, service_level, updated_at
             FROM safety_stock WHERE product_id = ?"
        )
        .bind(product_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct SafetyStockRow {
    id: String,
    product_id: String,
    warehouse_id: String,
    safety_stock: i64,
    reorder_point: i64,
    reorder_quantity: i64,
    lead_time_days: i64,
    service_level: i64,
    updated_at: String,
}

impl From<SafetyStockRow> for SafetyStock {
    fn from(r: SafetyStockRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            product_id: Uuid::parse_str(&r.product_id).unwrap_or_default(),
            warehouse_id: Uuid::parse_str(&r.warehouse_id).unwrap_or_default(),
            safety_stock: r.safety_stock,
            reorder_point: r.reorder_point,
            reorder_quantity: r.reorder_quantity,
            lead_time_days: r.lead_time_days as i32,
            service_level: r.service_level as i32,
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

pub struct ReplenishmentOrderService;

impl ReplenishmentOrderService {
    pub fn new() -> Self { Self }

    pub async fn create_order(
        pool: &SqlitePool,
        product_id: Uuid,
        warehouse_id: Uuid,
        order_type: ReplenishmentType,
        quantity: i64,
        source: Option<&str>,
    ) -> Result<ReplenishmentOrder> {
        let now = chrono::Utc::now();
        let order_number = format!("RO-{}", now.format("%Y%m%d%H%M%S"));
        let order = ReplenishmentOrder {
            id: Uuid::new_v4(),
            order_number: order_number.clone(),
            product_id,
            warehouse_id,
            order_type: order_type.clone(),
            quantity,
            status: ReplenishmentStatus::Draft,
            source: source.map(|s| s.to_string()),
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO replenishment_orders (id, order_number, product_id, warehouse_id, order_type, quantity, status, source, created_at)
             VALUES (?, ?, ?, ?, ?, ?, 'Draft', ?, ?)"
        )
        .bind(order.id.to_string())
        .bind(&order.order_number)
        .bind(order.product_id.to_string())
        .bind(order.warehouse_id.to_string())
        .bind(format!("{:?}", order.order_type))
        .bind(order.quantity)
        .bind(&order.source)
        .bind(order.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(order)
    }

    pub async fn list_orders(pool: &SqlitePool) -> Result<Vec<ReplenishmentOrder>> {
        let rows = sqlx::query_as::<_, ReplenishmentOrderRow>(
            "SELECT id, order_number, product_id, warehouse_id, order_type, quantity, status, source, created_at
             FROM replenishment_orders ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct ReplenishmentOrderRow {
    id: String,
    order_number: String,
    product_id: String,
    warehouse_id: String,
    order_type: String,
    quantity: i64,
    status: String,
    source: Option<String>,
    created_at: String,
}

impl From<ReplenishmentOrderRow> for ReplenishmentOrder {
    fn from(r: ReplenishmentOrderRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            order_number: r.order_number,
            product_id: Uuid::parse_str(&r.product_id).unwrap_or_default(),
            warehouse_id: Uuid::parse_str(&r.warehouse_id).unwrap_or_default(),
            order_type: match r.order_type.as_str() {
                "Transfer" => ReplenishmentType::Transfer,
                "Production" => ReplenishmentType::Production,
                _ => ReplenishmentType::Purchase,
            },
            quantity: r.quantity,
            status: match r.status.as_str() {
                "Submitted" => ReplenishmentStatus::Submitted,
                "Completed" => ReplenishmentStatus::Completed,
                "Cancelled" => ReplenishmentStatus::Cancelled,
                _ => ReplenishmentStatus::Draft,
            },
            source: r.source,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}
