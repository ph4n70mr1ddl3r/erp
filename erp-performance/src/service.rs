use chrono::NaiveDate;
use erp_core::{Error, Result, Status};
use sqlx::SqlitePool;
use uuid::Uuid;
use crate::models::*;

pub struct PerformanceService;

impl Default for PerformanceService {
    fn default() -> Self {
        Self::new()
    }
}

impl PerformanceService {
    pub fn new() -> Self {
        Self
    }

    pub async fn create_cycle(
        pool: &SqlitePool,
        name: &str,
        cycle_type: CycleType,
        start_date: &str,
        end_date: &str,
        review_due_date: &str,
    ) -> Result<PerformanceCycle> {
        if name.is_empty() {
            return Err(Error::validation("Name is required"));
        }
        let start_date = NaiveDate::parse_from_str(start_date, "%Y-%m-%d")
            .map_err(|_| Error::validation("Invalid start_date format"))?;
        let end_date = NaiveDate::parse_from_str(end_date, "%Y-%m-%d")
            .map_err(|_| Error::validation("Invalid end_date format"))?;
        let review_due_date = NaiveDate::parse_from_str(review_due_date, "%Y-%m-%d")
            .map_err(|_| Error::validation("Invalid review_due_date format"))?;
        let now = chrono::Utc::now();
        let cycle = PerformanceCycle {
            id: Uuid::new_v4(),
            name: name.to_string(),
            cycle_type,
            start_date,
            end_date,
            review_due_date,
            status: Status::Draft,
            created_at: now,
        };
        
        sqlx::query(
            "INSERT INTO performance_cycles (id, name, cycle_type, start_date, end_date, review_due_date, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, 'Draft', ?)"
        )
        .bind(cycle.id.to_string())
        .bind(&cycle.name)
        .bind(format!("{:?}", cycle.cycle_type))
        .bind(cycle.start_date.to_string())
        .bind(cycle.end_date.to_string())
        .bind(cycle.review_due_date.to_string())
        .bind(now.to_rfc3339())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(cycle)
    }

    pub async fn list_cycles(pool: &SqlitePool) -> Result<Vec<PerformanceCycle>> {
        let rows = sqlx::query_as::<_, PerformanceCycleRow>(
            "SELECT id, name, cycle_type, start_date, end_date, review_due_date, status, created_at FROM performance_cycles ORDER BY created_at DESC"
        )
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn get_cycle(pool: &SqlitePool, id: Uuid) -> Result<PerformanceCycle> {
        let row = sqlx::query_as::<_, PerformanceCycleRow>(
            "SELECT id, name, cycle_type, start_date, end_date, review_due_date, status, created_at FROM performance_cycles WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("PerformanceCycle", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn activate_cycle(pool: &SqlitePool, id: Uuid) -> Result<PerformanceCycle> {
        sqlx::query("UPDATE performance_cycles SET status = 'Active' WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        
        Self::get_cycle(pool, id).await
    }

    pub async fn close_cycle(pool: &SqlitePool, id: Uuid) -> Result<PerformanceCycle> {
        sqlx::query("UPDATE performance_cycles SET status = 'Closed' WHERE id = ?")
            .bind(id.to_string())
            .execute(pool)
            .await
            .map_err(Error::Database)?;
        
        Self::get_cycle(pool, id).await
    }

    pub async fn create_goal(
        pool: &SqlitePool,
        employee_id: Uuid,
        cycle_id: Uuid,
        title: &str,
        description: Option<&str>,
        weight: i32,
        target_value: Option<&str>,
    ) -> Result<PerformanceGoal> {
        let goal = PerformanceGoal {
            id: Uuid::new_v4(),
            employee_id,
            cycle_id,
            title: title.to_string(),
            description: description.map(|s| s.to_string()),
            weight,
            target_value: target_value.map(|s| s.to_string()),
            actual_value: None,
            self_rating: None,
            manager_rating: None,
            final_rating: None,
            status: Status::Draft,
        };
        
        sqlx::query(
            "INSERT INTO performance_goals (id, employee_id, cycle_id, title, description, weight, target_value, actual_value, self_rating, manager_rating, final_rating, status)
             VALUES (?, ?, ?, ?, ?, ?, ?, NULL, NULL, NULL, NULL, 'Draft')"
        )
        .bind(goal.id.to_string())
        .bind(goal.employee_id.to_string())
        .bind(goal.cycle_id.to_string())
        .bind(&goal.title)
        .bind(&goal.description)
        .bind(goal.weight)
        .bind(&goal.target_value)
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(goal)
    }

    pub async fn list_goals_by_cycle(pool: &SqlitePool, cycle_id: Uuid) -> Result<Vec<PerformanceGoal>> {
        let rows = sqlx::query_as::<_, PerformanceGoalRow>(
            "SELECT id, employee_id, cycle_id, title, description, weight, target_value, actual_value, self_rating, manager_rating, final_rating, status FROM performance_goals WHERE cycle_id = ?"
        )
        .bind(cycle_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn list_goals_by_employee(pool: &SqlitePool, employee_id: Uuid) -> Result<Vec<PerformanceGoal>> {
        let rows = sqlx::query_as::<_, PerformanceGoalRow>(
            "SELECT id, employee_id, cycle_id, title, description, weight, target_value, actual_value, self_rating, manager_rating, final_rating, status FROM performance_goals WHERE employee_id = ?"
        )
        .bind(employee_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn get_goal(pool: &SqlitePool, id: Uuid) -> Result<PerformanceGoal> {
        let row = sqlx::query_as::<_, PerformanceGoalRow>(
            "SELECT id, employee_id, cycle_id, title, description, weight, target_value, actual_value, self_rating, manager_rating, final_rating, status FROM performance_goals WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("PerformanceGoal", &id.to_string()))?;
        
        Ok(row.into())
    }

    pub async fn update_goal_rating(
        pool: &SqlitePool,
        id: Uuid,
        rating_type: &str,
        rating: i32,
        actual_value: Option<&str>,
    ) -> Result<PerformanceGoal> {
        let now = chrono::Utc::now();
        match rating_type {
            "self" => {
                sqlx::query("UPDATE performance_goals SET self_rating = ?, actual_value = ?, status = 'Pending', updated_at = ? WHERE id = ?")
                    .bind(rating)
                    .bind(actual_value)
                    .bind(now.to_rfc3339())
                    .bind(id.to_string())
                    .execute(pool)
                    .await
                    .map_err(Error::Database)?;
            }
            "manager" => {
                sqlx::query("UPDATE performance_goals SET manager_rating = ?, status = 'Approved', updated_at = ? WHERE id = ?")
                    .bind(rating)
                    .bind(now.to_rfc3339())
                    .bind(id.to_string())
                    .execute(pool)
                    .await
                    .map_err(Error::Database)?;
            }
            _ => return Err(Error::validation("Invalid rating type")),
        }
        
        Self::get_goal(pool, id).await
    }

    pub async fn create_review(
        pool: &SqlitePool,
        employee_id: Uuid,
        reviewer_id: Uuid,
        cycle_id: Uuid,
        review_type: ReviewType,
    ) -> Result<PerformanceReview> {
        let review = PerformanceReview {
            id: Uuid::new_v4(),
            employee_id,
            reviewer_id,
            cycle_id,
            review_type,
            overall_rating: None,
            strengths: None,
            areas_for_improvement: None,
            comments: None,
            submitted_at: None,
            status: Status::Draft,
        };
        
        sqlx::query(
            "INSERT INTO performance_reviews (id, employee_id, reviewer_id, cycle_id, review_type, overall_rating, strengths, areas_for_improvement, comments, submitted_at, status)
             VALUES (?, ?, ?, ?, ?, NULL, NULL, NULL, NULL, NULL, 'Draft')"
        )
        .bind(review.id.to_string())
        .bind(review.employee_id.to_string())
        .bind(review.reviewer_id.to_string())
        .bind(review.cycle_id.to_string())
        .bind(format!("{:?}", review.review_type))
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(review)
    }

    pub async fn submit_review(
        pool: &SqlitePool,
        id: Uuid,
        overall_rating: i32,
        strengths: Option<&str>,
        areas_for_improvement: Option<&str>,
        comments: Option<&str>,
    ) -> Result<PerformanceReview> {
        let now = chrono::Utc::now();
        
        sqlx::query(
            "UPDATE performance_reviews SET overall_rating = ?, strengths = ?, areas_for_improvement = ?, comments = ?, submitted_at = ?, status = 'Submitted' WHERE id = ?"
        )
        .bind(overall_rating)
        .bind(strengths)
        .bind(areas_for_improvement)
        .bind(comments)
        .bind(now.to_rfc3339())
        .bind(id.to_string())
        .execute(pool)
        .await
        .map_err(Error::Database)?;
        
        Self::get_review(pool, id).await
    }

    pub async fn list_reviews_by_cycle(pool: &SqlitePool, cycle_id: Uuid) -> Result<Vec<PerformanceReview>> {
        let rows = sqlx::query_as::<_, PerformanceReviewRow>(
            "SELECT id, employee_id, reviewer_id, cycle_id, review_type, overall_rating, strengths, areas_for_improvement, comments, submitted_at, status FROM performance_reviews WHERE cycle_id = ?"
        )
        .bind(cycle_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn list_reviews_by_employee(pool: &SqlitePool, employee_id: Uuid) -> Result<Vec<PerformanceReview>> {
        let rows = sqlx::query_as::<_, PerformanceReviewRow>(
            "SELECT id, employee_id, reviewer_id, cycle_id, review_type, overall_rating, strengths, areas_for_improvement, comments, submitted_at, status FROM performance_reviews WHERE employee_id = ?"
        )
        .bind(employee_id.to_string())
        .fetch_all(pool)
        .await
        .map_err(Error::Database)?;
        
        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn get_review(pool: &SqlitePool, id: Uuid) -> Result<PerformanceReview> {
        let row = sqlx::query_as::<_, PerformanceReviewRow>(
            "SELECT id, employee_id, reviewer_id, cycle_id, review_type, overall_rating, strengths, areas_for_improvement, comments, submitted_at, status FROM performance_reviews WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(Error::Database)?
        .ok_or_else(|| Error::not_found("PerformanceReview", &id.to_string()))?;
        
        Ok(row.into())
    }
}

#[derive(sqlx::FromRow)]
struct PerformanceCycleRow {
    id: String,
    name: String,
    cycle_type: String,
    start_date: String,
    end_date: String,
    review_due_date: String,
    status: String,
    created_at: String,
}


impl From<PerformanceCycleRow> for PerformanceCycle {
    fn from(r: PerformanceCycleRow) -> Self {
        Self {
            id: uuid::Uuid::parse_str(&r.id).unwrap_or_default(),
            name: r.name,
            cycle_type: match r.cycle_type.as_str() {
                "MidYear" => CycleType::MidYear,
                "Quarterly" => CycleType::Quarterly,
                _ => CycleType::Annual,
            },
            start_date: chrono::NaiveDate::parse_from_str(&r.start_date, "%Y-%m-%d").unwrap_or_default(),
            end_date: chrono::NaiveDate::parse_from_str(&r.end_date, "%Y-%m-%d").unwrap_or_default(),
            review_due_date: chrono::NaiveDate::parse_from_str(&r.review_due_date, "%Y-%m-%d").unwrap_or_default(),
            status: match r.status.as_str() {
                "Active" => Status::Active,
                "Closed" => Status::Completed,
                _ => Status::Draft,
            },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&chrono::Utc))
                .unwrap_or_else(|_| chrono::Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct PerformanceGoalRow {
    id: String,
    employee_id: String,
    cycle_id: String,
    title: String,
    description: Option<String>,
    weight: i32,
    target_value: Option<String>,
    actual_value: Option<String>,
    self_rating: Option<i32>,
    manager_rating: Option<i32>,
    final_rating: Option<i32>,
    status: String,
}

impl From<PerformanceGoalRow> for PerformanceGoal {
    fn from(r: PerformanceGoalRow) -> Self {
        Self {
            id: uuid::Uuid::parse_str(&r.id).unwrap_or_default(),
            employee_id: uuid::Uuid::parse_str(&r.employee_id).unwrap_or_default(),
            cycle_id: uuid::Uuid::parse_str(&r.cycle_id).unwrap_or_default(),
            title: r.title,
            description: r.description,
            weight: r.weight,
            target_value: r.target_value,
            actual_value: r.actual_value,
            self_rating: r.self_rating,
            manager_rating: r.manager_rating,
            final_rating: r.final_rating,
            status: match r.status.as_str() {
                "Pending" => Status::Pending,
                "Approved" => Status::Approved,
                _ => Status::Draft,
            },
        }
    }
}

#[derive(sqlx::FromRow)]
struct PerformanceReviewRow {
    id: String,
    employee_id: String,
    reviewer_id: String,
    cycle_id: String,
    review_type: String,
    overall_rating: Option<i32>,
    strengths: Option<String>,
    areas_for_improvement: Option<String>,
    comments: Option<String>,
    submitted_at: Option<String>,
    status: String,
}

impl From<PerformanceReviewRow> for PerformanceReview {
    fn from(r: PerformanceReviewRow) -> Self {
        Self {
            id: uuid::Uuid::parse_str(&r.id).unwrap_or_default(),
            employee_id: uuid::Uuid::parse_str(&r.employee_id).unwrap_or_default(),
            reviewer_id: uuid::Uuid::parse_str(&r.reviewer_id).unwrap_or_default(),
            cycle_id: uuid::Uuid::parse_str(&r.cycle_id).unwrap_or_default(),
            review_type: match r.review_type.as_str() {
                "ManagerReview" => ReviewType::ManagerReview,
                "PeerReview" => ReviewType::PeerReview,
                _ => ReviewType::SelfReview,
            },
            overall_rating: r.overall_rating,
            strengths: r.strengths,
            areas_for_improvement: r.areas_for_improvement,
            comments: r.comments,
            submitted_at: r.submitted_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&chrono::Utc)),
            status: match r.status.as_str() {
                "Submitted" => Status::Pending,
                "Approved" => Status::Approved,
                _ => Status::Draft,
            },
        }
    }
}
