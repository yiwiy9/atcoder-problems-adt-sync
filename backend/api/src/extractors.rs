use crate::constants::{EXTENSION_NAME_ENV, X_EXTENSION_NAME_HEADER};
use crate::error::ErrorResponse;
use axum::{extract::FromRequestParts, http::request::Parts};
use lambda_http::tracing;
use std::env;

/// Extractor that validates the `x-extension-name` header against the expected value.
/// Rejects the request with 403 if missing or invalid.
pub struct VerifiedExtension;

impl<S> FromRequestParts<S> for VerifiedExtension
where
    S: Send + Sync,
{
    type Rejection = ErrorResponse;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = &parts.headers;

        let expected_name = env::var(EXTENSION_NAME_ENV)
            .unwrap_or_else(|_| panic!("Environment variable {} is not set", EXTENSION_NAME_ENV));

        let actual = headers
            .get(X_EXTENSION_NAME_HEADER)
            .and_then(|v| v.to_str().ok());

        if actual == Some(&expected_name) {
            Ok(VerifiedExtension)
        } else {
            tracing::error!(
                "Forbidden: {} header is missing or invalid",
                X_EXTENSION_NAME_HEADER
            );
            Err(ErrorResponse::forbidden())
        }
    }
}
