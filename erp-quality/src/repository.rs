use crate::models::*;
use async_trait::async_trait;
use chrono::Utc;
use erp_core::{BaseEntity, Result};
use sqlx::{Row, SqlitePool};
use uuid::Uuid;

#[async_trait]
pub trait QualityRepository: Send + Sync {
    async fn create_inspection(&self, inspection: &QualityInspection) -> Result<QualityInspection>;
    async fn get_inspection(&self, id: Uuid) -> Result<Option<QualityInspection>>;
    async fn list_inspections(&self, status: Option<InspectionStatus>, inspection_type: Option<InspectionType>) -> Result<Vec<QualityInspection>>;
    async fn update_inspection(&self, inspection: &QualityInspection) -> Result<QualityInspection>;
    async fn delete_inspection(&self, id: Uuid) -> Result<()>;
    async fn add_inspection_item(&self, item: &InspectionItem) -> Result<InspectionItem>;
    async fn get_inspection_items(&self, inspection_id: Uuid) -> Result<Vec<InspectionItem>>;
    async fn update_inspection_item(&self, item: &InspectionItem) -> Result<InspectionItem>;
    async fn delete_inspection_items(&self, inspection_id: Uuid) -> Result<()>;
    async fn get_next_inspection_number(&self) -> Result<String>;
    
    async fn create_ncr(&self, ncr: &NonConformanceReport) -> Result<NonConformanceReport>;
    async fn get_ncr(&self, id: Uuid) -> Result<Option<NonConformanceReport>>;
    async fn list_ncrs(&self, status: Option<NCRStatus>, severity: Option<NCRSeverity>) -> Result<Vec<NonConformanceReport>>;
    async fn update_ncr(&self, ncr: &NonConformanceReport) -> Result<NonConformanceReport>;
    async fn delete_ncr(&self, id: Uuid) -> Result<()>;
    async fn get_next_ncr_number(&self) -> Result<String>;

    async fn create_calibration_device(&self, device: &CalibrationDevice) -> Result<CalibrationDevice>;
    async fn get_calibration_device(&self, id: Uuid) -> Result<Option<CalibrationDevice>>;
    async fn list_calibration_devices(&self, status: Option<CalibrationStatus>) -> Result<Vec<CalibrationDevice>>;
    async fn update_calibration_device(&self, device: &CalibrationDevice) -> Result<CalibrationDevice>;
    async fn create_calibration_record(&self, record: &CalibrationRecord) -> Result<CalibrationRecord>;
    async fn get_calibration_record(&self, id: Uuid) -> Result<Option<CalibrationRecord>>;
    async fn add_calibration_reading(&self, reading: &CalibrationReading) -> Result<CalibrationReading>;
    async fn get_calibration_readings(&self, record_id: Uuid) -> Result<Vec<CalibrationReading>>;
    async fn get_next_calibration_record_number(&self) -> Result<String>;

    // CAPA Management
    async fn create_capa(&self, capa: &CAPA) -> Result<CAPA>;
    async fn get_capa(&self, id: Uuid) -> Result<Option<CAPA>>;
    async fn list_capas(&self, status: Option<CAPAStatus>, priority: Option<NCRSeverity>) -> Result<Vec<CAPA>>;
    async fn update_capa(&self, capa: &CAPA) -> Result<CAPA>;
    async fn get_next_capa_number(&self) -> Result<String>;
    
    async fn create_capa_action(&self, action: &CAPAAction) -> Result<CAPAAction>;
    async fn list_capa_actions(&self, capa_id: Uuid) -> Result<Vec<CAPAAction>>;
    async fn update_capa_action(&self, action: &CAPAAction) -> Result<CAPAAction>;
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
    async fn create_inspection(&self, inspection: &QualityInspection) -> Result<QualityInspection> {
        let now = Utc::now();
        sqlx::query(
            r#"INSERT INTO quality_inspections 
               (id, inspection_number, inspection_type, entity_type, entity_id, inspector_id, 
                inspection_date, status, result, notes, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(inspection.base.id)
        .bind(&inspection.inspection_number)
        .bind(&inspection.inspection_type)
        .bind(&inspection.entity_type)
        .bind(inspection.entity_id)
        .bind(inspection.inspector_id)
        .bind(inspection.inspection_date)
        .bind(&inspection.status)
        .bind(&inspection.result)
        .bind(&inspection.notes)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(inspection.clone())
    }

    async fn get_inspection(&self, id: Uuid) -> Result<Option<QualityInspection>> {
        let row = sqlx::query(
            "SELECT id, inspection_number, inspection_type, entity_type, entity_id, inspector_id, 
                    inspection_date, status, result, notes, created_at, updated_at 
             FROM quality_inspections WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| QualityInspection {
            base: BaseEntity {
                id: r.get("id"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                created_by: None,
                updated_by: None,
            },
            inspection_number: r.get("inspection_number"),
            inspection_type: r.get("inspection_type"),
            entity_type: r.get("entity_type"),
            entity_id: r.get("entity_id"),
            inspector_id: r.get("inspector_id"),
            inspection_date: r.get("inspection_date"),
            status: r.get("status"),
            result: r.get("result"),
            notes: r.get("notes"),
        }))
    }

    async fn list_inspections(&self, status: Option<InspectionStatus>, inspection_type: Option<InspectionType>) -> Result<Vec<QualityInspection>> {
        let mut query = "SELECT id, inspection_number, inspection_type, entity_type, entity_id, inspector_id, 
                        inspection_date, status, result, notes, created_at, updated_at 
                        FROM quality_inspections WHERE 1=1".to_string();

        if status.is_some() {
            query.push_str(" AND status = ?");
        }
        if inspection_type.is_some() {
            query.push_str(" AND inspection_type = ?");
        }
        query.push_str(" ORDER BY created_at DESC");

        let mut sql_query = sqlx::query(&query);
        if let Some(s) = status {
            sql_query = sql_query.bind(s);
        }
        if let Some(t) = inspection_type {
            sql_query = sql_query.bind(t);
        }

        let rows = sql_query.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| QualityInspection {
            base: BaseEntity {
                id: r.get("id"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                created_by: None,
                updated_by: None,
            },
            inspection_number: r.get("inspection_number"),
            inspection_type: r.get("inspection_type"),
            entity_type: r.get("entity_type"),
            entity_id: r.get("entity_id"),
            inspector_id: r.get("inspector_id"),
            inspection_date: r.get("inspection_date"),
            status: r.get("status"),
            result: r.get("result"),
            notes: r.get("notes"),
        }).collect())
    }

    async fn update_inspection(&self, inspection: &QualityInspection) -> Result<QualityInspection> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE quality_inspections SET inspection_type=?, entity_type=?, entity_id=?, inspector_id=?, 
             inspection_date=?, status=?, result=?, notes=?, updated_at=? WHERE id=?"
        )
        .bind(&inspection.inspection_type)
        .bind(&inspection.entity_type)
        .bind(inspection.entity_id)
        .bind(inspection.inspector_id)
        .bind(inspection.inspection_date)
        .bind(&inspection.status)
        .bind(&inspection.result)
        .bind(&inspection.notes)
        .bind(now)
        .bind(inspection.base.id)
        .execute(&self.pool)
        .await?;
        Ok(inspection.clone())
    }

    async fn delete_inspection(&self, id: Uuid) -> Result<()> {
        self.delete_inspection_items(id).await?;
        sqlx::query("DELETE FROM quality_inspections WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn add_inspection_item(&self, item: &InspectionItem) -> Result<InspectionItem> {
        sqlx::query(
            r#"INSERT INTO inspection_items 
               (id, inspection_id, criterion, expected_value, actual_value, pass_fail, notes, created_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(item.id)
        .bind(item.inspection_id)
        .bind(&item.criterion)
        .bind(&item.expected_value)
        .bind(&item.actual_value)
        .bind(item.pass_fail)
        .bind(&item.notes)
        .bind(item.created_at)
        .execute(&self.pool)
        .await?;
        Ok(item.clone())
    }

    async fn get_inspection_items(&self, inspection_id: Uuid) -> Result<Vec<InspectionItem>> {
        let rows = sqlx::query(
            "SELECT id, inspection_id, criterion, expected_value, actual_value, pass_fail, notes, created_at 
             FROM inspection_items WHERE inspection_id = ?"
        )
        .bind(inspection_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| InspectionItem {
            id: r.get("id"),
            inspection_id: r.get("inspection_id"),
            criterion: r.get("criterion"),
            expected_value: r.get("expected_value"),
            actual_value: r.get("actual_value"),
            pass_fail: r.get("pass_fail"),
            notes: r.get("notes"),
            created_at: r.get("created_at"),
        }).collect())
    }

    async fn update_inspection_item(&self, item: &InspectionItem) -> Result<InspectionItem> {
        sqlx::query(
            "UPDATE inspection_items SET actual_value=?, pass_fail=?, notes=? WHERE id=?"
        )
        .bind(&item.actual_value)
        .bind(item.pass_fail)
        .bind(&item.notes)
        .bind(item.id)
        .execute(&self.pool)
        .await?;
        Ok(item.clone())
    }

    async fn delete_inspection_items(&self, inspection_id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM inspection_items WHERE inspection_id = ?")
            .bind(inspection_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_next_inspection_number(&self) -> Result<String> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM quality_inspections")
            .fetch_one(&self.pool)
            .await?;
        Ok(format!("QI-{:06}", count.0 + 1))
    }

    async fn create_ncr(&self, ncr: &NonConformanceReport) -> Result<NonConformanceReport> {
        let now = Utc::now();
        sqlx::query(
            r#"INSERT INTO non_conformance_reports 
               (id, ncr_number, source_type, source_id, product_id, description, severity, status, 
                assigned_to, root_cause, corrective_action, preventive_action, resolution_date, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(ncr.base.id)
        .bind(&ncr.ncr_number)
        .bind(&ncr.source_type)
        .bind(ncr.source_id)
        .bind(ncr.product_id)
        .bind(&ncr.description)
        .bind(&ncr.severity)
        .bind(&ncr.status)
        .bind(ncr.assigned_to)
        .bind(&ncr.root_cause)
        .bind(&ncr.corrective_action)
        .bind(&ncr.preventive_action)
        .bind(ncr.resolution_date)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(ncr.clone())
    }

    async fn get_ncr(&self, id: Uuid) -> Result<Option<NonConformanceReport>> {
        let row = sqlx::query(
            "SELECT id, ncr_number, source_type, source_id, product_id, description, severity, status, 
                    assigned_to, root_cause, corrective_action, preventive_action, resolution_date, created_at, updated_at
             FROM non_conformance_reports WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| NonConformanceReport {
            base: BaseEntity {
                id: r.get("id"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                created_by: None,
                updated_by: None,
            },
            ncr_number: r.get("ncr_number"),
            source_type: r.get("source_type"),
            source_id: r.get("source_id"),
            product_id: r.get("product_id"),
            description: r.get("description"),
            severity: r.get("severity"),
            status: r.get("status"),
            assigned_to: r.get("assigned_to"),
            root_cause: r.get("root_cause"),
            corrective_action: r.get("corrective_action"),
            preventive_action: r.get("preventive_action"),
            resolution_date: r.get("resolution_date"),
        }))
    }

    async fn list_ncrs(&self, status: Option<NCRStatus>, severity: Option<NCRSeverity>) -> Result<Vec<NonConformanceReport>> {
        let mut query = "SELECT id, ncr_number, source_type, source_id, product_id, description, severity, status, 
                        assigned_to, root_cause, corrective_action, preventive_action, resolution_date, created_at, updated_at
                        FROM non_conformance_reports WHERE 1=1".to_string();

        if status.is_some() {
            query.push_str(" AND status = ?");
        }
        if severity.is_some() {
            query.push_str(" AND severity = ?");
        }
        query.push_str(" ORDER BY created_at DESC");

        let mut sql_query = sqlx::query(&query);
        if let Some(s) = status {
            sql_query = sql_query.bind(s);
        }
        if let Some(sev) = severity {
            sql_query = sql_query.bind(sev);
        }

        let rows = sql_query.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| NonConformanceReport {
            base: BaseEntity {
                id: r.get("id"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                created_by: None,
                updated_by: None,
            },
            ncr_number: r.get("ncr_number"),
            source_type: r.get("source_type"),
            source_id: r.get("source_id"),
            product_id: r.get("product_id"),
            description: r.get("description"),
            severity: r.get("severity"),
            status: r.get("status"),
            assigned_to: r.get("assigned_to"),
            root_cause: r.get("root_cause"),
            corrective_action: r.get("corrective_action"),
            preventive_action: r.get("preventive_action"),
            resolution_date: r.get("resolution_date"),
        }).collect())
    }

    async fn update_ncr(&self, ncr: &NonConformanceReport) -> Result<NonConformanceReport> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE non_conformance_reports SET source_type=?, source_id=?, product_id=?, description=?, 
             severity=?, status=?, assigned_to=?, root_cause=?, corrective_action=?, preventive_action=?, 
             resolution_date=?, updated_at=? WHERE id=?"
        )
        .bind(&ncr.source_type)
        .bind(ncr.source_id)
        .bind(ncr.product_id)
        .bind(&ncr.description)
        .bind(&ncr.severity)
        .bind(&ncr.status)
        .bind(ncr.assigned_to)
        .bind(&ncr.root_cause)
        .bind(&ncr.corrective_action)
        .bind(&ncr.preventive_action)
        .bind(ncr.resolution_date)
        .bind(now)
        .bind(ncr.base.id)
        .execute(&self.pool)
        .await?;
        Ok(ncr.clone())
    }

    async fn delete_ncr(&self, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM non_conformance_reports WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn get_next_ncr_number(&self) -> Result<String> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM non_conformance_reports")
            .fetch_one(&self.pool)
            .await?;
        Ok(format!("NCR-{:06}", count.0 + 1))
    }

    async fn create_calibration_device(&self, device: &CalibrationDevice) -> Result<CalibrationDevice> {
        let now = Utc::now();
        sqlx::query(
            r#"INSERT INTO calibration_devices 
               (id, device_number, name, description, manufacturer, model, serial_number, 
                location, calibration_frequency_days, last_calibration_date, next_calibration_date, 
                status, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(device.base.id)
        .bind(&device.device_number)
        .bind(&device.name)
        .bind(&device.description)
        .bind(&device.manufacturer)
        .bind(&device.model)
        .bind(&device.serial_number)
        .bind(&device.location)
        .bind(device.calibration_frequency_days)
        .bind(device.last_calibration_date)
        .bind(device.next_calibration_date)
        .bind(&device.status)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(device.clone())
    }

    async fn get_calibration_device(&self, id: Uuid) -> Result<Option<CalibrationDevice>> {
        let row = sqlx::query(
            "SELECT id, device_number, name, description, manufacturer, model, serial_number, 
                    location, calibration_frequency_days, last_calibration_date, next_calibration_date, 
                    status, created_at, updated_at 
             FROM calibration_devices WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| CalibrationDevice {
            base: BaseEntity {
                id: r.get("id"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                created_by: None,
                updated_by: None,
            },
            device_number: r.get("device_number"),
            name: r.get("name"),
            description: r.get("description"),
            manufacturer: r.get("manufacturer"),
            model: r.get("model"),
            serial_number: r.get("serial_number"),
            location: r.get("location"),
            calibration_frequency_days: r.get("calibration_frequency_days"),
            last_calibration_date: r.get("last_calibration_date"),
            next_calibration_date: r.get("next_calibration_date"),
            status: r.get("status"),
        }))
    }

    async fn list_calibration_devices(&self, status: Option<CalibrationStatus>) -> Result<Vec<CalibrationDevice>> {
        let mut query = "SELECT id, device_number, name, description, manufacturer, model, serial_number, 
                        location, calibration_frequency_days, last_calibration_date, next_calibration_date, 
                        status, created_at, updated_at 
                        FROM calibration_devices WHERE 1=1".to_string();

        if status.is_some() {
            query.push_str(" AND status = ?");
        }
        query.push_str(" ORDER BY created_at DESC");

        let mut sql_query = sqlx::query(&query);
        if let Some(s) = status {
            sql_query = sql_query.bind(s);
        }

        let rows = sql_query.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| CalibrationDevice {
            base: BaseEntity {
                id: r.get("id"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                created_by: None,
                updated_by: None,
            },
            device_number: r.get("device_number"),
            name: r.get("name"),
            description: r.get("description"),
            manufacturer: r.get("manufacturer"),
            model: r.get("model"),
            serial_number: r.get("serial_number"),
            location: r.get("location"),
            calibration_frequency_days: r.get("calibration_frequency_days"),
            last_calibration_date: r.get("last_calibration_date"),
            next_calibration_date: r.get("next_calibration_date"),
            status: r.get("status"),
        }).collect())
    }

    async fn update_calibration_device(&self, device: &CalibrationDevice) -> Result<CalibrationDevice> {
        let now = Utc::now();
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
        .bind(device.last_calibration_date)
        .bind(device.next_calibration_date)
        .bind(&device.status)
        .bind(now)
        .bind(device.base.id)
        .execute(&self.pool)
        .await?;
        Ok(device.clone())
    }

    async fn create_calibration_record(&self, record: &CalibrationRecord) -> Result<CalibrationRecord> {
        let now = Utc::now();
        sqlx::query(
            r#"INSERT INTO calibration_records 
               (id, record_number, device_id, calibration_date, calibrated_by, status, 
                certificate_number, notes, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(record.base.id)
        .bind(&record.record_number)
        .bind(record.device_id)
        .bind(record.calibration_date)
        .bind(record.calibrated_by)
        .bind(&record.status)
        .bind(&record.certificate_number)
        .bind(&record.notes)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(record.clone())
    }

    async fn get_calibration_record(&self, id: Uuid) -> Result<Option<CalibrationRecord>> {
        let row = sqlx::query(
            "SELECT id, record_number, device_id, calibration_date, calibrated_by, status, 
                    certificate_number, notes, created_at, updated_at 
             FROM calibration_records WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| CalibrationRecord {
            base: BaseEntity {
                id: r.get("id"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                created_by: None,
                updated_by: None,
            },
            record_number: r.get("record_number"),
            device_id: r.get("device_id"),
            calibration_date: r.get("calibration_date"),
            calibrated_by: r.get("calibrated_by"),
            status: r.get("status"),
            certificate_number: r.get("certificate_number"),
            notes: r.get("notes"),
        }))
    }

    async fn add_calibration_reading(&self, reading: &CalibrationReading) -> Result<CalibrationReading> {
        sqlx::query(
            r#"INSERT INTO calibration_readings 
               (id, record_id, parameter, reference_value, actual_value, tolerance_min, 
                tolerance_max, pass_fail, uom)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(reading.id)
        .bind(reading.record_id)
        .bind(&reading.parameter)
        .bind(reading.reference_value)
        .bind(reading.actual_value)
        .bind(reading.tolerance_min)
        .bind(reading.tolerance_max)
        .bind(reading.pass_fail)
        .bind(&reading.uom)
        .execute(&self.pool)
        .await?;
        Ok(reading.clone())
    }

    async fn get_calibration_readings(&self, record_id: Uuid) -> Result<Vec<CalibrationReading>> {
        let rows = sqlx::query(
            "SELECT id, record_id, parameter, reference_value, actual_value, tolerance_min, 
                    tolerance_max, pass_fail, uom 
             FROM calibration_readings WHERE record_id = ?"
        )
        .bind(record_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| CalibrationReading {
            id: r.get("id"),
            record_id: r.get("record_id"),
            parameter: r.get("parameter"),
            reference_value: r.get("reference_value"),
            actual_value: r.get("actual_value"),
            tolerance_min: r.get("tolerance_min"),
            tolerance_max: r.get("tolerance_max"),
            pass_fail: r.get("pass_fail"),
            uom: r.get("uom"),
        }).collect())
    }

    async fn get_next_calibration_record_number(&self) -> Result<String> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM calibration_records")
            .fetch_one(&self.pool)
            .await?;
        Ok(format!("CAL-{:06}", count.0 + 1))
    }

    // CAPA Management
    async fn create_capa(&self, capa: &CAPA) -> Result<CAPA> {
        let now = Utc::now();
        sqlx::query(
            r#"INSERT INTO quality_capas 
               (id, capa_number, title, source_type, source_id, description, priority, status, 
                initiator_id, owner_id, root_cause_analysis, action_plan, verification_plan, 
                effectiveness_criteria, target_completion_date, actual_completion_date, effectiveness_result, created_at, updated_at)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(capa.base.id)
        .bind(&capa.capa_number)
        .bind(&capa.title)
        .bind(&capa.source_type)
        .bind(capa.source_id)
        .bind(&capa.description)
        .bind(&capa.priority)
        .bind(&capa.status)
        .bind(capa.initiator_id)
        .bind(capa.owner_id)
        .bind(&capa.root_cause_analysis)
        .bind(&capa.action_plan)
        .bind(&capa.verification_plan)
        .bind(&capa.effectiveness_criteria)
        .bind(capa.target_completion_date)
        .bind(capa.actual_completion_date)
        .bind(capa.effectiveness_result)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;
        Ok(capa.clone())
    }

    async fn get_capa(&self, id: Uuid) -> Result<Option<CAPA>> {
        let row = sqlx::query(
            "SELECT id, capa_number, title, source_type, source_id, description, priority, status, 
                    initiator_id, owner_id, root_cause_analysis, action_plan, verification_plan, 
                    effectiveness_criteria, target_completion_date, actual_completion_date, effectiveness_result, created_at, updated_at
             FROM quality_capas WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| CAPA {
            base: BaseEntity {
                id: r.get("id"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                created_by: None,
                updated_by: None,
            },
            capa_number: r.get("capa_number"),
            title: r.get("title"),
            source_type: r.get("source_type"),
            source_id: r.get("source_id"),
            description: r.get("description"),
            priority: r.get("priority"),
            status: r.get("status"),
            initiator_id: r.get("initiator_id"),
            owner_id: r.get("owner_id"),
            root_cause_analysis: r.get("root_cause_analysis"),
            action_plan: r.get("action_plan"),
            verification_plan: r.get("verification_plan"),
            effectiveness_criteria: r.get("effectiveness_criteria"),
            target_completion_date: r.get("target_completion_date"),
            actual_completion_date: r.get("actual_completion_date"),
            effectiveness_result: r.get("effectiveness_result"),
        }))
    }

    async fn list_capas(&self, status: Option<CAPAStatus>, priority: Option<NCRSeverity>) -> Result<Vec<CAPA>> {
        let mut query = "SELECT id, capa_number, title, source_type, source_id, description, priority, status, 
                        initiator_id, owner_id, root_cause_analysis, action_plan, verification_plan, 
                        effectiveness_criteria, target_completion_date, actual_completion_date, effectiveness_result, created_at, updated_at
                        FROM quality_capas WHERE 1=1".to_string();

        if status.is_some() {
            query.push_str(" AND status = ?");
        }
        if priority.is_some() {
            query.push_str(" AND priority = ?");
        }
        query.push_str(" ORDER BY created_at DESC");

        let mut sql_query = sqlx::query(&query);
        if let Some(s) = status {
            sql_query = sql_query.bind(s);
        }
        if let Some(p) = priority {
            sql_query = sql_query.bind(p);
        }

        let rows = sql_query.fetch_all(&self.pool).await?;
        Ok(rows.into_iter().map(|r| CAPA {
            base: BaseEntity {
                id: r.get("id"),
                created_at: r.get("created_at"),
                updated_at: r.get("updated_at"),
                created_by: None,
                updated_by: None,
            },
            capa_number: r.get("capa_number"),
            title: r.get("title"),
            source_type: r.get("source_type"),
            source_id: r.get("source_id"),
            description: r.get("description"),
            priority: r.get("priority"),
            status: r.get("status"),
            initiator_id: r.get("initiator_id"),
            owner_id: r.get("owner_id"),
            root_cause_analysis: r.get("root_cause_analysis"),
            action_plan: r.get("action_plan"),
            verification_plan: r.get("verification_plan"),
            effectiveness_criteria: r.get("effectiveness_criteria"),
            target_completion_date: r.get("target_completion_date"),
            actual_completion_date: r.get("actual_completion_date"),
            effectiveness_result: r.get("effectiveness_result"),
        }).collect())
    }

    async fn update_capa(&self, capa: &CAPA) -> Result<CAPA> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE quality_capas SET title=?, description=?, priority=?, owner_id=?, 
             root_cause_analysis=?, action_plan=?, verification_plan=?, effectiveness_criteria=?, 
             target_completion_date=?, actual_completion_date=?, effectiveness_result=?, status=?, updated_at=? WHERE id=?"
        )
        .bind(&capa.title)
        .bind(&capa.description)
        .bind(&capa.priority)
        .bind(capa.owner_id)
        .bind(&capa.root_cause_analysis)
        .bind(&capa.action_plan)
        .bind(&capa.verification_plan)
        .bind(&capa.effectiveness_criteria)
        .bind(capa.target_completion_date)
        .bind(capa.actual_completion_date)
        .bind(capa.effectiveness_result)
        .bind(&capa.status)
        .bind(now)
        .bind(capa.base.id)
        .execute(&self.pool)
        .await?;
        Ok(capa.clone())
    }

    async fn get_next_capa_number(&self) -> Result<String> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM quality_capas")
            .fetch_one(&self.pool)
            .await?;
        Ok(format!("CAPA-{:06}", count.0 + 1))
    }

    async fn create_capa_action(&self, action: &CAPAAction) -> Result<CAPAAction> {
        sqlx::query(
            r#"INSERT INTO quality_capa_actions 
               (id, capa_id, action_type, description, assigned_to, due_date, completed_at, status, evidence)
               VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"#
        )
        .bind(action.id)
        .bind(action.capa_id)
        .bind(&action.action_type)
        .bind(&action.description)
        .bind(action.assigned_to)
        .bind(action.due_date)
        .bind(action.completed_at)
        .bind(&action.status)
        .bind(&action.evidence)
        .execute(&self.pool)
        .await?;
        Ok(action.clone())
    }

    async fn list_capa_actions(&self, capa_id: Uuid) -> Result<Vec<CAPAAction>> {
        let rows = sqlx::query(
            "SELECT id, capa_id, action_type, description, assigned_to, due_date, completed_at, status, evidence 
             FROM quality_capa_actions WHERE capa_id = ?"
        )
        .bind(capa_id)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| CAPAAction {
            id: r.get("id"),
            capa_id: r.get("capa_id"),
            action_type: r.get("action_type"),
            description: r.get("description"),
            assigned_to: r.get("assigned_to"),
            due_date: r.get("due_date"),
            completed_at: r.get("completed_at"),
            status: r.get("status"),
            evidence: r.get("evidence"),
        }).collect())
    }

    async fn update_capa_action(&self, action: &CAPAAction) -> Result<CAPAAction> {
        sqlx::query(
            "UPDATE quality_capa_actions SET description=?, assigned_to=?, due_date=?, completed_at=?, status=?, evidence=? WHERE id=?"
        )
        .bind(&action.description)
        .bind(action.assigned_to)
        .bind(action.due_date)
        .bind(action.completed_at)
        .bind(&action.status)
        .bind(&action.evidence)
        .bind(action.id)
        .execute(&self.pool)
        .await?;
        Ok(action.clone())
    }
}

