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
use erp_credit::{CreditService, CreditCheckRequest, CreditCheckResponse, CustomerCreditProfile, CreditTransaction, CreditHold, CreditLimitChange, CreditAlert, CreditSummary};

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

#[derive(Deserialize)]
pub struct CheckCreditRequest {
    pub customer_id: Uuid,
    pub order_id: Option<Uuid>,
    pub order_amount: i64,
    pub currency: String,
}

#[derive(Deserialize)]
pub struct UpdateCreditLimitRequest {
    pub credit_limit: i64,
    pub reason: String,
}

#[derive(Deserialize)]
pub struct PlaceHoldRequest {
    pub reason: String,
}

#[derive(Deserialize)]
pub struct ReleaseHoldRequest {
    pub override_reason: String,
}

#[derive(Deserialize)]
pub struct RecordInvoiceRequest {
    pub customer_id: Uuid,
    pub invoice_id: Uuid,
    pub invoice_number: String,
    pub amount: i64,
}

#[derive(Deserialize)]
pub struct RecordPaymentRequest {
    pub customer_id: Uuid,
    pub invoice_id: Option<Uuid>,
    pub amount: i64,
}

#[derive(Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub limit: Option<i64>,
}

#[derive(Serialize)]
pub struct CreditProfileResponse {
    pub id: String,
    pub customer_id: String,
    pub credit_limit: i64,
    pub credit_used: i64,
    pub available_credit: i64,
    pub outstanding_invoices: i64,
    pub pending_orders: i64,
    pub overdue_amount: i64,
    pub overdue_days_avg: i32,
    pub credit_score: Option<i32>,
    pub risk_level: String,
    pub payment_history_score: Option<f64>,
    pub last_credit_review: Option<String>,
    pub next_review_date: Option<String>,
    pub auto_hold_enabled: bool,
    pub hold_threshold_percent: i32,
    pub status: String,
    pub utilization_percent: f64,
    pub is_on_hold: bool,
    pub created_at: String,
    pub updated_at: String,
}

impl From<CustomerCreditProfile> for CreditProfileResponse {
    fn from(p: CustomerCreditProfile) -> Self {
        let utilization = if p.credit_limit > 0 {
            (p.credit_used as f64 / p.credit_limit as f64) * 100.0
        } else {
            0.0
        };
        Self {
            id: p.base.id.to_string(),
            customer_id: p.customer_id.to_string(),
            credit_limit: p.credit_limit,
            credit_used: p.credit_used,
            available_credit: p.available_credit,
            outstanding_invoices: p.outstanding_invoices,
            pending_orders: p.pending_orders,
            overdue_amount: p.overdue_amount,
            overdue_days_avg: p.overdue_days_avg,
            credit_score: p.credit_score,
            risk_level: format!("{:?}", p.risk_level),
            payment_history_score: p.payment_history_score,
            last_credit_review: p.last_credit_review.map(|d| d.to_rfc3339()),
            next_review_date: p.next_review_date.map(|d| d.to_rfc3339()),
            auto_hold_enabled: p.auto_hold_enabled,
            hold_threshold_percent: p.hold_threshold_percent,
            status: format!("{:?}", p.status),
            utilization_percent: utilization,
            is_on_hold: p.credit_used > p.credit_limit,
            created_at: p.created_at.to_rfc3339(),
            updated_at: p.updated_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct CreditTransactionResponse {
    pub id: String,
    pub profile_id: String,
    pub customer_id: String,
    pub transaction_type: String,
    pub amount: i64,
    pub previous_credit_used: i64,
    pub new_credit_used: i64,
    pub reference_type: Option<String>,
    pub reference_id: Option<String>,
    pub reference_number: Option<String>,
    pub description: Option<String>,
    pub created_by: Option<String>,
    pub created_at: String,
}

impl From<CreditTransaction> for CreditTransactionResponse {
    fn from(t: CreditTransaction) -> Self {
        Self {
            id: t.id.to_string(),
            profile_id: t.profile_id.to_string(),
            customer_id: t.customer_id.to_string(),
            transaction_type: format!("{:?}", t.transaction_type),
            amount: t.amount,
            previous_credit_used: t.previous_credit_used,
            new_credit_used: t.new_credit_used,
            reference_type: t.reference_type,
            reference_id: t.reference_id.map(|id| id.to_string()),
            reference_number: t.reference_number,
            description: t.description,
            created_by: t.created_by.map(|id| id.to_string()),
            created_at: t.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct CreditHoldResponse {
    pub id: String,
    pub profile_id: String,
    pub customer_id: String,
    pub hold_type: String,
    pub reason: String,
    pub amount_over_limit: i64,
    pub related_order_id: Option<String>,
    pub related_invoice_id: Option<String>,
    pub status: String,
    pub placed_by: Option<String>,
    pub placed_at: String,
    pub released_by: Option<String>,
    pub released_at: Option<String>,
    pub override_reason: Option<String>,
    pub notes: Option<String>,
    pub created_at: String,
}

impl From<CreditHold> for CreditHoldResponse {
    fn from(h: CreditHold) -> Self {
        Self {
            id: h.id.to_string(),
            profile_id: h.profile_id.to_string(),
            customer_id: h.customer_id.to_string(),
            hold_type: format!("{:?}", h.hold_type),
            reason: h.reason,
            amount_over_limit: h.amount_over_limit,
            related_order_id: h.related_order_id.map(|id| id.to_string()),
            related_invoice_id: h.related_invoice_id.map(|id| id.to_string()),
            status: format!("{:?}", h.status),
            placed_by: h.placed_by.map(|id| id.to_string()),
            placed_at: h.placed_at.to_rfc3339(),
            released_by: h.released_by.map(|id| id.to_string()),
            released_at: h.released_at.map(|d| d.to_rfc3339()),
            override_reason: h.override_reason,
            notes: h.notes,
            created_at: h.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct CreditLimitChangeResponse {
    pub id: String,
    pub profile_id: String,
    pub customer_id: String,
    pub previous_limit: i64,
    pub new_limit: i64,
    pub change_reason: String,
    pub approved_by: Option<String>,
    pub approved_at: Option<String>,
    pub effective_date: String,
    pub created_by: String,
    pub created_at: String,
}

impl From<CreditLimitChange> for CreditLimitChangeResponse {
    fn from(c: CreditLimitChange) -> Self {
        Self {
            id: c.id.to_string(),
            profile_id: c.profile_id.to_string(),
            customer_id: c.customer_id.to_string(),
            previous_limit: c.previous_limit,
            new_limit: c.new_limit,
            change_reason: c.change_reason,
            approved_by: c.approved_by.map(|id| id.to_string()),
            approved_at: c.approved_at.map(|d| d.to_rfc3339()),
            effective_date: c.effective_date.to_rfc3339(),
            created_by: c.created_by.to_string(),
            created_at: c.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct CreditAlertResponse {
    pub id: String,
    pub profile_id: String,
    pub customer_id: String,
    pub alert_type: String,
    pub severity: String,
    pub message: String,
    pub threshold_value: i64,
    pub actual_value: i64,
    pub is_read: bool,
    pub acknowledged_by: Option<String>,
    pub acknowledged_at: Option<String>,
    pub created_at: String,
}

impl From<CreditAlert> for CreditAlertResponse {
    fn from(a: CreditAlert) -> Self {
        Self {
            id: a.id.to_string(),
            profile_id: a.profile_id.to_string(),
            customer_id: a.customer_id.to_string(),
            alert_type: format!("{:?}", a.alert_type),
            severity: format!("{:?}", a.severity),
            message: a.message,
            threshold_value: a.threshold_value,
            actual_value: a.actual_value,
            is_read: a.is_read,
            acknowledged_by: a.acknowledged_by.map(|id| id.to_string()),
            acknowledged_at: a.acknowledged_at.map(|d| d.to_rfc3339()),
            created_at: a.created_at.to_rfc3339(),
        }
    }
}

#[derive(Serialize)]
pub struct CreditSummaryResponse {
    pub total_customers: i64,
    pub total_credit_limit: i64,
    pub total_credit_used: i64,
    pub total_available_credit: i64,
    pub total_overdue: i64,
    pub customers_on_hold: i64,
    pub high_risk_customers: i64,
    pub avg_utilization_percent: f64,
}

impl From<CreditSummary> for CreditSummaryResponse {
    fn from(s: CreditSummary) -> Self {
        Self {
            total_customers: s.total_customers,
            total_credit_limit: s.total_credit_limit,
            total_credit_used: s.total_credit_used,
            total_available_credit: s.total_available_credit,
            total_overdue: s.total_overdue,
            customers_on_hold: s.customers_on_hold,
            high_risk_customers: s.high_risk_customers,
            avg_utilization_percent: s.avg_utilization_percent,
        }
    }
}

pub async fn check_credit(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Json(req): Json<CheckCreditRequest>,
) -> ApiResult<Json<CreditCheckResponse>> {
    if req.order_amount < 0 {
        return Err(crate::error::ApiError::from(erp_core::Error::validation("Order amount cannot be negative")));
    }
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = CreditService::new();
    let check_req = CreditCheckRequest {
        customer_id: req.customer_id,
        order_id: req.order_id,
        order_amount: req.order_amount,
        currency: req.currency,
    };
    let result = svc.check_credit(&state.pool, check_req, Some(user_id)).await?;
    Ok(Json(result))
}

pub async fn get_profile(
    State(state): State<AppState>,
    Path(customer_id): Path<Uuid>,
) -> ApiResult<Json<CreditProfileResponse>> {
    let svc = CreditService::new();
    let profile = svc.get_profile(&state.pool, customer_id).await?
        .ok_or_else(|| anyhow::anyhow!("Credit profile not found"))?;
    Ok(Json(CreditProfileResponse::from(profile)))
}

pub async fn list_profiles(
    State(state): State<AppState>,
    Query(query): Query<PaginationQuery>,
) -> ApiResult<Json<ApiResponse<Vec<CreditProfileResponse>>>> {
    let svc = CreditService::new();
    let page = query.page.unwrap_or(1);
    let limit = query.limit.unwrap_or(20);
    let profiles = svc.list_profiles(&state.pool, page, limit).await?;
    let items: Vec<CreditProfileResponse> = profiles.into_iter().map(CreditProfileResponse::from).collect();
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn list_on_hold(
    State(state): State<AppState>,
) -> ApiResult<Json<ApiResponse<Vec<CreditProfileResponse>>>> {
    let svc = CreditService::new();
    let profiles = svc.list_on_hold(&state.pool).await?;
    let items: Vec<CreditProfileResponse> = profiles.into_iter().map(CreditProfileResponse::from).collect();
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn list_high_risk(
    State(state): State<AppState>,
) -> ApiResult<Json<ApiResponse<Vec<CreditProfileResponse>>>> {
    let svc = CreditService::new();
    let profiles = svc.list_high_risk(&state.pool).await?;
    let items: Vec<CreditProfileResponse> = profiles.into_iter().map(CreditProfileResponse::from).collect();
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn update_credit_limit(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Path(customer_id): Path<Uuid>,
    Json(req): Json<UpdateCreditLimitRequest>,
) -> ApiResult<Json<CreditProfileResponse>> {
    if req.credit_limit < 0 {
        return Err(crate::error::ApiError::from(erp_core::Error::validation("Credit limit cannot be negative")));
    }
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = CreditService::new();
    let profile = svc.update_credit_limit(&state.pool, customer_id, req.credit_limit, req.reason, user_id).await?;
    Ok(Json(CreditProfileResponse::from(profile)))
}

pub async fn place_hold(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Path(customer_id): Path<Uuid>,
    Json(req): Json<PlaceHoldRequest>,
) -> ApiResult<Json<CreditHoldResponse>> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = CreditService::new();
    let hold = svc.place_manual_hold(&state.pool, customer_id, req.reason, user_id).await?;
    Ok(Json(CreditHoldResponse::from(hold)))
}

pub async fn release_hold(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Path(customer_id): Path<Uuid>,
    Json(req): Json<ReleaseHoldRequest>,
) -> ApiResult<StatusCode> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = CreditService::new();
    svc.release_hold(&state.pool, customer_id, req.override_reason, user_id).await?;
    Ok(StatusCode::OK)
}

pub async fn list_transactions(
    State(state): State<AppState>,
    Path(customer_id): Path<Uuid>,
    Query(query): Query<PaginationQuery>,
) -> ApiResult<Json<ApiResponse<Vec<CreditTransactionResponse>>>> {
    let svc = CreditService::new();
    let limit = query.limit.unwrap_or(50);
    let txns = svc.list_transactions(&state.pool, customer_id, limit).await?;
    let items: Vec<CreditTransactionResponse> = txns.into_iter().map(CreditTransactionResponse::from).collect();
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn list_holds(
    State(state): State<AppState>,
    Path(customer_id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<Vec<CreditHoldResponse>>>> {
    let svc = CreditService::new();
    let holds = svc.list_holds(&state.pool, customer_id).await?;
    let items: Vec<CreditHoldResponse> = holds.into_iter().map(CreditHoldResponse::from).collect();
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn list_limit_changes(
    State(state): State<AppState>,
    Path(customer_id): Path<Uuid>,
) -> ApiResult<Json<ApiResponse<Vec<CreditLimitChangeResponse>>>> {
    let svc = CreditService::new();
    let changes = svc.list_limit_changes(&state.pool, customer_id).await?;
    let items: Vec<CreditLimitChangeResponse> = changes.into_iter().map(CreditLimitChangeResponse::from).collect();
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn record_invoice(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Json(req): Json<RecordInvoiceRequest>,
) -> ApiResult<Json<CreditProfileResponse>> {
    if req.amount < 0 {
        return Err(crate::error::ApiError::from(erp_core::Error::validation("Invoice amount cannot be negative")));
    }
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = CreditService::new();
    let profile = svc.record_invoice(&state.pool, req.customer_id, req.invoice_id, req.invoice_number, req.amount, Some(user_id)).await?;
    Ok(Json(CreditProfileResponse::from(profile)))
}

pub async fn record_payment(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Json(req): Json<RecordPaymentRequest>,
) -> ApiResult<Json<CreditProfileResponse>> {
    if req.amount < 0 {
        return Err(crate::error::ApiError::from(erp_core::Error::validation("Payment amount cannot be negative")));
    }
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = CreditService::new();
    let profile = svc.record_payment(&state.pool, req.customer_id, req.invoice_id, req.amount, Some(user_id)).await?;
    Ok(Json(CreditProfileResponse::from(profile)))
}

pub async fn get_summary(
    State(state): State<AppState>,
) -> ApiResult<Json<CreditSummaryResponse>> {
    let svc = CreditService::new();
    let summary = svc.get_summary(&state.pool).await?;
    Ok(Json(CreditSummaryResponse::from(summary)))
}

pub async fn list_alerts(
    State(state): State<AppState>,
) -> ApiResult<Json<ApiResponse<Vec<CreditAlertResponse>>>> {
    let svc = CreditService::new();
    let alerts = svc.list_unread_alerts(&state.pool).await?;
    let items: Vec<CreditAlertResponse> = alerts.into_iter().map(CreditAlertResponse::from).collect();
    Ok(Json(ApiResponse { success: true, data: items }))
}

pub async fn acknowledge_alert(
    State(state): State<AppState>,
    axum::Extension(auth_user): axum::Extension<AuthUser>,
    Path(alert_id): Path<Uuid>,
) -> ApiResult<StatusCode> {
    let user_id = uuid::Uuid::parse_str(&auth_user.0.user_id)
        .map_err(|_| erp_core::Error::Unauthorized)?;
    let svc = CreditService::new();
    svc.acknowledge_alert(&state.pool, alert_id, user_id).await?;
    Ok(StatusCode::OK)
}
