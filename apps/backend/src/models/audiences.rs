use crate::models::enums::{AudienceScope, MatchOperator};
use chrono::{DateTime, Utc};
use pgmap::FromRow;
use uuid::Uuid;

#[derive(FromRow)]
pub struct Audience {
    pub id: Uuid,
    pub name: String,
    pub attribute: String,
    pub operator: MatchOperator,
    pub value: String,
    pub scope: AudienceScope,
    pub project_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
