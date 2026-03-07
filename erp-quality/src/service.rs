use crate::models::*;
use crate::repository::{QualityRepository, SqliteQualityRepository};
use chrono::Utc;
use erp_core::{BaseEntity, Result, Error};
use sqlx::SqlitePool;
use tracing::info;
use uuid::Uuid;

pub struct QualityService<R: QualityRepository = SqliteQualityRepository> {
    repo: R,
}

impl QualityService<SqliteQualityRepository> {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            repo: SqliteQualityRepository::new(pool),
        }
    }
}

impl<R: QualityRepository> QualityService<R> {
    pub fn with_repo(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_inspection(&self, req: CreateInspectionRequest, created_by: Option<Uuid>) -> Result<QualityInspectionWithItems> {
        if req.items.is_empty() {
            return Err(Error::validation("At least one inspection item is required"));
        }

        let inspection_number = self.repo.get_next_inspection_number().await?;
        let now = Utc::now();
        let inspection_id = Uuid::new_v4();

        let inspection = QualityInspection {
            base: BaseEntity {
                id: inspection_id,
                created_at: now,
                updated_at: now,
                created_by,
                updated_by: None,
            },
            inspection_number,
            inspection_type: req.inspection_type,
            entity_type: req.entity_type,
            entity_id: req.entity_id,
            inspector_id: req.inspector_id,
            inspection_date: req.inspection_date,
            status: InspectionStatus::Pending,
            result: None,
            notes: req.notes,
        };

        let inspection = self.repo.create_inspection(&inspection).await?;

        let mut items = Vec::new();
        for item_req in &req.items {
            let item = InspectionItem {
                id: Uuid::new_v4(),
                inspection_id,
                criterion: item_req.criterion.clone(),
                expected_value: item_req.expected_value.clone(),
                actual_value: item_req.actual_value.clone(),
                pass_fail: item_req.pass_fail,
                notes: item_req.notes.clone(),
                created_at: now,
            };
            items.push(self.repo.add_inspection_item(&item).await?);
        }

        info!("Created quality inspection {} with {} items", inspection.inspection_number, items.len());
        Ok(QualityInspectionWithItems { inspection, items })
    }

    pub async fn get_inspection(&self, id: Uuid) -> Result<Option<QualityInspectionWithItems>> {
        let inspection = match self.repo.get_inspection(id).await? {
            Some(i) => i,
            None => return Ok(None),
        };
        let items = self.repo.get_inspection_items(id).await?;
        Ok(Some(QualityInspectionWithItems { inspection, items }))
    }

    pub async fn list_inspections(&self, status: Option<InspectionStatus>, inspection_type: Option<InspectionType>) -> Result<Vec<QualityInspection>> {
        self.repo.list_inspections(status, inspection_type).await
    }

    pub async fn start_inspection(&self, id: Uuid) -> Result<QualityInspection> {
        let mut inspection = self.repo.get_inspection(id).await?
            .ok_or_else(|| Error::not_found("Inspection", &id.to_string()))?;

        if inspection.status != InspectionStatus::Pending {
            return Err(Error::business_rule("Only pending inspections can be started"));
        }

        inspection.status = InspectionStatus::InProgress;
        inspection.base.updated_at = Utc::now();
        self.repo.update_inspection(&inspection).await
    }

    pub async fn complete_inspection(&self, id: Uuid) -> Result<QualityInspection> {
        let mut inspection = self.repo.get_inspection(id).await?
            .ok_or_else(|| Error::not_found("Inspection", &id.to_string()))?;

        if inspection.status != InspectionStatus::InProgress && inspection.status != InspectionStatus::Pending {
            return Err(Error::business_rule("Only pending or in-progress inspections can be completed"));
        }

        let items = self.repo.get_inspection_items(id).await?;
        if items.iter().any(|i| i.pass_fail.is_none()) {
            return Err(Error::validation("All inspection items must be evaluated before completing"));
        }

        let all_passed = items.iter().all(|i| i.pass_fail == Some(true));
        inspection.result = if all_passed {
            Some(InspectionResult::Pass)
        } else {
            Some(InspectionResult::Fail)
        };
        inspection.status = if all_passed { InspectionStatus::Passed } else { InspectionStatus::Failed };
        inspection.base.updated_at = Utc::now();

        info!("Completed quality inspection {} with result: {:?}", inspection.inspection_number, inspection.result);
        self.repo.update_inspection(&inspection).await
    }

    pub async fn cancel_inspection(&self, id: Uuid) -> Result<QualityInspection> {
        let mut inspection = self.repo.get_inspection(id).await?
            .ok_or_else(|| Error::not_found("Inspection", &id.to_string()))?;

        if inspection.status == InspectionStatus::Passed || inspection.status == InspectionStatus::Failed {
            return Err(Error::business_rule("Completed inspections cannot be cancelled"));
        }

        inspection.status = InspectionStatus::Cancelled;
        inspection.base.updated_at = Utc::now();
        self.repo.update_inspection(&inspection).await
    }

    pub async fn delete_inspection(&self, id: Uuid) -> Result<()> {
        let inspection = self.repo.get_inspection(id).await?
            .ok_or_else(|| Error::not_found("Inspection", &id.to_string()))?;

        if inspection.status != InspectionStatus::Pending && inspection.status != InspectionStatus::Cancelled {
            return Err(Error::business_rule("Only pending or cancelled inspections can be deleted"));
        }

        self.repo.delete_inspection(id).await?;
        info!("Deleted quality inspection {}", inspection.inspection_number);
        Ok(())
    }

    pub async fn update_inspection_item(&self, inspection_id: Uuid, item_id: Uuid, req: UpdateInspectionItemRequest) -> Result<InspectionItem> {
        let inspection = self.repo.get_inspection(inspection_id).await?
            .ok_or_else(|| Error::not_found("Inspection", &inspection_id.to_string()))?;

        if inspection.status != InspectionStatus::Pending && inspection.status != InspectionStatus::InProgress {
            return Err(Error::business_rule("Can only update items on pending or in-progress inspections"));
        }

        let mut items = self.repo.get_inspection_items(inspection_id).await?;
        let item = items.iter_mut().find(|i| i.id == item_id)
            .ok_or_else(|| Error::not_found("InspectionItem", &item_id.to_string()))?;

        if let Some(actual_value) = req.actual_value {
            item.actual_value = Some(actual_value);
        }
        if let Some(pass_fail) = req.pass_fail {
            item.pass_fail = Some(pass_fail);
        }
        if let Some(notes) = req.notes {
            item.notes = Some(notes);
        }

        self.repo.update_inspection_item(item).await
    }

    pub async fn add_inspection_item(&self, inspection_id: Uuid, req: CreateInspectionItemRequest) -> Result<InspectionItem> {
        let inspection = self.repo.get_inspection(inspection_id).await?
            .ok_or_else(|| Error::not_found("Inspection", &inspection_id.to_string()))?;

        if inspection.status != InspectionStatus::Pending && inspection.status != InspectionStatus::InProgress {
            return Err(Error::business_rule("Can only add items to pending or in-progress inspections"));
        }

        let item = InspectionItem {
            id: Uuid::new_v4(),
            inspection_id,
            criterion: req.criterion,
            expected_value: req.expected_value,
            actual_value: req.actual_value,
            pass_fail: req.pass_fail,
            notes: req.notes,
            created_at: Utc::now(),
        };

        self.repo.add_inspection_item(&item).await
    }

    pub async fn get_inspection_items(&self, inspection_id: Uuid) -> Result<Vec<InspectionItem>> {
        self.repo.get_inspection_items(inspection_id).await
    }

    pub async fn create_ncr(&self, req: CreateNCRRequest, created_by: Option<Uuid>) -> Result<NonConformanceReport> {
        let ncr_number = self.repo.get_next_ncr_number().await?;
        let now = Utc::now();

        let ncr = NonConformanceReport {
            base: BaseEntity {
                id: Uuid::new_v4(),
                created_at: now,
                updated_at: now,
                created_by,
                updated_by: None,
            },
            ncr_number,
            source_type: req.source_type,
            source_id: req.source_id,
            product_id: req.product_id,
            description: req.description,
            severity: req.severity,
            status: NCRStatus::Open,
            assigned_to: req.assigned_to,
            root_cause: None,
            corrective_action: None,
            preventive_action: None,
            resolution_date: None,
        };

        let ncr = self.repo.create_ncr(&ncr).await?;
        info!("Created NCR {} with severity: {:?}", ncr.ncr_number, ncr.severity);
        Ok(ncr)
    }

    pub async fn get_ncr(&self, id: Uuid) -> Result<Option<NonConformanceReport>> {
        self.repo.get_ncr(id).await
    }

    pub async fn list_ncrs(&self, status: Option<NCRStatus>, severity: Option<NCRSeverity>) -> Result<Vec<NonConformanceReport>> {
        self.repo.list_ncrs(status, severity).await
    }

    pub async fn update_ncr(&self, id: Uuid, req: UpdateNCRRequest) -> Result<NonConformanceReport> {
        let mut ncr = self.repo.get_ncr(id).await?
            .ok_or_else(|| Error::not_found("NCR", &id.to_string()))?;

        if let Some(root_cause) = req.root_cause {
            ncr.root_cause = Some(root_cause);
        }
        if let Some(corrective_action) = req.corrective_action {
            ncr.corrective_action = Some(corrective_action);
        }
        if let Some(preventive_action) = req.preventive_action {
            ncr.preventive_action = Some(preventive_action);
        }
        if let Some(status) = req.status {
            ncr.status = status.clone();
            if status == NCRStatus::Closed {
                ncr.resolution_date = Some(chrono::Utc::now().date_naive());
            }
        }

        ncr.base.updated_at = Utc::now();
        self.repo.update_ncr(&ncr).await
    }

    pub async fn close_ncr(&self, id: Uuid) -> Result<NonConformanceReport> {
        let mut ncr = self.repo.get_ncr(id).await?
            .ok_or_else(|| Error::not_found("NCR", &id.to_string()))?;

        if ncr.status == NCRStatus::Closed || ncr.status == NCRStatus::Cancelled {
            return Err(Error::business_rule("NCR is already closed or cancelled"));
        }

        ncr.status = NCRStatus::Closed;
        ncr.resolution_date = Some(chrono::Utc::now().date_naive());
        ncr.base.updated_at = Utc::now();

        info!("Closed NCR {}", ncr.ncr_number);
        self.repo.update_ncr(&ncr).await
    }

    pub async fn delete_ncr(&self, id: Uuid) -> Result<()> {
        let ncr = self.repo.get_ncr(id).await?
            .ok_or_else(|| Error::not_found("NCR", &id.to_string()))?;

        if ncr.status != NCRStatus::Open {
            return Err(Error::business_rule("Only open NCRs can be deleted"));
        }

        self.repo.delete_ncr(id).await?;
        info!("Deleted NCR {}", ncr.ncr_number);
        Ok(())
    }

    pub async fn get_analytics(&self) -> Result<QualityAnalytics> {
        let inspections = self.repo.list_inspections(None, None).await?;
        let ncrs = self.repo.list_ncrs(None, None).await?;

        let total_inspections = inspections.len() as i64;
        let passed_inspections = inspections.iter().filter(|i| i.status == InspectionStatus::Passed).count() as i64;
        let failed_inspections = inspections.iter().filter(|i| i.status == InspectionStatus::Failed).count() as i64;
        let pass_rate = if total_inspections > 0 {
            (passed_inspections as f64 / total_inspections as f64) * 100.0
        } else {
            0.0
        };

        let total_ncrs = ncrs.len() as i64;
        let open_ncrs = ncrs.iter().filter(|n| n.status != NCRStatus::Closed && n.status != NCRStatus::Cancelled).count() as i64;
        let closed_ncrs = ncrs.iter().filter(|n| n.status == NCRStatus::Closed).count() as i64;

        let mut ncrs_by_severity: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        for ncr in &ncrs {
            let key = format!("{:?}", ncr.severity);
            *ncrs_by_severity.entry(key).or_insert(0) += 1;
        }

        let mut inspections_by_type: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
        for inspection in &inspections {
            let key = format!("{:?}", inspection.inspection_type);
            *inspections_by_type.entry(key).or_insert(0) += 1;
        }

        Ok(QualityAnalytics {
            total_inspections,
            passed_inspections,
            failed_inspections,
            pass_rate,
            total_ncrs,
            open_ncrs,
            closed_ncrs,
            ncrs_by_severity: serde_json::to_value(ncrs_by_severity).unwrap_or(serde_json::json!({})),
            inspections_by_type: serde_json::to_value(inspections_by_type).unwrap_or(serde_json::json!({})),
        })
    }

    pub async fn create_calibration_device(&self, device: CalibrationDevice, created_by: Option<Uuid>) -> Result<CalibrationDevice> {
        let now = Utc::now();
        let mut device = device;
        device.base.created_at = now;
        device.base.updated_at = now;
        device.base.created_by = created_by;
        
        if device.next_calibration_date.is_none() {
            device.next_calibration_date = Some(Utc::now().date_naive() + chrono::Duration::days(device.calibration_frequency_days as i64));
        }

        self.repo.create_calibration_device(&device).await
    }

    pub async fn get_calibration_device(&self, id: Uuid) -> Result<Option<CalibrationDevice>> {
        self.repo.get_calibration_device(id).await
    }

    pub async fn list_calibration_devices(&self, status: Option<CalibrationStatus>) -> Result<Vec<CalibrationDevice>> {
        self.repo.list_calibration_devices(status).await
    }

    pub async fn calibrate_device(&self, device_id: Uuid, readings: Vec<CalibrationReading>, calibrated_by: Option<Uuid>) -> Result<CalibrationRecordWithReadings> {
        let mut device = self.repo.get_calibration_device(device_id).await?
            .ok_or_else(|| Error::not_found("CalibrationDevice", &device_id.to_string()))?;
        
        let record_number = self.repo.get_next_calibration_record_number().await?;
        let now = Utc::now();
        let record_id = Uuid::new_v4();
        
        let mut record = CalibrationRecord {
            base: BaseEntity {
                id: record_id,
                created_at: now,
                updated_at: now,
                created_by: calibrated_by,
                updated_by: None,
            },
            record_number,
            device_id,
            calibration_date: now.date_naive(),
            calibrated_by,
            status: CalibrationStatus::InProgress,
            certificate_number: None,
            notes: None,
        };
        
        let all_passed = readings.iter().all(|r| r.pass_fail);
        record.status = if all_passed { CalibrationStatus::Passed } else { CalibrationStatus::Failed };
        
        let record = self.repo.create_calibration_record(&record).await?;
        
        let mut saved_readings = Vec::new();
        for mut reading in readings {
            reading.id = Uuid::new_v4();
            reading.record_id = record_id;
            saved_readings.push(self.repo.add_calibration_reading(&reading).await?);
        }
        
        // Update device
        device.last_calibration_date = Some(record.calibration_date);
        device.next_calibration_date = Some(record.calibration_date + chrono::Duration::days(device.calibration_frequency_days as i64));
        device.status = record.status.clone();
        device.base.updated_at = now;
        self.repo.update_calibration_device(&device).await?;
        
        Ok(CalibrationRecordWithReadings { record, readings: saved_readings })
    }

    pub async fn get_calibration_record(&self, id: Uuid) -> Result<Option<CalibrationRecordWithReadings>> {
        let record = match self.repo.get_calibration_record(id).await? {
            Some(r) => r,
            None => return Ok(None),
        };
        let readings = self.repo.get_calibration_readings(id).await?;
        Ok(Some(CalibrationRecordWithReadings { record, readings }))
    }

    pub async fn create_capa(&self, req: CreateCAPARequest, created_by: Option<Uuid>) -> Result<CAPA> {
        let capa_number = self.repo.get_next_capa_number().await?;
        let now = Utc::now();

        let capa = CAPA {
            base: BaseEntity {
                id: Uuid::new_v4(),
                created_at: now,
                updated_at: now,
                created_by,
                updated_by: None,
            },
            capa_number,
            title: req.title,
            source_type: req.source_type,
            source_id: req.source_id,
            description: req.description,
            priority: req.priority,
            status: CAPAStatus::Draft,
            initiator_id: req.initiator_id,
            owner_id: None,
            root_cause_analysis: None,
            action_plan: None,
            verification_plan: None,
            effectiveness_criteria: None,
            target_completion_date: None,
            actual_completion_date: None,
            effectiveness_result: None,
        };

        let capa = self.repo.create_capa(&capa).await?;
        info!("Created CAPA {} from source {:?}", capa.capa_number, capa.source_type);
        Ok(capa)
    }

    pub async fn get_capa(&self, id: Uuid) -> Result<Option<CAPA>> {
        self.repo.get_capa(id).await
    }

    pub async fn list_capas(&self, status: Option<CAPAStatus>, priority: Option<NCRSeverity>) -> Result<Vec<CAPA>> {
        self.repo.list_capas(status, priority).await
    }

    pub async fn update_capa(&self, id: Uuid, req: UpdateCAPARequest) -> Result<CAPA> {
        let mut capa = self.repo.get_capa(id).await?
            .ok_or_else(|| Error::not_found("CAPA", &id.to_string()))?;

        if let Some(title) = req.title { capa.title = title; }
        if let Some(description) = req.description { capa.description = description; }
        if let Some(priority) = req.priority { capa.priority = priority; }
        if let Some(owner_id) = req.owner_id { capa.owner_id = Some(owner_id); }
        if let Some(root_cause) = req.root_cause_analysis { capa.root_cause_analysis = Some(root_cause); }
        if let Some(action_plan) = req.action_plan { capa.action_plan = Some(action_plan); }
        if let Some(verification_plan) = req.verification_plan { capa.verification_plan = Some(verification_plan); }
        if let Some(criteria) = req.effectiveness_criteria { capa.effectiveness_criteria = Some(criteria); }
        if let Some(date) = req.target_completion_date { capa.target_completion_date = Some(date); }
        if let Some(result) = req.effectiveness_result { capa.effectiveness_result = Some(result); }
        if let Some(status) = req.status { capa.status = status; }

        capa.base.updated_at = Utc::now();
        self.repo.update_capa(&capa).await
    }

    pub async fn add_capa_action(&self, capa_id: Uuid, description: String, action_type: String, due_date: chrono::NaiveDate, assigned_to: Option<Uuid>) -> Result<CAPAAction> {
        let action = CAPAAction {
            id: Uuid::new_v4(),
            capa_id,
            action_type,
            description,
            assigned_to,
            due_date,
            completed_at: None,
            status: CAPAActionStatus::Pending,
            evidence: None,
        };
        self.repo.create_capa_action(&action).await
    }

    pub async fn update_capa_action(&self, capa_id: Uuid, action_id: Uuid, status: CAPAActionStatus, evidence: Option<String>) -> Result<CAPAAction> {
        let actions = self.repo.list_capa_actions(capa_id).await?;
        let mut action = actions.into_iter().find(|a| a.id == action_id)
            .ok_or_else(|| Error::not_found("CAPAAction", &action_id.to_string()))?;

        action.status = status.clone();
        if status == CAPAActionStatus::Completed {
            action.completed_at = Some(Utc::now());
        }
        if let Some(e) = evidence {
            action.evidence = Some(e);
        }

        self.repo.update_capa_action(&action).await
    }

    pub async fn complete_capa_action(&self, capa_id: Uuid, action_id: Uuid, evidence: String) -> Result<CAPAAction> {
        self.update_capa_action(capa_id, action_id, CAPAActionStatus::Completed, Some(evidence)).await
    }

    pub async fn list_capa_actions(&self, capa_id: Uuid) -> Result<Vec<CAPAAction>> {
        self.repo.list_capa_actions(capa_id).await
    }
}
