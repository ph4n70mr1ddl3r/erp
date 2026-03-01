use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, BaseEntity, Pagination, Paginated};
use crate::models::*;
use crate::repository::*;

pub struct PortalUserService { repo: SqlitePortalUserRepository }
impl Default for PortalUserService {
    fn default() -> Self {
        Self::new()
    }
}

impl PortalUserService {
    pub fn new() -> Self { Self { repo: SqlitePortalUserRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<PortalUser> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn register(&self, pool: &SqlitePool, portal_type: PortalType, username: String, email: String, password: &str, external_id: Option<Uuid>) -> Result<PortalUser> {
        if username.is_empty() || email.is_empty() || password.is_empty() {
            return Err(Error::validation("Username, email, and password are required"));
        }
        
        if self.repo.find_by_email(pool, &email).await.is_ok() {
            return Err(Error::validation("Email already registered"));
        }
        
        if self.repo.find_by_username(pool, &username).await.is_ok() {
            return Err(Error::validation("Username already taken"));
        }
        
        let password_hash = Self::hash_password(password)?;
        
        let user = PortalUser {
            base: BaseEntity::new(),
            portal_type,
            external_id,
            username,
            email,
            password_hash,
            display_name: None,
            company_name: None,
            phone: None,
            access_level: PortalAccessLevel::Standard,
            permissions: None,
            preferences: None,
            language: "en".to_string(),
            timezone: "UTC".to_string(),
            avatar_url: None,
            last_login_at: None,
            login_count: 0,
            failed_login_count: 0,
            locked_until: None,
            password_changed_at: None,
            must_change_password: false,
            two_factor_enabled: false,
            two_factor_secret: None,
            api_key: None,
            api_key_expires_at: None,
            session_timeout_minutes: 30,
            status: PortalUserStatus::Active,
            invited_by: None,
            invited_at: None,
            accepted_at: Some(Utc::now()),
            notification_preferences: None,
        };
        
        self.repo.create(pool, user).await
    }
    
    pub async fn login(&self, pool: &SqlitePool, username: &str, password: &str) -> Result<(PortalUser, PortalSession)> {
        let user = match self.repo.find_by_username(pool, username).await {
            Ok(u) => u,
            Err(_) => return Err(Error::not_found("User", username)),
        };
        
        if user.status != PortalUserStatus::Active {
            return Err(Error::validation("Account is not active"));
        }
        
        if let Some(locked) = user.locked_until {
            if locked > Utc::now() {
                return Err(Error::validation("Account is temporarily locked"));
            }
        }
        
        if !Self::verify_password(password, &user.password_hash)? {
            return Err(Error::validation("Invalid credentials"));
        }
        
        self.repo.update_last_login(pool, user.base.id).await?;
        
        let session = self.create_session(pool, user.base.id, None, None).await?;
        
        Ok((user, session))
    }
    
    pub async fn create_session(&self, pool: &SqlitePool, user_id: Uuid, ip_address: Option<String>, user_agent: Option<String>) -> Result<PortalSession> {
        let now = Utc::now();
        let expires = now + chrono::Duration::hours(24);
        
        let session = PortalSession {
            id: Uuid::new_v4(),
            portal_user_id: user_id,
            session_token: Self::generate_token(),
            refresh_token: Self::generate_token(),
            ip_address,
            user_agent,
            device_type: None,
            login_at: now,
            last_activity_at: now,
            expires_at: expires,
            logout_at: None,
            logout_reason: None,
            status: SessionStatus::Active,
        };
        
        SqlitePortalSessionRepository.create(pool, session.clone()).await?;
        Ok(session)
    }
    
    pub async fn logout(&self, pool: &SqlitePool, session_id: Uuid) -> Result<()> {
        SqlitePortalSessionRepository.invalidate(pool, session_id).await
    }
    
    pub async fn validate_session(&self, pool: &SqlitePool, token: &str) -> Result<PortalSession> {
        SqlitePortalSessionRepository.find_by_token(pool, token).await
    }
    
    pub async fn update(&self, pool: &SqlitePool, mut user: PortalUser) -> Result<PortalUser> {
        user.base.updated_at = Utc::now();
        self.repo.update(pool, &user).await?;
        Ok(user)
    }
    
    fn hash_password(password: &str) -> Result<String> {
        use argon2::{
            password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
            Argon2,
        };
        
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|_| Error::internal("Failed to hash password"))
    }
    
    fn verify_password(password: &str, hash: &str) -> Result<bool> {
        use argon2::{
            password_hash::{PasswordHash, PasswordVerifier},
            Argon2,
        };
        
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|_| Error::internal("Invalid password hash"))?;
        
        let argon2 = Argon2::default();
        
        Ok(argon2
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok())
    }
    
    fn generate_token() -> String {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let token: String = (0..64).map(|_| rng.sample(rand::distributions::Alphanumeric) as char).collect();
        token
    }
}

pub struct PortalOrderService { repo: SqlitePortalOrderRepository }
impl Default for PortalOrderService {
    fn default() -> Self {
        Self::new()
    }
}

impl PortalOrderService {
    pub fn new() -> Self { Self { repo: SqlitePortalOrderRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<PortalOrder> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn list_by_user(&self, pool: &SqlitePool, user_id: Uuid, pagination: Pagination) -> Result<Paginated<PortalOrder>> {
        self.repo.find_by_user(pool, user_id, pagination).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, user_id: Uuid, customer_id: Uuid, lines: Vec<PortalOrderLine>, billing_address: Option<String>, shipping_address: Option<String>) -> Result<PortalOrder> {
        if lines.is_empty() {
            return Err(Error::validation("Order must have at least one line"));
        }
        
        let order_number = format!("PO-{}", Utc::now().format("%Y%m%d%H%M%S"));
        let subtotal: i64 = lines.iter().map(|l| l.line_total_cents).sum();
        let tax: i64 = lines.iter().map(|l| l.tax_cents).sum();
        
        let mut order = PortalOrder {
            base: BaseEntity::new(),
            portal_user_id: user_id,
            portal_order_number: order_number,
            erp_order_id: None,
            erp_order_number: None,
            customer_id,
            order_type: PortalOrderType::Standard,
            status: PortalOrderStatus::Draft,
            billing_address,
            shipping_address,
            requested_delivery_date: None,
            shipping_method: None,
            payment_method: None,
            notes: None,
            internal_notes: None,
            subtotal_cents: subtotal,
            tax_cents: tax,
            shipping_cents: 0,
            discount_cents: 0,
            total_cents: subtotal + tax,
            currency: "USD".to_string(),
            submitted_at: None,
            confirmed_at: None,
            lines: vec![],
        };
        
        for mut line in lines {
            line.id = Uuid::new_v4();
            line.portal_order_id = order.base.id;
            order.lines.push(line);
        }
        
        self.repo.create(pool, order).await
    }
    
    pub async fn submit(&self, pool: &SqlitePool, id: Uuid) -> Result<PortalOrder> {
        let order = self.repo.find_by_id(pool, id).await?;
        
        if order.status != PortalOrderStatus::Draft {
            return Err(Error::validation("Only draft orders can be submitted"));
        }
        
        self.repo.update_status(pool, id, PortalOrderStatus::Submitted).await?;
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn cancel(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let order = self.repo.find_by_id(pool, id).await?;
        
        if order.status == PortalOrderStatus::Shipped || order.status == PortalOrderStatus::Delivered {
            return Err(Error::validation("Cannot cancel shipped or delivered orders"));
        }
        
        self.repo.update_status(pool, id, PortalOrderStatus::Cancelled).await
    }
}

pub struct PortalInvoiceService;
impl Default for PortalInvoiceService {
    fn default() -> Self {
        Self::new()
    }
}

impl PortalInvoiceService {
    pub fn new() -> Self { Self }
    
    pub async fn list_by_customer(&self, pool: &SqlitePool, customer_id: Uuid, _pagination: Pagination) -> Result<Paginated<PortalInvoice>> {
        let _ = (pool, customer_id);
        Ok(Paginated::new(vec![], 0, Pagination::default()))
    }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<PortalInvoice> {
        let _ = (pool, id);
        Err(Error::not_found("PortalInvoice", &id.to_string()))
    }
}

pub struct PortalPaymentService;
impl Default for PortalPaymentService {
    fn default() -> Self {
        Self::new()
    }
}

impl PortalPaymentService {
    pub fn new() -> Self { Self }
    
    pub async fn process_payment(&self, pool: &SqlitePool, user_id: Uuid, invoice_ids: Vec<Uuid>, amount_cents: i64, method: PaymentMethodType) -> Result<PortalPayment> {
        let _ = pool;
        Ok(PortalPayment {
            base: BaseEntity::new(),
            portal_user_id: user_id,
            payment_reference: format!("PAY-{}", Utc::now().format("%Y%m%d%H%M%S")),
            erp_payment_id: None,
            customer_id: Uuid::nil(),
            invoice_ids: serde_json::to_string(&invoice_ids).unwrap_or_default(),
            payment_method: method,
            amount_cents,
            currency: "USD".to_string(),
            status: PaymentStatus::Completed,
            payment_provider: Some("Stripe".to_string()),
            provider_transaction_id: Some(format!("tx_{}", Uuid::new_v4())),
            provider_response: None,
            card_last_four: None,
            card_brand: None,
            bank_name: None,
            check_number: None,
            processed_at: Some(Utc::now()),
            failed_at: None,
            failure_reason: None,
            refunded_at: None,
            refund_amount_cents: None,
            notes: None,
        })
    }
}

pub struct PortalNotificationService;
impl Default for PortalNotificationService {
    fn default() -> Self {
        Self::new()
    }
}

impl PortalNotificationService {
    pub fn new() -> Self { Self }
    
    pub async fn create(&self, pool: &SqlitePool, user_id: Uuid, notification_type: PortalNotificationType, title: String, message: String, action_url: Option<String>) -> Result<PortalNotification> {
        let _ = pool;
        Ok(PortalNotification {
            id: Uuid::new_v4(),
            portal_user_id: user_id,
            notification_type,
            title,
            message,
            entity_type: None,
            entity_id: None,
            action_url,
            priority: 5,
            read_at: None,
            emailed_at: None,
            created_at: Utc::now(),
        })
    }
    
    pub async fn mark_read(&self, pool: &SqlitePool, _id: Uuid) -> Result<()> {
        let _ = pool;
        Ok(())
    }
}

pub struct SupplierPortalService;
impl Default for SupplierPortalService {
    fn default() -> Self {
        Self::new()
    }
}

impl SupplierPortalService {
    pub fn new() -> Self { Self }
    
    pub async fn submit_quote(&self, pool: &SqlitePool, user_id: Uuid, vendor_id: Uuid, rfq_id: Uuid, lines: Vec<SupplierQuoteLine>) -> Result<SupplierQuoteSubmission> {
        let _ = pool;
        let subtotal: i64 = lines.iter().map(|l| l.line_total_cents).sum();
        
        Ok(SupplierQuoteSubmission {
            base: BaseEntity::new(),
            portal_user_id: user_id,
            vendor_id,
            rfq_id,
            quote_number: format!("SQ-{}", Utc::now().format("%Y%m%d%H%M%S")),
            erp_quote_id: None,
            status: SupplierQuoteStatus::Submitted,
            valid_until: Some(Utc::now() + chrono::Duration::days(30)),
            delivery_lead_time_days: 14,
            payment_terms: Some("Net 30".to_string()),
            incoterms: Some("FOB".to_string()),
            notes: None,
            internal_notes: None,
            subtotal_cents: subtotal,
            tax_cents: 0,
            total_cents: subtotal,
            currency: "USD".to_string(),
            submitted_at: Some(Utc::now()),
            lines,
        })
    }
}
