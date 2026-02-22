use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status, Money, Currency};
use crate::models::*;
use crate::repository::*;

pub struct VendorService { repo: SqliteVendorRepository }
impl VendorService {
    pub fn new() -> Self { Self { repo: SqliteVendorRepository } }
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<Vendor> { self.repo.find_by_id(pool, id).await }
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Vendor>> { self.repo.find_all(pool, pagination).await }
    pub async fn create(&self, pool: &SqlitePool, vendor: Vendor) -> Result<Vendor> {
        if vendor.code.is_empty() { return Err(Error::validation("Vendor code is required")); }
        if vendor.name.is_empty() { return Err(Error::validation("Vendor name is required")); }
        self.repo.create(pool, vendor).await
    }
    pub async fn update(&self, pool: &SqlitePool, vendor: Vendor) -> Result<Vendor> { self.repo.update(pool, vendor).await }
}

pub struct PurchaseOrderService { repo: SqlitePurchaseOrderRepository }
impl PurchaseOrderService {
    pub fn new() -> Self { Self { repo: SqlitePurchaseOrderRepository } }
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<PurchaseOrder> { self.repo.find_by_id(pool, id).await }
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<PurchaseOrder>> { self.repo.find_all(pool, pagination).await }
    
    pub async fn create(&self, pool: &SqlitePool, mut order: PurchaseOrder) -> Result<PurchaseOrder> {
        if order.lines.is_empty() { return Err(Error::validation("PO must have at least one line")); }
        let subtotal: i64 = order.lines.iter().map(|l| l.line_total.amount).sum();
        order.subtotal = Money::new(subtotal, Currency::USD);
        order.total = Money::new(subtotal + order.tax_amount.amount, Currency::USD);
        order.po_number = format!("PO-{}", Utc::now().format("%Y%m%d%H%M%S"));
        order.base = BaseEntity::new();
        order.status = Status::Draft;
        for line in &mut order.lines { line.id = Uuid::new_v4(); }
        self.repo.create(pool, order).await
    }
    
    pub async fn submit(&self, pool: &SqlitePool, id: Uuid) -> Result<()> { self.repo.update_status(pool, id, Status::Pending).await }
    pub async fn approve(&self, pool: &SqlitePool, id: Uuid) -> Result<()> { self.repo.update_status(pool, id, Status::Approved).await }
    pub async fn receive(&self, pool: &SqlitePool, id: Uuid) -> Result<()> { self.repo.update_status(pool, id, Status::Completed).await }
}
