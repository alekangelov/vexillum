use uuid::Uuid;
use chrono::{DateTime, Utc};
use pgmap::FromRow;

#[derive(FromRow)]
pub struct ApiKey {
    pub id: Uuid,
    pub user_id: Option<Uuid>,
    pub environment_id: Option<Uuid>,
    pub is_server_key: bool,
    pub key_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
