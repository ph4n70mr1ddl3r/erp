use chrono::{DateTime, NaiveDate, Utc};
use erp_core::Status;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CycleType {
    MidYear,
    Annual,
    Quarterly,
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

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReviewType {
    SelfReview,
    ManagerReview,
    PeerReview,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitReviewRequest {
    pub overall_rating: i32,
    pub strengths: Option<String>,
    pub areas_for_improvement: Option<String>,
    pub comments: Option<String>,
}
