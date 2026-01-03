use crate::pkg::error::AppError;
use crate::pkg::state::AppState;
use argon2::{
    PasswordVerifier, password_hash::PasswordHash, password_hash::PasswordHasher,
    password_hash::SaltString,
};
use axum::{Json, Router, extract::State, response::IntoResponse};
use axum_extra::extract::{
    WithRejection,
    cookie::{Cookie, SameSite},
};
use pgmap::FromRow;
use rand::TryRngCore;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct MagicLinkRequest {
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct MagicLinkVerifyRequest {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse {
    pub access_token: String,
}

// Auth handlers
#[axum_macros::debug_handler]
async fn login(
    State(state): State<AppState>,
    WithRejection(Json(payload), _): WithRejection<Json<LoginRequest>, AppError>,
) -> Result<impl IntoResponse, AppError> {
    let client = state.db_pool.get().await?;

    let row = client
        .query_opt("SELECT * FROM users WHERE email = $1", &[&payload.email])
        .await?
        .ok_or(AppError::Unauthorized(
            "Invalid email or password".to_string(),
        ))?;

    let user = crate::models::users::User::from_row(&row)
        .map_err(|_| AppError::InternalError("Failed to parse user data".to_string()))?;

    // Verify password
    let parsed_hash = PasswordHash::new(&user.password_hash)?;

    state
        .argon2
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized("Invalid email or password".to_string()))?;

    // Generate tokens (placeholder - implement your JWT logic)
    let access_token = format!("access_{}", Uuid::new_v4());
    let refresh_token = format!("refresh_{}", Uuid::new_v4());

    // Create refresh token cookie
    let cookie = Cookie::build(("refresh_token", refresh_token))
        .path("/")
        .http_only(true)
        .secure(true) // Only send over HTTPS in production
        .same_site(SameSite::Strict)
        .build();
    Ok((
        [("Set-Cookie", cookie.to_string())],
        Json(AuthResponse { access_token }),
    ))
}

#[axum_macros::debug_handler]
async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<impl IntoResponse, AppError> {
    let client = state.db_pool.get().await?;

    // Check if user already exists
    let existing = client
        .query_opt("SELECT id FROM users WHERE email = $1", &[&payload.email])
        .await?;

    if existing.is_some() {
        return Err(AppError::Conflict("User already exists".to_string()));
    }

    // Hash password
    let salt = rand::rngs::OsRng
        .try_next_u64()
        .ok()
        .and_then(|num| SaltString::encode_b64(&num.to_le_bytes()).ok())
        .ok_or(AppError::InternalError(
            "Failed to generate salt".to_string(),
        ))?;

    let hashed_password = state
        .argon2
        .hash_password(payload.password.as_bytes(), &salt)?
        .to_string();

    // Insert new user
    let user_id = Uuid::new_v4();
    client
        .execute(
            "INSERT INTO users (id, email, password_hash, role) VALUES ($1, $2, $3, $4)",
            &[
                &user_id,
                &payload.email,
                &hashed_password,
                &crate::models::enums::UserRole::Viewer,
            ],
        )
        .await?;

    // Generate tokens
    let access_token = format!("access_{}", Uuid::new_v4());
    let refresh_token = format!("refresh_{}", Uuid::new_v4());

    // Create refresh token cookie
    let cookie = Cookie::build(("refresh_token", refresh_token))
        .path("/")
        .http_only(true)
        .secure(true) // Only send over HTTPS in production
        .same_site(SameSite::Strict)
        .build();

    Ok((
        [("Set-Cookie", cookie.to_string())],
        Json(AuthResponse { access_token }),
    ))
}

async fn request_magic_link(
    State(_state): State<AppState>,
    Json(_payload): Json<MagicLinkRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    todo!("Implement magic link request");
}

async fn verify_magic_link(
    State(_state): State<AppState>,
    Json(_payload): Json<MagicLinkVerifyRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    todo!("Implement magic link verification");
}

async fn refresh_token(
    State(_state): State<AppState>,
    Json(_payload): Json<RefreshTokenRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    todo!("Implement token refresh");
}

async fn logout() -> Result<Json<serde_json::Value>, AppError> {
    todo!("Implement logout");
}

async fn get_current_user() -> Result<Json<serde_json::Value>, AppError> {
    todo!("Get current authenticated user");
}

pub fn router() -> Router<AppState> {
    let auth_routes = Router::new()
        .route("/login", axum::routing::post(login))
        .route("/register", axum::routing::post(register))
        .route(
            "/magic-link/request",
            axum::routing::post(request_magic_link),
        )
        .route("/magic-link/verify", axum::routing::post(verify_magic_link))
        .route("/refresh", axum::routing::post(refresh_token))
        .route("/logout", axum::routing::post(logout))
        .route("/me", axum::routing::get(get_current_user));

    Router::new().nest("/v1/auth", auth_routes)
}
