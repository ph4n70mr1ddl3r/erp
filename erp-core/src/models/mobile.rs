use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::models::common::Status;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DeviceType {
    Ios,
    Android,
    Web,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileDevice {
    pub id: Uuid,
    pub user_id: Uuid,
    pub device_type: DeviceType,
    pub device_token: String,
    pub device_name: Option<String>,
    pub os_version: Option<String>,
    pub app_version: Option<String>,
    pub last_active: Option<DateTime<Utc>>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MobileSession {
    pub id: Uuid,
    pub device_id: Uuid,
    pub user_id: Uuid,
    pub login_at: DateTime<Utc>,
    pub logout_at: Option<DateTime<Utc>>,
    pub ip_address: Option<String>,
    pub status: Status,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum PushNotificationStatus {
    Pending,
    Sent,
    Delivered,
    Read,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushNotification {
    pub id: Uuid,
    pub device_id: Uuid,
    pub title: String,
    pub message: String,
    pub data: Option<String>,
    pub sent_at: Option<DateTime<Utc>>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub read_at: Option<DateTime<Utc>>,
    pub status: PushNotificationStatus,
}
