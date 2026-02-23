use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BarcodeType {
    EAN13,
    EAN8,
    UPC_A,
    UPC_E,
    Code128,
    Code39,
    Code93,
    ITF14,
    QRCode,
    DataMatrix,
    PDF417,
    GS1_128,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BarcodeEntityType {
    Product,
    Lot,
    SerialNumber,
    Asset,
    Location,
    Pallet,
    Container,
    Document,
    Employee,
    Customer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodeDefinition {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub barcode_type: BarcodeType,
    pub entity_type: BarcodeEntityType,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub include_check_digit: bool,
    pub auto_generate: bool,
    pub sequence_start: i64,
    pub sequence_current: i64,
    pub padding_length: i32,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Barcode {
    pub base: BaseEntity,
    pub barcode: String,
    pub barcode_type: BarcodeType,
    pub entity_type: BarcodeEntityType,
    pub entity_id: Uuid,
    pub definition_id: Option<Uuid>,
    pub is_primary: bool,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodePrintJob {
    pub base: BaseEntity,
    pub job_number: String,
    pub printer_id: Uuid,
    pub template_id: Uuid,
    pub quantity: i32,
    pub printed_count: i32,
    pub status: PrintJobStatus,
    pub created_by: Option<Uuid>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub items: Vec<BarcodePrintItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PrintJobStatus {
    Pending,
    Queued,
    Printing,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodePrintItem {
    pub id: Uuid,
    pub job_id: Uuid,
    pub barcode_id: Uuid,
    pub barcode: String,
    pub entity_id: Uuid,
    pub entity_type: BarcodeEntityType,
    pub copies: i32,
    pub printed_copies: i32,
    pub status: PrintJobStatus,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodePrinter {
    pub base: BaseEntity,
    pub name: String,
    pub printer_type: PrinterType,
    pub ip_address: Option<String>,
    pub port: Option<i32>,
    pub connection_type: PrinterConnection,
    pub dpi: i32,
    pub label_width_mm: f64,
    pub label_height_mm: f64,
    pub status: Status,
    pub last_seen_at: Option<DateTime<Utc>>,
    pub is_default: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PrinterType {
    Thermal,
    Laser,
    Inkjet,
    DotMatrix,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PrinterConnection {
    Network,
    USB,
    Serial,
    Parallel,
    Bluetooth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodeTemplate {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub width_mm: f64,
    pub height_mm: f64,
    pub margin_top_mm: f64,
    pub margin_bottom_mm: f64,
    pub margin_left_mm: f64,
    pub margin_right_mm: f64,
    pub barcode_type: BarcodeType,
    pub barcode_width_mm: f64,
    pub barcode_height_mm: f64,
    pub barcode_position_x: f64,
    pub barcode_position_y: f64,
    pub include_text: bool,
    pub text_font: String,
    pub text_size: i32,
    pub text_position: TextPosition,
    pub elements: Vec<TemplateElement>,
    pub zpl_template: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TextPosition {
    Below,
    Above,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateElement {
    pub id: Uuid,
    pub template_id: Uuid,
    pub element_type: ElementType,
    pub position_x: f64,
    pub position_y: f64,
    pub width: f64,
    pub height: f64,
    pub content: String,
    pub font_size: i32,
    pub font_name: String,
    pub alignment: TextAlignment,
    pub is_bold: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ElementType {
    Text,
    Barcode,
    QRCode,
    Line,
    Rectangle,
    Image,
    DataMatrix,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TextAlignment {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanEvent {
    pub id: Uuid,
    pub barcode: String,
    pub barcode_type: BarcodeType,
    pub scanner_id: Uuid,
    pub user_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub entity_type: BarcodeEntityType,
    pub entity_id: Option<Uuid>,
    pub action: ScanAction,
    pub quantity: i64,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub scanned_at: DateTime<Utc>,
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ScanAction {
    Lookup,
    Receive,
    Pick,
    Pack,
    Ship,
    Count,
    Move,
    Issue,
    Return,
    Verify,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodeScanner {
    pub base: BaseEntity,
    pub name: String,
    pub scanner_type: ScannerType,
    pub connection_type: ScannerConnection,
    pub device_id: Option<String>,
    pub location_id: Option<Uuid>,
    pub status: Status,
    pub last_used_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ScannerType {
    Handheld,
    Fixed,
    Mobile,
    Presentation,
    Wearable,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ScannerConnection {
    USB,
    Bluetooth,
    Serial,
    Network,
    Wireless,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerialNumber {
    pub base: BaseEntity,
    pub serial_number: String,
    pub product_id: Uuid,
    pub lot_id: Option<Uuid>,
    pub warehouse_id: Option<Uuid>,
    pub location_id: Option<Uuid>,
    pub status: SerialStatus,
    pub manufactured_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub warranty_expiry: Option<DateTime<Utc>>,
    pub customer_id: Option<Uuid>,
    pub sale_date: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SerialStatus {
    Available,
    Reserved,
    Sold,
    InTransit,
    Damaged,
    Returned,
    WrittenOff,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GS1Prefix {
    pub base: BaseEntity,
    pub prefix: String,
    pub company_name: String,
    pub country_code: String,
    pub status: Status,
    pub valid_from: DateTime<Utc>,
    pub valid_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodeValidation {
    pub id: Uuid,
    pub barcode: String,
    pub barcode_type: BarcodeType,
    pub is_valid: bool,
    pub validation_errors: Option<String>,
    pub check_digit: Option<String>,
    pub calculated_check_digit: Option<String>,
    pub validated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BarcodeRule {
    pub base: BaseEntity,
    pub name: String,
    pub barcode_pattern: String,
    pub entity_type: BarcodeEntityType,
    pub action: String,
    pub parameters: String,
    pub priority: i32,
    pub status: Status,
}
