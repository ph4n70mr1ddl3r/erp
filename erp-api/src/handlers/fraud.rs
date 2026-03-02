use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use crate::db::AppState;
use crate::error::ApiResult;
use erp_fraud::{FraudService, ReviewAlertRequest, CaseResolution, Evidence, EvidenceType};

#[derive(Deserialize)]
pub struct ListAlertsQuery {
    status: Option<String>,
    severity: Option<String>,
    #[serde(default = "default_limit")]
    limit: i64,
}

fn default_limit() -> i64 { 50 }

#[derive(Deserialize)]
pub struct AnalyticsQuery {
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
}

pub fn routes() -> axum::Router<AppState> {
    axum::Router::new()
        .route("/alerts", axum::routing::get(list_alerts).post(create_alert))
        .route("/alerts/:id", axum::routing::get(get_alert))
        .route("/alerts/:id/review", axum::routing::post(review_alert))
        .route("/alerts/:id/assign", axum::routing::post(assign_alert))
        .route("/alerts/:id/escalate", axum::routing::post(escalate_alert))
        .route("/rules", axum::routing::get(list_rules).post(create_rule))
        .route("/rules/:id", axum::routing::get(get_rule).delete(delete_rule))
        .route("/evaluate", axum::routing::post(evaluate_transaction))
        .route("/cases", axum::routing::get(list_cases).post(create_case))
        .route("/cases/:id", axum::routing::get(get_case))
        .route("/cases/:id/evidence", axum::routing::post(add_evidence))
        .route("/cases/:id/resolve", axum::routing::post(resolve_case))
        .route("/risk/vendor/:vendor_id", axum::routing::get(get_vendor_risk).post(calculate_vendor_risk))
        .route("/risk/employee/:employee_id", axum::routing::get(get_employee_risk).post(calculate_employee_risk))
        .route("/analytics", axum::routing::get(get_analytics))
}

async fn create_alert(
    State(_state): State<AppState>,
    Json(_request): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    let _service = FraudService::new();
    Ok(Json(serde_json::json!({"id": Uuid::new_v4()})))
}

async fn get_alert(
    Path(id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = FraudService::new();
    let alert = service.get_alert(id).await?;
    Ok(Json(serde_json::to_value(alert)?))
}

async fn list_alerts(
    Query(query): Query<ListAlertsQuery>,
    State(_state): State<AppState>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let service = FraudService::new();
    let status = query.status.and_then(|s| match s.as_str() {
        "new" => Some(erp_fraud::AlertStatus::New),
        "under_review" => Some(erp_fraud::AlertStatus::UnderReview),
        "confirmed" => Some(erp_fraud::AlertStatus::Confirmed),
        "false_positive" => Some(erp_fraud::AlertStatus::FalsePositive),
        "escalated" => Some(erp_fraud::AlertStatus::Escalated),
        "resolved" => Some(erp_fraud::AlertStatus::Resolved),
        "closed" => Some(erp_fraud::AlertStatus::Closed),
        _ => None,
    });
    let severity = query.severity.and_then(|s| match s.as_str() {
        "low" => Some(erp_fraud::AlertSeverity::Low),
        "medium" => Some(erp_fraud::AlertSeverity::Medium),
        "high" => Some(erp_fraud::AlertSeverity::High),
        "critical" => Some(erp_fraud::AlertSeverity::Critical),
        _ => None,
    });
    let alerts = service.list_alerts(status, severity, query.limit).await?;
    Ok(Json(alerts.into_iter().map(|a| serde_json::to_value(a).unwrap_or_default()).collect()))
}

#[derive(Deserialize)]
struct ReviewAlertBody {
    status: String,
    resolution_type: Option<String>,
    notes: Option<String>,
}

async fn review_alert(
    Path(id): Path<Uuid>,
    State(_state): State<AppState>,
    axum::Extension(user_id): axum::Extension<Uuid>,
    Json(body): Json<ReviewAlertBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = FraudService::new();
    let status = match body.status.as_str() {
        "confirmed" => erp_fraud::AlertStatus::Confirmed,
        "false_positive" => erp_fraud::AlertStatus::FalsePositive,
        "resolved" => erp_fraud::AlertStatus::Resolved,
        _ => erp_fraud::AlertStatus::UnderReview,
    };
    
    let alert = service.review_alert(id, user_id, ReviewAlertRequest {
        status,
        resolution: None,
        notes: body.notes,
    }).await?;
    Ok(Json(serde_json::to_value(alert)?))
}

#[derive(Deserialize)]
struct AssignAlertBody {
    assignee_id: Uuid,
}

async fn assign_alert(
    Path(id): Path<Uuid>,
    State(_state): State<AppState>,
    Json(body): Json<AssignAlertBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = FraudService::new();
    let alert = service.assign_alert(id, body.assignee_id).await?;
    Ok(Json(serde_json::to_value(alert)?))
}

async fn escalate_alert(
    Path(id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = FraudService::new();
    let alert = service.escalate_alert(id).await?;
    Ok(Json(serde_json::to_value(alert)?))
}

async fn create_rule(
    State(_state): State<AppState>,
    Json(_rule): Json<serde_json::Value>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(_rule))
}

async fn get_rule(
    Path(_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({})))
}

async fn list_rules(
    State(_state): State<AppState>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let service = FraudService::new();
    let rules = service.list_rules(false).await?;
    Ok(Json(rules.into_iter().map(|r| serde_json::to_value(r).unwrap_or_default()).collect()))
}

async fn delete_rule(
    Path(_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<StatusCode> {
    Ok(StatusCode::NO_CONTENT)
}

#[derive(Deserialize)]
struct EvaluateTransactionBody {
    entity_type: String,
    entity_id: Uuid,
    data: serde_json::Value,
}

async fn evaluate_transaction(
    State(_state): State<AppState>,
    Json(body): Json<EvaluateTransactionBody>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let service = FraudService::new();
    let alerts = service.evaluate_transaction(&body.entity_type, body.entity_id, body.data).await?;
    Ok(Json(alerts.into_iter().map(|a| serde_json::to_value(a).unwrap_or_default()).collect()))
}

#[derive(Deserialize)]
struct CreateCaseBody {
    alert_ids: Vec<Uuid>,
    title: String,
    description: String,
}

async fn create_case(
    State(_state): State<AppState>,
    Json(body): Json<CreateCaseBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = FraudService::new();
    let case = service.create_case(body.alert_ids, body.title, body.description).await?;
    Ok(Json(serde_json::to_value(case)?))
}

async fn get_case(
    Path(id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = FraudService::new();
    let case = service.get_case(id).await?;
    Ok(Json(serde_json::to_value(case)?))
}

#[derive(Deserialize)]
struct ListCasesQuery {
    status: Option<String>,
    #[serde(default = "default_limit")]
    limit: i64,
}

async fn list_cases(
    Query(query): Query<ListCasesQuery>,
    State(_state): State<AppState>,
) -> ApiResult<Json<Vec<serde_json::Value>>> {
    let service = FraudService::new();
    let status = query.status.and_then(|s| match s.as_str() {
        "open" => Some(erp_fraud::CaseStatus::Open),
        "under_investigation" => Some(erp_fraud::CaseStatus::UnderInvestigation),
        "pending_review" => Some(erp_fraud::CaseStatus::PendingReview),
        "awaiting_action" => Some(erp_fraud::CaseStatus::AwaitingAction),
        "closed" => Some(erp_fraud::CaseStatus::Closed),
        _ => None,
    });
    let cases = service.list_cases(status, query.limit).await?;
    Ok(Json(cases.into_iter().map(|c| serde_json::to_value(c).unwrap_or_default()).collect()))
}

#[derive(Deserialize)]
struct AddEvidenceBody {
    evidence_type: String,
    description: String,
    file_path: Option<String>,
}

async fn add_evidence(
    Path(case_id): Path<Uuid>,
    State(_state): State<AppState>,
    axum::Extension(user_id): axum::Extension<Uuid>,
    Json(body): Json<AddEvidenceBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = FraudService::new();
    let evidence = Evidence {
        id: Uuid::new_v4(),
        evidence_type: match body.evidence_type.as_str() {
            "document" => EvidenceType::Document,
            "screenshot" => EvidenceType::Screenshot,
            "log_entry" => EvidenceType::LogEntry,
            "transaction_record" => EvidenceType::TransactionRecord,
            _ => EvidenceType::Other,
        },
        description: body.description,
        file_path: body.file_path,
        collected_at: Utc::now(),
        collected_by: user_id,
        verified: false,
    };
    let case = service.add_evidence(case_id, evidence).await?;
    Ok(Json(serde_json::to_value(case)?))
}

#[derive(Deserialize)]
struct ResolveCaseBody {
    outcome: String,
    summary: String,
    actions_taken: Vec<String>,
    recommendations: Vec<String>,
}

async fn resolve_case(
    Path(case_id): Path<Uuid>,
    State(_state): State<AppState>,
    axum::Extension(user_id): axum::Extension<Uuid>,
    Json(body): Json<ResolveCaseBody>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = FraudService::new();
    let outcome = match body.outcome.as_str() {
        "confirmed" => erp_fraud::CaseOutcome::Confirmed,
        "partially_confirmed" => erp_fraud::CaseOutcome::PartiallyConfirmed,
        "not_confirmed" => erp_fraud::CaseOutcome::NotConfirmed,
        "insufficient_evidence" => erp_fraud::CaseOutcome::InsufficientEvidence,
        _ => erp_fraud::CaseOutcome::Referred,
    };
    let resolution = CaseResolution {
        outcome,
        summary: body.summary,
        actions_taken: body.actions_taken,
        recommendations: body.recommendations,
        resolved_by: user_id,
        resolved_at: Utc::now(),
    };
    let case = service.resolve_case(case_id, resolution).await?;
    Ok(Json(serde_json::to_value(case)?))
}

async fn get_vendor_risk(
    Path(_vendor_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({})))
}

async fn calculate_vendor_risk(
    Path(vendor_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = FraudService::new();
    let profile = service.calculate_vendor_risk(vendor_id).await?;
    Ok(Json(serde_json::to_value(profile)?))
}

async fn get_employee_risk(
    Path(_employee_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    Ok(Json(serde_json::json!({})))
}

async fn calculate_employee_risk(
    Path(employee_id): Path<Uuid>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = FraudService::new();
    let profile = service.calculate_employee_risk(employee_id).await?;
    Ok(Json(serde_json::to_value(profile)?))
}

async fn get_analytics(
    Query(query): Query<AnalyticsQuery>,
    State(_state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let service = FraudService::new();
    let start = query.start_date.unwrap_or_else(|| Utc::now() - chrono::Duration::days(30));
    let end = query.end_date.unwrap_or_else(Utc::now);
    let analytics = service.get_analytics(start, end).await?;
    Ok(Json(serde_json::to_value(analytics)?))
}
