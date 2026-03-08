use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Readiness {
    Emergency, // Ready now for emergency cover
    ReadyNow,  // Ready for full promotion now
    OneToTwoYears,
    ThreeToFiveYears,
    LongTerm,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LossRisk {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Successor {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub employee_id: Uuid,
    pub readiness: Readiness,
    pub risk_of_loss: LossRisk,
    pub impact_of_loss: LossRisk,
    pub development_notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlanStatus {
    Draft,
    Active,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuccessionPlan {
    pub id: Uuid,
    pub position_id: Uuid,
    pub incumbent_id: Option<Uuid>,
    pub status: PlanStatus,
    pub successors: Vec<Successor>,
    pub last_review_date: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SuccessionPlan {
    pub fn new(position_id: Uuid, incumbent_id: Option<Uuid>) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            position_id,
            incumbent_id,
            status: PlanStatus::Draft,
            successors: Vec::new(),
            last_review_date: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_successor(&mut self, employee_id: Uuid, readiness: Readiness) {
        let now = Utc::now();
        self.successors.push(Successor {
            id: Uuid::new_v4(),
            plan_id: self.id,
            employee_id,
            readiness,
            risk_of_loss: LossRisk::Low,
            impact_of_loss: LossRisk::Medium,
            development_notes: None,
            created_at: now,
            updated_at: now,
        });
        self.updated_at = now;
    }

    pub fn activate(&mut self) {
        self.status = PlanStatus::Active;
        self.updated_at = Utc::now();
    }
}

pub struct SuccessionService {}

impl SuccessionService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn identify_emergency_successors(&self, plan: &SuccessionPlan) -> Vec<Uuid> {
        plan.successors.iter()
            .filter(|s| s.readiness == Readiness::Emergency || s.readiness == Readiness::ReadyNow)
            .map(|s| s.employee_id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_succession_plan_flow() {
        let position_id = Uuid::new_v4();
        let incumbent_id = Uuid::new_v4();
        let mut plan = SuccessionPlan::new(position_id, Some(incumbent_id));

        assert_eq!(plan.status, PlanStatus::Draft);
        assert_eq!(plan.successors.len(), 0);

        let successor_id = Uuid::new_v4();
        plan.add_successor(successor_id, Readiness::ReadyNow);
        assert_eq!(plan.successors.len(), 1);
        assert_eq!(plan.successors[0].readiness, Readiness::ReadyNow);

        plan.activate();
        assert_eq!(plan.status, PlanStatus::Active);

        let service = SuccessionService::new();
        let emergency_ids = service.identify_emergency_successors(&plan);
        assert_eq!(emergency_ids.len(), 1);
        assert_eq!(emergency_ids[0], successor_id);
    }

    #[test]
    fn test_multiple_successors() {
        let mut plan = SuccessionPlan::new(Uuid::new_v4(), None);
        plan.add_successor(Uuid::new_v4(), Readiness::LongTerm);
        plan.add_successor(Uuid::new_v4(), Readiness::Emergency);

        let service = SuccessionService::new();
        let emergency_ids = service.identify_emergency_successors(&plan);
        assert_eq!(emergency_ids.len(), 1);
    }
}
