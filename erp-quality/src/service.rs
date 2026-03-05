use crate::models::*;
use crate::repository::{QualityRepository, SqliteQualityRepository};
use anyhow::Result;
use chrono::Utc;
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use tracing::info;
use uuid::Uuid;

pub struct QualityService<R: QualityRepository> {
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
            anyhow::bail!("At least one inspection item is required");
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
            .ok_or_else(|| anyhow::anyhow!("Inspection not found"))?;

        if inspection.status != InspectionStatus::Pending {
            anyhow::bail!("Only pending inspections can be started");
        }

        inspection.status = InspectionStatus::InProgress;
        inspection.base.updated_at = Utc::now();
        self.repo.update_inspection(&inspection).await
    }

    pub async fn complete_inspection(&self, id: Uuid) -> Result<QualityInspection> {
        let mut inspection = self.repo.get_inspection(id).await?
            .ok_or_else(|| anyhow::anyhow!("Inspection not found"))?;

        if inspection.status != InspectionStatus::InProgress && inspection.status != InspectionStatus::Pending {
            anyhow::bail!("Only pending or in-progress inspections can be completed");
        }

        let items = self.repo.get_inspection_items(id).await?;
        if items.iter().any(|i| i.pass_fail.is_none()) {
            anyhow::bail!("All inspection items must be evaluated before completing");
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
            .ok_or_else(|| anyhow::anyhow!("Inspection not found"))?;

        if inspection.status == InspectionStatus::Passed || inspection.status == InspectionStatus::Failed {
            anyhow::bail!("Completed inspections cannot be cancelled");
        }

        inspection.status = InspectionStatus::Cancelled;
        inspection.base.updated_at = Utc::now();
        self.repo.update_inspection(&inspection).await
    }

    pub async fn delete_inspection(&self, id: Uuid) -> Result<()> {
        let inspection = self.repo.get_inspection(id).await?
            .ok_or_else(|| anyhow::anyhow!("Inspection not found"))?;

        if inspection.status != InspectionStatus::Pending && inspection.status != InspectionStatus::Cancelled {
            anyhow::bail!("Only pending or cancelled inspections can be deleted");
        }

        self.repo.delete_inspection(id).await?;
        info!("Deleted quality inspection {}", inspection.inspection_number);
        Ok(())
    }

    pub async fn update_inspection_item(&self, inspection_id: Uuid, item_id: Uuid, req: UpdateInspectionItemRequest) -> Result<InspectionItem> {
        let inspection = self.repo.get_inspection(inspection_id).await?
            .ok_or_else(|| anyhow::anyhow!("Inspection not found"))?;

        if inspection.status != InspectionStatus::Pending && inspection.status != InspectionStatus::InProgress {
            anyhow::bail!("Can only update items on pending or in-progress inspections");
        }

        let mut items = self.repo.get_inspection_items(inspection_id).await?;
        let item = items.iter_mut().find(|i| i.id == item_id)
            .ok_or_else(|| anyhow::anyhow!("Item not found"))?;

        item.actual_value = req.actual_value;
        item.pass_fail = req.pass_fail;
        item.notes = req.notes;

        self.repo.update_inspection_item(item).await
    }

    pub async fn add_inspection_item(&self, inspection_id: Uuid, req: CreateInspectionItemRequest) -> Result<InspectionItem> {
        let inspection = self.repo.get_inspection(inspection_id).await?
            .ok_or_else(|| anyhow::anyhow!("Inspection not found"))?;

        if inspection.status != InspectionStatus::Pending && inspection.status != InspectionStatus::InProgress {
            anyhow::bail!("Can only add items to pending or in-progress inspections");
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
            .ok_or_else(|| anyhow::anyhow!("NCR not found"))?;

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
            .ok_or_else(|| anyhow::anyhow!("NCR not found"))?;

        if ncr.status == NCRStatus::Closed || ncr.status == NCRStatus::Cancelled {
            anyhow::bail!("NCR is already closed or cancelled");
        }

        if ncr.corrective_action.is_none() {
            anyhow::bail!("Corrective action must be specified before closing");
        }

        ncr.status = NCRStatus::Closed;
        ncr.resolution_date = Some(chrono::Utc::now().date_naive());
        ncr.base.updated_at = Utc::now();

        info!("Closed NCR {}", ncr.ncr_number);
        self.repo.update_ncr(&ncr).await
    }

    pub async fn delete_ncr(&self, id: Uuid) -> Result<()> {
        let ncr = self.repo.get_ncr(id).await?
            .ok_or_else(|| anyhow::anyhow!("NCR not found"))?;

        if ncr.status != NCRStatus::Open {
            anyhow::bail!("Only open NCRs can be deleted");
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
}
