use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::pkg::response::DataResponse;

#[derive(Debug, Error)]
pub enum AppError {
    // 400 - Bad Request
    #[error("{0}")]
    BadRequest(String),

    // 401 - Unauthorized
    #[error("{0}")]
    Unauthorized(String),

    // 404 - Not Found
    #[error("{0}")]
    NotFound(String),

    // 409 - Conflict
    #[error("{0}")]
    Conflict(String),

    // 422 - Unprocessable Entity (Validation)
    #[error("{0}")]
    UnprocessableEntity(String),

    // 500 - Internal Server Error
    #[error("{0}")]
    InternalError(String),

    // JSON parsing error
    #[error(transparent)]
    JsonExtractorRejection(#[from] JsonRejection),
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),
            AppError::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, msg),
            AppError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg),
            AppError::UnprocessableEntity(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            AppError::JsonExtractorRejection(rejection) => {
                let status = rejection.status();
                let error = ErrorResponse {
                    error: rejection.body_text(),
                };
                return (status, Json(error)).into_response();
            }
        };

        let body = DataResponse::<()>::new()
            .success(false)
            .message("Oops, something went wrong")
            .error_details(error_message)
            .error_code(status.into())
            .build();

        (status, Json(body)).into_response()
    }
}

impl From<tokio_postgres::error::Error> for AppError {
    fn from(err: tokio_postgres::error::Error) -> Self {
        AppError::InternalError(format!("Database error: {}", err))
    }
}

impl From<deadpool_postgres::CreatePoolError> for AppError {
    fn from(err: deadpool_postgres::CreatePoolError) -> Self {
        AppError::InternalError(format!("Failed to create connection pool: {}", err))
    }
}

impl From<deadpool_postgres::PoolError> for AppError {
    fn from(_err: deadpool_postgres::PoolError) -> Self {
        AppError::InternalError("Failed to get database connection".to_string())
    }
}

impl From<argon2::password_hash::Error> for AppError {
    fn from(_err: argon2::password_hash::Error) -> Self {
        AppError::InternalError("Failed to process password".to_string())
    }
}

impl From<std::string::FromUtf8Error> for AppError {
    fn from(_err: std::string::FromUtf8Error) -> Self {
        AppError::BadRequest("Invalid UTF-8 in request".to_string())
    }
}

impl From<serde_json::error::Error> for AppError {
    fn from(err: serde_json::error::Error) -> Self {
        AppError::BadRequest(format!("JSON error: {}", err))
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::InternalError(format!("IO error: {}", err))
    }
}
