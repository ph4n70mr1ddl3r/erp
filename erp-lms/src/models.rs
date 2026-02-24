use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub id: Uuid,
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub category: CourseCategory,
    pub difficulty: DifficultyLevel,
    pub duration_hours: f64,
    pub format: CourseFormat,
    pub instructor_id: Option<Uuid>,
    pub max_enrollments: Option<i32>,
    pub credits: i32,
    pub status: CourseStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CourseCategory {
    Compliance,
    Technical,
    Leadership,
    Sales,
    Safety,
    Onboarding,
    ProfessionalDevelopment,
    ProductTraining,
    SoftSkills,
    ITSecurity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DifficultyLevel {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CourseFormat {
    Online,
    InPerson,
    Hybrid,
    SelfPaced,
    VirtualLive,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CourseStatus {
    Draft,
    Published,
    Archived,
    Deprecated,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseModule {
    pub id: Uuid,
    pub course_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub sequence: i32,
    pub duration_minutes: i32,
    pub content_type: ContentType,
    pub content_url: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContentType {
    Video,
    Document,
    Quiz,
    Assignment,
    SCORM,
    Interactive,
    ExternalLink,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Enrollment {
    pub id: Uuid,
    pub course_id: Uuid,
    pub employee_id: Uuid,
    pub enrolled_at: DateTime<Utc>,
    pub status: EnrollmentStatus,
    pub progress_percent: f64,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub due_date: Option<NaiveDate>,
    pub score: Option<f64>,
    pub certificate_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EnrollmentStatus {
    Enrolled,
    InProgress,
    Completed,
    Failed,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPath {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub target_role: Option<String>,
    pub courses: Vec<LearningPathCourse>,
    pub total_credits: i32,
    pub status: PathStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PathStatus {
    Active,
    Inactive,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningPathCourse {
    pub course_id: Uuid,
    pub sequence: i32,
    pub is_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Assessment {
    pub id: Uuid,
    pub course_id: Option<Uuid>,
    pub title: String,
    pub description: Option<String>,
    pub assessment_type: AssessmentType,
    pub time_limit_minutes: Option<i32>,
    pub passing_score: f64,
    pub max_attempts: i32,
    pub questions: Vec<Question>,
    pub status: AssessmentStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssessmentType {
    Quiz,
    Exam,
    Survey,
    Certification,
    SkillAssessment,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AssessmentStatus {
    Draft,
    Active,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub id: Uuid,
    pub assessment_id: Uuid,
    pub question_text: String,
    pub question_type: QuestionType,
    pub options: Option<Vec<QuestionOption>>,
    pub correct_answer: String,
    pub points: i32,
    pub sequence: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QuestionOption {
    pub label: String,
    pub value: String,
    pub is_correct: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuestionType {
    MultipleChoice,
    TrueFalse,
    ShortAnswer,
    Essay,
    Matching,
    FillBlank,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssessmentAttempt {
    pub id: Uuid,
    pub assessment_id: Uuid,
    pub employee_id: Uuid,
    pub attempt_number: i32,
    pub started_at: DateTime<Utc>,
    pub submitted_at: Option<DateTime<Utc>>,
    pub score: Option<f64>,
    pub passed: Option<bool>,
    pub answers: serde_json::Value,
    pub status: AttemptStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AttemptStatus {
    InProgress,
    Submitted,
    Graded,
    TimedOut,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Certificate {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub course_id: Uuid,
    pub certificate_number: String,
    pub issued_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub verification_code: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMatrix {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub skill_id: Uuid,
    pub proficiency_level: i32,
    pub assessed_at: DateTime<Utc>,
    pub assessed_by: Option<Uuid>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub id: Uuid,
    pub name: String,
    pub category: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingRecord {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub training_type: String,
    pub training_name: String,
    pub provider: Option<String>,
    pub completed_at: NaiveDate,
    pub hours: f64,
    pub credits: i32,
    pub certificate_number: Option<String>,
    pub cost: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCourseRequest {
    pub code: String,
    pub title: String,
    pub description: Option<String>,
    pub category: CourseCategory,
    pub difficulty: DifficultyLevel,
    pub duration_hours: f64,
    pub format: CourseFormat,
    pub instructor_id: Option<Uuid>,
    pub max_enrollments: Option<i32>,
    pub credits: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrollRequest {
    pub course_id: Uuid,
    pub employee_id: Uuid,
    pub due_date: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitAssessmentRequest {
    pub attempt_id: Uuid,
    pub answers: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LearningStats {
    pub total_courses: i64,
    pub total_enrollments: i64,
    pub completion_rate: f64,
    pub average_score: f64,
    pub total_certificates: i64,
    pub training_hours: f64,
}
