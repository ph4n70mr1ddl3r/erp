use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SourcingStatus {
    Draft,
    Published,
    Bidding,
    Evaluation,
    Awarded,
    Cancelled,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AuctionType {
    Forward,
    Reverse,
    Dutch,
    English,
    SealedBid,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BidStatus {
    Draft,
    Submitted,
    UnderReview,
    Accepted,
    Rejected,
    Withdrawn,
    Winner,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcingEvent {
    pub base: BaseEntity,
    pub event_number: String,
    pub title: String,
    pub description: Option<String>,
    pub event_type: SourcingEventType,
    pub status: SourcingStatus,
    pub auction_type: Option<AuctionType>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub currency: String,
    pub estimated_value: i64,
    pub budget: Option<i64>,
    pub requirements: Option<String>,
    pub evaluation_criteria: Option<String>,
    pub terms_conditions: Option<String>,
    pub buyer_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
    pub is_public: bool,
    pub allow_reverse_auction: bool,
    pub min_bid_decrement: Option<i64>,
    pub auto_extend: bool,
    pub extension_minutes: Option<i32>,
    pub created_by: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SourcingEventType {
    RFI,
    RFQ,
    RFP,
    Auction,
    Tender,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcingItem {
    pub base: BaseEntity,
    pub event_id: Uuid,
    pub product_id: Option<Uuid>,
    pub sku: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub quantity: i32,
    pub unit_of_measure: String,
    pub target_price: Option<i64>,
    pub max_price: Option<i64>,
    pub specifications: Option<String>,
    pub delivery_date: Option<DateTime<Utc>>,
    pub delivery_location: Option<String>,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcingSupplier {
    pub base: BaseEntity,
    pub event_id: Uuid,
    pub vendor_id: Uuid,
    pub invited_at: DateTime<Utc>,
    pub accepted_at: Option<DateTime<Utc>>,
    pub declined_at: Option<DateTime<Utc>>,
    pub status: Status,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bid {
    pub base: BaseEntity,
    pub event_id: Uuid,
    pub vendor_id: Uuid,
    pub bid_number: String,
    pub status: BidStatus,
    pub submitted_at: Option<DateTime<Utc>>,
    pub valid_until: Option<DateTime<Utc>>,
    pub total_amount: i64,
    pub currency: String,
    pub terms: Option<String>,
    pub notes: Option<String>,
    pub rank: Option<i32>,
    pub score: Option<f64>,
    pub is_winner: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidLine {
    pub base: BaseEntity,
    pub bid_id: Uuid,
    pub item_id: Uuid,
    pub unit_price: i64,
    pub quantity: i32,
    pub total_price: i64,
    pub delivery_date: Option<DateTime<Utc>>,
    pub lead_time_days: Option<i32>,
    pub specifications_met: bool,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidAttachment {
    pub base: BaseEntity,
    pub bid_id: Uuid,
    pub file_name: String,
    pub file_path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub uploaded_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionRound {
    pub base: BaseEntity,
    pub event_id: Uuid,
    pub round_number: i32,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub status: Status,
    pub min_bid: Option<i64>,
    pub max_bid: Option<i64>,
    pub bid_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuctionBid {
    pub base: BaseEntity,
    pub round_id: Uuid,
    pub vendor_id: Uuid,
    pub amount: i64,
    pub bid_time: DateTime<Utc>,
    pub rank: i32,
    pub is_winning: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationCriteria {
    pub base: BaseEntity,
    pub event_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub weight: f64,
    pub max_score: i32,
    pub evaluation_method: EvaluationMethod,
    pub sort_order: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EvaluationMethod {
    Score,
    PassFail,
    Weighted,
    Ranking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BidEvaluation {
    pub base: BaseEntity,
    pub bid_id: Uuid,
    pub criteria_id: Uuid,
    pub score: i32,
    pub comments: Option<String>,
    pub evaluated_by: Option<Uuid>,
    pub evaluated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcingAward {
    pub base: BaseEntity,
    pub event_id: Uuid,
    pub bid_id: Uuid,
    pub vendor_id: Uuid,
    pub item_id: Option<Uuid>,
    pub awarded_quantity: i32,
    pub awarded_price: i64,
    pub total_value: i64,
    pub currency: String,
    pub awarded_at: DateTime<Utc>,
    pub award_type: AwardType,
    pub purchase_order_id: Option<Uuid>,
    pub contract_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AwardType {
    Full,
    Partial,
    Split,
    Conditional,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourcingContract {
    pub base: BaseEntity,
    pub event_id: Option<Uuid>,
    pub vendor_id: Uuid,
    pub contract_number: String,
    pub title: String,
    pub description: Option<String>,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub total_value: i64,
    pub currency: String,
    pub terms: Option<String>,
    pub status: Status,
    pub renewal_type: Option<RenewalType>,
    pub auto_renew: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RenewalType {
    Automatic,
    Manual,
    Evergreen,
    None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierQualification {
    pub base: BaseEntity,
    pub vendor_id: Uuid,
    pub qualification_type: String,
    pub name: String,
    pub description: Option<String>,
    pub issued_by: Option<String>,
    pub issued_date: Option<DateTime<Utc>>,
    pub expiry_date: Option<DateTime<Utc>>,
    pub certificate_number: Option<String>,
    pub document_path: Option<String>,
    pub status: Status,
}
