use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UnitStatus {
    Vacant,
    Occupied,
    Maintenance,
    Reserved,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PropertyType {
    Commercial,
    Residential,
    Industrial,
    Retail,
    Land,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Property {
    pub id: Uuid,
    pub name: String,
    pub property_type: PropertyType,
    pub address: String,
    pub total_area: f64,
    pub area_unit: String, // e.g., "sqft", "sqm"
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Building {
    pub id: Uuid,
    pub property_id: Uuid,
    pub name: String,
    pub total_floors: u32,
    pub construction_year: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RealEstateUnit {
    pub id: Uuid,
    pub building_id: Uuid,
    pub unit_number: String,
    pub floor_number: u32,
    pub area: f64,
    pub status: UnitStatus,
    pub monthly_rent: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Property {
    pub fn new(name: String, property_type: PropertyType, address: String, total_area: f64) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            property_type,
            address,
            total_area,
            area_unit: "sqft".to_string(),
            created_at: Utc::now(),
        }
    }
}

impl RealEstateUnit {
    pub fn new(building_id: Uuid, unit_number: String, floor_number: u32, area: f64) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            building_id,
            unit_number,
            floor_number,
            area,
            status: UnitStatus::Vacant,
            monthly_rent: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn set_status(&mut self, status: UnitStatus) {
        self.status = status;
        self.updated_at = Utc::now();
    }

    pub fn set_rent(&mut self, rent: f64) {
        self.monthly_rent = Some(rent);
        self.updated_at = Utc::now();
    }
}

pub struct RealEstateService {}

impl RealEstateService {
    pub fn new() -> Self {
        Self {}
    }

    pub fn calculate_occupancy_rate(&self, units: &[RealEstateUnit]) -> f64 {
        if units.is_empty() {
            return 0.0;
        }
        let occupied = units.iter().filter(|u| u.status == UnitStatus::Occupied).count();
        (occupied as f64 / units.len() as f64) * 100.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property_and_unit_creation() {
        let property = Property::new(
            "Tech Park".to_string(),
            PropertyType::Commercial,
            "123 Innovation Way".to_string(),
            50000.0,
        );
        assert_eq!(property.name, "Tech Park");

        let building_id = Uuid::new_v4();
        let mut unit = RealEstateUnit::new(building_id, "Suite 101".to_string(), 1, 1200.0);
        
        assert_eq!(unit.status, UnitStatus::Vacant);
        assert_eq!(unit.unit_number, "Suite 101");

        unit.set_status(UnitStatus::Occupied);
        assert_eq!(unit.status, UnitStatus::Occupied);
        
        unit.set_rent(2500.0);
        assert_eq!(unit.monthly_rent, Some(2500.0));
    }

    #[test]
    fn test_occupancy_calculation() {
        let service = RealEstateService::new();
        let b_id = Uuid::new_v4();
        
        let mut u1 = RealEstateUnit::new(b_id, "101".to_string(), 1, 1000.0);
        let mut u2 = RealEstateUnit::new(b_id, "102".to_string(), 1, 1000.0);
        let u3 = RealEstateUnit::new(b_id, "103".to_string(), 1, 1000.0);
        
        u1.set_status(UnitStatus::Occupied);
        u2.set_status(UnitStatus::Occupied);
        // u3 is Vacant
        
        let rate = service.calculate_occupancy_rate(&[u1, u2, u3]);
        assert!((rate - 66.66).abs() < 0.1);
    }
}
