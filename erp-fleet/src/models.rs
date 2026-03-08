use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Vehicle {
    #[serde(flatten)]
    pub base: BaseEntity,
    pub make: String,
    pub model: String,
    pub year: i32,
    pub license_plate: String,
    pub status: VehicleStatus,
    pub mileage: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VehicleStatus {
    Available,
    InUse,
    Maintenance,
    Retired,
}

impl Default for VehicleStatus {
    fn default() -> Self {
        Self::Available
    }
}
