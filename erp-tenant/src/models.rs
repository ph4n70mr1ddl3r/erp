use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tenant {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub plan: TenantPlan,
    pub status: TenantStatus,
    pub settings: serde_json::Value,
    pub branding: Option<TenantBranding>,
    pub limits: TenantLimits,
    pub trial_ends_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TenantPlan {
    Free,
    Starter,
    Professional,
    Enterprise,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TenantStatus {
    Trial,
    Active,
    Suspended,
    Cancelled,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantBranding {
    pub logo_url: Option<String>,
    pub primary_color: Option<String>,
    pub secondary_color: Option<String>,
    pub custom_domain: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantLimits {
    pub max_users: i32,
    pub max_products: i32,
    pub max_orders_per_month: i32,
    pub storage_mb: i32,
    pub api_calls_per_day: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantUser {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub role: TenantRole,
    pub permissions: Vec<String>,
    pub is_primary: bool,
    pub invited_by: Option<Uuid>,
    pub invited_at: Option<DateTime<Utc>>,
    pub joined_at: Option<DateTime<Utc>>,
    pub status: TenantUserStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TenantRole {
    Owner,
    Admin,
    Manager,
    User,
    ReadOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TenantUserStatus {
    Pending,
    Active,
    Suspended,
    Removed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantUsage {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub period: String,
    pub users_count: i32,
    pub products_count: i32,
    pub orders_count: i32,
    pub storage_used_mb: i32,
    pub api_calls: i32,
    pub computed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantBilling {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub billing_email: String,
    pub billing_address: BillingAddress,
    pub payment_method: Option<PaymentMethod>,
    pub subscription_id: Option<String>,
    pub next_billing_date: Option<DateTime<Utc>>,
    pub amount_due: i64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BillingAddress {
    pub company_name: String,
    pub address1: String,
    pub address2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub tax_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentMethod {
    pub type_: String,
    pub last_four: Option<String>,
    pub expiry: Option<String>,
    pub brand: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantInvitation {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub email: String,
    pub role: TenantRole,
    pub invited_by: Uuid,
    pub token: String,
    pub expires_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub status: InvitationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InvitationStatus {
    Pending,
    Accepted,
    Declined,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantFeature {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub feature_key: String,
    pub enabled: bool,
    pub settings: Option<serde_json::Value>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantAuditLog {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub user_id: Option<Uuid>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub old_value: Option<serde_json::Value>,
    pub new_value: Option<serde_json::Value>,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTenantRequest {
    pub code: String,
    pub name: String,
    pub plan: TenantPlan,
    pub admin_email: String,
    pub admin_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InviteUserRequest {
    pub email: String,
    pub role: TenantRole,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTenantRequest {
    pub name: Option<String>,
    pub plan: Option<TenantPlan>,
    pub settings: Option<serde_json::Value>,
    pub branding: Option<TenantBranding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TenantStats {
    pub total_tenants: i64,
    pub active_tenants: i64,
    pub trial_tenants: i64,
    pub by_plan: std::collections::HashMap<String, i64>,
    pub total_users: i64,
    pub total_revenue: i64,
}
