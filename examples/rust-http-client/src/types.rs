use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

use crate::error::Error;

/// A validated GitHub personal access token.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ApiKey(String);

impl ApiKey {
    /// { !token.trim().is_empty() }
    /// fn new(token: impl `Into<String>`) -> Result<ApiKey, Error>
    /// { Ok(_) ==> ret.0 == token.into().trim() }
    ///
    /// Creates a new API key from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), rust_http_client::Error> {
    /// let key = rust_http_client::ApiKey::new("ghp_xxx")?;
    /// assert_eq!(key.as_str(), "ghp_xxx");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(token: impl Into<String>) -> Result<Self, Error> {
        let s = token.into();
        let trimmed = s.trim();
        if trimmed.is_empty() {
            Err(Error::InvalidToken)
        } else {
            Ok(Self(trimmed.to_owned()))
        }
    }

    /// { true }
    /// fn as_str(&self) -> &str
    /// { ret == self.0.as_str() }
    ///
    /// Returns the key as a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), rust_http_client::Error> {
    /// let key = rust_http_client::ApiKey::new("secret")?;
    /// assert_eq!(key.as_str(), "secret");
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// A validated API base URL.
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct ApiUrl(String);

impl ApiUrl {
    /// { !url.trim().is_empty() }
    /// fn new(url: impl `Into<String>`) -> Result<ApiUrl, Error>
    /// { Ok(_) ==> ret.0.parse::<reqwest::Url>().is_ok() }
    ///
    /// Creates a new API URL from a string.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), rust_http_client::Error> {
    /// let url = rust_http_client::ApiUrl::new("https://api.github.com")?;
    /// assert_eq!(url.as_str(), "https://api.github.com/");
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(url: impl Into<String>) -> Result<Self, Error> {
        let s = url.into();
        let trimmed = s.trim();
        if trimmed.is_empty() {
            return Err(Error::InvalidUrl("empty string".to_owned()));
        }
        match trimmed.parse::<reqwest::Url>() {
            Ok(url) => Ok(Self(url.as_str().to_owned())),
            Err(e) => Err(Error::InvalidUrl(e.to_string())),
        }
    }

    /// { true }
    /// fn as_str(&self) -> &str
    /// { ret == self.0.as_str() }
    ///
    /// Returns the URL as a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), rust_http_client::Error> {
    /// let url = rust_http_client::ApiUrl::new("https://api.github.com")?;
    /// assert_eq!(url.as_str(), "https://api.github.com/");
    /// # Ok(())
    /// # }
    /// ```
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

/// A non-zero page number for pagination.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Page(NonZeroU32);

impl Page {
    /// { n > 0 }
    /// fn new(n: u32) -> Result<Page, Error>
    /// { Ok(_) ==> ret.0.get() == n }
    ///
    /// Creates a new page number.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), rust_http_client::Error> {
    /// let page = rust_http_client::Page::new(1)?;
    /// assert_eq!(page.get(), 1);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(n: u32) -> Result<Self, Error> {
        NonZeroU32::new(n)
            .map(Self)
            .ok_or(Error::InvalidPage)
    }

    /// { true }
    /// fn get(&self) -> u32
    /// { ret == self.0.get() }
    ///
    /// Returns the page number.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), rust_http_client::Error> {
    /// let page = rust_http_client::Page::new(5)?;
    /// assert_eq!(page.get(), 5);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get(&self) -> u32 {
        self.0.get()
    }
}

/// Items per page, clamped to 1..=100.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct PerPage(u8);

impl PerPage {
    /// { n > 0 && n <= 100 }
    /// fn new(n: u8) -> Result<PerPage, Error>
    /// { Ok(_) ==> ret.0 == n }
    ///
    /// Creates a new per-page value.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), rust_http_client::Error> {
    /// let per_page = rust_http_client::PerPage::new(30)?;
    /// assert_eq!(per_page.get(), 30);
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(n: u8) -> Result<Self, Error> {
        if n == 0 || n > 100 {
            Err(Error::InvalidPerPage(n))
        } else {
            Ok(Self(n))
        }
    }

    /// { true }
    /// fn get(&self) -> u8
    /// { ret == self.0 }
    ///
    /// Returns the per-page value.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() -> Result<(), rust_http_client::Error> {
    /// let per_page = rust_http_client::PerPage::new(50)?;
    /// assert_eq!(per_page.get(), 50);
    /// # Ok(())
    /// # }
    /// ```
    pub fn get(&self) -> u8 {
        self.0
    }
}

/// An opaque pagination token.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct PageToken(Option<String>);

impl PageToken {
    /// { true }
    /// fn new(token: Option<impl `Into<String>`>) -> PageToken
    /// { ret.0 == token.map(|t| t.into()) }
    ///
    /// Creates a new page token.
    ///
    /// # Examples
    ///
    /// ```
    /// let token = rust_http_client::PageToken::new(Some("abc123"));
    /// assert_eq!(token.as_str(), Some("abc123"));
    ///
    /// let empty = rust_http_client::PageToken::new(None::<&str>);
    /// assert_eq!(empty.as_str(), None);
    /// ```
    pub fn new(token: Option<impl Into<String>>) -> Self {
        Self(token.map(Into::into))
    }

    /// { true }
    /// fn as_str(&self) -> Option<&str>
    /// { ret == self.0.as_deref() }
    ///
    /// Returns the token as a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// let token = rust_http_client::PageToken::new(Some("xyz"));
    /// assert_eq!(token.as_str(), Some("xyz"));
    /// ```
    pub fn as_str(&self) -> Option<&str> {
        self.0.as_deref()
    }
}

/// A GitHub repository.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Repository {
    pub id: u64,
    pub name: String,
    pub full_name: String,
    pub html_url: String,
    pub description: Option<String>,
    pub stargazers_count: u64,
    pub language: Option<String>,
}

/// A GitHub issue.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Issue {
    pub id: u64,
    pub number: u32,
    pub title: String,
    pub state: String,
    pub body: Option<String>,
}

/// A paginated response wrapper.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Paginated<T> {
    pub items: Vec<T>,
    pub next_page_token: PageToken,
}
