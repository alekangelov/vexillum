use crate::models::enums::UserRole;
use chrono::{DateTime, Utc};
use pgmap::FromRow;
use uuid::Uuid;

#[derive(FromRow, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow, Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct MagicLink {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub token: Uuid,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}
