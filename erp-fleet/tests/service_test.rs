use std::sync::Arc;
use uuid::Uuid;
use erp_core::BaseEntity;
use erp_fleet::{Vehicle, VehicleStatus, InMemoryVehicleRepository, VehicleService};
use chrono::Utc;

#[tokio::test]
async fn test_register_vehicle() {
    let repo = Arc::new(InMemoryVehicleRepository::new());
    let service = VehicleService::new(repo);

    let vehicle = Vehicle {
        base: BaseEntity {
            id: Uuid::new_v4(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: None,
            updated_by: None,
        },
        make: "Toyota".to_string(),
        model: "Corolla".to_string(),
        year: 2020,
        license_plate: "XYZ-123".to_string(),
        status: VehicleStatus::Available,
        mileage: 10000,
    };

    let result = service.register_vehicle(vehicle.clone()).await;
    assert!(result.is_ok());

    let fetched = service.get_vehicle(vehicle.base.id).await.unwrap().unwrap();
    assert_eq!(fetched.license_plate, "XYZ-123");
}

#[tokio::test]
async fn test_update_mileage() {
    let repo = Arc::new(InMemoryVehicleRepository::new());
    let service = VehicleService::new(repo);

    let id = Uuid::new_v4();
    let vehicle = Vehicle {
        base: BaseEntity {
            id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: None,
            updated_by: None,
        },
        make: "Ford".to_string(),
        model: "Transit".to_string(),
        year: 2018,
        license_plate: "VAN-001".to_string(),
        status: VehicleStatus::Available,
        mileage: 50000,
    };

    service.register_vehicle(vehicle).await.unwrap();

    // Valid update
    let updated = service.update_mileage(id, 51000).await.unwrap();
    assert_eq!(updated.mileage, 51000);

    // Invalid update (mileage goes backwards)
    let result = service.update_mileage(id, 50500).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "New mileage cannot be less than current mileage");
}

#[tokio::test]
async fn test_change_status() {
    let repo = Arc::new(InMemoryVehicleRepository::new());
    let service = VehicleService::new(repo);

    let id = Uuid::new_v4();
    let vehicle = Vehicle {
        base: BaseEntity {
            id,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: None,
            updated_by: None,
        },
        make: "Tesla".to_string(),
        model: "Model 3".to_string(),
        year: 2022,
        license_plate: "EV-001".to_string(),
        status: VehicleStatus::Available,
        mileage: 5000,
    };

    service.register_vehicle(vehicle).await.unwrap();

    // Valid state transition
    let updated = service.change_status(id, VehicleStatus::InUse).await.unwrap();
    assert_eq!(updated.status, VehicleStatus::InUse);

    // Invalid state transition (Maintenance to InUse requires Available first based on our naive logic, wait, in our service: if new_status == InUse, it MUST be Available)
    // Currently it's InUse, let's try to set it to InUse again or from Maintenance
    let _ = service.change_status(id, VehicleStatus::Maintenance).await.unwrap();
    
    let result = service.change_status(id, VehicleStatus::InUse).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Vehicle must be Available to be put InUse");
}
