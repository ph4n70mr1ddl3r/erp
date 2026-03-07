#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::repository::QualityRepository;
    use crate::service::QualityService;
    use anyhow::Result;
    use async_trait::async_trait;
    use chrono::Utc;
    use erp_core::BaseEntity;
    use uuid::Uuid;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    struct MockQualityRepository {
        devices: Arc<Mutex<Vec<CalibrationDevice>>>,
        records: Arc<Mutex<Vec<CalibrationRecord>>>,
        readings: Arc<Mutex<Vec<CalibrationReading>>>,
    }

    impl MockQualityRepository {
        fn new() -> Self {
            Self {
                devices: Arc::new(Mutex::new(Vec::new())),
                records: Arc::new(Mutex::new(Vec::new())),
                readings: Arc::new(Mutex::new(Vec::new())),
            }
        }
    }

    #[async_trait]
    impl QualityRepository for MockQualityRepository {
        async fn create_inspection(&self, _i: &QualityInspection) -> Result<QualityInspection> { todo!() }
        async fn get_inspection(&self, _id: Uuid) -> Result<Option<QualityInspection>> { todo!() }
        async fn list_inspections(&self, _s: Option<InspectionStatus>, _t: Option<InspectionType>) -> Result<Vec<QualityInspection>> { todo!() }
        async fn update_inspection(&self, _i: &QualityInspection) -> Result<QualityInspection> { todo!() }
        async fn delete_inspection(&self, _id: Uuid) -> Result<()> { todo!() }
        async fn add_inspection_item(&self, _item: &InspectionItem) -> Result<InspectionItem> { todo!() }
        async fn get_inspection_items(&self, _id: Uuid) -> Result<Vec<InspectionItem>> { todo!() }
        async fn update_inspection_item(&self, _item: &InspectionItem) -> Result<InspectionItem> { todo!() }
        async fn delete_inspection_items(&self, _id: Uuid) -> Result<()> { todo!() }
        async fn get_next_inspection_number(&self) -> Result<String> { todo!() }
        async fn create_ncr(&self, _ncr: &NonConformanceReport) -> Result<NonConformanceReport> { todo!() }
        async fn get_ncr(&self, _id: Uuid) -> Result<Option<NonConformanceReport>> { todo!() }
        async fn list_ncrs(&self, _s: Option<NCRStatus>, _sev: Option<NCRSeverity>) -> Result<Vec<NonConformanceReport>> { todo!() }
        async fn update_ncr(&self, _ncr: &NonConformanceReport) -> Result<NonConformanceReport> { todo!() }
        async fn delete_ncr(&self, _id: Uuid) -> Result<()> { todo!() }
        async fn get_next_ncr_number(&self) -> Result<String> { todo!() }

        async fn create_calibration_device(&self, device: &CalibrationDevice) -> Result<CalibrationDevice> {
            let mut devices: tokio::sync::MutexGuard<'_, Vec<CalibrationDevice>> = self.devices.lock().await;
            devices.push(device.clone());
            Ok(device.clone())
        }
        async fn get_calibration_device(&self, id: Uuid) -> Result<Option<CalibrationDevice>> {
            let devices: tokio::sync::MutexGuard<'_, Vec<CalibrationDevice>> = self.devices.lock().await;
            Ok(devices.iter().find(|d| d.base.id == id).cloned())
        }
        async fn list_calibration_devices(&self, _s: Option<CalibrationStatus>) -> Result<Vec<CalibrationDevice>> {
            let devices: tokio::sync::MutexGuard<'_, Vec<CalibrationDevice>> = self.devices.lock().await;
            Ok(devices.clone())
        }
        async fn update_calibration_device(&self, device: &CalibrationDevice) -> Result<CalibrationDevice> {
            let mut devices: tokio::sync::MutexGuard<'_, Vec<CalibrationDevice>> = self.devices.lock().await;
            if let Some(d) = devices.iter_mut().find(|d| d.base.id == device.base.id) {
                *d = device.clone();
            }
            Ok(device.clone())
        }
        async fn create_calibration_record(&self, record: &CalibrationRecord) -> Result<CalibrationRecord> {
            let mut records: tokio::sync::MutexGuard<'_, Vec<CalibrationRecord>> = self.records.lock().await;
            records.push(record.clone());
            Ok(record.clone())
        }
        async fn get_calibration_record(&self, id: Uuid) -> Result<Option<CalibrationRecord>> {
            let records: tokio::sync::MutexGuard<'_, Vec<CalibrationRecord>> = self.records.lock().await;
            Ok(records.iter().find(|r| r.base.id == id).cloned())
        }
        async fn add_calibration_reading(&self, reading: &CalibrationReading) -> Result<CalibrationReading> {
            let mut readings: tokio::sync::MutexGuard<'_, Vec<CalibrationReading>> = self.readings.lock().await;
            readings.push(reading.clone());
            Ok(reading.clone())
        }
        async fn get_calibration_readings(&self, record_id: Uuid) -> Result<Vec<CalibrationReading>> {
            let readings: tokio::sync::MutexGuard<'_, Vec<CalibrationReading>> = self.readings.lock().await;
            Ok(readings.iter().filter(|r| r.record_id == record_id).cloned().collect())
        }
        async fn get_next_calibration_record_number(&self) -> Result<String> {
            Ok("CAL-000001".to_string())
        }

        async fn create_capa(&self, capa: &CAPA) -> Result<CAPA> { Ok(capa.clone()) }
        async fn get_capa(&self, _id: Uuid) -> Result<Option<CAPA>> { Ok(None) }
        async fn list_capas(&self, _s: Option<CAPAStatus>, _p: Option<NCRSeverity>) -> Result<Vec<CAPA>> { Ok(vec![]) }
        async fn update_capa(&self, capa: &CAPA) -> Result<CAPA> { Ok(capa.clone()) }
        async fn get_next_capa_number(&self) -> Result<String> { Ok("CAPA-000001".to_string()) }
        async fn create_capa_action(&self, action: &CAPAAction) -> Result<CAPAAction> { Ok(action.clone()) }
        async fn list_capa_actions(&self, _id: Uuid) -> Result<Vec<CAPAAction>> { Ok(vec![]) }
        async fn update_capa_action(&self, action: &CAPAAction) -> Result<CAPAAction> { Ok(action.clone()) }
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
}
