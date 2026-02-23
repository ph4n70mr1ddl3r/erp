use sqlx::SqlitePool;
use std::sync::Arc;
use crate::Config;

#[derive(Clone)]
pub struct AppState {
    pub pool: SqlitePool,
    pub config: Arc<Config>,
}

impl AppState {
    pub async fn new(config: Config) -> anyhow::Result<Self> {
        let pool = SqlitePool::connect(&config.database_url).await?;
        
        run_migrations(&pool).await?;
        
        Ok(Self {
            pool,
            config: Arc::new(config),
        })
    }
}

async fn run_migrations(pool: &SqlitePool) -> anyhow::Result<()> {
    let migration_queries = vec![
        include_str!("../../migrations/20240101000000_finance.sql"),
        include_str!("../../migrations/20240101000001_inventory.sql"),
        include_str!("../../migrations/20240101000002_sales.sql"),
        include_str!("../../migrations/20240101000003_purchasing.sql"),
        include_str!("../../migrations/20240101000004_manufacturing.sql"),
        include_str!("../../migrations/20240101000005_hr.sql"),
        include_str!("../../migrations/20240101000006_auth.sql"),
        include_str!("../../migrations/20240101000007_audit.sql"),
        include_str!("../../migrations/20240101000008_quotations.sql"),
        include_str!("../../migrations/20240101000009_workflows.sql"),
        include_str!("../../migrations/20240101000010_attachments.sql"),
        include_str!("../../migrations/20240101000011_extended_features.sql"),
        include_str!("../../migrations/20240101000012_advanced_features.sql"),
        include_str!("../../migrations/20240101000013_enterprise_features.sql"),
        include_str!("../../migrations/20240101000014_indexes.sql"),
        include_str!("../../migrations/20240101000015_service_management.sql"),
        include_str!("../../migrations/20240101000016_it_assets.sql"),
        include_str!("../../migrations/20240101000017_compliance.sql"),
        include_str!("../../migrations/20240101000018_enterprise_additions.sql"),
        include_str!("../../migrations/20240101000019_new_modules.sql"),
        include_str!("../../migrations/20240101000020_ai_portals_iot_automation.sql"),
    ];
    
    for migration in migration_queries {
        for statement in migration.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() {
                if let Err(e) = sqlx::query(statement).execute(pool).await {
                    if !e.to_string().contains("already exists") {
                        return Err(e.into());
                    }
                }
            }
        }
    }
    
    Ok(())
}
