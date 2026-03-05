use crate::models::*;
use async_trait::async_trait;
use chrono::Utc;
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use uuid::Uuid;

#[async_trait]
pub trait QualityRepository: Send + Sync {
    async fn create_inspection(&self, inspection: &QualityInspection) -> anyhow::Result<QualityInspection>;
    async fn get_inspection(&self, id: Uuid) -> anyhow::Result<Option<QualityInspection>>;
    async fn list_inspections(&self, status: Option<InspectionStatus>, inspection_type: Option<InspectionType>) -> anyhow::Result<Vec<QualityInspection>>;
    async fn update_inspection(&self, inspection: &QualityInspection) -> anyhow::Result<QualityInspection>;
    async fn delete_inspection(&self, id: Uuid) -> anyhow::Result<()>;
    async fn add_inspection_item(&self, item: &InspectionItem) -> anyhow::Result<InspectionItem>;
    async fn get_inspection_items(&self, inspection_id: Uuid) -> anyhow::Result<Vec<InspectionItem>>;
    async fn update_inspection_item(&self, item: &InspectionItem) -> anyhow::Result<InspectionItem>;
    async fn delete_inspection_items(&self, inspection_id: Uuid) -> anyhow::Result<()>;
    async fn get_next_inspection_number(&self) -> anyhow::Result<String>;
    
    async fn create_ncr(&self, ncr: &NonConformanceReport) -> anyhow::Result<NonConformanceReport>;
    async fn get_ncr(&self, id: Uuid) -> anyhow::Result<Option<NonConformanceReport>>;
    async fn list_ncrs(&self, status: Option<NCRStatus>, severity: Option<NCRSeverity>) -> anyhow::Result<Vec<NonConformanceReport>>;
    async fn update_ncr(&self, ncr: &NonConformanceReport) -> anyhow::Result<NonConformanceReport>;
    async fn delete_ncr(&self, id: Uuid) -> anyhow::Result<()>;
    async fn get_next_ncr_number(&self) -> anyhow::Result<String>;
}

pub struct SqliteQualityRepository {
    pool: SqlitePool,
}

impl SqliteQualityRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl QualityRepository for SqliteQualityRepository {
    async fn create_inspection(&self, inspection: &QualityInspection) -> anyhow::Result<QualityInspection> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            r#"INSERT INTO quality_inspections 
               (id, inspection_number, inspection_type, entity_type, entity_id, inspector_id, 
                inspection_date, status, result, notes, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(inspection.base.id.to_string())
        .bind(&inspection.inspection_number)
        .bind(format!("{:?}", inspection.inspection_type))
        .bind(&inspection.entity_type)
        .bind(inspection.entity_id.to_string())
        .bind(inspection.inspector_id.map(|id| id.to_string()))
        .bind(inspection.inspection_date.to_string())
        .bind(format!("{:?}", inspection.status))
        .bind(inspection.result.as_ref().map(|r| format!("{:?}", r)))
        .bind(&inspection.notes)
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(inspection.clone())
    }

    async fn get_inspection(&self, id: Uuid) -> anyhow::Result<Option<QualityInspection>> {
        let row = sqlx::query_as::<_, (String, String, String, String, String, Option<String>, String, String, Option<String>, Option<String>, String)>(
            "SELECT id, inspection_number, inspection_type, entity_type, entity_id, inspector_id, 
                    inspection_date, status, result, notes, created_at 
             FROM quality_inspections WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| QualityInspection {
            base: BaseEntity {
                id: Uuid::parse_str(&r.0).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.10).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: Utc::now(),
                created_by: None,
                updated_by: None,
            },
            inspection_number: r.1,
            inspection_type: parse_inspection_type(&r.2),
            entity_type: r.3,
            entity_id: Uuid::parse_str(&r.4).unwrap_or_default(),
            inspector_id: r.5.and_then(|s| Uuid::parse_str(&s).ok()),
            inspection_date: chrono::NaiveDate::parse_from_str(&r.6, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            status: parse_inspection_status(&r.7),
            result: r.8.and_then(|s| parse_inspection_result(&s)),
            notes: r.9,
        }))
    }

    async fn list_inspections(&self, status: Option<InspectionStatus>, inspection_type: Option<InspectionType>) -> anyhow::Result<Vec<QualityInspection>> {
        let mut query = "SELECT id, inspection_number, inspection_type, entity_type, entity_id, inspector_id, 
                        inspection_date, status, result, notes, created_at 
                        FROM quality_inspections WHERE 1=1".to_string();
        let mut binds: Vec<String> = Vec::new();

        if let Some(s) = status {
            query.push_str(" AND status = ?");
            binds.push(format!("{:?}", s));
        }
        if let Some(t) = inspection_type {
            query.push_str(" AND inspection_type = ?");
            binds.push(format!("{:?}", t));
        }
        query.push_str(" ORDER BY created_at DESC");

        let mut sql_query = sqlx::query_as::<_, (String, String, String, String, String, Option<String>, String, String, Option<String>, Option<String>, String)>(&query);
        for bind in binds {
            sql_query = sql_query.bind(bind);
        }

        let rows = sql_query.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| QualityInspection {
            base: BaseEntity {
                id: Uuid::parse_str(&r.0).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.10).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: Utc::now(),
                created_by: None,
                updated_by: None,
            },
            inspection_number: r.1,
            inspection_type: parse_inspection_type(&r.2),
            entity_type: r.3,
            entity_id: Uuid::parse_str(&r.4).unwrap_or_default(),
            inspector_id: r.5.and_then(|s| Uuid::parse_str(&s).ok()),
            inspection_date: chrono::NaiveDate::parse_from_str(&r.6, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            status: parse_inspection_status(&r.7),
            result: r.8.and_then(|s| parse_inspection_result(&s)),
            notes: r.9,
        }).collect())
    }

    async fn update_inspection(&self, inspection: &QualityInspection) -> anyhow::Result<QualityInspection> {
        sqlx::query(
            "UPDATE quality_inspections SET inspection_type=?, entity_type=?, entity_id=?, inspector_id=?, 
             inspection_date=?, status=?, result=?, notes=? WHERE id=?"
        )
        .bind(format!("{:?}", inspection.inspection_type))
        .bind(&inspection.entity_type)
        .bind(inspection.entity_id.to_string())
        .bind(inspection.inspector_id.map(|id| id.to_string()))
        .bind(inspection.inspection_date.to_string())
        .bind(format!("{:?}", inspection.status))
        .bind(inspection.result.as_ref().map(|r| format!("{:?}", r)))
        .bind(&inspection.notes)
        .bind(inspection.base.id.to_string())
        .execute(&self.pool)
        .await?;
        Ok(inspection.clone())
    }

    async fn delete_inspection(&self, id: Uuid) -> anyhow::Result<()> {
        self.delete_inspection_items(id).await?;
        sqlx::query("DELETE FROM quality_inspections WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn add_inspection_item(&self, item: &InspectionItem) -> anyhow::Result<InspectionItem> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            r#"INSERT INTO inspection_items 
               (id, inspection_id, criterion, expected_value, actual_value, pass_fail, notes, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(item.id.to_string())
        .bind(item.inspection_id.to_string())
        .bind(&item.criterion)
        .bind(&item.expected_value)
        .bind(&item.actual_value)
        .bind(item.pass_fail.map(|b| if b { 1 } else { 0 }))
        .bind(&item.notes)
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(item.clone())
    }

    async fn get_inspection_items(&self, inspection_id: Uuid) -> anyhow::Result<Vec<InspectionItem>> {
        let rows = sqlx::query_as::<_, (String, String, String, Option<String>, Option<String>, Option<i32>, Option<String>, String)>(
            "SELECT id, inspection_id, criterion, expected_value, actual_value, pass_fail, notes, created_at 
             FROM inspection_items WHERE inspection_id = ?"
        )
        .bind(inspection_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| InspectionItem {
            id: Uuid::parse_str(&r.0).unwrap_or_default(),
            inspection_id: Uuid::parse_str(&r.1).unwrap_or_default(),
            criterion: r.2,
            expected_value: r.3,
            actual_value: r.4,
            pass_fail: r.5.map(|v| v == 1),
            notes: r.6,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.7).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }).collect())
    }

    async fn update_inspection_item(&self, item: &InspectionItem) -> anyhow::Result<InspectionItem> {
        sqlx::query(
            "UPDATE inspection_items SET actual_value=?, pass_fail=?, notes=? WHERE id=?"
        )
        .bind(&item.actual_value)
        .bind(item.pass_fail.map(|b| if b { 1 } else { 0 }))
        .bind(&item.notes)
        .bind(item.id.to_string())
        .execute(&self.pool)
        .await?;
        Ok(item.clone())
    }

    async fn delete_inspection_items(&self, inspection_id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM inspection_items WHERE inspection_id = ?")
            .bind(inspection_id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_next_inspection_number(&self) -> anyhow::Result<String> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM quality_inspections")
            .fetch_one(&self.pool)
            .await?;
        Ok(format!("QI-{:06}", count.0 + 1))
    }

    async fn create_ncr(&self, ncr: &NonConformanceReport) -> anyhow::Result<NonConformanceReport> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            r#"INSERT INTO non_conformance_reports 
               (id, ncr_number, source_type, source_id, product_id, description, severity, status, 
                assigned_to, root_cause, corrective_action, preventive_action, resolution_date, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(ncr.base.id.to_string())
        .bind(&ncr.ncr_number)
        .bind(format!("{:?}", ncr.source_type))
        .bind(ncr.source_id.map(|id| id.to_string()))
        .bind(ncr.product_id.map(|id| id.to_string()))
        .bind(&ncr.description)
        .bind(format!("{:?}", ncr.severity))
        .bind(format!("{:?}", ncr.status))
        .bind(ncr.assigned_to.map(|id| id.to_string()))
        .bind(&ncr.root_cause)
        .bind(&ncr.corrective_action)
        .bind(&ncr.preventive_action)
        .bind(ncr.resolution_date.map(|d| d.to_string()))
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(ncr.clone())
    }

    async fn get_ncr(&self, id: Uuid) -> anyhow::Result<Option<NonConformanceReport>> {
        let row = sqlx::query_as::<_, (String, String, String, Option<String>, Option<String>, String, String, String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, String, String)>(
            "SELECT id, ncr_number, source_type, source_id, product_id, description, severity, status, 
                    assigned_to, root_cause, corrective_action, preventive_action, resolution_date, created_at, updated_at
             FROM non_conformance_reports WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| NonConformanceReport {
            base: BaseEntity {
                id: Uuid::parse_str(&r.0).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.13).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.14).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            ncr_number: r.1,
            source_type: parse_ncr_source(&r.2),
            source_id: r.3.and_then(|s| Uuid::parse_str(&s).ok()),
            product_id: r.4.and_then(|s| Uuid::parse_str(&s).ok()),
            description: r.5,
            severity: parse_ncr_severity(&r.6),
            status: parse_ncr_status(&r.7),
            assigned_to: r.8.and_then(|s| Uuid::parse_str(&s).ok()),
            root_cause: r.9,
            corrective_action: r.10,
            preventive_action: r.11,
            resolution_date: r.12.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
        }))
    }

    async fn list_ncrs(&self, status: Option<NCRStatus>, severity: Option<NCRSeverity>) -> anyhow::Result<Vec<NonConformanceReport>> {
        let mut query = "SELECT id, ncr_number, source_type, source_id, product_id, description, severity, status, 
                        assigned_to, root_cause, corrective_action, preventive_action, resolution_date, created_at, updated_at
                        FROM non_conformance_reports WHERE 1=1".to_string();
        let mut binds: Vec<String> = Vec::new();

        if let Some(s) = status {
            query.push_str(" AND status = ?");
            binds.push(format!("{:?}", s));
        }
        if let Some(sev) = severity {
            query.push_str(" AND severity = ?");
            binds.push(format!("{:?}", sev));
        }
        query.push_str(" ORDER BY created_at DESC");

        let mut sql_query = sqlx::query_as::<_, (String, String, String, Option<String>, Option<String>, String, String, String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, String, String)>(&query);
        for bind in binds {
            sql_query = sql_query.bind(bind);
        }

        let rows = sql_query.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| NonConformanceReport {
            base: BaseEntity {
                id: Uuid::parse_str(&r.0).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.13).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.14).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            ncr_number: r.1,
            source_type: parse_ncr_source(&r.2),
            source_id: r.3.and_then(|s| Uuid::parse_str(&s).ok()),
            product_id: r.4.and_then(|s| Uuid::parse_str(&s).ok()),
            description: r.5,
            severity: parse_ncr_severity(&r.6),
            status: parse_ncr_status(&r.7),
            assigned_to: r.8.and_then(|s| Uuid::parse_str(&s).ok()),
            root_cause: r.9,
            corrective_action: r.10,
            preventive_action: r.11,
            resolution_date: r.12.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
        }).collect())
    }

    async fn update_ncr(&self, ncr: &NonConformanceReport) -> anyhow::Result<NonConformanceReport> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE non_conformance_reports SET source_type=?, source_id=?, product_id=?, description=?, 
             severity=?, status=?, assigned_to=?, root_cause=?, corrective_action=?, preventive_action=?, 
             resolution_date=?, updated_at=? WHERE id=?"
        )
        .bind(format!("{:?}", ncr.source_type))
        .bind(ncr.source_id.map(|id| id.to_string()))
        .bind(ncr.product_id.map(|id| id.to_string()))
        .bind(&ncr.description)
        .bind(format!("{:?}", ncr.severity))
        .bind(format!("{:?}", ncr.status))
        .bind(ncr.assigned_to.map(|id| id.to_string()))
        .bind(&ncr.root_cause)
        .bind(&ncr.corrective_action)
        .bind(&ncr.preventive_action)
        .bind(ncr.resolution_date.map(|d| d.to_string()))
        .bind(&now)
        .bind(ncr.base.id.to_string())
        .execute(&self.pool)
        .await?;
        Ok(ncr.clone())
    }

    async fn delete_ncr(&self, id: Uuid) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM non_conformance_reports WHERE id = ?")
            .bind(id.to_string())
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_next_ncr_number(&self) -> anyhow::Result<String> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM non_conformance_reports")
            .fetch_one(&self.pool)
            .await?;
        Ok(format!("NCR-{:06}", count.0 + 1))
    }
}

fn parse_inspection_type(s: &str) -> InspectionType {
    match s {
        "Incoming" => InspectionType::Incoming,
        "InProcess" => InspectionType::InProcess,
        "Final" => InspectionType::Final,
        "Outgoing" => InspectionType::Outgoing,
        "Supplier" => InspectionType::Supplier,
        "Customer" => InspectionType::Customer,
        _ => InspectionType::Incoming,
    }
}

fn parse_inspection_status(s: &str) -> InspectionStatus {
    match s {
        "Pending" => InspectionStatus::Pending,
        "InProgress" => InspectionStatus::InProgress,
        "Passed" => InspectionStatus::Passed,
        "Failed" => InspectionStatus::Failed,
        "Partial" => InspectionStatus::Partial,
        _ => InspectionStatus::Cancelled,
    }
}

fn parse_inspection_result(s: &str) -> Option<InspectionResult> {
    match s {
        "Pass" => Some(InspectionResult::Pass),
        "Fail" => Some(InspectionResult::Fail),
        "ConditionalPass" => Some(InspectionResult::ConditionalPass),
        _ => None,
    }
}

fn parse_ncr_source(s: &str) -> NCRSource {
    match s {
        "IncomingInspection" => NCRSource::IncomingInspection,
        "InProcessInspection" => NCRSource::InProcessInspection,
        "FinalInspection" => NCRSource::FinalInspection,
        "CustomerComplaint" => NCRSource::CustomerComplaint,
        "InternalAudit" => NCRSource::InternalAudit,
        "SupplierIssue" => NCRSource::SupplierIssue,
        "ProductionIssue" => NCRSource::ProductionIssue,
        _ => NCRSource::Other,
    }
}

fn parse_ncr_severity(s: &str) -> NCRSeverity {
    match s {
        "Minor" => NCRSeverity::Minor,
        "Major" => NCRSeverity::Major,
        "Critical" => NCRSeverity::Critical,
        _ => NCRSeverity::Minor,
    }
}

fn parse_ncr_status(s: &str) -> NCRStatus {
    match s {
        "Open" => NCRStatus::Open,
        "UnderInvestigation" => NCRStatus::UnderInvestigation,
        "CorrectiveAction" => NCRStatus::CorrectiveAction,
        "Verification" => NCRStatus::Verification,
        "Closed" => NCRStatus::Closed,
        _ => NCRStatus::Cancelled,
    }
}
