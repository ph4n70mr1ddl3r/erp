#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::repository::*;
    use crate::service::GRCService;
    use anyhow::Result;
    use async_trait::async_trait;
    use uuid::Uuid;

    struct MockGRCRepository;

    #[async_trait]
    impl GRCRepository for MockGRCRepository {
        async fn create_hs_code(&self, _hs_code: &HSCode) -> Result<()> { Ok(()) }
        async fn get_hs_code(&self, _id: Uuid) -> Result<Option<HSCode>> { Ok(None) }
        async fn list_hs_codes(&self) -> Result<Vec<HSCode>> { Ok(vec![]) }
        async fn create_product_trade_data(&self, _data: &ProductTradeData) -> Result<()> { Ok(()) }
        async fn get_product_trade_data(&self, _product_id: Uuid) -> Result<Option<ProductTradeData>> { Ok(None) }
        async fn update_product_trade_data(&self, _data: &ProductTradeData) -> Result<()> { Ok(()) }
        async fn create_trade_license(&self, _license: &TradeLicense) -> Result<()> { Ok(()) }
        async fn get_trade_license(&self, _id: Uuid) -> Result<Option<TradeLicense>> { Ok(None) }
        async fn list_trade_licenses(&self, _entity_id: Uuid) -> Result<Vec<TradeLicense>> { Ok(vec![]) }
        async fn create_screening_result(&self, _result: &ScreeningResult) -> Result<()> { Ok(()) }
        async fn get_latest_screening_result(&self, _entity_id: Uuid) -> Result<Option<ScreeningResult>> { Ok(None) }

        // DSAR Management
        async fn create_dsar_request(&self, _request: &DSARRequest) -> Result<()> { Ok(()) }
        async fn get_dsar_request(&self, _id: Uuid) -> Result<Option<DSARRequest>> { Ok(None) }
        async fn list_dsar_requests(&self, _status: Option<DSARStatus>) -> Result<Vec<DSARRequest>> { Ok(vec![]) }
        async fn update_dsar_status(&self, _id: Uuid, _status: DSARStatus) -> Result<()> { Ok(()) }
        async fn create_dsar_task(&self, _task: &DSARTask) -> Result<()> { Ok(()) }
        async fn list_dsar_tasks(&self, _request_id: Uuid) -> Result<Vec<DSARTask>> { Ok(vec![]) }
        async fn update_dsar_task(&self, _task: &DSARTask) -> Result<()> { Ok(()) }
    }

    #[tokio::test]
    async fn test_create_hs_code() -> Result<()> {
        let service = GRCService::new(MockGRCRepository);
        let req = CreateHSCodeRequest {
            code: "8471.30.0100".to_string(),
            description: "Portable digital automatic data processing machines".to_string(),
            general_duty_rate: 0.0,
        };
        let hs_code = service.create_hs_code(req).await?;
        assert_eq!(hs_code.code, "8471.30.0100");
        assert_eq!(hs_code.general_duty_rate, 0.0);
        Ok(())
    }

    #[tokio::test]
    async fn test_screening_entity() -> Result<()> {
        let service = GRCService::new(MockGRCRepository);
        let entity_id = Uuid::new_v4();
        let result = service.screening_entity(entity_id, "Vendor".to_string()).await?;
        assert_eq!(result.entity_id, entity_id);
        assert_eq!(result.status, ScreeningStatus::Clear);
        Ok(())
    }

    #[tokio::test]
    async fn test_create_dsar_request() -> Result<()> {
        let service = GRCService::new(MockGRCRepository);
        let subject_id = Uuid::new_v4();
        let req = CreateDSARRequest {
            subject_id,
            subject_type: "Employee".to_string(),
            request_type: DSARType::Erasure,
            identity_proof_ref: Some("REF-123".to_string()),
        };
        let request = service.create_dsar_request(req).await?;
        assert_eq!(request.subject_id, subject_id);
        assert_eq!(request.status, DSARStatus::New);
        assert_eq!(request.request_type, DSARType::Erasure);
        Ok(())
    }
}
