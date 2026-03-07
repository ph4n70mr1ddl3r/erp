use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::models::*;
use uuid::Uuid;

#[async_trait]
pub trait GRCRepository: Send + Sync {
    async fn create_hs_code(&self, hs_code: &HSCode) -> anyhow::Result<()>;
    async fn get_hs_code(&self, id: Uuid) -> anyhow::Result<Option<HSCode>>;
    async fn list_hs_codes(&self) -> anyhow::Result<Vec<HSCode>>;

    async fn create_product_trade_data(&self, data: &ProductTradeData) -> anyhow::Result<()>;
    async fn get_product_trade_data(&self, product_id: Uuid) -> anyhow::Result<Option<ProductTradeData>>;
    async fn update_product_trade_data(&self, data: &ProductTradeData) -> anyhow::Result<()>;

    async fn create_trade_license(&self, license: &TradeLicense) -> anyhow::Result<()>;
    async fn get_trade_license(&self, id: Uuid) -> anyhow::Result<Option<TradeLicense>>;
    async fn list_trade_licenses(&self, entity_id: Uuid) -> anyhow::Result<Vec<TradeLicense>>;

    async fn create_screening_result(&self, result: &ScreeningResult) -> anyhow::Result<()>;
    async fn get_latest_screening_result(&self, entity_id: Uuid) -> anyhow::Result<Option<ScreeningResult>>;
}

#[allow(dead_code)]
pub struct SqliteGRCRepository {
    pool: SqlitePool,
}

impl SqliteGRCRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GRCRepository for SqliteGRCRepository {
    async fn create_hs_code(&self, _hs_code: &HSCode) -> anyhow::Result<()> {
        Ok(())
    }

    async fn get_hs_code(&self, _id: Uuid) -> anyhow::Result<Option<HSCode>> {
        Ok(None)
    }

    async fn list_hs_codes(&self) -> anyhow::Result<Vec<HSCode>> {
        Ok(vec![])
    }

    async fn create_product_trade_data(&self, _data: &ProductTradeData) -> anyhow::Result<()> {
        Ok(())
    }

    async fn get_product_trade_data(&self, _product_id: Uuid) -> anyhow::Result<Option<ProductTradeData>> {
        Ok(None)
    }

    async fn update_product_trade_data(&self, _data: &ProductTradeData) -> anyhow::Result<()> {
        Ok(())
    }

    async fn create_trade_license(&self, _license: &TradeLicense) -> anyhow::Result<()> {
        Ok(())
    }

    async fn get_trade_license(&self, _id: Uuid) -> anyhow::Result<Option<TradeLicense>> {
        Ok(None)
    }

    async fn list_trade_licenses(&self, _entity_id: Uuid) -> anyhow::Result<Vec<TradeLicense>> {
        Ok(vec![])
    }

    async fn create_screening_result(&self, _result: &ScreeningResult) -> anyhow::Result<()> {
        Ok(())
    }

    async fn get_latest_screening_result(&self, _entity_id: Uuid) -> anyhow::Result<Option<ScreeningResult>> {
        Ok(None)
    }
}
