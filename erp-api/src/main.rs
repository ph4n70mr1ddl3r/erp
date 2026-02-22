use anyhow::Result;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| "erp_api=debug,tower_http=debug".into()))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = erp_api::Config::from_env();
    
    erp_auth::init_jwt_secret(&config.jwt_secret);
    
    let state = erp_api::AppState::new(config.clone()).await?;
    let app = erp_api::routes::create_router(state);

    let addr = format!("{}:{}", config.server_host, config.server_port);
    tracing::info!("ERP server listening on {}", addr);
    tracing::info!("API endpoints:");
    tracing::info!("  POST /auth/register - Register new user");
    tracing::info!("  POST /auth/login    - Login");
    tracing::info!("  GET  /auth/me       - Get current user (requires auth)");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
