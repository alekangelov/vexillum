use crate::pkg::error::AppError;
use crate::pkg::state::AppState;
use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use axum_auth::AuthBearer;
use uuid::Uuid;

/// User context extracted from JWT access token
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub id: Uuid,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        // Extract the Authorization header
        let AuthBearer(token) =
            AuthBearer::from_request_parts(parts, state)
                .await
                .map_err(|_| {
                    AppError::Unauthorized("Missing or invalid authorization header".to_string())
                })?;

        // Validate the token
        let claims = app_state.jwt.validate_token(&token)?;

        // Parse user ID from claims
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::InternalError("Invalid user ID in token".to_string()))?;

        Ok(AuthUser { id: user_id })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_auth_user_creation() {
        let user_id = Uuid::new_v4();
        let auth_user = AuthUser { id: user_id };
        assert_eq!(auth_user.id, user_id);
    }
}
