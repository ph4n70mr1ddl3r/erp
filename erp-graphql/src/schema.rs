#![allow(clippy::type_complexity)]
use async_graphql::{Object, Context, SimpleObject, InputObject, Enum};
use sqlx::SqlitePool;

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(InputObject)]
pub struct PaginationInput {
    pub page: i32,
    pub per_page: i32,
}

#[derive(SimpleObject)]
pub struct PageInfo {
    pub page: i32,
    pub per_page: i32,
    pub total_pages: i32,
    pub total_items: i64,
    pub has_next: bool,
    pub has_prev: bool,
}

#[derive(SimpleObject)]
pub struct Product {
    pub id: String,
    pub sku: String,
    pub name: String,
    pub description: Option<String>,
    pub unit_of_measure: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(SimpleObject)]
pub struct ProductConnection {
    pub items: Vec<Product>,
    pub page_info: PageInfo,
}

#[derive(SimpleObject)]
pub struct Customer {
    pub id: String,
    pub code: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: String,
    pub credit_limit: Option<i64>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(SimpleObject)]
pub struct CustomerConnection {
    pub items: Vec<Customer>,
    pub page_info: PageInfo,
}

#[derive(SimpleObject)]
pub struct Order {
    pub id: String,
    pub order_number: String,
    pub customer_id: String,
    pub status: String,
    pub total_amount: i64,
    pub currency: String,
    pub order_date: String,
    pub created_at: String,
}

#[derive(SimpleObject)]
pub struct OrderConnection {
    pub items: Vec<Order>,
    pub page_info: PageInfo,
}

#[derive(SimpleObject)]
pub struct Vendor {
    pub id: String,
    pub code: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub status: String,
    pub payment_terms: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(SimpleObject)]
pub struct VendorConnection {
    pub items: Vec<Vendor>,
    pub page_info: PageInfo,
}

#[derive(SimpleObject)]
pub struct Employee {
    pub id: String,
    pub employee_number: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub department: Option<String>,
    pub position: Option<String>,
    pub status: String,
    pub hire_date: Option<String>,
    pub created_at: String,
}

#[derive(SimpleObject)]
pub struct EmployeeConnection {
    pub items: Vec<Employee>,
    pub page_info: PageInfo,
}

#[derive(SimpleObject)]
pub struct DashboardStats {
    pub total_products: i64,
    pub total_customers: i64,
    pub total_orders: i64,
    pub total_vendors: i64,
    pub total_employees: i64,
    pub pending_orders: i64,
    pub revenue_this_month: i64,
    pub orders_this_month: i64,
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn products(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
    ) -> async_graphql::Result<ProductConnection> {
        let pool = ctx.data::<SqlitePool>()?;
        let page = pagination.as_ref().map(|p| p.page).unwrap_or(1);
        let per_page = pagination.as_ref().map(|p| p.per_page).unwrap_or(20);
        let offset = (page - 1) * per_page;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products")
            .fetch_one(pool)
            .await?;

        let rows: Vec<(String, String, String, Option<String>, String, String, String, String)> = 
            sqlx::query_as(
                "SELECT id, sku, name, description, unit_of_measure, status, created_at, updated_at 
                 FROM products ORDER BY created_at DESC LIMIT ? OFFSET ?"
            )
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        let items = rows.into_iter().map(|row| Product {
            id: row.0,
            sku: row.1,
            name: row.2,
            description: row.3,
            unit_of_measure: row.4,
            status: row.5,
            created_at: row.6,
            updated_at: row.7,
        }).collect();

        let total_pages = ((count.0 as f64) / (per_page as f64)).ceil() as i32;

        Ok(ProductConnection {
            items,
            page_info: PageInfo {
                page,
                per_page,
                total_pages,
                total_items: count.0,
                has_next: page < total_pages,
                has_prev: page > 1,
            },
        })
    }

    async fn product(&self, ctx: &Context<'_>, id: String) -> async_graphql::Result<Option<Product>> {
        let pool = ctx.data::<SqlitePool>()?;

        let row: Option<(String, String, String, Option<String>, String, String, String, String)> = 
            sqlx::query_as(
                "SELECT id, sku, name, description, unit_of_measure, status, created_at, updated_at 
                 FROM products WHERE id = ?"
            )
            .bind(&id)
            .fetch_optional(pool)
            .await?;

        Ok(row.map(|row| Product {
            id: row.0,
            sku: row.1,
            name: row.2,
            description: row.3,
            unit_of_measure: row.4,
            status: row.5,
            created_at: row.6,
            updated_at: row.7,
        }))
    }

    async fn customers(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
    ) -> async_graphql::Result<CustomerConnection> {
        let pool = ctx.data::<SqlitePool>()?;
        let page = pagination.as_ref().map(|p| p.page).unwrap_or(1);
        let per_page = pagination.as_ref().map(|p| p.per_page).unwrap_or(20);
        let offset = (page - 1) * per_page;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM customers")
            .fetch_one(pool)
            .await?;

        let rows: Vec<(String, String, String, Option<String>, Option<String>, String, Option<i64>, String, String)> = 
            sqlx::query_as(
                "SELECT id, code, name, email, phone, status, credit_limit, created_at, updated_at 
                 FROM customers ORDER BY created_at DESC LIMIT ? OFFSET ?"
            )
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        let items = rows.into_iter().map(|row| Customer {
            id: row.0,
            code: row.1,
            name: row.2,
            email: row.3,
            phone: row.4,
            status: row.5,
            credit_limit: row.6,
            created_at: row.7,
            updated_at: row.8,
        }).collect();

        let total_pages = ((count.0 as f64) / (per_page as f64)).ceil() as i32;

        Ok(CustomerConnection {
            items,
            page_info: PageInfo {
                page,
                per_page,
                total_pages,
                total_items: count.0,
                has_next: page < total_pages,
                has_prev: page > 1,
            },
        })
    }

    async fn orders(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
    ) -> async_graphql::Result<OrderConnection> {
        let pool = ctx.data::<SqlitePool>()?;
        let page = pagination.as_ref().map(|p| p.page).unwrap_or(1);
        let per_page = pagination.as_ref().map(|p| p.per_page).unwrap_or(20);
        let offset = (page - 1) * per_page;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sales_orders")
            .fetch_one(pool)
            .await?;

        let rows: Vec<(String, String, String, String, i64, String, String, String)> = 
            sqlx::query_as(
                "SELECT id, order_number, customer_id, status, total_amount, currency, order_date, created_at 
                 FROM sales_orders ORDER BY created_at DESC LIMIT ? OFFSET ?"
            )
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        let items = rows.into_iter().map(|row| Order {
            id: row.0,
            order_number: row.1,
            customer_id: row.2,
            status: row.3,
            total_amount: row.4,
            currency: row.5,
            order_date: row.6,
            created_at: row.7,
        }).collect();

        let total_pages = ((count.0 as f64) / (per_page as f64)).ceil() as i32;

        Ok(OrderConnection {
            items,
            page_info: PageInfo {
                page,
                per_page,
                total_pages,
                total_items: count.0,
                has_next: page < total_pages,
                has_prev: page > 1,
            },
        })
    }

    async fn vendors(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
    ) -> async_graphql::Result<VendorConnection> {
        let pool = ctx.data::<SqlitePool>()?;
        let page = pagination.as_ref().map(|p| p.page).unwrap_or(1);
        let per_page = pagination.as_ref().map(|p| p.per_page).unwrap_or(20);
        let offset = (page - 1) * per_page;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM vendors")
            .fetch_one(pool)
            .await?;

        let rows: Vec<(String, String, String, Option<String>, Option<String>, String, Option<String>, String, String)> = 
            sqlx::query_as(
                "SELECT id, code, name, email, phone, status, payment_terms, created_at, updated_at 
                 FROM vendors ORDER BY created_at DESC LIMIT ? OFFSET ?"
            )
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        let items = rows.into_iter().map(|row| Vendor {
            id: row.0,
            code: row.1,
            name: row.2,
            email: row.3,
            phone: row.4,
            status: row.5,
            payment_terms: row.6,
            created_at: row.7,
            updated_at: row.8,
        }).collect();

        let total_pages = ((count.0 as f64) / (per_page as f64)).ceil() as i32;

        Ok(VendorConnection {
            items,
            page_info: PageInfo {
                page,
                per_page,
                total_pages,
                total_items: count.0,
                has_next: page < total_pages,
                has_prev: page > 1,
            },
        })
    }

    async fn employees(
        &self,
        ctx: &Context<'_>,
        pagination: Option<PaginationInput>,
    ) -> async_graphql::Result<EmployeeConnection> {
        let pool = ctx.data::<SqlitePool>()?;
        let page = pagination.as_ref().map(|p| p.page).unwrap_or(1);
        let per_page = pagination.as_ref().map(|p| p.per_page).unwrap_or(20);
        let offset = (page - 1) * per_page;

        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM employees")
            .fetch_one(pool)
            .await?;

        let rows: Vec<(String, String, String, String, String, Option<String>, Option<String>, String, Option<String>, String)> = 
            sqlx::query_as(
                "SELECT id, employee_number, first_name, last_name, email, department, position, status, hire_date, created_at 
                 FROM employees ORDER BY created_at DESC LIMIT ? OFFSET ?"
            )
            .bind(per_page)
            .bind(offset)
            .fetch_all(pool)
            .await?;

        let items = rows.into_iter().map(|row| Employee {
            id: row.0,
            employee_number: row.1,
            first_name: row.2,
            last_name: row.3,
            email: row.4,
            department: row.5,
            position: row.6,
            status: row.7,
            hire_date: row.8,
            created_at: row.9,
        }).collect();

        let total_pages = ((count.0 as f64) / (per_page as f64)).ceil() as i32;

        Ok(EmployeeConnection {
            items,
            page_info: PageInfo {
                page,
                per_page,
                total_pages,
                total_items: count.0,
                has_next: page < total_pages,
                has_prev: page > 1,
            },
        })
    }

    async fn dashboard_stats(&self, ctx: &Context<'_>) -> async_graphql::Result<DashboardStats> {
        let pool = ctx.data::<SqlitePool>()?;

        let products: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM products").fetch_one(pool).await?;
        let customers: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM customers").fetch_one(pool).await?;
        let orders: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sales_orders").fetch_one(pool).await?;
        let vendors: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM vendors").fetch_one(pool).await?;
        let employees: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM employees").fetch_one(pool).await?;
        let pending: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM sales_orders WHERE status = 'pending'").fetch_one(pool).await?;

        let month_start = chrono::Utc::now().format("%Y-%m-01").to_string();
        let revenue: (i64,) = sqlx::query_as(
            "SELECT COALESCE(SUM(total_amount), 0) FROM sales_orders WHERE status = 'confirmed' AND order_date >= ?"
        )
        .bind(&month_start)
        .fetch_one(pool)
        .await?;

        let month_orders: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM sales_orders WHERE order_date >= ?"
        )
        .bind(&month_start)
        .fetch_one(pool)
        .await?;

        Ok(DashboardStats {
            total_products: products.0,
            total_customers: customers.0,
            total_orders: orders.0,
            total_vendors: vendors.0,
            total_employees: employees.0,
            pending_orders: pending.0,
            revenue_this_month: revenue.0,
            orders_this_month: month_orders.0,
        })
    }
}
