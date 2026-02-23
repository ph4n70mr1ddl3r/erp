use async_trait::async_trait;
use anyhow::Result;
use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::{DateTime, Utc, NaiveDate};
use crate::models::*;

#[derive(sqlx::FromRow)]
struct AssetRow {
    id: String,
    asset_tag: String,
    name: String,
    description: Option<String>,
    asset_type: String,
    status: String,
    model: Option<String>,
    manufacturer: Option<String>,
    serial_number: Option<String>,
    purchase_date: Option<String>,
    purchase_cost: i64,
    currency: String,
    warranty_expiry: Option<String>,
    location_id: Option<String>,
    assigned_to: Option<String>,
    assigned_date: Option<String>,
    department_id: Option<String>,
    notes: Option<String>,
    created_at: String,
    updated_at: String,
}

impl AssetRow {
    fn into_asset(self) -> ITAsset {
        ITAsset {
            base: erp_core::BaseEntity {
                id: Uuid::parse_str(&self.id).unwrap_or_default(),
                created_at: DateTime::parse_from_rfc3339(&self.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                updated_at: DateTime::parse_from_rfc3339(&self.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
                created_by: None,
                updated_by: None,
            },
            asset_tag: self.asset_tag,
            name: self.name,
            description: self.description,
            asset_type: serde_json::from_str(&self.asset_type).unwrap_or(ITAssetType::Hardware),
            status: serde_json::from_str(&self.status).unwrap_or(ITAssetStatus::Available),
            model: self.model,
            manufacturer: self.manufacturer,
            serial_number: self.serial_number,
            purchase_date: self.purchase_date.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            purchase_cost: self.purchase_cost,
            currency: self.currency,
            warranty_expiry: self.warranty_expiry.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            location_id: self.location_id.and_then(|s| Uuid::parse_str(&s).ok()),
            assigned_to: self.assigned_to.and_then(|s| Uuid::parse_str(&s).ok()),
            assigned_date: self.assigned_date.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            department_id: self.department_id.and_then(|s| Uuid::parse_str(&s).ok()),
            notes: self.notes,
        }
    }
}

#[async_trait]
pub trait ITAssetRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, asset: &ITAsset) -> Result<()>;
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<ITAsset>>;
    async fn find_all(&self, pool: &SqlitePool, limit: i64, offset: i64) -> Result<Vec<ITAsset>>;
    async fn find_by_tag(&self, pool: &SqlitePool, tag: &str) -> Result<Option<ITAsset>>;
    async fn find_by_assignee(&self, pool: &SqlitePool, assignee_id: Uuid) -> Result<Vec<ITAsset>>;
    async fn find_by_status(&self, pool: &SqlitePool, status: ITAssetStatus) -> Result<Vec<ITAsset>>;
    async fn update(&self, pool: &SqlitePool, asset: &ITAsset) -> Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn count_by_status(&self, pool: &SqlitePool) -> Result<Vec<(String, i64)>>;
}

pub struct SqliteITAssetRepository;

#[async_trait]
impl ITAssetRepository for SqliteITAssetRepository {
    async fn create(&self, pool: &SqlitePool, asset: &ITAsset) -> Result<()> {
        sqlx::query(
            "INSERT INTO it_assets (id, asset_tag, name, description, asset_type, status, model, manufacturer, serial_number, purchase_date, purchase_cost, currency, warranty_expiry, location_id, assigned_to, assigned_date, department_id, notes, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(asset.base.id.to_string())
            .bind(&asset.asset_tag)
            .bind(&asset.name)
            .bind(&asset.description)
            .bind(serde_json::to_string(&asset.asset_type)?)
            .bind(serde_json::to_string(&asset.status)?)
            .bind(&asset.model)
            .bind(&asset.manufacturer)
            .bind(&asset.serial_number)
            .bind(asset.purchase_date.map(|d| d.to_string()))
            .bind(asset.purchase_cost)
            .bind(&asset.currency)
            .bind(asset.warranty_expiry.map(|d| d.to_string()))
            .bind(asset.location_id.map(|id| id.to_string()))
            .bind(asset.assigned_to.map(|id| id.to_string()))
            .bind(asset.assigned_date.map(|d| d.to_string()))
            .bind(asset.department_id.map(|id| id.to_string()))
            .bind(&asset.notes)
            .bind(asset.base.created_at.to_rfc3339())
            .bind(asset.base.updated_at.to_rfc3339())
            .execute(pool).await?;
        Ok(())
    }

    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<ITAsset>> {
        let row = sqlx::query_as::<_, AssetRow>(
            "SELECT id, asset_tag, name, description, asset_type, status, model, manufacturer, serial_number, purchase_date, purchase_cost, currency, warranty_expiry, location_id, assigned_to, assigned_date, department_id, notes, created_at, updated_at FROM it_assets WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(pool).await?;
        Ok(row.map(|r| r.into_asset()))
    }

    async fn find_all(&self, pool: &SqlitePool, limit: i64, offset: i64) -> Result<Vec<ITAsset>> {
        let rows = sqlx::query_as::<_, AssetRow>(
            "SELECT id, asset_tag, name, description, asset_type, status, model, manufacturer, serial_number, purchase_date, purchase_cost, currency, warranty_expiry, location_id, assigned_to, assigned_date, department_id, notes, created_at, updated_at FROM it_assets ORDER BY created_at DESC LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_asset()).collect())
    }

    async fn find_by_tag(&self, pool: &SqlitePool, tag: &str) -> Result<Option<ITAsset>> {
        let row = sqlx::query_as::<_, AssetRow>(
            "SELECT id, asset_tag, name, description, asset_type, status, model, manufacturer, serial_number, purchase_date, purchase_cost, currency, warranty_expiry, location_id, assigned_to, assigned_date, department_id, notes, created_at, updated_at FROM it_assets WHERE asset_tag = ?")
            .bind(tag)
            .fetch_optional(pool).await?;
        Ok(row.map(|r| r.into_asset()))
    }

    async fn find_by_assignee(&self, pool: &SqlitePool, assignee_id: Uuid) -> Result<Vec<ITAsset>> {
        let rows = sqlx::query_as::<_, AssetRow>(
            "SELECT id, asset_tag, name, description, asset_type, status, model, manufacturer, serial_number, purchase_date, purchase_cost, currency, warranty_expiry, location_id, assigned_to, assigned_date, department_id, notes, created_at, updated_at FROM it_assets WHERE assigned_to = ? ORDER BY name")
            .bind(assignee_id.to_string())
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_asset()).collect())
    }

    async fn find_by_status(&self, pool: &SqlitePool, status: ITAssetStatus) -> Result<Vec<ITAsset>> {
        let status_str = serde_json::to_string(&status)?;
        let rows = sqlx::query_as::<_, AssetRow>(
            "SELECT id, asset_tag, name, description, asset_type, status, model, manufacturer, serial_number, purchase_date, purchase_cost, currency, warranty_expiry, location_id, assigned_to, assigned_date, department_id, notes, created_at, updated_at FROM it_assets WHERE status = ? ORDER BY name")
            .bind(status_str)
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_asset()).collect())
    }

    async fn update(&self, pool: &SqlitePool, asset: &ITAsset) -> Result<()> {
        sqlx::query(
            "UPDATE it_assets SET asset_tag = ?, name = ?, description = ?, asset_type = ?, status = ?, model = ?, manufacturer = ?, serial_number = ?, purchase_date = ?, purchase_cost = ?, currency = ?, warranty_expiry = ?, location_id = ?, assigned_to = ?, assigned_date = ?, department_id = ?, notes = ?, updated_at = ? WHERE id = ?")
            .bind(&asset.asset_tag)
            .bind(&asset.name)
            .bind(&asset.description)
            .bind(serde_json::to_string(&asset.asset_type)?)
            .bind(serde_json::to_string(&asset.status)?)
            .bind(&asset.model)
            .bind(&asset.manufacturer)
            .bind(&asset.serial_number)
            .bind(asset.purchase_date.map(|d| d.to_string()))
            .bind(asset.purchase_cost)
            .bind(&asset.currency)
            .bind(asset.warranty_expiry.map(|d| d.to_string()))
            .bind(asset.location_id.map(|id| id.to_string()))
            .bind(asset.assigned_to.map(|id| id.to_string()))
            .bind(asset.assigned_date.map(|d| d.to_string()))
            .bind(asset.department_id.map(|id| id.to_string()))
            .bind(&asset.notes)
            .bind(asset.base.updated_at.to_rfc3339())
            .bind(asset.base.id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM it_assets WHERE id = ?")
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn count_by_status(&self, pool: &SqlitePool) -> Result<Vec<(String, i64)>> {
        #[derive(sqlx::FromRow)]
        struct StatusCount { status: String, count: i64 }
        let rows = sqlx::query_as::<_, StatusCount>(
            "SELECT status, COUNT(*) as count FROM it_assets GROUP BY status")
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| (r.status, r.count)).collect())
    }
}

#[derive(sqlx::FromRow)]
struct LicenseRow {
    id: String,
    license_key: String,
    product_name: String,
    vendor: String,
    license_type: String,
    seats_purchased: i32,
    seats_used: i32,
    purchase_date: String,
    purchase_cost: i64,
    currency: String,
    start_date: String,
    expiry_date: Option<String>,
    auto_renew: i32,
    support_expiry: Option<String>,
    status: String,
    notes: Option<String>,
    created_at: String,
    updated_at: String,
}

impl LicenseRow {
    fn into_license(self) -> SoftwareLicense {
        SoftwareLicense {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            license_key: self.license_key,
            product_name: self.product_name,
            vendor: self.vendor,
            license_type: serde_json::from_str(&self.license_type).unwrap_or(LicenseType::Perpetual),
            seats_purchased: self.seats_purchased,
            seats_used: self.seats_used,
            purchase_date: NaiveDate::parse_from_str(&self.purchase_date, "%Y-%m-%d").unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
            purchase_cost: self.purchase_cost,
            currency: self.currency,
            start_date: NaiveDate::parse_from_str(&self.start_date, "%Y-%m-%d").unwrap_or_else(|_| NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()),
            expiry_date: self.expiry_date.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            auto_renew: self.auto_renew != 0,
            support_expiry: self.support_expiry.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            status: serde_json::from_str(&self.status).unwrap_or(erp_core::Status::Active),
            notes: self.notes,
            created_at: DateTime::parse_from_rfc3339(&self.created_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            updated_at: DateTime::parse_from_rfc3339(&self.updated_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[async_trait]
pub trait SoftwareLicenseRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, license: &SoftwareLicense) -> Result<()>;
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<SoftwareLicense>>;
    async fn find_all(&self, pool: &SqlitePool, limit: i64, offset: i64) -> Result<Vec<SoftwareLicense>>;
    async fn update(&self, pool: &SqlitePool, license: &SoftwareLicense) -> Result<()>;
    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
    async fn find_expiring(&self, pool: &SqlitePool, days: i32) -> Result<Vec<SoftwareLicense>>;
}

pub struct SqliteSoftwareLicenseRepository;

#[async_trait]
impl SoftwareLicenseRepository for SqliteSoftwareLicenseRepository {
    async fn create(&self, pool: &SqlitePool, license: &SoftwareLicense) -> Result<()> {
        sqlx::query(
            "INSERT INTO software_licenses (id, license_key, product_name, vendor, license_type, seats_purchased, seats_used, purchase_date, purchase_cost, currency, start_date, expiry_date, auto_renew, support_expiry, status, notes, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(license.id.to_string())
            .bind(&license.license_key)
            .bind(&license.product_name)
            .bind(&license.vendor)
            .bind(serde_json::to_string(&license.license_type)?)
            .bind(license.seats_purchased)
            .bind(license.seats_used)
            .bind(license.purchase_date.to_string())
            .bind(license.purchase_cost)
            .bind(&license.currency)
            .bind(license.start_date.to_string())
            .bind(license.expiry_date.map(|d| d.to_string()))
            .bind(if license.auto_renew { 1 } else { 0 })
            .bind(license.support_expiry.map(|d| d.to_string()))
            .bind(serde_json::to_string(&license.status)?)
            .bind(&license.notes)
            .bind(license.created_at.to_rfc3339())
            .bind(license.updated_at.to_rfc3339())
            .execute(pool).await?;
        Ok(())
    }

    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Option<SoftwareLicense>> {
        let row = sqlx::query_as::<_, LicenseRow>(
            "SELECT id, license_key, product_name, vendor, license_type, seats_purchased, seats_used, purchase_date, purchase_cost, currency, start_date, expiry_date, auto_renew, support_expiry, status, notes, created_at, updated_at FROM software_licenses WHERE id = ?")
            .bind(id.to_string())
            .fetch_optional(pool).await?;
        Ok(row.map(|r| r.into_license()))
    }

    async fn find_all(&self, pool: &SqlitePool, limit: i64, offset: i64) -> Result<Vec<SoftwareLicense>> {
        let rows = sqlx::query_as::<_, LicenseRow>(
            "SELECT id, license_key, product_name, vendor, license_type, seats_purchased, seats_used, purchase_date, purchase_cost, currency, start_date, expiry_date, auto_renew, support_expiry, status, notes, created_at, updated_at FROM software_licenses ORDER BY product_name LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_license()).collect())
    }

    async fn update(&self, pool: &SqlitePool, license: &SoftwareLicense) -> Result<()> {
        sqlx::query(
            "UPDATE software_licenses SET license_key = ?, product_name = ?, vendor = ?, license_type = ?, seats_purchased = ?, seats_used = ?, purchase_date = ?, purchase_cost = ?, currency = ?, start_date = ?, expiry_date = ?, auto_renew = ?, support_expiry = ?, status = ?, notes = ?, updated_at = ? WHERE id = ?")
            .bind(&license.license_key)
            .bind(&license.product_name)
            .bind(&license.vendor)
            .bind(serde_json::to_string(&license.license_type)?)
            .bind(license.seats_purchased)
            .bind(license.seats_used)
            .bind(license.purchase_date.to_string())
            .bind(license.purchase_cost)
            .bind(&license.currency)
            .bind(license.start_date.to_string())
            .bind(license.expiry_date.map(|d| d.to_string()))
            .bind(if license.auto_renew { 1 } else { 0 })
            .bind(license.support_expiry.map(|d| d.to_string()))
            .bind(serde_json::to_string(&license.status)?)
            .bind(&license.notes)
            .bind(license.updated_at.to_rfc3339())
            .bind(license.id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn delete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        sqlx::query("DELETE FROM software_licenses WHERE id = ?")
            .bind(id.to_string())
            .execute(pool).await?;
        Ok(())
    }

    async fn find_expiring(&self, pool: &SqlitePool, days: i32) -> Result<Vec<SoftwareLicense>> {
        let rows = sqlx::query_as::<_, LicenseRow>(
            "SELECT id, license_key, product_name, vendor, license_type, seats_purchased, seats_used, purchase_date, purchase_cost, currency, start_date, expiry_date, auto_renew, support_expiry, status, notes, created_at, updated_at FROM software_licenses WHERE expiry_date IS NOT NULL AND date(expiry_date) <= date('now', '+' || ? || ' days') AND status = '\"Active\"' ORDER BY expiry_date")
            .bind(days.to_string())
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_license()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct AssignmentRow {
    id: String,
    asset_id: String,
    assigned_to: String,
    assigned_by: String,
    assigned_at: String,
    expected_return: Option<String>,
    returned_at: Option<String>,
    returned_by: Option<String>,
    notes: Option<String>,
    status: String,
}

impl AssignmentRow {
    fn into_assignment(self) -> AssetAssignment {
        AssetAssignment {
            id: Uuid::parse_str(&self.id).unwrap_or_default(),
            asset_id: Uuid::parse_str(&self.asset_id).unwrap_or_default(),
            assigned_to: Uuid::parse_str(&self.assigned_to).unwrap_or_default(),
            assigned_by: Uuid::parse_str(&self.assigned_by).unwrap_or_default(),
            assigned_at: DateTime::parse_from_rfc3339(&self.assigned_at).map(|d| d.with_timezone(&Utc)).unwrap_or_else(|_| Utc::now()),
            expected_return: self.expected_return.and_then(|s| NaiveDate::parse_from_str(&s, "%Y-%m-%d").ok()),
            returned_at: self.returned_at.and_then(|s| DateTime::parse_from_rfc3339(&s).ok().map(|d| d.with_timezone(&Utc))),
            returned_by: self.returned_by.and_then(|s| Uuid::parse_str(&s).ok()),
            notes: self.notes,
            status: serde_json::from_str(&self.status).unwrap_or(AssignmentStatus::Active),
        }
    }
}

#[async_trait]
pub trait AssetAssignmentRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, assignment: &AssetAssignment) -> Result<()>;
    async fn find_by_asset(&self, pool: &SqlitePool, asset_id: Uuid) -> Result<Vec<AssetAssignment>>;
    async fn find_active_by_user(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Vec<AssetAssignment>>;
    async fn update(&self, pool: &SqlitePool, assignment: &AssetAssignment) -> Result<()>;
}

pub struct SqliteAssetAssignmentRepository;

#[async_trait]
impl AssetAssignmentRepository for SqliteAssetAssignmentRepository {
    async fn create(&self, pool: &SqlitePool, assignment: &AssetAssignment) -> Result<()> {
        sqlx::query(
            "INSERT INTO asset_assignments (id, asset_id, assigned_to, assigned_by, assigned_at, expected_return, returned_at, returned_by, notes, status) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)")
            .bind(assignment.id.to_string())
            .bind(assignment.asset_id.to_string())
            .bind(assignment.assigned_to.to_string())
            .bind(assignment.assigned_by.to_string())
            .bind(assignment.assigned_at.to_rfc3339())
            .bind(assignment.expected_return.map(|d| d.to_string()))
            .bind(assignment.returned_at.map(|d| d.to_rfc3339()))
            .bind(assignment.returned_by.map(|id| id.to_string()))
            .bind(&assignment.notes)
            .bind(serde_json::to_string(&assignment.status)?)
            .execute(pool).await?;
        Ok(())
    }

    async fn find_by_asset(&self, pool: &SqlitePool, asset_id: Uuid) -> Result<Vec<AssetAssignment>> {
        let rows = sqlx::query_as::<_, AssignmentRow>(
            "SELECT id, asset_id, assigned_to, assigned_by, assigned_at, expected_return, returned_at, returned_by, notes, status FROM asset_assignments WHERE asset_id = ? ORDER BY assigned_at DESC")
            .bind(asset_id.to_string())
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_assignment()).collect())
    }

    async fn find_active_by_user(&self, pool: &SqlitePool, user_id: Uuid) -> Result<Vec<AssetAssignment>> {
        let rows = sqlx::query_as::<_, AssignmentRow>(
            "SELECT id, asset_id, assigned_to, assigned_by, assigned_at, expected_return, returned_at, returned_by, notes, status FROM asset_assignments WHERE assigned_to = ? AND status = '\"Active\"' ORDER BY assigned_at DESC")
            .bind(user_id.to_string())
            .fetch_all(pool).await?;
        Ok(rows.into_iter().map(|r| r.into_assignment()).collect())
    }

    async fn update(&self, pool: &SqlitePool, assignment: &AssetAssignment) -> Result<()> {
        sqlx::query(
            "UPDATE asset_assignments SET returned_at = ?, returned_by = ?, notes = ?, status = ? WHERE id = ?")
            .bind(assignment.returned_at.map(|d| d.to_rfc3339()))
            .bind(assignment.returned_by.map(|id| id.to_string()))
            .bind(&assignment.notes)
            .bind(serde_json::to_string(&assignment.status)?)
            .bind(assignment.id.to_string())
            .execute(pool).await?;
        Ok(())
    }
}
