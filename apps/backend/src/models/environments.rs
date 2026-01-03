use uuid::Uuid;
use chrono::{DateTime, Utc};
use pgmap::FromRow;

#[derive(FromRow)]
pub struct Environment {
    pub id: Uuid,
    pub project_id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
