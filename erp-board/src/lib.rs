use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BoardRole {
    Chairman,
    ViceChairman,
    Director,
    IndependentDirector,
    Secretary,
    Observer,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoardMember {
    pub id: Uuid,
    pub employee_id: Option<Uuid>,
    pub first_name: String,
    pub last_name: String,
    pub role: BoardRole,
    pub appointed_at: DateTime<Utc>,
    pub term_expires_at: Option<DateTime<Utc>>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MeetingStatus {
    Scheduled,
    Cancelled,
    InProgress,
    Adjourned,
    MinutesPending,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BoardMeeting {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub scheduled_at: DateTime<Utc>,
    pub location: String,
    pub status: MeetingStatus,
    pub agenda_items: Vec<AgendaItem>,
    pub resolutions: Vec<Resolution>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AgendaItem {
    pub id: Uuid,
    pub meeting_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub presenter_id: Option<Uuid>,
    pub duration_minutes: u32,
    pub sequence: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ResolutionStatus {
    Proposed,
    Passed,
    Failed,
    Tabled,
    Withdrawn,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Resolution {
    pub id: Uuid,
    pub meeting_id: Uuid,
    pub title: String,
    pub content: String,
    pub status: ResolutionStatus,
    pub votes_for: u32,
    pub votes_against: u32,
    pub abstentions: u32,
    pub created_at: DateTime<Utc>,
}

impl BoardMember {
    pub fn new(first_name: String, last_name: String, role: BoardRole) -> Self {
        Self {
            id: Uuid::new_v4(),
            employee_id: None,
            first_name,
            last_name,
            role,
            appointed_at: Utc::now(),
            term_expires_at: None,
            is_active: true,
        }
    }
}

impl BoardMeeting {
    pub fn new(title: String, scheduled_at: DateTime<Utc>, location: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            scheduled_at,
            location,
            status: MeetingStatus::Scheduled,
            agenda_items: Vec::new(),
            resolutions: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_agenda_item(&mut self, title: String, duration: u32) {
        let sequence = (self.agenda_items.len() + 1) as u32;
        self.agenda_items.push(AgendaItem {
            id: Uuid::new_v4(),
            meeting_id: self.id,
            title,
            description: None,
            presenter_id: None,
            duration_minutes: duration,
            sequence,
        });
        self.updated_at = Utc::now();
    }

    pub fn propose_resolution(&mut self, title: String, content: String) -> Uuid {
        let res = Resolution {
            id: Uuid::new_v4(),
            meeting_id: self.id,
            title,
            content,
            status: ResolutionStatus::Proposed,
            votes_for: 0,
            votes_against: 0,
            abstentions: 0,
            created_at: Utc::now(),
        };
        let id = res.id;
        self.resolutions.push(res);
        self.updated_at = Utc::now();
        id
    }

    pub fn record_vote(&mut self, resolution_id: Uuid, votes_for: u32, votes_against: u32, abstentions: u32) -> Result<(), &'static str> {
        if let Some(res) = self.resolutions.iter_mut().find(|r| r.id == resolution_id) {
            res.votes_for = votes_for;
            res.votes_against = votes_against;
            res.abstentions = abstentions;
            res.status = if votes_for > votes_against {
                ResolutionStatus::Passed
            } else {
                ResolutionStatus::Failed
            };
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Resolution not found")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;

    #[test]
    fn test_board_meeting_and_resolutions() {
        let now = Utc::now();
        let scheduled_at = now + Duration::days(7);
        let mut meeting = BoardMeeting::new(
            "Q1 Strategy Review".to_string(),
            scheduled_at,
            "Main Boardroom".to_string(),
        );

        assert_eq!(meeting.status, MeetingStatus::Scheduled);

        meeting.add_agenda_item("Financial Results Review".to_string(), 30);
        meeting.add_agenda_item("New Market Expansion".to_string(), 45);
        assert_eq!(meeting.agenda_items.len(), 2);

        let res_id = meeting.propose_resolution(
            "Approve Dividend".to_string(),
            "The board resolves to pay a dividend of $0.50 per share.".to_string(),
        );

        assert_eq!(meeting.resolutions.len(), 1);
        assert_eq!(meeting.resolutions[0].status, ResolutionStatus::Proposed);

        // Record votes: 5 for, 0 against, 0 abstentions
        meeting.record_vote(res_id, 5, 0, 0).unwrap();
        assert_eq!(meeting.resolutions[0].status, ResolutionStatus::Passed);
    }

    #[test]
    fn test_board_member_creation() {
        let member = BoardMember::new(
            "Jane".to_string(),
            "Smith".to_string(),
            BoardRole::Chairman,
        );
        assert_eq!(member.role, BoardRole::Chairman);
        assert!(member.is_active);
    }
}
