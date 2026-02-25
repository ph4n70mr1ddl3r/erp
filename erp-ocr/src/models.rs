use chrono::{DateTime, Utc};
use erp_core::models::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrDocument {
    pub base: BaseEntity,
    pub document_type: DocumentType,
    pub original_filename: String,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub status: OcrStatus,
    pub processing_started_at: Option<DateTime<Utc>>,
    pub processing_completed_at: Option<DateTime<Utc>>,
    pub confidence_score: Option<f64>,
    pub raw_text: Option<String>,
    pub extracted_data: Option<serde_json::Value>,
    pub validation_errors: Vec<String>,
    pub reviewed_by: Option<Uuid>,
    pub reviewed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum DocumentType {
    Invoice,
    Receipt,
    PurchaseOrder,
    SalesOrder,
    DeliveryNote,
    BankStatement,
    Check,
    Contract,
    IdDocument,
    BusinessCard,
    Form,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum OcrStatus {
    Pending,
    Processing,
    Completed,
    Failed,
    RequiresReview,
    Validated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrTemplate {
    pub base: BaseEntity,
    pub name: String,
    pub document_type: DocumentType,
    pub vendor_id: Option<Uuid>,
    pub field_mappings: Vec<FieldMapping>,
    pub sample_images: Vec<String>,
    pub accuracy_score: f64,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldMapping {
    pub source_field: String,
    pub target_field: String,
    pub extraction_pattern: Option<String>,
    pub data_type: DataType,
    pub required: bool,
    pub validation_rules: Vec<ValidationRule>,
    pub default_value: Option<String>,
    pub transform_rules: Vec<TransformRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    String,
    Number,
    Date,
    Currency,
    Percentage,
    Boolean,
    Email,
    Phone,
    Address,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationRule {
    pub rule_type: ValidationRuleType,
    pub parameters: serde_json::Value,
    pub error_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValidationRuleType {
    Required,
    MinLength(i32),
    MaxLength(i32),
    MinValue(f64),
    MaxValue(f64),
    Pattern(String),
    InList(Vec<String>),
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransformRule {
    pub transform_type: TransformType,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransformType {
    Uppercase,
    Lowercase,
    Trim,
    DateFormat(String),
    NumberFormat(String),
    CurrencyToCents,
    RegexExtract(String),
    Replace(String, String),
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedInvoice {
    pub invoice_number: String,
    pub invoice_date: DateTime<Utc>,
    pub due_date: Option<DateTime<Utc>>,
    pub vendor_name: String,
    pub vendor_address: Option<String>,
    pub vendor_tax_id: Option<String>,
    pub customer_name: Option<String>,
    pub customer_address: Option<String>,
    pub line_items: Vec<ExtractedLineItem>,
    pub subtotal: i64,
    pub tax_amount: i64,
    pub total_amount: i64,
    pub currency: String,
    pub payment_terms: Option<String>,
    pub notes: Option<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedLineItem {
    pub description: String,
    pub quantity: f64,
    pub unit_price: i64,
    pub amount: i64,
    pub tax_rate: Option<f64>,
    pub product_code: Option<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedReceipt {
    pub merchant_name: String,
    pub merchant_address: Option<String>,
    pub receipt_date: DateTime<Utc>,
    pub receipt_number: Option<String>,
    pub line_items: Vec<ExtractedLineItem>,
    pub subtotal: i64,
    pub tax_amount: i64,
    pub total_amount: i64,
    pub payment_method: Option<String>,
    pub currency: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrBatchJob {
    pub base: BaseEntity,
    pub name: String,
    pub document_ids: Vec<Uuid>,
    pub template_id: Option<Uuid>,
    pub status: BatchJobStatus,
    pub total_documents: i32,
    pub processed_documents: i32,
    pub successful_documents: i32,
    pub failed_documents: i32,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub error_details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "text")]
pub enum BatchJobStatus {
    Pending,
    Running,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrSettings {
    pub default_language: String,
    pub auto_detect_language: bool,
    pub output_format: OutputFormat,
    pub enable_table_extraction: bool,
    pub enable_handwriting_recognition: bool,
    pub confidence_threshold: f64,
    pub auto_validate: bool,
    pub auto_create_entities: bool,
    pub retention_days: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OutputFormat {
    Json,
    Xml,
    Csv,
    PlainText,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadDocumentRequest {
    pub document_type: DocumentType,
    pub filename: String,
    pub content: String,
    pub template_id: Option<Uuid>,
    pub auto_process: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrResult {
    pub document_id: Uuid,
    pub status: OcrStatus,
    pub extracted_data: Option<serde_json::Value>,
    pub raw_text: Option<String>,
    pub confidence: f64,
    pub validation_errors: Vec<String>,
    pub suggestions: Vec<String>,
}
