use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProcessCriticality {
    Low,
    Medium,
    High,
    MissionCritical,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BusinessProcess {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub department_id: Uuid,
    pub criticality: ProcessCriticality,
    pub rto_hours: u32, // Recovery Time Objective
    pub rpo_hours: u32, // Recovery Point Objective
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PlanStatus {
    Draft,
    Active,
    Testing,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ContinuityPlan {
    pub id: Uuid,
    pub title: String,
    pub process_id: Uuid,
    pub status: PlanStatus,
    pub version: String,
    pub steps: Vec<RecoveryStep>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RecoveryStep {
    pub sequence: u32,
    pub action: String,
    pub responsibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TestResult {
    Success,
    PartialSuccess,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BcpTest {
    pub id: Uuid,
    pub plan_id: Uuid,
    pub test_date: DateTime<Utc>,
    pub actual_recovery_time_hours: u32,
    pub result: TestResult,
    pub findings: Option<String>,
}

impl ContinuityPlan {
    pub fn new(title: String, process_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            process_id,
            status: PlanStatus::Draft,
            version: "1.0".to_string(),
            steps: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_step(&mut self, action: String, responsibility: String) {
        let sequence = (self.steps.len() + 1) as u32;
        self.steps.push(RecoveryStep {
            sequence,
            action,
            responsibility,
        });
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        self.status = PlanStatus::Active;
        self.updated_at = Utc::now();
    }
}

pub struct BcpService {}

impl BcpService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn validate_rto_compliance(&self, process: &BusinessProcess, test: &BcpTest) -> bool {
        test.actual_recovery_time_hours <= process.rto_hours
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bcp_lifecycle() {
        let process_id = Uuid::new_v4();
        let mut plan = ContinuityPlan::new("Finance Recovery Plan".to_string(), process_id);
        
        assert_eq!(plan.status, PlanStatus::Draft);
        
        plan.add_step("Identify backup location".to_string(), "IT Manager".to_string());
        plan.add_step("Restore database from snapshot".to_string(), "DBA".to_string());
        
        assert_eq!(plan.steps.len(), 2);
        assert_eq!(plan.steps[1].sequence, 2);

        plan.activate();
        assert_eq!(plan.status, PlanStatus::Active);
    }

    #[test]
    fn test_rto_compliance_check() {
        let service = BcpService::new();
        let process = BusinessProcess {
            id: Uuid::new_v4(),
            name: "Payroll Processing".to_string(),
            description: "Bi-weekly payroll".to_string(),
            department_id: Uuid::new_v4(),
            criticality: ProcessCriticality::MissionCritical,
            rto_hours: 4,
            rpo_hours: 1,
        };

        let successful_test = BcpTest {
            id: Uuid::new_v4(),
            plan_id: Uuid::new_v4(),
            test_date: Utc::now(),
            actual_recovery_time_hours: 3,
            result: TestResult::Success,
            findings: None,
        };

        let failed_test = BcpTest {
            id: Uuid::new_v4(),
            plan_id: Uuid::new_v4(),
            test_date: Utc::now(),
            actual_recovery_time_hours: 6,
            result: TestResult::PartialSuccess,
            findings: Some("Server hardware delay".to_string()),
        };

        assert!(service.validate_rto_compliance(&process, &successful_test));
        assert!(!service.validate_rto_compliance(&process, &failed_test));
    }
}
