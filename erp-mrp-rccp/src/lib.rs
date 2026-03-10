use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapacityRequirement {
    pub resource_id: Uuid,
    pub period_start: DateTime<Utc>,
    pub required_hours: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceCapacity {
    pub resource_id: Uuid,
    pub name: String,
    pub available_hours_per_period: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CapacityLoadReport {
    pub resource_id: Uuid,
    pub resource_name: String,
    pub period_start: DateTime<Utc>,
    pub required_hours: f64,
    pub available_hours: f64,
    pub load_percent: f64,
    pub status: LoadStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LoadStatus {
    Underloaded,
    Optimal,
    Overloaded,
}

pub struct RccpEngine {
    resources: Vec<ResourceCapacity>,
    requirements: Vec<CapacityRequirement>,
}

impl RccpEngine {
    pub fn new() -> Self {
        Self {
            resources: Vec::new(),
            requirements: Vec::new(),
        }
    }

    pub fn add_resource(&mut self, resource: ResourceCapacity) {
        self.resources.push(resource);
    }

    pub fn add_requirement(&mut self, requirement: CapacityRequirement) {
        self.requirements.push(requirement);
    }

    pub fn generate_report(&self) -> Vec<CapacityLoadReport> {
        let mut reports = Vec::new();

        for resource in &self.resources {
            // Group requirements by period for this resource
            let mut period_map: std::collections::HashMap<DateTime<Utc>, f64> = std::collections::HashMap::new();
            
            for req in &self.requirements {
                if req.resource_id == resource.resource_id {
                    *period_map.entry(req.period_start).or_insert(0.0) += req.required_hours;
                }
            }

            for (period, total_required) in period_map {
                let load_percent = (total_required / resource.available_hours_per_period) * 100.0;
                let status = if load_percent > 100.0 {
                    LoadStatus::Overloaded
                } else if load_percent > 85.0 {
                    LoadStatus::Optimal
                } else {
                    LoadStatus::Underloaded
                };

                reports.push(CapacityLoadReport {
                    resource_id: resource.resource_id,
                    resource_name: resource.name.clone(),
                    period_start: period,
                    required_hours: total_required,
                    available_hours: resource.available_hours_per_period,
                    load_percent,
                    status,
                });
            }
        }

        reports
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_rccp_overload_detection() {
        let mut engine = RccpEngine::new();
        let resource_id = Uuid::new_v4();
        let period = Utc.with_ymd_and_hms(2026, 4, 1, 0, 0, 0).unwrap();

        engine.add_resource(ResourceCapacity {
            resource_id,
            name: "Main Assembly Line".to_string(),
            available_hours_per_period: 40.0,
        });

        // Add multiple requirements that exceed capacity
        engine.add_requirement(CapacityRequirement {
            resource_id,
            period_start: period,
            required_hours: 25.0,
        });
        engine.add_requirement(CapacityRequirement {
            resource_id,
            period_start: period,
            required_hours: 20.0,
        });

        let reports = engine.generate_report();
        assert_eq!(reports.len(), 1);
        let report = &reports[0];
        
        assert_eq!(report.required_hours, 45.0);
        assert_eq!(report.load_percent, 112.5);
        assert_eq!(report.status, LoadStatus::Overloaded);
    }

    #[test]
    fn test_rccp_optimal_load() {
        let mut engine = RccpEngine::new();
        let resource_id = Uuid::new_v4();
        let period = Utc.with_ymd_and_hms(2026, 4, 1, 0, 0, 0).unwrap();

        engine.add_resource(ResourceCapacity {
            resource_id,
            name: "CNC Machine".to_string(),
            available_hours_per_period: 100.0,
        });

        engine.add_requirement(CapacityRequirement {
            resource_id,
            period_start: period,
            required_hours: 90.0,
        });

        let reports = engine.generate_report();
        let report = &reports[0];
        assert_eq!(report.status, LoadStatus::Optimal);
    }
}
