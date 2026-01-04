// Auto-generated database models
use uuid::Uuid;
use chrono::{DateTime, Utc};
use pgmap::FromRow;
use serde::{Serialize, Deserialize};
use utoipa::ToSchema;
use super::enums::*;

#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct ApiKeys {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub environment_id: Option<Uuid>,
    pub is_server_key: Option<bool>,
    pub key_hash: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct Audiences {
    pub id: Uuid,
    pub org_id: Option<Uuid>,
    pub name: String,
    pub attribute: String,
    pub operator: MatchOperator,
    pub value: String,
    pub scope: AudienceScope,
    pub project_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct Environments {
    pub id: Uuid,
    pub project_id: Option<Uuid>,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct FeatureFlagOverrides {
    pub id: Uuid,
    pub feature_flag_id: Option<Uuid>,
    pub audience_id: Option<Uuid>,
    pub is_enabled: Option<bool>,
    pub r#type: FeatureFlagType,
    pub value: Option<serde_json::Value>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct FeatureFlags {
    pub id: Uuid,
    pub org_id: Option<Uuid>,
    pub key: String,
    pub is_enabled: Option<bool>,
    pub r#type: FeatureFlagType,
    pub value: Option<serde_json::Value>,
    pub project_id: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct MagicLinks {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub token: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: Option<DateTime<Utc>>,
}
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct Orgs {
    pub id: Uuid,
    pub slug: String,
    pub name: String,
    pub region: String,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct ProjectOwners {
    pub project_id: Uuid,
    pub user_id: Uuid,
}
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct Projects {
    pub id: Uuid,
    pub org_id: Option<Uuid>,
    pub name: String,
    pub description: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
#[derive(FromRow, Serialize, Deserialize, ToSchema)]
pub struct Users {
    pub id: Uuid,
    pub org_id: Option<Uuid>,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
