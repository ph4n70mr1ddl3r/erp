use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ABCClass {
    A, // High velocity (top 20% products, ~80% picks)
    B, // Medium velocity (next 30% products, ~15% picks)
    C, // Low velocity (remaining 50% products, ~5% picks)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductVelocity {
    pub product_id: Uuid,
    pub pick_count: u32,
    pub class: Option<ABCClass>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlottingLocation {
    pub location_id: Uuid,
    pub travel_distance_from_dock: u32,
    pub current_product_id: Option<Uuid>,
    pub zone: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SlottingRecommendation {
    pub product_id: Uuid,
    pub suggested_location_id: Uuid,
    pub reason: String,
}

pub struct SlottingEngine {
    velocities: Vec<ProductVelocity>,
    locations: Vec<SlottingLocation>,
}

impl SlottingEngine {
    pub fn new(velocities: Vec<ProductVelocity>, locations: Vec<SlottingLocation>) -> Self {
        Self {
            velocities,
            locations,
        }
    }

    /// Performs ABC analysis on products based on their pick counts.
    pub fn perform_abc_analysis(&mut self) {
        // Sort by pick count descending
        self.velocities.sort_by(|a, b| b.pick_count.cmp(&a.pick_count));

        let total_products = self.velocities.len();
        if total_products == 0 { return; }

        for (i, velocity) in self.velocities.iter_mut().enumerate() {
            let percentile = (i as f64 + 1.0) / total_products as f64;
            if percentile <= 0.20 {
                velocity.class = Some(ABCClass::A);
            } else if percentile <= 0.50 {
                velocity.class = Some(ABCClass::B);
            } else {
                velocity.class = Some(ABCClass::C);
            }
        }
    }

    /// Generates slotting recommendations by placing Class A items in locations closest to the dock.
    pub fn optimize(&mut self) -> Vec<SlottingRecommendation> {
        self.perform_abc_analysis();

        // Sort locations by distance ascending
        let mut sorted_locations = self.locations.clone();
        sorted_locations.sort_by(|a, b| a.travel_distance_from_dock.cmp(&b.travel_distance_from_dock));

        let mut recommendations = Vec::new();
        let mut loc_idx = 0;

        for product in &self.velocities {
            if loc_idx >= sorted_locations.len() {
                break;
            }

            let suggested_loc = &sorted_locations[loc_idx];
            
            // Recommend if the product isn't already there
            if suggested_loc.current_product_id != Some(product.product_id) {
                recommendations.push(SlottingRecommendation {
                    product_id: product.product_id,
                    suggested_location_id: suggested_loc.location_id,
                    reason: format!(
                        "Moving Class {:?} product to location at distance {}",
                        product.class.as_ref().unwrap(),
                        suggested_loc.travel_distance_from_dock
                    ),
                });
            }

            loc_idx += 1;
        }

        recommendations
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_abc_analysis() {
        let p1 = Uuid::new_v4();
        let p2 = Uuid::new_v4();
        let p3 = Uuid::new_v4();
        let p4 = Uuid::new_v4();
        let p5 = Uuid::new_v4();

        let velocities = vec![
            ProductVelocity { product_id: p1, pick_count: 100, class: None },
            ProductVelocity { product_id: p2, pick_count: 50, class: None },
            ProductVelocity { product_id: p3, pick_count: 20, class: None },
            ProductVelocity { product_id: p4, pick_count: 10, class: None },
            ProductVelocity { product_id: p5, pick_count: 5, class: None },
        ];

        let mut engine = SlottingEngine::new(velocities, vec![]);
        engine.perform_abc_analysis();

        // p1 (top 20%) should be A
        assert_eq!(engine.velocities[0].class, Some(ABCClass::A));
        // p2 (top 40%) should be B
        assert_eq!(engine.velocities[1].class, Some(ABCClass::B));
        // p5 (bottom) should be C
        assert_eq!(engine.velocities[4].class, Some(ABCClass::C));
    }

    #[test]
    fn test_slotting_optimization() {
        let p_fast = Uuid::new_v4();
        let p_slow = Uuid::new_v4();
        
        let l_near = Uuid::new_v4();
        let l_far = Uuid::new_v4();

        let velocities = vec![
            ProductVelocity { product_id: p_fast, pick_count: 1000, class: None },
            ProductVelocity { product_id: p_slow, pick_count: 1, class: None },
        ];

        let locations = vec![
            SlottingLocation { 
                location_id: l_far, 
                travel_distance_from_dock: 500, 
                current_product_id: Some(p_fast), // Mis-slotted!
                zone: "Z1".to_string() 
            },
            SlottingLocation { 
                location_id: l_near, 
                travel_distance_from_dock: 10, 
                current_product_id: None, 
                zone: "Z1".to_string() 
            },
        ];

        let mut engine = SlottingEngine::new(velocities, locations);
        let recs = engine.optimize();

        // Should recommend moving p_fast to l_near
        let fast_rec = recs.iter().find(|r| r.product_id == p_fast).unwrap();
        assert_eq!(fast_rec.suggested_location_id, l_near);
    }
}
