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

pub struct WMSService;

impl WMSService {
    pub fn new() -> Self { Self }

    pub async fn create_zone(
        pool: &SqlitePool,
        warehouse_id: Uuid,
        zone_code: &str,
        name: &str,
        zone_type: ZoneType,
        temperature_controlled: bool,
        max_capacity: Option<i64>,
    ) -> Result<WarehouseZone> {
        let now = chrono::Utc::now();
        let zone = WarehouseZone {
            id: Uuid::new_v4(),
            warehouse_id,
            zone_code: zone_code.to_string(),
            name: name.to_string(),
            zone_type: zone_type.clone(),
            temperature_controlled,
            max_capacity,
            current_utilization: 0,
            status: Status::Active,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO warehouse_zones (id, warehouse_id, zone_code, name, zone_type, temperature_controlled, max_capacity, current_utilization, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(zone.id.to_string())
        .bind(zone.warehouse_id.to_string())
        .bind(&zone.zone_code)
        .bind(&zone.name)
        .bind(format!("{:?}", zone.zone_type))
        .bind(zone.temperature_controlled)
        .bind(zone.max_capacity)
        .bind(zone.current_utilization)
        .bind("Active")
        .bind(zone.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(zone)
    }

    pub async fn create_bin(
        pool: &SqlitePool,
        zone_id: Uuid,
        bin_code: &str,
        bin_type: BinType,
        aisle: Option<&str>,
        row_number: Option<i32>,
        level_number: Option<i32>,
        capacity: Option<i64>,
    ) -> Result<WarehouseBin> {
        let bin = WarehouseBin {
            id: Uuid::new_v4(),
            zone_id,
            bin_code: bin_code.to_string(),
            bin_type: bin_type.clone(),
            aisle: aisle.map(|s| s.to_string()),
            row_number,
            level_number,
            capacity,
            current_quantity: 0,
            status: Status::Active,
        };
        
        sqlx::query(
            "INSERT INTO warehouse_bins (id, zone_id, bin_code, bin_type, aisle, row_number, level_number, capacity, current_quantity, status)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(bin.id.to_string())
        .bind(bin.zone_id.to_string())
        .bind(&bin.bin_code)
        .bind(format!("{:?}", bin.bin_type))
        .bind(&bin.aisle)
        .bind(bin.row_number)
        .bind(bin.level_number)
        .bind(bin.capacity)
        .bind(bin.current_quantity)
        .bind("Active")
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(bin)
    }

    pub async fn create_pick_list(
        pool: &SqlitePool,
        warehouse_id: Uuid,
        order_id: Option<Uuid>,
        priority: i32,
    ) -> Result<PickList> {
        let now = chrono::Utc::now();
        let pick_number = format!("PL-{}", now.format("%Y%m%d%H%M%S"));
        let pick_list = PickList {
            id: Uuid::new_v4(),
            pick_number: pick_number.clone(),
            warehouse_id,
            order_id,
            assigned_to: None,
            priority,
            status: PickListStatus::Pending,
            total_items: 0,
            picked_items: 0,
            created_at: now,
            completed_at: None,
        };
        
        sqlx::query(
            "INSERT INTO pick_lists (id, pick_number, warehouse_id, order_id, assigned_to, priority, status, total_items, picked_items, created_at, completed_at)
             VALUES (?, ?, ?, ?, NULL, ?, 'Pending', 0, 0, ?, NULL)"
        )
        .bind(pick_list.id.to_string())
        .bind(&pick_list.pick_number)
        .bind(pick_list.warehouse_id.to_string())
        .bind(pick_list.order_id.map(|id| id.to_string()))
        .bind(pick_list.priority)
        .bind(pick_list.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(pick_list)
    }

    pub async fn get_pick_list(pool: &SqlitePool, id: Uuid) -> Result<PickList> {
        let row = sqlx::query_as::<_, PickListRow>(
            "SELECT id, pick_number, warehouse_id, order_id, assigned_to, priority, status, total_items, picked_items, created_at, completed_at
             FROM pick_lists WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("PickList", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn pick_item(
        pool: &SqlitePool,
        pick_list_id: Uuid,
        product_id: Uuid,
        bin_id: Uuid,
        lot_id: Option<Uuid>,
        requested_qty: i64,
        picked_qty: i64,
    ) -> Result<PickListItem> {
        let item = PickListItem {
            id: Uuid::new_v4(),
            pick_list_id,
            product_id,
            bin_id,
            lot_id,
            requested_qty,
            picked_qty,
            status: if picked_qty >= requested_qty { PickItemStatus::Picked } else { PickItemStatus::Short },
        };
        
        sqlx::query(
            "INSERT INTO pick_list_items (id, pick_list_id, product_id, bin_id, lot_id, requested_qty, picked_qty, status)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(item.id.to_string())
        .bind(item.pick_list_id.to_string())
        .bind(item.product_id.to_string())
        .bind(item.bin_id.to_string())
        .bind(item.lot_id.map(|id| id.to_string()))
        .bind(item.requested_qty)
        .bind(item.picked_qty)
        .bind(format!("{:?}", item.status))
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        sqlx::query(
            "UPDATE pick_lists SET total_items = total_items + 1, picked_items = picked_items + 1 WHERE id = ?"
        )
        .bind(pick_list_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(item)
    }

    pub async fn complete_pick(pool: &SqlitePool, id: Uuid) -> Result<PickList> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE pick_lists SET status = 'Completed', completed_at = ? WHERE id = ?"
        )
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get_pick_list(pool, id).await
    }

    pub async fn create_pack_list(
        pool: &SqlitePool,
        pick_list_id: Uuid,
        warehouse_id: Uuid,
    ) -> Result<PackList> {
        let now = chrono::Utc::now();
        let pack_number = format!("PK-{}", now.format("%Y%m%d%H%M%S"));
        let pack_list = PackList {
            id: Uuid::new_v4(),
            pack_number: pack_number.clone(),
            pick_list_id,
            warehouse_id,
            packed_by: None,
            status: PackListStatus::Pending,
            total_weight: None,
            tracking_number: None,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO pack_lists (id, pack_number, pick_list_id, warehouse_id, packed_by, status, total_weight, tracking_number, created_at)
             VALUES (?, ?, ?, ?, NULL, 'Pending', NULL, NULL, ?)"
        )
        .bind(pack_list.id.to_string())
        .bind(&pack_list.pack_number)
        .bind(pack_list.pick_list_id.to_string())
        .bind(pack_list.warehouse_id.to_string())
        .bind(pack_list.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(pack_list)
    }

    pub async fn pack_item(
        pool: &SqlitePool,
        pack_list_id: Uuid,
        product_id: Uuid,
        quantity: i64,
        box_number: i32,
    ) -> Result<PackListItem> {
        let item = PackListItem {
            id: Uuid::new_v4(),
            pack_list_id,
            product_id,
            quantity,
            box_number,
        };
        
        sqlx::query(
            "INSERT INTO pack_list_items (id, pack_list_id, product_id, quantity, box_number)
             VALUES (?, ?, ?, ?, ?)"
        )
        .bind(item.id.to_string())
        .bind(item.pack_list_id.to_string())
        .bind(item.product_id.to_string())
        .bind(item.quantity)
        .bind(item.box_number)
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(item)
    }

    pub async fn complete_pack(pool: &SqlitePool, id: Uuid) -> Result<PackList> {
        sqlx::query(
            "UPDATE pack_lists SET status = 'Completed' WHERE id = ?"
        )
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get_pack_list(pool, id).await
    }

    async fn get_pack_list(pool: &SqlitePool, id: Uuid) -> Result<PackList> {
        let row = sqlx::query_as::<_, PackListRow>(
            "SELECT id, pack_number, pick_list_id, warehouse_id, packed_by, status, total_weight, tracking_number, created_at
             FROM pack_lists WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("PackList", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn create_shipment(
        pool: &SqlitePool,
        warehouse_id: Uuid,
        carrier_id: Option<Uuid>,
        service_type: Option<&str>,
        ship_to_name: &str,
        ship_to_address: &str,
        ship_to_city: &str,
        ship_to_state: Option<&str>,
        ship_to_postal: &str,
        ship_to_country: &str,
        total_weight: Option<i64>,
        freight_charge: i64,
        insurance_charge: i64,
    ) -> Result<ShipmentOrder> {
        let now = chrono::Utc::now();
        let shipment_number = format!("SH-{}", now.format("%Y%m%d%H%M%S"));
        let shipment = ShipmentOrder {
            id: Uuid::new_v4(),
            shipment_number: shipment_number.clone(),
            warehouse_id,
            carrier_id,
            service_type: service_type.map(|s| s.to_string()),
            ship_to_name: ship_to_name.to_string(),
            ship_to_address: ship_to_address.to_string(),
            ship_to_city: ship_to_city.to_string(),
            ship_to_state: ship_to_state.map(|s| s.to_string()),
            ship_to_postal: ship_to_postal.to_string(),
            ship_to_country: ship_to_country.to_string(),
            total_weight,
            tracking_number: None,
            ship_date: None,
            estimated_delivery: None,
            actual_delivery: None,
            status: ShipmentStatus::Draft,
            freight_charge,
            insurance_charge,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO shipment_orders (id, shipment_number, warehouse_id, carrier_id, service_type, ship_to_name, ship_to_address, ship_to_city, ship_to_state, ship_to_postal, ship_to_country, total_weight, tracking_number, ship_date, estimated_delivery, actual_delivery, status, freight_charge, insurance_charge, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, NULL, NULL, NULL, NULL, 'Draft', ?, ?, ?)"
        )
        .bind(shipment.id.to_string())
        .bind(&shipment.shipment_number)
        .bind(shipment.warehouse_id.to_string())
        .bind(shipment.carrier_id.map(|id| id.to_string()))
        .bind(&shipment.service_type)
        .bind(&shipment.ship_to_name)
        .bind(&shipment.ship_to_address)
        .bind(&shipment.ship_to_city)
        .bind(&shipment.ship_to_state)
        .bind(&shipment.ship_to_postal)
        .bind(&shipment.ship_to_country)
        .bind(shipment.total_weight)
        .bind(shipment.freight_charge)
        .bind(shipment.insurance_charge)
        .bind(shipment.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(shipment)
    }

    pub async fn ship_order(pool: &SqlitePool, id: Uuid, tracking_number: &str) -> Result<ShipmentOrder> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE shipment_orders SET status = 'Shipped', tracking_number = ?, ship_date = ? WHERE id = ?"
        )
        .bind(tracking_number)
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get_shipment(pool, id).await
    }

    async fn get_shipment(pool: &SqlitePool, id: Uuid) -> Result<ShipmentOrder> {
        let row = sqlx::query_as::<_, ShipmentOrderRow>(
            "SELECT id, shipment_number, warehouse_id, carrier_id, service_type, ship_to_name, ship_to_address, ship_to_city, ship_to_state, ship_to_postal, ship_to_country, total_weight, tracking_number, ship_date, estimated_delivery, actual_delivery, status, freight_charge, insurance_charge, created_at
             FROM shipment_orders WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("ShipmentOrder", &id.to_string()))?;
        
        Ok(row.into())
    }
}

#[derive(sqlx::FromRow)]
struct PickListRow {
    id: String,
    pick_number: String,
    warehouse_id: String,
    order_id: Option<String>,
    assigned_to: Option<String>,
    priority: i64,
    status: String,
    total_items: i64,
    picked_items: i64,
    created_at: String,
    completed_at: Option<String>,
}

impl From<PickListRow> for PickList {
    fn from(r: PickListRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            pick_number: r.pick_number,
            warehouse_id: Uuid::parse_str(&r.warehouse_id).unwrap_or_default(),
            order_id: r.order_id.and_then(|id| Uuid::parse_str(&id).ok()),
            assigned_to: r.assigned_to.and_then(|id| Uuid::parse_str(&id).ok()),
            priority: r.priority as i32,
            status: match r.status.as_str() {
                "InProgress" => PickListStatus::InProgress,
                "Completed" => PickListStatus::Completed,
                "Cancelled" => PickListStatus::Cancelled,
                _ => PickListStatus::Pending,
            },
            total_items: r.total_items as i32,
            picked_items: r.picked_items as i32,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            completed_at: r.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
        }
    }
}

#[derive(sqlx::FromRow)]
struct PackListRow {
    id: String,
    pack_number: String,
    pick_list_id: String,
    warehouse_id: String,
    packed_by: Option<String>,
    status: String,
    total_weight: Option<i64>,
    tracking_number: Option<String>,
    created_at: String,
}

impl From<PackListRow> for PackList {
    fn from(r: PackListRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            pack_number: r.pack_number,
            pick_list_id: Uuid::parse_str(&r.pick_list_id).unwrap_or_default(),
            warehouse_id: Uuid::parse_str(&r.warehouse_id).unwrap_or_default(),
            packed_by: r.packed_by.and_then(|id| Uuid::parse_str(&id).ok()),
            status: match r.status.as_str() {
                "InProgress" => PackListStatus::InProgress,
                "Completed" => PackListStatus::Completed,
                "Shipped" => PackListStatus::Shipped,
                _ => PackListStatus::Pending,
            },
            total_weight: r.total_weight,
            tracking_number: r.tracking_number,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct ShipmentOrderRow {
    id: String,
    shipment_number: String,
    warehouse_id: String,
    carrier_id: Option<String>,
    service_type: Option<String>,
    ship_to_name: String,
    ship_to_address: String,
    ship_to_city: String,
    ship_to_state: Option<String>,
    ship_to_postal: String,
    ship_to_country: String,
    total_weight: Option<i64>,
    tracking_number: Option<String>,
    ship_date: Option<String>,
    estimated_delivery: Option<String>,
    actual_delivery: Option<String>,
    status: String,
    freight_charge: i64,
    insurance_charge: i64,
    created_at: String,
}

impl From<ShipmentOrderRow> for ShipmentOrder {
    fn from(r: ShipmentOrderRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            shipment_number: r.shipment_number,
            warehouse_id: Uuid::parse_str(&r.warehouse_id).unwrap_or_default(),
            carrier_id: r.carrier_id.and_then(|id| Uuid::parse_str(&id).ok()),
            service_type: r.service_type,
            ship_to_name: r.ship_to_name,
            ship_to_address: r.ship_to_address,
            ship_to_city: r.ship_to_city,
            ship_to_state: r.ship_to_state,
            ship_to_postal: r.ship_to_postal,
            ship_to_country: r.ship_to_country,
            total_weight: r.total_weight,
            tracking_number: r.tracking_number,
            ship_date: r.ship_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            estimated_delivery: r.estimated_delivery.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            actual_delivery: r.actual_delivery.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            status: match r.status.as_str() {
                "Pending" => ShipmentStatus::Pending,
                "Shipped" => ShipmentStatus::Shipped,
                "InTransit" => ShipmentStatus::InTransit,
                "Delivered" => ShipmentStatus::Delivered,
                "Cancelled" => ShipmentStatus::Cancelled,
                _ => ShipmentStatus::Draft,
            },
            freight_charge: r.freight_charge,
            insurance_charge: r.insurance_charge,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

pub struct ShippingService;

impl ShippingService {
    pub fn new() -> Self { Self }

    pub async fn create_carrier(
        pool: &SqlitePool,
        code: &str,
        name: &str,
        api_endpoint: Option<&str>,
        api_key: Option<&str>,
        account_number: Option<&str>,
        supports_tracking: bool,
        supports_label_generation: bool,
    ) -> Result<ShippingCarrier> {
        let now = chrono::Utc::now();
        let carrier = ShippingCarrier {
            id: Uuid::new_v4(),
            code: code.to_string(),
            name: name.to_string(),
            api_endpoint: api_endpoint.map(|s| s.to_string()),
            api_key: api_key.map(|s| s.to_string()),
            account_number: account_number.map(|s| s.to_string()),
            supports_tracking,
            supports_label_generation,
            status: Status::Active,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO shipping_carriers (id, code, name, api_endpoint, api_key, account_number, supports_tracking, supports_label_generation, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'Active', ?)"
        )
        .bind(carrier.id.to_string())
        .bind(&carrier.code)
        .bind(&carrier.name)
        .bind(&carrier.api_endpoint)
        .bind(&carrier.api_key)
        .bind(&carrier.account_number)
        .bind(carrier.supports_tracking)
        .bind(carrier.supports_label_generation)
        .bind(carrier.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(carrier)
    }

    pub async fn create_service(
        pool: &SqlitePool,
        carrier_id: Uuid,
        service_code: &str,
        service_name: &str,
        delivery_days: Option<i32>,
    ) -> Result<CarrierService> {
        let service = CarrierService {
            id: Uuid::new_v4(),
            carrier_id,
            service_code: service_code.to_string(),
            service_name: service_name.to_string(),
            delivery_days,
            status: Status::Active,
        };
        
        sqlx::query(
            "INSERT INTO carrier_services (id, carrier_id, service_code, service_name, delivery_days, status)
             VALUES (?, ?, ?, ?, ?, 'Active')"
        )
        .bind(service.id.to_string())
        .bind(service.carrier_id.to_string())
        .bind(&service.service_code)
        .bind(&service.service_name)
        .bind(service.delivery_days)
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(service)
    }

    pub async fn create_rate_card(
        pool: &SqlitePool,
        carrier_id: Uuid,
        service_id: Uuid,
        zone_from: &str,
        zone_to: &str,
        weight_from: i64,
        weight_to: i64,
        base_rate: i64,
        per_kg_rate: i64,
        effective_date: &str,
        expiry_date: Option<&str>,
    ) -> Result<ShippingRateCard> {
        let now = chrono::Utc::now();
        let rate_card = ShippingRateCard {
            id: Uuid::new_v4(),
            carrier_id,
            service_id,
            zone_from: zone_from.to_string(),
            zone_to: zone_to.to_string(),
            weight_from,
            weight_to,
            base_rate,
            per_kg_rate,
            effective_date: chrono::DateTime::parse_from_rfc3339(effective_date)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or(now),
            expiry_date: expiry_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
        };
        
        sqlx::query(
            "INSERT INTO shipping_rate_cards (id, carrier_id, service_id, zone_from, zone_to, weight_from, weight_to, base_rate, per_kg_rate, effective_date, expiry_date)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(rate_card.id.to_string())
        .bind(rate_card.carrier_id.to_string())
        .bind(rate_card.service_id.to_string())
        .bind(&rate_card.zone_from)
        .bind(&rate_card.zone_to)
        .bind(rate_card.weight_from)
        .bind(rate_card.weight_to)
        .bind(rate_card.base_rate)
        .bind(rate_card.per_kg_rate)
        .bind(rate_card.effective_date.to_rfc3339())
        .bind(rate_card.expiry_date.map(|d| d.to_rfc3339()))
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rate_card)
    }

    pub async fn calculate_shipping_rate(
        pool: &SqlitePool,
        carrier_id: Uuid,
        service_id: Uuid,
        zone_from: &str,
        zone_to: &str,
        weight: i64,
    ) -> Result<i64> {
        let row = sqlx::query_as::<_, (i64, i64)>(
            "SELECT base_rate, per_kg_rate FROM shipping_rate_cards
             WHERE carrier_id = ? AND service_id = ? AND zone_from = ? AND zone_to = ?
             AND weight_from <= ? AND weight_to >= ?
             AND effective_date <= datetime('now') AND (expiry_date IS NULL OR expiry_date > datetime('now'))
             LIMIT 1"
        )
        .bind(carrier_id.to_string())
        .bind(service_id.to_string())
        .bind(zone_from)
        .bind(zone_to)
        .bind(weight)
        .bind(weight)
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("ShippingRateCard", "no matching rate"))?;
        
        let (base_rate, per_kg_rate) = row;
        Ok(base_rate + (per_kg_rate * weight / 1000))
    }

    pub async fn track_shipment(pool: &SqlitePool, shipment_id: Uuid) -> Result<ShipmentOrder> {
        let row = sqlx::query_as::<_, ShipmentOrderRow>(
            "SELECT id, shipment_number, warehouse_id, carrier_id, service_type, ship_to_name, ship_to_address, ship_to_city, ship_to_state, ship_to_postal, ship_to_country, total_weight, tracking_number, ship_date, estimated_delivery, actual_delivery, status, freight_charge, insurance_charge, created_at
             FROM shipment_orders WHERE id = ?"
        )
        .bind(shipment_id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("ShipmentOrder", &shipment_id.to_string()))?;
        
        Ok(row.into())
    }
}

pub struct EDIService;

impl EDIService {
    pub fn new() -> Self { Self }

    pub async fn create_partner(
        pool: &SqlitePool,
        partner_code: &str,
        partner_name: &str,
        partner_type: EDIPartnerType,
        edi_standard: EDIStandard,
        communication_method: CommunicationMethod,
        ftp_host: Option<&str>,
        ftp_username: Option<&str>,
        ftp_password: Option<&str>,
        api_endpoint: Option<&str>,
        api_key: Option<&str>,
    ) -> Result<EDIPartner> {
        let now = chrono::Utc::now();
        let partner = EDIPartner {
            id: Uuid::new_v4(),
            partner_code: partner_code.to_string(),
            partner_name: partner_name.to_string(),
            partner_type: partner_type.clone(),
            edi_standard: edi_standard.clone(),
            communication_method: communication_method.clone(),
            ftp_host: ftp_host.map(|s| s.to_string()),
            ftp_username: ftp_username.map(|s| s.to_string()),
            ftp_password: ftp_password.map(|s| s.to_string()),
            api_endpoint: api_endpoint.map(|s| s.to_string()),
            api_key: api_key.map(|s| s.to_string()),
            status: Status::Active,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO edi_partners (id, partner_code, partner_name, partner_type, edi_standard, communication_method, ftp_host, ftp_username, ftp_password, api_endpoint, api_key, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, 'Active', ?)"
        )
        .bind(partner.id.to_string())
        .bind(&partner.partner_code)
        .bind(&partner.partner_name)
        .bind(format!("{:?}", partner.partner_type))
        .bind(format!("{:?}", partner.edi_standard))
        .bind(format!("{:?}", partner.communication_method))
        .bind(&partner.ftp_host)
        .bind(&partner.ftp_username)
        .bind(&partner.ftp_password)
        .bind(&partner.api_endpoint)
        .bind(&partner.api_key)
        .bind(partner.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(partner)
    }

    pub async fn create_mapping(
        pool: &SqlitePool,
        partner_id: Uuid,
        document_type: &str,
        segment_id: &str,
        element_position: i32,
        internal_field: &str,
        transformation_rule: Option<&str>,
    ) -> Result<EDIMapping> {
        let mapping = EDIMapping {
            id: Uuid::new_v4(),
            partner_id,
            document_type: document_type.to_string(),
            segment_id: segment_id.to_string(),
            element_position,
            internal_field: internal_field.to_string(),
            transformation_rule: transformation_rule.map(|s| s.to_string()),
        };
        
        sqlx::query(
            "INSERT INTO edi_mappings (id, partner_id, document_type, segment_id, element_position, internal_field, transformation_rule)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(mapping.id.to_string())
        .bind(mapping.partner_id.to_string())
        .bind(&mapping.document_type)
        .bind(&mapping.segment_id)
        .bind(mapping.element_position)
        .bind(&mapping.internal_field)
        .bind(&mapping.transformation_rule)
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(mapping)
    }

    pub async fn process_inbound_document(
        pool: &SqlitePool,
        partner_id: Uuid,
        document_type: EDIDocumentType,
        reference_number: Option<&str>,
        raw_content: &str,
    ) -> Result<EDIDocument> {
        let now = chrono::Utc::now();
        let document_number = format!("EDI-{}", now.format("%Y%m%d%H%M%S"));
        let document = EDIDocument {
            id: Uuid::new_v4(),
            document_number: document_number.clone(),
            partner_id,
            document_type: document_type.clone(),
            direction: EDIDirection::Inbound,
            reference_number: reference_number.map(|s| s.to_string()),
            raw_content: Some(raw_content.to_string()),
            parsed_data: None,
            status: EDIDocumentStatus::Pending,
            processed_at: None,
            error_message: None,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO edi_documents (id, document_number, partner_id, document_type, direction, reference_number, raw_content, parsed_data, status, processed_at, error_message, created_at)
             VALUES (?, ?, ?, ?, 'Inbound', ?, ?, NULL, 'Pending', NULL, NULL, ?)"
        )
        .bind(document.id.to_string())
        .bind(&document.document_number)
        .bind(document.partner_id.to_string())
        .bind(format!("{:?}", document.document_type))
        .bind(&document.reference_number)
        .bind(&document.raw_content)
        .bind(document.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(document)
    }

    pub async fn generate_outbound_document(
        pool: &SqlitePool,
        partner_id: Uuid,
        document_type: EDIDocumentType,
        reference_number: Option<&str>,
        parsed_data: &str,
    ) -> Result<EDIDocument> {
        let now = chrono::Utc::now();
        let document_number = format!("EDI-{}", now.format("%Y%m%d%H%M%S"));
        let document = EDIDocument {
            id: Uuid::new_v4(),
            document_number: document_number.clone(),
            partner_id,
            document_type: document_type.clone(),
            direction: EDIDirection::Outbound,
            reference_number: reference_number.map(|s| s.to_string()),
            raw_content: None,
            parsed_data: Some(parsed_data.to_string()),
            status: EDIDocumentStatus::Pending,
            processed_at: None,
            error_message: None,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO edi_documents (id, document_number, partner_id, document_type, direction, reference_number, raw_content, parsed_data, status, processed_at, error_message, created_at)
             VALUES (?, ?, ?, ?, 'Outbound', ?, NULL, ?, 'Pending', NULL, NULL, ?)"
        )
        .bind(document.id.to_string())
        .bind(&document.document_number)
        .bind(document.partner_id.to_string())
        .bind(format!("{:?}", document.document_type))
        .bind(&document.reference_number)
        .bind(&document.parsed_data)
        .bind(document.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(document)
    }
}

pub struct SupplierPortalService;

impl SupplierPortalService {
    pub fn new() -> Self { Self }

    pub async fn invite_supplier(
        pool: &SqlitePool,
        vendor_id: Uuid,
        email: &str,
    ) -> Result<SupplierInvitation> {
        let now = chrono::Utc::now();
        let expires_at = now + chrono::Duration::days(7);
        let invitation_token = Uuid::new_v4().to_string();
        
        let invitation = SupplierInvitation {
            id: Uuid::new_v4(),
            vendor_id,
            email: email.to_string(),
            invitation_token: invitation_token.clone(),
            expires_at,
            accepted_at: None,
            status: InvitationStatus::Pending,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO supplier_invitations (id, vendor_id, email, invitation_token, expires_at, accepted_at, status, created_at)
             VALUES (?, ?, ?, ?, ?, NULL, 'Pending', ?)"
        )
        .bind(invitation.id.to_string())
        .bind(invitation.vendor_id.to_string())
        .bind(&invitation.email)
        .bind(&invitation.invitation_token)
        .bind(invitation.expires_at.to_rfc3339())
        .bind(invitation.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(invitation)
    }

    pub async fn register_supplier_user(
        pool: &SqlitePool,
        vendor_id: Uuid,
        username: &str,
        email: &str,
        password_hash: &str,
        role: SupplierUserRole,
    ) -> Result<SupplierUser> {
        let now = chrono::Utc::now();
        let user = SupplierUser {
            id: Uuid::new_v4(),
            vendor_id,
            username: username.to_string(),
            email: email.to_string(),
            password_hash: password_hash.to_string(),
            role: role.clone(),
            last_login: None,
            status: Status::Active,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO supplier_users (id, vendor_id, username, email, password_hash, role, last_login, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, NULL, 'Active', ?)"
        )
        .bind(user.id.to_string())
        .bind(user.vendor_id.to_string())
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(format!("{:?}", user.role))
        .bind(user.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(user)
    }

    pub async fn upload_document(
        pool: &SqlitePool,
        vendor_id: Uuid,
        document_type: SupplierDocumentType,
        document_name: &str,
        file_path: &str,
        uploaded_by: Option<Uuid>,
        expiry_date: Option<&str>,
    ) -> Result<SupplierDocument> {
        let now = chrono::Utc::now();
        let document = SupplierDocument {
            id: Uuid::new_v4(),
            vendor_id,
            document_type: document_type.clone(),
            document_name: document_name.to_string(),
            file_path: file_path.to_string(),
            uploaded_by,
            expiry_date: expiry_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            status: Status::Active,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO supplier_documents (id, vendor_id, document_type, document_name, file_path, uploaded_by, expiry_date, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, 'Active', ?)"
        )
        .bind(document.id.to_string())
        .bind(document.vendor_id.to_string())
        .bind(format!("{:?}", document.document_type))
        .bind(&document.document_name)
        .bind(&document.file_path)
        .bind(document.uploaded_by.map(|id| id.to_string()))
        .bind(document.expiry_date.map(|d| d.to_rfc3339()))
        .bind(document.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(document)
    }

    pub async fn get_documents(pool: &SqlitePool, vendor_id: Uuid) -> Result<Vec<SupplierDocument>> {
        let rows = sqlx::query_as::<_, SupplierDocumentRow>(
            "SELECT id, vendor_id, document_type, document_name, file_path, uploaded_by, expiry_date, status, created_at
             FROM supplier_documents WHERE vendor_id = ? ORDER BY created_at DESC"
        )
        .bind(vendor_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct SupplierDocumentRow {
    id: String,
    vendor_id: String,
    document_type: String,
    document_name: String,
    file_path: String,
    uploaded_by: Option<String>,
    expiry_date: Option<String>,
    status: String,
    created_at: String,
}

impl From<SupplierDocumentRow> for SupplierDocument {
    fn from(r: SupplierDocumentRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            vendor_id: Uuid::parse_str(&r.vendor_id).unwrap_or_default(),
            document_type: match r.document_type.as_str() {
                "Certificate" => SupplierDocumentType::Certificate,
                "Insurance" => SupplierDocumentType::Insurance,
                "TaxForm" => SupplierDocumentType::TaxForm,
                "Contract" => SupplierDocumentType::Contract,
                _ => SupplierDocumentType::Other,
            },
            document_name: r.document_name,
            file_path: r.file_path,
            uploaded_by: r.uploaded_by.and_then(|id| Uuid::parse_str(&id).ok()),
            expiry_date: r.expiry_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            status: match r.status.as_str() {
                "Inactive" => Status::Inactive,
                _ => Status::Active,
            },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

pub struct RFQService;

impl RFQService {
    pub fn new() -> Self { Self }

    pub async fn create_rfq(
        pool: &SqlitePool,
        title: &str,
        description: Option<&str>,
        buyer_id: Uuid,
        currency: &str,
        submission_deadline: &str,
        valid_until: Option<&str>,
    ) -> Result<RFQ> {
        let now = chrono::Utc::now();
        let rfq_number = format!("RFQ-{}", now.format("%Y%m%d%H%M%S"));
        let rfq = RFQ {
            id: Uuid::new_v4(),
            rfq_number: rfq_number.clone(),
            title: title.to_string(),
            description: description.map(|s| s.to_string()),
            buyer_id,
            currency: currency.to_string(),
            submission_deadline: chrono::DateTime::parse_from_rfc3339(submission_deadline)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or(now),
            valid_until: valid_until.and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            status: RFQStatus::Draft,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO rfqs (id, rfq_number, title, description, buyer_id, currency, submission_deadline, valid_until, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'Draft', ?)"
        )
        .bind(rfq.id.to_string())
        .bind(&rfq.rfq_number)
        .bind(&rfq.title)
        .bind(&rfq.description)
        .bind(rfq.buyer_id.to_string())
        .bind(&rfq.currency)
        .bind(rfq.submission_deadline.to_rfc3339())
        .bind(rfq.valid_until.map(|d| d.to_rfc3339()))
        .bind(rfq.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rfq)
    }

    pub async fn add_vendor(
        pool: &SqlitePool,
        rfq_id: Uuid,
        vendor_id: Uuid,
    ) -> Result<RFQVendor> {
        let now = chrono::Utc::now();
        let rfq_vendor = RFQVendor {
            id: Uuid::new_v4(),
            rfq_id,
            vendor_id,
            invited_at: Some(now),
            responded_at: None,
            status: RFQVendorStatus::Invited,
        };
        
        sqlx::query(
            "INSERT INTO rfq_vendors (id, rfq_id, vendor_id, invited_at, responded_at, status)
             VALUES (?, ?, ?, ?, NULL, 'Invited')"
        )
        .bind(rfq_vendor.id.to_string())
        .bind(rfq_vendor.rfq_id.to_string())
        .bind(rfq_vendor.vendor_id.to_string())
        .bind(rfq_vendor.invited_at.map(|d| d.to_rfc3339()))
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rfq_vendor)
    }

    pub async fn submit_to_vendor(pool: &SqlitePool, rfq_id: Uuid) -> Result<RFQ> {
        sqlx::query(
            "UPDATE rfqs SET status = 'Published' WHERE id = ?"
        )
        .bind(rfq_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get_rfq(pool, rfq_id).await
    }

    pub async fn submit_response(
        pool: &SqlitePool,
        rfq_id: Uuid,
        vendor_id: Uuid,
        payment_terms: Option<i32>,
        delivery_terms: Option<&str>,
        notes: Option<&str>,
        valid_until: Option<&str>,
    ) -> Result<RFQResponse> {
        let now = chrono::Utc::now();
        let response_number = format!("RSP-{}", now.format("%Y%m%d%H%M%S"));
        let response = RFQResponse {
            id: Uuid::new_v4(),
            rfq_id,
            vendor_id,
            response_number: response_number.clone(),
            response_date: now,
            valid_until: valid_until.and_then(|d| chrono::DateTime::parse_from_rfc3339(d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            payment_terms,
            delivery_terms: delivery_terms.map(|s| s.to_string()),
            notes: notes.map(|s| s.to_string()),
            status: RFQResponseStatus::Submitted,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO rfq_responses (id, rfq_id, vendor_id, response_number, response_date, valid_until, payment_terms, delivery_terms, notes, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, 'Submitted', ?)"
        )
        .bind(response.id.to_string())
        .bind(response.rfq_id.to_string())
        .bind(response.vendor_id.to_string())
        .bind(&response.response_number)
        .bind(response.response_date.to_rfc3339())
        .bind(response.valid_until.map(|d| d.to_rfc3339()))
        .bind(response.payment_terms)
        .bind(&response.delivery_terms)
        .bind(&response.notes)
        .bind(response.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        sqlx::query(
            "UPDATE rfq_vendors SET status = 'Responded', responded_at = ? WHERE rfq_id = ? AND vendor_id = ?"
        )
        .bind(now.to_rfc3339())
        .bind(rfq_id.to_string())
        .bind(vendor_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(response)
    }

    pub async fn compare_responses(pool: &SqlitePool, rfq_id: Uuid) -> Result<Vec<RFQResponse>> {
        let rows = sqlx::query_as::<_, RFQResponseRow>(
            "SELECT id, rfq_id, vendor_id, response_number, response_date, valid_until, payment_terms, delivery_terms, notes, status, created_at
             FROM rfq_responses WHERE rfq_id = ? ORDER BY created_at"
        )
        .bind(rfq_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn award_rfq(pool: &SqlitePool, rfq_id: Uuid, vendor_id: Uuid) -> Result<RFQ> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE rfqs SET status = 'Awarded' WHERE id = ?"
        )
        .bind(rfq_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        sqlx::query(
            "UPDATE rfq_vendors SET status = 'Awarded' WHERE rfq_id = ? AND vendor_id = ?"
        )
        .bind(rfq_id.to_string())
        .bind(vendor_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        sqlx::query(
            "UPDATE rfq_responses SET status = 'Accepted' WHERE rfq_id = ? AND vendor_id = ?"
        )
        .bind(rfq_id.to_string())
        .bind(vendor_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Self::get_rfq(pool, rfq_id).await
    }

    async fn get_rfq(pool: &SqlitePool, id: Uuid) -> Result<RFQ> {
        let row = sqlx::query_as::<_, RFQRow>(
            "SELECT id, rfq_number, title, description, buyer_id, currency, submission_deadline, valid_until, status, created_at
             FROM rfqs WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("RFQ", &id.to_string()))?;
        
        Ok(row.into())
    }
}

#[derive(sqlx::FromRow)]
struct RFQRow {
    id: String,
    rfq_number: String,
    title: String,
    description: Option<String>,
    buyer_id: String,
    currency: String,
    submission_deadline: String,
    valid_until: Option<String>,
    status: String,
    created_at: String,
}

impl From<RFQRow> for RFQ {
    fn from(r: RFQRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            rfq_number: r.rfq_number,
            title: r.title,
            description: r.description,
            buyer_id: Uuid::parse_str(&r.buyer_id).unwrap_or_default(),
            currency: r.currency,
            submission_deadline: chrono::DateTime::parse_from_rfc3339(&r.submission_deadline)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            valid_until: r.valid_until.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            status: match r.status.as_str() {
                "Published" => RFQStatus::Published,
                "Closed" => RFQStatus::Closed,
                "Awarded" => RFQStatus::Awarded,
                "Cancelled" => RFQStatus::Cancelled,
                _ => RFQStatus::Draft,
            },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct RFQResponseRow {
    id: String,
    rfq_id: String,
    vendor_id: String,
    response_number: String,
    response_date: String,
    valid_until: Option<String>,
    payment_terms: Option<i64>,
    delivery_terms: Option<String>,
    notes: Option<String>,
    status: String,
    created_at: String,
}

impl From<RFQResponseRow> for RFQResponse {
    fn from(r: RFQResponseRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            rfq_id: Uuid::parse_str(&r.rfq_id).unwrap_or_default(),
            vendor_id: Uuid::parse_str(&r.vendor_id).unwrap_or_default(),
            response_number: r.response_number,
            response_date: chrono::DateTime::parse_from_rfc3339(&r.response_date)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
            valid_until: r.valid_until.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            payment_terms: r.payment_terms.map(|t| t as i32),
            delivery_terms: r.delivery_terms,
            notes: r.notes,
            status: match r.status.as_str() {
                "UnderReview" => RFQResponseStatus::UnderReview,
                "Accepted" => RFQResponseStatus::Accepted,
                "Rejected" => RFQResponseStatus::Rejected,
                _ => RFQResponseStatus::Submitted,
            },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}
