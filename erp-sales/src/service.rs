use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status, Money, Currency};
use crate::models::*;
use crate::repository::*;

pub struct CustomerService { repo: SqliteCustomerRepository }
impl CustomerService {
    pub fn new() -> Self { Self { repo: SqliteCustomerRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<Customer> { self.repo.find_by_id(pool, id).await }
    
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Customer>> {
        self.repo.find_all(pool, pagination).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, customer: Customer) -> Result<Customer> {
        if customer.code.is_empty() { return Err(Error::validation("Customer code is required")); }
        if customer.name.is_empty() { return Err(Error::validation("Customer name is required")); }
        self.repo.create(pool, customer).await
    }
    
    pub async fn update(&self, pool: &SqlitePool, customer: Customer) -> Result<Customer> {
        self.repo.update(pool, customer).await
    }
}

pub struct SalesOrderService { repo: SqliteSalesOrderRepository }
impl SalesOrderService {
    pub fn new() -> Self { Self { repo: SqliteSalesOrderRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<SalesOrder> { self.repo.find_by_id(pool, id).await }
    
    pub async fn list(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<SalesOrder>> {
        self.repo.find_all(pool, pagination).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut order: SalesOrder) -> Result<SalesOrder> {
        if order.lines.is_empty() { return Err(Error::validation("Order must have at least one line")); }
        
        let subtotal: i64 = order.lines.iter().map(|l| l.line_total.amount).sum();
        order.subtotal = Money::new(subtotal, Currency::USD);
        order.total = Money::new(subtotal + order.tax_amount.amount, Currency::USD);
        order.order_number = format!("SO-{}", Utc::now().format("%Y%m%d%H%M%S"));
        order.base = BaseEntity::new();
        order.status = Status::Draft;
        
        for line in &mut order.lines { line.id = Uuid::new_v4(); }
        self.repo.create(pool, order).await
    }
    
    pub async fn confirm(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, Status::Approved).await
    }
    
    pub async fn ship(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, Status::Completed).await
    }
}
