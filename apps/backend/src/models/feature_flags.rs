use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde_json::Value;
use crate::models::enums::FeatureFlagType;
use pgmap::FromRow;

#[derive(FromRow)]
pub struct FeatureFlag {
    pub id: Uuid,
    pub key: String,
    pub is_enabled: bool,
    pub flag_type: FeatureFlagType,
    pub value: Option<Value>,
    pub project_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow)]
pub struct FeatureFlagOverride {
    pub id: Uuid,
    pub feature_flag_id: Uuid,
    pub audience_id: Option<Uuid>,
    pub is_enabled: bool,
    pub override_type: FeatureFlagType,
    pub value: Option<Value>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
