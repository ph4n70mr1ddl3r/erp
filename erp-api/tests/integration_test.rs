use axum::body::Body;
use http_body_util::BodyExt;
use tower::util::ServiceExt;
use axum::http::{Request, StatusCode, Method};
use serde_json::json;
use sqlx::SqlitePool;
use std::sync::Once;
use erp_api::db::AppState;
use erp_api::routes::create_router;
use erp_api::Config;
use erp_auth::init_jwt_secret;

static INIT: Once = Once::new();

fn init_test_env() {
    INIT.call_once(|| {
        init_jwt_secret("test-secret-key-for-integration-tests").expect("Failed to init JWT secret");
    });
}

async fn setup_test_db() -> SqlitePool {
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    run_migrations(&pool).await;
    pool
}

async fn run_migrations(pool: &SqlitePool) {
    let migration_queries = vec![
        include_str!("../../migrations/20240101000000_finance.sql"),
        include_str!("../../migrations/20240101000001_inventory.sql"),
        include_str!("../../migrations/20240101000002_sales.sql"),
        include_str!("../../migrations/20240101000003_purchasing.sql"),
        include_str!("../../migrations/20240101000004_manufacturing.sql"),
        include_str!("../../migrations/20240101000005_hr.sql"),
        include_str!("../../migrations/20240101000006_auth.sql"),
    ];
    
    for migration in migration_queries {
        for statement in migration.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() {
                let _ = sqlx::query(statement).execute(pool).await;
            }
        }
    }
}

fn create_test_app(pool: SqlitePool) -> AppState {
    let config = Config {
        database_url: ":memory:".to_string(),
        server_host: "127.0.0.1".to_string(),
        server_port: 3000,
        jwt_secret: "test-secret".to_string(),
        jwt_expiration: 24,
    };
    AppState {
        pool,
        config: std::sync::Arc::new(config),
    }
}

async fn make_request(app: &axum::Router, method: Method, uri: &str, body: Option<serde_json::Value>) -> (StatusCode, serde_json::Value) {
    let mut builder = Request::builder().method(method).uri(uri);
    
    let request = if let Some(b) = body {
        builder = builder.header("Content-Type", "application/json");
        builder.body(Body::from(serde_json::to_string(&b).unwrap())).unwrap()
    } else {
        builder.body(Body::empty()).unwrap()
    };
    
    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body_text = String::from_utf8_lossy(&body_bytes);
    let body_json: serde_json::Value = if body_text.is_empty() {
        json!({})
    } else {
        serde_json::from_str(&body_text).unwrap_or_else(|_| json!({ "raw": body_text }))
    };
    
    (status, body_json)
}

fn with_auth(token: &str) -> String {
    format!("Bearer {}", token)
}

#[tokio::test]
async fn test_health_check() {
    init_test_env();
    let pool = setup_test_db().await;
    let state = create_test_app(pool);
    let app = create_router(state);
    
    let (status, body) = make_request(&app, Method::GET, "/health", None).await;
    
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "healthy");
}

#[tokio::test]
async fn test_register_user() {
    init_test_env();
    let pool = setup_test_db().await;
    let state = create_test_app(pool);
    let app = create_router(state);
    
    let (status, body) = make_request(
        &app,
        Method::POST,
        "/auth/register",
        Some(json!({
            "username": "testuser",
            "email": "test@example.com",
            "password": "password123",
            "full_name": "Test User"
        })),
    ).await;
    
    assert_eq!(status, StatusCode::OK);
    assert!(!body["token"].as_str().unwrap().is_empty());
    assert_eq!(body["user"]["username"], "testuser");
}

#[tokio::test]
async fn test_register_duplicate_username() {
    init_test_env();
    let pool = setup_test_db().await;
    let state = create_test_app(pool);
    let app = create_router(state);
    
    let user = json!({
        "username": "duplicate",
        "email": "dup1@example.com",
        "password": "password123",
        "full_name": "Duplicate User"
    });
    
    let (status1, _) = make_request(&app, Method::POST, "/auth/register", Some(user.clone())).await;
    assert_eq!(status1, StatusCode::OK);
    
    let (status2, _) = make_request(
        &app,
        Method::POST,
        "/auth/register",
        Some(json!({
            "username": "duplicate",
            "email": "dup2@example.com",
            "password": "password123",
            "full_name": "Another User"
        })),
    ).await;
    assert_eq!(status2, StatusCode::CONFLICT);
}

#[tokio::test]
async fn test_login_user() {
    init_test_env();
    let pool = setup_test_db().await;
    let state = create_test_app(pool);
    let app = create_router(state);
    
    make_request(
        &app,
        Method::POST,
        "/auth/register",
        Some(json!({
            "username": "loginuser",
            "email": "login@example.com",
            "password": "password123",
            "full_name": "Login User"
        })),
    ).await;
    
    let (status, body) = make_request(
        &app,
        Method::POST,
        "/auth/login",
        Some(json!({
            "username": "loginuser",
            "password": "password123"
        })),
    ).await;
    
    assert_eq!(status, StatusCode::OK);
    assert!(!body["token"].as_str().unwrap().is_empty());
}

#[tokio::test]
async fn test_login_wrong_password() {
    init_test_env();
    let pool = setup_test_db().await;
    let state = create_test_app(pool);
    let app = create_router(state);
    
    make_request(
        &app,
        Method::POST,
        "/auth/register",
        Some(json!({
            "username": "wrongpass",
            "email": "wrong@example.com",
            "password": "password123",
            "full_name": "Wrong Pass User"
        })),
    ).await;
    
    let (status, _) = make_request(
        &app,
        Method::POST,
        "/auth/login",
        Some(json!({
            "username": "wrongpass",
            "password": "wrongpassword"
        })),
    ).await;
    
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_protected_endpoint_without_auth() {
    init_test_env();
    let pool = setup_test_db().await;
    let state = create_test_app(pool);
    let app = create_router(state);
    
    let (status, _) = make_request(&app, Method::GET, "/auth/me", None).await;
    
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_create_product_with_auth() {
    init_test_env();
    let pool = setup_test_db().await;
    let state = create_test_app(pool);
    let app = create_router(state);
    
    let (_, reg_body) = make_request(
        &app,
        Method::POST,
        "/auth/register",
        Some(json!({
            "username": "productuser",
            "email": "product@example.com",
            "password": "password123",
            "full_name": "Product User"
        })),
    ).await;
    let token = reg_body["token"].as_str().unwrap();
    
    let mut req = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/inventory/products")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token));
    
    let request = req.body(Body::from(serde_json::to_string(&json!({
        "sku": "TEST-001",
        "name": "Test Product",
        "unit_of_measure": "PCS",
        "cost": 1000,
        "price": 1500
    })).unwrap())).unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_account_with_auth() {
    init_test_env();
    let pool = setup_test_db().await;
    let state = create_test_app(pool);
    let app = create_router(state);
    
    let (_, reg_body) = make_request(
        &app,
        Method::POST,
        "/auth/register",
        Some(json!({
            "username": "accountuser",
            "email": "account@example.com",
            "password": "password123",
            "full_name": "Account User"
        })),
    ).await;
    let token = reg_body["token"].as_str().unwrap();
    
    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/finance/accounts")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "code": "1000",
            "name": "Cash",
            "account_type": "Asset"
        })).unwrap())).unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_list_products_with_auth() {
    init_test_env();
    let pool = setup_test_db().await;
    let state = create_test_app(pool);
    let app = create_router(state);
    
    let (_, reg_body) = make_request(
        &app,
        Method::POST,
        "/auth/register",
        Some(json!({
            "username": "listuser",
            "email": "list@example.com",
            "password": "password123",
            "full_name": "List User"
        })),
    ).await;
    let token = reg_body["token"].as_str().unwrap();
    
    let request = Request::builder()
        .method(Method::GET)
        .uri("/api/v1/inventory/products?page=1&per_page=20")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty()).unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_create_customer_with_auth() {
    init_test_env();
    let pool = setup_test_db().await;
    let state = create_test_app(pool);
    let app = create_router(state);
    
    let (_, reg_body) = make_request(
        &app,
        Method::POST,
        "/auth/register",
        Some(json!({
            "username": "customeruser",
            "email": "customer@example.com",
            "password": "password123",
            "full_name": "Customer User"
        })),
    ).await;
    let token = reg_body["token"].as_str().unwrap();
    
    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/sales/customers")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "code": "CUST-001",
            "name": "Test Customer",
            "email": "customer@test.com"
        })).unwrap())).unwrap();
    
    let response = app.clone().oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}
