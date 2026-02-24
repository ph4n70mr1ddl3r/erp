use axum::{
    extract::{Path, State},
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

use crate::db::AppState;
use erp_company::{CompanyService, IntercompanyService, ConsolidationService, CreateCompanyRequest, CreateIntercompanyRequest, CompanyType, ConsolidationMethod};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/companies", get(list_companies).post(create_company))
        .route("/companies/:id", get(get_company))
        .route("/companies/:id/children", get(get_company_children))
        .route("/companies/:id/tree", get(get_company_tree))
        .route("/intercompany", get(list_intercompany).post(create_intercompany))
        .route("/intercompany/pending-eliminations", get(get_pending_eliminations))
        .route("/consolidations", post(create_consolidation))
        .route("/consolidations/:id/run-eliminations", post(run_eliminations))
}

#[derive(Deserialize)]
pub struct CreateCompanyBody {
    pub code: String,
    pub name: String,
    pub legal_name: String,
    #[serde(default)]
    pub company_type: String,
    pub parent_id: Option<Uuid>,
    pub tax_id: Option<String>,
    pub registration_number: Option<String>,
    #[serde(default = "default_currency")]
    pub currency: String,
    #[serde(default = "default_fiscal_year_start")]
    pub fiscal_year_start: i32,
    #[serde(default)]
    pub consolidation_method: String,
    #[serde(default = "default_ownership")]
    pub ownership_percentage: f64,
    pub street: String,
    pub city: String,
    pub state: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub website: Option<String>,
}

fn default_currency() -> String { "USD".to_string() }
fn default_fiscal_year_start() -> i32 { 1 }
fn default_ownership() -> f64 { 100.0 }

async fn create_company(
    State(state): State<AppState>,
    Json(body): Json<CreateCompanyBody>,
) -> Json<serde_json::Value> {
    let req = CreateCompanyRequest {
        code: body.code,
        name: body.name,
        legal_name: body.legal_name,
        company_type: match body.company_type.as_str() {
            "Parent" => CompanyType::Parent,
            "Division" => CompanyType::Division,
            "Branch" => CompanyType::Branch,
            _ => CompanyType::Subsidiary,
        },
        parent_id: body.parent_id,
        tax_id: body.tax_id,
        registration_number: body.registration_number,
        currency: body.currency,
        fiscal_year_start: body.fiscal_year_start,
        consolidation_method: match body.consolidation_method.as_str() {
            "Equity" => ConsolidationMethod::Equity,
            "Proportional" => ConsolidationMethod::Proportional,
            "None" => ConsolidationMethod::None,
            _ => ConsolidationMethod::Full,
        },
        ownership_percentage: body.ownership_percentage,
        street: body.street,
        city: body.city,
        state: body.state,
        postal_code: body.postal_code,
        country: body.country,
        phone: body.phone,
        email: body.email,
        website: body.website,
    };
    match CompanyService::create(&state.pool, req).await {
        Ok(company) => Json(json!({
            "id": company.id,
            "code": company.code,
            "name": company.name,
            "legal_name": company.legal_name,
            "company_type": company.company_type,
            "currency": company.currency,
            "status": company.status
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn list_companies(State(state): State<AppState>) -> Json<serde_json::Value> {
    match CompanyService::list(&state.pool).await {
        Ok(companies) => Json(json!({
            "items": companies.iter().map(|c| json!({
                "id": c.id,
                "code": c.code,
                "name": c.name,
                "company_type": c.company_type,
                "currency": c.currency,
                "status": c.status
            })).collect::<Vec<_>>()
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn get_company(State(state): State<AppState>, Path(id): Path<Uuid>) -> Json<serde_json::Value> {
    match CompanyService::get(&state.pool, id).await {
        Ok(Some(company)) => Json(json!({
            "id": company.id,
            "code": company.code,
            "name": company.name,
            "legal_name": company.legal_name,
            "company_type": company.company_type,
            "parent_id": company.parent_id,
            "currency": company.currency,
            "status": company.status
        })),
        Ok(None) => Json(json!({ "error": "Company not found" })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn get_company_children(State(state): State<AppState>, Path(id): Path<Uuid>) -> Json<serde_json::Value> {
    match CompanyService::get_children(&state.pool, id).await {
        Ok(children) => Json(json!({
            "items": children.iter().map(|c| json!({
                "id": c.id,
                "code": c.code,
                "name": c.name
            })).collect::<Vec<_>>()
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn get_company_tree(State(state): State<AppState>, Path(id): Path<Uuid>) -> Json<serde_json::Value> {
    match CompanyService::get_company_tree(&state.pool, id).await {
        Ok(tree) => Json(json!({
            "items": tree.iter().map(|c| json!({
                "id": c.id,
                "code": c.code,
                "name": c.name
            })).collect::<Vec<_>>()
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct CreateIntercompanyBody {
    pub from_company_id: Uuid,
    pub to_company_id: Uuid,
    pub transaction_type: String,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub amount: i64,
    pub currency: String,
    #[serde(default = "default_exchange_rate")]
    pub exchange_rate: f64,
    pub description: String,
    pub due_date: Option<chrono::DateTime<chrono::Utc>>,
}

fn default_exchange_rate() -> f64 { 1.0 }

async fn create_intercompany(
    State(state): State<AppState>,
    Json(body): Json<CreateIntercompanyBody>,
) -> Json<serde_json::Value> {
    let user_id = Uuid::nil();
    let req = CreateIntercompanyRequest {
        from_company_id: body.from_company_id,
        to_company_id: body.to_company_id,
        transaction_type: body.transaction_type,
        reference_type: body.reference_type,
        reference_id: body.reference_id,
        amount: body.amount,
        currency: body.currency,
        exchange_rate: body.exchange_rate,
        description: body.description,
        due_date: body.due_date,
    };
    match IntercompanyService::create(&state.pool, req, user_id).await {
        Ok(txn) => Json(json!({
            "id": txn.id,
            "transaction_number": txn.transaction_number,
            "from_company_id": txn.from_company_id,
            "to_company_id": txn.to_company_id,
            "amount": txn.amount,
            "currency": txn.currency,
            "status": txn.status
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn list_intercompany(State(state): State<AppState>) -> Json<serde_json::Value> {
    match IntercompanyService::list(&state.pool).await {
        Ok(transactions) => Json(json!({
            "items": transactions.iter().map(|t| json!({
                "id": t.id,
                "transaction_number": t.transaction_number,
                "amount": t.amount,
                "currency": t.currency,
                "status": t.status
            })).collect::<Vec<_>>()
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn get_pending_eliminations(State(state): State<AppState>) -> Json<serde_json::Value> {
    match IntercompanyService::get_pending_eliminations(&state.pool).await {
        Ok(transactions) => Json(json!({
            "items": transactions.iter().map(|t| json!({
                "id": t.id,
                "transaction_number": t.transaction_number,
                "amount": t.amount
            })).collect::<Vec<_>>()
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

#[derive(Deserialize)]
pub struct CreateConsolidationBody {
    pub name: String,
    pub period_start: chrono::DateTime<chrono::Utc>,
    pub period_end: chrono::DateTime<chrono::Utc>,
}

async fn create_consolidation(
    State(state): State<AppState>,
    Json(body): Json<CreateConsolidationBody>,
) -> Json<serde_json::Value> {
    let user_id = Uuid::nil();
    match ConsolidationService::create_consolidation(&state.pool, body.name, body.period_start, body.period_end, user_id).await {
        Ok(consolidation) => Json(json!({
            "id": consolidation.id,
            "name": consolidation.name,
            "status": consolidation.status
        })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}

async fn run_eliminations(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Json<serde_json::Value> {
    match ConsolidationService::run_eliminations(&state.pool, id).await {
        Ok(total) => Json(json!({ "total_eliminations": total })),
        Err(e) => Json(json!({ "error": e.to_string() })),
    }
}
