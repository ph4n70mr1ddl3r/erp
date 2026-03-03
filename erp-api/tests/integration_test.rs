use axum::body::Body;
use http_body_util::BodyExt;
use tower::util::ServiceExt;
use axum::http::{Request, StatusCode, Method};
use serde_json::json;
use sqlx::SqlitePool;
use std::sync::Once;
use chrono::Datelike;
use erp_api::db::AppState;
use erp_api::routes::create_router;
use erp_api::Config;
use erp_auth::init_jwt_secret;

static INIT: Once = Once::new();

fn init_test_env() {
    INIT.call_once(|| {
        let _ = tracing_subscriber::fmt::try_init();
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
        include_str!("../../migrations/20240101000011_extended_features.sql"),
        include_str!("../../migrations/20260302000000_inventory_adjustments.sql"),
        include_str!("../../migrations/20260302100000_stock_transfers.sql"),
        include_str!("../../migrations/20240309000000_vendor_bills.sql"),
    ];
    
    for migration in migration_queries {
        for statement in migration.split(';') {
            let statement = statement.trim();
            if !statement.is_empty() {
                if let Err(e) = sqlx::query(statement).execute(pool).await {
                    eprintln!("Migration error: {}", e);
                }
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
        cors_allowed_origins: vec!["http://localhost:5173".to_string()],
        trust_proxy: false,
    };
    let ws_manager = std::sync::Arc::new(erp_api::handlers::websocket::WebSocketManagerInner::new());
    AppState {
        pool,
        config: std::sync::Arc::new(config),
        ws_manager,
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
    
    let req = Request::builder()
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

#[tokio::test]
async fn test_inventory_adjustments() {
    init_test_env();
    let pool = setup_test_db().await;
    let state = create_test_app(pool);
    let app = create_router(state);

    let (_, reg_body) = make_request(
        &app,
        Method::POST,
        "/auth/register",
        Some(json!({
            "username": "adjustmentuser",
            "email": "adjustment@example.com",
            "password": "password123",
            "full_name": "Adjustment User"
        })),
    ).await;
    let token = reg_body["token"].as_str().unwrap();

    let warehouse_request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/inventory/warehouses")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "code": "WH-001",
            "name": "Test Warehouse"
        })).unwrap())).unwrap();
    let warehouse_response = app.clone().oneshot(warehouse_request).await.unwrap();
    let wh_body_bytes = warehouse_response.into_body().collect().await.unwrap().to_bytes();
    let warehouse: serde_json::Value = serde_json::from_slice(&wh_body_bytes).unwrap();
    let warehouse_id = warehouse["id"].as_str().unwrap();

    let product_request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/inventory/products")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "sku": "PROD-001",
            "name": "Test Product",
            "product_type": "Goods",
            "unit_of_measure": "EA"
        })).unwrap())).unwrap();
    let product_response = app.clone().oneshot(product_request).await.unwrap();
    let prod_body_bytes = product_response.into_body().collect().await.unwrap().to_bytes();
    let product: serde_json::Value = serde_json::from_slice(&prod_body_bytes).unwrap();
    let product_id = product["id"].as_str().unwrap();

    let location_id = warehouse_id;

    let request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/inventory-adjustments")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "warehouse_id": warehouse_id,
            "adjustment_type": "CountVariance",
            "reason": "Annual stock count",
            "lines": [{
                "product_id": product_id,
                "location_id": location_id,
                "system_quantity": 100,
                "counted_quantity": 95,
                "unit_cost": 1000
            }]
        })).unwrap())).unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body_text = String::from_utf8_lossy(&body_bytes);
    if status != StatusCode::OK {
        eprintln!("Error response: {} - {}", status, body_text);
    }
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn test_inventory_adjustments_workflow() {
    init_test_env();
    let pool = setup_test_db().await;
    let state = create_test_app(pool);
    let app = create_router(state);

    let (_, reg_body) = make_request(
        &app,
        Method::POST,
        "/auth/register",
        Some(json!({
            "username": "wfuser",
            "email": "wf@example.com",
            "password": "password123",
            "full_name": "Workflow User"
        })),
    ).await;
    let token = reg_body["token"].as_str().unwrap();

    let warehouse_request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/inventory/warehouses")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "code": "WH-002",
            "name": "Test Warehouse 2"
        })).unwrap())).unwrap();
    let warehouse_response = app.clone().oneshot(warehouse_request).await.unwrap();
    let wh_body_bytes = warehouse_response.into_body().collect().await.unwrap().to_bytes();
    let warehouse: serde_json::Value = serde_json::from_slice(&wh_body_bytes).unwrap();
    let warehouse_id = warehouse["id"].as_str().unwrap();

    let product_request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/inventory/products")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "sku": "PROD-002",
            "name": "Test Product 2",
            "product_type": "Goods",
            "unit_of_measure": "EA"
        })).unwrap())).unwrap();
    let product_response = app.clone().oneshot(product_request).await.unwrap();
    let prod_body_bytes = product_response.into_body().collect().await.unwrap().to_bytes();
    let product: serde_json::Value = serde_json::from_slice(&prod_body_bytes).unwrap();
    let product_id = product["id"].as_str().unwrap();

    let location_id = warehouse_id;

    let create_request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/inventory-adjustments")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "warehouse_id": warehouse_id,
            "adjustment_type": "Damage",
            "reason": "Damaged goods",
            "lines": [{
                "product_id": product_id,
                "location_id": location_id,
                "system_quantity": 50,
                "counted_quantity": 45,
                "unit_cost": 2000
            }]
        })).unwrap())).unwrap();

    let response = app.clone().oneshot(create_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let adj: serde_json::Value = serde_json::from_slice(&body_bytes).unwrap();
    let adj_id = adj["id"].as_str().unwrap();

    let submit_request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/api/v1/inventory-adjustments/{}/submit", adj_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty()).unwrap();

    let response = app.clone().oneshot(submit_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    let approve_request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/api/v1/inventory-adjustments/{}/approve", adj_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty()).unwrap();

    let response = app.clone().oneshot(approve_request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_expense_report_workflow() {
    init_test_env();
    let pool = setup_test_db().await;
    let state = create_test_app(pool);
    let app = create_router(state);
    
    let (_, reg_body) = make_request(
        &app,
        Method::POST,
        "/auth/register",
        Some(json!({
            "username": "expenseuser",
            "email": "expense@example.com",
            "password": "password123",
            "full_name": "Expense User"
        })),
    ).await;
    let token = reg_body["token"].as_str().unwrap();

    let employee_request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/hr/employees")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "employee_number": "EMP-001",
            "first_name": "John",
            "last_name": "Doe",
            "email": "john.doe@example.com",
            "hire_date": "2024-01-01"
        })).unwrap())).unwrap();
    let employee_response = app.clone().oneshot(employee_request).await.unwrap();
    assert_eq!(employee_response.status(), StatusCode::OK);
    let emp_body_bytes = employee_response.into_body().collect().await.unwrap().to_bytes();
    let employee: serde_json::Value = serde_json::from_slice(&emp_body_bytes).unwrap();
    let employee_id = employee["id"].as_str().unwrap();

    let category_request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/expense-categories")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "name": "Travel",
            "code": "TRAVEL"
        })).unwrap())).unwrap();
    let category_response = app.clone().oneshot(category_request).await.unwrap();
    assert_eq!(category_response.status(), StatusCode::OK);
    let cat_body_bytes = category_response.into_body().collect().await.unwrap().to_bytes();
    let category: serde_json::Value = serde_json::from_slice(&cat_body_bytes).unwrap();
    let category_id = category["id"].as_str().unwrap();

    let report_request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/expense-reports")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "employee_id": employee_id,
            "title": "Business trip to NYC",
            "lines": [{
                "category_id": category_id,
                "expense_date": "2024-01-15",
                "amount": 25000,
                "description": "Flight tickets"
            }, {
                "category_id": category_id,
                "expense_date": "2024-01-16",
                "amount": 15000,
                "description": "Hotel"
            }]
        })).unwrap())).unwrap();
    let report_response = app.clone().oneshot(report_request).await.unwrap();
    assert_eq!(report_response.status(), StatusCode::OK);
    let report_body_bytes = report_response.into_body().collect().await.unwrap().to_bytes();
    let report: serde_json::Value = serde_json::from_slice(&report_body_bytes).unwrap();
    let report_id = report["id"].as_str().unwrap();
    assert_eq!(report["status"], "Draft");

    let submit_request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/api/v1/expense-reports/{}/submit", report_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty()).unwrap();
    let submit_response = app.clone().oneshot(submit_request).await.unwrap();
    assert_eq!(submit_response.status(), StatusCode::OK);
    let submit_body_bytes = submit_response.into_body().collect().await.unwrap().to_bytes();
    let submitted: serde_json::Value = serde_json::from_slice(&submit_body_bytes).unwrap();
    assert_eq!(submitted["status"], "Submitted");

    let approve_request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/api/v1/expense-reports/{}/approve", report_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty()).unwrap();
    let approve_response = app.clone().oneshot(approve_request).await.unwrap();
    assert_eq!(approve_response.status(), StatusCode::OK);
    let approve_body_bytes = approve_response.into_body().collect().await.unwrap().to_bytes();
    let approved: serde_json::Value = serde_json::from_slice(&approve_body_bytes).unwrap();
    assert_eq!(approved["status"], "Approved");
}

#[tokio::test]
async fn test_expense_report_rejection() {
    init_test_env();
    let pool = setup_test_db().await;
    let state = create_test_app(pool);
    let app = create_router(state);
    
    let (_, reg_body) = make_request(
        &app,
        Method::POST,
        "/auth/register",
        Some(json!({
            "username": "rejectuser",
            "email": "reject@example.com",
            "password": "password123",
            "full_name": "Reject User"
        })),
    ).await;
    let token = reg_body["token"].as_str().unwrap();

    let employee_request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/hr/employees")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "employee_number": "EMP-002",
            "first_name": "Jane",
            "last_name": "Smith",
            "email": "jane.smith@example.com",
            "hire_date": "2024-01-01"
        })).unwrap())).unwrap();
    let employee_response = app.clone().oneshot(employee_request).await.unwrap();
    let emp_body_bytes = employee_response.into_body().collect().await.unwrap().to_bytes();
    let employee: serde_json::Value = serde_json::from_slice(&emp_body_bytes).unwrap();
    let employee_id = employee["id"].as_str().unwrap();

    let category_request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/expense-categories")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "name": "Meals",
            "code": "MEALS"
        })).unwrap())).unwrap();
    let category_response = app.clone().oneshot(category_request).await.unwrap();
    let cat_body_bytes = category_response.into_body().collect().await.unwrap().to_bytes();
    let category: serde_json::Value = serde_json::from_slice(&cat_body_bytes).unwrap();
    let category_id = category["id"].as_str().unwrap();

    let report_request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/expense-reports")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "employee_id": employee_id,
            "title": "Client dinner",
            "lines": [{
                "category_id": category_id,
                "expense_date": "2024-01-20",
                "amount": 5000,
                "description": "Restaurant"
            }]
        })).unwrap())).unwrap();
    let report_response = app.clone().oneshot(report_request).await.unwrap();
    let report_body_bytes = report_response.into_body().collect().await.unwrap().to_bytes();
    let report: serde_json::Value = serde_json::from_slice(&report_body_bytes).unwrap();
    let report_id = report["id"].as_str().unwrap();

    let submit_request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/api/v1/expense-reports/{}/submit", report_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty()).unwrap();
    let _ = app.clone().oneshot(submit_request).await.unwrap();

    let reject_request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/api/v1/expense-reports/{}/reject", report_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "reason": "Missing receipt"
        })).unwrap())).unwrap();
    let reject_response = app.clone().oneshot(reject_request).await.unwrap();
    assert_eq!(reject_response.status(), StatusCode::OK);
    let reject_body_bytes = reject_response.into_body().collect().await.unwrap().to_bytes();
    let rejected: serde_json::Value = serde_json::from_slice(&reject_body_bytes).unwrap();
    assert_eq!(rejected["status"], "Rejected");
    assert_eq!(rejected["rejection_reason"], "Missing receipt");
}

#[tokio::test]
async fn test_stock_transfer_workflow() {
    init_test_env();
    let pool = setup_test_db().await;
    let app = create_router(create_test_app(pool));

    let register_request = Request::builder()
        .method(Method::POST)
        .uri("/auth/register")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&json!({
            "username": "transferuser",
            "email": "transfer@test.com",
            "password": "password123",
            "full_name": "Transfer User"
        })).unwrap())).unwrap();
    let register_response = app.clone().oneshot(register_request).await.unwrap();
    assert_eq!(register_response.status(), StatusCode::OK);

    let login_request = Request::builder()
        .method(Method::POST)
        .uri("/auth/login")
        .header("Content-Type", "application/json")
        .body(Body::from(serde_json::to_string(&json!({
            "username": "transferuser",
            "password": "password123"
        })).unwrap())).unwrap();
    let login_response = app.clone().oneshot(login_request).await.unwrap();
    let login_body_bytes = login_response.into_body().collect().await.unwrap().to_bytes();
    let login: serde_json::Value = serde_json::from_slice(&login_body_bytes).unwrap();
    let token = login["token"].as_str().unwrap();

    let warehouse1_id = uuid::Uuid::new_v4().to_string();
    let warehouse2_id = uuid::Uuid::new_v4().to_string();
    let product_id = uuid::Uuid::new_v4().to_string();

    let create_request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/stock-transfers")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "from_warehouse_id": warehouse1_id,
            "to_warehouse_id": warehouse2_id,
            "priority": "Normal",
            "lines": [{
                "product_id": product_id,
                "requested_quantity": 100,
                "unit_cost": 1000
            }]
        })).unwrap())).unwrap();
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    assert_eq!(create_response.status(), StatusCode::OK);
    let create_body_bytes = create_response.into_body().collect().await.unwrap().to_bytes();
    let transfer: serde_json::Value = serde_json::from_slice(&create_body_bytes).unwrap();
    assert_eq!(transfer["status"], "Draft");
    let transfer_id = transfer["id"].as_str().unwrap();

    let submit_request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/api/v1/stock-transfers/{}/submit", transfer_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty()).unwrap();
    let submit_response = app.clone().oneshot(submit_request).await.unwrap();
    assert_eq!(submit_response.status(), StatusCode::OK);
    let submit_body_bytes = submit_response.into_body().collect().await.unwrap().to_bytes();
    let submitted: serde_json::Value = serde_json::from_slice(&submit_body_bytes).unwrap();
    assert_eq!(submitted["status"], "Pending");

    let approve_request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/api/v1/stock-transfers/{}/approve", transfer_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty()).unwrap();
    let approve_response = app.clone().oneshot(approve_request).await.unwrap();
    assert_eq!(approve_response.status(), StatusCode::OK);
    let approve_body_bytes = approve_response.into_body().collect().await.unwrap().to_bytes();
    let approved: serde_json::Value = serde_json::from_slice(&approve_body_bytes).unwrap();
    assert_eq!(approved["status"], "Approved");

    let ship_request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/api/v1/stock-transfers/{}/ship", transfer_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "lines": [{
                "product_id": product_id,
                "shipped_quantity": 100
            }]
        })).unwrap())).unwrap();
    let ship_response = app.clone().oneshot(ship_request).await.unwrap();
    assert_eq!(ship_response.status(), StatusCode::OK);
    let ship_body_bytes = ship_response.into_body().collect().await.unwrap().to_bytes();
    let shipped: serde_json::Value = serde_json::from_slice(&ship_body_bytes).unwrap();
    assert_eq!(shipped["status"], "InTransit");

    let receive_request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/api/v1/stock-transfers/{}/receive", transfer_id))
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "lines": [{
                "product_id": product_id,
                "received_quantity": 100
            }]
        })).unwrap())).unwrap();
    let receive_response = app.clone().oneshot(receive_request).await.unwrap();
    assert_eq!(receive_response.status(), StatusCode::OK);
    let receive_body_bytes = receive_response.into_body().collect().await.unwrap().to_bytes();
    let received: serde_json::Value = serde_json::from_slice(&receive_body_bytes).unwrap();
    assert_eq!(received["status"], "Received");
}

#[tokio::test]
async fn test_vendor_bills_crud() {
    init_test_env();
    let pool = setup_test_db().await;
    let app = create_router(create_test_app(pool));

    let (_, register_body) = make_request(&app, Method::POST, "/auth/register", Some(json!({
        "username": "vbuser",
        "email": "vb@example.com",
        "password": "password123",
        "full_name": "VB User"
    }))).await;
    let token = register_body["token"].as_str().unwrap();

    let vendor_request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/purchasing/vendors")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "code": "V001",
            "name": "Test Vendor",
            "email": "vendor@example.com",
            "phone": "555-0001"
        })).unwrap())).unwrap();
    let vendor_response = app.clone().oneshot(vendor_request).await.unwrap();
    assert_eq!(vendor_response.status(), StatusCode::OK);
    let vendor_body_bytes = vendor_response.into_body().collect().await.unwrap().to_bytes();
    let vendor_body: serde_json::Value = serde_json::from_slice(&vendor_body_bytes).unwrap();
    let vendor_id = vendor_body["id"].as_str().unwrap();

    let now = chrono::Utc::now();
    let due = now + chrono::Duration::days(30);

    let create_request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/vendor-bills")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "vendor_invoice_number": "INV-2024-001",
            "vendor_id": vendor_id,
            "bill_date": now.to_rfc3339(),
            "due_date": due.to_rfc3339(),
            "lines": [{
                "description": "Office Supplies",
                "quantity": 10,
                "unit_price": 2500,
                "tax_rate": 10.0
            }]
        })).unwrap())).unwrap();
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    assert_eq!(create_response.status(), StatusCode::OK);
    let create_body_bytes = create_response.into_body().collect().await.unwrap().to_bytes();
    let bill: serde_json::Value = serde_json::from_slice(&create_body_bytes).unwrap();
    assert_eq!(bill["status"], "Draft");
    assert_eq!(bill["vendor_invoice_number"], "INV-2024-001");
    let bill_id = bill["id"].as_str().unwrap();

    let submit_request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/api/v1/vendor-bills/{}/submit", bill_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty()).unwrap();
    let submit_response = app.clone().oneshot(submit_request).await.unwrap();
    assert_eq!(submit_response.status(), StatusCode::OK);
    let submit_body_bytes = submit_response.into_body().collect().await.unwrap().to_bytes();
    let submitted: serde_json::Value = serde_json::from_slice(&submit_body_bytes).unwrap();
    assert_eq!(submitted["status"], "submitted");

    let approve_request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/api/v1/vendor-bills/{}/approve", bill_id))
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty()).unwrap();
    let approve_response = app.clone().oneshot(approve_request).await.unwrap();
    assert_eq!(approve_response.status(), StatusCode::OK);
    let approve_body_bytes = approve_response.into_body().collect().await.unwrap().to_bytes();
    let approved: serde_json::Value = serde_json::from_slice(&approve_body_bytes).unwrap();
    assert_eq!(approved["status"], "approved");

    let list_request = Request::builder()
        .method(Method::GET)
        .uri("/api/v1/vendor-bills?page=1&per_page=10")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty()).unwrap();
    let list_response = app.clone().oneshot(list_request).await.unwrap();
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body_bytes = list_response.into_body().collect().await.unwrap().to_bytes();
    let list: serde_json::Value = serde_json::from_slice(&list_body_bytes).unwrap();
    assert!(list["items"].as_array().unwrap().len() > 0);
}

#[tokio::test]
async fn test_budgets_crud() {
    init_test_env();
    let pool = setup_test_db().await;
    let app = create_router(create_test_app(pool));

    let (_, register_body) = make_request(&app, Method::POST, "/auth/register", Some(json!({
        "username": "budgetuser",
        "email": "budget@example.com",
        "password": "password123",
        "full_name": "Budget User"
    }))).await;
    let token = register_body["token"].as_str().unwrap();

    let account_request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/finance/accounts")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "code": "EXP001",
            "name": "Operating Expenses",
            "account_type": "Expense"
        })).unwrap())).unwrap();
    let account_response = app.clone().oneshot(account_request).await.unwrap();
    assert_eq!(account_response.status(), StatusCode::OK);
    let account_body_bytes = account_response.into_body().collect().await.unwrap().to_bytes();
    let account_body: serde_json::Value = serde_json::from_slice(&account_body_bytes).unwrap();
    let account_id = account_body["id"].as_str().unwrap();

    let now = chrono::Utc::now();
    let start_of_year = chrono::DateTime::parse_from_rfc3339(&format!("{}-01-01T00:00:00Z", now.year()))
        .unwrap()
        .with_timezone(&chrono::Utc);
    let end_of_year = chrono::DateTime::parse_from_rfc3339(&format!("{}-12-31T23:59:59Z", now.year()))
        .unwrap()
        .with_timezone(&chrono::Utc);

    let create_request = Request::builder()
        .method(Method::POST)
        .uri("/api/v1/budgets")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::from(serde_json::to_string(&json!({
            "name": format!("FY {} Operating Budget", now.year()),
            "start_date": start_of_year.to_rfc3339(),
            "end_date": end_of_year.to_rfc3339(),
            "lines": [
                {
                    "account_id": account_id,
                    "period": 1,
                    "amount": 500000
                },
                {
                    "account_id": account_id,
                    "period": 2,
                    "amount": 600000
                }
            ]
        })).unwrap())).unwrap();
    let create_response = app.clone().oneshot(create_request).await.unwrap();
    assert_eq!(create_response.status(), StatusCode::OK);
    let create_body_bytes = create_response.into_body().collect().await.unwrap().to_bytes();
    let budget: serde_json::Value = serde_json::from_slice(&create_body_bytes).unwrap();
    assert_eq!(budget["name"], format!("FY {} Operating Budget", now.year()));
    assert_eq!(budget["status"], "Draft");
    assert_eq!(budget["total_amount"], 11000.0);
    assert!(budget["lines"].as_array().unwrap().len() == 2);

    let list_request = Request::builder()
        .method(Method::GET)
        .uri("/api/v1/budgets")
        .header("Authorization", format!("Bearer {}", token))
        .body(Body::empty()).unwrap();
    let list_response = app.clone().oneshot(list_request).await.unwrap();
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body_bytes = list_response.into_body().collect().await.unwrap().to_bytes();
    let budgets: serde_json::Value = serde_json::from_slice(&list_body_bytes).unwrap();
    assert!(budgets.as_array().unwrap().len() > 0);
    
    let first_budget = &budgets.as_array().unwrap()[0];
    assert_eq!(first_budget["name"], format!("FY {} Operating Budget", now.year()));
    assert!(first_budget["lines"].as_array().unwrap().len() == 2);
}
