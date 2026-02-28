use axum::{
    extract::{Query, State},
    body::Body,
    http::{header, Response, StatusCode},
};
use crate::db::AppState;
use crate::error::ApiResult;
use tracing::warn;

#[derive(Debug, serde::Deserialize)]
pub struct ExportQuery {
    pub entity: String,
}

pub async fn export_csv(
    State(state): State<AppState>,
    Query(query): Query<ExportQuery>,
) -> ApiResult<Response<Body>> {
    let csv = match query.entity.as_str() {
        "products" => export_products(&state.pool).await?,
        "customers" => export_customers(&state.pool).await?,
        "vendors" => export_vendors(&state.pool).await?,
        "accounts" => export_accounts(&state.pool).await?,
        "employees" => export_employees(&state.pool).await?,
        _ => return Err(erp_core::Error::validation("Unknown entity type").into()),
    };
    
    Response::builder()
        .header(header::CONTENT_TYPE, "text/csv")
        .header(header::CONTENT_DISPOSITION, format!("attachment; filename=\"{}.csv\"", query.entity))
        .body(Body::from(csv))
        .map_err(|e| erp_core::Error::Internal(anyhow::anyhow!("Failed to build response: {}", e)).into())
}

async fn export_products(pool: &sqlx::SqlitePool) -> crate::error::ApiResult<String> {
    let rows: Vec<(String, String, String, String, String)> = sqlx::query_as(
        "SELECT id, sku, name, unit_of_measure, status FROM products"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| erp_core::Error::Database(e.into()))?;
    
    let mut csv = String::from("id,sku,name,unit_of_measure,status\n");
    for (id, sku, name, uom, status) in rows {
        csv.push_str(&format!("{},{},{},{},{}\n", id, escape_csv(&sku), escape_csv(&name), uom, status));
    }
    Ok(csv)
}

async fn export_customers(pool: &sqlx::SqlitePool) -> crate::error::ApiResult<String> {
    let rows: Vec<(String, String, String, Option<String>, String)> = sqlx::query_as(
        "SELECT id, code, name, email, status FROM customers"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| erp_core::Error::Database(e.into()))?;
    
    let mut csv = String::from("id,code,name,email,status\n");
    for (id, code, name, email, status) in rows {
        csv.push_str(&format!("{},{},{},{},{}\n", id, code, escape_csv(&name), email.unwrap_or_default(), status));
    }
    Ok(csv)
}

async fn export_vendors(pool: &sqlx::SqlitePool) -> crate::error::ApiResult<String> {
    let rows: Vec<(String, String, String, Option<String>, String)> = sqlx::query_as(
        "SELECT id, code, name, email, status FROM vendors"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| erp_core::Error::Database(e.into()))?;
    
    let mut csv = String::from("id,code,name,email,status\n");
    for (id, code, name, email, status) in rows {
        csv.push_str(&format!("{},{},{},{},{}\n", id, code, escape_csv(&name), email.unwrap_or_default(), status));
    }
    Ok(csv)
}

async fn export_accounts(pool: &sqlx::SqlitePool) -> crate::error::ApiResult<String> {
    let rows: Vec<(String, String, String, String, String)> = sqlx::query_as(
        "SELECT id, code, name, account_type, status FROM accounts"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| erp_core::Error::Database(e.into()))?;
    
    let mut csv = String::from("id,code,name,account_type,status\n");
    for (id, code, name, account_type, status) in rows {
        csv.push_str(&format!("{},{},{},{},{}\n", id, code, escape_csv(&name), account_type, status));
    }
    Ok(csv)
}

async fn export_employees(pool: &sqlx::SqlitePool) -> crate::error::ApiResult<String> {
    let rows: Vec<(String, String, String, String, String)> = sqlx::query_as(
        "SELECT id, employee_number, first_name, last_name, status FROM employees"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| erp_core::Error::Database(e.into()))?;
    
    let mut csv = String::from("id,employee_number,first_name,last_name,status\n");
    for (id, emp_num, first, last, status) in rows {
        csv.push_str(&format!("{},{},{},{},{}\n", id, emp_num, escape_csv(&first), escape_csv(&last), status));
    }
    Ok(csv)
}

fn escape_csv(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = line.chars().peekable();
    
    while let Some(c) = chars.next() {
        match c {
            '"' if !in_quotes => {
                in_quotes = true;
            }
            '"' if in_quotes => {
                if chars.peek() == Some(&'"') {
                    current.push('"');
                    chars.next();
                } else {
                    in_quotes = false;
                }
            }
            ',' if !in_quotes => {
                fields.push(current);
                current = String::new();
            }
            _ => {
                current.push(c);
            }
        }
    }
    fields.push(current);
    fields
}

#[derive(Debug, serde::Deserialize)]
pub struct ImportQuery {
    pub entity: String,
}

pub async fn import_csv(
    State(state): State<AppState>,
    Query(query): Query<ImportQuery>,
    body: String,
) -> ApiResult<axum::Json<serde_json::Value>> {
    let lines: Vec<&str> = body.lines().collect();
    if lines.is_empty() {
        return Err(erp_core::Error::validation("Empty CSV").into());
    }
    
    let count = match query.entity.as_str() {
        "products" => import_products(&state.pool, &lines).await?,
        "customers" => import_customers(&state.pool, &lines).await?,
        "vendors" => import_vendors(&state.pool, &lines).await?,
        "accounts" => import_accounts(&state.pool, &lines).await?,
        "employees" => import_employees(&state.pool, &lines).await?,
        _ => return Err(erp_core::Error::validation("Unknown entity type").into()),
    };
    
    Ok(axum::Json(serde_json::json!({ "imported": count })))
}

async fn import_products(pool: &sqlx::SqlitePool, lines: &[&str]) -> crate::error::ApiResult<usize> {
    let mut count = 0;
    for line in lines.iter().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let parts = parse_csv_line(line);
        if parts.len() >= 4 {
            let id = uuid::Uuid::new_v4().to_string();
            let now = chrono::Utc::now().to_rfc3339();
            if let Err(e) = sqlx::query(
                "INSERT OR IGNORE INTO products (id, sku, name, unit_of_measure, status, created_at, updated_at)
                 VALUES (?, ?, ?, ?, 'Active', ?, ?)"
            )
            .bind(&id)
            .bind(&parts[1])
            .bind(&parts[2])
            .bind(&parts[3])
            .bind(&now)
            .bind(&now)
            .execute(pool)
            .await
            {
                warn!(sku = %parts[1], error = %e, "Failed to import product");
            } else {
                count += 1;
            }
        }
    }
    Ok(count)
}

async fn import_customers(pool: &sqlx::SqlitePool, lines: &[&str]) -> crate::error::ApiResult<usize> {
    let mut count = 0;
    for line in lines.iter().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let parts = parse_csv_line(line);
        if parts.len() >= 3 {
            let id = uuid::Uuid::new_v4().to_string();
            let now = chrono::Utc::now().to_rfc3339();
            if let Err(e) = sqlx::query(
                "INSERT OR IGNORE INTO customers (id, code, name, email, status, created_at, updated_at, payment_terms)
                 VALUES (?, ?, ?, ?, 'Active', ?, ?, 30)"
            )
            .bind(&id)
            .bind(&parts[1])
            .bind(&parts[2])
            .bind(parts.get(3).unwrap_or(&"".to_string()))
            .bind(&now)
            .bind(&now)
            .execute(pool)
            .await
            {
                warn!(code = %parts[1], error = %e, "Failed to import customer");
            } else {
                count += 1;
            }
        }
    }
    Ok(count)
}

async fn import_vendors(pool: &sqlx::SqlitePool, lines: &[&str]) -> crate::error::ApiResult<usize> {
    let mut count = 0;
    for line in lines.iter().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let parts = parse_csv_line(line);
        if parts.len() >= 3 {
            let id = uuid::Uuid::new_v4().to_string();
            let now = chrono::Utc::now().to_rfc3339();
            if let Err(e) = sqlx::query(
                "INSERT OR IGNORE INTO vendors (id, code, name, email, status, created_at, updated_at, payment_terms)
                 VALUES (?, ?, ?, ?, 'Active', ?, ?, 30)"
            )
            .bind(&id)
            .bind(&parts[1])
            .bind(&parts[2])
            .bind(parts.get(3).unwrap_or(&"".to_string()))
            .bind(&now)
            .bind(&now)
            .execute(pool)
            .await
            {
                warn!(code = %parts[1], error = %e, "Failed to import vendor");
            } else {
                count += 1;
            }
        }
    }
    Ok(count)
}

async fn import_accounts(pool: &sqlx::SqlitePool, lines: &[&str]) -> crate::error::ApiResult<usize> {
    let mut count = 0;
    for line in lines.iter().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let parts = parse_csv_line(line);
        if parts.len() >= 4 {
            let id = uuid::Uuid::new_v4().to_string();
            let now = chrono::Utc::now().to_rfc3339();
            if let Err(e) = sqlx::query(
                "INSERT OR IGNORE INTO accounts (id, code, name, account_type, status, created_at, updated_at)
                 VALUES (?, ?, ?, ?, 'Active', ?, ?)"
            )
            .bind(&id)
            .bind(&parts[1])
            .bind(&parts[2])
            .bind(&parts[3])
            .bind(&now)
            .bind(&now)
            .execute(pool)
            .await
            {
                warn!(code = %parts[1], error = %e, "Failed to import account");
            } else {
                count += 1;
            }
        }
    }
    Ok(count)
}

async fn import_employees(pool: &sqlx::SqlitePool, lines: &[&str]) -> crate::error::ApiResult<usize> {
    let mut count = 0;
    for line in lines.iter().skip(1) {
        if line.trim().is_empty() {
            continue;
        }
        let parts = parse_csv_line(line);
        if parts.len() >= 4 {
            let id = uuid::Uuid::new_v4().to_string();
            let now = chrono::Utc::now().to_rfc3339();
            if let Err(e) = sqlx::query(
                "INSERT OR IGNORE INTO employees (id, employee_number, first_name, last_name, email, status, created_at, updated_at, hire_date, birth_date)
                 VALUES (?, ?, ?, ?, '', 'Active', ?, ?, date('now'), date('now'))"
            )
            .bind(&id)
            .bind(&parts[1])
            .bind(&parts[2])
            .bind(&parts[3])
            .bind(&now)
            .bind(&now)
            .execute(pool)
            .await
            {
                warn!(employee_number = %parts[1], error = %e, "Failed to import employee");
            } else {
                count += 1;
            }
        }
    }
    Ok(count)
}
