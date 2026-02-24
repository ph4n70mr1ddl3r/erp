use crate::models::*;
use crate::repository::PLMRepository;
use chrono::Utc;
use uuid::Uuid;

pub struct PLMService<R: PLMRepository> {
    repo: R,
}

impl<R: PLMRepository> PLMService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_item(&self, req: CreatePLMItemRequest) -> anyhow::Result<PLMItem> {
        let now = Utc::now();
        let item_number = format!("PLM-{}", now.format("%Y%m%d%H%M%S"));
        let item = PLMItem {
            id: Uuid::new_v4(),
            item_number,
            name: req.name,
            description: req.description,
            category: req.category,
            status: ItemStatus::Draft,
            version: "1.0".to_string(),
            revision: 1,
            lifecycle_phase: "Concept".to_string(),
            owner_id: None,
            product_id: req.product_id,
            parent_item_id: req.parent_item_id,
            effective_date: None,
            obsolete_date: None,
            security_classification: req.security_classification,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_item(&item).await?;
        Ok(item)
    }

    pub async fn get_item(&self, id: Uuid) -> anyhow::Result<Option<PLMItem>> {
        self.repo.get_item(id).await
    }

    pub async fn list_items(&self, status: Option<ItemStatus>, page: i32, page_size: i32) -> anyhow::Result<Vec<PLMItem>> {
        let offset = (page - 1) * page_size;
        self.repo.list_items(status, page_size, offset).await
    }

    pub async fn release_item(&self, id: Uuid) -> anyhow::Result<PLMItem> {
        let mut item = self.repo.get_item(id).await?.ok_or_else(|| anyhow::anyhow!("Item not found"))?;
        item.status = ItemStatus::Released;
        item.effective_date = Some(Utc::now());
        item.updated_at = Utc::now();
        self.repo.update_item(&item).await?;
        Ok(item)
    }

    pub async fn obsolete_item(&self, id: Uuid) -> anyhow::Result<PLMItem> {
        let mut item = self.repo.get_item(id).await?.ok_or_else(|| anyhow::anyhow!("Item not found"))?;
        item.status = ItemStatus::Obsolete;
        item.obsolete_date = Some(Utc::now());
        item.updated_at = Utc::now();
        self.repo.update_item(&item).await?;
        Ok(item)
    }

    pub async fn create_ecr(&self, req: CreateECRRequest, requested_by: Uuid) -> anyhow::Result<EngineeringChangeRequest> {
        let now = Utc::now();
        let ecr_number = format!("ECR-{}", now.format("%Y%m%d%H%M%S"));
        let ecr = EngineeringChangeRequest {
            id: Uuid::new_v4(),
            ecr_number,
            title: req.title,
            description: req.description,
            reason: req.reason,
            priority: req.priority,
            status: ChangeRequestStatus::Draft,
            change_type: req.change_type,
            requested_by,
            submitted_at: None,
            target_date: req.target_date,
            implemented_date: None,
            impact_assessment: None,
            cost_estimate: req.cost_estimate,
            currency: req.currency,
            approved_by: None,
            approved_at: None,
            rejected_reason: None,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_ecr(&ecr).await?;
        Ok(ecr)
    }

    pub async fn submit_ecr(&self, id: Uuid) -> anyhow::Result<EngineeringChangeRequest> {
        let mut ecr = self.repo.get_ecr(id).await?.ok_or_else(|| anyhow::anyhow!("ECR not found"))?;
        if ecr.status != ChangeRequestStatus::Draft {
            return Err(anyhow::anyhow!("ECR must be in Draft status to submit"));
        }
        ecr.status = ChangeRequestStatus::Submitted;
        ecr.submitted_at = Some(Utc::now());
        ecr.updated_at = Utc::now();
        self.repo.update_ecr(&ecr).await?;
        Ok(ecr)
    }

    pub async fn approve_ecr(&self, id: Uuid, approved_by: Uuid) -> anyhow::Result<EngineeringChangeRequest> {
        let mut ecr = self.repo.get_ecr(id).await?.ok_or_else(|| anyhow::anyhow!("ECR not found"))?;
        if ecr.status != ChangeRequestStatus::UnderReview {
            return Err(anyhow::anyhow!("ECR must be UnderReview to approve"));
        }
        ecr.status = ChangeRequestStatus::Approved;
        ecr.approved_by = Some(approved_by);
        ecr.approved_at = Some(Utc::now());
        ecr.updated_at = Utc::now();
        self.repo.update_ecr(&ecr).await?;
        Ok(ecr)
    }

    pub async fn reject_ecr(&self, id: Uuid, reason: String) -> anyhow::Result<EngineeringChangeRequest> {
        let mut ecr = self.repo.get_ecr(id).await?.ok_or_else(|| anyhow::anyhow!("ECR not found"))?;
        ecr.status = ChangeRequestStatus::Rejected;
        ecr.rejected_reason = Some(reason);
        ecr.updated_at = Utc::now();
        self.repo.update_ecr(&ecr).await?;
        Ok(ecr)
    }

    pub async fn create_ecn(&self, ecr_id: Uuid, title: String, description: String,
        effective_date: chrono::DateTime<Utc>, created_by: Uuid) -> anyhow::Result<EngineeringChangeNotice> {
        let now = Utc::now();
        let ecn_number = format!("ECN-{}", now.format("%Y%m%d%H%M%S"));
        let ecn = EngineeringChangeNotice {
            id: Uuid::new_v4(),
            ecn_number,
            ecr_id,
            title,
            description,
            status: ChangeRequestStatus::Draft,
            effective_date,
            implementation_instructions: None,
            created_by,
            approved_by: None,
            approved_at: None,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_ecn(&ecn).await?;
        Ok(ecn)
    }

    pub async fn implement_ecn(&self, id: Uuid) -> anyhow::Result<EngineeringChangeNotice> {
        let mut ecn = self.repo.get_ecn(id).await?.ok_or_else(|| anyhow::anyhow!("ECN not found"))?;
        ecn.status = ChangeRequestStatus::Implemented;
        ecn.updated_at = Utc::now();
        self.repo.update_ecn(&ecn).await?;
        
        let affected_items = self.repo.list_ecn_affected_items(id).await?;
        for item in affected_items {
            if let Some(mut plm_item) = self.repo.get_item(item.item_id).await? {
                plm_item.version = item.new_version;
                plm_item.revision = item.new_revision.parse().unwrap_or(plm_item.revision);
                plm_item.updated_at = Utc::now();
                self.repo.update_item(&plm_item).await?;
            }
        }
        
        Ok(ecn)
    }

    pub async fn create_bom(&self, item_id: Uuid, name: String, description: Option<String>,
        bom_type: String, quantity: f64, unit_of_measure: String) -> anyhow::Result<PLMBOM> {
        let now = Utc::now();
        let bom_number = format!("BOM-{}", now.format("%Y%m%d%H%M%S"));
        let bom = PLMBOM {
            id: Uuid::new_v4(),
            bom_number,
            name,
            description,
            item_id,
            version: "1.0".to_string(),
            revision: 1,
            status: ItemStatus::Draft,
            bom_type,
            quantity,
            unit_of_measure,
            effective_date: None,
            obsolete_date: None,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_bom(&bom).await?;
        Ok(bom)
    }

    pub async fn add_bom_line(&self, bom_id: Uuid, item_id: Uuid, quantity: f64,
        unit_of_measure: String, sort_order: i32) -> anyhow::Result<PLMBOMLine> {
        let line = PLMBOMLine {
            id: Uuid::new_v4(),
            bom_id,
            item_id,
            line_number: sort_order,
            quantity,
            unit_of_measure,
            find_number: None,
            reference_designator: None,
            substitute_item_id: None,
            is_phantom: false,
            sort_order,
            created_at: Utc::now(),
        };
        self.repo.create_bom_line(&line).await?;
        Ok(line)
    }

    pub async fn create_specification(&self, req: CreateSpecificationRequest) -> anyhow::Result<Specification> {
        let now = Utc::now();
        let spec = Specification {
            id: Uuid::new_v4(),
            spec_number: req.spec_number,
            name: req.name,
            description: req.description,
            item_id: req.item_id,
            spec_type: req.spec_type,
            status: ItemStatus::Draft,
            version: "1.0".to_string(),
            revision: 1,
            parameters: serde_json::to_value(&req.parameters)?,
            owner_id: None,
            effective_date: None,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_specification(&spec).await?;
        
        for (i, param_req) in req.parameters.iter().enumerate() {
            let param = SpecificationParameter {
                id: Uuid::new_v4(),
                spec_id: spec.id,
                parameter_name: param_req.parameter_name.clone(),
                parameter_type: param_req.parameter_type.clone(),
                target_value: param_req.target_value.clone(),
                min_value: param_req.min_value.clone(),
                max_value: param_req.max_value.clone(),
                unit: param_req.unit.clone(),
                test_method: param_req.test_method.clone(),
                is_critical: param_req.is_critical,
                sort_order: i as i32,
                created_at: now,
            };
            self.repo.create_spec_parameter(&param).await?;
        }
        
        Ok(spec)
    }

    pub async fn create_design_review(&self, item_id: Uuid, review_type: String,
        scheduled_date: chrono::DateTime<Utc>, facilitator_id: Option<Uuid>) -> anyhow::Result<DesignReview> {
        let now = Utc::now();
        let review_number = format!("DR-{}", now.format("%Y%m%d%H%M%S"));
        let review = DesignReview {
            id: Uuid::new_v4(),
            review_number,
            item_id,
            review_type,
            status: "Scheduled".to_string(),
            scheduled_date,
            conducted_date: None,
            facilitator_id,
            location: None,
            outcome: None,
            action_items: None,
            created_at: now,
            updated_at: now,
        };
        self.repo.create_design_review(&review).await?;
        Ok(review)
    }

    pub async fn conduct_design_review(&self, id: Uuid, outcome: String, action_items: Option<String>) -> anyhow::Result<DesignReview> {
        let mut review = self.repo.get_design_review(id).await?.ok_or_else(|| anyhow::anyhow!("Review not found"))?;
        review.status = "Completed".to_string();
        review.conducted_date = Some(Utc::now());
        review.outcome = Some(outcome);
        review.action_items = action_items;
        review.updated_at = Utc::now();
        self.repo.update_design_review(&review).await?;
        Ok(review)
    }

    pub async fn check_out_document(&self, id: Uuid, user_id: Uuid) -> anyhow::Result<PLMDocument> {
        let mut doc = self.repo.get_document(id).await?.ok_or_else(|| anyhow::anyhow!("Document not found"))?;
        if doc.checked_out_by.is_some() {
            return Err(anyhow::anyhow!("Document is already checked out"));
        }
        doc.checked_out_by = Some(user_id);
        doc.checked_out_at = Some(Utc::now());
        doc.updated_at = Utc::now();
        self.repo.update_document(&doc).await?;
        Ok(doc)
    }

    pub async fn check_in_document(&self, id: Uuid, user_id: Uuid) -> anyhow::Result<PLMDocument> {
        let mut doc = self.repo.get_document(id).await?.ok_or_else(|| anyhow::anyhow!("Document not found"))?;
        if doc.checked_out_by != Some(user_id) {
            return Err(anyhow::anyhow!("Document is checked out by another user"));
        }
        doc.checked_out_by = None;
        doc.checked_out_at = None;
        doc.updated_at = Utc::now();
        self.repo.update_document(&doc).await?;
        Ok(doc)
    }
}
