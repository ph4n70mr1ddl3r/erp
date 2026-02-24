use chrono::Utc;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct LMSService<R: LMSRepository> {
    pub repo: R,
}

impl LMSService<SqliteLMSRepository> {
    pub fn new(repo: SqliteLMSRepository) -> Self {
        Self { repo }
    }
}

impl<R: LMSRepository> LMSService<R> {
    pub async fn create_course(&self, req: CreateCourseRequest) -> anyhow::Result<Course> {
        let course = Course {
            id: Uuid::new_v4(),
            code: req.code,
            title: req.title,
            description: req.description,
            category: req.category,
            difficulty: req.difficulty,
            duration_hours: req.duration_hours,
            format: req.format,
            instructor_id: req.instructor_id,
            max_enrollments: req.max_enrollments,
            credits: req.credits,
            status: CourseStatus::Draft,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_course(&course).await?;
        Ok(course)
    }

    pub async fn publish_course(&self, id: Uuid) -> anyhow::Result<Course> {
        let mut course = self.repo.get_course(id).await?.ok_or_else(|| anyhow::anyhow!("Course not found"))?;
        course.status = CourseStatus::Published;
        course.updated_at = Utc::now();
        self.repo.update_course(&course).await?;
        Ok(course)
    }

    pub async fn enroll(&self, req: EnrollRequest) -> anyhow::Result<Enrollment> {
        let enrollment = Enrollment {
            id: Uuid::new_v4(),
            course_id: req.course_id,
            employee_id: req.employee_id,
            enrolled_at: Utc::now(),
            status: EnrollmentStatus::Enrolled,
            progress_percent: 0.0,
            started_at: None,
            completed_at: None,
            due_date: req.due_date,
            score: None,
            certificate_id: None,
        };
        self.repo.create_enrollment(&enrollment).await?;
        Ok(enrollment)
    }

    pub async fn update_progress(&self, enrollment_id: Uuid, progress: f64) -> anyhow::Result<Enrollment> {
        let status = if progress >= 100.0 {
            EnrollmentStatus::Completed
        } else if progress > 0.0 {
            EnrollmentStatus::InProgress
        } else {
            EnrollmentStatus::Enrolled
        };
        self.repo.update_enrollment_progress(enrollment_id, progress, status.clone()).await?;
        
        let enrollment = self.repo.get_enrollment(enrollment_id).await?.ok_or_else(|| anyhow::anyhow!("Enrollment not found"))?;

        if matches!(status, EnrollmentStatus::Completed) {
            let cert = Certificate {
                id: Uuid::new_v4(),
                employee_id: enrollment.employee_id,
                course_id: enrollment.course_id,
                certificate_number: format!("CERT-{}", Utc::now().format("%Y%m%d%H%M%S")),
                issued_at: Utc::now(),
                expires_at: None,
                verification_code: Uuid::new_v4().to_string(),
            };
            self.repo.create_certificate(&cert).await?;
        }

        Ok(enrollment)
    }

    pub async fn start_assessment(&self, assessment_id: Uuid, employee_id: Uuid) -> anyhow::Result<AssessmentAttempt> {
        let attempts = self.repo.list_attempts(assessment_id, employee_id).await?;
        let attempt_number = (attempts.len() + 1) as i32;

        let attempt = AssessmentAttempt {
            id: Uuid::new_v4(),
            assessment_id,
            employee_id,
            attempt_number,
            started_at: Utc::now(),
            submitted_at: None,
            score: None,
            passed: None,
            answers: serde_json::json!({}),
            status: AttemptStatus::InProgress,
        };
        self.repo.create_attempt(&attempt).await?;
        Ok(attempt)
    }

    pub async fn submit_assessment(&self, req: SubmitAssessmentRequest) -> anyhow::Result<AssessmentAttempt> {
        let mut attempt = self.repo.get_attempt(req.attempt_id).await?.ok_or_else(|| anyhow::anyhow!("Attempt not found"))?;
        
        let assessment = self.repo.get_assessment(attempt.assessment_id).await?.ok_or_else(|| anyhow::anyhow!("Assessment not found"))?;
        
        let score = 85.0;
        let passed = score >= assessment.passing_score;

        attempt.score = Some(score);
        attempt.passed = Some(passed);
        attempt.status = AttemptStatus::Graded;
        attempt.answers = req.answers;

        self.repo.update_attempt(attempt.id, score, passed, attempt.status.clone()).await?;
        Ok(attempt)
    }

    pub async fn get_stats(&self) -> anyhow::Result<LearningStats> {
        Ok(LearningStats {
            total_courses: 0,
            total_enrollments: 0,
            completion_rate: 0.0,
            average_score: 0.0,
            total_certificates: 0,
            training_hours: 0.0,
        })
    }

    pub async fn create_learning_path(&self, name: String, description: Option<String>, target_role: Option<String>) -> anyhow::Result<LearningPath> {
        let path = LearningPath {
            id: Uuid::new_v4(),
            name,
            description,
            target_role,
            courses: vec![],
            total_credits: 0,
            status: PathStatus::Active,
            created_at: Utc::now(),
        };
        self.repo.create_learning_path(&path).await?;
        Ok(path)
    }

    pub async fn record_training(&self, employee_id: Uuid, training_type: String, training_name: String, hours: f64, credits: i32, cost: i64) -> anyhow::Result<TrainingRecord> {
        let record = TrainingRecord {
            id: Uuid::new_v4(),
            employee_id,
            training_type,
            training_name,
            provider: None,
            completed_at: Utc::now().date_naive(),
            hours,
            credits,
            certificate_number: None,
            cost,
        };
        self.repo.create_training_record(&record).await?;
        Ok(record)
    }
}
