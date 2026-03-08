#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::models::*;
    use crate::repository::QualityRepository;
    use crate::service::QualityService;
    use async_trait::async_trait;
    use chrono::Utc;
    use erp_core::{BaseEntity, Result};
    use uuid::Uuid;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct MockQualityRepository {
        inspections: Arc<Mutex<Vec<QualityInspection>>>,
        items: Arc<Mutex<Vec<InspectionItem>>>,
        ncrs: Arc<Mutex<Vec<NonConformanceReport>>>,
        devices: Arc<Mutex<Vec<CalibrationDevice>>>,
        records: Arc<Mutex<Vec<CalibrationRecord>>>,
        readings: Arc<Mutex<Vec<CalibrationReading>>>,
        capas: Arc<Mutex<Vec<CAPA>>>,
        capa_actions: Arc<Mutex<Vec<CAPAAction>>>,
    }

    impl MockQualityRepository {
        fn new() -> Self {
            Self {
                inspections: Arc::new(Mutex::new(Vec::new())),
                items: Arc::new(Mutex::new(Vec::new())),
                ncrs: Arc::new(Mutex::new(Vec::new())),
                devices: Arc::new(Mutex::new(Vec::new())),
                records: Arc::new(Mutex::new(Vec::new())),
                readings: Arc::new(Mutex::new(Vec::new())),
                capas: Arc::new(Mutex::new(Vec::new())),
                capa_actions: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    #[async_trait]
    impl QualityRepository for MockQualityRepository {
        async fn create_inspection(&self, i: &QualityInspection) -> Result<QualityInspection> {
            let mut inspections = self.inspections.lock().await;
            inspections.push(i.clone());
            Ok(i.clone())
        }
        async fn get_inspection(&self, id: Uuid) -> Result<Option<QualityInspection>> {
            let inspections = self.inspections.lock().await;
            Ok(inspections.iter().find(|i| i.base.id == id).cloned())
        }
        async fn list_inspections(&self, _s: Option<InspectionStatus>, _t: Option<InspectionType>) -> Result<Vec<QualityInspection>> {
            let inspections = self.inspections.lock().await;
            Ok(inspections.clone())
        }
        async fn update_inspection(&self, i: &QualityInspection) -> Result<QualityInspection> {
            let mut inspections = self.inspections.lock().await;
            if let Some(ins) = inspections.iter_mut().find(|ins| ins.base.id == i.base.id) {
                *ins = i.clone();
            }
            Ok(i.clone())
        }
        async fn delete_inspection(&self, id: Uuid) -> Result<()> {
            let mut inspections = self.inspections.lock().await;
            inspections.retain(|i| i.base.id != id);
            Ok(())
        }
        async fn add_inspection_item(&self, item: &InspectionItem) -> Result<InspectionItem> {
            let mut items = self.items.lock().await;
            items.push(item.clone());
            Ok(item.clone())
        }
        async fn get_inspection_items(&self, id: Uuid) -> Result<Vec<InspectionItem>> {
            let items = self.items.lock().await;
            Ok(items.iter().filter(|i| i.inspection_id == id).cloned().collect())
        }
        async fn update_inspection_item(&self, item: &InspectionItem) -> Result<InspectionItem> {
            let mut items = self.items.lock().await;
            if let Some(i) = items.iter_mut().find(|i| i.id == item.id) {
                *i = item.clone();
            }
            Ok(item.clone())
        }
        async fn delete_inspection_items(&self, id: Uuid) -> Result<()> {
            let mut items = self.items.lock().await;
            items.retain(|i| i.inspection_id != id);
            Ok(())
        }
        async fn get_next_inspection_number(&self) -> Result<String> {
            let inspections = self.inspections.lock().await;
            Ok(format!("QI-{:06}", inspections.len() + 1))
        }
        async fn create_ncr(&self, ncr: &NonConformanceReport) -> Result<NonConformanceReport> {
            let mut ncrs = self.ncrs.lock().await;
            ncrs.push(ncr.clone());
            Ok(ncr.clone())
        }
        async fn get_ncr(&self, id: Uuid) -> Result<Option<NonConformanceReport>> {
            let ncrs = self.ncrs.lock().await;
            Ok(ncrs.iter().find(|n| n.base.id == id).cloned())
        }
        async fn list_ncrs(&self, _s: Option<NCRStatus>, _sev: Option<NCRSeverity>) -> Result<Vec<NonConformanceReport>> {
            let ncrs = self.ncrs.lock().await;
            Ok(ncrs.clone())
        }
        async fn update_ncr(&self, ncr: &NonConformanceReport) -> Result<NonConformanceReport> {
            let mut ncrs = self.ncrs.lock().await;
            if let Some(n) = ncrs.iter_mut().find(|n| n.base.id == ncr.base.id) {
                *n = ncr.clone();
            }
            Ok(ncr.clone())
        }
        async fn delete_ncr(&self, id: Uuid) -> Result<()> {
            let mut ncrs = self.ncrs.lock().await;
            ncrs.retain(|n| n.base.id != id);
            Ok(())
        }
        async fn get_next_ncr_number(&self) -> Result<String> {
            let ncrs = self.ncrs.lock().await;
            Ok(format!("NCR-{:06}", ncrs.len() + 1))
        }

        async fn create_calibration_device(&self, device: &CalibrationDevice) -> Result<CalibrationDevice> {
            let mut devices = self.devices.lock().await;
            devices.push(device.clone());
            Ok(device.clone())
        }
        async fn get_calibration_device(&self, id: Uuid) -> Result<Option<CalibrationDevice>> {
            let devices = self.devices.lock().await;
            Ok(devices.iter().find(|d| d.base.id == id).cloned())
        }
        async fn list_calibration_devices(&self, _s: Option<CalibrationStatus>) -> Result<Vec<CalibrationDevice>> {
            let devices = self.devices.lock().await;
            Ok(devices.clone())
        }
        async fn update_calibration_device(&self, device: &CalibrationDevice) -> Result<CalibrationDevice> {
            let mut devices = self.devices.lock().await;
            if let Some(d) = devices.iter_mut().find(|d| d.base.id == device.base.id) {
                *d = device.clone();
            }
            Ok(device.clone())
        }
        async fn create_calibration_record(&self, record: &CalibrationRecord) -> Result<CalibrationRecord> {
            let mut records = self.records.lock().await;
            records.push(record.clone());
            Ok(record.clone())
        }
        async fn get_calibration_record(&self, id: Uuid) -> Result<Option<CalibrationRecord>> {
            let records = self.records.lock().await;
            Ok(records.iter().find(|r| r.base.id == id).cloned())
        }
        async fn add_calibration_reading(&self, reading: &CalibrationReading) -> Result<CalibrationReading> {
            let mut readings = self.readings.lock().await;
            readings.push(reading.clone());
            Ok(reading.clone())
        }
        async fn get_calibration_readings(&self, record_id: Uuid) -> Result<Vec<CalibrationReading>> {
            let readings = self.readings.lock().await;
            Ok(readings.iter().filter(|r| r.record_id == record_id).cloned().collect())
        }
        async fn get_next_calibration_record_number(&self) -> Result<String> {
            Ok("CAL-000001".to_string())
        }

        async fn create_capa(&self, capa: &CAPA) -> Result<CAPA> {
            let mut capas = self.capas.lock().await;
            capas.push(capa.clone());
            Ok(capa.clone())
        }
        async fn get_capa(&self, id: Uuid) -> Result<Option<CAPA>> {
            let capas = self.capas.lock().await;
            Ok(capas.iter().find(|c| c.base.id == id).cloned())
        }
        async fn list_capas(&self, _s: Option<CAPAStatus>, _p: Option<NCRSeverity>) -> Result<Vec<CAPA>> {
            let capas = self.capas.lock().await;
            Ok(capas.clone())
        }
        async fn update_capa(&self, capa: &CAPA) -> Result<CAPA> {
            let mut capas = self.capas.lock().await;
            if let Some(c) = capas.iter_mut().find(|c| c.base.id == capa.base.id) {
                *c = capa.clone();
            }
            Ok(capa.clone())
        }
        async fn get_next_capa_number(&self) -> Result<String> {
            let capas = self.capas.lock().await;
            Ok(format!("CAPA-{:06}", capas.len() + 1))
        }
        async fn create_capa_action(&self, action: &CAPAAction) -> Result<CAPAAction> {
            let mut actions = self.capa_actions.lock().await;
            actions.push(action.clone());
            Ok(action.clone())
        }
        async fn list_capa_actions(&self, capa_id: Uuid) -> Result<Vec<CAPAAction>> {
            let actions = self.capa_actions.lock().await;
            Ok(actions.iter().filter(|a| a.capa_id == capa_id).cloned().collect())
        }
        async fn update_capa_action(&self, action: &CAPAAction) -> Result<CAPAAction> {
            let mut actions = self.capa_actions.lock().await;
            if let Some(a) = actions.iter_mut().find(|a| a.id == action.id) {
                *a = action.clone();
            }
            Ok(action.clone())
        }
    }

    #[tokio::test]
    async fn test_create_capa() -> Result<()> {
        let repo = MockQualityRepository::new();
        let service = QualityService::with_repo(repo);
        let initiator_id = Uuid::new_v4();

        let req = CreateCAPARequest {
            title: "Material Defect in Part A".to_string(),
            source_type: CAPASource::NCR,
            source_id: Some(Uuid::new_v4()),
            description: "High rate of fractures observed in Part A during final inspection".to_string(),
            priority: NCRSeverity::Major,
            initiator_id,
        };

        let capa = service.create_capa(req, Some(initiator_id)).await?;

        assert_eq!(capa.title, "Material Defect in Part A");
        assert_eq!(capa.status, CAPAStatus::Draft);
        assert_eq!(capa.initiator_id, initiator_id);
        
        Ok(())
    }

    #[tokio::test]
    async fn test_update_capa() -> Result<()> {
        let repo = MockQualityRepository::new();
        let service = QualityService::with_repo(repo);
        let initiator_id = Uuid::new_v4();

        let req = CreateCAPARequest {
            title: "Original Title".to_string(),
            source_type: CAPASource::Other,
            source_id: None,
            description: "Original description".to_string(),
            priority: NCRSeverity::Minor,
            initiator_id,
        };

        let capa = service.create_capa(req, Some(initiator_id)).await?;
        let capa_id = capa.base.id;

        let update_req = UpdateCAPARequest {
            title: Some("Updated Title".to_string()),
            description: None,
            priority: Some(NCRSeverity::Critical),
            owner_id: Some(Uuid::new_v4()),
            root_cause_analysis: Some("Found root cause".to_string()),
            action_plan: None,
            verification_plan: None,
            effectiveness_criteria: None,
            target_completion_date: None,
            effectiveness_result: None,
            status: Some(CAPAStatus::Investigation),
        };

        let updated_capa = service.update_capa(capa_id, update_req).await?;

        assert_eq!(updated_capa.title, "Updated Title");
        assert_eq!(updated_capa.priority, NCRSeverity::Critical);
        assert_eq!(updated_capa.status, CAPAStatus::Investigation);
        assert_eq!(updated_capa.root_cause_analysis, Some("Found root cause".to_string()));
        assert!(updated_capa.owner_id.is_some());

        Ok(())
    }

    #[tokio::test]
    async fn test_calibrate_device() -> Result<()> {
        let repo = MockQualityRepository::new();
        let service = QualityService::with_repo(repo);
        let device_id = Uuid::new_v4();

        let device = CalibrationDevice {
            base: BaseEntity::new_with_id(device_id),
            device_number: "DEV-001".to_string(),
            name: "Digital Caliper".to_string(),
            description: None,
            manufacturer: Some("Mitutoyo".to_string()),
            model: Some("CD-6\"CX".to_string()),
            serial_number: Some("123456".to_string()),
            location: Some("Lab A".to_string()),
            calibration_frequency_days: 365,
            last_calibration_date: None,
            next_calibration_date: None,
            status: CalibrationStatus::Pending,
        };

        service.create_calibration_device(device, None).await?;

        let readings = vec![
            CalibrationReading {
                id: Uuid::new_v4(),
                record_id: Uuid::nil(),
                parameter: "Accuracy".to_string(),
                reference_value: 10.0,
                actual_value: 10.01,
                tolerance_min: 9.99,
                tolerance_max: 10.01,
                pass_fail: true,
                uom: "mm".to_string(),
            }
        ];

        let result = service.calibrate_device(device_id, readings, None).await?;

        assert_eq!(result.record.status, CalibrationStatus::Passed);
        assert_eq!(result.readings.len(), 1);
        
        let updated_device = service.get_calibration_device(device_id).await?.unwrap();
        assert_eq!(updated_device.status, CalibrationStatus::Passed);
        assert!(updated_device.last_calibration_date.is_some());
        assert_eq!(updated_device.last_calibration_date.unwrap(), Utc::now().date_naive());

        Ok(())
    }

    #[tokio::test]
    async fn test_inspection_lifecycle() -> Result<()> {
        let repo = MockQualityRepository::new();
        let service = QualityService::with_repo(repo);
        let entity_id = Uuid::new_v4();

        let req = CreateInspectionRequest {
            inspection_type: InspectionType::Incoming,
            entity_type: "PurchaseOrder".to_string(),
            entity_id,
            inspector_id: Some(Uuid::new_v4()),
            inspection_date: Utc::now().date_naive(),
            notes: Some("Test inspection".to_string()),
            items: vec![
                CreateInspectionItemRequest {
                    criterion: "Dimension A".to_string(),
                    expected_value: Some("10mm".to_string()),
                    actual_value: None,
                    pass_fail: None,
                    notes: None,
                }
            ],
        };

        let inspection_with_items = service.create_inspection(req, None).await?;
        let inspection_id = inspection_with_items.inspection.base.id;
        let item_id = inspection_with_items.items[0].id;

        assert_eq!(inspection_with_items.inspection.status, InspectionStatus::Pending);

        // Start inspection
        let inspection = service.start_inspection(inspection_id).await?;
        assert_eq!(inspection.status, InspectionStatus::InProgress);

        // Update item
        service.update_inspection_item(inspection_id, item_id, UpdateInspectionItemRequest {
            actual_value: Some("10.01mm".to_string()),
            pass_fail: Some(true),
            notes: Some("Within tolerance".to_string()),
        }).await?;

        // Complete inspection
        let completed = service.complete_inspection(inspection_id).await?;
        assert_eq!(completed.status, InspectionStatus::Passed);
        assert_eq!(completed.result, Some(InspectionResult::Pass));

        Ok(())
    }

    #[tokio::test]
    async fn test_ncr_lifecycle() -> Result<()> {
        let repo = MockQualityRepository::new();
        let service = QualityService::with_repo(repo);
        let product_id = Uuid::new_v4();

        let req = CreateNCRRequest {
            source_type: NCRSource::ProductionIssue,
            source_id: None,
            product_id: Some(product_id),
            description: "Surface scratches on housing".to_string(),
            severity: NCRSeverity::Minor,
            assigned_to: Some(Uuid::new_v4()),
        };

        let ncr = service.create_ncr(req, None).await?;
        assert_eq!(ncr.status, NCRStatus::Open);

        // Update NCR
        let update_req = UpdateNCRRequest {
            root_cause: Some("Improper handling during assembly".to_string()),
            corrective_action: Some("Retrain assembly team".to_string()),
            preventive_action: Some("Add protective film during assembly".to_string()),
            status: Some(NCRStatus::CorrectiveAction),
        };

        let updated = service.update_ncr(ncr.base.id, update_req).await?;
        assert_eq!(updated.status, NCRStatus::CorrectiveAction);

        // Close NCR
        let closed = service.close_ncr(ncr.base.id).await?;
        assert_eq!(closed.status, NCRStatus::Closed);
        assert!(closed.resolution_date.is_some());

        Ok(())
    }
}
