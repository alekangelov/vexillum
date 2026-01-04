mod auth;
mod health;

use crate::pkg::state::AppState;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(info(
    title = "Vexillum API",
    description = "Feature flag management system API",
    version = "0.1.0",
))]
pub struct ApiDoc;

fn api_router() -> Router<AppState> {
    Router::new().merge(auth::router())
}

pub fn router(state: AppState) -> Router {
    let router = Router::new()
        .merge(health::router())
        .nest("/api", api_router())
        .with_state(state);

    let mut openapi = ApiDoc::openapi();
    openapi.merge(auth::AuthApi::openapi());
    openapi.merge(health::HealthApi::openapi());

    router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
}
