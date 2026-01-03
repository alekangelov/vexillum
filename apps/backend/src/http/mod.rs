mod auth;
mod health;

use crate::pkg::state::AppState;
use axum::Router;

fn api_router() -> Router<AppState> {
    Router::new().merge(auth::router())
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .merge(health::router())
        .nest("/api", api_router())
        .with_state(state)
}
