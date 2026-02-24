use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait LMSRepository: Send + Sync {
    async fn create_course(&self, _course: &Course) -> anyhow::Result<()> { Ok(()) }
    async fn get_course(&self, _id: Uuid) -> anyhow::Result<Option<Course>> { Ok(None) }
    async fn list_courses(&self, _category: Option<CourseCategory>, _status: Option<CourseStatus>) -> anyhow::Result<Vec<Course>> { Ok(vec![]) }
    async fn update_course(&self, _course: &Course) -> anyhow::Result<()> { Ok(()) }
    async fn create_module(&self, _module: &CourseModule) -> anyhow::Result<()> { Ok(()) }
    async fn list_modules(&self, _course_id: Uuid) -> anyhow::Result<Vec<CourseModule>> { Ok(vec![]) }
    async fn create_enrollment(&self, _enrollment: &Enrollment) -> anyhow::Result<()> { Ok(()) }
    async fn get_enrollment(&self, _id: Uuid) -> anyhow::Result<Option<Enrollment>> { Ok(None) }
    async fn list_enrollments(&self, _employee_id: Option<Uuid>, _course_id: Option<Uuid>) -> anyhow::Result<Vec<Enrollment>> { Ok(vec![]) }
    async fn update_enrollment_progress(&self, _id: Uuid, _progress: f64, _status: EnrollmentStatus) -> anyhow::Result<()> { Ok(()) }
    async fn create_learning_path(&self, _path: &LearningPath) -> anyhow::Result<()> { Ok(()) }
    async fn get_learning_path(&self, _id: Uuid) -> anyhow::Result<Option<LearningPath>> { Ok(None) }
    async fn list_learning_paths(&self) -> anyhow::Result<Vec<LearningPath>> { Ok(vec![]) }
    async fn create_assessment(&self, _assessment: &Assessment) -> anyhow::Result<()> { Ok(()) }
    async fn get_assessment(&self, _id: Uuid) -> anyhow::Result<Option<Assessment>> { Ok(None) }
    async fn list_assessments(&self, _course_id: Option<Uuid>) -> anyhow::Result<Vec<Assessment>> { Ok(vec![]) }
    async fn create_attempt(&self, _attempt: &AssessmentAttempt) -> anyhow::Result<()> { Ok(()) }
    async fn get_attempt(&self, _id: Uuid) -> anyhow::Result<Option<AssessmentAttempt>> { Ok(None) }
    async fn list_attempts(&self, _assessment_id: Uuid, _employee_id: Uuid) -> anyhow::Result<Vec<AssessmentAttempt>> { Ok(vec![]) }
    async fn update_attempt(&self, _id: Uuid, _score: f64, _passed: bool, _status: AttemptStatus) -> anyhow::Result<()> { Ok(()) }
    async fn create_certificate(&self, _cert: &Certificate) -> anyhow::Result<()> { Ok(()) }
    async fn get_certificate(&self, _id: Uuid) -> anyhow::Result<Option<Certificate>> { Ok(None) }
    async fn list_certificates(&self, _employee_id: Uuid) -> anyhow::Result<Vec<Certificate>> { Ok(vec![]) }
    async fn create_training_record(&self, _record: &TrainingRecord) -> anyhow::Result<()> { Ok(()) }
    async fn list_training_records(&self, _employee_id: Uuid) -> anyhow::Result<Vec<TrainingRecord>> { Ok(vec![]) }
}

pub struct SqliteLMSRepository {
    pub pool: SqlitePool,
}

impl SqliteLMSRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LMSRepository for SqliteLMSRepository {}
