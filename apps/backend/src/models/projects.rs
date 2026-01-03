use chrono::{DateTime, Utc};
use pgmap::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(FromRow)]
pub struct ProjectOwner {
    pub project_id: Uuid,
    pub user_id: Uuid,
}
