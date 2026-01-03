use crate::pkg::auth::AuthUser;
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
    #[serde(default)]
    pub remember_me: bool,
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
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PublicKeyResponse {
    pub public_key: String,
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

    // Generate tokens using JWT service
    let access_token = state.jwt.generate_token(
        user.id,
        state.jwt.get_length(&crate::pkg::jwt::TokenType::Access),
    )?;
    let refresh_token = state.jwt.generate_token(
        user.id,
        match payload.remember_me {
            true => state.jwt.get_length(&crate::pkg::jwt::TokenType::Long),
            false => state.jwt.get_length(&crate::pkg::jwt::TokenType::Refresh),
        },
    )?;

    // Create refresh token cookie
    let cookie = Cookie::build(("refresh_token", refresh_token.clone()))
        .path("/")
        .http_only(true)
        .secure(true) // Only send over HTTPS in production
        .same_site(SameSite::Strict)
        .build();
    Ok((
        [("Set-Cookie", cookie.to_string())],
        Json(AuthResponse {
            access_token,
            refresh_token: Some(refresh_token),
        }),
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

    // Generate tokens using JWT service
    let access_token = state.jwt.generate_token(
        user_id,
        state.jwt.get_length(&crate::pkg::jwt::TokenType::Access),
    )?;
    let refresh_token = state.jwt.generate_token(
        user_id,
        state.jwt.get_length(&crate::pkg::jwt::TokenType::Refresh),
    )?;
    // Create refresh token cookie
    let cookie = Cookie::build(("refresh_token", refresh_token.clone()))
        .path("/")
        .http_only(true)
        .secure(true) // Only send over HTTPS in production
        .same_site(SameSite::Strict)
        .build();

    Ok((
        [("Set-Cookie", cookie.to_string())],
        Json(AuthResponse {
            access_token,
            refresh_token: Some(refresh_token),
        }),
    ))
}

async fn request_magic_link(
    State(state): State<AppState>,
    Json(payload): Json<MagicLinkRequest>,
) -> Result<Json<serde_json::Value>, AppError> {
    let client = state.db_pool.get().await?;

    // Check if user exists
    let user_exists = client
        .query_opt("SELECT id FROM users WHERE email = $1", &[&payload.email])
        .await?;

    // Generate magic link token
    let token = Uuid::new_v4();
    let expires_at = chrono::Utc::now() + chrono::Duration::hours(24);

    // Extract user_id if user exists
    let user_id: Option<Uuid> = user_exists.as_ref().and_then(|row| row.try_get(0).ok());

    // Insert magic link record
    client
        .execute(
            "INSERT INTO magic_links (id, user_id, token, expires_at, created_at) VALUES ($1, $2, $3, $4, $5)",
            &[
                &Uuid::new_v4(),
                &user_id,
                &token,
                &expires_at,
                &chrono::Utc::now(),
            ],
        )
        .await?;

    // TODO: Send email with magic link
    Ok(Json(serde_json::json!({
        "message": "Magic link sent to email"
    })))
}

async fn verify_magic_link(
    State(state): State<AppState>,
    Json(payload): Json<MagicLinkVerifyRequest>,
) -> Result<impl IntoResponse, AppError> {
    let client = state.db_pool.get().await?;

    // Find and validate magic link token
    let magic_link_row = client
        .query_opt(
            "SELECT user_id, expires_at FROM magic_links WHERE token = $1",
            &[&uuid::Uuid::parse_str(&payload.token)
                .map_err(|_| AppError::BadRequest("Invalid token format".to_string()))?],
        )
        .await?
        .ok_or(AppError::Unauthorized(
            "Invalid or expired magic link".to_string(),
        ))?;

    let user_id: Option<Uuid> = magic_link_row
        .try_get(0)
        .map_err(|_| AppError::InternalError("Failed to parse user_id".to_string()))?;
    let expires_at: chrono::DateTime<chrono::Utc> = magic_link_row
        .try_get(1)
        .map_err(|_| AppError::InternalError("Failed to parse expires_at".to_string()))?;

    // Check if magic link has expired
    if expires_at < chrono::Utc::now() {
        return Err(AppError::Unauthorized("Magic link has expired".to_string()));
    }

    let user_id = user_id.ok_or(AppError::Unauthorized(
        "Magic link not associated with user".to_string(),
    ))?;

    // Delete the used magic link
    client
        .execute(
            "DELETE FROM magic_links WHERE token = $1",
            &[&uuid::Uuid::parse_str(&payload.token)
                .map_err(|_| AppError::BadRequest("Invalid token format".to_string()))?],
        )
        .await?;

    // Generate tokens
    let access_token = state.jwt.generate_token(
        user_id,
        state.jwt.get_length(&crate::pkg::jwt::TokenType::Access),
    )?;
    let refresh_token = state.jwt.generate_token(
        user_id,
        state.jwt.get_length(&crate::pkg::jwt::TokenType::Refresh),
    )?;

    // Create refresh token cookie
    let cookie = Cookie::build(("refresh_token", refresh_token.clone()))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .build();

    Ok((
        [("Set-Cookie", cookie.to_string())],
        Json(AuthResponse {
            access_token,
            refresh_token: Some(refresh_token),
        }),
    ))
}

async fn refresh_token(
    State(state): State<AppState>,
    jar: axum_extra::extract::CookieJar,
) -> Result<impl IntoResponse, AppError> {
    // Extract refresh token from cookies
    let refresh_token_str = jar
        .get("refresh_token")
        .map(|cookie| cookie.value().to_string())
        .ok_or(AppError::Unauthorized(
            "Refresh token not found".to_string(),
        ))?;

    // Validate the refresh token
    let claims = state.jwt.validate_token(&refresh_token_str)?;

    // Generate new access token
    let user_id = uuid::Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::InternalError("Invalid user ID in token".to_string()))?;

    let access_token = state.jwt.generate_token(
        user_id,
        state.jwt.get_length(&crate::pkg::jwt::TokenType::Access),
    )?;

    // Create refresh token cookie
    Ok((Json(AuthResponse {
        access_token,
        refresh_token: None,
    }),))
}

async fn logout() -> Result<impl IntoResponse, AppError> {
    // Create an empty refresh token cookie to clear it
    let cookie = Cookie::build(("refresh_token", ""))
        .path("/")
        .http_only(true)
        .secure(true)
        .same_site(SameSite::Strict)
        .build();

    Ok((
        [("Set-Cookie", cookie.to_string())],
        Json(serde_json::json!({
            "message": "Successfully logged out"
        })),
    ))
}

async fn get_current_user(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<Json<crate::models::users::User>, AppError> {
    let client = state.db_pool.get().await?;

    let row = client
        .query_opt("SELECT * FROM users WHERE id = $1", &[&auth_user.id])
        .await?
        .ok_or(AppError::NotFound("User not found".to_string()))?;

    let user = crate::models::users::User::from_row(&row)
        .map_err(|_| AppError::InternalError("Failed to parse user data".to_string()))?;

    Ok(Json(user))
}

async fn get_keys(State(state): State<AppState>) -> Result<Json<PublicKeyResponse>, AppError> {
    let public_key_bytes = state.jwt.public_key();
    let public_key_str = String::from_utf8(public_key_bytes.to_vec())
        .map_err(|_| AppError::InternalError("Invalid public key format".to_string()))?;

    Ok(Json(PublicKeyResponse {
        public_key: public_key_str,
    }))
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
        .route("/me", axum::routing::get(get_current_user))
        .route("/keys", axum::routing::get(get_keys));

    Router::new().nest("/v1/auth", auth_routes)
}
