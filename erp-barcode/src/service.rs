use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use erp_core::{Error, Result, BaseEntity};
use crate::models::*;
use crate::repository::*;

pub struct BarcodeService { repo: SqliteBarcodeRepository }
impl BarcodeService {
    pub fn new() -> Self { Self { repo: SqliteBarcodeRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<Barcode> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn get_by_barcode(&self, pool: &SqlitePool, barcode: &str) -> Result<Barcode> {
        self.repo.find_by_barcode(pool, barcode).await
    }
    
    pub async fn get_by_entity(&self, pool: &SqlitePool, entity_type: BarcodeEntityType, entity_id: Uuid) -> Result<Vec<Barcode>> {
        self.repo.find_by_entity(pool, entity_type, entity_id).await
    }
    
    pub async fn create(&self, pool: &SqlitePool, mut barcode: Barcode) -> Result<Barcode> {
        if barcode.barcode.is_empty() {
            return Err(Error::validation("Barcode value is required"));
        }
        
        if self.repo.find_by_barcode(pool, &barcode.barcode).await.is_ok() {
            return Err(Error::validation("Barcode already exists"));
        }
        
        barcode.base = BaseEntity::new();
        barcode.status = erp_core::Status::Active;
        barcode.created_at = Utc::now();
        self.repo.create(pool, barcode).await
    }
    
    pub async fn generate(&self, pool: &SqlitePool, entity_type: BarcodeEntityType, entity_id: Uuid, barcode_type: BarcodeType, definition_id: Option<Uuid>) -> Result<Barcode> {
        let barcode_value = match definition_id {
            Some(def_id) => self.generate_from_definition(pool, def_id).await?,
            None => Self::generate_default(&barcode_type),
        };
        
        let barcode = Barcode {
            base: BaseEntity::new(),
            barcode: barcode_value,
            barcode_type,
            entity_type,
            entity_id,
            definition_id,
            is_primary: true,
            status: erp_core::Status::Active,
            created_at: Utc::now(),
        };
        
        self.repo.create(pool, barcode).await
    }
    
    async fn generate_from_definition(&self, pool: &SqlitePool, definition_id: Uuid) -> Result<String> {
        let row = sqlx::query_as::<_, BarcodeDefinitionRow>(
            "SELECT * FROM barcode_definitions WHERE id = ?"
        )
        .bind(definition_id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("BarcodeDefinition", &definition_id.to_string()))?;
        
        let next_seq = row.sequence_current + 1;
        sqlx::query(
            "UPDATE barcode_definitions SET sequence_current = ? WHERE id = ?"
        )
        .bind(next_seq)
        .bind(definition_id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        let padded = format!("{:0width$}", next_seq, width = row.padding_length as usize);
        let mut result = row.prefix.unwrap_or_default();
        result.push_str(&padded);
        result.push_str(&row.suffix.unwrap_or_default());
        
        Ok(result)
    }
    
    fn generate_default(barcode_type: &BarcodeType) -> String {
        let timestamp = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let random: u32 = rand::random();
        
        match barcode_type {
            BarcodeType::EAN13 => format!("{}{:05}", timestamp, random % 100000),
            BarcodeType::UpcA => format!("{}{:04}", &timestamp[4..], random % 10000),
            BarcodeType::Code128 => format!("BC{}", timestamp),
            BarcodeType::Code39 => format!("BC{}", timestamp),
            BarcodeType::QRCode => format!("QR-{}-{}", timestamp, random),
            _ => format!("BC{}", timestamp),
        }
    }
    
    pub async fn set_primary(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.set_primary(pool, id).await
    }
}

#[derive(sqlx::FromRow)]
struct BarcodeDefinitionRow {
    id: String,
    prefix: Option<String>,
    suffix: Option<String>,
    padding_length: i32,
    sequence_current: i64,
}

pub struct BarcodePrintService { repo: SqliteBarcodePrintJobRepository }
impl BarcodePrintService {
    pub fn new() -> Self { Self { repo: SqliteBarcodePrintJobRepository } }
    
    pub async fn get(&self, pool: &SqlitePool, id: Uuid) -> Result<BarcodePrintJob> {
        self.repo.find_by_id(pool, id).await
    }
    
    pub async fn create_job(
        &self,
        pool: &SqlitePool,
        printer_id: Uuid,
        template_id: Uuid,
        items: Vec<(Uuid, String, BarcodeEntityType, Uuid)>,
    ) -> Result<BarcodePrintJob> {
        let job_number = format!("BPJ-{}", Utc::now().format("%Y%m%d%H%M%S"));
        let quantity = items.len() as i32;
        
        let mut print_items = Vec::new();
        for (barcode_id, barcode, entity_type, entity_id) in items {
            print_items.push(BarcodePrintItem {
                id: Uuid::new_v4(),
                job_id: Uuid::nil(),
                barcode_id,
                barcode,
                entity_id,
                entity_type,
                copies: 1,
                printed_copies: 0,
                status: PrintJobStatus::Pending,
                error_message: None,
            });
        }
        
        let job = BarcodePrintJob {
            base: BaseEntity::new(),
            job_number,
            printer_id,
            template_id,
            quantity,
            printed_count: 0,
            status: PrintJobStatus::Pending,
            created_by: None,
            started_at: None,
            completed_at: None,
            items: print_items,
        };
        
        self.repo.create(pool, job).await
    }
    
    pub async fn start(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, PrintJobStatus::Printing).await
    }
    
    pub async fn complete(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, PrintJobStatus::Completed).await
    }
    
    pub async fn cancel(&self, pool: &SqlitePool, id: Uuid) -> Result<()> {
        self.repo.update_status(pool, id, PrintJobStatus::Cancelled).await
    }
}

pub struct ScanService { repo: SqliteScanEventRepository }
impl ScanService {
    pub fn new() -> Self { Self { repo: SqliteScanEventRepository } }
    
    pub async fn scan(
        &self,
        pool: &SqlitePool,
        barcode: &str,
        barcode_type: BarcodeType,
        scanner_id: Uuid,
        user_id: Option<Uuid>,
        location_id: Option<Uuid>,
        action: ScanAction,
        quantity: i64,
        reference_type: Option<&str>,
        reference_id: Option<Uuid>,
    ) -> Result<ScanEvent> {
        let barcode_repo = SqliteBarcodeRepository;
        let entity = barcode_repo.find_by_barcode(pool, barcode).await.ok();
        
        let event = ScanEvent {
            id: Uuid::new_v4(),
            barcode: barcode.to_string(),
            barcode_type,
            scanner_id,
            user_id,
            location_id,
            entity_type: entity.as_ref().map(|e| e.entity_type.clone()).unwrap_or(BarcodeEntityType::Product),
            entity_id: entity.map(|e| e.entity_id),
            action,
            quantity,
            reference_type: reference_type.map(|s| s.to_string()),
            reference_id,
            scanned_at: Utc::now(),
            metadata: None,
        };
        
        self.repo.create(pool, event.clone()).await
    }
    
    pub async fn get_history(&self, pool: &SqlitePool, barcode: &str, limit: i64) -> Result<Vec<ScanEvent>> {
        self.repo.find_by_barcode(pool, barcode, limit).await
    }
}

pub struct BarcodeValidationService;
impl BarcodeValidationService {
    pub fn new() -> Self { Self }
    
    pub fn validate(barcode: &str, barcode_type: &BarcodeType) -> BarcodeValidation {
        let (is_valid, check_digit, calculated_check_digit, errors) = match barcode_type {
            BarcodeType::EAN13 => Self::validate_ean13(barcode),
            BarcodeType::UpcA => Self::validate_upc_a(barcode),
            BarcodeType::EAN8 => Self::validate_ean8(barcode),
            _ => (true, None::<String>, None::<String>, None::<String>),
        };
        
        BarcodeValidation {
            id: Uuid::new_v4(),
            barcode: barcode.to_string(),
            barcode_type: barcode_type.clone(),
            is_valid,
            validation_errors: errors,
            check_digit,
            calculated_check_digit,
            validated_at: Utc::now(),
        }
    }
    
    fn validate_ean13(barcode: &str) -> (bool, Option<String>, Option<String>, Option<String>) {
        if barcode.len() != 13 {
            return (false, None, None, Some("EAN-13 must be 13 digits".to_string()));
        }
        
        let digits: Vec<u32> = barcode.chars().filter_map(|c| c.to_digit(10)).collect();
        if digits.len() != 13 {
            return (false, None, None, Some("EAN-13 must contain only digits".to_string()));
        }
        
        let check = digits[12];
        let calculated = Self::calculate_ean_check_digit(&digits[..12]);
        
        (calculated == check, Some(check.to_string()), Some(calculated.to_string()), None)
    }
    
    fn validate_upc_a(barcode: &str) -> (bool, Option<String>, Option<String>, Option<String>) {
        if barcode.len() != 12 {
            return (false, None, None, Some("UPC-A must be 12 digits".to_string()));
        }
        
        let digits: Vec<u32> = barcode.chars().filter_map(|c| c.to_digit(10)).collect();
        if digits.len() != 12 {
            return (false, None, None, Some("UPC-A must contain only digits".to_string()));
        }
        
        let check = digits[11];
        let calculated = Self::calculate_ean_check_digit(&digits[..11]);
        
        (calculated == check, Some(check.to_string()), Some(calculated.to_string()), None)
    }
    
    fn validate_ean8(barcode: &str) -> (bool, Option<String>, Option<String>, Option<String>) {
        if barcode.len() != 8 {
            return (false, None, None, Some("EAN-8 must be 8 digits".to_string()));
        }
        
        let digits: Vec<u32> = barcode.chars().filter_map(|c| c.to_digit(10)).collect();
        if digits.len() != 8 {
            return (false, None, None, Some("EAN-8 must contain only digits".to_string()));
        }
        
        let check = digits[7];
        let calculated = Self::calculate_ean_check_digit(&digits[..7]);
        
        (calculated == check, Some(check.to_string()), Some(calculated.to_string()), None)
    }
    
    fn calculate_ean_check_digit(digits: &[u32]) -> u32 {
        let sum: u32 = digits.iter().enumerate().map(|(i, &d)| {
            if i % 2 == 0 { d } else { d * 3 }
        }).sum();
        let remainder = sum % 10;
        if remainder == 0 { 0 } else { 10 - remainder }
    }
}
