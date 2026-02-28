use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{Address, BaseEntity, ContactInfo, Money, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Employee {
    pub base: BaseEntity,
    pub employee_number: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub contact: ContactInfo,
    pub address: Address,
    pub birth_date: NaiveDate,
    pub hire_date: NaiveDate,
    pub termination_date: Option<NaiveDate>,
    pub department_id: Option<Uuid>,
    pub position_id: Option<Uuid>,
    pub manager_id: Option<Uuid>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Department {
    pub base: BaseEntity,
    pub code: String,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub manager_id: Option<Uuid>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Position {
    pub base: BaseEntity,
    pub code: String,
    pub title: String,
    pub department_id: Option<Uuid>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attendance {
    pub base: BaseEntity,
    pub employee_id: Uuid,
    pub date: NaiveDate,
    pub check_in: Option<DateTime<Utc>>,
    pub check_out: Option<DateTime<Utc>>,
    pub work_hours: f64,
    pub overtime_hours: f64,
    pub status: AttendanceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AttendanceStatus {
    Present,
    Absent,
    Late,
    HalfDay,
    Leave,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveRequest {
    pub base: BaseEntity,
    pub employee_id: Uuid,
    pub leave_type: LeaveType,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days: f64,
    pub reason: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum LeaveType {
    Annual,
    Sick,
    Personal,
    Maternity,
    Paternity,
    Unpaid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Payroll {
    pub base: BaseEntity,
    pub employee_id: Uuid,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub base_salary: Money,
    pub overtime: Money,
    pub bonuses: Money,
    pub deductions: Money,
    pub net_salary: Money,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SalaryStructure {
    pub base: BaseEntity,
    pub employee_id: Uuid,
    pub base_salary: Money,
    pub allowances: Vec<Allowance>,
    pub effective_date: NaiveDate,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Allowance {
    pub id: Uuid,
    pub name: String,
    pub amount: Money,
    pub taxable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveTypeDef {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub days_per_year: i64,
    pub carry_over: bool,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveBalance {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub leave_type_id: Uuid,
    pub year: i32,
    pub entitled: i64,
    pub used: i64,
    pub remaining: i64,
    pub carried_over: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveRequestExtended {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub leave_type_id: Uuid,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub days: i64,
    pub reason: Option<String>,
    pub status: LeaveRequestStatus,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum LeaveRequestStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseCategory {
    pub id: Uuid,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseReport {
    pub id: Uuid,
    pub report_number: String,
    pub employee_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub total_amount: i64,
    pub status: ExpenseReportStatus,
    pub submitted_at: Option<DateTime<Utc>>,
    pub approved_by: Option<Uuid>,
    pub approved_at: Option<DateTime<Utc>>,
    pub rejected_at: Option<DateTime<Utc>>,
    pub rejection_reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ExpenseReportStatus {
    Draft,
    Submitted,
    Approved,
    Rejected,
    Paid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExpenseLine {
    pub id: Uuid,
    pub expense_report_id: Uuid,
    pub category_id: Uuid,
    pub expense_date: NaiveDate,
    pub description: String,
    pub amount: i64,
    pub currency: String,
    pub receipt_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayGrade {
    pub id: Uuid,
    pub grade_code: String,
    pub name: String,
    pub description: Option<String>,
    pub min_salary: i64,
    pub max_salary: i64,
    pub midpoint: Option<i64>,
    pub currency: String,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayComponent {
    pub id: Uuid,
    pub component_code: String,
    pub name: String,
    pub component_type: ComponentType,
    pub calculation_type: CalculationType,
    pub default_value: Option<i64>,
    pub taxable: bool,
    pub affects_gross: bool,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ComponentType {
    Earning,
    Deduction,
    Benefit,
    Tax,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CalculationType {
    Fixed,
    Percentage,
    Formula,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeSalary {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub pay_grade_id: Option<Uuid>,
    pub effective_date: NaiveDate,
    pub base_salary: i64,
    pub salary_type: SalaryType,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SalaryType {
    Annual,
    Monthly,
    Hourly,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeComponent {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub component_id: Uuid,
    pub value: i64,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollRun {
    pub id: Uuid,
    pub run_number: String,
    pub pay_period_start: NaiveDate,
    pub pay_period_end: NaiveDate,
    pub pay_date: NaiveDate,
    pub total_gross: i64,
    pub total_deductions: i64,
    pub total_net: i64,
    pub status: PayrollRunStatus,
    pub processed_at: Option<DateTime<Utc>>,
    pub approved_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PayrollRunStatus {
    Draft,
    Processing,
    Processed,
    Approved,
    Paid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollEntry {
    pub id: Uuid,
    pub payroll_run_id: Uuid,
    pub employee_id: Uuid,
    pub gross_pay: i64,
    pub total_deductions: i64,
    pub net_pay: i64,
    pub payment_method: PaymentMethod,
    pub bank_account: Option<String>,
    pub status: PayrollEntryStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PaymentMethod {
    BankTransfer,
    Check,
    Cash,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum PayrollEntryStatus {
    Pending,
    Paid,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PayrollLineItem {
    pub id: Uuid,
    pub payroll_entry_id: Uuid,
    pub component_id: Uuid,
    pub amount: i64,
    pub is_deduction: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaxTable {
    pub id: Uuid,
    pub tax_name: String,
    pub tax_type: TaxType,
    pub year: i32,
    pub bracket_min: i64,
    pub bracket_max: Option<i64>,
    pub rate: f64,
    pub base_amount: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TaxType {
    FederalIncome,
    StateIncome,
    SocialSecurity,
    Medicare,
    Unemployment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenefitPlan {
    pub id: Uuid,
    pub plan_code: String,
    pub name: String,
    pub plan_type: BenefitType,
    pub provider: Option<String>,
    pub coverage_type: Option<String>,
    pub employee_contribution: i64,
    pub employer_contribution: i64,
    pub max_dependents: i32,
    pub waiting_period_days: i32,
    pub effective_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum BenefitType {
    Medical,
    Dental,
    Vision,
    LifeInsurance,
    Disability,
    Retirement,
    Hsa,
    Fsa,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeBenefit {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub plan_id: Uuid,
    pub coverage_level: Option<String>,
    pub enrollment_date: NaiveDate,
    pub effective_date: NaiveDate,
    pub termination_date: Option<NaiveDate>,
    pub employee_cost: i64,
    pub employer_cost: i64,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenefitDependent {
    pub id: Uuid,
    pub employee_benefit_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub relationship: Relationship,
    pub birth_date: Option<NaiveDate>,
    pub gender: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum Relationship {
    Spouse,
    DomesticPartner,
    Child,
    Parent,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceCycle {
    pub id: Uuid,
    pub name: String,
    pub cycle_type: CycleType,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub review_due_date: NaiveDate,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CycleType {
    MidYear,
    Annual,
    Quarterly,
}

pub type PerformanceCycleType = CycleType;
pub type PerformanceCycleStatus = Status;
pub type PerformanceGoalStatus = Status;
pub type EmployeeTrainingStatus = TrainingStatus;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum JobPostingStatus {
    Draft,
    Published,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceGoal {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub cycle_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub weight: i32,
    pub target_value: Option<String>,
    pub actual_value: Option<String>,
    pub self_rating: Option<i32>,
    pub manager_rating: Option<i32>,
    pub final_rating: Option<i32>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReview {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub reviewer_id: Uuid,
    pub cycle_id: Uuid,
    pub review_type: ReviewType,
    pub overall_rating: Option<i32>,
    pub strengths: Option<String>,
    pub areas_for_improvement: Option<String>,
    pub comments: Option<String>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReviewType {
    SelfReview,
    ManagerReview,
    PeerReview,
    UpwardReview,
    ThreeSixty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompetencyRating {
    pub id: Uuid,
    pub review_id: Uuid,
    pub competency_name: String,
    pub rating: i32,
    pub comments: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingCourse {
    pub id: Uuid,
    pub course_code: String,
    pub title: String,
    pub description: Option<String>,
    pub category: Option<String>,
    pub duration_hours: Option<f64>,
    pub delivery_method: DeliveryMethod,
    pub provider: Option<String>,
    pub cost: i64,
    pub required_for: Option<String>,
    pub certification_valid_days: Option<i32>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DeliveryMethod {
    Online,
    InPerson,
    Hybrid,
    SelfPaced,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseModule {
    pub id: Uuid,
    pub course_id: Uuid,
    pub module_number: i32,
    pub title: String,
    pub content_type: ContentType,
    pub content_path: Option<String>,
    pub duration_minutes: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ContentType {
    Video,
    Document,
    Quiz,
    Assignment,
    Scorm,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmployeeTraining {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub course_id: Uuid,
    pub enrollment_date: NaiveDate,
    pub start_date: Option<NaiveDate>,
    pub completion_date: Option<NaiveDate>,
    pub due_date: Option<NaiveDate>,
    pub score: Option<f64>,
    pub passed: bool,
    pub certificate_number: Option<String>,
    pub certificate_expiry: Option<NaiveDate>,
    pub status: TrainingStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum TrainingStatus {
    Enrolled,
    InProgress,
    Completed,
    Failed,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingSession {
    pub id: Uuid,
    pub course_id: Uuid,
    pub session_date: NaiveDate,
    pub start_time: String,
    pub end_time: String,
    pub location: Option<String>,
    pub instructor: Option<String>,
    pub max_attendees: Option<i32>,
    pub current_attendees: i32,
    pub status: SessionStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum SessionStatus {
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobPosting {
    pub id: Uuid,
    pub job_code: String,
    pub title: String,
    pub department_id: Option<Uuid>,
    pub location: Option<String>,
    pub employment_type: EmploymentType,
    pub min_salary: Option<i64>,
    pub max_salary: Option<i64>,
    pub description: String,
    pub requirements: Option<String>,
    pub posted_date: Option<NaiveDate>,
    pub closing_date: Option<NaiveDate>,
    pub openings: i32,
    pub filled: i32,
    pub status: JobPostingStatus,
    pub hiring_manager: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EmploymentType {
    FullTime,
    PartTime,
    Contract,
    Temporary,
    Intern,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobApplication {
    pub id: Uuid,
    pub job_id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub resume_path: Option<String>,
    pub cover_letter: Option<String>,
    pub source: Option<String>,
    pub applied_at: DateTime<Utc>,
    pub status: ApplicationStatus,
    pub rating: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum ApplicationStatus {
    New,
    Screening,
    Interviewing,
    Offered,
    Hired,
    Rejected,
    Withdrawn,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApplicationStage {
    pub id: Uuid,
    pub application_id: Uuid,
    pub stage: ApplicationStageType,
    pub entered_at: DateTime<Utc>,
    pub exited_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ApplicationStageType {
    New,
    Screening,
    PhoneScreen,
    TechnicalInterview,
    OnsiteInterview,
    ReferenceCheck,
    BackgroundCheck,
    Offer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Interview {
    pub id: Uuid,
    pub application_id: Uuid,
    pub interview_type: InterviewType,
    pub scheduled_at: DateTime<Utc>,
    pub duration_minutes: i32,
    pub interviewer: String,
    pub location: Option<String>,
    pub notes: Option<String>,
    pub feedback: Option<String>,
    pub rating: Option<i32>,
    pub status: InterviewStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum InterviewType {
    Phone,
    Video,
    Onsite,
    Panel,
    Technical,
    Cultural,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum InterviewStatus {
    Scheduled,
    Completed,
    Cancelled,
    NoShow,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobOffer {
    pub id: Uuid,
    pub application_id: Uuid,
    pub offer_date: NaiveDate,
    pub salary: i64,
    pub start_date: NaiveDate,
    pub expiration_date: Option<NaiveDate>,
    pub terms: Option<String>,
    pub status: OfferStatus,
    pub responded_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "TEXT")]
pub enum OfferStatus {
    Pending,
    Accepted,
    Declined,
    Withdrawn,
    Expired,
}
