use chrono::{DateTime, Utc, NaiveDate};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScreeningStatus {
    Clear,
    PotentialMatch,
    Blocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RestrictedParty {
    pub id: Uuid,
    pub name: String,
    pub alias: Option<String>,
    pub list_source: String, // e.g., "OFAC SDN", "EU Consolidated"
    pub reason: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TradeScreeningResult {
    pub id: Uuid,
    pub entity_name: String,
    pub status: ScreeningStatus,
    pub match_count: usize,
    pub matched_parties: Vec<Uuid>,
    pub screened_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportClassification {
    pub id: Uuid,
    pub product_id: Uuid,
    pub eccn: String, // Export Control Classification Number
    pub hts_code: String, // Harmonized Tariff Schedule
    pub country_of_origin: String,
    pub status: String,
    pub effective_from: NaiveDate,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExportLicense {
    pub id: Uuid,
    pub license_number: String,
    pub license_type: String, // e.g., "General", "Specific"
    pub country_destination: String,
    pub start_date: NaiveDate,
    pub expiry_date: NaiveDate,
    pub max_value: Option<f64>,
    pub max_quantity: Option<f64>,
    pub used_value: f64,
    pub status: String,
}

impl ExportLicense {
    pub fn is_valid_for(&self, country: &str, date: NaiveDate) -> bool {
        self.status == "Active" 
            && self.country_destination == country 
            && date >= self.start_date 
            && date <= self.expiry_date
    }
}

pub struct TradeComplianceService {
    // In a real implementation, these would be in a DB
    pub blocked_parties: Vec<RestrictedParty>,
}

impl TradeComplianceService {
    pub fn new() -> Self {
        Self { blocked_parties: Vec::new() }
    }

    pub fn add_blocked_party(&mut self, party: RestrictedParty) {
        self.blocked_parties.push(party);
    }

    pub fn screen_entity(&self, name: &str) -> TradeScreeningResult {
        let matched_parties: Vec<Uuid> = self.blocked_parties.iter()
            .filter(|p| p.name.to_lowercase() == name.to_lowercase())
            .map(|p| p.id)
            .collect();

        let status = if matched_parties.is_empty() {
            ScreeningStatus::Clear
        } else {
            ScreeningStatus::Blocked
        };

        TradeScreeningResult {
            id: Uuid::new_v4(),
            entity_name: name.to_string(),
            status,
            match_count: matched_parties.len(),
            matched_parties,
            screened_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restricted_party_screening() {
        let mut service = TradeComplianceService::new();
        
        let bad_actor = RestrictedParty {
            id: Uuid::new_v4(),
            name: "Globex Evil Corp".to_string(),
            alias: Some("GEC".to_string()),
            list_source: "Denied List 1.0".to_string(),
            reason: "Sanctioned entity".to_string(),
            created_at: Utc::now(),
        };
        
        service.add_blocked_party(bad_actor);

        // Screen a clear entity
        let res_clear = service.screen_entity("Innocent Inc");
        assert_eq!(res_clear.status, ScreeningStatus::Clear);
        assert_eq!(res_clear.match_count, 0);

        // Screen a blocked entity
        let res_blocked = service.screen_entity("Globex Evil Corp");
        assert_eq!(res_blocked.status, ScreeningStatus::Blocked);
        assert_eq!(res_blocked.match_count, 1);
    }

    #[test]
    fn test_export_license_validity() {
        let start = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let expiry = NaiveDate::from_ymd_opt(2026, 12, 31).unwrap();
        
        let license = ExportLicense {
            id: Uuid::new_v4(),
            license_number: "LIC-12345".to_string(),
            license_type: "Specific".to_string(),
            country_destination: "FR".to_string(),
            start_date: start,
            expiry_date: expiry,
            max_value: None,
            max_quantity: None,
            used_value: 0.0,
            status: "Active".to_string(),
        };

        assert!(license.is_valid_for("FR", NaiveDate::from_ymd_opt(2026, 6, 1).unwrap()));
        assert!(!license.is_valid_for("DE", NaiveDate::from_ymd_opt(2026, 6, 1).unwrap())); // Wrong country
        assert!(!license.is_valid_for("FR", NaiveDate::from_ymd_opt(2025, 12, 31).unwrap())); // Before start
    }
}
