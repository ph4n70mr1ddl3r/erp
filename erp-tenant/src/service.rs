use chrono::Utc;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct TenantService<R: TenantRepository> {
    repo: R,
}

impl TenantService<SqliteTenantRepository> {
    pub fn new(repo: SqliteTenantRepository) -> Self {
        Self { repo }
    }
}

impl<R: TenantRepository> TenantService<R> {
    pub async fn create_tenant(&self, req: CreateTenantRequest) -> anyhow::Result<Tenant> {
        let limits = match req.plan {
            TenantPlan::Free => TenantLimits { max_users: 3, max_products: 100, max_orders_per_month: 50, storage_mb: 100, api_calls_per_day: 100 },
            TenantPlan::Starter => TenantLimits { max_users: 10, max_products: 1000, max_orders_per_month: 500, storage_mb: 1000, api_calls_per_day: 1000 },
            TenantPlan::Professional => TenantLimits { max_users: 50, max_products: 10000, max_orders_per_month: 5000, storage_mb: 10000, api_calls_per_day: 10000 },
            TenantPlan::Enterprise => TenantLimits { max_users: 0, max_products: 0, max_orders_per_month: 0, storage_mb: 0, api_calls_per_day: 0 },
            TenantPlan::Custom => TenantLimits { max_users: 0, max_products: 0, max_orders_per_month: 0, storage_mb: 0, api_calls_per_day: 0 },
        };

        let tenant = Tenant {
            id: Uuid::new_v4(),
            code: req.code,
            name: req.name,
            plan: req.plan,
            status: TenantStatus::Trial,
            settings: serde_json::json!({}),
            branding: None,
            limits,
            trial_ends_at: Some(Utc::now() + chrono::Duration::days(14)),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_tenant(&tenant).await?;
        Ok(tenant)
    }

    pub async fn get_tenant(&self, id: Uuid) -> anyhow::Result<Option<Tenant>> {
        self.repo.get_tenant(id).await
    }

    pub async fn list_tenants(&self, status: Option<TenantStatus>) -> anyhow::Result<Vec<Tenant>> {
        self.repo.list_tenants(status).await
    }

    pub async fn invite_user(&self, tenant_id: Uuid, req: InviteUserRequest, invited_by: Uuid) -> anyhow::Result<TenantInvitation> {
        let token = Uuid::new_v4().to_string();
        let invitation = TenantInvitation {
            id: Uuid::new_v4(),
            tenant_id,
            email: req.email,
            role: req.role,
            invited_by,
            token,
            expires_at: Utc::now() + chrono::Duration::days(7),
            accepted_at: None,
            status: InvitationStatus::Pending,
        };
        self.repo.create_invitation(&invitation).await?;
        Ok(invitation)
    }

    pub async fn accept_invitation(&self, token: &str, user_id: Uuid) -> anyhow::Result<()> {
        self.repo.accept_invitation(token, user_id).await
    }

    pub async fn record_usage(&self, tenant_id: Uuid, users: i32, products: i32, orders: i32, storage: i32, api_calls: i32) -> anyhow::Result<TenantUsage> {
        let usage = TenantUsage {
            id: Uuid::new_v4(),
            tenant_id,
            period: Utc::now().format("%Y-%m").to_string(),
            users_count: users,
            products_count: products,
            orders_count: orders,
            storage_used_mb: storage,
            api_calls,
            computed_at: Utc::now(),
        };
        self.repo.record_usage(&usage).await?;
        Ok(usage)
    }

    pub async fn set_feature(&self, tenant_id: Uuid, feature_key: String, enabled: bool, settings: Option<serde_json::Value>) -> anyhow::Result<TenantFeature> {
        let feature = TenantFeature {
            id: Uuid::new_v4(),
            tenant_id,
            feature_key,
            enabled,
            settings,
            updated_at: Utc::now(),
        };
        self.repo.set_feature(&feature).await?;
        Ok(feature)
    }

    pub async fn get_stats(&self) -> anyhow::Result<TenantStats> {
        let tenants = self.repo.list_tenants(None).await?;
        let active = tenants.iter().filter(|t| matches!(t.status, TenantStatus::Active)).count() as i64;
        let trial = tenants.iter().filter(|t| matches!(t.status, TenantStatus::Trial)).count() as i64;
        
        Ok(TenantStats {
            total_tenants: tenants.len() as i64,
            active_tenants: active,
            trial_tenants: trial,
            by_plan: std::collections::HashMap::new(),
            total_users: 0,
            total_revenue: 0,
        })
    }
}
