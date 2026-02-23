use async_trait::async_trait;
use sqlx::SqlitePool;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity};
use crate::models::*;
use uuid::Uuid;
use chrono::Utc;

#[async_trait]
pub trait PortalUserRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<PortalUser>;
    async fn find_by_email(&self, pool: &SqlitePool, email: &str) -> Result<PortalUser>;
    async fn find_by_username(&self, pool: &SqlitePool, username: &str) -> Result<PortalUser>;
    async fn create(&self, pool: &SqlitePool, user: PortalUser) -> Result<PortalUser>;
    async fn update(&self, pool: &SqlitePool, user: &PortalUser) -> Result<()>;
    async fn update_last_login(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqlitePortalUserRepository;

#[async_trait]
impl PortalUserRepository for SqlitePortalUserRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<PortalUser> {
        let row = sqlx::query_as::<_, PortalUserRow>(
            "SELECT * FROM portal_users WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("PortalUser", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn find_by_email(&self, pool: &SqlitePool, email: &str) -> Result<PortalUser> {
        let row = sqlx::query_as::<_, PortalUserRow>(
            "SELECT * FROM portal_users WHERE email = ?"
        )
        .bind(email)
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("PortalUser", email))?;
        
        Ok(row.into())
    }
    
    async fn find_by_username(&self, pool: &SqlitePool, username: &str) -> Result<PortalUser> {
        let row = sqlx::query_as::<_, PortalUserRow>(
            "SELECT * FROM portal_users WHERE username = ?"
        )
        .bind(username)
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("PortalUser", username))?;
        
        Ok(row.into())
    }
    
    async fn create(&self, pool: &SqlitePool, user: PortalUser) -> Result<PortalUser> {
        sqlx::query(
            r#"INSERT INTO portal_users (id, portal_type, external_id, username, email, password_hash,
               display_name, company_name, phone, access_level, permissions, preferences, language,
               timezone, status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(user.base.id.to_string())
        .bind(format!("{:?}", user.portal_type))
        .bind(user.external_id.map(|id| id.to_string()))
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.display_name)
        .bind(&user.company_name)
        .bind(&user.phone)
        .bind(format!("{:?}", user.access_level))
        .bind(&user.permissions)
        .bind(&user.preferences)
        .bind(&user.language)
        .bind(&user.timezone)
        .bind(format!("{:?}", user.status))
        .bind(user.base.created_at.to_rfc3339())
        .bind(user.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(user)
    }
    
    async fn update(&self, pool: &SqlitePool, user: &PortalUser) -> Result<()> {
        sqlx::query(
            r#"UPDATE portal_users SET display_name = ?, company_name = ?, phone = ?, 
               preferences = ?, updated_at = ? WHERE id = ?"#
        )
        .bind(&user.display_name)
        .bind(&user.company_name)
        .bind(&user.phone)
        .bind(&user.preferences)
        .bind(user.base.updated_at.to_rfc3339())
        .bind(user.base.id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        Ok(())
    }
    
    async fn update_last_login(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE portal_users SET last_login_at = ?, login_count = login_count + 1, failed_login_count = 0 WHERE id = ?"
        )
        .bind(Utc::now().to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct PortalUserRow {
    id: String,
    portal_type: String,
    external_id: Option<String>,
    username: String,
    email: String,
    password_hash: String,
    display_name: Option<String>,
    company_name: Option<String>,
    phone: Option<String>,
    access_level: String,
    permissions: Option<String>,
    preferences: Option<String>,
    language: String,
    timezone: String,
    last_login_at: Option<String>,
    login_count: i32,
    failed_login_count: i32,
    status: String,
    created_at: String,
    updated_at: String,
}

impl From<PortalUserRow> for PortalUser {
    fn from(r: PortalUserRow) -> Self {
        use chrono::{DateTime, Utc};
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            portal_type: match r.portal_type.as_str() {
                "Supplier" => PortalType::Supplier,
                "Partner" => PortalType::Partner,
                "Employee" => PortalType::Employee,
                _ => PortalType::Customer,
            },
            external_id: r.external_id.and_then(|id| Uuid::parse_str(&id).ok()),
            username: r.username,
            email: r.email,
            password_hash: r.password_hash,
            display_name: r.display_name,
            company_name: r.company_name,
            phone: r.phone,
            access_level: match r.access_level.as_str() {
                "Standard" => PortalAccessLevel::Standard,
                "Premium" => PortalAccessLevel::Premium,
                "Admin" => PortalAccessLevel::Admin,
                _ => PortalAccessLevel::ReadOnly,
            },
            permissions: r.permissions,
            preferences: r.preferences,
            language: r.language,
            timezone: r.timezone,
            avatar_url: None,
            last_login_at: r.last_login_at.and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            login_count: r.login_count,
            failed_login_count: r.failed_login_count,
            locked_until: None,
            password_changed_at: None,
            must_change_password: false,
            two_factor_enabled: false,
            two_factor_secret: None,
            api_key: None,
            api_key_expires_at: None,
            session_timeout_minutes: 30,
            status: match r.status.as_str() {
                "Pending" => PortalUserStatus::Pending,
                "Suspended" => PortalUserStatus::Suspended,
                "Locked" => PortalUserStatus::Locked,
                "Deleted" => PortalUserStatus::Deleted,
                _ => PortalUserStatus::Active,
            },
            invited_by: None,
            invited_at: None,
            accepted_at: None,
            notification_preferences: None,
        }
    }
}

#[async_trait]
pub trait PortalOrderRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<PortalOrder>;
    async fn find_by_user(&self, pool: &SqlitePool, user_id: Uuid, pagination: Pagination) -> Result<Paginated<PortalOrder>>;
    async fn create(&self, pool: &SqlitePool, order: PortalOrder) -> Result<PortalOrder>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: PortalOrderStatus) -> Result<()>;
}

pub struct SqlitePortalOrderRepository;

#[async_trait]
impl PortalOrderRepository for SqlitePortalOrderRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<PortalOrder> {
        let row = sqlx::query_as::<_, PortalOrderRow>(
            "SELECT * FROM portal_orders WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::not_found("PortalOrder", &id.to_string()))?;
        
        let lines = self.get_lines(pool, id).await?;
        Ok(row.into_order(lines))
    }
    
    async fn find_by_user(&self, pool: &SqlitePool, user_id: Uuid, pagination: Pagination) -> Result<Paginated<PortalOrder>> {
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM portal_orders WHERE portal_user_id = ?"
        )
        .bind(user_id.to_string())
        .fetch_one(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let rows = sqlx::query_as::<_, PortalOrderRow>(
            "SELECT * FROM portal_orders WHERE portal_user_id = ? ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(user_id.to_string())
        .bind(pagination.limit())
        .bind(pagination.offset())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        let orders: Vec<PortalOrder> = futures::future::try_join_all(
            rows.into_iter().map(|r| async {
                let lines = self.get_lines(pool, Uuid::parse_str(&r.id).unwrap_or_default()).await?;
                Ok::<_, Error>(r.into_order(lines))
            })
        ).await?;
        
        Ok(Paginated::new(orders, count as u64, pagination))
    }
    
    async fn create(&self, pool: &SqlitePool, order: PortalOrder) -> Result<PortalOrder> {
        sqlx::query(
            r#"INSERT INTO portal_orders (id, portal_user_id, portal_order_number, erp_order_id,
               erp_order_number, customer_id, order_type, status, billing_address, shipping_address,
               requested_delivery_date, shipping_method, payment_method, notes, subtotal_cents,
               tax_cents, shipping_cents, discount_cents, total_cents, currency, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(order.base.id.to_string())
        .bind(order.portal_user_id.to_string())
        .bind(&order.portal_order_number)
        .bind(order.erp_order_id.map(|id| id.to_string()))
        .bind(&order.erp_order_number)
        .bind(order.customer_id.to_string())
        .bind(format!("{:?}", order.order_type))
        .bind(format!("{:?}", order.status))
        .bind(&order.billing_address)
        .bind(&order.shipping_address)
        .bind(order.requested_delivery_date.map(|d| d.to_rfc3339()))
        .bind(&order.shipping_method)
        .bind(&order.payment_method)
        .bind(&order.notes)
        .bind(order.subtotal_cents)
        .bind(order.tax_cents)
        .bind(order.shipping_cents)
        .bind(order.discount_cents)
        .bind(order.total_cents)
        .bind(&order.currency)
        .bind(order.base.created_at.to_rfc3339())
        .bind(order.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        for line in &order.lines {
            self.create_line(pool, line).await?;
        }
        
        Ok(order)
    }
    
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: PortalOrderStatus) -> Result<()> {
        let now = Utc::now();
        let submitted_at = matches!(status, PortalOrderStatus::Submitted).then_some(now);
        
        sqlx::query(
            "UPDATE portal_orders SET status = ?, submitted_at = COALESCE(?, submitted_at), updated_at = ? WHERE id = ?"
        )
        .bind(format!("{:?}", status))
        .bind(submitted_at.map(|d| d.to_rfc3339()))
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        Ok(())
    }
}

impl SqlitePortalOrderRepository {
    async fn get_lines(&self, pool: &SqlitePool, order_id: Uuid) -> Result<Vec<PortalOrderLine>> {
        let rows = sqlx::query_as::<_, PortalOrderLineRow>(
            "SELECT * FROM portal_order_lines WHERE portal_order_id = ? ORDER BY line_number"
        )
        .bind(order_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    async fn create_line(&self, pool: &SqlitePool, line: &PortalOrderLine) -> Result<()> {
        sqlx::query(
            r#"INSERT INTO portal_order_lines (id, portal_order_id, line_number, product_id,
               product_code, product_name, description, quantity, unit_of_measure, unit_price_cents,
               discount_percent, discount_cents, tax_percent, tax_cents, line_total_cents, notes)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(line.id.to_string())
        .bind(line.portal_order_id.to_string())
        .bind(line.line_number)
        .bind(line.product_id.to_string())
        .bind(&line.product_code)
        .bind(&line.product_name)
        .bind(&line.description)
        .bind(line.quantity)
        .bind(&line.unit_of_measure)
        .bind(line.unit_price_cents)
        .bind(line.discount_percent)
        .bind(line.discount_cents)
        .bind(line.tax_percent)
        .bind(line.tax_cents)
        .bind(line.line_total_cents)
        .bind(&line.notes)
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct PortalOrderRow {
    id: String,
    portal_user_id: String,
    portal_order_number: String,
    erp_order_id: Option<String>,
    erp_order_number: Option<String>,
    customer_id: String,
    order_type: String,
    status: String,
    billing_address: Option<String>,
    shipping_address: Option<String>,
    requested_delivery_date: Option<String>,
    shipping_method: Option<String>,
    payment_method: Option<String>,
    notes: Option<String>,
    subtotal_cents: i64,
    tax_cents: i64,
    shipping_cents: i64,
    discount_cents: i64,
    total_cents: i64,
    currency: String,
    submitted_at: Option<String>,
    created_at: String,
    updated_at: String,
}

impl PortalOrderRow {
    fn into_order(self, lines: Vec<PortalOrderLine>) -> PortalOrder {
        use chrono::{DateTime, Utc};
        PortalOrder {
            base: BaseEntity {
                id: Uuid::parse_str(&self.id).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&self.created_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&self.updated_at)
                    .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            portal_user_id: Uuid::parse_str(&self.portal_user_id).unwrap_or_default(),
            portal_order_number: self.portal_order_number,
            erp_order_id: self.erp_order_id.and_then(|id| Uuid::parse_str(&id).ok()),
            erp_order_number: self.erp_order_number,
            customer_id: Uuid::parse_str(&self.customer_id).unwrap_or_default(),
            order_type: match self.order_type.as_str() {
                "Rush" => PortalOrderType::Rush,
                "Replenishment" => PortalOrderType::Replenishment,
                "Sample" => PortalOrderType::Sample,
                "Consignment" => PortalOrderType::Consignment,
                _ => PortalOrderType::Standard,
            },
            status: match self.status.as_str() {
                "PendingApproval" => PortalOrderStatus::PendingApproval,
                "Submitted" => PortalOrderStatus::Submitted,
                "Confirmed" => PortalOrderStatus::Confirmed,
                "Processing" => PortalOrderStatus::Processing,
                "PartiallyShipped" => PortalOrderStatus::PartiallyShipped,
                "Shipped" => PortalOrderStatus::Shipped,
                "Delivered" => PortalOrderStatus::Delivered,
                "Cancelled" => PortalOrderStatus::Cancelled,
                "OnHold" => PortalOrderStatus::OnHold,
                _ => PortalOrderStatus::Draft,
            },
            billing_address: self.billing_address,
            shipping_address: self.shipping_address,
            requested_delivery_date: self.requested_delivery_date.and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            shipping_method: self.shipping_method,
            payment_method: self.payment_method,
            notes: self.notes,
            internal_notes: None,
            subtotal_cents: self.subtotal_cents,
            tax_cents: self.tax_cents,
            shipping_cents: self.shipping_cents,
            discount_cents: self.discount_cents,
            total_cents: self.total_cents,
            currency: self.currency,
            submitted_at: self.submitted_at.and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            confirmed_at: None,
            lines,
        }
    }
}

#[derive(sqlx::FromRow)]
struct PortalOrderLineRow {
    id: String,
    portal_order_id: String,
    line_number: i32,
    product_id: String,
    product_code: String,
    product_name: String,
    description: Option<String>,
    quantity: i64,
    unit_of_measure: String,
    unit_price_cents: i64,
    discount_percent: f64,
    discount_cents: i64,
    tax_percent: f64,
    tax_cents: i64,
    line_total_cents: i64,
    notes: Option<String>,
}

impl From<PortalOrderLineRow> for PortalOrderLine {
    fn from(r: PortalOrderLineRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            portal_order_id: Uuid::parse_str(&r.portal_order_id).unwrap_or_default(),
            line_number: r.line_number,
            product_id: Uuid::parse_str(&r.product_id).unwrap_or_default(),
            product_code: r.product_code,
            product_name: r.product_name,
            description: r.description,
            quantity: r.quantity,
            unit_of_measure: r.unit_of_measure,
            unit_price_cents: r.unit_price_cents,
            discount_percent: r.discount_percent,
            discount_cents: r.discount_cents,
            tax_percent: r.tax_percent,
            tax_cents: r.tax_cents,
            line_total_cents: r.line_total_cents,
            notes: r.notes,
            erp_line_id: None,
        }
    }
}

#[async_trait]
pub trait PortalSessionRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, session: PortalSession) -> Result<PortalSession>;
    async fn find_by_token(&self, pool: &SqlitePool, token: &str) -> Result<PortalSession>;
    async fn invalidate(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqlitePortalSessionRepository;

#[async_trait]
impl PortalSessionRepository for SqlitePortalSessionRepository {
    async fn create(&self, pool: &SqlitePool, session: PortalSession) -> Result<PortalSession> {
        sqlx::query(
            r#"INSERT INTO portal_sessions (id, portal_user_id, session_token, refresh_token,
               ip_address, user_agent, login_at, last_activity_at, expires_at, status)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(session.id.to_string())
        .bind(session.portal_user_id.to_string())
        .bind(&session.session_token)
        .bind(&session.refresh_token)
        .bind(&session.ip_address)
        .bind(&session.user_agent)
        .bind(session.login_at.to_rfc3339())
        .bind(session.last_activity_at.to_rfc3339())
        .bind(session.expires_at.to_rfc3339())
        .bind(format!("{:?}", session.status))
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        
        Ok(session)
    }
    
    async fn find_by_token(&self, pool: &SqlitePool, token: &str) -> Result<PortalSession> {
        let row = sqlx::query_as::<_, PortalSessionRow>(
            "SELECT * FROM portal_sessions WHERE session_token = ? AND status = 'Active' AND expires_at > ?"
        )
        .bind(token)
        .bind(Utc::now().to_rfc3339())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e))?
        .ok_or_else(|| Error::unauthorized("Invalid or expired session"))?;
        
        Ok(row.into())
    }
    
    async fn invalidate(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE portal_sessions SET status = 'LoggedOut', logout_at = ? WHERE id = ?"
        )
        .bind(Utc::now().to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct PortalSessionRow {
    id: String,
    portal_user_id: String,
    session_token: String,
    refresh_token: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
    login_at: String,
    last_activity_at: String,
    expires_at: String,
    logout_at: Option<String>,
    status: String,
}

impl From<PortalSessionRow> for PortalSession {
    fn from(r: PortalSessionRow) -> Self {
        use chrono::{DateTime, Utc};
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            portal_user_id: Uuid::parse_str(&r.portal_user_id).unwrap_or_default(),
            session_token: r.session_token,
            refresh_token: r.refresh_token,
            ip_address: r.ip_address,
            user_agent: r.user_agent,
            device_type: None,
            login_at: DateTime::parse_from_rfc3339(&r.login_at)
                .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            last_activity_at: DateTime::parse_from_rfc3339(&r.last_activity_at)
                .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            expires_at: DateTime::parse_from_rfc3339(&r.expires_at)
                .map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            logout_at: r.logout_at.and_then(|d| DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            logout_reason: None,
            status: match r.status.as_str() {
                "Expired" => SessionStatus::Expired,
                "LoggedOut" => SessionStatus::LoggedOut,
                "Revoked" => SessionStatus::Revoked,
                _ => SessionStatus::Active,
            },
        }
    }
}
