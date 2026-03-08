use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OkrStatus {
    Draft,
    Active,
    Closed,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct KeyResult {
    pub id: Uuid,
    pub objective_id: Uuid,
    pub title: String,
    pub target_value: f64,
    pub current_value: f64,
    pub weight: f64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Objective {
    pub id: Uuid,
    pub owner_id: Uuid, // Employee or Team ID
    pub title: String,
    pub description: Option<String>,
    pub status: OkrStatus,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub key_results: Vec<KeyResult>,
    pub progress_percentage: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl KeyResult {
    pub fn new(objective_id: Uuid, title: String, target_value: f64, weight: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            objective_id,
            title,
            target_value,
            current_value: 0.0,
            weight,
            updated_at: Utc::now(),
        }
    }

    pub fn update_progress(&mut self, value: f64) {
        self.current_value = value.clamp(0.0, self.target_value);
        self.updated_at = Utc::now();
    }

    pub fn progress_percentage(&self) -> f64 {
        if self.target_value <= 0.0 {
            return 0.0;
        }
        (self.current_value / self.target_value) * 100.0
    }
}

impl Objective {
    pub fn new(
        owner_id: Uuid,
        title: String,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            owner_id,
            title,
            description: None,
            status: OkrStatus::Draft,
            start_date,
            end_date,
            key_results: Vec::new(),
            progress_percentage: 0.0,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn add_key_result(&mut self, kr: KeyResult) {
        self.key_results.push(kr);
        self.recalculate_progress();
    }

    pub fn update_key_result_progress(&mut self, kr_id: Uuid, new_value: f64) -> Result<(), &'static str> {
        if let Some(kr) = self.key_results.iter_mut().find(|kr| kr.id == kr_id) {
            kr.update_progress(new_value);
            self.recalculate_progress();
            Ok(())
        } else {
            Err("Key Result not found")
        }
    }

    fn recalculate_progress(&mut self) {
        let total_weight: f64 = self.key_results.iter().map(|kr| kr.weight).sum();
        if total_weight <= 0.0 || self.key_results.is_empty() {
            self.progress_percentage = 0.0;
        } else {
            let weighted_progress: f64 = self.key_results
                .iter()
                .map(|kr| kr.progress_percentage() * kr.weight)
                .sum();
            self.progress_percentage = weighted_progress / total_weight;
        }
        self.updated_at = Utc::now();
    }

    pub fn activate(&mut self) {
        if self.status == OkrStatus::Draft {
            self.status = OkrStatus::Active;
            self.updated_at = Utc::now();
        }
    }

    pub fn close(&mut self) {
        self.status = OkrStatus::Closed;
        self.updated_at = Utc::now();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_okr_lifecycle_and_progress() {
        let owner_id = Uuid::new_v4();
        let start_date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let end_date = NaiveDate::from_ymd_opt(2026, 3, 31).unwrap();
        
        let mut objective = Objective::new(owner_id, "Q1 Launch".to_string(), start_date, end_date);
        assert_eq!(objective.status, OkrStatus::Draft);
        assert_eq!(objective.progress_percentage, 0.0);

        let kr1 = KeyResult::new(objective.id, "Acquire 1000 users".to_string(), 1000.0, 1.0);
        let kr1_id = kr1.id;
        objective.add_key_result(kr1);

        let kr2 = KeyResult::new(objective.id, "Reduce latency to 100ms".to_string(), 100.0, 2.0); // Wait, this needs inverted logic generally, but we'll assume target_value is reached from 0. Let's make it simpler for test.
        let kr2_id = kr2.id;
        objective.add_key_result(kr2);
        
        objective.activate();
        assert_eq!(objective.status, OkrStatus::Active);

        // Update KR1: 500 / 1000 = 50%
        assert!(objective.update_key_result_progress(kr1_id, 500.0).is_ok());
        
        // Update KR2: 50 / 100 = 50%
        assert!(objective.update_key_result_progress(kr2_id, 50.0).is_ok());

        // Both are at 50%, so total should be 50% regardless of weights
        assert_eq!(objective.progress_percentage, 50.0);

        // Update KR1 to 100% (1000/1000)
        assert!(objective.update_key_result_progress(kr1_id, 1000.0).is_ok());
        
        // Total weight is 3. KR1 weight=1 (100%), KR2 weight=2 (50%).
        // Weighted sum = (100 * 1) + (50 * 2) = 200. Total weight = 3.
        // 200 / 3 = 66.666...%
        assert!((objective.progress_percentage - 66.666).abs() < 0.01);

        objective.close();
        assert_eq!(objective.status, OkrStatus::Closed);
    }
}
