use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case")]
enum ErrorType {
    Forbidden,
    InternalError,
}

/// Standard error response structure returned by API handlers.
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    error: ErrorType,
    message: String,
}

impl ErrorResponse {
    fn new(error: ErrorType, message: impl Into<String>) -> Self {
        Self {
            error,
            message: message.into(),
        }
    }

    pub fn forbidden() -> Self {
        Self::new(ErrorType::Forbidden, "Access is forbidden.")
    }

    pub fn internal() -> Self {
        Self::new(ErrorType::InternalError, "An unexpected error occurred.")
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = match self.error {
            ErrorType::Forbidden => StatusCode::FORBIDDEN,
            ErrorType::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status, Json(self)).into_response()
    }
}
