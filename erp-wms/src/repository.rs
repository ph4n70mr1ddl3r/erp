use async_trait::async_trait;
use sqlx::SqlitePool;
use uuid::Uuid;

use crate::models::*;

#[async_trait]
pub trait WMSRepository: Send + Sync {
    async fn create_location(&self, _location: &StorageLocation) -> anyhow::Result<()> { Ok(()) }
    async fn get_location(&self, _id: Uuid) -> anyhow::Result<Option<StorageLocation>> { Ok(None) }
    async fn list_locations(&self, _warehouse_id: Uuid, _page: i32, _page_size: i32) -> anyhow::Result<Vec<StorageLocation>> { Ok(vec![]) }
    async fn update_location_occupied(&self, _id: Uuid, _delta: i64) -> anyhow::Result<()> { Ok(()) }
    async fn create_putaway_task(&self, _task: &PutAwayTask) -> anyhow::Result<()> { Ok(()) }
    async fn list_putaway_tasks(&self, _status: Option<PutAwayStatus>) -> anyhow::Result<Vec<PutAwayTask>> { Ok(vec![]) }
    async fn update_putaway_status(&self, _id: Uuid, _status: PutAwayStatus, _location: Option<Uuid>) -> anyhow::Result<()> { Ok(()) }
    async fn create_pick_task(&self, _task: &PickTask) -> anyhow::Result<()> { Ok(()) }
    async fn list_pick_tasks(&self, _wave_id: Option<Uuid>, _status: Option<PickStatus>) -> anyhow::Result<Vec<PickTask>> { Ok(vec![]) }
    async fn update_pick_status(&self, _id: Uuid, _status: PickStatus, _qty: i64) -> anyhow::Result<()> { Ok(()) }
    async fn create_wave(&self, _wave: &Wave) -> anyhow::Result<()> { Ok(()) }
    async fn get_wave(&self, _id: Uuid) -> anyhow::Result<Option<Wave>> { Ok(None) }
    async fn list_waves(&self, _warehouse_id: Uuid, _status: Option<WaveStatus>) -> anyhow::Result<Vec<Wave>> { Ok(vec![]) }
    async fn update_wave_status(&self, _id: Uuid, _status: WaveStatus) -> anyhow::Result<()> { Ok(()) }
    async fn create_crossdock(&self, _order: &CrossDockOrder) -> anyhow::Result<()> { Ok(()) }
    async fn list_crossdocks(&self, _status: Option<CrossDockStatus>) -> anyhow::Result<Vec<CrossDockOrder>> { Ok(vec![]) }
    async fn create_receipt(&self, _receipt: &ReceivingReceipt) -> anyhow::Result<()> { Ok(()) }
    async fn get_receipt(&self, _id: Uuid) -> anyhow::Result<Option<ReceivingReceipt>> { Ok(None) }
    async fn create_receipt_line(&self, _line: &ReceiptLine) -> anyhow::Result<()> { Ok(()) }
    async fn create_manifest(&self, _manifest: &ShippingManifest) -> anyhow::Result<()> { Ok(()) }
    async fn list_manifests(&self, _warehouse_id: Uuid) -> anyhow::Result<Vec<ShippingManifest>> { Ok(vec![]) }
    async fn create_cycle_count(&self, _count: &CycleCount) -> anyhow::Result<()> { Ok(()) }
    async fn get_cycle_count(&self, _id: Uuid) -> anyhow::Result<Option<CycleCount>> { Ok(None) }
    async fn create_count_line(&self, _line: &CountLine) -> anyhow::Result<()> { Ok(()) }
    async fn update_count_line(&self, _id: Uuid, _counted_qty: i64) -> anyhow::Result<()> { Ok(()) }
    async fn create_zone(&self, _zone: &WarehouseZone) -> anyhow::Result<()> { Ok(()) }
    async fn list_zones(&self, _warehouse_id: Uuid) -> anyhow::Result<Vec<WarehouseZone>> { Ok(vec![]) }
}

pub struct SqliteWMSRepository {
    pub pool: SqlitePool,
}

impl SqliteWMSRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WMSRepository for SqliteWMSRepository {}
