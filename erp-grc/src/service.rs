use crate::models::*;
use crate::repository::{GRCRepository, SqliteGRCRepository};
use chrono::Utc;
use erp_core::{BaseEntity, Status};
use uuid::Uuid;

pub struct GRCService<R: GRCRepository> {
    repo: R,
}

impl<R: GRCRepository> GRCService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_hs_code(&self, req: CreateHSCodeRequest) -> anyhow::Result<HSCode> {
        let hs_code = HSCode {
            id: Uuid::new_v4(),
            code: req.code,
            description: req.description,
            section: None,
            chapter: None,
            heading: None,
            subheading: None,
            general_duty_rate: req.general_duty_rate,
            status: erp_core::Status::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_hs_code(&hs_code).await?;
        Ok(hs_code)
    }

    pub async fn get_hs_code(&self, id: Uuid) -> anyhow::Result<Option<HSCode>> {
        self.repo.get_hs_code(id).await
    }

    pub async fn set_product_trade_data(&self, product_id: Uuid, req: UpdateProductTradeDataRequest) -> anyhow::Result<ProductTradeData> {
        let existing = self.repo.get_product_trade_data(product_id).await?;
        let now = Utc::now();
        
        let data = if let Some(mut data) = existing {
            data.hs_code_id = req.hs_code_id;
            data.country_of_origin = req.country_of_origin;
            data.eccn = req.eccn;
            data.export_license_required = req.export_license_required;
            data.import_license_required = req.import_license_required;
            data.updated_at = now;
            self.repo.update_product_trade_data(&data).await?;
            data
        } else {
            let data = ProductTradeData {
                id: Uuid::new_v4(),
                product_id,
                hs_code_id: req.hs_code_id,
                country_of_origin: req.country_of_origin,
                eccn: req.eccn,
                export_license_required: req.export_license_required,
                import_license_required: req.import_license_required,
                dual_use: false,
                scheduled_b_number: None,
                created_at: now,
                updated_at: now,
            };
            self.repo.create_product_trade_data(&data).await?;
            data
        };
        
        Ok(data)
    }

    pub async fn screening_entity(&self, entity_id: Uuid, entity_type: String) -> anyhow::Result<ScreeningResult> {
        // In a real system, this would call an external API (like Dow Jones or LexisNexis)
        let result = ScreeningResult {
            id: Uuid::new_v4(),
            entity_id,
            entity_type,
            screening_date: Utc::now(),
            status: ScreeningStatus::Clear,
            source: "Internal Screening Engine".to_string(),
            match_count: 0,
            match_details: None,
            expiration_date: Some(Utc::now() + chrono::Duration::days(30)),
        };
        self.repo.create_screening_result(&result).await?;
        Ok(result)
    }

    pub async fn create_dsar_request(&self, req: CreateDSARRequest) -> anyhow::Result<DSARRequest> {
        let now = Utc::now();
        let request_number = format!("DSAR-{}", now.format("%Y%m%d%H%M%S"));
        let request = DSARRequest {
            base: BaseEntity::new(),
            request_number,
            subject_id: req.subject_id,
            subject_type: req.subject_type,
            request_type: req.request_type,
            status: DSARStatus::New,
            requested_date: now,
            due_date: now + chrono::Duration::days(30), // standard 30 day limit
            completed_date: None,
            assigned_to: None,
            identity_proof_ref: req.identity_proof_ref,
            notes: None,
        };
        self.repo.create_dsar_request(&request).await?;
        
        // Auto-generate tasks based on request type
        let modules = vec!["HR", "Sales", "Finance", "Auth"];
        for module in modules {
            let task = DSARTask {
                id: Uuid::new_v4(),
                request_id: request.base.id,
                module_name: module.to_string(),
                task_description: format!("Identify and process data for {} in module {}", request.subject_type, module),
                status: Status::Active,
                assigned_to: None,
                completed_at: None,
                result_metadata: None,
            };
            self.repo.create_dsar_task(&task).await?;
        }

        Ok(request)
    }

    pub async fn get_dsar_request(&self, id: Uuid) -> anyhow::Result<Option<DSARRequest>> {
        self.repo.get_dsar_request(id).await
    }

    pub async fn list_dsar_tasks(&self, request_id: Uuid) -> anyhow::Result<Vec<DSARTask>> {
        self.repo.list_dsar_tasks(request_id).await
    }
}
