use anyhow::Result;
use uuid::Uuid;
use std::sync::Arc;
use crate::models::{Vehicle, VehicleStatus};
use crate::repository::VehicleRepository;

pub struct VehicleService {
    repo: Arc<dyn VehicleRepository>,
}

impl VehicleService {
    pub fn new(repo: Arc<dyn VehicleRepository>) -> Self {
        Self { repo }
    }

    pub async fn register_vehicle(&self, vehicle: Vehicle) -> Result<Vehicle> {
        // Business logic: check if license plate is valid, etc.
        if vehicle.license_plate.is_empty() {
            anyhow::bail!("License plate cannot be empty");
        }
        self.repo.create(vehicle).await
    }

    pub async fn get_vehicle(&self, id: Uuid) -> Result<Option<Vehicle>> {
        self.repo.get(id).await
    }

    pub async fn update_mileage(&self, id: Uuid, new_mileage: i32) -> Result<Vehicle> {
        let mut vehicle = self.repo.get(id).await?
            .ok_or_else(|| anyhow::anyhow!("Vehicle not found"))?;
        
        if new_mileage < vehicle.mileage {
            anyhow::bail!("New mileage cannot be less than current mileage");
        }

        vehicle.mileage = new_mileage;
        self.repo.update(vehicle).await
    }

    pub async fn change_status(&self, id: Uuid, new_status: VehicleStatus) -> Result<Vehicle> {
        let mut vehicle = self.repo.get(id).await?
            .ok_or_else(|| anyhow::anyhow!("Vehicle not found"))?;

        // Logic: if changing to InUse, it must be Available
        if new_status == VehicleStatus::InUse && vehicle.status != VehicleStatus::Available {
            anyhow::bail!("Vehicle must be Available to be put InUse");
        }

        vehicle.status = new_status;
        self.repo.update(vehicle).await
    }

    pub async fn list_vehicles(&self) -> Result<Vec<Vehicle>> {
        self.repo.list().await
    }
}
