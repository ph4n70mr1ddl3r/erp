use sqlx::{SqlitePool, sqlite::SqlitePoolOptions};
use std::sync::Arc;
use crate::Config;
use crate::handlers::websocket::WebSocketManager;
use erp_auth::AuthService;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub config: Arc<Config>,
    pub ws_manager: WebSocketManager,
    pub auth_svc: Arc<AuthService>,
    pub project_svc: Arc<erp_projects::ProjectService>,
    pub timesheet_svc: Arc<erp_projects::TimesheetService>,
    pub payment_svc: Arc<erp_payments::PaymentService>,
    pub gateway_svc: Arc<erp_payments::GatewayService>,
    pub stripe_svc: Arc<Option<erp_payments::StripeService>>,
    pub backup_svc: Arc<erp_backup::BackupService>,
}

impl AppState {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let pool = SqlitePoolOptions::new()
            .max_connections(10)
            .min_connections(1)
            .connect(&config.database_url)
            .await?;
        
        run_migrations(&pool).await?;
        
        let stripe_svc = if let Some(stripe_config) = config.stripe.clone() {
            erp_payments::StripeService::new(stripe_config, pool.clone()).ok()
        } else {
            None
        };

        Ok(Self {
            pool: pool.clone(),
            config: Arc::new(config),
            ws_manager: Arc::new(crate::handlers::websocket::WebSocketManagerInner::new()),
            auth_svc: Arc::new(AuthService::new(pool.clone())),
            project_svc: Arc::new(erp_projects::ProjectService::new(pool.clone())),
            timesheet_svc: Arc::new(erp_projects::TimesheetService::new(pool.clone())),
            payment_svc: Arc::new(erp_payments::PaymentService::new(pool.clone())),
            gateway_svc: Arc::new(erp_payments::GatewayService::new(pool.clone())),
            stripe_svc: Arc::new(stripe_svc),
            backup_svc: Arc::new(erp_backup::BackupService::new(pool)),
        })
    }
}

async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    let migration_queries: Vec<&str> = vec![
        include_str!("../../migrations/20240101000000_finance.sql"),
        include_str!("../../migrations/20240101000001_inventory.sql"),
        include_str!("../../migrations/20240101000002_sales.sql"),
        include_str!("../../migrations/20240101000003_purchasing.sql"),
        include_str!("../../migrations/20240101000005_hr.sql"),
        include_str!("../../migrations/20240101000006_auth.sql"),
        include_str!("../../migrations/20240101000013_enterprise_features.sql"),
        include_str!("../../migrations/20240101200500_enterprise_wms_demand_edi_tenant_revrec_intercompany_lms.sql"),
        include_str!("../../migrations/20240304000000_stripe_payments.sql"),
    ];
    
    for query in migration_queries {
        for statement in query.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() {
                if let Err(e) = sqlx::query(statement).execute(pool).await {
                    if !e.to_string().contains("already exists") {
                        return Err(anyhow::anyhow!(e));
                    }
                }
            }
        }
    }
    
    Ok(())
}
