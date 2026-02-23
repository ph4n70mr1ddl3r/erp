use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{Utc, NaiveDate};
use crate::compliance::*;
use crate::{Error, Result, Pagination, Paginated};

pub struct ComplianceService;

impl ComplianceService {
    pub async fn create_data_subject(pool: &SqlitePool, email: String, first_name: Option<String>, last_name: Option<String>) -> Result<DataSubject> {
        let now = Utc::now();
        let id = Uuid::new_v4();
        
        sqlx::query(
            "INSERT INTO data_subjects (id, email, first_name, last_name, verification_status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id.to_string())
        .bind(&email)
        .bind(&first_name)
        .bind(&last_name)
        .bind("Unverified")
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(DataSubject {
            id,
            email,
            first_name,
            last_name,
            phone: None,
            address: None,
            identifier_type: None,
            identifier_value: None,
            verification_status: VerificationStatus::Unverified,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn list_data_subjects(pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<DataSubject>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM data_subjects")
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?;
        
        let rows = sqlx::query_as::<_, DataSubjectRow>(
            "SELECT id, email, first_name, last_name, phone, address, identifier_type, identifier_value, verification_status, created_at, updated_at
             FROM data_subjects ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(Paginated::new(
            rows.into_iter().map(|r| r.into()).collect(),
            count.0 as u64,
            pagination,
        ))
    }

    pub async fn create_consent(pool: &SqlitePool, data_subject_id: Uuid, consent_type: String, purpose: String, legal_basis: LegalBasis) -> Result<ConsentRecord> {
        let now = Utc::now();
        let id = Uuid::new_v4();
        
        sqlx::query(
            "INSERT INTO consent_records (id, data_subject_id, consent_type, purpose, legal_basis, granted_at, source, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id.to_string())
        .bind(data_subject_id.to_string())
        .bind(&consent_type)
        .bind(&purpose)
        .bind(format!("{:?}", legal_basis))
        .bind(now.to_rfc3339())
        .bind("web")
        .bind("Granted")
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(ConsentRecord {
            id,
            data_subject_id,
            consent_type,
            purpose,
            legal_basis,
            granted_at: Some(now),
            withdrawn_at: None,
            source: "web".to_string(),
            ip_address: None,
            user_agent: None,
            evidence_path: None,
            status: ConsentStatus::Granted,
            expiry_date: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn withdraw_consent(pool: &SqlitePool, id: Uuid) -> Result<ConsentRecord> {
        let now = Utc::now();
        
        sqlx::query(
            "UPDATE consent_records SET status = 'Withdrawn', withdrawn_at = ?, updated_at = ? WHERE id = ?"
        )
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Self::get_consent(pool, id).await
    }

    pub async fn get_consent(pool: &SqlitePool, id: Uuid) -> Result<ConsentRecord> {
        let row = sqlx::query_as::<_, ConsentRow>(
            "SELECT id, data_subject_id, consent_type, purpose, legal_basis, granted_at, withdrawn_at, source, ip_address, user_agent, evidence_path, status, expiry_date, created_at, updated_at
             FROM consent_records WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?
        .ok_or_else(|| Error::not_found("ConsentRecord", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn list_consents(pool: &SqlitePool, data_subject_id: Option<Uuid>) -> Result<Vec<ConsentRecord>> {
        let rows = if let Some(ds_id) = data_subject_id {
            sqlx::query_as::<_, ConsentRow>(
                "SELECT id, data_subject_id, consent_type, purpose, legal_basis, granted_at, withdrawn_at, source, ip_address, user_agent, evidence_path, status, expiry_date, created_at, updated_at
                 FROM consent_records WHERE data_subject_id = ? ORDER BY created_at DESC"
            )
            .bind(ds_id.to_string())
            .fetch_all(pool)
            .await
        } else {
            sqlx::query_as::<_, ConsentRow>(
                "SELECT id, data_subject_id, consent_type, purpose, legal_basis, granted_at, withdrawn_at, source, ip_address, user_agent, evidence_path, status, expiry_date, created_at, updated_at
                 FROM consent_records ORDER BY created_at DESC LIMIT 100"
            )
            .fetch_all(pool)
            .await
        };
        
        let rows = rows.map_err(|e| Error::Database(e.into()))?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn create_dsar_request(pool: &SqlitePool, data_subject_id: Uuid, request_type: DSARType, description: Option<String>) -> Result<DSARRequest> {
        let now = Utc::now();
        let id = Uuid::new_v4();
        let request_number = format!("DSAR-{}", chrono::Local::now().format("%Y%m%d%H%M%S"));
        let due_date = (chrono::Local::now() + chrono::Duration::days(30)).date_naive();
        
        sqlx::query(
            "INSERT INTO dsar_requests (id, request_number, data_subject_id, request_type, description, received_at, due_date, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id.to_string())
        .bind(&request_number)
        .bind(data_subject_id.to_string())
        .bind(format!("{:?}", request_type))
        .bind(&description)
        .bind(now.to_rfc3339())
        .bind(due_date.to_string())
        .bind("Received")
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(DSARRequest {
            id,
            request_number,
            data_subject_id,
            request_type,
            description,
            received_at: now,
            due_date,
            completed_at: None,
            status: DSARStatus::Received,
            assigned_to: None,
            response: None,
            rejection_reason: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn complete_dsar(pool: &SqlitePool, id: Uuid, response: String) -> Result<DSARRequest> {
        let now = Utc::now();
        
        sqlx::query(
            "UPDATE dsar_requests SET status = 'Completed', completed_at = ?, response = ?, updated_at = ? WHERE id = ?"
        )
        .bind(now.to_rfc3339())
        .bind(&response)
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Self::get_dsar(pool, id).await
    }

    pub async fn get_dsar(pool: &SqlitePool, id: Uuid) -> Result<DSARRequest> {
        let row = sqlx::query_as::<_, DSARRow>(
            "SELECT id, request_number, data_subject_id, request_type, description, received_at, due_date, completed_at, status, assigned_to, response, rejection_reason, created_at, updated_at
             FROM dsar_requests WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?
        .ok_or_else(|| Error::not_found("DSARRequest", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn list_dsars(pool: &SqlitePool, status: Option<DSARStatus>) -> Result<Vec<DSARRequest>> {
        let rows = if let Some(s) = status {
            sqlx::query_as::<_, DSARRow>(
                "SELECT id, request_number, data_subject_id, request_type, description, received_at, due_date, completed_at, status, assigned_to, response, rejection_reason, created_at, updated_at
                 FROM dsar_requests WHERE status = ? ORDER BY created_at DESC"
            )
            .bind(format!("{:?}", s))
            .fetch_all(pool)
            .await
        } else {
            sqlx::query_as::<_, DSARRow>(
                "SELECT id, request_number, data_subject_id, request_type, description, received_at, due_date, completed_at, status, assigned_to, response, rejection_reason, created_at, updated_at
                 FROM dsar_requests ORDER BY created_at DESC LIMIT 100"
            )
            .fetch_all(pool)
            .await
        };
        
        let rows = rows.map_err(|e| Error::Database(e.into()))?;
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn create_data_breach(pool: &SqlitePool, title: String, description: String, breach_type: BreachType, severity: BreachSeverity) -> Result<DataBreach> {
        let now = Utc::now();
        let id = Uuid::new_v4();
        let breach_number = format!("BR-{}", chrono::Local::now().format("%Y%m%d%H%M%S"));
        
        sqlx::query(
            "INSERT INTO data_breaches (id, breach_number, title, description, breach_type, severity, discovered_at, affected_records, affected_data_subjects, data_categories, authority_notified, subjects_notified, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(id.to_string())
        .bind(&breach_number)
        .bind(&title)
        .bind(&description)
        .bind(format!("{:?}", breach_type))
        .bind(format!("{:?}", severity))
        .bind(now.to_rfc3339())
        .bind(0i64)
        .bind(0i64)
        .bind("[]")
        .bind(0i64)
        .bind(0i64)
        .bind("Detected")
        .bind(now.to_rfc3339())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(DataBreach {
            id,
            breach_number,
            title,
            description,
            breach_type,
            severity,
            discovered_at: now,
            occurred_at: None,
            reported_at: None,
            affected_records: 0,
            affected_data_subjects: 0,
            data_categories: vec![],
            containment_measures: None,
            remediation_measures: None,
            authority_notified: false,
            authority_notification_date: None,
            subjects_notified: false,
            subject_notification_date: None,
            status: BreachStatus::Detected,
            assigned_to: None,
            created_at: now,
            updated_at: now,
        })
    }

    pub async fn list_breaches(pool: &SqlitePool) -> Result<Vec<DataBreach>> {
        let rows = sqlx::query_as::<_, BreachRow>(
            "SELECT id, breach_number, title, description, breach_type, severity, discovered_at, occurred_at, reported_at, affected_records, affected_data_subjects, data_categories, containment_measures, remediation_measures, authority_notified, authority_notification_date, subjects_notified, subject_notification_date, status, assigned_to, created_at, updated_at
             FROM data_breaches ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn list_retention_policies(pool: &SqlitePool) -> Result<Vec<DataRetentionPolicy>> {
        let rows = sqlx::query_as::<_, RetentionPolicyRow>(
            "SELECT id, name, description, data_category, retention_period_days, legal_basis, disposal_method, review_frequency_days, last_review_date, next_review_date, status, created_at, updated_at
             FROM data_retention_policies ORDER BY name"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn list_processors(pool: &SqlitePool) -> Result<Vec<ThirdPartyProcessor>> {
        let rows = sqlx::query_as::<_, ProcessorRow>(
            "SELECT id, name, description, contact_name, contact_email, contact_phone, address, country, processing_activities, data_categories, contract_date, contract_expiry, dpa_signed, security_assessment_date, security_assessment_result, status, created_at, updated_at
             FROM third_party_processors ORDER BY name"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn compliance_stats(pool: &SqlitePool) -> Result<ComplianceStats> {
        let ds_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM data_subjects")
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?;
        
        let consent_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM consent_records WHERE status = 'Granted'")
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?;
        
        let pending_dsars: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM dsar_requests WHERE status IN ('Received', 'Verification', 'InProgress')")
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?;
        
        let active_breaches: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM data_breaches WHERE status NOT IN ('Resolved', 'Closed')")
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?;
        
        let processor_count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM third_party_processors WHERE status = 'Active'")
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?;
        
        Ok(ComplianceStats {
            data_subjects: ds_count.0,
            active_consents: consent_count.0,
            pending_dsars: pending_dsars.0,
            active_breaches: active_breaches.0,
            active_processors: processor_count.0,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceStats {
    pub data_subjects: i64,
    pub active_consents: i64,
    pub pending_dsars: i64,
    pub active_breaches: i64,
    pub active_processors: i64,
}

use serde::{Deserialize, Serialize};

#[derive(sqlx::FromRow)]
struct DataSubjectRow {
    id: String,
    email: String,
    first_name: Option<String>,
    last_name: Option<String>,
    phone: Option<String>,
    address: Option<String>,
    identifier_type: Option<String>,
    identifier_value: Option<String>,
    verification_status: String,
    created_at: String,
    updated_at: String,
}

impl From<DataSubjectRow> for DataSubject {
    fn from(r: DataSubjectRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            email: r.email,
            first_name: r.first_name,
            last_name: r.last_name,
            phone: r.phone,
            address: r.address,
            identifier_type: r.identifier_type,
            identifier_value: r.identifier_value,
            verification_status: match r.verification_status.as_str() {
                "Pending" => VerificationStatus::Pending,
                "Verified" => VerificationStatus::Verified,
                "Rejected" => VerificationStatus::Rejected,
                _ => VerificationStatus::Unverified,
            },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct ConsentRow {
    id: String,
    data_subject_id: String,
    consent_type: String,
    purpose: String,
    legal_basis: String,
    granted_at: Option<String>,
    withdrawn_at: Option<String>,
    source: String,
    ip_address: Option<String>,
    user_agent: Option<String>,
    evidence_path: Option<String>,
    status: String,
    expiry_date: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<ConsentRow> for ConsentRecord {
    fn from(r: ConsentRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            data_subject_id: Uuid::parse_str(&r.data_subject_id).unwrap_or_default(),
            consent_type: r.consent_type,
            purpose: r.purpose,
            legal_basis: match r.legal_basis.as_str() {
                "Contract" => LegalBasis::Contract,
                "LegalObligation" => LegalBasis::LegalObligation,
                "VitalInterests" => LegalBasis::VitalInterests,
                "PublicTask" => LegalBasis::PublicTask,
                "LegitimateInterest" => LegalBasis::LegitimateInterest,
                _ => LegalBasis::Consent,
            },
            granted_at: r.granted_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            withdrawn_at: r.withdrawn_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            source: r.source,
            ip_address: r.ip_address,
            user_agent: r.user_agent,
            evidence_path: r.evidence_path,
            status: match r.status.as_str() {
                "Withdrawn" => ConsentStatus::Withdrawn,
                "Expired" => ConsentStatus::Expired,
                "Pending" => ConsentStatus::Pending,
                _ => ConsentStatus::Granted,
            },
            expiry_date: r.expiry_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct DSARRow {
    id: String,
    request_number: String,
    data_subject_id: String,
    request_type: String,
    description: Option<String>,
    received_at: String,
    due_date: String,
    completed_at: Option<String>,
    status: String,
    assigned_to: Option<String>,
    response: Option<String>,
    rejection_reason: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<DSARRow> for DSARRequest {
    fn from(r: DSARRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            request_number: r.request_number,
            data_subject_id: Uuid::parse_str(&r.data_subject_id).unwrap_or_default(),
            request_type: match r.request_type.as_str() {
                "Rectification" => DSARType::Rectification,
                "Erasure" => DSARType::Erasure,
                "Restriction" => DSARType::Restriction,
                "Portability" => DSARType::Portability,
                "Objection" => DSARType::Objection,
                "AutomatedDecision" => DSARType::AutomatedDecision,
                _ => DSARType::Access,
            },
            description: r.description,
            received_at: chrono::DateTime::parse_from_rfc3339(&r.received_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            due_date: NaiveDate::parse_from_str(&r.due_date, "%Y-%m-%d").unwrap_or_else(|_| chrono::Local::now().date_naive()),
            completed_at: r.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            status: match r.status.as_str() {
                "Verification" => DSARStatus::Verification,
                "InProgress" => DSARStatus::InProgress,
                "Completed" => DSARStatus::Completed,
                "Rejected" => DSARStatus::Rejected,
                "Expired" => DSARStatus::Expired,
                _ => DSARStatus::Received,
            },
            assigned_to: r.assigned_to.and_then(|id| Uuid::parse_str(&id).ok()),
            response: r.response,
            rejection_reason: r.rejection_reason,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct BreachRow {
    id: String,
    breach_number: String,
    title: String,
    description: String,
    breach_type: String,
    severity: String,
    discovered_at: String,
    occurred_at: Option<String>,
    reported_at: Option<String>,
    affected_records: i64,
    affected_data_subjects: i64,
    data_categories: String,
    containment_measures: Option<String>,
    remediation_measures: Option<String>,
    authority_notified: i64,
    authority_notification_date: Option<String>,
    subjects_notified: i64,
    subject_notification_date: Option<String>,
    status: String,
    assigned_to: Option<String>,
    created_at: String,
    updated_at: String,
}

impl From<BreachRow> for DataBreach {
    fn from(r: BreachRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            breach_number: r.breach_number,
            title: r.title,
            description: r.description,
            breach_type: match r.breach_type.as_str() {
                "Integrity" => BreachType::Integrity,
                "Availability" => BreachType::Availability,
                "UnauthorizedAccess" => BreachType::UnauthorizedAccess,
                "UnauthorizedDisclosure" => BreachType::UnauthorizedDisclosure,
                "Loss" => BreachType::Loss,
                "Destruction" => BreachType::Destruction,
                _ => BreachType::Confidentiality,
            },
            severity: match r.severity.as_str() {
                "Medium" => BreachSeverity::Medium,
                "High" => BreachSeverity::High,
                "Critical" => BreachSeverity::Critical,
                _ => BreachSeverity::Low,
            },
            discovered_at: chrono::DateTime::parse_from_rfc3339(&r.discovered_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            occurred_at: r.occurred_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            reported_at: r.reported_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            affected_records: r.affected_records as i32,
            affected_data_subjects: r.affected_data_subjects as i32,
            data_categories: serde_json::from_str(&r.data_categories).unwrap_or_default(),
            containment_measures: r.containment_measures,
            remediation_measures: r.remediation_measures,
            authority_notified: r.authority_notified != 0,
            authority_notification_date: r.authority_notification_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            subjects_notified: r.subjects_notified != 0,
            subject_notification_date: r.subject_notification_date.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            status: match r.status.as_str() {
                "Investigating" => BreachStatus::Investigating,
                "Contained" => BreachStatus::Contained,
                "Resolved" => BreachStatus::Resolved,
                "Closed" => BreachStatus::Closed,
                _ => BreachStatus::Detected,
            },
            assigned_to: r.assigned_to.and_then(|id| Uuid::parse_str(&id).ok()),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct RetentionPolicyRow {
    id: String,
    name: String,
    description: Option<String>,
    data_category: String,
    retention_period_days: i64,
    legal_basis: Option<String>,
    disposal_method: String,
    review_frequency_days: i64,
    last_review_date: Option<String>,
    next_review_date: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
}

impl From<RetentionPolicyRow> for DataRetentionPolicy {
    fn from(r: RetentionPolicyRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            name: r.name,
            description: r.description,
            data_category: r.data_category,
            retention_period_days: r.retention_period_days as i32,
            legal_basis: r.legal_basis,
            disposal_method: match r.disposal_method.as_str() {
                "PhysicalDestruction" => DisposalMethod::PhysicalDestruction,
                "Anonymization" => DisposalMethod::Anonymization,
                "Pseudonymization" => DisposalMethod::Pseudonymization,
                "Archival" => DisposalMethod::Archival,
                _ => DisposalMethod::SecureDeletion,
            },
            review_frequency_days: r.review_frequency_days as i32,
            last_review_date: r.last_review_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            next_review_date: r.next_review_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            status: crate::Status::Active,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct ProcessorRow {
    id: String,
    name: String,
    description: Option<String>,
    contact_name: Option<String>,
    contact_email: Option<String>,
    contact_phone: Option<String>,
    address: Option<String>,
    country: String,
    processing_activities: String,
    data_categories: String,
    contract_date: Option<String>,
    contract_expiry: Option<String>,
    dpa_signed: i64,
    security_assessment_date: Option<String>,
    security_assessment_result: Option<String>,
    status: String,
    created_at: String,
    updated_at: String,
}

impl From<ProcessorRow> for ThirdPartyProcessor {
    fn from(r: ProcessorRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            name: r.name,
            description: r.description,
            contact_name: r.contact_name,
            contact_email: r.contact_email,
            contact_phone: r.contact_phone,
            address: r.address,
            country: r.country,
            processing_activities: serde_json::from_str(&r.processing_activities).unwrap_or_default(),
            data_categories: serde_json::from_str(&r.data_categories).unwrap_or_default(),
            contract_date: r.contract_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            contract_expiry: r.contract_expiry.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            dpa_signed: r.dpa_signed != 0,
            security_assessment_date: r.security_assessment_date.and_then(|d| NaiveDate::parse_from_str(&d, "%Y-%m-%d").ok()),
            security_assessment_result: r.security_assessment_result,
            status: crate::Status::Active,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}
