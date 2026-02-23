use chrono::{DateTime, Utc};
use erp_core::{BaseEntity, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum POSStatus {
    Active,
    Inactive,
    Maintenance,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RegisterStatus {
    Open,
    Closed,
    Suspended,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentMethod {
    Cash,
    CreditCard,
    DebitCard,
    GiftCard,
    Check,
    MobilePayment,
    StoreCredit,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TransactionType {
    Sale,
    Return,
    Void,
    Refund,
    Layaway,
    SpecialOrder,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct POSStore {
    pub base: BaseEntity,
    pub store_code: String,
    pub name: String,
    pub address: String,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub manager_id: Option<Uuid>,
    pub warehouse_id: Option<Uuid>,
    pub status: POSStatus,
    pub opening_time: String,
    pub closing_time: String,
    pub timezone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct POSTerminal {
    pub base: BaseEntity,
    pub terminal_code: String,
    pub store_id: Uuid,
    pub name: String,
    pub location: Option<String>,
    pub status: POSStatus,
    pub printer_name: Option<String>,
    pub receipt_printer: Option<String>,
    pub cash_drawer: bool,
    pub customer_display: bool,
    pub barcode_scanner: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Register {
    pub base: BaseEntity,
    pub register_number: String,
    pub store_id: Uuid,
    pub terminal_id: Uuid,
    pub status: RegisterStatus,
    pub opened_at: Option<DateTime<Utc>>,
    pub opened_by: Option<Uuid>,
    pub closed_at: Option<DateTime<Utc>>,
    pub closed_by: Option<Uuid>,
    pub opening_float: Money,
    pub closing_float: Option<Money>,
    pub expected_cash: Option<Money>,
    pub cash_variance: Option<Money>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct POSTransaction {
    pub base: BaseEntity,
    pub transaction_number: String,
    pub store_id: Uuid,
    pub terminal_id: Uuid,
    pub register_id: Uuid,
    pub transaction_type: TransactionType,
    pub customer_id: Option<Uuid>,
    pub sales_rep_id: Option<Uuid>,
    pub lines: Vec<POSTransactionLine>,
    pub payments: Vec<POSTransactionPayment>,
    pub subtotal: Money,
    pub discount_amount: Money,
    pub tax_amount: Money,
    pub total: Money,
    pub change_amount: Money,
    pub status: Status,
    pub original_transaction_id: Option<Uuid>,
    pub notes: Option<String>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct POSTransactionLine {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub line_number: i32,
    pub product_id: Uuid,
    pub description: String,
    pub quantity: i64,
    pub unit_price: Money,
    pub discount_percent: f64,
    pub discount_amount: Money,
    pub tax_rate_id: Option<Uuid>,
    pub tax_amount: Money,
    pub line_total: Money,
    pub lot_number: Option<String>,
    pub serial_number: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct POSTransactionPayment {
    pub id: Uuid,
    pub transaction_id: Uuid,
    pub payment_method: PaymentMethod,
    pub amount: Money,
    pub reference: Option<String>,
    pub card_last_four: Option<String>,
    pub card_type: Option<String>,
    pub authorization_code: Option<String>,
    pub gift_card_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashDrawerSession {
    pub base: BaseEntity,
    pub register_id: Uuid,
    pub opened_at: DateTime<Utc>,
    pub opened_by: Uuid,
    pub closed_at: Option<DateTime<Utc>>,
    pub closed_by: Option<Uuid>,
    pub opening_amount: Money,
    pub closing_amount: Option<Money>,
    pub cash_sales: Money,
    pub cash_returns: Money,
    pub cash_paid_out: Money,
    pub cash_paid_in: Money,
    pub expected_amount: Option<Money>,
    pub variance: Option<Money>,
    pub status: RegisterStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CashMovement {
    pub id: Uuid,
    pub session_id: Uuid,
    pub movement_type: CashMovementType,
    pub amount: Money,
    pub reason: String,
    pub created_at: DateTime<Utc>,
    pub created_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CashMovementType {
    PayIn,
    PayOut,
    Drop,
    Loan,
    Pickup,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCard {
    pub base: BaseEntity,
    pub card_number: String,
    pub initial_amount: Money,
    pub current_balance: Money,
    pub sold_at: DateTime<Utc>,
    pub sold_at_store_id: Uuid,
    pub customer_id: Option<Uuid>,
    pub expires_at: Option<DateTime<Utc>>,
    pub status: GiftCardStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum GiftCardStatus {
    Active,
    Redeemed,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GiftCardTransaction {
    pub id: Uuid,
    pub gift_card_id: Uuid,
    pub transaction_id: Option<Uuid>,
    pub amount: Money,
    pub balance_after: Money,
    pub transaction_type: GiftCardTransactionType,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum GiftCardTransactionType {
    Issue,
    Reload,
    Redeem,
    Refund,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct POSShift {
    pub base: BaseEntity,
    pub store_id: Uuid,
    pub shift_name: String,
    pub start_time: String,
    pub end_time: String,
    pub break_minutes: i32,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyProgram {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub points_per_currency: f64,
    pub redemption_rate: f64,
    pub minimum_points: i64,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyAccount {
    pub base: BaseEntity,
    pub customer_id: Uuid,
    pub program_id: Uuid,
    pub points_balance: i64,
    pub tier: String,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoyaltyTransaction {
    pub id: Uuid,
    pub account_id: Uuid,
    pub points: i64,
    pub transaction_type: LoyaltyTransactionType,
    pub reference_type: Option<String>,
    pub reference_id: Option<Uuid>,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LoyaltyTransactionType {
    Earn,
    Redeem,
    Adjust,
    Expire,
}
