use async_trait::async_trait;
use sqlx::SqlitePool;
use erp_core::{Error, Result, Pagination, Paginated, BaseEntity};
use crate::models::*;
use uuid::Uuid;

#[async_trait]
pub trait BarcodeRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Barcode>;
    async fn find_by_barcode(&self, pool: &SqlitePool, barcode: &str) -> Result<Barcode>;
    async fn find_by_entity(&self, pool: &SqlitePool, entity_type: BarcodeEntityType, entity_id: Uuid) -> Result<Vec<Barcode>>;
    async fn create(&self, pool: &SqlitePool, barcode: Barcode) -> Result<Barcode>;
    async fn set_primary(&self, pool: &SqlitePool, id: Uuid) -> Result<()>;
}

pub struct SqliteBarcodeRepository;

#[async_trait]
impl BarcodeRepository for SqliteBarcodeRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<Barcode> {
        let row = sqlx::query_as::<_, BarcodeRow>(
            "SELECT * FROM barcodes WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("Barcode", &id.to_string()))?;
        
        Ok(row.into())
    }
    
    async fn find_by_barcode(&self, pool: &SqlitePool, barcode: &str) -> Result<Barcode> {
        let row = sqlx::query_as::<_, BarcodeRow>(
            "SELECT * FROM barcodes WHERE barcode = ?"
        )
        .bind(barcode)
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("Barcode", barcode))?;
        
        Ok(row.into())
    }
    
    async fn find_by_entity(&self, pool: &SqlitePool, entity_type: BarcodeEntityType, entity_id: Uuid) -> Result<Vec<Barcode>> {
        let rows = sqlx::query_as::<_, BarcodeRow>(
            "SELECT * FROM barcodes WHERE entity_type = ? AND entity_id = ?"
        )
        .bind(format!("{:?}", entity_type))
        .bind(entity_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    async fn create(&self, pool: &SqlitePool, barcode: Barcode) -> Result<Barcode> {
        sqlx::query(
            "INSERT INTO barcodes (id, barcode, barcode_type, entity_type, entity_id, definition_id, is_primary, status, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(barcode.base.id.to_string())
        .bind(&barcode.barcode)
        .bind(format!("{:?}", barcode.barcode_type))
        .bind(format!("{:?}", barcode.entity_type))
        .bind(barcode.entity_id.to_string())
        .bind(barcode.definition_id.map(|id| id.to_string()))
        .bind(barcode.is_primary as i32)
        .bind(format!("{:?}", barcode.status))
        .bind(barcode.created_at.to_rfc3339())
        .bind(barcode.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(barcode)
    }
    
    async fn set_primary(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        let barcode = self.find_by_id(pool, id).await?;
        
        sqlx::query(
            "UPDATE barcodes SET is_primary = 0 WHERE entity_type = ? AND entity_id = ?"
        )
        .bind(format!("{:?}", barcode.entity_type))
        .bind(barcode.entity_id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        sqlx::query(
            "UPDATE barcodes SET is_primary = 1 WHERE id = ?"
        )
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct BarcodeRow {
    id: String,
    barcode: String,
    barcode_type: String,
    entity_type: String,
    entity_id: String,
    definition_id: Option<String>,
    is_primary: i32,
    status: String,
    created_at: String,
    updated_at: String,
}

impl From<BarcodeRow> for Barcode {
    fn from(r: BarcodeRow) -> Self {
        Self {
            base: BaseEntity {
                id: Uuid::parse_str(&r.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&r.updated_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            barcode: r.barcode,
            barcode_type: match r.barcode_type.as_str() {
                "EAN8" => BarcodeType::EAN8,
                "UPC_A" => BarcodeType::UPC_A,
                "UPC_E" => BarcodeType::UPC_E,
                "Code128" => BarcodeType::Code128,
                "Code39" => BarcodeType::Code39,
                "Code93" => BarcodeType::Code93,
                "ITF14" => BarcodeType::ITF14,
                "QRCode" => BarcodeType::QRCode,
                "DataMatrix" => BarcodeType::DataMatrix,
                "PDF417" => BarcodeType::PDF417,
                "GS1_128" => BarcodeType::GS1_128,
                _ => BarcodeType::EAN13,
            },
            entity_type: match r.entity_type.as_str() {
                "Lot" => BarcodeEntityType::Lot,
                "SerialNumber" => BarcodeEntityType::SerialNumber,
                "Asset" => BarcodeEntityType::Asset,
                "Location" => BarcodeEntityType::Location,
                "Pallet" => BarcodeEntityType::Pallet,
                "Container" => BarcodeEntityType::Container,
                "Document" => BarcodeEntityType::Document,
                "Employee" => BarcodeEntityType::Employee,
                "Customer" => BarcodeEntityType::Customer,
                _ => BarcodeEntityType::Product,
            },
            entity_id: Uuid::parse_str(&r.entity_id).unwrap_or_default(),
            definition_id: r.definition_id.and_then(|id| Uuid::parse_str(&id).ok()),
            is_primary: r.is_primary != 0,
            status: match r.status.as_str() {
                "Inactive" => erp_core::Status::Inactive,
                _ => erp_core::Status::Active,
            },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[async_trait]
pub trait BarcodePrintJobRepository: Send + Sync {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<BarcodePrintJob>;
    async fn create(&self, pool: &SqlitePool, job: BarcodePrintJob) -> Result<BarcodePrintJob>;
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: PrintJobStatus) -> Result<()>;
}

pub struct SqliteBarcodePrintJobRepository;

#[async_trait]
impl BarcodePrintJobRepository for SqliteBarcodePrintJobRepository {
    async fn find_by_id(&self, pool: &SqlitePool, id: Uuid) -> Result<BarcodePrintJob> {
        let row = sqlx::query_as::<_, BarcodePrintJobRow>(
            "SELECT * FROM barcode_print_jobs WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("BarcodePrintJob", &id.to_string()))?;
        
        let items = self.get_items(pool, id).await?;
        Ok(row.into_job(items))
    }
    
    async fn create(&self, pool: &SqlitePool, job: BarcodePrintJob) -> Result<BarcodePrintJob> {
        sqlx::query(
            "INSERT INTO barcode_print_jobs (id, job_number, printer_id, template_id, quantity, printed_count, status, created_by, started_at, completed_at, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(job.base.id.to_string())
        .bind(&job.job_number)
        .bind(job.printer_id.to_string())
        .bind(job.template_id.to_string())
        .bind(job.quantity)
        .bind(job.printed_count)
        .bind(format!("{:?}", job.status))
        .bind(job.created_by.map(|id| id.to_string()))
        .bind(job.started_at.map(|d| d.to_rfc3339()))
        .bind(job.completed_at.map(|d| d.to_rfc3339()))
        .bind(job.base.created_at.to_rfc3339())
        .bind(job.base.updated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        for item in &job.items {
            self.create_item(pool, item).await?;
        }
        
        Ok(job)
    }
    
    async fn update_status(&self, pool: &SqlitePool, id: Uuid, status: PrintJobStatus) -> Result<()> {
        let now = chrono::Utc::now();
        let completed_at = matches!(status, PrintJobStatus::Completed | PrintJobStatus::Failed | PrintJobStatus::Cancelled).then_some(now);
        
        sqlx::query(
            "UPDATE barcode_print_jobs SET status = ?, completed_at = COALESCE(?, completed_at), updated_at = ? WHERE id = ?"
        )
        .bind(format!("{:?}", status))
        .bind(completed_at.map(|d| d.to_rfc3339()))
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

impl SqliteBarcodePrintJobRepository {
    async fn get_items(&self, pool: &SqlitePool, job_id: Uuid) -> Result<Vec<BarcodePrintItem>> {
        let rows = sqlx::query_as::<_, BarcodePrintItemRow>(
            "SELECT * FROM barcode_print_items WHERE job_id = ?"
        )
        .bind(job_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
    
    async fn create_item(&self, pool: &SqlitePool, item: &BarcodePrintItem) -> Result<()> {
        sqlx::query(
            "INSERT INTO barcode_print_items (id, job_id, barcode_id, barcode, entity_id, entity_type, copies, printed_copies, status, error_message)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(item.id.to_string())
        .bind(item.job_id.to_string())
        .bind(item.barcode_id.to_string())
        .bind(&item.barcode)
        .bind(item.entity_id.to_string())
        .bind(format!("{:?}", item.entity_type))
        .bind(item.copies)
        .bind(item.printed_copies)
        .bind(format!("{:?}", item.status))
        .bind(&item.error_message)
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct BarcodePrintJobRow {
    id: String,
    job_number: String,
    printer_id: String,
    template_id: String,
    quantity: i32,
    printed_count: i32,
    status: String,
    created_by: Option<String>,
    started_at: Option<String>,
    completed_at: Option<String>,
    created_at: String,
    updated_at: String,
}

impl BarcodePrintJobRow {
    fn into_job(self, items: Vec<BarcodePrintItem>) -> BarcodePrintJob {
        BarcodePrintJob {
            base: BaseEntity {
                id: Uuid::parse_str(&self.id).unwrap_or_default(),
                created_at: chrono::DateTime::parse_from_rfc3339(&self.created_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                updated_at: chrono::DateTime::parse_from_rfc3339(&self.updated_at)
                    .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
                created_by: None,
                updated_by: None,
            },
            job_number: self.job_number,
            printer_id: Uuid::parse_str(&self.printer_id).unwrap_or_default(),
            template_id: Uuid::parse_str(&self.template_id).unwrap_or_default(),
            quantity: self.quantity,
            printed_count: self.printed_count,
            status: match self.status.as_str() {
                "Queued" => PrintJobStatus::Queued,
                "Printing" => PrintJobStatus::Printing,
                "Completed" => PrintJobStatus::Completed,
                "Failed" => PrintJobStatus::Failed,
                "Cancelled" => PrintJobStatus::Cancelled,
                _ => PrintJobStatus::Pending,
            },
            created_by: self.created_by.and_then(|id| Uuid::parse_str(&id).ok()),
            started_at: self.started_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            completed_at: self.completed_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            items,
        }
    }
}

#[derive(sqlx::FromRow)]
struct BarcodePrintItemRow {
    id: String,
    job_id: String,
    barcode_id: String,
    barcode: String,
    entity_id: String,
    entity_type: String,
    copies: i32,
    printed_copies: i32,
    status: String,
    error_message: Option<String>,
}

impl From<BarcodePrintItemRow> for BarcodePrintItem {
    fn from(r: BarcodePrintItemRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            job_id: Uuid::parse_str(&r.job_id).unwrap_or_default(),
            barcode_id: Uuid::parse_str(&r.barcode_id).unwrap_or_default(),
            barcode: r.barcode,
            entity_id: Uuid::parse_str(&r.entity_id).unwrap_or_default(),
            entity_type: match r.entity_type.as_str() {
                "Lot" => BarcodeEntityType::Lot,
                "SerialNumber" => BarcodeEntityType::SerialNumber,
                "Asset" => BarcodeEntityType::Asset,
                "Location" => BarcodeEntityType::Location,
                "Pallet" => BarcodeEntityType::Pallet,
                "Container" => BarcodeEntityType::Container,
                "Document" => BarcodeEntityType::Document,
                "Employee" => BarcodeEntityType::Employee,
                "Customer" => BarcodeEntityType::Customer,
                _ => BarcodeEntityType::Product,
            },
            copies: r.copies,
            printed_copies: r.printed_copies,
            status: match r.status.as_str() {
                "Queued" => PrintJobStatus::Queued,
                "Printing" => PrintJobStatus::Printing,
                "Completed" => PrintJobStatus::Completed,
                "Failed" => PrintJobStatus::Failed,
                "Cancelled" => PrintJobStatus::Cancelled,
                _ => PrintJobStatus::Pending,
            },
            error_message: r.error_message,
        }
    }
}

#[async_trait]
pub trait ScanEventRepository: Send + Sync {
    async fn create(&self, pool: &SqlitePool, event: ScanEvent) -> Result<ScanEvent>;
    async fn find_by_barcode(&self, pool: &SqlitePool, barcode: &str, limit: i64) -> Result<Vec<ScanEvent>>;
}

pub struct SqliteScanEventRepository;

#[async_trait]
impl ScanEventRepository for SqliteScanEventRepository {
    async fn create(&self, pool: &SqlitePool, event: ScanEvent) -> Result<ScanEvent> {
        sqlx::query(
            "INSERT INTO scan_events (id, barcode, barcode_type, scanner_id, user_id, location_id, entity_type, entity_id, action, quantity, reference_type, reference_id, scanned_at, metadata)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(event.id.to_string())
        .bind(&event.barcode)
        .bind(format!("{:?}", event.barcode_type))
        .bind(event.scanner_id.to_string())
        .bind(event.user_id.map(|id| id.to_string()))
        .bind(event.location_id.map(|id| id.to_string()))
        .bind(format!("{:?}", event.entity_type))
        .bind(event.entity_id.map(|id| id.to_string()))
        .bind(format!("{:?}", event.action))
        .bind(event.quantity)
        .bind(&event.reference_type)
        .bind(event.reference_id.map(|id| id.to_string()))
        .bind(event.scanned_at.to_rfc3339())
        .bind(&event.metadata)
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(event)
    }
    
    async fn find_by_barcode(&self, pool: &SqlitePool, barcode: &str, limit: i64) -> Result<Vec<ScanEvent>> {
        let rows = sqlx::query_as::<_, ScanEventRow>(
            "SELECT * FROM scan_events WHERE barcode = ? ORDER BY scanned_at DESC LIMIT ?"
        )
        .bind(barcode)
        .bind(limit)
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

#[derive(sqlx::FromRow)]
struct ScanEventRow {
    id: String,
    barcode: String,
    barcode_type: String,
    scanner_id: String,
    user_id: Option<String>,
    location_id: Option<String>,
    entity_type: String,
    entity_id: Option<String>,
    action: String,
    quantity: i64,
    reference_type: Option<String>,
    reference_id: Option<String>,
    scanned_at: String,
    metadata: Option<String>,
}

impl From<ScanEventRow> for ScanEvent {
    fn from(r: ScanEventRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            barcode: r.barcode,
            barcode_type: match r.barcode_type.as_str() {
                "EAN8" => BarcodeType::EAN8,
                "UPC_A" => BarcodeType::UPC_A,
                "UPC_E" => BarcodeType::UPC_E,
                "Code128" => BarcodeType::Code128,
                "Code39" => BarcodeType::Code39,
                "Code93" => BarcodeType::Code93,
                "ITF14" => BarcodeType::ITF14,
                "QRCode" => BarcodeType::QRCode,
                "DataMatrix" => BarcodeType::DataMatrix,
                "PDF417" => BarcodeType::PDF417,
                "GS1_128" => BarcodeType::GS1_128,
                _ => BarcodeType::EAN13,
            },
            scanner_id: Uuid::parse_str(&r.scanner_id).unwrap_or_default(),
            user_id: r.user_id.and_then(|id| Uuid::parse_str(&id).ok()),
            location_id: r.location_id.and_then(|id| Uuid::parse_str(&id).ok()),
            entity_type: match r.entity_type.as_str() {
                "Lot" => BarcodeEntityType::Lot,
                "SerialNumber" => BarcodeEntityType::SerialNumber,
                "Asset" => BarcodeEntityType::Asset,
                "Location" => BarcodeEntityType::Location,
                "Pallet" => BarcodeEntityType::Pallet,
                "Container" => BarcodeEntityType::Container,
                "Document" => BarcodeEntityType::Document,
                "Employee" => BarcodeEntityType::Employee,
                "Customer" => BarcodeEntityType::Customer,
                _ => BarcodeEntityType::Product,
            },
            entity_id: r.entity_id.and_then(|id| Uuid::parse_str(&id).ok()),
            action: match r.action.as_str() {
                "Receive" => ScanAction::Receive,
                "Pick" => ScanAction::Pick,
                "Pack" => ScanAction::Pack,
                "Ship" => ScanAction::Ship,
                "Count" => ScanAction::Count,
                "Move" => ScanAction::Move,
                "Issue" => ScanAction::Issue,
                "Return" => ScanAction::Return,
                "Verify" => ScanAction::Verify,
                _ => ScanAction::Lookup,
            },
            quantity: r.quantity,
            reference_type: r.reference_type,
            reference_id: r.reference_id.and_then(|id| Uuid::parse_str(&id).ok()),
            scanned_at: chrono::DateTime::parse_from_rfc3339(&r.scanned_at)
                .map(|d| d.with_timezone(&chrono::Utc)).unwrap_or_else(|_| chrono::Utc::now()),
            metadata: r.metadata,
        }
    }
}
