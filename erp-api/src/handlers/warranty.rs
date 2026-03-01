use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::error::ApiResult;
use crate::db::AppState;
use crate::handlers::auth::AuthUser;
use erp_warranty::{
    WarrantyService, SqliteWarrantyRepository,
    WarrantyPolicy, ProductWarranty, WarrantyClaim, WarrantyClaimLine, WarrantyClaimLabor,
    WarrantyExtension, WarrantyAnalytics,
    CreateWarrantyPolicyRequest, CreateProductWarrantyRequest, CreateWarrantyClaimRequest,
    AddClaimLineRequest, AddClaimLaborRequest, ResolveClaimRequest, TransferWarrantyRequest,
    ExtendWarrantyRequest,
    WarrantyType, WarrantyDurationUnit, WarrantyStatus, WarrantyClaimStatus, ClaimResolutionType,
};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i32>,
    pub limit: Option<i32>,
}

#[derive(Deserialize)]
pub struct WarrantyFilterQuery {
    pub customer_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Deserialize)]
pub struct ClaimFilterQuery {
    pub customer_id: Option<Uuid>,
    pub warranty_id: Option<Uuid>,
    pub status: Option<String>,
}

#[derive(Serialize)]
pub struct WarrantyPolicyResponse {
    pub id: String,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub warranty_type: String,
    pub duration_value: i32,
    pub duration_unit: String,
    pub coverage_percentage: f64,
    pub labor_covered: bool,
    pub parts_covered: bool,
    pub on_site_service: bool,
    pub max_claims: Option<i32>,
    pub deductible_amount: i64,
    pub terms_and_conditions: Option<String>,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

impl From<WarrantyPolicy> for WarrantyPolicyResponse {
    fn from(p: WarrantyPolicy) -> Self {
        Self {
            id: p.base.id.to_string(),
            code: p.code,
            name: p.name,
            description: p.description,
            warranty_type: format!("{:?}", p.warranty_type),
            duration_value: p.duration_value,
            duration_unit: format!("{:?}", p.duration_unit),
            coverage_percentage: p.coverage_percentage,
            labor_covered: p.labor_covered,
            parts_covered: p.parts_covered,
            on_site_service: p.on_site_service,
            max_claims: p.max_claims,
            deductible_amount: p.deductible_amount,
            terms_and_conditions: p.terms_and_conditions,
            status: format!("{:?}", p.status),
            created_at: p.base.created_at.to_rfc3339(),
            updated_at: p.base.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct ProductWarrantyResponse {
    pub id: String,
    pub warranty_number: String,
    pub policy_id: String,
    pub product_id: String,
    pub customer_id: String,
    pub sales_order_id: Option<String>,
    pub sales_order_line_id: Option<String>,
    pub serial_number: Option<String>,
    pub lot_number: Option<String>,
    pub purchase_date: String,
    pub activation_date: Option<String>,
    pub start_date: String,
    pub end_date: String,
    pub status: String,
    pub transferred_to_customer_id: Option<String>,
    pub transferred_at: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<ProductWarranty> for ProductWarrantyResponse {
    fn from(w: ProductWarranty) -> Self {
        Self {
            id: w.base.id.to_string(),
            warranty_number: w.warranty_number,
            policy_id: w.policy_id.to_string(),
            product_id: w.product_id.to_string(),
            customer_id: w.customer_id.to_string(),
            sales_order_id: w.sales_order_id.map(|id| id.to_string()),
            sales_order_line_id: w.sales_order_line_id.map(|id| id.to_string()),
            serial_number: w.serial_number,
            lot_number: w.lot_number,
            purchase_date: w.purchase_date.to_rfc3339(),
            activation_date: w.activation_date.map(|d| d.to_rfc3339()),
            start_date: w.start_date.to_rfc3339(),
            end_date: w.end_date.to_rfc3339(),
            status: format!("{:?}", w.status),
            transferred_to_customer_id: w.transferred_to_customer_id.map(|id| id.to_string()),
            transferred_at: w.transferred_at.map(|d| d.to_rfc3339()),
            notes: w.notes,
            created_at: w.base.created_at.to_rfc3339(),
            updated_at: w.base.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct WarrantyClaimResponse {
    pub id: String,
    pub claim_number: String,
    pub product_warranty_id: String,
    pub customer_id: String,
    pub reported_date: String,
    pub issue_description: String,
    pub issue_category: Option<String>,
    pub symptom_codes: Option<String>,
    pub status: String,
    pub priority: i32,
    pub assigned_to: Option<String>,
    pub assigned_at: Option<String>,
    pub resolution_type: Option<String>,
    pub resolution_notes: Option<String>,
    pub resolved_at: Option<String>,
    pub resolved_by: Option<String>,
    pub customer_notified: bool,
    pub notification_date: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<WarrantyClaim> for WarrantyClaimResponse {
    fn from(c: WarrantyClaim) -> Self {
        Self {
            id: c.base.id.to_string(),
            claim_number: c.claim_number,
            product_warranty_id: c.product_warranty_id.to_string(),
            customer_id: c.customer_id.to_string(),
            reported_date: c.reported_date.to_rfc3339(),
            issue_description: c.issue_description,
            issue_category: c.issue_category,
            symptom_codes: c.symptom_codes,
            status: format!("{:?}", c.status),
            priority: c.priority,
            assigned_to: c.assigned_to.map(|id| id.to_string()),
            assigned_at: c.assigned_at.map(|d| d.to_rfc3339()),
            resolution_type: c.resolution_type.map(|r| format!("{:?}", r)),
            resolution_notes: c.resolution_notes,
            resolved_at: c.resolved_at.map(|d| d.to_rfc3339()),
            resolved_by: c.resolved_by.map(|id| id.to_string()),
            customer_notified: c.customer_notified,
            notification_date: c.notification_date.map(|d| d.to_rfc3339()),
            created_at: c.base.created_at.to_rfc3339(),
            updated_at: c.base.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct ClaimLineResponse {
    pub id: String,
    pub claim_id: String,
    pub product_id: String,
    pub description: String,
    pub quantity: i64,
    pub unit_cost: i64,
    pub total_cost: i64,
    pub coverage_percentage: f64,
    pub covered_amount: i64,
    pub customer_amount: i64,
    pub created_at: String,
}

impl From<WarrantyClaimLine> for ClaimLineResponse {
    fn from(l: WarrantyClaimLine) -> Self {
        Self {
            id: l.id.to_string(),
            claim_id: l.claim_id.to_string(),
            product_id: l.product_id.to_string(),
            description: l.description,
            quantity: l.quantity,
            unit_cost: l.unit_cost,
            total_cost: l.total_cost,
            coverage_percentage: l.coverage_percentage,
            covered_amount: l.covered_amount,
            customer_amount: l.customer_amount,
            created_at: l.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct ClaimLaborResponse {
    pub id: String,
    pub claim_id: String,
    pub technician_id: Option<String>,
    pub work_description: String,
    pub labor_hours: f64,
    pub hourly_rate: i64,
    pub total_cost: i64,
    pub covered_amount: i64,
    pub customer_amount: i64,
    pub work_date: String,
    pub created_at: String,
}

impl From<WarrantyClaimLabor> for ClaimLaborResponse {
    fn from(l: WarrantyClaimLabor) -> Self {
        Self {
            id: l.id.to_string(),
            claim_id: l.claim_id.to_string(),
            technician_id: l.technician_id.map(|id| id.to_string()),
            work_description: l.work_description,
            labor_hours: l.labor_hours,
            hourly_rate: l.hourly_rate,
            total_cost: l.total_cost,
            covered_amount: l.covered_amount,
            customer_amount: l.customer_amount,
            work_date: l.work_date.to_rfc3339(),
            created_at: l.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct WarrantyExtensionResponse {
    pub id: String,
    pub product_warranty_id: String,
    pub policy_id: String,
    pub extension_date: String,
    pub additional_duration_value: i32,
    pub additional_duration_unit: String,
    pub new_end_date: String,
    pub cost: i64,
    pub invoice_id: Option<String>,
    pub status: String,
    pub created_at: String,
}

impl From<WarrantyExtension> for WarrantyExtensionResponse {
    fn from(e: WarrantyExtension) -> Self {
        Self {
            id: e.base.id.to_string(),
            product_warranty_id: e.product_warranty_id.to_string(),
            policy_id: e.policy_id.to_string(),
            extension_date: e.extension_date.to_rfc3339(),
            additional_duration_value: e.additional_duration_value,
            additional_duration_unit: format!("{:?}", e.additional_duration_unit),
            new_end_date: e.new_end_date.to_rfc3339(),
            cost: e.cost,
            invoice_id: e.invoice_id.map(|id| id.to_string()),
            status: format!("{:?}", e.status),
            created_at: e.base.created_at.to_rfc3339(),
        }
    }
}

#[derive(Deserialize)]
pub struct CreatePolicyRequestDto {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    #[serde(default)]
    pub warranty_type: String,
    pub duration_value: i32,
    #[serde(default)]
    pub duration_unit: String,
    pub coverage_percentage: f64,
    #[serde(default = "default_true")]
    pub labor_covered: bool,
    #[serde(default = "default_true")]
    pub parts_covered: bool,
    #[serde(default)]
    pub on_site_service: bool,
    pub max_claims: Option<i32>,
    #[serde(default)]
    pub deductible_amount: i64,
    pub terms_and_conditions: Option<String>,
}

fn default_true() -> bool { true }

#[derive(Deserialize)]
pub struct CreateProductWarrantyRequestDto {
    pub policy_id: Uuid,
    pub product_id: Uuid,
    pub customer_id: Uuid,
    pub sales_order_id: Option<Uuid>,
    pub sales_order_line_id: Option<Uuid>,
    pub serial_number: Option<String>,
    pub lot_number: Option<String>,
    pub purchase_date: String,
    pub activation_date: Option<String>,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateClaimRequestDto {
    pub product_warranty_id: Uuid,
    pub customer_id: Uuid,
    pub reported_date: String,
    pub issue_description: String,
    pub issue_category: Option<String>,
    pub symptom_codes: Option<String>,
    #[serde(default = "default_priority")]
    pub priority: i32,
}

fn default_priority() -> i32 { 3 }

#[derive(Deserialize)]
pub struct AddClaimLineRequestDto {
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
    pub unit_cost: i64,
    pub coverage_percentage: f64,
}

#[derive(Deserialize)]
pub struct AddClaimLaborRequestDto {
    pub technician_id: Option<Uuid>,
    pub work_description: String,
    pub labor_hours: f64,
    pub hourly_rate: i64,
    pub work_date: String,
}

#[derive(Deserialize)]
pub struct ResolveClaimRequestDto {
    pub resolution_type: String,
    pub resolution_notes: Option<String>,
}

#[derive(Deserialize)]
pub struct TransferWarrantyRequestDto {
    pub new_customer_id: Uuid,
    pub notes: Option<String>,
}

#[derive(Deserialize)]
pub struct ExtendWarrantyRequestDto {
    pub policy_id: Uuid,
    pub additional_duration_value: i32,
    pub additional_duration_unit: String,
    pub cost: i64,
    pub invoice_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct VoidWarrantyRequestDto {
    pub reason: String,
}

#[derive(Deserialize)]
pub struct RejectClaimRequestDto {
    pub reason: String,
}

#[derive(Deserialize)]
pub struct AssignClaimRequestDto {
    pub assigned_to: Uuid,
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/policies", axum::routing::get(list_policies).post(create_policy))
        .route("/policies/:id", axum::routing::get(get_policy).put(update_policy).delete(delete_policy))
        .route("/warranties", axum::routing::get(list_warranties).post(create_warranty))
        .route("/warranties/expiring", axum::routing::get(list_expiring_warranties))
        .route("/warranties/:id", axum::routing::get(get_warranty))
        .route("/warranties/:id/transfer", axum::routing::post(transfer_warranty))
        .route("/warranties/:id/void", axum::routing::post(void_warranty))
        .route("/warranties/:id/extend", axum::routing::post(extend_warranty))
        .route("/warranties/:id/extensions", axum::routing::get(list_warranty_extensions))
        .route("/claims", axum::routing::get(list_claims).post(create_claim))
        .route("/claims/:id", axum::routing::get(get_claim))
        .route("/claims/:id/assign", axum::routing::post(assign_claim))
        .route("/claims/:id/approve", axum::routing::post(approve_claim))
        .route("/claims/:id/reject", axum::routing::post(reject_claim))
        .route("/claims/:id/start", axum::routing::post(start_claim_work))
        .route("/claims/:id/resolve", axum::routing::post(resolve_claim))
        .route("/claims/:id/lines", axum::routing::get(list_claim_lines).post(add_claim_line))
        .route("/claims/:id/labor", axum::routing::get(list_claim_labor).post(add_claim_labor))
        .route("/analytics", axum::routing::get(get_analytics))
}

fn parse_warranty_type(s: &str) -> WarrantyType {
    match s {
        "Extended" => WarrantyType::Extended,
        "Lifetime" => WarrantyType::Lifetime,
        "ProRated" => WarrantyType::ProRated,
        _ => WarrantyType::Standard,
    }
}

fn parse_duration_unit(s: &str) -> WarrantyDurationUnit {
    match s {
        "Days" => WarrantyDurationUnit::Days,
        "Years" => WarrantyDurationUnit::Years,
        _ => WarrantyDurationUnit::Months,
    }
}

fn parse_claim_status(s: &str) -> Option<WarrantyClaimStatus> {
    match s {
        "Submitted" => Some(WarrantyClaimStatus::Submitted),
        "UnderReview" => Some(WarrantyClaimStatus::UnderReview),
        "Approved" => Some(WarrantyClaimStatus::Approved),
        "Rejected" => Some(WarrantyClaimStatus::Rejected),
        "InProgress" => Some(WarrantyClaimStatus::InProgress),
        "Completed" => Some(WarrantyClaimStatus::Completed),
        "Cancelled" => Some(WarrantyClaimStatus::Cancelled),
        _ => None,
    }
}

fn parse_resolution_type(s: &str) -> ClaimResolutionType {
    match s {
        "Replacement" => ClaimResolutionType::Replacement,
        "Refund" => ClaimResolutionType::Refund,
        "Credit" => ClaimResolutionType::Credit,
        "PartialRefund" => ClaimResolutionType::PartialRefund,
        "Denied" => ClaimResolutionType::Denied,
        _ => ClaimResolutionType::Repair,
    }
}

pub async fn create_policy(
    State(_state): State<AppState>,
    Json(req): Json<CreatePolicyRequestDto>,
) -> ApiResult<Json<WarrantyPolicyResponse>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let policy = svc.create_policy(CreateWarrantyPolicyRequest {
        code: req.code,
        name: req.name,
        description: req.description,
        warranty_type: parse_warranty_type(&req.warranty_type),
        duration_value: req.duration_value,
        duration_unit: parse_duration_unit(&req.duration_unit),
        coverage_percentage: req.coverage_percentage,
        labor_covered: req.labor_covered,
        parts_covered: req.parts_covered,
        on_site_service: req.on_site_service,
        max_claims: req.max_claims,
        deductible_amount: req.deductible_amount,
        terms_and_conditions: req.terms_and_conditions,
    }).await?;
    
    Ok(Json(WarrantyPolicyResponse::from(policy)))
}

pub async fn get_policy(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<WarrantyPolicyResponse>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let policy = svc.get_policy(id).await?
        .ok_or_else(|| anyhow::anyhow!("Policy not found"))?;
    
    Ok(Json(WarrantyPolicyResponse::from(policy)))
}

pub async fn list_policies(
    State(_state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> ApiResult<Json<ApiResponse<Vec<WarrantyPolicyResponse>>>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let policies = svc.list_policies(None).await?;
    let items: Vec<WarrantyPolicyResponse> = policies.into_iter().map(WarrantyPolicyResponse::from).collect();
    
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn update_policy(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(_req): Json<CreatePolicyRequestDto>,
) -> ApiResult<Json<WarrantyPolicyResponse>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let policy = svc.get_policy(id).await?
        .ok_or_else(|| anyhow::anyhow!("Policy not found"))?;
    
    Ok(Json(WarrantyPolicyResponse::from(policy)))
}

pub async fn delete_policy(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    svc.delete_policy(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn create_warranty(
    State(_state): State<AppState>,
    Json(req): Json<CreateProductWarrantyRequestDto>,
) -> ApiResult<Json<ProductWarrantyResponse>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let purchase_date = chrono::DateTime::parse_from_rfc3339(&req.purchase_date)
        .map_err(|e| anyhow::anyhow!("Invalid purchase date: {}", e))?
        .with_timezone(&chrono::Utc);
    
    let activation_date = req.activation_date
        .map(|d| chrono::DateTime::parse_from_rfc3339(&d))
        .transpose()
        .map_err(|e| anyhow::anyhow!("Invalid activation date: {}", e))?
        .map(|d| d.with_timezone(&chrono::Utc));
    
    let warranty = svc.create_product_warranty(CreateProductWarrantyRequest {
        policy_id: req.policy_id,
        product_id: req.product_id,
        customer_id: req.customer_id,
        sales_order_id: req.sales_order_id,
        sales_order_line_id: req.sales_order_line_id,
        serial_number: req.serial_number,
        lot_number: req.lot_number,
        purchase_date,
        activation_date,
        notes: req.notes,
    }).await?;
    
    Ok(Json(ProductWarrantyResponse::from(warranty)))
}

pub async fn get_warranty(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ProductWarrantyResponse>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let warranty = svc.get_product_warranty(id).await?
        .ok_or_else(|| anyhow::anyhow!("Warranty not found"))?;
    
    Ok(Json(ProductWarrantyResponse::from(warranty)))
}

pub async fn list_warranties(
    State(_state): State<AppState>,
    Query(query): Query<WarrantyFilterQuery>,
) -> ApiResult<Json<ApiResponse<Vec<ProductWarrantyResponse>>>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let warranties = svc.list_product_warranties(query.customer_id, query.product_id, query.status).await?;
    let items: Vec<ProductWarrantyResponse> = warranties.into_iter().map(ProductWarrantyResponse::from).collect();
    
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn list_expiring_warranties(
    State(_state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> ApiResult<Json<ApiResponse<Vec<ProductWarrantyResponse>>>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let days = query.limit.unwrap_or(30);
    let warranties = svc.list_expiring_warranties(days).await?;
    let items: Vec<ProductWarrantyResponse> = warranties.into_iter().map(ProductWarrantyResponse::from).collect();
    
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn transfer_warranty(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<TransferWarrantyRequestDto>,
) -> ApiResult<Json<ProductWarrantyResponse>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let warranty = svc.transfer_warranty(id, TransferWarrantyRequest {
        new_customer_id: req.new_customer_id,
        notes: req.notes,
    }).await?;
    
    Ok(Json(ProductWarrantyResponse::from(warranty)))
}

pub async fn void_warranty(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<VoidWarrantyRequestDto>,
) -> ApiResult<Json<ProductWarrantyResponse>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let warranty = svc.void_warranty(id, req.reason).await?;
    
    Ok(Json(ProductWarrantyResponse::from(warranty)))
}

pub async fn extend_warranty(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<ExtendWarrantyRequestDto>,
) -> ApiResult<Json<WarrantyExtensionResponse>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let extension = svc.extend_warranty(id, ExtendWarrantyRequest {
        policy_id: req.policy_id,
        additional_duration_value: req.additional_duration_value,
        additional_duration_unit: parse_duration_unit(&req.additional_duration_unit),
        cost: req.cost,
        invoice_id: req.invoice_id,
    }).await?;
    
    Ok(Json(WarrantyExtensionResponse::from(extension)))
}

pub async fn list_warranty_extensions(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<Vec<WarrantyExtensionResponse>>>> {
    let repo = SqliteWarrantyRepository::new();
    
    let extensions = repo.list_extensions(id).await.map_err(|e| anyhow::anyhow!(e))?;
    let items: Vec<WarrantyExtensionResponse> = extensions.into_iter().map(WarrantyExtensionResponse::from).collect();
    
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn create_claim(
    State(_state): State<AppState>,
    Json(req): Json<CreateClaimRequestDto>,
) -> ApiResult<Json<WarrantyClaimResponse>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let reported_date = chrono::DateTime::parse_from_rfc3339(&req.reported_date)
        .map_err(|e| anyhow::anyhow!("Invalid reported date: {}", e))?
        .with_timezone(&chrono::Utc);
    
    let claim = svc.create_claim(CreateWarrantyClaimRequest {
        product_warranty_id: req.product_warranty_id,
        customer_id: req.customer_id,
        reported_date,
        issue_description: req.issue_description,
        issue_category: req.issue_category,
        symptom_codes: req.symptom_codes,
        priority: req.priority,
    }).await?;
    
    Ok(Json(WarrantyClaimResponse::from(claim)))
}

pub async fn get_claim(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<WarrantyClaimResponse>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let claim = svc.get_claim(id).await?
        .ok_or_else(|| anyhow::anyhow!("Claim not found"))?;
    
    Ok(Json(WarrantyClaimResponse::from(claim)))
}

pub async fn list_claims(
    State(_state): State<AppState>,
    Query(query): Query<ClaimFilterQuery>,
) -> ApiResult<Json<ApiResponse<Vec<WarrantyClaimResponse>>>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let status = query.status.and_then(|s| parse_claim_status(&s));
    let claims = svc.list_claims(query.customer_id, query.warranty_id, status).await?;
    let items: Vec<WarrantyClaimResponse> = claims.into_iter().map(WarrantyClaimResponse::from).collect();
    
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn assign_claim(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<AssignClaimRequestDto>,
) -> ApiResult<Json<WarrantyClaimResponse>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let claim = svc.assign_claim(id, req.assigned_to).await?;
    
    Ok(Json(WarrantyClaimResponse::from(claim)))
}

pub async fn approve_claim(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<WarrantyClaimResponse>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let claim = svc.approve_claim(id).await?;
    
    Ok(Json(WarrantyClaimResponse::from(claim)))
}

pub async fn reject_claim(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<RejectClaimRequestDto>,
) -> ApiResult<Json<WarrantyClaimResponse>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let claim = svc.reject_claim(id, req.reason).await?;
    
    Ok(Json(WarrantyClaimResponse::from(claim)))
}

pub async fn start_claim_work(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<WarrantyClaimResponse>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let claim = svc.start_claim_work(id).await?;
    
    Ok(Json(WarrantyClaimResponse::from(claim)))
}

pub async fn resolve_claim(
    State(_state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Path(id): Path<Uuid>,
    Json(req): Json<ResolveClaimRequestDto>,
) -> ApiResult<Json<WarrantyClaimResponse>> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let claim = svc.resolve_claim(id, ResolveClaimRequest {
        resolution_type: parse_resolution_type(&req.resolution_type),
        resolution_notes: req.resolution_notes,
    }, user_id).await?;
    
    Ok(Json(WarrantyClaimResponse::from(claim)))
}

pub async fn add_claim_line(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<AddClaimLineRequestDto>,
) -> ApiResult<Json<ClaimLineResponse>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let line = svc.add_claim_line(AddClaimLineRequest {
        claim_id: id,
        product_id: req.product_id,
        description: req.description,
        quantity: req.quantity,
        unit_cost: req.unit_cost,
        coverage_percentage: req.coverage_percentage,
    }).await?;
    
    Ok(Json(ClaimLineResponse::from(line)))
}

pub async fn list_claim_lines(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<Vec<ClaimLineResponse>>>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let lines = svc.list_claim_lines(id).await?;
    let items: Vec<ClaimLineResponse> = lines.into_iter().map(ClaimLineResponse::from).collect();
    
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn add_claim_labor(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(req): Json<AddClaimLaborRequestDto>,
) -> ApiResult<Json<ClaimLaborResponse>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let work_date = chrono::DateTime::parse_from_rfc3339(&req.work_date)
        .map_err(|e| anyhow::anyhow!("Invalid work date: {}", e))?
        .with_timezone(&chrono::Utc);
    
    let labor = svc.add_claim_labor(AddClaimLaborRequest {
        claim_id: id,
        technician_id: req.technician_id,
        work_description: req.work_description,
        labor_hours: req.labor_hours,
        hourly_rate: req.hourly_rate,
        work_date,
    }).await?;
    
    Ok(Json(ClaimLaborResponse::from(labor)))
}

pub async fn list_claim_labor(
    State(_state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<Vec<ClaimLaborResponse>>>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let labors = svc.list_claim_labors(id).await?;
    let items: Vec<ClaimLaborResponse> = labors.into_iter().map(ClaimLaborResponse::from).collect();
    
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn get_analytics(
    State(_state): State<AppState>,
) -> ApiResult<Json<WarrantyAnalytics>> {
    let repo = SqliteWarrantyRepository::new();
    let svc = WarrantyService::new(repo);
    
    let analytics = svc.get_analytics().await?;
    
    Ok(Json(analytics))
}
