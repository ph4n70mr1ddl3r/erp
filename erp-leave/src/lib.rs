use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LeaveType {
    AnnualLeave,
    SickLeave,
    MaternityLeave,
    PaternityLeave,
    UnpaidLeave,
    Bereavement,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LeaveStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeaveBalance {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub leave_type: LeaveType,
    pub total_allocated: f64,
    pub used_days: f64,
    pub pending_days: f64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LeaveRequest {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub leave_type: LeaveType,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub requested_days: f64,
    pub status: LeaveStatus,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl LeaveBalance {
    pub fn new(employee_id: Uuid, leave_type: LeaveType, total_allocated: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            employee_id,
            leave_type,
            total_allocated,
            used_days: 0.0,
            pending_days: 0.0,
            updated_at: Utc::now(),
        }
    }

    pub fn available_balance(&self) -> f64 {
        self.total_allocated - self.used_days - self.pending_days
    }
}

impl LeaveRequest {
    pub fn new(
        employee_id: Uuid,
        leave_type: LeaveType,
        start_date: NaiveDate,
        end_date: NaiveDate,
        requested_days: f64,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            employee_id,
            leave_type,
            start_date,
            end_date,
            requested_days,
            status: LeaveStatus::Pending,
            reason: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_reason(&mut self, reason: String) {
        self.reason = Some(reason);
        self.updated_at = Utc::now();
    }

    pub fn approve(&mut self, balance: &mut LeaveBalance) -> Result<(), &'static str> {
        if self.status != LeaveStatus::Pending {
            return Err("Only pending requests can be approved.");
        }
        if self.leave_type != balance.leave_type || self.employee_id != balance.employee_id {
            return Err("Mismatched leave balance provided.");
        }

        // Move days from pending to used
        balance.pending_days -= self.requested_days;
        if balance.pending_days < 0.0 {
            balance.pending_days = 0.0;
        }
        balance.used_days += self.requested_days;
        balance.updated_at = Utc::now();

        self.status = LeaveStatus::Approved;
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn reject(&mut self, balance: &mut LeaveBalance) -> Result<(), &'static str> {
        if self.status != LeaveStatus::Pending {
            return Err("Only pending requests can be rejected.");
        }
        if self.leave_type != balance.leave_type || self.employee_id != balance.employee_id {
            return Err("Mismatched leave balance provided.");
        }

        // Free up pending days
        balance.pending_days -= self.requested_days;
        if balance.pending_days < 0.0 {
            balance.pending_days = 0.0;
        }
        balance.updated_at = Utc::now();

        self.status = LeaveStatus::Rejected;
        self.updated_at = Utc::now();

        Ok(())
    }

    pub fn submit_request(&self, balance: &mut LeaveBalance) -> Result<(), &'static str> {
        if balance.available_balance() < self.requested_days {
            return Err("Insufficient leave balance.");
        }
        
        balance.pending_days += self.requested_days;
        balance.updated_at = Utc::now();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leave_request_lifecycle() {
        let employee_id = Uuid::new_v4();
        
        let mut balance = LeaveBalance::new(employee_id, LeaveType::AnnualLeave, 20.0);
        assert_eq!(balance.available_balance(), 20.0);

        let start_date = NaiveDate::from_ymd_opt(2026, 4, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2026, 4, 5).unwrap();
        let mut request = LeaveRequest::new(employee_id, LeaveType::AnnualLeave, start_date, end_date, 5.0);
        request.add_reason("Spring vacation".to_string());
        
        // Submit
        let submit_res = request.submit_request(&mut balance);
        assert!(submit_res.is_ok());
        assert_eq!(balance.pending_days, 5.0);
        assert_eq!(balance.available_balance(), 15.0);
        assert_eq!(balance.used_days, 0.0);

        // Approve
        let approve_res = request.approve(&mut balance);
        assert!(approve_res.is_ok());
        assert_eq!(request.status, LeaveStatus::Approved);
        assert_eq!(balance.pending_days, 0.0);
        assert_eq!(balance.used_days, 5.0);
        assert_eq!(balance.available_balance(), 15.0);
    }

    #[test]
    fn test_leave_rejection() {
        let employee_id = Uuid::new_v4();
        
        let mut balance = LeaveBalance::new(employee_id, LeaveType::SickLeave, 10.0);

        let start_date = NaiveDate::from_ymd_opt(2026, 5, 10).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2026, 5, 11).unwrap();
        let mut request = LeaveRequest::new(employee_id, LeaveType::SickLeave, start_date, end_date, 2.0);
        
        assert!(request.submit_request(&mut balance).is_ok());
        assert_eq!(balance.pending_days, 2.0);
        assert_eq!(balance.available_balance(), 8.0);

        // Reject
        assert!(request.reject(&mut balance).is_ok());
        assert_eq!(request.status, LeaveStatus::Rejected);
        assert_eq!(balance.pending_days, 0.0);
        assert_eq!(balance.used_days, 0.0);
        assert_eq!(balance.available_balance(), 10.0);
    }

    #[test]
    fn test_insufficient_balance() {
        let employee_id = Uuid::new_v4();
        let mut balance = LeaveBalance::new(employee_id, LeaveType::AnnualLeave, 3.0);

        let start_date = NaiveDate::from_ymd_opt(2026, 6, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2026, 6, 5).unwrap();
        let request = LeaveRequest::new(employee_id, LeaveType::AnnualLeave, start_date, end_date, 5.0);
        
        let res = request.submit_request(&mut balance);
        assert!(res.is_err());
        assert_eq!(res.unwrap_err(), "Insufficient leave balance.");
        assert_eq!(balance.pending_days, 0.0);
    }
}
