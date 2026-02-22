use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity, Status, Money, Currency, Address, ContactInfo};
use crate::models::*;

#[derive(sqlx::FromRow)]
struct CustomerRow {
    id: String,
    code: String,
    name: String,
    email: Option<String>,
    phone: Option<String>,
    credit_limit: Option<i64>,
    payment_terms: i64,
    status: String,
    created_at: String,
    updated_at: String,
}

impl CustomerRow {
    fn into_customer(self) -> Customer {
        Customer {
            base: BaseEntity {
                id: Uuid::parse_str(&self.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            code: self.code,
            name: self.name,
            contact: ContactInfo { email: self.email, phone: self.phone, fax: None, website: None },
            billing_address: Address { street: String::new(), city: String::new(), state: None, postal_code: String::new(), country: String::new() },
            shipping_address: None,
            credit_limit: self.credit_limit.map(|v| Money::new(v, Currency::USD)),
            payment_terms: self.payment_terms as u32,
            status: match self.status.as_str() { "Inactive" => Status::Inactive, _ => Status::Active },
        }
    }
}

pub struct SqliteCustomerRepository;

#[async_trait]
impl CustomerRepository for SqliteCustomerRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Customer> {
        let row = sqlx::query_as::<_, CustomerRow>(
            "SELECT id, code, name, email, phone, credit_limit, payment_terms, status, created_at, updated_at
             FROM customers WHERE id = ?"
        ).bind(id.to_string()).fetch_optional(pool).await?
        .ok_or_else(|| Error::not_found("Customer", &id.to_string()))?;
        Ok(row.into_customer())
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Customer>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM customers WHERE status != 'Deleted'").fetch_one(pool).await?;
        let rows = sqlx::query_as::<_, CustomerRow>(
            "SELECT id, code, name, email, phone, credit_limit, payment_terms, status, created_at, updated_at
             FROM customers WHERE status != 'Deleted' ORDER BY code LIMIT ? OFFSET ?"
        ).bind(pagination.limit() as i64).bind(pagination.offset() as i64).fetch_all(pool).await?;
        Ok(Paginated::new(rows.into_iter().map(|r| r.into_customer()).collect(), count.0 as u64, pagination))
    }

    async fn create(&self, pool: &SqlitePool, customer: Customer) -> Result<Customer> {
        let now = Utc::now();
        let credit_limit = customer.credit_limit.as_ref().map(|m| m.amount);
        sqlx::query(
            "INSERT INTO customers (id, code, name, email, phone, credit_limit, payment_terms, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        ).bind(customer.base.id.to_string()).bind(&customer.code).bind(&customer.name)
        .bind(&customer.contact.email).bind(&customer.contact.phone)
        .bind(credit_limit).bind(customer.payment_terms as i64)
        .bind(format!("{:?}", customer.status)).bind(customer.base.created_at.to_rfc3339()).bind(now.to_rfc3339())
        .execute(pool).await?;
        Ok(customer)
    }

    async fn update(&self, pool: &SqlitePool, customer: Customer) -> Result<Customer> {
        let now = Utc::now();
        let credit_limit = customer.credit_limit.as_ref().map(|m| m.amount);
        let id = customer.base.id.to_string();
        let rows = sqlx::query(
            "UPDATE customers SET code=?, name=?, email=?, phone=?, credit_limit=?, payment_terms=?, status=?, updated_at=? WHERE id=?"
        ).bind(&customer.code).bind(&customer.name).bind(&customer.contact.email).bind(&customer.contact.phone)
        .bind(credit_limit).bind(customer.payment_terms as i64)
        .bind(format!("{:?}", customer.status)).bind(now.to_rfc3339()).bind(&id)
        .execute(pool).await?;
        if rows.rows_affected() == 0 { return Err(Error::not_found("Customer", &id)); }
        Ok(customer)
    }
}

#[derive(sqlx::FromRow)]
struct SalesOrderRow {
    id: String,
    order_number: String,
    customer_id: String,
    order_date: String,
    subtotal: i64,
    tax_amount: i64,
    total: i64,
    status: String,
    created_at: String,
    updated_at: String,
}

#[derive(sqlx::FromRow)]
#[allow(dead_code)]
struct SalesOrderLineRow {
    id: String,
    sales_order_id: String,
    product_id: String,
    description: String,
    quantity: i64,
    unit_price: i64,
    discount_percent: f64,
    line_total: i64,
}

pub struct SqliteSalesOrderRepository;

#[async_trait]
impl SalesOrderRepository for SqliteSalesOrderRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<SalesOrder> {
        let row = sqlx::query_as::<_, SalesOrderRow>(
            "SELECT id, order_number, customer_id, order_date, subtotal, tax_amount, total, status, created_at, updated_at
             FROM sales_orders WHERE id = ?"
        ).bind(id.to_string()).fetch_optional(pool).await?
        .ok_or_else(|| Error::not_found("SalesOrder", &id.to_string()))?;
        
        let lines = sqlx::query_as::<_, SalesOrderLineRow>(
            "SELECT id, sales_order_id, product_id, description, quantity, unit_price, discount_percent, line_total
             FROM sales_order_lines WHERE sales_order_id = ?"
        ).bind(id.to_string()).fetch_all(pool).await?;
        
        Ok(SalesOrder {
            base: BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None, updated_by: None,
            },
            order_number: row.order_number,
            customer_id: Uuid::parse_str(&row.customer_id).unwrap_or_default(),
            order_date: chrono::DateTime::parse_from_rfc3339(&row.order_date).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            required_date: None,
            lines: lines.into_iter().map(|l| SalesOrderLine {
                id: Uuid::parse_str(&l.id).unwrap_or_default(),
                product_id: Uuid::parse_str(&l.product_id).unwrap_or_default(),
                description: l.description,
                quantity: l.quantity,
                unit_price: Money::new(l.unit_price, Currency::USD),
                discount_percent: l.discount_percent,
                tax_rate: 0.0,
                line_total: Money::new(l.line_total, Currency::USD),
            }).collect(),
            subtotal: Money::new(row.subtotal, Currency::USD),
            tax_amount: Money::new(row.tax_amount, Currency::USD),
            total: Money::new(row.total, Currency::USD),
            status: match row.status.as_str() { "Confirmed" => Status::Approved, "Completed" => Status::Completed, _ => Status::Draft },
        })
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<SalesOrder>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sales_orders").fetch_one(pool).await?;
        let rows = sqlx::query_as::<_, SalesOrderRow>(
            "SELECT id, order_number, customer_id, order_date, subtotal, tax_amount, total, status, created_at, updated_at
             FROM sales_orders ORDER BY order_date DESC LIMIT ? OFFSET ?"
        ).bind(pagination.limit() as i64).bind(pagination.offset() as i64).fetch_all(pool).await?;
        
        let mut orders = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id).unwrap_or_default();
            let lines = sqlx::query_as::<_, SalesOrderLineRow>(
                "SELECT id, sales_order_id, product_id, description, quantity, unit_price, discount_percent, line_total
                 FROM sales_order_lines WHERE sales_order_id = ?"
            ).bind(id.to_string()).fetch_all(pool).await?;
            
            orders.push(SalesOrder {
                base: BaseEntity {
                    id, created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                    created_by: None, updated_by: None,
                },
                order_number: row.order_number,
                customer_id: Uuid::parse_str(&row.customer_id).unwrap_or_default(),
                order_date: chrono::DateTime::parse_from_rfc3339(&row.order_date).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                required_date: None,
                lines: lines.into_iter().map(|l| SalesOrderLine {
                    id: Uuid::parse_str(&l.id).unwrap_or_default(),
                    product_id: Uuid::parse_str(&l.product_id).unwrap_or_default(),
                    description: l.description, quantity: l.quantity,
                    unit_price: Money::new(l.unit_price, Currency::USD),
                    discount_percent: l.discount_percent, tax_rate: 0.0,
                    line_total: Money::new(l.line_total, Currency::USD),
                }).collect(),
                subtotal: Money::new(row.subtotal, Currency::USD),
                tax_amount: Money::new(row.tax_amount, Currency::USD),
                total: Money::new(row.total, Currency::USD),
                status: match row.status.as_str() { "Confirmed" => Status::Approved, "Completed" => Status::Completed, _ => Status::Draft },
            });
        }
        Ok(Paginated::new(orders, count.0 as u64, pagination))
    }

    async fn create(&self, pool: &SqlitePool, order: SalesOrder) -> Result<SalesOrder> {
        let now = Utc::now();
        let mut tx = pool.begin().await?;
        
        sqlx::query(
            "INSERT INTO sales_orders (id, order_number, customer_id, order_date, subtotal, tax_amount, total, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        ).bind(order.base.id.to_string()).bind(&order.order_number).bind(order.customer_id.to_string())
        .bind(order.order_date.to_rfc3339()).bind(order.subtotal.amount).bind(order.tax_amount.amount)
        .bind(order.total.amount).bind(format!("{:?}", order.status))
        .bind(order.base.created_at.to_rfc3339()).bind(now.to_rfc3339())
        .execute(&mut *tx).await?;
        
        for line in &order.lines {
            sqlx::query(
                "INSERT INTO sales_order_lines (id, sales_order_id, product_id, description, quantity, unit_price, discount_percent, line_total)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
            ).bind(line.id.to_string()).bind(order.base.id.to_string()).bind(line.product_id.to_string())
            .bind(&line.description).bind(line.quantity).bind(line.unit_price.amount)
            .bind(line.discount_percent).bind(line.line_total.amount)
            .execute(&mut *tx).await?;
        }
        
        tx.commit().await?;
        Ok(order)
    }

    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: Status) -> Result<()> {
        let status_str = format!("{:?}", status);
        let rows = sqlx::query("UPDATE sales_orders SET status = ?, updated_at = ? WHERE id = ?")
            .bind(&status_str).bind(Utc::now().to_rfc3339()).bind(id.to_string())
            .execute(pool).await?;
        if rows.rows_affected() == 0 { return Err(Error::not_found("SalesOrder", &id.to_string())); }
        Ok(())
    }
}

#[async_trait]
pub trait CustomerRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Customer>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Customer>>;
    async fn create(&self, pool: &SqlitePool, customer: Customer) -> Result<Customer>;
    async fn update(&self, pool: &SqlitePool, customer: Customer) -> Result<Customer>;
}

#[async_trait]
pub trait SalesOrderRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<SalesOrder>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<SalesOrder>>;
    async fn create(&self, pool: &SqlitePool, order: SalesOrder) -> Result<SalesOrder>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: Status) -> Result<()>;
}

#[derive(sqlx::FromRow)]
struct QuotationRow {
    id: String,
    quote_number: String,
    customer_id: String,
    quote_date: String,
    valid_until: Option<String>,
    subtotal: i64,
    tax_amount: i64,
    total: i64,
    status: String,
    notes: Option<String>,
    created_at: String,
    updated_at: String,
}

#[derive(sqlx::FromRow)]
struct QuotationLineRow {
    id: String,
    quotation_id: String,
    product_id: String,
    description: Option<String>,
    quantity: i64,
    unit_price: i64,
    discount: i64,
    line_total: i64,
}

pub struct SqliteQuotationRepository;

#[async_trait]
impl QuotationRepository for SqliteQuotationRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<SalesQuote> {
        let row = sqlx::query_as::<_, QuotationRow>(
            "SELECT id, quote_number, customer_id, quote_date, valid_until, subtotal, tax_amount, total, status, notes, created_at, updated_at
             FROM quotations WHERE id = ?"
        ).bind(id.to_string()).fetch_optional(pool).await?
        .ok_or_else(|| Error::not_found("Quotation", &id.to_string()))?;
        
        let lines = sqlx::query_as::<_, QuotationLineRow>(
            "SELECT id, quotation_id, product_id, description, quantity, unit_price, discount, line_total
             FROM quotation_lines WHERE quotation_id = ?"
        ).bind(id.to_string()).fetch_all(pool).await?;
        
        Ok(SalesQuote {
            base: BaseEntity {
                id: Uuid::parse_str(&row.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None, updated_by: None,
            },
            quote_number: row.quote_number,
            customer_id: Uuid::parse_str(&row.customer_id).unwrap_or_default(),
            quote_date: chrono::DateTime::parse_from_rfc3339(&row.quote_date).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            valid_until: row.valid_until.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|| Utc::now() + chrono::Duration::days(30)),
            lines: lines.into_iter().map(|l| SalesQuoteLine {
                id: Uuid::parse_str(&l.id).unwrap_or_default(),
                product_id: Uuid::parse_str(&l.product_id).unwrap_or_default(),
                description: l.description.unwrap_or_default(),
                quantity: l.quantity,
                unit_price: Money::new(l.unit_price, Currency::USD),
                discount_percent: l.discount as f64 / 100.0,
                line_total: Money::new(l.line_total, Currency::USD),
            }).collect(),
            subtotal: Money::new(row.subtotal, Currency::USD),
            tax_amount: Money::new(row.tax_amount, Currency::USD),
            total: Money::new(row.total, Currency::USD),
            status: match row.status.as_str() { "Sent" => Status::Pending, "Accepted" => Status::Approved, "Rejected" => Status::Rejected, _ => Status::Draft },
        })
    }

    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<SalesQuote>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM quotations").fetch_one(pool).await?;
        let rows = sqlx::query_as::<_, QuotationRow>(
            "SELECT id, quote_number, customer_id, quote_date, valid_until, subtotal, tax_amount, total, status, notes, created_at, updated_at
             FROM quotations ORDER BY quote_date DESC LIMIT ? OFFSET ?"
        ).bind(pagination.limit() as i64).bind(pagination.offset() as i64).fetch_all(pool).await?;
        
        let mut quotes = Vec::new();
        for row in rows {
            let id = Uuid::parse_str(&row.id).unwrap_or_default();
            let lines = sqlx::query_as::<_, QuotationLineRow>(
                "SELECT id, quotation_id, product_id, description, quantity, unit_price, discount, line_total
                 FROM quotation_lines WHERE quotation_id = ?"
            ).bind(id.to_string()).fetch_all(pool).await?;
            
            quotes.push(SalesQuote {
                base: BaseEntity {
                    id, created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                    updated_at: chrono::DateTime::parse_from_rfc3339(&row.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                    created_by: None, updated_by: None,
                },
                quote_number: row.quote_number,
                customer_id: Uuid::parse_str(&row.customer_id).unwrap_or_default(),
                quote_date: chrono::DateTime::parse_from_rfc3339(&row.quote_date).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                valid_until: row.valid_until.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok()).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|| Utc::now() + chrono::Duration::days(30)),
                lines: lines.into_iter().map(|l| SalesQuoteLine {
                    id: Uuid::parse_str(&l.id).unwrap_or_default(),
                    product_id: Uuid::parse_str(&l.product_id).unwrap_or_default(),
                    description: l.description.unwrap_or_default(),
                    quantity: l.quantity,
                    unit_price: Money::new(l.unit_price, Currency::USD),
                    discount_percent: l.discount as f64 / 100.0,
                    line_total: Money::new(l.line_total, Currency::USD),
                }).collect(),
                subtotal: Money::new(row.subtotal, Currency::USD),
                tax_amount: Money::new(row.tax_amount, Currency::USD),
                total: Money::new(row.total, Currency::USD),
                status: match row.status.as_str() { "Sent" => Status::Pending, "Accepted" => Status::Approved, "Rejected" => Status::Rejected, _ => Status::Draft },
            });
        }
        Ok(Paginated::new(quotes, count.0 as u64, pagination))
    }

    async fn create(&self, pool: &SqlitePool, quote: SalesQuote) -> Result<SalesQuote> {
        let now = Utc::now();
        let mut tx = pool.begin().await?;
        
        sqlx::query(
            "INSERT INTO quotations (id, quote_number, customer_id, quote_date, valid_until, subtotal, tax_amount, total, status, notes, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        ).bind(quote.base.id.to_string()).bind(&quote.quote_number).bind(quote.customer_id.to_string())
        .bind(quote.quote_date.to_rfc3339()).bind(quote.valid_until.to_rfc3339())
        .bind(quote.subtotal.amount).bind(quote.tax_amount.amount).bind(quote.total.amount)
        .bind(format!("{:?}", quote.status)).bind(None::<String>)
        .bind(quote.base.created_at.to_rfc3339()).bind(now.to_rfc3339())
        .execute(&mut *tx).await?;
        
        for line in &quote.lines {
            sqlx::query(
                "INSERT INTO quotation_lines (id, quotation_id, product_id, description, quantity, unit_price, discount, line_total)
                 VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
            ).bind(line.id.to_string()).bind(quote.base.id.to_string()).bind(line.product_id.to_string())
            .bind(&line.description).bind(line.quantity).bind(line.unit_price.amount)
            .bind((line.discount_percent * 100.0) as i64).bind(line.line_total.amount)
            .execute(&mut *tx).await?;
        }
        
        tx.commit().await?;
        Ok(quote)
    }

    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: Status) -> Result<()> {
        let status_str = format!("{:?}", status);
        let rows = sqlx::query("UPDATE quotations SET status = ?, updated_at = ? WHERE id = ?")
            .bind(&status_str).bind(Utc::now().to_rfc3339()).bind(id.to_string())
            .execute(pool).await?;
        if rows.rows_affected() == 0 { return Err(Error::not_found("Quotation", &id.to_string())); }
        Ok(())
    }

    async fn convert_to_order(&self, pool: &SqlitePool, quote: SalesQuote) -> Result<SalesOrder> {
        let order = SalesOrder {
            base: BaseEntity::new(),
            order_number: format!("SO-{}", chrono::Local::now().format("%Y%m%d%H%M%S")),
            customer_id: quote.customer_id,
            order_date: Utc::now(),
            required_date: None,
            lines: quote.lines.into_iter().map(|l| SalesOrderLine {
                id: Uuid::new_v4(),
                product_id: l.product_id,
                description: l.description,
                quantity: l.quantity,
                unit_price: l.unit_price,
                discount_percent: l.discount_percent,
                tax_rate: 0.0,
                line_total: l.line_total,
            }).collect(),
            subtotal: quote.subtotal,
            tax_amount: quote.tax_amount,
            total: quote.total,
            status: Status::Draft,
        };
        
        let order_repo = SqliteSalesOrderRepository;
        let created = order_repo.create(pool, order).await?;
        
        self.update_status(pool, quote.base.id, Status::Completed).await?;
        
        Ok(created)
    }
}

#[async_trait]
pub trait QuotationRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<SalesQuote>;
    async fn find_all(&self, pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<SalesQuote>>;
    async fn create(&self, pool: &SqlitePool, quote: SalesQuote) -> Result<SalesQuote>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: Status) -> Result<()>;
    async fn convert_to_order(&self, pool: &SqlitePool, quote: SalesQuote) -> Result<SalesOrder>;
}
