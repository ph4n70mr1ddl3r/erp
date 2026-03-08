use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RecognitionType {
    PeerToPeer,
    ManagerToDirect,
    CompanyWide,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Badge {
    pub id: Uuid,
    pub name: String,
    pub icon_url: Option<String>,
    pub points_value: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Recognition {
    pub id: Uuid,
    pub recognizer_id: Uuid, // Employee who gives recognition
    pub recipient_id: Uuid,  // Employee who receives recognition
    pub recognition_type: RecognitionType,
    pub message: String,
    pub badge_id: Option<Uuid>,
    pub points_awarded: u32,
    pub is_public: bool,
    pub created_at: DateTime<Utc>,
}

impl Recognition {
    pub fn new(
        recognizer_id: Uuid,
        recipient_id: Uuid,
        recognition_type: RecognitionType,
        message: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            recognizer_id,
            recipient_id,
            recognition_type,
            message,
            badge_id: None,
            points_awarded: 0,
            is_public: true,
            created_at: Utc::now(),
        }
    }

    pub fn with_badge(mut self, badge: &Badge) -> Self {
        self.badge_id = Some(badge.id);
        self.points_awarded = badge.points_value;
        self
    }

    pub fn set_private(&mut self) {
        self.is_public = false;
    }
}

pub struct RecognitionService {}

impl RecognitionService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate_total_points_received(&self, recognitions: &[Recognition], recipient_id: Uuid) -> u32 {
        recognitions.iter()
            .filter(|r| r.recipient_id == recipient_id)
            .map(|r| r.points_awarded)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recognition_creation() {
        let sender = Uuid::new_v4();
        let receiver = Uuid::new_v4();
        let mut rec = Recognition::new(
            sender,
            receiver,
            RecognitionType::PeerToPeer,
            "Great job on the presentation!".to_string(),
        );

        assert_eq!(rec.recognizer_id, sender);
        assert_eq!(rec.recipient_id, receiver);
        assert!(rec.is_public);

        rec.set_private();
        assert!(!rec.is_public);
    }

    #[test]
    fn test_recognition_with_badge() {
        let badge = Badge {
            id: Uuid::new_v4(),
            name: "Team Player".to_string(),
            icon_url: None,
            points_value: 50,
        };

        let rec = Recognition::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            RecognitionType::PeerToPeer,
            "Thanks for helping!".to_string(),
        ).with_badge(&badge);

        assert_eq!(rec.badge_id, Some(badge.id));
        assert_eq!(rec.points_awarded, 50);
    }

    #[test]
    fn test_point_calculation() {
        let service = RecognitionService::new();
        let recipient = Uuid::new_v4();
        
        let recognitions = vec![
            Recognition {
                id: Uuid::new_v4(),
                recognizer_id: Uuid::new_v4(),
                recipient_id: recipient,
                recognition_type: RecognitionType::PeerToPeer,
                message: "A".to_string(),
                badge_id: None,
                points_awarded: 10,
                is_public: true,
                created_at: Utc::now(),
            },
            Recognition {
                id: Uuid::new_v4(),
                recognizer_id: Uuid::new_v4(),
                recipient_id: recipient,
                recognition_type: RecognitionType::ManagerToDirect,
                message: "B".to_string(),
                badge_id: None,
                points_awarded: 100,
                is_public: true,
                created_at: Utc::now(),
            },
            Recognition {
                id: Uuid::new_v4(),
                recognizer_id: Uuid::new_v4(),
                recipient_id: Uuid::new_v4(), // Different recipient
                recognition_type: RecognitionType::PeerToPeer,
                message: "C".to_string(),
                badge_id: None,
                points_awarded: 50,
                is_public: true,
                created_at: Utc::now(),
            },
        ];

        assert_eq!(service.calculate_total_points_received(&recognitions, recipient), 110);
    }
}
