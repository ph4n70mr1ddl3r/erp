use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait TenantRepository: Send + Sync {
    async fn create_tenant(&self, _tenant: &Tenant) -> anyhow::Result<()> { Ok(()) }
    async fn get_tenant(&self, _id: Uuid) -> anyhow::Result<Option<Tenant>> { Ok(None) }
    async fn get_tenant_by_code(&self, _code: &str) -> anyhow::Result<Option<Tenant>> { Ok(None) }
    async fn list_tenants(&self, _status: Option<TenantStatus>) -> anyhow::Result<Vec<Tenant>> { Ok(vec![]) }
    async fn update_tenant(&self, _tenant: &Tenant) -> anyhow::Result<()> { Ok(()) }
    async fn create_tenant_user(&self, _user: &TenantUser) -> anyhow::Result<()> { Ok(()) }
    async fn get_tenant_user(&self, _id: Uuid) -> anyhow::Result<Option<TenantUser>> { Ok(None) }
    async fn list_tenant_users(&self, _tenant_id: Uuid) -> anyhow::Result<Vec<TenantUser>> { Ok(vec![]) }
    async fn update_tenant_user_status(&self, _id: Uuid, _status: TenantUserStatus) -> anyhow::Result<()> { Ok(()) }
    async fn create_invitation(&self, _invitation: &TenantInvitation) -> anyhow::Result<()> { Ok(()) }
    async fn get_invitation(&self, _token: &str) -> anyhow::Result<Option<TenantInvitation>> { Ok(None) }
    async fn accept_invitation(&self, _token: &str, _user_id: Uuid) -> anyhow::Result<()> { Ok(()) }
    async fn record_usage(&self, _usage: &TenantUsage) -> anyhow::Result<()> { Ok(()) }
    async fn get_latest_usage(&self, _tenant_id: Uuid) -> anyhow::Result<Option<TenantUsage>> { Ok(None) }
    async fn set_feature(&self, _feature: &TenantFeature) -> anyhow::Result<()> { Ok(()) }
    async fn get_features(&self, _tenant_id: Uuid) -> anyhow::Result<Vec<TenantFeature>> { Ok(vec![]) }
    async fn create_audit_log(&self, _log: &TenantAuditLog) -> anyhow::Result<()> { Ok(()) }
    async fn list_audit_logs(&self, _tenant_id: Uuid, _limit: i32) -> anyhow::Result<Vec<TenantAuditLog>> { Ok(vec![]) }
    async fn create_billing(&self, _billing: &TenantBilling) -> anyhow::Result<()> { Ok(()) }
    async fn get_billing(&self, _tenant_id: Uuid) -> anyhow::Result<Option<TenantBilling>> { Ok(None) }
    async fn update_billing(&self, _billing: &TenantBilling) -> anyhow::Result<()> { Ok(()) }
}

pub struct SqliteTenantRepository {
    pub pool: SqlitePool,
}

impl SqliteTenantRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TenantRepository for SqliteTenantRepository {}
