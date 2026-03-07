use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum GiftCardStatus {
    Active,
    Inactive,
    Redeemed,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum GiftCardType {
    Physical,
    Digital,
    ECode,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCard {
    pub base: BaseEntity,
    pub card_number: String,
    pub pin: Option<String>,
    pub barcode: Option<String>,
    pub gift_card_type: GiftCardType,
    pub initial_balance: i64,
    pub current_balance: i64,
    pub currency: String,
    pub customer_id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub purchased_by: Option<Uuid>,
    pub recipient_email: Option<String>,
    pub recipient_name: Option<String>,
    pub message: Option<String>,
    pub status: GiftCardStatus,
    pub issued_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub last_used_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardTransaction {
    pub base: BaseEntity,
    pub transaction_number: String,
    pub gift_card_id: Uuid,
    pub transaction_type: GiftCardTransactionType,
    pub amount: i64,
    pub balance_before: i64,
    pub balance_after: i64,
    pub order_id: Option<Uuid>,
    pub reference: Option<String>,
    pub notes: Option<String>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum GiftCardTransactionType {
    Issue,
    Reload,
    Redeem,
    Refund,
    Adjust,
    Expire,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardTemplate {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub initial_amount: i64,
    pub currency: String,
    pub is_reloadable: bool,
    pub validity_months: i32,
    pub design_url: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct CreateGiftCardRequest {
    pub gift_card_type: GiftCardType,
    pub initial_balance: i64,
    pub currency: Option<String>,
    pub customer_id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub purchased_by: Option<Uuid>,
    pub recipient_email: Option<String>,
    pub recipient_name: Option<String>,
    pub message: Option<String>,
    pub expiry_date: Option<NaiveDate>,
}

#[derive(Debug, Deserialize)]
pub struct RedeemGiftCardRequest {
    pub amount: i64,
    pub order_id: Option<Uuid>,
    pub reference: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReloadGiftCardRequest {
    pub amount: i64,
    pub order_id: Option<Uuid>,
    pub reference: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AdjustGiftCardRequest {
    pub amount: i64,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct CancelGiftCardRequest {
    pub reason: String,
}

#[derive(Debug, Serialize)]
pub struct GiftCardResponse {
    pub id: Uuid,
    pub card_number: String,
    pub gift_card_type: GiftCardType,
    pub initial_balance: i64,
    pub current_balance: i64,
    pub currency: String,
    pub status: GiftCardStatus,
    pub issued_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub recipient_name: Option<String>,
    pub recipient_email: Option<String>,
}

impl From<GiftCard> for GiftCardResponse {
    fn from(card: GiftCard) -> Self {
        Self {
            id: card.base.id,
            card_number: card.card_number,
            gift_card_type: card.gift_card_type,
            initial_balance: card.initial_balance,
            current_balance: card.current_balance,
            currency: card.currency,
            status: card.status,
            issued_date: card.issued_date,
            expiry_date: card.expiry_date,
            recipient_name: card.recipient_name,
            recipient_email: card.recipient_email,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct GiftCardTransactionResponse {
    pub id: Uuid,
    pub transaction_number: String,
    pub gift_card_id: Uuid,
    pub transaction_type: GiftCardTransactionType,
    pub amount: i64,
    pub balance_before: i64,
    pub balance_after: i64,
    pub created_at: DateTime<Utc>,
}

impl From<GiftCardTransaction> for GiftCardTransactionResponse {
    fn from(tx: GiftCardTransaction) -> Self {
        Self {
            id: tx.base.id,
            transaction_number: tx.transaction_number,
            gift_card_id: tx.gift_card_id,
            transaction_type: tx.transaction_type,
            amount: tx.amount,
            balance_before: tx.balance_before,
            balance_after: tx.balance_after,
            created_at: tx.created_at,
        }
    }
}
