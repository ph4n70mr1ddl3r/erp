#[cfg(test)]
mod tests {
    use crate::models::*;
    use crate::repository::*;
    use crate::service::PLMService;
    use anyhow::Result;
    use async_trait::async_trait;
    use uuid::Uuid;

    struct MockPLMRepository;

    #[async_trait]
    impl PLMRepository for MockPLMRepository {
        async fn create_item(&self, _item: &PLMItem) -> Result<()> { Ok(()) }
        async fn get_item(&self, _id: Uuid) -> Result<Option<PLMItem>> { Ok(None) }
        async fn list_items(&self, _status: Option<ItemStatus>, _limit: i32, _offset: i32) -> Result<Vec<PLMItem>> { Ok(vec![]) }
        async fn update_item(&self, _item: &PLMItem) -> Result<()> { Ok(()) }
        async fn create_document(&self, _doc: &PLMDocument) -> Result<()> { Ok(()) }
        async fn get_document(&self, _id: Uuid) -> Result<Option<PLMDocument>> { Ok(None) }
        async fn list_documents(&self, _item_id: Option<Uuid>) -> Result<Vec<PLMDocument>> { Ok(vec![]) }
        async fn update_document(&self, _doc: &PLMDocument) -> Result<()> { Ok(()) }
        async fn create_ecr(&self, _ecr: &EngineeringChangeRequest) -> Result<()> { Ok(()) }
        async fn get_ecr(&self, _id: Uuid) -> Result<Option<EngineeringChangeRequest>> { Ok(None) }
        async fn list_ecrs(&self, _status: Option<ChangeRequestStatus>) -> Result<Vec<EngineeringChangeRequest>> { Ok(vec![]) }
        async fn update_ecr(&self, _ecr: &EngineeringChangeRequest) -> Result<()> { Ok(()) }
        async fn create_ecn(&self, _ecn: &EngineeringChangeNotice) -> Result<()> { Ok(()) }
        async fn get_ecn(&self, _id: Uuid) -> Result<Option<EngineeringChangeNotice>> { Ok(None) }
        async fn list_ecns(&self, _ecr_id: Option<Uuid>) -> Result<Vec<EngineeringChangeNotice>> { Ok(vec![]) }
        async fn update_ecn(&self, _ecn: &EngineeringChangeNotice) -> Result<()> { Ok(()) }
        async fn create_ecn_affected_item(&self, _item: &ECNAffectedItem) -> Result<()> { Ok(()) }
        async fn list_ecn_affected_items(&self, _ecn_id: Uuid) -> Result<Vec<ECNAffectedItem>> { Ok(vec![]) }
        async fn create_bom(&self, _bom: &PLMBOM) -> Result<()> { Ok(()) }
        async fn get_bom(&self, _id: Uuid) -> Result<Option<PLMBOM>> { Ok(None) }
        async fn list_boms(&self, _item_id: Option<Uuid>) -> Result<Vec<PLMBOM>> { Ok(vec![]) }
        async fn create_bom_line(&self, _line: &PLMBOMLine) -> Result<()> { Ok(()) }
        async fn list_bom_lines(&self, _bom_id: Uuid) -> Result<Vec<PLMBOMLine>> { Ok(vec![]) }
        async fn create_specification(&self, _spec: &Specification) -> Result<()> { Ok(()) }
        async fn get_specification(&self, _id: Uuid) -> Result<Option<Specification>> { Ok(None) }
        async fn list_specifications(&self, _item_id: Option<Uuid>) -> Result<Vec<Specification>> { Ok(vec![]) }
        async fn create_spec_parameter(&self, _param: &SpecificationParameter) -> Result<()> { Ok(()) }
        async fn list_spec_parameters(&self, _spec_id: Uuid) -> Result<Vec<SpecificationParameter>> { Ok(vec![]) }
        async fn create_design_review(&self, _review: &DesignReview) -> Result<()> { Ok(()) }
        async fn get_design_review(&self, _id: Uuid) -> Result<Option<DesignReview>> { Ok(None) }
        async fn list_design_reviews(&self, _item_id: Uuid) -> Result<Vec<DesignReview>> { Ok(vec![]) }
        async fn update_design_review(&self, _review: &DesignReview) -> Result<()> { Ok(()) }
        async fn create_compliance_requirement(&self, _req: &ComplianceRequirement) -> Result<()> { Ok(()) }
        async fn list_compliance_requirements(&self) -> Result<Vec<ComplianceRequirement>> { Ok(vec![]) }
        async fn create_item_compliance(&self, _compliance: &ItemCompliance) -> Result<()> { Ok(()) }
        async fn list_item_compliances(&self, _item_id: Uuid) -> Result<Vec<ItemCompliance>> { Ok(vec![]) }
        
        async fn create_workflow(&self, _workflow: &PLMWorkflow) -> Result<()> { Ok(()) }
        async fn get_workflow(&self, _id: Uuid) -> Result<Option<PLMWorkflow>> { Ok(None) }
        async fn update_workflow(&self, _workflow: &PLMWorkflow) -> Result<()> { Ok(()) }
        async fn create_workflow_step(&self, _step: &PLMWorkflowStep) -> Result<()> { Ok(()) }
        async fn list_workflow_steps(&self, _workflow_id: Uuid) -> Result<Vec<PLMWorkflowStep>> { Ok(vec![]) }
        async fn update_workflow_step(&self, _step: &PLMWorkflowStep) -> Result<()> { Ok(()) }
        async fn create_cad_file(&self, _cad: &CADFile) -> Result<()> { Ok(()) }
        async fn get_cad_file(&self, _id: Uuid) -> Result<Option<CADFile>> { Ok(None) }
        async fn list_cad_files(&self, _document_id: Uuid) -> Result<Vec<CADFile>> { Ok(vec![]) }

        // IP Management
        async fn create_ip_asset(&self, _asset: &IPAsset) -> Result<()> { Ok(()) }
        async fn get_ip_asset(&self, _id: Uuid) -> Result<Option<IPAsset>> { Ok(None) }
        async fn list_ip_assets(&self, _ip_type: Option<IPType>, _status: Option<IPStatus>) -> Result<Vec<IPAsset>> { Ok(vec![]) }
        async fn update_ip_asset(&self, _asset: &IPAsset) -> Result<()> { Ok(()) }
        async fn create_ip_filing(&self, _filing: &IPFiling) -> Result<()> { Ok(()) }
        async fn list_ip_filings(&self, _ip_id: Uuid) -> Result<Vec<IPFiling>> { Ok(vec![]) }
        async fn create_ip_maintenance(&self, _maintenance: &IPMaintenance) -> Result<()> { Ok(()) }
        async fn list_ip_maintenance(&self, _ip_id: Uuid) -> Result<Vec<IPMaintenance>> { Ok(vec![]) }
        async fn create_ip_item_link(&self, _link: &IPItemLink) -> Result<()> { Ok(()) }
        async fn list_ip_item_links(&self, _ip_id: Uuid) -> Result<Vec<IPItemLink>> { Ok(vec![]) }
    }

    #[tokio::test]
    async fn test_create_ip_asset() -> Result<()> {
        let service = PLMService::new(MockPLMRepository);
        let req = CreateIPAssetRequest {
            title: "Smart Widget Patent".to_string(),
            description: Some("A new smart widget".to_string()),
            ip_type: IPType::Patent,
            jurisdiction: "US".to_string(),
            internal_ref: Some("IP-2026-001".to_string()),
            owner_id: Some(Uuid::new_v4()),
            inventor_ids: vec![Uuid::new_v4()],
        };
        let asset = service.create_ip_asset(req).await?;
        assert_eq!(asset.title, "Smart Widget Patent");
        assert_eq!(asset.status, IPStatus::Discovery);
        Ok(())
    }
}
