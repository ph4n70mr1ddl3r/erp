use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ExamStatus {
    Scheduled,
    Completed,
    Cancelled,
    Overdue,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HealthResult {
    FitForWork,
    FitWithRestrictions,
    UnfitForWork,
    PendingResults,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MedicalExam {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub exam_type: String, // e.g., "Annual Physical", "Hearing Test", "Eye Exam"
    pub scheduled_date: NaiveDate,
    pub completed_date: Option<NaiveDate>,
    pub status: ExamStatus,
    pub result: Option<HealthResult>,
    pub notes: Option<String>,
    pub medical_professional: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Vaccination {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub vaccine_name: String,
    pub dose_number: u32,
    pub date_administered: NaiveDate,
    pub next_dose_due: Option<NaiveDate>,
    pub manufacturer: Option<String>,
    pub lot_number: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HealthSurveillance {
    pub id: Uuid,
    pub employee_id: Uuid,
    pub hazard_type: String, // e.g., "Noise", "Lead", "Asbestos", "Radiation"
    pub monitoring_frequency_months: u32,
    pub last_monitoring_date: Option<NaiveDate>,
    pub next_monitoring_due: NaiveDate,
    pub status: String, // e.g., "Active", "Suspended"
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl MedicalExam {
    pub fn new(employee_id: Uuid, exam_type: String, scheduled_date: NaiveDate) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            employee_id,
            exam_type,
            scheduled_date,
            completed_date: None,
            status: ExamStatus::Scheduled,
            result: None,
            notes: None,
            medical_professional: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn complete(&mut self, date: NaiveDate, result: HealthResult, professional: String) {
        self.completed_date = Some(date);
        self.result = Some(result);
        self.medical_professional = Some(professional);
        self.status = ExamStatus::Completed;
        self.updated_at = Utc::now();
    }
}

pub struct HealthService {
    // Implementation placeholder
}

impl HealthService {
    pub fn is_exam_overdue(&self, exam: &MedicalExam) -> bool {
        if exam.status == ExamStatus::Completed || exam.status == ExamStatus::Cancelled {
            return false;
        }
        let today = Utc::now().date_naive();
        exam.scheduled_date < today
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_medical_exam_completion() {
        let employee_id = Uuid::new_v4();
        let scheduled = NaiveDate::from_ymd_opt(2026, 3, 1).unwrap();
        let mut exam = MedicalExam::new(employee_id, "Standard Annual".to_string(), scheduled);
        
        assert_eq!(exam.status, ExamStatus::Scheduled);
        assert!(exam.result.is_none());

        let completed_date = NaiveDate::from_ymd_opt(2026, 3, 5).unwrap();
        exam.complete(completed_date, HealthResult::FitForWork, "Dr. Smith".to_string());

        assert_eq!(exam.status, ExamStatus::Completed);
        assert_eq!(exam.result, Some(HealthResult::FitForWork));
        assert_eq!(exam.medical_professional.as_deref(), Some("Dr. Smith"));
    }

    #[test]
    fn test_overdue_exam_detection() {
        let service = HealthService {};
        let employee_id = Uuid::new_v4();
        
        // Past date
        let past_date = NaiveDate::from_ymd_opt(2020, 1, 1).unwrap();
        let exam_past = MedicalExam::new(employee_id, "Past Exam".to_string(), past_date);
        assert!(service.is_exam_overdue(&exam_past));

        // Future date
        let future_date = NaiveDate::from_ymd_opt(2030, 1, 1).unwrap();
        let exam_future = MedicalExam::new(employee_id, "Future Exam".to_string(), future_date);
        assert!(!service.is_exam_overdue(&exam_future));
    }

    #[test]
    fn test_vaccination_creation() {
        let employee_id = Uuid::new_v4();
        let admin_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let vacc = Vaccination {
            id: Uuid::new_v4(),
            employee_id,
            vaccine_name: "Hepatitis B".to_string(),
            dose_number: 1,
            date_administered: admin_date,
            next_dose_due: Some(NaiveDate::from_ymd_opt(2026, 2, 1).unwrap()),
            manufacturer: Some("PharmaCorp".to_string()),
            lot_number: Some("LOT123".to_string()),
            notes: None,
            created_at: Utc::now(),
        };

        assert_eq!(vacc.vaccine_name, "Hepatitis B");
        assert_eq!(vacc.dose_number, 1);
    }
}
