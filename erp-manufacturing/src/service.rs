use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status};
use crate::models::*;
use crate::repository::*;

pub struct BillOfMaterialService { repo: SqliteBillOfMaterialRepository }
impl BillOfMaterialService {
    pub fn new() -> Self { Self { repo: SqliteBillOfMaterialRepository } }
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<BillOfMaterial> { self.repo.find_by_id(pool, id).await }
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<BillOfMaterial>> { self.repo.find_all(pool, pagination).await }
    
    pub async fn create(&self, pool: &SqlitePool, mut bom: BillOfMaterial) -> Result<BillOfMaterial> {
        if bom.components.is_empty() { return Err(Error::validation("BOM must have at least one component")); }
        if bom.name.is_empty() { return Err(Error::validation("BOM name is required")); }
        bom.base = BaseEntity::new();
        bom.status = Status::Draft;
        for c in &mut bom.components { c.id = Uuid::new_v4(); }
        self.repo.create(pool, bom).await
    }
}

pub struct WorkOrderService { repo: SqliteWorkOrderRepository }
impl WorkOrderService {
    pub fn new() -> Self { Self { repo: SqliteWorkOrderRepository } }
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<WorkOrder> { self.repo.find_by_id(pool, id).await }
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<WorkOrder>> { self.repo.find_all(pool, pagination).await }
    
    pub async fn create(&self, pool: &SqlitePool, mut order: WorkOrder) -> Result<WorkOrder> {
        if order.quantity <= 0 { return Err(Error::validation("Quantity must be positive")); }
        order.order_number = format!("WO-{}", Utc::now().format("%Y%m%d%H%M%S"));
        order.base = BaseEntity::new();
        order.status = Status::Draft;
        self.repo.create(pool, order).await
    }
    
    pub async fn start(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, Status::Pending, Some(Utc::now().to_rfc3339()), None).await
    }
    
    pub async fn complete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, Status::Completed, None, Some(Utc::now().to_rfc3339())).await
    }
}
