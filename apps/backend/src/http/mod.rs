pub mod health;

use crate::state::AppState;
use axum::Router;

pub fn router(state: AppState) -> Router {
    Router::new().merge(health::router()).with_state(state)
}
