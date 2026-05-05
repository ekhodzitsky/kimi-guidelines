use crate::error::Error;
use crate::types::{ApiKey, ApiUrl, Issue, Page, PageToken, PerPage, Repository, Paginated};

/// Marker type for an unauthenticated client.
#[derive(Debug)]
pub struct Unauthenticated;

/// Marker type for an authenticated client.
#[derive(Debug)]
pub struct Authenticated {
    token: ApiKey,
}

/// HTTP client for the GitHub API with type-state authentication.
///
/// Use [`GitHubClient::new`] to create an unauthenticated client,
/// then call [`GitHubClient::authenticate`] to transition to an
/// authenticated client.
#[derive(Debug)]
pub struct GitHubClient<State> {
    client: reqwest::Client,
    base_url: ApiUrl,
    state: State,
}

impl GitHubClient<Unauthenticated> {
    /// { true }
    /// fn new(base_url: ApiUrl) -> Result<`GitHubClient<Unauthenticated>`, Error>
    /// { Ok(_) ==> ret.base_url.as_str() == base_url.as_str() }
    ///
    /// Creates a new unauthenticated client.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), rust_http_client::Error> {
    /// let client = rust_http_client::GitHubClient::new(
    ///     rust_http_client::ApiUrl::new("https://api.github.com")?
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(base_url: ApiUrl) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        Ok(Self {
            client,
            base_url,
            state: Unauthenticated,
        })
    }

    /// { true }
    /// fn authenticate(self, token: ApiKey) -> `GitHubClient<Authenticated>`
    /// { ret.state.token.as_str() == token.as_str() && ret.base_url.as_str() == self.base_url.as_str() }
    ///
    /// Transitions an unauthenticated client to an authenticated client.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), rust_http_client::Error> {
    /// let client = rust_http_client::GitHubClient::new(
    ///     rust_http_client::ApiUrl::new("https://api.github.com")?
    /// )?;
    /// let auth = client.authenticate(rust_http_client::ApiKey::new("ghp_xxx")?);
    /// # Ok(())
    /// # }
    /// ```
    pub fn authenticate(self, token: ApiKey) -> GitHubClient<Authenticated> {
        GitHubClient {
            client: self.client,
            base_url: self.base_url,
            state: Authenticated { token },
        }
    }
}

impl GitHubClient<Authenticated> {
    /// { true }
    /// fn with_token(base_url: ApiUrl, token: ApiKey) -> Result<`GitHubClient<Authenticated>`, Error>
    /// { Ok(_) ==> ret.state.token.as_str() == token.as_str() && ret.base_url.as_str() == base_url.as_str() }
    ///
    /// Creates a new authenticated client directly.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), rust_http_client::Error> {
    /// let client = rust_http_client::GitHubClient::with_token(
    ///     rust_http_client::ApiUrl::new("https://api.github.com")?,
    ///     rust_http_client::ApiKey::new("ghp_xxx")?
    /// )?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn with_token(base_url: ApiUrl, token: ApiKey) -> Result<Self, Error> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;
        Ok(Self {
            client,
            base_url,
            state: Authenticated { token },
        })
    }

    /// { true }
    /// fn token(&self) -> &ApiKey
    /// { ret.as_str() == self.state.token.as_str() }
    ///
    /// Returns a reference to the API token.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # fn main() -> Result<(), rust_http_client::Error> {
    /// let client = rust_http_client::GitHubClient::with_token(
    ///     rust_http_client::ApiUrl::new("https://api.github.com")?,
    ///     rust_http_client::ApiKey::new("ghp_xxx")?
    /// )?;
    /// assert_eq!(client.token().as_str(), "ghp_xxx");
    /// # Ok(())
    /// # }
    /// ```
    pub fn token(&self) -> &ApiKey {
        &self.state.token
    }

    /// { page.get() > 0 && per_page.get() > 0 && per_page.get() <= 100 }
    /// async fn list_public_repos(&self, page: Page, per_page: PerPage) -> Result<`Paginated<Repository>`, Error>
    /// { Ok(_) ==> ret.items.len() <= per_page.get() as usize }
    ///
    /// Lists public repositories with pagination.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), rust_http_client::Error> {
    /// let client = rust_http_client::GitHubClient::with_token(
    ///     rust_http_client::ApiUrl::new("https://api.github.com")?,
    ///     rust_http_client::ApiKey::new("ghp_xxx")?
    /// )?;
    /// let page = client.list_public_repos(
    ///     rust_http_client::Page::new(1)?,
    ///     rust_http_client::PerPage::new(30)?
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_public_repos(
        &self,
        page: Page,
        per_page: PerPage,
    ) -> Result<Paginated<Repository>, Error> {
        let base = self.base_url.as_str();
        let page_num = page.get();
        let count = per_page.get();
        let url = format!("{base}/repositories?page={page_num}&per_page={count}");

        let token = self.state.token.as_str();
        let request = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {token}"))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "rust-http-client");

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            let items: Vec<Repository> = response.json().await?;
            let next = if items.len() == per_page.get() as usize {
                PageToken::new(Some(format!("{}", page.get() + 1)))
            } else {
                PageToken::default()
            };
            Ok(Paginated {
                items,
                next_page_token: next,
            })
        } else {
            let body = response.text().await?;
            Err(Error::Http(status.as_u16(), body))
        }
    }

    /// { page.get() > 0 && per_page.get() > 0 && per_page.get() <= 100 }
    /// async fn list_issues(&self, owner: &str, repo: &str, page: Page, per_page: PerPage) -> Result<`Paginated<Issue>`, Error>
    /// { Ok(_) ==> ret.items.len() <= per_page.get() as usize }
    ///
    /// Lists issues for a repository.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # async fn example() -> Result<(), rust_http_client::Error> {
    /// let client = rust_http_client::GitHubClient::with_token(
    ///     rust_http_client::ApiUrl::new("https://api.github.com")?,
    ///     rust_http_client::ApiKey::new("ghp_xxx")?
    /// )?;
    /// let issues = client.list_issues(
    ///     "octocat",
    ///     "hello-world",
    ///     rust_http_client::Page::new(1)?,
    ///     rust_http_client::PerPage::new(30)?
    /// ).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list_issues(
        &self,
        owner: &str,
        repo: &str,
        page: Page,
        per_page: PerPage,
    ) -> Result<Paginated<Issue>, Error> {
        let base = self.base_url.as_str();
        let page_num = page.get();
        let count = per_page.get();
        let url = format!("{base}/repos/{owner}/{repo}/issues?page={page_num}&per_page={count}");

        let token = self.state.token.as_str();
        let request = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {token}"))
            .header("Accept", "application/vnd.github+json")
            .header("User-Agent", "rust-http-client");

        let response = request.send().await?;
        let status = response.status();

        if status.is_success() {
            let items: Vec<Issue> = response.json().await?;
            let next = if items.len() == per_page.get() as usize {
                PageToken::new(Some(format!("{}", page.get() + 1)))
            } else {
                PageToken::default()
            };
            Ok(Paginated {
                items,
                next_page_token: next,
            })
        } else {
            let body = response.text().await?;
            Err(Error::Http(status.as_u16(), body))
        }
    }
}
