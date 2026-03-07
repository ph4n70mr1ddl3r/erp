use crate::models::*;
use async_trait::async_trait;
use chrono::Utc;
use erp_core::BaseEntity;
use sqlx::{Row, SqlitePool};
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

    async fn create_calibration_device(&self, device: &CalibrationDevice) -> anyhow::Result<CalibrationDevice>;
    async fn get_calibration_device(&self, id: Uuid) -> anyhow::Result<Option<CalibrationDevice>>;
    async fn list_calibration_devices(&self, status: Option<CalibrationStatus>) -> anyhow::Result<Vec<CalibrationDevice>>;
    async fn update_calibration_device(&self, device: &CalibrationDevice) -> anyhow::Result<CalibrationDevice>;
    async fn create_calibration_record(&self, record: &CalibrationRecord) -> anyhow::Result<CalibrationRecord>;
    async fn get_calibration_record(&self, id: Uuid) -> anyhow::Result<Option<CalibrationRecord>>;
    async fn add_calibration_reading(&self, reading: &CalibrationReading) -> anyhow::Result<CalibrationReading>;
    async fn get_calibration_readings(&self, record_id: Uuid) -> anyhow::Result<Vec<CalibrationReading>>;
    async fn get_next_calibration_record_number(&self) -> anyhow::Result<String>;

    // CAPA Management
    async fn create_capa(&self, capa: &CAPA) -> anyhow::Result<CAPA>;
    async fn get_capa(&self, id: Uuid) -> anyhow::Result<Option<CAPA>>;
    async fn list_capas(&self, status: Option<CAPAStatus>, priority: Option<NCRSeverity>) -> anyhow::Result<Vec<CAPA>>;
    async fn update_capa(&self, capa: &CAPA) -> anyhow::Result<CAPA>;
    async fn get_next_capa_number(&self) -> anyhow::Result<String>;
    
    async fn create_capa_action(&self, action: &CAPAAction) -> anyhow::Result<CAPAAction>;
    async fn list_capa_actions(&self, capa_id: Uuid) -> anyhow::Result<Vec<CAPAAction>>;
    async fn update_capa_action(&self, action: &CAPAAction) -> anyhow::Result<CAPAAction>;
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

    async fn create_calibration_device(&self, device: &CalibrationDevice) -> anyhow::Result<CalibrationDevice> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            r#"INSERT INTO calibration_devices 
               (id, device_number, name, description, manufacturer, model, serial_number, 
                location, calibration_frequency_days, last_calibration_date, next_calibration_date, 
                status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(device.base.id.to_string())
        .bind(&device.device_number)
        .bind(&device.name)
        .bind(&device.description)
        .bind(&device.manufacturer)
        .bind(&device.model)
        .bind(&device.serial_number)
        .bind(&device.location)
        .bind(device.calibration_frequency_days)
        .bind(device.last_calibration_date.map(|d| d.to_string()))
        .bind(device.next_calibration_date.map(|d| d.to_string()))
        .bind(format!("{:?}", device.status))
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(device.clone())
    }

    async fn get_calibration_device(&self, id: Uuid) -> anyhow::Result<Option<CalibrationDevice>> {
        let row = sqlx::query_as::<_, (String, String, String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, i32, Option<String>, Option<String>, String, String, String)>(
            "SELECT id, device_number, name, description, manufacturer, model, serial_number, 
                    location, calibration_frequency_days, last_calibration_date, next_calibration_date, 
                    status, created_at, updated_at 
             FROM calibration_devices WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| CalibrationDevice {
            base: BaseEntity {
                id: Uuid::parse_str(&r.0).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.12).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.13).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            device_number: r.1,
            name: r.2,
            description: r.3,
            manufacturer: r.4,
            model: r.5,
            serial_number: r.6,
            location: r.7,
            calibration_frequency_days: r.8,
            last_calibration_date: r.9.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            next_calibration_date: r.10.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            status: parse_calibration_status(&r.11),
        }))
    }

    async fn list_calibration_devices(&self, status: Option<CalibrationStatus>) -> anyhow::Result<Vec<CalibrationDevice>> {
        let mut query = "SELECT id, device_number, name, description, manufacturer, model, serial_number, 
                        location, calibration_frequency_days, last_calibration_date, next_calibration_date, 
                        status, created_at, updated_at 
                        FROM calibration_devices WHERE 1=1".to_string();
        let mut binds: Vec<String> = Vec::new();

        if let Some(s) = status {
            query.push_str(" AND status = ?");
            binds.push(format!("{:?}", s));
        }
        query.push_str(" ORDER BY created_at DESC");

        let mut sql_query = sqlx::query_as::<_, (String, String, String, Option<String>, Option<String>, Option<String>, Option<String>, Option<String>, i32, Option<String>, Option<String>, String, String, String)>(&query);
        for bind in binds {
            sql_query = sql_query.bind(bind);
        }

        let rows = sql_query.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| CalibrationDevice {
            base: BaseEntity {
                id: Uuid::parse_str(&r.0).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.12).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.13).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            device_number: r.1,
            name: r.2,
            description: r.3,
            manufacturer: r.4,
            model: r.5,
            serial_number: r.6,
            location: r.7,
            calibration_frequency_days: r.8,
            last_calibration_date: r.9.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            next_calibration_date: r.10.and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            status: parse_calibration_status(&r.11),
        }).collect())
    }

    async fn update_calibration_device(&self, device: &CalibrationDevice) -> anyhow::Result<CalibrationDevice> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE calibration_devices SET name=?, description=?, manufacturer=?, model=?, serial_number=?, 
             location=?, calibration_frequency_days=?, last_calibration_date=?, next_calibration_date=?, 
             status=?, updated_at=? WHERE id=?"
        )
        .bind(&device.name)
        .bind(&device.description)
        .bind(&device.manufacturer)
        .bind(&device.model)
        .bind(&device.serial_number)
        .bind(&device.location)
        .bind(device.calibration_frequency_days)
        .bind(device.last_calibration_date.map(|d| d.to_string()))
        .bind(device.next_calibration_date.map(|d| d.to_string()))
        .bind(format!("{:?}", device.status))
        .bind(&now)
        .bind(device.base.id.to_string())
        .execute(&self.pool)
        .await?;
        Ok(device.clone())
    }

    async fn create_calibration_record(&self, record: &CalibrationRecord) -> anyhow::Result<CalibrationRecord> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            r#"INSERT INTO calibration_records 
               (id, record_number, device_id, calibration_date, calibrated_by, status, 
                certificate_number, notes, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(record.base.id.to_string())
        .bind(&record.record_number)
        .bind(record.device_id.to_string())
        .bind(record.calibration_date.to_string())
        .bind(record.calibrated_by.map(|id| id.to_string()))
        .bind(format!("{:?}", record.status))
        .bind(&record.certificate_number)
        .bind(&record.notes)
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(record.clone())
    }

    async fn get_calibration_record(&self, id: Uuid) -> anyhow::Result<Option<CalibrationRecord>> {
        let row = sqlx::query_as::<_, (String, String, String, String, Option<String>, String, Option<String>, Option<String>, String, String)>(
            "SELECT id, record_number, device_id, calibration_date, calibrated_by, status, 
                    certificate_number, notes, created_at, updated_at 
             FROM calibration_records WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| CalibrationRecord {
            base: BaseEntity {
                id: Uuid::parse_str(&r.0).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.8).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.9).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            record_number: r.1,
            device_id: Uuid::parse_str(&r.2).unwrap_or_default(),
            calibration_date: chrono::NaiveDate::parse_from_str(&r.3, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            calibrated_by: r.4.and_then(|s| Uuid::parse_str(&s).ok()),
            status: parse_calibration_status(&r.5),
            certificate_number: r.6,
            notes: r.7,
        }))
    }

    async fn add_calibration_reading(&self, reading: &CalibrationReading) -> anyhow::Result<CalibrationReading> {
        sqlx::query(
            r#"INSERT INTO calibration_readings 
               (id, record_id, parameter, reference_value, actual_value, tolerance_min, 
                tolerance_max, pass_fail, uom)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(reading.id.to_string())
        .bind(reading.record_id.to_string())
        .bind(&reading.parameter)
        .bind(reading.reference_value)
        .bind(reading.actual_value)
        .bind(reading.tolerance_min)
        .bind(reading.tolerance_max)
        .bind(if reading.pass_fail { 1 } else { 0 })
        .bind(&reading.uom)
        .execute(&self.pool)
        .await?;
        Ok(reading.clone())
    }

    async fn get_calibration_readings(&self, record_id: Uuid) -> anyhow::Result<Vec<CalibrationReading>> {
        let rows = sqlx::query_as::<_, (String, String, String, f64, f64, f64, f64, i32, String)>(
            "SELECT id, record_id, parameter, reference_value, actual_value, tolerance_min, 
                    tolerance_max, pass_fail, uom 
             FROM calibration_readings WHERE record_id = ?"
        )
        .bind(record_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| CalibrationReading {
            id: Uuid::parse_str(&r.0).unwrap_or_default(),
            record_id: Uuid::parse_str(&r.1).unwrap_or_default(),
            parameter: r.2,
            reference_value: r.3,
            actual_value: r.4,
            tolerance_min: r.5,
            tolerance_max: r.6,
            pass_fail: r.7 == 1,
            uom: r.8,
        }).collect())
    }

    async fn get_next_calibration_record_number(&self) -> anyhow::Result<String> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM calibration_records")
            .fetch_one(&self.pool)
            .await?;
        Ok(format!("CAL-{:06}", count.0 + 1))
    }

    // CAPA Management
    async fn create_capa(&self, capa: &CAPA) -> anyhow::Result<CAPA> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            r#"INSERT INTO quality_capas 
               (id, capa_number, title, source_type, source_id, description, priority, status, 
                initiator_id, owner_id, root_cause_analysis, action_plan, verification_plan, 
                effectiveness_criteria, target_completion_date, effectiveness_result, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(capa.base.id.to_string())
        .bind(&capa.capa_number)
        .bind(&capa.title)
        .bind(format!("{:?}", capa.source_type))
        .bind(capa.source_id.map(|id| id.to_string()))
        .bind(&capa.description)
        .bind(format!("{:?}", capa.priority))
        .bind(format!("{:?}", capa.status))
        .bind(capa.initiator_id.to_string())
        .bind(capa.owner_id.map(|id| id.to_string()))
        .bind(&capa.root_cause_analysis)
        .bind(&capa.action_plan)
        .bind(&capa.verification_plan)
        .bind(&capa.effectiveness_criteria)
        .bind(capa.target_completion_date.map(|d| d.to_string()))
        .bind(capa.effectiveness_result.map(|b| if b { 1 } else { 0 }))
        .bind(&now)
        .bind(&now)
        .execute(&self.pool)
        .await?;
        Ok(capa.clone())
    }

    async fn get_capa(&self, id: Uuid) -> anyhow::Result<Option<CAPA>> {
        let row = sqlx::query(
            "SELECT id, capa_number, title, source_type, source_id, description, priority, status, 
                    initiator_id, owner_id, root_cause_analysis, action_plan, verification_plan, 
                    effectiveness_criteria, target_completion_date, effectiveness_result, created_at, updated_at
             FROM quality_capas WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| CAPA {
            base: BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            capa_number: r.get("capa_number"),
            title: r.get("title"),
            source_type: parse_capa_source(r.get("source_type")),
            source_id: r.get::<Option<String>, _>("source_id").and_then(|s| Uuid::parse_str(&s).ok()),
            description: r.get("description"),
            priority: parse_ncr_severity(r.get("priority")),
            status: parse_capa_status(r.get("status")),
            initiator_id: Uuid::parse_str(r.get::<&str, _>("initiator_id")).unwrap_or_default(),
            owner_id: r.get::<Option<String>, _>("owner_id").and_then(|s| Uuid::parse_str(&s).ok()),
            root_cause_analysis: r.get("root_cause_analysis"),
            action_plan: r.get("action_plan"),
            verification_plan: r.get("verification_plan"),
            effectiveness_criteria: r.get("effectiveness_criteria"),
            target_completion_date: r.get::<Option<String>, _>("target_completion_date").and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            actual_completion_date: None,
            effectiveness_result: r.get::<Option<i32>, _>("effectiveness_result").map(|v| v == 1),
        }))
    }

    async fn list_capas(&self, status: Option<CAPAStatus>, priority: Option<NCRSeverity>) -> anyhow::Result<Vec<CAPA>> {
        let mut query = "SELECT id, capa_number, title, source_type, source_id, description, priority, status, 
                        initiator_id, owner_id, root_cause_analysis, action_plan, verification_plan, 
                        effectiveness_criteria, target_completion_date, effectiveness_result, created_at, updated_at
                        FROM quality_capas WHERE 1=1".to_string();
        let mut binds: Vec<String> = Vec::new();

        if let Some(s) = status {
            query.push_str(" AND status = ?");
            binds.push(format!("{:?}", s));
        }
        if let Some(p) = priority {
            query.push_str(" AND priority = ?");
            binds.push(format!("{:?}", p));
        }
        query.push_str(" ORDER BY created_at DESC");

        let mut sql_query = sqlx::query(&query);
        for bind in binds {
            sql_query = sql_query.bind(bind);
        }

        let rows = sql_query.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| CAPA {
            base: BaseEntity {
                id: Uuid::parse_str(r.get::<&str, _>("id")).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("created_at")).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(r.get::<&str, _>("updated_at")).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            capa_number: r.get("capa_number"),
            title: r.get("title"),
            source_type: parse_capa_source(r.get("source_type")),
            source_id: r.get::<Option<String>, _>("source_id").and_then(|s| Uuid::parse_str(&s).ok()),
            description: r.get("description"),
            priority: parse_ncr_severity(r.get("priority")),
            status: parse_capa_status(r.get("status")),
            initiator_id: Uuid::parse_str(r.get::<&str, _>("initiator_id")).unwrap_or_default(),
            owner_id: r.get::<Option<String>, _>("owner_id").and_then(|s| Uuid::parse_str(&s).ok()),
            root_cause_analysis: r.get("root_cause_analysis"),
            action_plan: r.get("action_plan"),
            verification_plan: r.get("verification_plan"),
            effectiveness_criteria: r.get("effectiveness_criteria"),
            target_completion_date: r.get::<Option<String>, _>("target_completion_date").and_then(|s| chrono::NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            actual_completion_date: None,
            effectiveness_result: r.get::<Option<i32>, _>("effectiveness_result").map(|v| v == 1),
        }).collect())
    }

    async fn update_capa(&self, capa: &CAPA) -> anyhow::Result<CAPA> {
        let now = Utc::now().to_rfc3339();
        sqlx::query(
            "UPDATE quality_capas SET title=?, description=?, priority=?, status=?, owner_id=?, 
             root_cause_analysis=?, action_plan=?, verification_plan=?, effectiveness_criteria=?, 
             target_completion_date=?, effectiveness_result=?, updated_at=? WHERE id=?"
        )
        .bind(&capa.title)
        .bind(&capa.description)
        .bind(format!("{:?}", capa.priority))
        .bind(format!("{:?}", capa.status))
        .bind(capa.owner_id.map(|id| id.to_string()))
        .bind(&capa.root_cause_analysis)
        .bind(&capa.action_plan)
        .bind(&capa.verification_plan)
        .bind(&capa.effectiveness_criteria)
        .bind(capa.target_completion_date.map(|d| d.to_string()))
        .bind(capa.effectiveness_result.map(|b| if b { 1 } else { 0 }))
        .bind(&now)
        .bind(capa.base.id.to_string())
        .execute(&self.pool)
        .await?;
        Ok(capa.clone())
    }

    async fn get_next_capa_number(&self) -> anyhow::Result<String> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM quality_capas")
            .fetch_one(&self.pool)
            .await?;
        Ok(format!("CAPA-{:06}", count.0 + 1))
    }

    async fn create_capa_action(&self, action: &CAPAAction) -> anyhow::Result<CAPAAction> {
        sqlx::query(
            r#"INSERT INTO quality_capa_actions 
               (id, capa_id, action_type, description, assigned_to, due_date, completed_at, status, evidence)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(action.id.to_string())
        .bind(action.capa_id.to_string())
        .bind(&action.action_type)
        .bind(&action.description)
        .bind(action.assigned_to.map(|id| id.to_string()))
        .bind(action.due_date.to_string())
        .bind(action.completed_at.map(|d| d.to_rfc3339()))
        .bind(format!("{:?}", action.status))
        .bind(&action.evidence)
        .execute(&self.pool)
        .await?;
        Ok(action.clone())
    }

    async fn list_capa_actions(&self, capa_id: Uuid) -> anyhow::Result<Vec<CAPAAction>> {
        let rows = sqlx::query_as::<_, (String, String, String, String, Option<String>, String, Option<String>, String, Option<String>)>(
            "SELECT id, capa_id, action_type, description, assigned_to, due_date, completed_at, status, evidence 
             FROM quality_capa_actions WHERE capa_id = ?"
        )
        .bind(capa_id.to_string())
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| CAPAAction {
            id: Uuid::parse_str(&r.0).unwrap_or_default(),
            capa_id: Uuid::parse_str(&r.1).unwrap_or_default(),
            action_type: r.2,
            description: r.3,
            assigned_to: r.4.and_then(|s| Uuid::parse_str(&s).ok()),
            due_date: chrono::NaiveDate::parse_from_str(&r.5, "%Y-%m-%d").unwrap_or_else(|_| chrono::Utc::now().date_naive()),
            completed_at: r.6.and_then(|s| chrono::DateTime::parse_from_rfc3339(&s).map(|d| d.with_timezone(&Utc)).ok()),
            status: parse_capa_action_status(&r.7),
            evidence: r.8,
        }).collect())
    }

    async fn update_capa_action(&self, action: &CAPAAction) -> anyhow::Result<CAPAAction> {
        sqlx::query(
            "UPDATE quality_capa_actions SET description=?, assigned_to=?, due_date=?, completed_at=?, status=?, evidence=? WHERE id=?"
        )
        .bind(&action.description)
        .bind(action.assigned_to.map(|id| id.to_string()))
        .bind(action.due_date.to_string())
        .bind(action.completed_at.map(|d| d.to_rfc3339()))
        .bind(format!("{:?}", action.status))
        .bind(&action.evidence)
        .bind(action.id.to_string())
        .execute(&self.pool)
        .await?;
        Ok(action.clone())
    }
}

fn parse_capa_source(s: &str) -> CAPASource {
    match s {
        "NCR" => CAPASource::NCR,
        "Audit" => CAPASource::Audit,
        "CustomerComplaint" => CAPASource::CustomerComplaint,
        "ManagementReview" => CAPASource::ManagementReview,
        "RiskAssessment" => CAPASource::RiskAssessment,
        "TrendAnalysis" => CAPASource::TrendAnalysis,
        _ => CAPASource::Other,
    }
}

fn parse_capa_status(s: &str) -> CAPAStatus {
    match s {
        "Draft" => CAPAStatus::Draft,
        "Investigation" => CAPAStatus::Investigation,
        "ActionPlan" => CAPAStatus::ActionPlan,
        "Implementation" => CAPAStatus::Implementation,
        "Verification" => CAPAStatus::Verification,
        "EffectivenessReview" => CAPAStatus::EffectivenessReview,
        "Closed" => CAPAStatus::Closed,
        _ => CAPAStatus::Cancelled,
    }
}

fn parse_capa_action_status(s: &str) -> CAPAActionStatus {
    match s {
        "Pending" => CAPAActionStatus::Pending,
        "InProgress" => CAPAActionStatus::InProgress,
        "Completed" => CAPAActionStatus::Completed,
        _ => CAPAActionStatus::Cancelled,
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

fn parse_calibration_status(s: &str) -> CalibrationStatus {
    match s {
        "Pending" => CalibrationStatus::Pending,
        "InProgress" => CalibrationStatus::InProgress,
        "Passed" => CalibrationStatus::Passed,
        "Failed" => CalibrationStatus::Failed,
        "Overdue" => CalibrationStatus::Overdue,
        _ => CalibrationStatus::Cancelled,
    }
}
