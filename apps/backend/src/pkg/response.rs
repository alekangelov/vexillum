use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct DataResponse<V: Serialize> {
    pub success: bool,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<V>,
    #[serde(skip)]
    pub error_code: Option<u16>,
    #[serde(rename = "error")]
    pub error_details: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warnings: Option<Vec<String>>,
}

impl<V: Serialize> DataResponse<V> {
    pub fn new() -> Self {
        DataResponse {
            success: true,
            message: String::new(),
            data: None,
            error_code: None,
            error_details: None,
            warnings: None,
        }
    }

    pub fn success(mut self, success: bool) -> Self {
        self.success = success;
        self
    }

    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = message.into();
        self
    }

    pub fn data(mut self, data: V) -> Self {
        self.data = Some(data);
        self
    }

    pub fn error_code(mut self, code: u16) -> Self {
        self.error_code = Some(code);
        self
    }

    pub fn error_details(mut self, details: impl Into<String>) -> Self {
        self.error_details = Some(details.into());
        self
    }

    pub fn warning(mut self, warning: impl Into<String>) -> Self {
        match self.warnings {
            Some(ref mut warnings) => warnings.push(warning.into()),
            None => self.warnings = Some(vec![warning.into()]),
        }
        self
    }

    pub fn warnings(mut self, warnings: Vec<String>) -> Self {
        self.warnings = Some(warnings);
        self
    }

    pub fn build(self) -> DataResponse<V> {
        DataResponse {
            success: self.success,
            message: self.message,
            data: self.data,
            error_code: self.error_code,
            error_details: self.error_details,
            warnings: self.warnings,
        }
    }

    pub fn error(message: impl Into<String>, code: u16, details: impl Into<String>) -> Self {
        DataResponse {
            success: false,
            message: message.into(),
            data: None,
            error_code: Some(code),
            error_details: Some(details.into()),
            warnings: None,
        }
    }

    pub fn result(message: impl Into<String>, data: V) -> Self {
        DataResponse {
            success: true,
            message: message.into(),
            data: Some(data),
            error_code: None,
            error_details: None,
            warnings: None,
        }
    }
}

impl<V: Serialize> Default for DataResponse<V> {
    fn default() -> Self {
        DataResponse {
            success: true,
            message: String::new(),
            data: None,
            error_code: None,
            error_details: None,
            warnings: None,
        }
    }
}

impl<V: Serialize> IntoResponse for DataResponse<V> {
    fn into_response(self) -> Response {
        let status = if self.success {
            StatusCode::OK
        } else {
            StatusCode::from_u16(self.error_code.unwrap_or(500))
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
        };

        (status, Json(self)).into_response()
    }
}
