/// Errors that can occur while using `AtCoderClient`.
#[derive(Debug, thiserror::Error)]
pub enum AtCoderClientError {
    /// A network or request error from reqwest.
    #[error("Reqwest client error: {0}")]
    ReqwestError(#[from] reqwest::Error),

    /// The provided REVEL_SESSION is invalid or has expired (HTTP 401 or redirect).
    #[error("Session is invalid or expired (HTTP 401 or redirect)")]
    InvalidSession,

    /// The requested page was not found (HTTP 404).
    #[error("Requested page does not exist (HTTP 404)")]
    NotFound,

    /// The server returned an internal error (HTTP 5xx).
    #[error("Server error (HTTP 5xx): {0}")]
    ServerError(reqwest::StatusCode),

    /// An unexpected HTTP status code was returned.
    #[error("Unexpected HTTP status code: {0}")]
    UnexpectedHttpStatus(reqwest::StatusCode),

    /// The page was fetched successfully but contains no meaningful contents.
    #[error("The page contains no meaningful contents")]
    EmptyContents,

    /// Failed to parse the expected HTML structure.
    #[error("Failed to parse HTML content")]
    HtmlParseError,
}
