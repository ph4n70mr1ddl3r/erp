use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdiPartner {
    pub id: Uuid,
    pub partner_code: String,
    pub partner_name: String,
    pub partner_type: PartnerType,
    pub qualifier: String,
    pub interchange_id: String,
    pub communication_type: CommunicationType,
    pub endpoint: String,
    pub encryption: Option<String>,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PartnerType {
    Customer,
    Vendor,
    Carrier,
    Bank,
    Warehouse,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CommunicationType {
    AS2,
    SFTP,
    FTP,
    HTTP,
    VAN,
    API,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdiTransaction {
    pub id: Uuid,
    pub partner_id: Uuid,
    pub transaction_type: EdiTransactionType,
    pub direction: EdiDirection,
    pub control_number: String,
    pub status: EdiStatus,
    pub raw_content: Option<String>,
    pub parsed_data: Option<serde_json::Value>,
    pub error_message: Option<String>,
    pub processed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdiTransactionType {
    X12_850,
    X12_810,
    X12_856,
    X12_855,
    X12_860,
    X12_865,
    X12_940,
    X12_945,
    X12_997,
    X12_820,
    EdifactOrders,
    EdifactInvoic,
    EdifactDesadv,
    PeppolInvoice,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdiDirection {
    Inbound,
    Outbound,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EdiStatus {
    Received,
    Validated,
    Parsing,
    Parsed,
    Processing,
    Processed,
    Error,
    Acknowledged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdiMapping {
    pub id: Uuid,
    pub name: String,
    pub transaction_type: EdiTransactionType,
    pub version: String,
    pub mapping_rules: serde_json::Value,
    pub is_active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdiAcknowledgment {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub ack_type: AckType,
    pub accepted: bool,
    pub error_codes: Vec<String>,
    pub segment_errors: Vec<SegmentError>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AckType {
    TA1,
    FA997,
    CONTRL,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SegmentError {
    pub segment_id: String,
    pub segment_position: i32,
    pub loop_id: Option<String>,
    pub error_code: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edi850PurchaseOrder {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub po_number: String,
    pub po_date: NaiveDate,
    pub customer_id: Uuid,
    pub ship_to: EdiAddress,
    pub bill_to: EdiAddress,
    pub lines: Vec<Edi850Line>,
    pub total_amount: i64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edi850Line {
    pub line_number: i32,
    pub product_id: Option<Uuid>,
    pub sku: String,
    pub description: String,
    pub quantity: i64,
    pub unit_price: i64,
    pub uom: String,
    pub requested_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edi810Invoice {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub invoice_number: String,
    pub invoice_date: NaiveDate,
    pub po_number: String,
    pub vendor_id: Uuid,
    pub lines: Vec<Edi810Line>,
    pub subtotal: i64,
    pub tax: i64,
    pub total: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edi810Line {
    pub line_number: i32,
    pub sku: String,
    pub quantity: i64,
    pub unit_price: i64,
    pub extended_price: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edi856ASN {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub asn_number: String,
    pub shipment_date: NaiveDate,
    pub expected_date: NaiveDate,
    pub po_number: String,
    pub carrier: String,
    pub tracking_number: Option<String>,
    pub packages: Vec<Edi856Package>,
    pub total_items: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edi856Package {
    pub package_id: String,
    pub weight: f64,
    pub items: Vec<Edi856Item>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edi856Item {
    pub sku: String,
    pub quantity: i64,
    pub lot_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdiAddress {
    pub name: String,
    pub address1: String,
    pub address2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePartnerRequest {
    pub partner_code: String,
    pub partner_name: String,
    pub partner_type: PartnerType,
    pub qualifier: String,
    pub interchange_id: String,
    pub communication_type: CommunicationType,
    pub endpoint: String,
    pub encryption: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessEdiRequest {
    pub raw_content: String,
    pub partner_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateEdiRequest {
    pub transaction_type: EdiTransactionType,
    pub partner_id: Uuid,
    pub reference_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdiTransmissionResult {
    pub transaction_id: Uuid,
    pub control_number: String,
    pub raw_content: String,
    pub sent_at: DateTime<Utc>,
}
