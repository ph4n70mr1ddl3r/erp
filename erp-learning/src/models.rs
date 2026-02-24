use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ContentType {
    Video,
    Document,
    Presentation,
    Interactive,
    SCORM,
    Quiz,
    Survey,
    Assignment,
    ExternalLink,
    VirtualClassroom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningCourse {
    pub base: BaseEntity,
    pub course_code: String,
    pub title: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub instructor_id: Option<Uuid>,
    pub difficulty_level: DifficultyLevel,
    pub estimated_duration_minutes: i32,
    pub language: String,
    pub keywords: Option<String>,
    pub prerequisites: Option<String>,
    pub learning_objectives: Option<String>,
    pub passing_score_percent: i32,
    pub max_attempts: i32,
    pub certificate_template_id: Option<Uuid>,
    pub certificate_validity_days: Option<i32>,
    pub is_mandatory: bool,
    pub is_featured: bool,
    pub enrollment_type: EnrollmentType,
    pub price: i64,
    pub currency: String,
    pub thumbnail_url: Option<String>,
    pub status: CourseStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EnrollmentType {
    SelfDirected,
    Manager,
    Admin,
    Automatic,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CourseStatus {
    Draft,
    Published,
    Archived,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseSection {
    pub id: Uuid,
    pub course_id: Uuid,
    pub section_number: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseContent {
    pub id: Uuid,
    pub section_id: Uuid,
    pub course_id: Uuid,
    pub content_number: i32,
    pub title: String,
    pub description: Option<String>,
    pub content_type: ContentType,
    pub content_path: Option<String>,
    pub external_url: Option<String>,
    pub duration_minutes: Option<i32>,
    pub is_preview: bool,
    pub is_required: bool,
    pub scorm_identifier: Option<String>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPath {
    pub base: BaseEntity,
    pub path_code: String,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub total_courses: i32,
    pub estimated_duration_hours: i32,
    pub is_mandatory: bool,
    pub target_roles: Option<String>,
    pub target_departments: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPathCourse {
    pub id: Uuid,
    pub learning_path_id: Uuid,
    pub course_id: Uuid,
    pub sequence: i32,
    pub is_required: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EnrollmentStatus {
    Enrolled,
    InProgress,
    Completed,
    Failed,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseEnrollment {
    pub base: BaseEntity,
    pub employee_id: Uuid,
    pub course_id: Uuid,
    pub learning_path_id: Option<Uuid>,
    pub enrolled_by: Option<Uuid>,
    pub enrolled_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub due_date: Option<NaiveDate>,
    pub progress_percent: i32,
    pub time_spent_minutes: i32,
    pub score: Option<f64>,
    pub passed: Option<bool>,
    pub attempts: i32,
    pub status: EnrollmentStatus,
    pub certificate_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentProgress {
    pub id: Uuid,
    pub enrollment_id: Uuid,
    pub content_id: Uuid,
    pub status: ContentProgressStatus,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub time_spent_seconds: i32,
    pub score: Option<f64>,
    pub last_position: Option<String>,
    pub attempts: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ContentProgressStatus {
    NotStarted,
    InProgress,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assessment {
    pub base: BaseEntity,
    pub course_id: Option<Uuid>,
    pub assessment_type: AssessmentType,
    pub title: String,
    pub description: Option<String>,
    pub instructions: Option<String>,
    pub time_limit_minutes: Option<i32>,
    pub passing_score_percent: i32,
    pub max_attempts: i32,
    pub shuffle_questions: bool,
    pub shuffle_answers: bool,
    pub show_correct_answers: bool,
    pub show_score_immediately: bool,
    pub randomize_questions: bool,
    pub questions_per_attempt: Option<i32>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AssessmentType {
    Quiz,
    Exam,
    Survey,
    Practice,
    PreTest,
    PostTest,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum QuestionType {
    MultipleChoice,
    MultipleSelect,
    TrueFalse,
    FillBlank,
    ShortAnswer,
    Essay,
    Matching,
    Ordering,
    Hotspot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionBank {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: Uuid,
    pub question_bank_id: Option<Uuid>,
    pub assessment_id: Option<Uuid>,
    pub question_type: QuestionType,
    pub question_text: String,
    pub explanation: Option<String>,
    pub hint: Option<String>,
    pub points: i32,
    pub difficulty: DifficultyLevel,
    pub media_url: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnswerOption {
    pub id: Uuid,
    pub question_id: Uuid,
    pub option_text: String,
    pub is_correct: bool,
    pub feedback: Option<String>,
    pub sort_order: i32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentAttempt {
    pub base: BaseEntity,
    pub assessment_id: Uuid,
    pub enrollment_id: Option<Uuid>,
    pub employee_id: Uuid,
    pub attempt_number: i32,
    pub started_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub time_spent_seconds: Option<i32>,
    pub score: Option<f64>,
    pub passed: Option<bool>,
    pub status: AssessmentAttemptStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AssessmentAttemptStatus {
    InProgress,
    Submitted,
    Graded,
    Abandoned,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttemptAnswer {
    pub id: Uuid,
    pub attempt_id: Uuid,
    pub question_id: Uuid,
    pub selected_options: Option<String>,
    pub text_answer: Option<String>,
    pub is_correct: Option<bool>,
    pub points_earned: Option<i32>,
    pub feedback: Option<String>,
    pub answered_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    pub base: BaseEntity,
    pub certificate_number: String,
    pub employee_id: Uuid,
    pub course_id: Option<Uuid>,
    pub learning_path_id: Option<Uuid>,
    pub certificate_template_id: Uuid,
    pub issue_date: NaiveDate,
    pub expiry_date: Option<NaiveDate>,
    pub verification_code: String,
    pub pdf_path: Option<String>,
    pub status: CertificateStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CertificateStatus {
    Active,
    Expired,
    Revoked,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CertificateTemplate {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub template_html: Option<String>,
    pub background_image: Option<String>,
    pub width_mm: f64,
    pub height_mm: f64,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningCategory {
    pub base: BaseEntity,
    pub name: String,
    pub description: Option<String>,
    pub parent_id: Option<Uuid>,
    pub icon: Option<String>,
    pub sort_order: i32,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Instructor {
    pub base: BaseEntity,
    pub employee_id: Option<Uuid>,
    pub name: String,
    pub bio: Option<String>,
    pub expertise: Option<String>,
    pub email: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualSession {
    pub base: BaseEntity,
    pub course_id: Uuid,
    pub session_title: String,
    pub description: Option<String>,
    pub instructor_id: Option<Uuid>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub timezone: String,
    pub platform: VirtualPlatform,
    pub meeting_url: Option<String>,
    pub recording_url: Option<String>,
    pub max_attendees: Option<i32>,
    pub current_attendees: i32,
    pub status: VirtualSessionStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum VirtualPlatform {
    Zoom,
    Teams,
    WebEx,
    GoogleMeet,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum VirtualSessionStatus {
    Scheduled,
    InProgress,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionAttendance {
    pub id: Uuid,
    pub session_id: Uuid,
    pub employee_id: Uuid,
    pub joined_at: Option<DateTime<Utc>>,
    pub left_at: Option<DateTime<Utc>>,
    pub duration_minutes: Option<i32>,
    pub status: AttendanceStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AttendanceStatus {
    Registered,
    Attended,
    Partial,
    Absent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Competency {
    pub base: BaseEntity,
    pub name: String,
    pub code: String,
    pub description: Option<String>,
    pub category_id: Option<Uuid>,
    pub proficiency_levels: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillGapAnalysis {
    pub base: BaseEntity,
    pub employee_id: Uuid,
    pub analysis_date: NaiveDate,
    pub current_competencies: String,
    pub required_competencies: String,
    pub gaps: String,
    pub recommended_courses: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningMetric {
    pub id: Uuid,
    pub metric_date: NaiveDate,
    pub employee_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub courses_completed: i32,
    pub courses_in_progress: i32,
    pub total_learning_hours: f64,
    pub average_score: Option<f64>,
    pub certificates_earned: i32,
    pub mandatory_completion_rate: Option<f64>,
    pub created_at: DateTime<Utc>,
}
