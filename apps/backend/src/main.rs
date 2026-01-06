use pkg::config::Config;
use pkg::state::BaseState;

mod http;
mod models;
mod pkg;

#[tokio::main]
async fn main() {
    // Load configuration
    let config = Config::load();
    env_logger::init();

    config.print_summary();

    // Initialize app state (handles database connection)
    let state = BaseState::new_arc(config)
        .await
        .expect("Failed to initialize app state");

    // Build the router
    let app = http::router(state.clone());

    let listener = tokio::net::TcpListener::bind(state.config.server_addr())
        .await
        .expect("Failed to bind server address");

    println!("Server running on http://{}", state.config.server_addr());
    println!(
        "Swagger UI available at http://{}/swagger-ui",
        state.config.server_addr()
    );

    println!("Frontend available at {}", state.config.frontend_url);

    axum::serve(listener, app).await.expect("Server error");
}
