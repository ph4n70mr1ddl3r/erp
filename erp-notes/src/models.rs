use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NoteType {
    General,
    Phone,
    Email,
    Meeting,
    Task,
    FollowUp,
    Important,
}

impl std::fmt::Display for NoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NoteType::General => write!(f, "General"),
            NoteType::Phone => write!(f, "Phone"),
            NoteType::Email => write!(f, "Email"),
            NoteType::Meeting => write!(f, "Meeting"),
            NoteType::Task => write!(f, "Task"),
            NoteType::FollowUp => write!(f, "FollowUp"),
            NoteType::Important => write!(f, "Important"),
        }
    }
}

impl std::str::FromStr for NoteType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "general" => Ok(NoteType::General),
            "phone" => Ok(NoteType::Phone),
            "email" => Ok(NoteType::Email),
            "meeting" => Ok(NoteType::Meeting),
            "task" => Ok(NoteType::Task),
            "followup" | "follow_up" | "follow-up" => Ok(NoteType::FollowUp),
            "important" => Ok(NoteType::Important),
            _ => Ok(NoteType::General),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub base: BaseEntity,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub note_type: NoteType,
    pub title: Option<String>,
    pub content: String,
    pub is_private: bool,
    pub is_pinned: bool,
    pub reminder_at: Option<DateTime<Utc>>,
    pub reminded_at: Option<DateTime<Utc>>,
}

impl Note {
    pub fn new(
        entity_type: &str,
        entity_id: Uuid,
        note_type: NoteType,
        content: String,
        created_by: Option<Uuid>,
    ) -> Self {
        Self {
            base: BaseEntity {
                created_by,
                ..BaseEntity::new()
            },
            entity_type: entity_type.to_string(),
            entity_id,
            note_type,
            title: None,
            content,
            is_private: false,
            is_pinned: false,
            reminder_at: None,
            reminded_at: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNoteRequest {
    pub entity_type: String,
    pub entity_id: Uuid,
    pub note_type: Option<String>,
    pub title: Option<String>,
    pub content: String,
    pub is_private: Option<bool>,
    pub reminder_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateNoteRequest {
    pub note_type: Option<String>,
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_private: Option<bool>,
    pub is_pinned: Option<bool>,
    pub reminder_at: Option<DateTime<Utc>>,
}
