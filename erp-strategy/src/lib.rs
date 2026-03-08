use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SwotCategory {
    Strength,
    Weakness,
    Opportunity,
    Threat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SwotItem {
    pub id: Uuid,
    pub analysis_id: Uuid,
    pub category: SwotCategory,
    pub title: String,
    pub description: Option<String>,
    pub importance: u32, // 1 to 5
    pub probability: Option<u32>, // 1 to 5 (mostly for O and T)
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnalysisStatus {
    Draft,
    Active,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SwotAnalysis {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: AnalysisStatus,
    pub owner_id: Uuid,
    pub department_id: Option<Uuid>,
    pub items: Vec<SwotItem>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl SwotAnalysis {
    pub fn new(title: String, owner_id: Uuid) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description: None,
            status: AnalysisStatus::Draft,
            owner_id,
            department_id: None,
            items: Vec::new(),
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_item(&mut self, category: SwotCategory, title: String, importance: u32) -> Uuid {
        let now = Utc::now();
        let item = SwotItem {
            id: Uuid::new_v4(),
            analysis_id: self.id,
            category,
            title,
            description: None,
            importance,
            probability: None,
            created_at: now,
            updated_at: now,
        };
        let id = item.id;
        self.items.push(item);
        self.updated_at = now;
        id
    }

    pub fn set_item_probability(&mut self, item_id: Uuid, probability: u32) -> Result<(), &'static str> {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == item_id) {
            item.probability = Some(probability);
            item.updated_at = Utc::now();
            self.updated_at = Utc::now();
            Ok(())
        } else {
            Err("Item not found")
        }
    }

    pub fn activate(&mut self) {
        self.status = AnalysisStatus::Active;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_swot_analysis_flow() {
        let owner_id = Uuid::new_v4();
        let mut analysis = SwotAnalysis::new("Q1 2026 Strategy".to_string(), owner_id);
        
        assert_eq!(analysis.status, AnalysisStatus::Draft);
        assert_eq!(analysis.items.len(), 0);

        analysis.add_item(SwotCategory::Strength, "High customer loyalty".to_string(), 5);
        let opp_id = analysis.add_item(SwotCategory::Opportunity, "Expansion into Southeast Asia".to_string(), 4);
        
        assert_eq!(analysis.items.len(), 2);
        assert_eq!(analysis.items[0].category, SwotCategory::Strength);
        
        analysis.set_item_probability(opp_id, 3).unwrap();
        assert_eq!(analysis.items[1].probability, Some(3));

        analysis.activate();
        assert_eq!(analysis.status, AnalysisStatus::Active);
    }
}
