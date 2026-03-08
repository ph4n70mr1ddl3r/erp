use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum IdeaStatus {
    Draft,
    Submitted,
    UnderEvaluation,
    Approved,
    Rejected,
    ConvertedToProject,
    OnHold,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum InnovationCategory {
    Product,
    Process,
    BusinessModel,
    Sustainability,
    Service,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Idea {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub submitter_id: Uuid,
    pub category: InnovationCategory,
    pub status: IdeaStatus,
    pub evaluation_score: Option<f64>,
    pub projected_impact: Option<String>,
    pub estimated_cost: Option<f64>,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EvaluationMetric {
    pub criteria: String,
    pub score: u32, // 1-5
    pub weight: f64,
}

impl Idea {
    pub fn new(title: String, description: String, submitter_id: Uuid, category: InnovationCategory) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            submitter_id,
            category,
            status: IdeaStatus::Draft,
            evaluation_score: None,
            projected_impact: None,
            estimated_cost: None,
            tags: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn submit(&mut self) {
        if self.status == IdeaStatus::Draft {
            self.status = IdeaStatus::Submitted;
            self.updated_at = Utc::now();
        }
    }

    pub fn approve(&mut self) {
        self.status = IdeaStatus::Approved;
        self.updated_at = Utc::now();
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
            self.updated_at = Utc::now();
        }
    }
}

pub struct InnovationService {}

impl InnovationService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate_weighted_score(&self, metrics: &[EvaluationMetric]) -> f64 {
        if metrics.is_empty() {
            return 0.0;
        }
        let total_weight: f64 = metrics.iter().map(|m| m.weight).sum();
        let weighted_sum: f64 = metrics.iter().map(|m| m.score as f64 * m.weight).sum();
        
        if total_weight > 0.0 {
            weighted_sum / total_weight
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_idea_lifecycle() {
        let submitter_id = Uuid::new_v4();
        let mut idea = Idea::new(
            "AI-Driven Supply Chain".to_string(),
            "Using machine learning to optimize inventory levels.".to_string(),
            submitter_id,
            InnovationCategory::Process,
        );

        assert_eq!(idea.status, IdeaStatus::Draft);
        
        idea.add_tag("AI".to_string());
        idea.add_tag("Supply Chain".to_string());
        assert_eq!(idea.tags.len(), 2);

        idea.submit();
        assert_eq!(idea.status, IdeaStatus::Submitted);

        idea.approve();
        assert_eq!(idea.status, IdeaStatus::Approved);
    }

    #[test]
    fn test_evaluation_scoring() {
        let service = InnovationService::new();
        let metrics = vec![
            EvaluationMetric { criteria: "Impact".to_string(), score: 5, weight: 2.0 },
            EvaluationMetric { criteria: "Feasibility".to_string(), score: 3, weight: 1.0 },
        ];

        // Weighted score = (5*2 + 3*1) / (2+1) = 13 / 3 = 4.333...
        let score = service.calculate_weighted_score(&metrics);
        assert!((score - 4.333).abs() < 0.01);
    }
}
