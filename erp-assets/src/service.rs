use anyhow::{Result, anyhow};
use chrono::Utc;
use erp_core::BaseEntity;
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::*;
use crate::repository::*;

pub struct ITAssetService {
    repo: SqliteITAssetRepository,
    assignment_repo: SqliteAssetAssignmentRepository,
}

impl Default for ITAssetService {
    fn default() -> Self {
        Self::new()
    }
}

impl ITAssetService {
    pub fn new() -> Self {
        Self { repo: SqliteITAssetRepository, assignment_repo: SqliteAssetAssignmentRepository }
    }

    pub async fn create(&self, pool: &SqlitePool, asset_tag: String, name: String, asset_type: ITAssetType, purchase_cost: i64, currency: String, description: Option<String>, model: Option<String>, manufacturer: Option<String>, serial_number: Option<String>, purchase_date: Option<chrono::NaiveDate>, warranty_expiry: Option<chrono::NaiveDate>, location_id: Option<Uuid>) -> Result<ITAsset> {
        let existing = self.repo.find_by_tag(pool, &asset_tag).await?;
        if existing.is_some() {
            return Err(anyhow!("Asset tag already exists"));
        }
        let asset = ITAsset {
            base: BaseEntity::new(),
            asset_tag,
            name,
            description,
            asset_type,
            status: ITAssetStatus::Available,
            model,
            manufacturer,
            serial_number,
            purchase_date,
            purchase_cost,
            currency,
            warranty_expiry,
            location_id,
            assigned_to: None,
            assigned_date: None,
            department_id: None,
            notes: None,
        };
        self.repo.create(pool, &asset).await?;
        Ok(asset)
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<ITAsset> {
        self.repo.find_by_id(pool, id).await?.ok_or_else(|| anyhow!("Asset not found"))
    }

    pub async fn get_by_tag(&self, pool: &SqlitePool, tag: &str) -> Result<ITAsset> {
        self.repo.find_by_tag(pool, tag).await?.ok_or_else(|| anyhow!("Asset not found"))
    }

    pub async fn list(&self, pool: &SqlitePool, page: i64, page_size: i64) -> Result<Vec<ITAsset>> {
        let offset = (page - 1) * page_size;
        self.repo.find_all(pool, page_size, offset).await
    }

    pub async fn assign(&self, pool: &SqlitePool, asset_id: Uuid, user_id: Uuid, assigned_by: Uuid, expected_return: Option<chrono::NaiveDate>) -> Result<ITAsset> {
        let mut asset = self.get(pool, asset_id).await?;
        if asset.status != ITAssetStatus::Available {
            return Err(anyhow!("Asset is not available for assignment"));
        }
        let now = Utc::now();
        let assignment = AssetAssignment {
            id: Uuid::new_v4(),
            asset_id,
            assigned_to: user_id,
            assigned_by,
            assigned_at: now,
            expected_return,
            returned_at: None,
            returned_by: None,
            notes: None,
            status: AssignmentStatus::Active,
        };
        self.assignment_repo.create(pool, &assignment).await?;
        asset.assigned_to = Some(user_id);
        asset.assigned_date = Some(now.date_naive());
        asset.status = ITAssetStatus::InUse;
        asset.base.updated_at = now;
        self.repo.update(pool, &asset).await?;
        Ok(asset)
    }

    pub async fn unassign(&self, pool: &SqlitePool, asset_id: Uuid, returned_by: Uuid) -> Result<ITAsset> {
        let mut asset = self.get(pool, asset_id).await?;
        if asset.assigned_to.is_none() {
            return Err(anyhow!("Asset is not currently assigned"));
        }
        let mut assignments = self.assignment_repo.find_by_asset(pool, asset_id).await?;
        if let Some(assignment) = assignments.iter_mut().find(|a| a.status == AssignmentStatus::Active) {
            assignment.returned_at = Some(Utc::now());
            assignment.returned_by = Some(returned_by);
            assignment.status = AssignmentStatus::Returned;
            self.assignment_repo.update(pool, assignment).await?;
        }
        asset.assigned_to = None;
        asset.assigned_date = None;
        asset.status = ITAssetStatus::Available;
        asset.base.updated_at = Utc::now();
        self.repo.update(pool, &asset).await?;
        Ok(asset)
    }

    pub async fn update_status(&self, pool: &SqlitePool, asset_id: Uuid, status: ITAssetStatus) -> Result<ITAsset> {
        let mut asset = self.get(pool, asset_id).await?;
        asset.status = status;
        asset.base.updated_at = Utc::now();
        self.repo.update(pool, &asset).await?;
        Ok(asset)
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete(pool, id).await
    }

    pub async fn get_by_assignee(&self, pool: &SqlitePool, assignee_id: Uuid) -> Result<Vec<ITAsset>> {
        self.repo.find_by_assignee(pool, assignee_id).await
    }

    pub async fn get_by_status(&self, pool: &SqlitePool, status: ITAssetStatus) -> Result<Vec<ITAsset>> {
        self.repo.find_by_status(pool, status).await
    }

    pub async fn get_stats(&self, pool: &SqlitePool) -> Result<Vec<(String, i64)>> {
        self.repo.count_by_status(pool).await
    }
}

pub struct SoftwareLicenseService {
    repo: SqliteSoftwareLicenseRepository,
}

impl Default for SoftwareLicenseService {
    fn default() -> Self {
        Self::new()
    }
}

impl SoftwareLicenseService {
    pub fn new() -> Self {
        Self { repo: SqliteSoftwareLicenseRepository }
    }

    pub async fn create(&self, pool: &SqlitePool, license_key: String, product_name: String, vendor: String, license_type: LicenseType, seats_purchased: i32, purchase_cost: i64, currency: String, purchase_date: chrono::NaiveDate, start_date: chrono::NaiveDate, expiry_date: Option<chrono::NaiveDate>) -> Result<SoftwareLicense> {
        let now = Utc::now();
        let license = SoftwareLicense {
            id: Uuid::new_v4(),
            license_key,
            product_name,
            vendor,
            license_type,
            seats_purchased,
            seats_used: 0,
            purchase_date,
            purchase_cost,
            currency,
            start_date,
            expiry_date,
            auto_renew: false,
            support_expiry: None,
            status: erp_core::Status::Active,
            notes: None,
            created_at: now,
            updated_at: now,
        };
        self.repo.create(pool, &license).await?;
        Ok(license)
    }

    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<SoftwareLicense> {
        self.repo.find_by_id(pool, id).await?.ok_or_else(|| anyhow!("License not found"))
    }

    pub async fn list(&self, pool: &SqlitePool, page: i64, page_size: i64) -> Result<Vec<SoftwareLicense>> {
        let offset = (page - 1) * page_size;
        self.repo.find_all(pool, page_size, offset).await
    }

    pub async fn increment_usage(&self, pool: &SqlitePool, id: Uuid) -> Result<SoftwareLicense> {
        let mut license = self.get(pool, id).await?;
        if license.seats_used >= license.seats_purchased {
            return Err(anyhow!("No seats available"));
        }
        license.seats_used += 1;
        license.updated_at = Utc::now();
        self.repo.update(pool, &license).await?;
        Ok(license)
    }

    pub async fn decrement_usage(&self, pool: &SqlitePool, id: Uuid) -> Result<SoftwareLicense> {
        let mut license = self.get(pool, id).await?;
        if license.seats_used > 0 {
            license.seats_used -= 1;
            license.updated_at = Utc::now();
            self.repo.update(pool, &license).await?;
        }
        Ok(license)
    }

    pub async fn get_expiring(&self, pool: &SqlitePool, days: i32) -> Result<Vec<SoftwareLicense>> {
        self.repo.find_expiring(pool, days).await
    }

    pub async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.delete(pool, id).await
    }
}
