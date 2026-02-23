use axum::{
    extract::State,
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::db::AppState;
use crate::error::ApiResult;
use erp_core::{Pagination, ComplianceService};
use erp_core::compliance::*;

#[derive(Deserialize)]
pub struct CreateDataSubjectRequest {
    pub email: String,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateConsentRequest {
    pub data_subject_id: String,
    pub consent_type: String,
    pub purpose: String,
    pub legal_basis: String,
}

#[derive(Deserialize)]
pub struct CreateDSARRequest {
    pub data_subject_id: String,
    pub request_type: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct CompleteDSARRequest {
    pub response: String,
}

#[derive(Deserialize)]
pub struct CreateBreachRequest {
    pub title: String,
    pub description: String,
    pub breach_type: String,
    pub severity: String,
}

pub async fn stats(State(state): State<AppState>) -> ApiResult<Json<crate::compliance_service::ComplianceStats>> {
    let stats = ComplianceService::compliance_stats(&state.pool).await?;
    Ok(Json(stats))
}

pub async fn list_data_subjects(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<DataSubject>>> {
    let result = ComplianceService::list_data_subjects(&state.pool, Pagination::new(1, 100)).await?;
    Ok(Json(result.items))
}

pub async fn create_data_subject(
    State(state): State<AppState>,
    Json(req): Json<CreateDataSubjectRequest>,
) -> ApiResult<Json<DataSubject>> {
    let subject = ComplianceService::create_data_subject(
        &state.pool,
        req.email,
        req.first_name,
        req.last_name,
    ).await?;
    Ok(Json(subject))
}

pub async fn list_consents(
    State(state): State<AppState>,
) -> ApiResult<Json<Vec<ConsentRecord>>> {
    let consents = ComplianceService::list_consents(&state.pool, None).await?;
    Ok(Json(consents))
}

pub async fn create_consent(
    State(state): State<AppState>,
    Json(req): Json<CreateConsentRequest>,
) -> ApiResult<Json<ConsentRecord>> {
    let data_subject_id = Uuid::parse_str(&req.data_subject_id)
        .map_err(|_| erp_core::Error::validation("Invalid data_subject_id"))?;
    
    let legal_basis = match req.legal_basis.as_str() {
        "Contract" => LegalBasis::Contract,
        "LegalObligation" => LegalBasis::LegalObligation,
        "VitalInterests" => LegalBasis::VitalInterests,
        "PublicTask" => LegalBasis::PublicTask,
        "LegitimateInterest" => LegalBasis::LegitimateInterest,
        _ => LegalBasis::Consent,
    };
    
    let consent = ComplianceService::create_consent(
        &state.pool,
        data_subject_id,
        req.consent_type,
        req.purpose,
        legal_basis,
    ).await?;
    Ok(Json(consent))
}

pub async fn withdraw_consent(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
) -> ApiResult<Json<ConsentRecord>> {
    let id = Uuid::parse_str(&id)
        .map_err(|_| erp_core::Error::validation("Invalid consent id"))?;
    let consent = ComplianceService::withdraw_consent(&state.pool, id).await?;
    Ok(Json(consent))
}

pub async fn list_dsars(State(state): State<AppState>) -> ApiResult<Json<Vec<DSARRequest>>> {
    let dsars = ComplianceService::list_dsars(&state.pool, None).await?;
    Ok(Json(dsars))
}

pub async fn create_dsar(
    State(state): State<AppState>,
    Json(req): Json<CreateDSARRequest>,
) -> ApiResult<Json<DSARRequest>> {
    let data_subject_id = Uuid::parse_str(&req.data_subject_id)
        .map_err(|_| erp_core::Error::validation("Invalid data_subject_id"))?;
    
    let request_type = match req.request_type.as_str() {
        "Rectification" => DSARType::Rectification,
        "Erasure" => DSARType::Erasure,
        "Restriction" => DSARType::Restriction,
        "Portability" => DSARType::Portability,
        "Objection" => DSARType::Objection,
        "AutomatedDecision" => DSARType::AutomatedDecision,
        _ => DSARType::Access,
    };
    
    let dsar = ComplianceService::create_dsar_request(
        &state.pool,
        data_subject_id,
        request_type,
        req.description,
    ).await?;
    Ok(Json(dsar))
}

pub async fn complete_dsar(
    State(state): State<AppState>,
    axum::extract::Path(id): axum::extract::Path<String>,
    Json(req): Json<CompleteDSARRequest>,
) -> ApiResult<Json<DSARRequest>> {
    let id = Uuid::parse_str(&id)
        .map_err(|_| erp_core::Error::validation("Invalid dsar id"))?;
    let dsar = ComplianceService::complete_dsar(&state.pool, id, req.response).await?;
    Ok(Json(dsar))
}

pub async fn list_breaches(State(state): State<AppState>) -> ApiResult<Json<Vec<DataBreach>>> {
    let breaches = ComplianceService::list_breaches(&state.pool).await?;
    Ok(Json(breaches))
}

pub async fn create_breach(
    State(state): State<AppState>,
    Json(req): Json<CreateBreachRequest>,
) -> ApiResult<Json<DataBreach>> {
    let breach_type = match req.breach_type.as_str() {
        "Integrity" => BreachType::Integrity,
        "Availability" => BreachType::Availability,
        "UnauthorizedAccess" => BreachType::UnauthorizedAccess,
        "UnauthorizedDisclosure" => BreachType::UnauthorizedDisclosure,
        "Loss" => BreachType::Loss,
        "Destruction" => BreachType::Destruction,
        _ => BreachType::Confidentiality,
    };
    
    let severity = match req.severity.as_str() {
        "Medium" => BreachSeverity::Medium,
        "High" => BreachSeverity::High,
        "Critical" => BreachSeverity::Critical,
        _ => BreachSeverity::Low,
    };
    
    let breach = ComplianceService::create_data_breach(
        &state.pool,
        req.title,
        req.description,
        breach_type,
        severity,
    ).await?;
    Ok(Json(breach))
}

pub async fn list_policies(State(state): State<AppState>) -> ApiResult<Json<Vec<DataRetentionPolicy>>> {
    let policies = ComplianceService::list_retention_policies(&state.pool).await?;
    Ok(Json(policies))
}

pub async fn list_processors(State(state): State<AppState>) -> ApiResult<Json<Vec<ThirdPartyProcessor>>> {
    let processors = ComplianceService::list_processors(&state.pool).await?;
    Ok(Json(processors))
}
