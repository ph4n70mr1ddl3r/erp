use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntercompanyEntity {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub legal_entity_id: Option<Uuid>,
    pub currency: String,
    pub timezone: String,
    pub tax_id: Option<String>,
    pub status: EntityStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityStatus {
    Active,
    Inactive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntercompanyTransaction {
    pub id: Uuid,
    pub transaction_number: String,
    pub source_entity_id: Uuid,
    pub target_entity_id: Uuid,
    pub transaction_type: ICType,
    pub source_amount: i64,
    pub source_currency: String,
    pub target_amount: i64,
    pub target_currency: String,
    pub exchange_rate: f64,
    pub status: ICTransactionStatus,
    pub source_document_id: Option<Uuid>,
    pub target_document_id: Option<Uuid>,
    pub due_date: Option<chrono::NaiveDate>,
    pub created_at: DateTime<Utc>,
    pub settled_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ICType {
    Sale,
    Purchase,
    Transfer,
    Loan,
    Service,
    Royalty,
    Dividend,
    ManagementFee,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ICTransactionStatus {
    Draft,
    Submitted,
    Approved,
    Rejected,
    InTransit,
    Received,
    Settled,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferPrice {
    pub id: Uuid,
    pub product_id: Uuid,
    pub source_entity_id: Uuid,
    pub target_entity_id: Uuid,
    pub price: i64,
    pub currency: String,
    pub method: TransferPriceMethod,
    pub effective_from: chrono::NaiveDate,
    pub effective_to: Option<chrono::NaiveDate>,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransferPriceMethod {
    ComparableUncontrolledPrice,
    ResalePrice,
    CostPlus,
    ProfitSplit,
    TNMM,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DueToFrom {
    pub id: Uuid,
    pub source_entity_id: Uuid,
    pub target_entity_id: Uuid,
    pub account_type: DueToFromType,
    pub amount: i64,
    pub currency: String,
    pub as_of_date: chrono::NaiveDate,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DueToFromType {
    DueTo,
    DueFrom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EliminationEntry {
    pub id: Uuid,
    pub consolidation_id: Uuid,
    pub source_transaction_id: Uuid,
    pub debit_entity_id: Uuid,
    pub credit_entity_id: Uuid,
    pub account_code: String,
    pub amount: i64,
    pub currency: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Consolidation {
    pub id: Uuid,
    pub consolidation_number: String,
    pub period: String,
    pub status: ConsolidationStatus,
    pub entities: Vec<Uuid>,
    pub elimination_entries: Vec<Uuid>,
    pub started_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConsolidationStatus {
    Draft,
    InProgress,
    Completed,
    Posted,
    Reversed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntercompanyAgreement {
    pub id: Uuid,
    pub agreement_number: String,
    pub name: String,
    pub source_entity_id: Uuid,
    pub target_entity_id: Uuid,
    pub agreement_type: AgreementType,
    pub start_date: chrono::NaiveDate,
    pub end_date: Option<chrono::NaiveDate>,
    pub terms: serde_json::Value,
    pub status: AgreementStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgreementType {
    ServiceLevel,
    Pricing,
    Distribution,
    Licensing,
    SharedServices,
    CostSharing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgreementStatus {
    Draft,
    Active,
    Expired,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateICTransactionRequest {
    pub source_entity_id: Uuid,
    pub target_entity_id: Uuid,
    pub transaction_type: ICType,
    pub source_amount: i64,
    pub source_currency: String,
    pub target_currency: String,
    pub exchange_rate: Option<f64>,
    pub due_date: Option<chrono::NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTransferPriceRequest {
    pub product_id: Uuid,
    pub source_entity_id: Uuid,
    pub target_entity_id: Uuid,
    pub price: i64,
    pub currency: String,
    pub method: TransferPriceMethod,
    pub effective_from: chrono::NaiveDate,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunConsolidationRequest {
    pub period: String,
    pub entity_ids: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsolidationResult {
    pub consolidation_id: Uuid,
    pub elimination_count: i32,
    pub total_eliminations: i64,
    pub status: ConsolidationStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateEntityRequest {
    pub code: String,
    pub name: String,
    pub legal_entity_id: Option<Uuid>,
    pub currency: String,
}
