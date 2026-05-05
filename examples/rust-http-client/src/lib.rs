//! A type-safe GitHub API client demonstrating real-world Rust patterns.
//!
//! Features:
//! - Newtypes for API keys, URLs, and pagination parameters.
//! - Typestate for unauthenticated vs authenticated clients.
//! - Comprehensive error handling via `thiserror`.
//! - Property-tested response parsing.

pub mod client;
pub mod error;
pub mod types;

#[cfg(kani)]
pub mod kani_proofs;

pub use client::{Authenticated, GitHubClient, Unauthenticated};
pub use error::Error;
pub use types::{
    ApiKey, ApiUrl, Issue, Page, PageToken, Paginated, PerPage, Repository,
};

/// { true }
/// fn parse_repository_page(body: &[u8]) -> Result<`Vec<Repository>`, Error>
/// { Ok(_) ==> ret is valid `Vec<Repository>` }
///
/// Parses a JSON byte slice as a list of repositories.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), rust_http_client::Error> {
/// let json = br#"[{"id":1,"name":"foo","full_name":"bar/foo","html_url":"https://github.com/bar/foo","stargazers_count":0}]"#;
/// let repos = rust_http_client::parse_repository_page(json)?;
/// assert_eq!(repos.len(), 1);
/// assert_eq!(repos.first().map(|r| r.name.as_str()), Some("foo"));
/// # Ok(())
/// # }
/// ```
pub fn parse_repository_page(body: &[u8]) -> Result<Vec<Repository>, Error> {
    Ok(serde_json::from_slice(body)?)
}

/// { true }
/// fn parse_issue_page(body: &[u8]) -> Result<`Vec<Issue>`, Error>
/// { Ok(_) ==> ret is valid `Vec<Issue>` }
///
/// Parses a JSON byte slice as a list of issues.
///
/// # Examples
///
/// ```
/// # fn main() -> Result<(), rust_http_client::Error> {
/// let json = br#"[{"id":1,"number":42,"title":"bug","state":"open"}]"#;
/// let issues = rust_http_client::parse_issue_page(json)?;
/// assert_eq!(issues.len(), 1);
/// assert_eq!(issues.first().map(|i| i.number), Some(42));
/// # Ok(())
/// # }
/// ```
pub fn parse_issue_page(body: &[u8]) -> Result<Vec<Issue>, Error> {
    Ok(serde_json::from_slice(body)?)
}
