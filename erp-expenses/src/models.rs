use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ExpenseStatus {
    Draft,
    Submitted,
    PendingApproval,
    Approved,
    Rejected,
    Paid,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ExpenseType {
    Travel,
    Meals,
    Lodging,
    Transportation,
    OfficeSupplies,
    Equipment,
    Software,
    Training,
    Entertainment,
    ProfessionalServices,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorporateCard {
    pub base: BaseEntity,
    pub card_number_masked: String,
    pub cardholder_id: Uuid,
    pub card_type: CardType,
    pub issuer: String,
    pub credit_limit: i64,
    pub current_balance: i64,
    pub available_credit: i64,
    pub currency: String,
    pub issue_date: NaiveDate,
    pub expiry_date: NaiveDate,
    pub billing_day: i32,
    pub status: CardStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CardType {
    Corporate,
    Purchasing,
    Fleet,
    Travel,
    Virtual,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CardStatus {
    Active,
    Suspended,
    Cancelled,
    Expired,
    Lost,
    Stolen,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardTransaction {
    pub base: BaseEntity,
    pub card_id: Uuid,
    pub transaction_date: NaiveDate,
    pub posting_date: Option<NaiveDate>,
    pub merchant_name: String,
    pub merchant_category: Option<String>,
    pub merchant_category_code: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub original_amount: Option<i64>,
    pub original_currency: Option<String>,
    pub expense_type: Option<ExpenseType>,
    pub expense_report_id: Option<Uuid>,
    pub receipt_attached: bool,
    pub receipt_required: bool,
    pub verified: bool,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub status: CardTransactionStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CardTransactionStatus {
    Pending,
    Cleared,
    Disputed,
    Reconciled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseReport {
    pub base: BaseEntity,
    pub report_number: String,
    pub employee_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub purpose: Option<String>,
    pub project_id: Option<Uuid>,
    pub cost_center_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub total_amount: i64,
    pub currency: String,
    pub reimbursable_amount: i64,
    pub approved_amount: Option<i64>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub paid_at: Option<DateTime<Utc>>,
    pub payment_id: Option<Uuid>,
    pub status: ExpenseStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseLine {
    pub id: Uuid,
    pub expense_report_id: Uuid,
    pub line_number: i32,
    pub expense_date: NaiveDate,
    pub expense_type: ExpenseType,
    pub vendor: Option<String>,
    pub description: String,
    pub amount: i64,
    pub currency: String,
    pub exchange_rate: Option<f64>,
    pub base_currency_amount: i64,
    pub tax_amount: Option<i64>,
    pub tax_code: Option<String>,
    pub receipt_path: Option<String>,
    pub receipt_required: bool,
    pub receipt_verified: bool,
    pub card_transaction_id: Option<Uuid>,
    pub billable: bool,
    pub customer_id: Option<Uuid>,
    pub project_id: Option<Uuid>,
    pub cost_center_id: Option<Uuid>,
    pub gl_account_id: Option<Uuid>,
    pub approved_amount: Option<i64>,
    pub status: ExpenseStatus,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerDiemRate {
    pub base: BaseEntity,
    pub name: String,
    pub country_code: String,
    pub city: Option<String>,
    pub rate_type: PerDiemType,
    pub lodging_rate: i64,
    pub meals_rate: i64,
    pub incidentals_rate: i64,
    pub total_rate: i64,
    pub currency: String,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PerDiemType {
    Domestic,
    International,
    HighCost,
    Standard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MileageRate {
    pub base: BaseEntity,
    pub name: String,
    pub vehicle_type: VehicleType,
    pub rate_per_mile: i64,
    pub rate_per_km: i64,
    pub currency: String,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum VehicleType {
    Car,
    Motorcycle,
    Bicycle,
    Truck,
    Van,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MileageLog {
    pub base: BaseEntity,
    pub log_number: String,
    pub employee_id: Uuid,
    pub expense_report_id: Option<Uuid>,
    pub travel_date: NaiveDate,
    pub vehicle_type: VehicleType,
    pub starting_odometer: i64,
    pub ending_odometer: i64,
    pub total_miles: i64,
    pub total_km: i64,
    pub origin: String,
    pub destination: String,
    pub purpose: String,
    pub rate_per_mile: i64,
    pub currency: String,
    pub total_amount: i64,
    pub notes: Option<String>,
    pub status: ExpenseStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelRequest {
    pub base: BaseEntity,
    pub request_number: String,
    pub employee_id: Uuid,
    pub purpose: String,
    pub destination_city: String,
    pub destination_country: String,
    pub departure_date: NaiveDate,
    pub return_date: NaiveDate,
    pub estimated_airfare: i64,
    pub estimated_lodging: i64,
    pub estimated_meals: i64,
    pub estimated_transportation: i64,
    pub estimated_other: i64,
    pub total_estimated: i64,
    pub currency: String,
    pub advance_required: bool,
    pub advance_amount: Option<i64>,
    pub expense_report_id: Option<Uuid>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub status: TravelRequestStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TravelRequestStatus {
    Draft,
    Submitted,
    Approved,
    Rejected,
    Cancelled,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TravelBooking {
    pub base: BaseEntity,
    pub booking_number: String,
    pub travel_request_id: Uuid,
    pub booking_type: BookingType,
    pub provider: String,
    pub confirmation_number: Option<String>,
    pub departure_datetime: Option<DateTime<Utc>>,
    pub arrival_datetime: Option<DateTime<Utc>>,
    pub origin: Option<String>,
    pub destination: Option<String>,
    pub class_type: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub booked_by: Option<Uuid>,
    pub booked_at: Option<DateTime<Utc>>,
    pub status: BookingStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BookingType {
    Flight,
    Hotel,
    CarRental,
    Train,
    Bus,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BookingStatus {
    Pending,
    Confirmed,
    Cancelled,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpensePolicy {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub expense_type: ExpenseType,
    pub daily_limit: Option<i64>,
    pub per_transaction_limit: Option<i64>,
    pub monthly_limit: Option<i64>,
    pub annual_limit: Option<i64>,
    pub requires_receipt_above: Option<i64>,
    pub requires_preapproval_above: Option<i64>,
    pub currency: String,
    pub approval_workflow_id: Option<Uuid>,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseAllocation {
    pub id: Uuid,
    pub expense_line_id: Uuid,
    pub allocation_percent: f64,
    pub project_id: Option<Uuid>,
    pub cost_center_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub gl_account_id: Option<Uuid>,
    pub customer_id: Option<Uuid>,
    pub amount: i64,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseAuditLog {
    pub id: Uuid,
    pub expense_report_id: Uuid,
    pub action: String,
    pub old_status: Option<ExpenseStatus>,
    pub new_status: ExpenseStatus,
    pub performed_by: Uuid,
    pub performed_at: DateTime<Utc>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VendorPayment {
    pub base: BaseEntity,
    pub payment_number: String,
    pub vendor_id: Uuid,
    pub payment_date: NaiveDate,
    pub payment_method: PaymentMethodType,
    pub bank_account_id: Option<Uuid>,
    pub check_number: Option<String>,
    pub amount: i64,
    pub currency: String,
    pub status: VendorPaymentStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentMethodType {
    Check,
    ACH,
    Wire,
    CreditCard,
    Cash,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum VendorPaymentStatus {
    Scheduled,
    Pending,
    Completed,
    Cancelled,
    Voided,
}
