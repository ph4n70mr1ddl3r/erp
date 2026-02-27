use chrono::{Duration, Utc};
use erp_core::BaseEntity;
use uuid::Uuid;

use crate::models::*;
use crate::repository::WarrantyRepository;

pub struct WarrantyService<R: WarrantyRepository> {
    repo: R,
}

impl<R: WarrantyRepository> WarrantyService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_policy(&self, req: CreateWarrantyPolicyRequest) -> Result<WarrantyPolicy, String> {
        let policy = WarrantyPolicy {
            base: BaseEntity::new(),
            code: req.code,
            name: req.name,
            description: req.description,
            warranty_type: req.warranty_type,
            duration_value: req.duration_value,
            duration_unit: req.duration_unit,
            coverage_percentage: req.coverage_percentage,
            labor_covered: req.labor_covered,
            parts_covered: req.parts_covered,
            on_site_service: req.on_site_service,
            max_claims: req.max_claims,
            deductible_amount: req.deductible_amount,
            terms_and_conditions: req.terms_and_conditions,
            status: erp_core::Status::Active,
        };
        self.repo.create_policy(&policy).await.map_err(|e| e.to_string())?;
        Ok(policy)
    }

    pub async fn get_policy(&self, id: Uuid) -> Result<Option<WarrantyPolicy>, String> {
        self.repo.get_policy(id).await.map_err(|e| e.to_string())
    }

    pub async fn list_policies(&self, status: Option<String>) -> Result<Vec<WarrantyPolicy>, String> {
        self.repo.list_policies(status).await.map_err(|e| e.to_string())
    }

    pub async fn update_policy(&self, mut policy: WarrantyPolicy) -> Result<WarrantyPolicy, String> {
        policy.base.updated_at = Utc::now();
        self.repo.update_policy(&policy).await.map_err(|e| e.to_string())?;
        Ok(policy)
    }

    pub async fn delete_policy(&self, id: Uuid) -> Result<(), String> {
        self.repo.delete_policy(id).await.map_err(|e| e.to_string())
    }

    pub async fn create_product_warranty(&self, req: CreateProductWarrantyRequest) -> Result<ProductWarranty, String> {
        let policy = self.repo.get_policy(req.policy_id).await
            .map_err(|e| e.to_string())?
            .ok_or("Policy not found")?;

        let start_date = req.activation_date.unwrap_or(req.purchase_date);
        let end_date = self.calculate_end_date(start_date, policy.duration_value, &policy.duration_unit);

        let warranty_number = format!("WTY-{}", Utc::now().format("%Y%m%d%H%M%S"));

        let warranty = ProductWarranty {
            base: BaseEntity::new(),
            warranty_number,
            policy_id: req.policy_id,
            product_id: req.product_id,
            customer_id: req.customer_id,
            sales_order_id: req.sales_order_id,
            sales_order_line_id: req.sales_order_line_id,
            serial_number: req.serial_number,
            lot_number: req.lot_number,
            purchase_date: req.purchase_date,
            activation_date: req.activation_date,
            start_date,
            end_date,
            status: WarrantyStatus::Active,
            transferred_to_customer_id: None,
            transferred_at: None,
            notes: req.notes,
        };

        self.repo.create_product_warranty(&warranty).await.map_err(|e| e.to_string())?;
        Ok(warranty)
    }

    pub async fn get_product_warranty(&self, id: Uuid) -> Result<Option<ProductWarranty>, String> {
        self.repo.get_product_warranty(id).await.map_err(|e| e.to_string())
    }

    pub async fn get_product_warranty_by_number(&self, number: &str) -> Result<Option<ProductWarranty>, String> {
        self.repo.get_product_warranty_by_number(number).await.map_err(|e| e.to_string())
    }

    pub async fn list_product_warranties(&self, customer_id: Option<Uuid>, product_id: Option<Uuid>, status: Option<String>) -> Result<Vec<ProductWarranty>, String> {
        self.repo.list_product_warranties(customer_id, product_id, status).await.map_err(|e| e.to_string())
    }

    pub async fn list_expiring_warranties(&self, days: i32) -> Result<Vec<ProductWarranty>, String> {
        self.repo.list_expiring_warranties(days).await.map_err(|e| e.to_string())
    }

    pub async fn transfer_warranty(&self, warranty_id: Uuid, req: TransferWarrantyRequest) -> Result<ProductWarranty, String> {
        let mut warranty = self.repo.get_product_warranty(warranty_id).await
            .map_err(|e| e.to_string())?
            .ok_or("Warranty not found")?;

        if warranty.status != WarrantyStatus::Active {
            return Err("Only active warranties can be transferred".to_string());
        }

        warranty.transferred_to_customer_id = Some(req.new_customer_id);
        warranty.transferred_at = Some(Utc::now());
        warranty.status = WarrantyStatus::Transferred;
        warranty.notes = Some(format!("{}; Transferred: {}", warranty.notes.unwrap_or_default(), req.notes.unwrap_or_default()));

        self.repo.update_product_warranty(&warranty).await.map_err(|e| e.to_string())?;
        Ok(warranty)
    }

    pub async fn void_warranty(&self, warranty_id: Uuid, reason: String) -> Result<ProductWarranty, String> {
        let mut warranty = self.repo.get_product_warranty(warranty_id).await
            .map_err(|e| e.to_string())?
            .ok_or("Warranty not found")?;

        warranty.status = WarrantyStatus::Voided;
        warranty.notes = Some(format!("{}; Voided: {}", warranty.notes.unwrap_or_default(), reason));
        warranty.base.updated_at = Utc::now();

        self.repo.update_product_warranty(&warranty).await.map_err(|e| e.to_string())?;
        Ok(warranty)
    }

    pub async fn extend_warranty(&self, warranty_id: Uuid, req: ExtendWarrantyRequest) -> Result<WarrantyExtension, String> {
        let warranty = self.repo.get_product_warranty(warranty_id).await
            .map_err(|e| e.to_string())?
            .ok_or("Warranty not found")?;

        let new_end_date = self.calculate_end_date(warranty.end_date, req.additional_duration_value, &req.additional_duration_unit);

        let extension = WarrantyExtension {
            base: BaseEntity::new(),
            product_warranty_id: warranty_id,
            policy_id: req.policy_id,
            extension_date: Utc::now(),
            additional_duration_value: req.additional_duration_value,
            additional_duration_unit: req.additional_duration_unit,
            new_end_date,
            cost: req.cost,
            invoice_id: req.invoice_id,
            status: erp_core::Status::Active,
        };

        self.repo.create_extension(&extension).await.map_err(|e| e.to_string())?;
        Ok(extension)
    }

    pub async fn create_claim(&self, req: CreateWarrantyClaimRequest) -> Result<WarrantyClaim, String> {
        let warranty = self.repo.get_product_warranty(req.product_warranty_id).await
            .map_err(|e| e.to_string())?
            .ok_or("Warranty not found")?;

        if warranty.status != WarrantyStatus::Active {
            return Err("Warranty is not active".to_string());
        }

        if warranty.end_date < Utc::now() {
            return Err("Warranty has expired".to_string());
        }

        let claim_number = format!("CLM-{}", Utc::now().format("%Y%m%d%H%M%S"));

        let claim = WarrantyClaim {
            base: BaseEntity::new(),
            claim_number,
            product_warranty_id: req.product_warranty_id,
            customer_id: req.customer_id,
            reported_date: req.reported_date,
            issue_description: req.issue_description,
            issue_category: req.issue_category,
            symptom_codes: req.symptom_codes,
            status: WarrantyClaimStatus::Submitted,
            priority: req.priority,
            assigned_to: None,
            assigned_at: None,
            resolution_type: None,
            resolution_notes: None,
            resolved_at: None,
            resolved_by: None,
            customer_notified: false,
            notification_date: None,
        };

        self.repo.create_claim(&claim).await.map_err(|e| e.to_string())?;
        Ok(claim)
    }

    pub async fn get_claim(&self, id: Uuid) -> Result<Option<WarrantyClaim>, String> {
        self.repo.get_claim(id).await.map_err(|e| e.to_string())
    }

    pub async fn get_claim_by_number(&self, number: &str) -> Result<Option<WarrantyClaim>, String> {
        self.repo.get_claim_by_number(number).await.map_err(|e| e.to_string())
    }

    pub async fn list_claims(&self, customer_id: Option<Uuid>, warranty_id: Option<Uuid>, status: Option<WarrantyClaimStatus>) -> Result<Vec<WarrantyClaim>, String> {
        self.repo.list_claims(customer_id, warranty_id, status).await.map_err(|e| e.to_string())
    }

    pub async fn assign_claim(&self, claim_id: Uuid, assigned_to: Uuid) -> Result<WarrantyClaim, String> {
        let mut claim = self.repo.get_claim(claim_id).await
            .map_err(|e| e.to_string())?
            .ok_or("Claim not found")?;

        claim.assigned_to = Some(assigned_to);
        claim.assigned_at = Some(Utc::now());
        claim.status = WarrantyClaimStatus::UnderReview;
        claim.base.updated_at = Utc::now();

        self.repo.update_claim(&claim).await.map_err(|e| e.to_string())?;
        Ok(claim)
    }

    pub async fn approve_claim(&self, claim_id: Uuid) -> Result<WarrantyClaim, String> {
        let mut claim = self.repo.get_claim(claim_id).await
            .map_err(|e| e.to_string())?
            .ok_or("Claim not found")?;

        claim.status = WarrantyClaimStatus::Approved;
        claim.base.updated_at = Utc::now();

        self.repo.update_claim(&claim).await.map_err(|e| e.to_string())?;
        Ok(claim)
    }

    pub async fn reject_claim(&self, claim_id: Uuid, reason: String) -> Result<WarrantyClaim, String> {
        let mut claim = self.repo.get_claim(claim_id).await
            .map_err(|e| e.to_string())?
            .ok_or("Claim not found")?;

        claim.status = WarrantyClaimStatus::Rejected;
        claim.resolution_type = Some(ClaimResolutionType::Denied);
        claim.resolution_notes = Some(reason);
        claim.resolved_at = Some(Utc::now());
        claim.base.updated_at = Utc::now();

        self.repo.update_claim(&claim).await.map_err(|e| e.to_string())?;
        Ok(claim)
    }

    pub async fn start_claim_work(&self, claim_id: Uuid) -> Result<WarrantyClaim, String> {
        let mut claim = self.repo.get_claim(claim_id).await
            .map_err(|e| e.to_string())?
            .ok_or("Claim not found")?;

        claim.status = WarrantyClaimStatus::InProgress;
        claim.base.updated_at = Utc::now();

        self.repo.update_claim(&claim).await.map_err(|e| e.to_string())?;
        Ok(claim)
    }

    pub async fn resolve_claim(&self, claim_id: Uuid, req: ResolveClaimRequest, resolved_by: Uuid) -> Result<WarrantyClaim, String> {
        let mut claim = self.repo.get_claim(claim_id).await
            .map_err(|e| e.to_string())?
            .ok_or("Claim not found")?;

        claim.status = WarrantyClaimStatus::Completed;
        claim.resolution_type = Some(req.resolution_type);
        claim.resolution_notes = req.resolution_notes;
        claim.resolved_at = Some(Utc::now());
        claim.resolved_by = Some(resolved_by);
        claim.base.updated_at = Utc::now();

        self.repo.update_claim(&claim).await.map_err(|e| e.to_string())?;
        Ok(claim)
    }

    pub async fn add_claim_line(&self, req: AddClaimLineRequest) -> Result<WarrantyClaimLine, String> {
        let claim = self.repo.get_claim(req.claim_id).await
            .map_err(|e| e.to_string())?
            .ok_or("Claim not found")?;

        let warranty = self.repo.get_product_warranty(claim.product_warranty_id).await
            .map_err(|e| e.to_string())?
            .ok_or("Warranty not found")?;

        let policy = self.repo.get_policy(warranty.policy_id).await
            .map_err(|e| e.to_string())?
            .ok_or("Policy not found")?;

        let total_cost = req.quantity * req.unit_cost;
        let coverage_pct = if policy.parts_covered { req.coverage_percentage } else { 0.0 };
        let covered_amount = ((total_cost as f64) * coverage_pct / 100.0) as i64;
        let customer_amount = total_cost - covered_amount;

        let line = WarrantyClaimLine {
            id: Uuid::new_v4(),
            claim_id: req.claim_id,
            product_id: req.product_id,
            description: req.description,
            quantity: req.quantity,
            unit_cost: req.unit_cost,
            total_cost,
            coverage_percentage: coverage_pct,
            covered_amount,
            customer_amount,
            created_at: Utc::now(),
        };

        self.repo.create_claim_line(&line).await.map_err(|e| e.to_string())?;
        Ok(line)
    }

    pub async fn list_claim_lines(&self, claim_id: Uuid) -> Result<Vec<WarrantyClaimLine>, String> {
        self.repo.list_claim_lines(claim_id).await.map_err(|e| e.to_string())
    }

    pub async fn add_claim_labor(&self, req: AddClaimLaborRequest) -> Result<WarrantyClaimLabor, String> {
        let claim = self.repo.get_claim(req.claim_id).await
            .map_err(|e| e.to_string())?
            .ok_or("Claim not found")?;

        let warranty = self.repo.get_product_warranty(claim.product_warranty_id).await
            .map_err(|e| e.to_string())?
            .ok_or("Warranty not found")?;

        let policy = self.repo.get_policy(warranty.policy_id).await
            .map_err(|e| e.to_string())?
            .ok_or("Policy not found")?;

        let total_cost = ((req.labor_hours * req.hourly_rate as f64) as i64);
        let covered_amount = if policy.labor_covered { total_cost } else { 0 };
        let customer_amount = total_cost - covered_amount;

        let labor = WarrantyClaimLabor {
            id: Uuid::new_v4(),
            claim_id: req.claim_id,
            technician_id: req.technician_id,
            work_description: req.work_description,
            labor_hours: req.labor_hours,
            hourly_rate: req.hourly_rate,
            total_cost,
            covered_amount,
            customer_amount,
            work_date: req.work_date,
            created_at: Utc::now(),
        };

        self.repo.create_claim_labor(&labor).await.map_err(|e| e.to_string())?;
        Ok(labor)
    }

    pub async fn list_claim_labors(&self, claim_id: Uuid) -> Result<Vec<WarrantyClaimLabor>, String> {
        self.repo.list_claim_labors(claim_id).await.map_err(|e| e.to_string())
    }

    pub async fn get_analytics(&self) -> Result<WarrantyAnalytics, String> {
        self.repo.get_analytics().await.map_err(|e| e.to_string())
    }

    fn calculate_end_date(&self, start: chrono::DateTime<Utc>, duration_value: i32, unit: &WarrantyDurationUnit) -> chrono::DateTime<Utc> {
        match unit {
            WarrantyDurationUnit::Days => start + Duration::days(duration_value as i64),
            WarrantyDurationUnit::Months => {
                let months = duration_value as i32;
                start + Duration::days((months * 30) as i64)
            }
            WarrantyDurationUnit::Years => {
                let years = duration_value as i32;
                start + Duration::days((years * 365) as i64)
            }
        }
    }
}
