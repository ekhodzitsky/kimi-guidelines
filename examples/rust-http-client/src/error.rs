use thiserror::Error;

/// Error type for all GitHub API client operations.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP request failed.
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),

    /// JSON serialization or deserialization failed.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// The provided URL is not valid.
    #[error("invalid URL: {0}")]
    InvalidUrl(String),

    /// The provided API token is empty.
    #[error("API token must be non-empty")]
    InvalidToken,

    /// The provided page number is zero.
    #[error("page number must be non-zero")]
    InvalidPage,

    /// The provided per-page value is out of range.
    #[error("per-page must be between 1 and 100, got {0}")]
    InvalidPerPage(u8),
}
