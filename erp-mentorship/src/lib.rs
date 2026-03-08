use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProgramStatus {
    Draft,
    Active,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MentorshipProgram {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub status: ProgramStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PairStatus {
    Matched,
    Active,
    Completed,
    Terminated,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MentorshipPair {
    pub id: Uuid,
    pub program_id: Uuid,
    pub mentor_id: Uuid,
    pub mentee_id: Uuid,
    pub status: PairStatus,
    pub matching_date: DateTime<Utc>,
    pub completion_date: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GoalStatus {
    InProgress,
    Achieved,
    Aborted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MentorshipGoal {
    pub id: Uuid,
    pub pair_id: Uuid,
    pub title: String,
    pub description: String,
    pub status: GoalStatus,
    pub target_date: Option<NaiveDate>,
    pub achieved_at: Option<DateTime<Utc>>,
}

impl MentorshipProgram {
    pub fn new(title: String, start_date: NaiveDate, end_date: NaiveDate) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description: String::new(),
            start_date,
            end_date,
            status: ProgramStatus::Draft,
            created_at: Utc::now(),
        }
    }

    pub fn activate(&mut self) {
        self.status = ProgramStatus::Active;
    }
}

impl MentorshipPair {
    pub fn new(program_id: Uuid, mentor_id: Uuid, mentee_id: Uuid) -> Self {
        Self {
            id: Uuid::new_v4(),
            program_id,
            mentor_id,
            mentee_id,
            status: PairStatus::Matched,
            matching_date: Utc::now(),
            completion_date: None,
            notes: None,
        }
    }

    pub fn start(&mut self) {
        self.status = PairStatus::Active;
    }

    pub fn complete(&mut self) {
        self.status = PairStatus::Completed;
        self.completion_date = Some(Utc::now());
    }
}

impl MentorshipGoal {
    pub fn new(pair_id: Uuid, title: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            pair_id,
            title,
            description,
            status: GoalStatus::InProgress,
            target_date: None,
            achieved_at: None,
        }
    }

    pub fn mark_achieved(&mut self) {
        self.status = GoalStatus::Achieved;
        self.achieved_at = Some(Utc::now());
    }
}

pub struct MentorshipService {}

impl MentorshipService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate_program_progress(&self, goals: &[MentorshipGoal]) -> f64 {
        if goals.is_empty() {
            return 0.0;
        }
        let achieved = goals.iter().filter(|g| g.status == GoalStatus::Achieved).count();
        (achieved as f64 / goals.len() as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mentorship_lifecycle() {
        let program = MentorshipProgram::new(
            "Leadership 2026".to_string(),
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            NaiveDate::from_ymd_opt(2026, 12, 31).unwrap(),
        );

        let mentor_id = Uuid::new_v4();
        let mentee_id = Uuid::new_v4();
        let mut pair = MentorshipPair::new(program.id, mentor_id, mentee_id);

        assert_eq!(pair.status, PairStatus::Matched);

        pair.start();
        assert_eq!(pair.status, PairStatus::Active);

        let mut goal = MentorshipGoal::new(pair.id, "Public Speaking".to_string(), "Deliver a conference talk".to_string());
        assert_eq!(goal.status, GoalStatus::InProgress);

        goal.mark_achieved();
        assert_eq!(goal.status, GoalStatus::Achieved);
        assert!(goal.achieved_at.is_some());

        pair.complete();
        assert_eq!(pair.status, PairStatus::Completed);
        assert!(pair.completion_date.is_some());
    }

    #[test]
    fn test_progress_calculation() {
        let service = MentorshipService::new();
        let pair_id = Uuid::new_v4();
        
        let mut g1 = MentorshipGoal::new(pair_id, "Goal 1".to_string(), "Desc 1".to_string());
        let mut g2 = MentorshipGoal::new(pair_id, "Goal 2".to_string(), "Desc 2".to_string());
        let g3 = MentorshipGoal::new(pair_id, "Goal 3".to_string(), "Desc 3".to_string());

        g1.mark_achieved();
        g2.mark_achieved();
        // g3 is in progress

        let progress = service.calculate_program_progress(&[g1, g2, g3]);
        assert!((progress - 66.66).abs() < 0.1);
    }
}
