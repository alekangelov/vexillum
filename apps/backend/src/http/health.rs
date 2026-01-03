use crate::pkg::state::AppState;
use axum::{Json, Router, extract::State, routing::get};
use deadpool_redis::redis;
use pgmap::FromRow;
use serde_json::{Value, json};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
}

#[axum_macros::debug_handler]
async fn healthz() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

#[derive(serde::Deserialize, serde::Serialize)]
struct XReq {
    some: String,
}

#[derive(pgmap::FromRow, serde::Deserialize, serde::Serialize)]
struct HealthRes {
    #[from_row(default)]
    ping: String,
    now: i64,
    #[from_row(json, default)]
    info: XReq,
}

#[axum_macros::debug_handler]
async fn readyz(State(s): State<AppState>) -> Json<Value> {
    let client = s.db_pool.get().await.unwrap();
    let ping_row =
        client.query_one(
            "SELECT 'pong' as ping, EXTRACT(EPOCH FROM NOW())::BIGINT AS now, '{\"some\": \"value\"}'::JSONB AS info",
            &[],
        )
        .await;
    let mut redis_conn = s.redis_pool.get().await.unwrap();
    let redis_ping: String = redis::cmd("PING")
        .query_async(&mut redis_conn)
        .await
        .unwrap();
    if ping_row.is_err() {
        return Json(json!({
            "status": "error",
            "now": chrono::Utc::now().timestamp(),
            "database": "unreachable"
        }));
    }
    Json(json!({
        "status": "ok",
        "now": chrono::Utc::now().timestamp(),
        "database": HealthRes::from_row(&ping_row.unwrap()).unwrap(),
        "redis": redis_ping
    }))
}
