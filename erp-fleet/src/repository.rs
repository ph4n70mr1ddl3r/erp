use async_trait::async_trait;
use anyhow::Result;
use uuid::Uuid;
use crate::models::*;
use tokio::sync::RwLock;

#[async_trait]
pub trait VehicleRepository: Send + Sync {
    async fn create(&self, vehicle: Vehicle) -> Result<Vehicle>;
    async fn get(&self, id: Uuid) -> Result<Option<Vehicle>>;
    async fn update(&self, vehicle: Vehicle) -> Result<Vehicle>;
    async fn delete(&self, id: Uuid) -> Result<()>;
    async fn list(&self) -> Result<Vec<Vehicle>>;
}

pub struct InMemoryVehicleRepository {
    vehicles: RwLock<Vec<Vehicle>>,
}

impl InMemoryVehicleRepository {
    pub fn new() -> Self {
        Self {
            vehicles: RwLock::new(Vec::new()),
        }
    }
}

#[async_trait]
impl VehicleRepository for InMemoryVehicleRepository {
    async fn create(&self, vehicle: Vehicle) -> Result<Vehicle> {
        let mut vehicles = self.vehicles.write().await;
        vehicles.push(vehicle.clone());
        Ok(vehicle)
    }

    async fn get(&self, id: Uuid) -> Result<Option<Vehicle>> {
        let vehicles = self.vehicles.read().await;
        Ok(vehicles.iter().find(|v| v.base.id == id).cloned())
    }

    async fn update(&self, vehicle: Vehicle) -> Result<Vehicle> {
        let mut vehicles = self.vehicles.write().await;
        if let Some(index) = vehicles.iter().position(|v| v.base.id == vehicle.base.id) {
            vehicles[index] = vehicle.clone();
            Ok(vehicle)
        } else {
            anyhow::bail!("Vehicle not found")
        }
    }

    async fn delete(&self, id: Uuid) -> Result<()> {
        let mut vehicles = self.vehicles.write().await;
        vehicles.retain(|v| v.base.id != id);
        Ok(())
    }

    async fn list(&self) -> Result<Vec<Vehicle>> {
        let vehicles = self.vehicles.read().await;
        Ok(vehicles.clone())
    }
}
