use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use std::str::FromStr;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status, Money, Currency, Address, ContactInfo};
use crate::models::*;

#[derive(sqlx::FromRow)]
struct VendorRow {
    id: String, code: String, name: String, email: Option<String>, phone: Option<String>,
    payment_terms: i64, status: String, created_at: String, updated_at: String,
}

impl VendorRow {
    fn into_vendor(self) -> Vendor {
        Vendor {
            base: BaseEntity {
                id: Uuid::parse_str(&self.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None, updated_by: None,
            },
            code: self.code, name: self.name,
            contact: ContactInfo { email: self.email, phone: self.phone, fax: None, website: None },
            address: Address { street: String::new(), city: String::new(), state: None, postal_code: String::new(), country: String::new() },
            payment_terms: self.payment_terms as u32,
            status: match self.status.as_str() { "Inactive" => Status::Inactive, _ => Status::Active },
        }
    }
}

pub struct SqliteVendorRepository;

#[async_trait]
impl VendorRepository for SqliteVendorRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Vendor> {
        let row = sqlx::query_as::<_, VendorRow>(
            "SELECT id, code, name, email, phone, payment_terms, status, created_at, updated_at FROM vendors WHERE id = ?"
        ).bind(id.to_string()).fetch_optional(pool).await?.ok_or_else(|| Error::not_found("Vendor", &id.to_string()))?;
        Ok(row.into_vendor())
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Vendor>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM vendors WHERE status != 'Deleted'").fetch_one(pool).await?;
        let rows = sqlx::query_as::<_, VendorRow>(
            "SELECT id, code, name, email, phone, payment_terms, status, created_at, updated_at FROM vendors WHERE status != 'Deleted' ORDER BY code LIMIT ? OFFSET ?"
        ).bind(pagination.limit() as i64).bind(pagination.offset() as i64).fetch_all(pool).await?;
        Ok(Paginated::new(rows.into_iter().map(|r| r.into_vendor()).collect(), count.0 as u64, pagination))
    }

    async fn create(&self, pool: &SqlitePool, vendor: Vendor) -> Result<Vendor> {
        let now = Utc::now();
        sqlx::query("INSERT INTO vendors (id, code, name, email, phone, payment_terms, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(vendor.base.id.to_string()).bind(&vendor.code).bind(&vendor.name)
            .bind(&vendor.contact.email).bind(&vendor.contact.phone).bind(vendor.payment_terms as i64)
            .bind(format!("{:?}", vendor.status)).bind(vendor.base.created_at.to_rfc3339()).bind(now.to_rfc3339())
            .execute(pool).await?;
        Ok(vendor)
    }

    async fn update(&self, pool: &SqlitePool, vendor: Vendor) -> Result<Vendor> {
        let now = Utc::now();
        let rows = sqlx::query("UPDATE vendors SET code=?, name=?, email=?, phone=?, payment_terms=?, status=?, updated_at=? WHERE id=?")
            .bind(&vendor.code).bind(&vendor.name).bind(&vendor.contact.email).bind(&vendor.contact.phone)
            .bind(vendor.payment_terms as i64).bind(format!("{:?}", vendor.status)).bind(now.to_rfc3339()).bind(vendor.base.id.to_string())
            .execute(pool).await?;
        if rows.rows_affected() == 0 { return Err(Error::not_found("Vendor", &vendor.base.id.to_string())); }
        Ok(vendor)
    }
}

#[derive(sqlx::FromRow)]
struct PurchaseOrderRow {
    id: String, po_number: String, vendor_id: String, order_date: String,
    subtotal: i64, tax_amount: i64, total: i64, status: String, created_at: String, updated_at: String,
}

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct POLineRow {
    id: String, purchase_order_id: String, product_id: String, description: String,
    quantity: i64, unit_price: i64, line_total: i64,
}

pub struct SqlitePurchaseOrderRepository;

#[async_trait]
impl PurchaseOrderRepository for SqlitePurchaseOrderRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<PurchaseOrder> {
        let row = sqlx::query_as::<_, PurchaseOrderRow>(
            "SELECT id, po_number, vendor_id, order_date, subtotal, tax_amount, total, status, created_at, updated_at FROM purchase_orders WHERE id = ?"
        ).bind(id.to_string()).fetch_optional(pool).await?.ok_or_else(|| Error::not_found("PurchaseOrder", &id.to_string()))?;
        
        let lines = sqlx::query_as::<_, POLineRow>(
            "SELECT id, purchase_order_id, product_id, description, quantity, unit_price, line_total FROM purchase_order_lines WHERE purchase_order_id = ?"
        ).bind(id.to_string()).fetch_all(pool).await?;
        
        Ok(PurchaseOrder {
            base: BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None, updated_by: None,
            },
            po_number: row.po_number,
            vendor_id: Uuid::parse_str(&row.vendor_id).unwrap_or_default(),
            order_date: chrono::DateTime::parse_from_rfc3339(&row.order_date).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            expected_date: None,
            lines: lines.into_iter().map(|l| PurchaseOrderLine {
                id: Uuid::parse_str(&l.id).unwrap_or_default(),
                product_id: Uuid::parse_str(&l.product_id).unwrap_or_default(),
                description: l.description, quantity: l.quantity,
                unit_price: Money::new(l.unit_price, Currency::USD), tax_rate: 0.0,
                line_total: Money::new(l.line_total, Currency::USD),
            }).collect(),
            subtotal: Money::new(row.subtotal, Currency::USD),
            tax_amount: Money::new(row.tax_amount, Currency::USD),
            total: Money::new(row.total, Currency::USD),
            status: match row.status.as_str() { "Approved" => Status::Approved, "Completed" => Status::Completed, _ => Status::Draft },
        })
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<PurchaseOrder>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM purchase_orders").fetch_one(pool).await?;
        let rows = sqlx::query_as::<_, PurchaseOrderRow>(
            "SELECT id, po_number, vendor_id, order_date, subtotal, tax_amount, total, status, created_at, updated_at FROM purchase_orders ORDER BY order_date DESC LIMIT ? OFFSET ?"
        ).bind(pagination.limit() as i64).bind(pagination.offset() as i64).fetch_all(pool).await?;
        
        let mut orders = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id).unwrap_or_default();
            let lines = sqlx::query_as::<_, POLineRow>(
                "SELECT id, purchase_order_id, product_id, description, quantity, unit_price, line_total FROM purchase_order_lines WHERE purchase_order_id = ?"
            ).bind(id.to_string()).fetch_all(pool).await?;
            
            orders.push(PurchaseOrder {
                base: BaseEntity { id, created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()), created_by: None, updated_by: None },
                po_number: row.po_number, vendor_id: Uuid::parse_str(&row.vendor_id).unwrap_or_default(),
                order_date: chrono::DateTime::parse_from_rfc3339(&row.order_date).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                expected_date: None,
                lines: lines.into_iter().map(|l| PurchaseOrderLine {
                    id: Uuid::parse_str(&l.id).unwrap_or_default(), product_id: Uuid::parse_str(&l.product_id).unwrap_or_default(),
                    description: l.description, quantity: l.quantity, unit_price: Money::new(l.unit_price, Currency::USD),
                    tax_rate: 0.0, line_total: Money::new(l.line_total, Currency::USD),
                }).collect(),
                subtotal: Money::new(row.subtotal, Currency::USD), tax_amount: Money::new(row.tax_amount, Currency::USD),
                total: Money::new(row.total, Currency::USD),
                status: match row.status.as_str() { "Approved" => Status::Approved, "Completed" => Status::Completed, _ => Status::Draft },
            });
        }
        Ok(Paginated::new(orders, count.0 as u64, pagination))
    }

    async fn create(&self, pool: &SqlitePool, order: PurchaseOrder) -> Result<PurchaseOrder> {
        let now = Utc::now();
        let mut tx = pool.begin().await?;
        
        sqlx::query("INSERT INTO purchase_orders (id, po_number, vendor_id, order_date, subtotal, tax_amount, total, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(order.base.id.to_string()).bind(&order.po_number).bind(order.vendor_id.to_string())
            .bind(order.order_date.to_rfc3339()).bind(order.subtotal.amount).bind(order.tax_amount.amount)
            .bind(order.total.amount).bind(format!("{:?}", order.status))
            .bind(order.base.created_at.to_rfc3339()).bind(now.to_rfc3339())
            .execute(&mut *tx).await?;
        
        for line in &order.lines {
            sqlx::query("INSERT INTO purchase_order_lines (id, purchase_order_id, product_id, description, quantity, unit_price, line_total) VALUES (?, ?, ?, ?, ?, ?, ?)")
                .bind(line.id.to_string()).bind(order.base.id.to_string()).bind(line.product_id.to_string())
                .bind(&line.description).bind(line.quantity).bind(line.unit_price.amount).bind(line.line_total.amount)
                .execute(&mut *tx).await?;
        }
        tx.commit().await?;
        Ok(order)
    }

    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: Status) -> Result<()> {
        let rows = sqlx::query("UPDATE purchase_orders SET status = ?, updated_at = ? WHERE id = ?")
            .bind(format!("{:?}", status)).bind(Utc::now().to_rfc3339()).bind(id.to_string()).execute(pool).await?;
        if rows.rows_affected() == 0 { return Err(Error::not_found("PurchaseOrder", &id.to_string())); }
        Ok(())
    }
}

pub struct SqliteLandedCostRepository;

#[async_trait]
impl LandedCostRepository for SqliteLandedCostRepository {
    async fn find_voucher_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<LandedCostVoucher> {
        let row = sqlx::query_as::<_, LandedCostVoucherRow>(
            "SELECT id, voucher_number, voucher_date, reference_type, reference_id, total_landed_cost, currency, status, created_at, updated_at FROM landed_cost_vouchers WHERE id = ?"
        ).bind(id.to_string()).fetch_optional(pool).await?.ok_or_else(|| Error::not_found("LandedCostVoucher", &id.to_string()))?;
        
        let lines = sqlx::query_as::<_, LandedCostLineRow>(
            "SELECT id, voucher_id, category_id, description, amount FROM landed_cost_lines WHERE voucher_id = ?"
        ).bind(id.to_string()).fetch_all(pool).await?;
        
        Ok(LandedCostVoucher {
            base: BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None, updated_by: None,
            },
            voucher_number: row.voucher_number,
            voucher_date: chrono::DateTime::parse_from_rfc3339(&row.voucher_date).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            reference_type: match row.reference_type.as_str() {
                "PurchaseOrder" => LandedCostReferenceType::PurchaseOrder,
                _ => LandedCostReferenceType::GoodsReceipt,
            },
            reference_id: Uuid::parse_str(&row.reference_id).unwrap_or_default(),
            total_landed_cost: Money::new(row.total_landed_cost, Currency::from_str(&row.currency).unwrap_or(Currency::USD)),
            status: match row.status.as_str() {
                "Posted" => Status::Posted,
                "Cancelled" => Status::Cancelled,
                _ => Status::Draft,
            },
            lines: lines.into_iter().map(|l| LandedCostLine {
                id: Uuid::parse_str(&l.id).unwrap_or_default(),
                voucher_id: Uuid::parse_str(&l.voucher_id).unwrap_or_default(),
                category_id: Uuid::parse_str(&l.category_id).unwrap_or_default(),
                description: l.description,
                amount: Money::new(l.amount, Currency::USD),
            }).collect(),
        })
    }

    async fn find_categories(&self, pool: &SqlitePool) -> Result<Vec<LandedCostCategory>> {
        let rows = sqlx::query_as::<_, LandedCostCategoryRow>(
            "SELECT id, code, name, description, allocation_method, status, created_at, updated_at FROM landed_cost_categories WHERE status = 'Active'"
        ).fetch_all(pool).await?;
        
        Ok(rows.into_iter().map(|r| LandedCostCategory {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            code: r.code, name: r.name, description: r.description,
            allocation_method: match r.allocation_method.as_str() {
                "Weight" => LandedCostAllocationMethod::ByWeight,
                "Volume" => LandedCostAllocationMethod::ByVolume,
                "Quantity" => LandedCostAllocationMethod::ByQuantity,
                _ => LandedCostAllocationMethod::ByValue,
            },
            status: Status::Active,
        }).collect())
    }

    async fn create_voucher(&self, pool: &SqlitePool, voucher: LandedCostVoucher) -> Result<LandedCostVoucher> {
        let now = Utc::now();
        let mut tx = pool.begin().await?;
        
        sqlx::query(
            "INSERT INTO landed_cost_vouchers (id, voucher_number, voucher_date, reference_type, reference_id, total_landed_cost, currency, status, created_at, updated_at) 
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(voucher.base.id.to_string())
        .bind(&voucher.voucher_number)
        .bind(voucher.voucher_date.to_rfc3339())
        .bind(format!("{:?}", voucher.reference_type))
        .bind(voucher.reference_id.to_string())
        .bind(voucher.total_landed_cost.amount)
        .bind(format!("{:?}", voucher.total_landed_cost.currency))
        .bind(format!("{:?}", voucher.status))
        .bind(voucher.base.created_at.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(&mut *tx).await?;
        
        for line in &voucher.lines {
            sqlx::query(
                "INSERT INTO landed_cost_lines (id, voucher_id, category_id, description, amount) 
                 VALUES (?, ?, ?, ?, ?)"
            )
            .bind(line.id.to_string())
            .bind(voucher.base.id.to_string())
            .bind(line.category_id.to_string())
            .bind(&line.description)
            .bind(line.amount.amount)
            .execute(&mut *tx).await?;
        }
        
        tx.commit().await?;
        Ok(voucher)
    }

    async fn update_voucher_status(&self, pool: &SqlitePool, id: Uuid, status: Status) -> Result<()> {
        sqlx::query("UPDATE landed_cost_vouchers SET status = ?, updated_at = ? WHERE id = ?")
            .bind(format!("{:?}", status)).bind(Utc::now().to_rfc3339()).bind(id.to_string()).execute(pool).await?;
        Ok(())
    }

    async fn save_allocations(&self, pool: &SqlitePool, allocations: Vec<LandedCostAllocation>) -> Result<()> {
        let now = Utc::now();
        let mut tx = pool.begin().await?;
        
        for allocation in allocations {
            sqlx::query(
                "INSERT INTO landed_cost_allocations (id, voucher_id, item_id, allocated_amount, allocation_factor, created_at) 
                 VALUES (?, ?, ?, ?, ?, ?)"
            )
            .bind(allocation.id.to_string())
            .bind(allocation.voucher_id.to_string())
            .bind(allocation.item_id.to_string())
            .bind(allocation.allocated_amount.amount)
            .bind(allocation.allocation_factor)
            .bind(now.to_rfc3339())
            .execute(&mut *tx).await?;
        }
        
        tx.commit().await?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct LandedCostVoucherRow {
    id: String, voucher_number: String, voucher_date: String, reference_type: String, 
    reference_id: String, total_landed_cost: i64, currency: String, status: String, created_at: String, updated_at: String,
}

#[derive(sqlx::FromRow)]
struct LandedCostLineRow {
    id: String, voucher_id: String, category_id: String, description: String, amount: i64,
}

#[derive(sqlx::FromRow)]
struct LandedCostCategoryRow {
    id: String, code: String, name: String, description: Option<String>, allocation_method: String, status: String, created_at: String, updated_at: String,
}

#[async_trait]
pub trait VendorRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Vendor>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Vendor>>;
    async fn create(&self, pool: &SqlitePool, vendor: Vendor) -> Result<Vendor>;
    async fn update(&self, pool: &SqlitePool, vendor: Vendor) -> Result<Vendor>;
}

#[async_trait]
pub trait PurchaseOrderRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<PurchaseOrder>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<PurchaseOrder>>;
    async fn create(&self, pool: &SqlitePool, order: PurchaseOrder) -> Result<PurchaseOrder>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: Status) -> Result<()>;
}

#[async_trait]
pub trait LandedCostRepository: Send + Sync {
    async fn find_voucher_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<LandedCostVoucher>;
    async fn find_categories(&self, pool: &SqlitePool) -> Result<Vec<LandedCostCategory>>;
    async fn create_voucher(&self, pool: &SqlitePool, voucher: LandedCostVoucher) -> Result<LandedCostVoucher>;
    async fn update_voucher_status(&self, pool: &SqlitePool, id: Uuid, status: Status) -> Result<()>;
    async fn save_allocations(&self, pool: &SqlitePool, allocations: Vec<LandedCostAllocation>) -> Result<()>;
}
