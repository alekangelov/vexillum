mod config;
mod http;
mod state;

use config::Config;
use state::BaseState;

#[tokio::main]
async fn main() {
    // Load configuration
    let config = Config::load();
    let server_addr = config.server_addr();
    env_logger::init();

    config.print_summary();

    // Initialize app state (handles database connection)
    let state = BaseState::new_arc(config)
        .await
        .expect("Failed to initialize app state");

    // Build the router
    let app = http::router(state);

    let listener = tokio::net::TcpListener::bind(server_addr.clone())
        .await
        .expect("Failed to bind server address");

    println!("Server running on http://{}", server_addr);

    axum::serve(listener, app).await.expect("Server error");
}
