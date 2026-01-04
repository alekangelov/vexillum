mod auth;
mod health;

use crate::pkg::state::AppState;
use axum::Router;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(
        auth::login,
        auth::register,
        auth::request_magic_link,
        auth::verify_magic_link,
        auth::refresh_token,
        auth::logout,
        auth::get_current_user,
        auth::get_keys,
        health::healthz,
        health::readyz,
    ),
    components(
        schemas(
            auth::LoginRequest,
            auth::RegisterRequest,
            auth::MagicLinkRequest,
            auth::MagicLinkVerifyRequest,
            auth::AuthResponse,
            auth::PublicKeyResponse,
            health::HealthRes,
            health::XReq,
            crate::pkg::response::DataResponse<serde_json::Value>,
            crate::models::db::Users,
        ),
    ),
    tags(
        (name = "Authentication", description = "User authentication endpoints"),
        (name = "Health", description = "Server health checks"),
    ),
    info(
        title = "Vexillum API",
        description = "Feature flag management system API",
        version = "0.1.0",
    ),
)]
pub struct ApiDoc;

fn api_router() -> Router<AppState> {
    Router::new().merge(auth::router())
}

pub fn router(state: AppState) -> Router {
    let router = Router::new()
        .merge(health::router())
        .nest("/api", api_router())
        .with_state(state);

    let openapi = ApiDoc::openapi();

    router.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
}
