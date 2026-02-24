use chrono::{DateTime, Utc};
use erp_core::BaseEntity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FlagTargetType {
    All,
    Users,
    Groups,
    Tenants,
    Percentage,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OverrideTargetType {
    User,
    Group,
    Tenant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlag {
    pub base: BaseEntity,
    pub key: String,
    pub name: String,
    pub description: Option<String>,
    pub enabled: bool,
    pub rollout_percentage: i32,
    pub target_type: FlagTargetType,
    pub target_ids: Option<String>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub prerequisites: Option<String>,
    pub variants: Option<String>,
    pub default_variant: Option<String>,
    pub is_system: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlagOverride {
    pub base: BaseEntity,
    pub flag_id: Uuid,
    pub target_type: OverrideTargetType,
    pub target_id: String,
    pub enabled: bool,
    pub variant: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlagHistory {
    pub base: BaseEntity,
    pub flag_id: Uuid,
    pub action: String,
    pub old_value: Option<String>,
    pub new_value: Option<String>,
    pub changed_by: Option<Uuid>,
    pub changed_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlagUsage {
    pub base: BaseEntity,
    pub flag_id: Uuid,
    pub user_id: Option<Uuid>,
    pub variant: Option<String>,
    pub evaluated_at: DateTime<Utc>,
    pub context: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FlagEvaluationContext {
    pub user_id: Option<Uuid>,
    pub group_ids: Vec<Uuid>,
    pub tenant_id: Option<Uuid>,
    pub custom_attributes: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlagUsageStats {
    pub flag_id: Uuid,
    pub total_evaluations: i64,
    pub unique_users: i64,
    pub variant_counts: std::collections::HashMap<String, i64>,
}
