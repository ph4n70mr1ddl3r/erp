use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WellnessCategory {
    Physical,
    Mental,
    Nutritional,
    Financial,
    Social,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProgramStatus {
    Active,
    Upcoming,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WellnessProgram {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub category: WellnessCategory,
    pub status: ProgramStatus,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub point_incentive: u32,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ActivityLog {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub program_id: Option<Uuid>,
    pub activity_type: String, // e.g., "Running", "Meditation", "Seminar"
    pub activity_date: NaiveDate,
    pub duration_minutes: u32,
    pub notes: Option<String>,
    pub points_earned: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct WellnessReward {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub points_redeemed: u32,
    pub reward_description: String,
    pub redeemed_at: DateTime<Utc>,
}

impl WellnessProgram {
    pub fn new(
        title: String,
        category: WellnessCategory,
        start_date: NaiveDate,
        end_date: NaiveDate,
        point_incentive: u32,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            title,
            description: String::new(),
            category,
            status: ProgramStatus::Upcoming,
            start_date,
            end_date,
            point_incentive,
            created_at: Utc::now(),
        }
    }

    pub fn activate(&mut self) {
        self.status = ProgramStatus::Active;
    }
}

pub struct WellnessService {}

impl WellnessService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate_total_points(&self, logs: &[ActivityLog]) -> u32 {
        logs.iter().map(|l| l.points_earned).sum()
    }

    pub fn can_redeem_reward(&self, total_points: u32, reward_cost: u32) -> bool {
        total_points >= reward_cost
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wellness_program_lifecycle() {
        let start = NaiveDate::from_ymd_opt(2026, 4, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2026, 4, 30).unwrap();
        let mut program = WellnessProgram::new(
            "Spring Step Challenge".to_string(),
            WellnessCategory::Physical,
            start,
            end,
            500,
        );

        assert_eq!(program.status, ProgramStatus::Upcoming);
        program.activate();
        assert_eq!(program.status, ProgramStatus::Active);
    }

    #[test]
    fn test_activity_points_and_redemption() {
        let service = WellnessService::new();
        let emp_id = Uuid::new_v4();
        
        let logs = vec![
            ActivityLog {
                id: Uuid::new_v4(),
                employee_id: emp_id,
                program_id: None,
                activity_type: "Meditation".to_string(),
                activity_date: Utc::now().date_naive(),
                duration_minutes: 20,
                notes: None,
                points_earned: 50,
            },
            ActivityLog {
                id: Uuid::new_v4(),
                employee_id: emp_id,
                program_id: None,
                activity_type: "Gym Session".to_string(),
                activity_date: Utc::now().date_naive(),
                duration_minutes: 60,
                notes: None,
                points_earned: 150,
            },
        ];

        let total = service.calculate_total_points(&logs);
        assert_eq!(total, 200);

        // Can redeem a 100 point reward
        assert!(service.can_redeem_reward(total, 100));
        // Cannot redeem a 500 point reward
        assert!(!service.can_redeem_reward(total, 500));
    }
}
