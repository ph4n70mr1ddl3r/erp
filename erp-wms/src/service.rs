use chrono::Utc;
use uuid::Uuid;

use crate::models::*;
use crate::repository::*;

pub struct WMSService<R: WMSRepository> {
    pub repo: R,
}

impl WMSService<SqliteWMSRepository> {
    pub fn new(repo: SqliteWMSRepository) -> Self {
        Self { repo }
    }
}

impl<R: WMSRepository> WMSService<R> {
    pub async fn create_location(&self, req: CreateLocationRequest) -> anyhow::Result<StorageLocation> {
        let location = StorageLocation {
            id: Uuid::new_v4(),
            warehouse_id: req.warehouse_id,
            zone: req.zone,
            aisle: req.aisle,
            rack: req.rack,
            shelf: req.shelf,
            bin: req.bin,
            location_type: req.location_type,
            capacity: req.capacity,
            occupied: 0,
            status: LocationStatus::Active,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.repo.create_location(&location).await?;
        Ok(location)
    }

    pub async fn get_location(&self, id: Uuid) -> anyhow::Result<Option<StorageLocation>> {
        self.repo.get_location(id).await
    }

    pub async fn list_locations(&self, warehouse_id: Uuid, page: i32, page_size: i32) -> anyhow::Result<Vec<StorageLocation>> {
        self.repo.list_locations(warehouse_id, page, page_size).await
    }

    pub async fn create_wave(&self, req: CreateWaveRequest) -> anyhow::Result<Wave> {
        let wave_number = format!("WAVE-{}", chrono::Utc::now().format("%Y%m%d%H%M"));
        let wave = Wave {
            id: Uuid::new_v4(),
            wave_number,
            warehouse_id: req.warehouse_id,
            status: WaveStatus::Planning,
            planned_date: req.planned_date,
            released_at: None,
            completed_at: None,
            total_picks: 0,
            completed_picks: 0,
            created_at: Utc::now(),
        };
        self.repo.create_wave(&wave).await?;
        Ok(wave)
    }

    pub async fn release_wave(&self, id: Uuid) -> anyhow::Result<Wave> {
        self.repo.update_wave_status(id, WaveStatus::Released).await?;
        self.repo.get_wave(id).await?.ok_or_else(|| anyhow::anyhow!("Wave not found"))
    }

    pub async fn create_pick_task(&self, req: CreatePickTaskRequest) -> anyhow::Result<PickTask> {
        let task = PickTask {
            id: Uuid::new_v4(),
            wave_id: None,
            order_id: req.order_id,
            product_id: req.product_id,
            location_id: Uuid::nil(),
            quantity: req.quantity,
            picked_quantity: 0,
            status: PickStatus::Released,
            pick_type: req.pick_type,
            priority: req.priority,
            assigned_to: None,
            started_at: None,
            completed_at: None,
            created_at: Utc::now(),
        };
        self.repo.create_pick_task(&task).await?;
        Ok(task)
    }

    pub async fn complete_pick(&self, id: Uuid, qty: i64) -> anyhow::Result<PickTask> {
        self.repo.update_pick_status(id, PickStatus::Picked, qty).await?;
        self.repo.list_pick_tasks(None, None).await?.into_iter().find(|t| t.id == id).ok_or_else(|| anyhow::anyhow!("Pick task not found"))
    }

    pub async fn create_cycle_count(&self, req: CreateCycleCountRequest) -> anyhow::Result<CycleCount> {
        let count_number = format!("CC-{}", chrono::Utc::now().format("%Y%m%d%H%M"));
        let count = CycleCount {
            id: Uuid::new_v4(),
            count_number,
            warehouse_id: req.warehouse_id,
            count_type: req.count_type,
            status: CountStatus::Scheduled,
            scheduled_date: req.scheduled_date,
            completed_date: None,
            variance_count: 0,
            accuracy_rate: None,
            created_at: Utc::now(),
        };
        self.repo.create_cycle_count(&count).await?;
        Ok(count)
    }

    pub async fn create_receipt(&self, warehouse_id: Uuid, po_id: Option<Uuid>) -> anyhow::Result<ReceivingReceipt> {
        let receipt_number = format!("RCPT-{}", chrono::Utc::now().format("%Y%m%d%H%M"));
        let receipt = ReceivingReceipt {
            id: Uuid::new_v4(),
            receipt_number,
            warehouse_id,
            po_id,
            carrier: None,
            tracking_number: None,
            status: ReceivingStatus::Expected,
            received_by: None,
            received_at: None,
            created_at: Utc::now(),
        };
        self.repo.create_receipt(&receipt).await?;
        Ok(receipt)
    }

    pub async fn create_zone(&self, warehouse_id: Uuid, zone_code: String, zone_name: String, zone_type: ZoneType) -> anyhow::Result<WarehouseZone> {
        let zone = WarehouseZone {
            id: Uuid::new_v4(),
            warehouse_id,
            zone_code,
            zone_name,
            zone_type,
            temperature_controlled: false,
            status: "Active".to_string(),
        };
        self.repo.create_zone(&zone).await?;
        Ok(zone)
    }

    pub async fn optimize_wave(&self, req: OptimizeWaveRequest) -> anyhow::Result<WaveOptimizationResult> {
        let order_count = req.order_ids.len();
        Ok(WaveOptimizationResult {
            waves: vec![WavePlan {
                wave_number: 1,
                order_ids: req.order_ids,
                zone_assignments: vec![],
            }],
            estimated_picks: order_count as i32 * 5,
            estimated_travel_time: order_count as i32 * 10,
        })
    }
}
