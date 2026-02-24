use axum::{
    extract::{Path, Query, State},
    Json,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::ApiResult;
use crate::state::AppState;

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    page: Option<i32>,
    page_size: Option<i32>,
    category: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    items: Vec<T>,
    total: i64,
    page: i32,
    page_size: i32,
}

pub async fn list_courses(
    State(state): State<AppState>,
    Query(query): Query<ListQuery>,
) -> ApiResult<Json<Vec<erp_lms::Course>>> {
    let category = query.category.and_then(|s| match s.as_str() {
        "Compliance" => Some(erp_lms::CourseCategory::Compliance),
        "Technical" => Some(erp_lms::CourseCategory::Technical),
        "Leadership" => Some(erp_lms::CourseCategory::Leadership),
        "Sales" => Some(erp_lms::CourseCategory::Sales),
        "Safety" => Some(erp_lms::CourseCategory::Safety),
        "Onboarding" => Some(erp_lms::CourseCategory::Onboarding),
        "ProfessionalDevelopment" => Some(erp_lms::CourseCategory::ProfessionalDevelopment),
        "ProductTraining" => Some(erp_lms::CourseCategory::ProductTraining),
        "SoftSkills" => Some(erp_lms::CourseCategory::SoftSkills),
        "ITSecurity" => Some(erp_lms::CourseCategory::ITSecurity),
        _ => None,
    });
    let courses = erp_lms::LMSService::new(erp_lms::SqliteLMSRepository::new(state.pool.clone()))
        .repo
        .list_courses(category, Some(erp_lms::CourseStatus::Published))
        .await?;
    Ok(Json(courses))
}

pub async fn create_course(
    State(state): State<AppState>,
    Json(req): Json<erp_lms::CreateCourseRequest>,
) -> ApiResult<Json<erp_lms::Course>> {
    let course = erp_lms::LMSService::new(erp_lms::SqliteLMSRepository::new(state.pool.clone()))
        .create_course(req)
        .await?;
    Ok(Json(course))
}

pub async fn get_course(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_lms::Course>> {
    let course = erp_lms::LMSService::new(erp_lms::SqliteLMSRepository::new(state.pool.clone()))
        .repo
        .get_course(id)
        .await?
        .ok_or_else(|| crate::error::ApiError::NotFound("Course not found".into()))?;
    Ok(Json(course))
}

pub async fn publish_course(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> ApiResult<Json<erp_lms::Course>> {
    let course = erp_lms::LMSService::new(erp_lms::SqliteLMSRepository::new(state.pool.clone()))
        .publish_course(id)
        .await?;
    Ok(Json(course))
}

pub async fn enroll(
    State(state): State<AppState>,
    Json(req): Json<erp_lms::EnrollRequest>,
) -> ApiResult<Json<erp_lms::Enrollment>> {
    let enrollment = erp_lms::LMSService::new(erp_lms::SqliteLMSRepository::new(state.pool.clone()))
        .enroll(req)
        .await?;
    Ok(Json(enrollment))
}

#[derive(Debug, Deserialize)]
pub struct UpdateProgressRequest {
    pub enrollment_id: Uuid,
    pub progress: f64,
}

pub async fn update_progress(
    State(state): State<AppState>,
    Json(req): Json<UpdateProgressRequest>,
) -> ApiResult<Json<erp_lms::Enrollment>> {
    let enrollment = erp_lms::LMSService::new(erp_lms::SqliteLMSRepository::new(state.pool.clone()))
        .update_progress(req.enrollment_id, req.progress)
        .await?;
    Ok(Json(enrollment))
}

#[derive(Debug, Deserialize)]
pub struct StartAssessmentRequest {
    pub assessment_id: Uuid,
    pub employee_id: Uuid,
}

pub async fn start_assessment(
    State(state): State<AppState>,
    Json(req): Json<StartAssessmentRequest>,
) -> ApiResult<Json<erp_lms::AssessmentAttempt>> {
    let attempt = erp_lms::LMSService::new(erp_lms::SqliteLMSRepository::new(state.pool.clone()))
        .start_assessment(req.assessment_id, req.employee_id)
        .await?;
    Ok(Json(attempt))
}

pub async fn submit_assessment(
    State(state): State<AppState>,
    Json(req): Json<erp_lms::SubmitAssessmentRequest>,
) -> ApiResult<Json<erp_lms::AssessmentAttempt>> {
    let attempt = erp_lms::LMSService::new(erp_lms::SqliteLMSRepository::new(state.pool.clone()))
        .submit_assessment(req)
        .await?;
    Ok(Json(attempt))
}

pub async fn get_stats(
    State(state): State<AppState>,
) -> ApiResult<Json<erp_lms::LearningStats>> {
    let stats = erp_lms::LMSService::new(erp_lms::SqliteLMSRepository::new(state.pool.clone()))
        .get_stats()
        .await?;
    Ok(Json(stats))
}

#[derive(Debug, Deserialize)]
pub struct CreateLearningPathRequest {
    pub name: String,
    pub description: Option<String>,
    pub target_role: Option<String>,
}

pub async fn create_learning_path(
    State(state): State<AppState>,
    Json(req): Json<CreateLearningPathRequest>,
) -> ApiResult<Json<erp_lms::LearningPath>> {
    let path = erp_lms::LMSService::new(erp_lms::SqliteLMSRepository::new(state.pool.clone()))
        .create_learning_path(req.name, req.description, req.target_role)
        .await?;
    Ok(Json(path))
}

#[derive(Debug, Deserialize)]
pub struct RecordTrainingRequest {
    pub employee_id: Uuid,
    pub training_type: String,
    pub training_name: String,
    pub hours: f64,
    pub credits: i32,
    pub cost: i64,
}

pub async fn record_training(
    State(state): State<AppState>,
    Json(req): Json<RecordTrainingRequest>,
) -> ApiResult<Json<erp_lms::TrainingRecord>> {
    let record = erp_lms::LMSService::new(erp_lms::SqliteLMSRepository::new(state.pool.clone()))
        .record_training(req.employee_id, req.training_type, req.training_name, req.hours, req.credits, req.cost)
        .await?;
    Ok(Json(record))
}

pub fn routes() -> axum::Router<crate::state::AppState> {
    axum::Router::new()
        .route("/courses", axum::routing::get(list_courses).post(create_course))
        .route("/courses/:id", axum::routing::get(get_course))
        .route("/courses/:id/publish", axum::routing::post(publish_course))
        .route("/enroll", axum::routing::post(enroll))
        .route("/progress", axum::routing::post(update_progress))
        .route("/assessments/start", axum::routing::post(start_assessment))
        .route("/assessments/submit", axum::routing::post(submit_assessment))
        .route("/stats", axum::routing::get(get_stats))
        .route("/learning-paths", axum::routing::post(create_learning_path))
        .route("/training-records", axum::routing::post(record_training))
}
