use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ApplicationStatus {
    Applied,
    Screening,
    Interviewing,
    Offered,
    Hired,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Candidate {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub resume_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JobApplication {
    pub id: Uuid,
    pub candidate_id: Uuid,
    pub job_posting_id: Uuid,
    pub status: ApplicationStatus,
    pub notes: Option<String>,
    pub applied_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Candidate {
    pub fn new(first_name: String, last_name: String, email: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            email,
            phone: None,
            resume_url: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_resume_url(&mut self, url: String) {
        self.resume_url = Some(url);
        self.updated_at = Utc::now();
    }
}

impl JobApplication {
    pub fn new(candidate_id: Uuid, job_posting_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            candidate_id,
            job_posting_id,
            status: ApplicationStatus::Applied,
            notes: None,
            applied_at: now,
            updated_at: now,
        }
    }

    pub fn update_status(&mut self, new_status: ApplicationStatus) {
        self.status = new_status;
        self.updated_at = Utc::now();
    }

    pub fn add_note(&mut self, note: String) {
        let mut new_notes = self.notes.clone().unwrap_or_default();
        if !new_notes.is_empty() {
            new_notes.push_str("\n");
        }
        new_notes.push_str(&note);
        self.notes = Some(new_notes);
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_candidate_creation_and_application() {
        let mut candidate = Candidate::new(
            "Alice".to_string(),
            "Smith".to_string(),
            "alice.smith@example.com".to_string(),
        );

        assert_eq!(candidate.first_name, "Alice");
        candidate.set_resume_url("https://example.com/resume.pdf".to_string());
        assert_eq!(candidate.resume_url.as_deref(), Some("https://example.com/resume.pdf"));

        let job_posting_id = Uuid::new_v4();
        let mut application = JobApplication::new(candidate.id, job_posting_id);

        assert_eq!(application.status, ApplicationStatus::Applied);
        assert_eq!(application.candidate_id, candidate.id);

        application.update_status(ApplicationStatus::Interviewing);
        assert_eq!(application.status, ApplicationStatus::Interviewing);

        application.add_note("Good technical skills.".to_string());
        assert_eq!(application.notes.as_deref(), Some("Good technical skills."));
        
        application.add_note("Cultural fit is great.".to_string());
        assert_eq!(application.notes.as_deref(), Some("Good technical skills.\nCultural fit is great."));
    }
}
