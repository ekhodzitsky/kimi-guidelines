use proptest::prelude::*;
use rust_http_client::{
    ApiKey, ApiUrl, Issue, Page, PageToken, PerPage, Repository,
};

#[test]
fn api_key_accepts_non_empty() -> Result<(), rust_http_client::Error> {
    let key = ApiKey::new("ghp_xxx")?;
    assert_eq!(key.as_str(), "ghp_xxx");
    Ok(())
}

#[test]
fn api_key_rejects_empty() {
    assert!(ApiKey::new("").is_err());
    assert!(ApiKey::new("   ").is_err());
}

#[test]
fn api_url_accepts_valid() -> Result<(), rust_http_client::Error> {
    let url = ApiUrl::new("https://api.github.com")?;
    assert_eq!(url.as_str(), "https://api.github.com/");
    Ok(())
}

#[test]
fn api_url_rejects_empty() {
    assert!(ApiUrl::new("").is_err());
}

#[test]
fn page_rejects_zero() {
    assert!(Page::new(0).is_err());
}

#[test]
fn page_accepts_positive() -> Result<(), rust_http_client::Error> {
    let page = Page::new(1)?;
    assert_eq!(page.get(), 1);
    Ok(())
}

#[test]
fn per_page_rejects_out_of_range() {
    assert!(PerPage::new(0).is_err());
    assert!(PerPage::new(101).is_err());
}

#[test]
fn per_page_accepts_valid() -> Result<(), rust_http_client::Error> {
    let pp = PerPage::new(30)?;
    assert_eq!(pp.get(), 30);
    Ok(())
}

#[test]
fn page_token_roundtrips() {
    let token = PageToken::new(Some("abc"));
    assert_eq!(token.as_str(), Some("abc"));
    let empty = PageToken::new(None::<&str>);
    assert_eq!(empty.as_str(), None);
}

#[test]
fn client_state_transition() -> Result<(), rust_http_client::Error> {
    let client = rust_http_client::GitHubClient::new(ApiUrl::new("https://api.github.com")?)?;
    let auth = client.authenticate(ApiKey::new("token")?);
    assert_eq!(auth.token().as_str(), "token");
    Ok(())
}

#[test]
fn parse_repository_list() -> Result<(), rust_http_client::Error> {
    let json = br#"[{"id":1,"name":"foo","full_name":"bar/foo","html_url":"https://github.com/bar/foo","stargazers_count":42,"language":"Rust"}]"#;
    let repos = rust_http_client::parse_repository_page(json)?;
    assert_eq!(repos.len(), 1);
    assert_eq!(repos.first().map(|r| r.name.as_str()), Some("foo"));
    Ok(())
}

#[test]
fn parse_issue_list() -> Result<(), rust_http_client::Error> {
    let json = br#"[{"id":1,"number":42,"title":"bug","state":"open","body":null}]"#;
    let issues = rust_http_client::parse_issue_page(json)?;
    assert_eq!(issues.len(), 1);
    assert_eq!(issues.first().map(|i| i.number), Some(42));
    Ok(())
}

#[test]
fn paginated_repository_roundtrip() -> Result<(), rust_http_client::Error> {
    let paginated = rust_http_client::Paginated {
        items: vec![Repository {
            id: 1,
            name: "foo".to_owned(),
            full_name: "bar/foo".to_owned(),
            html_url: "https://github.com/bar/foo".to_owned(),
            description: None,
            stargazers_count: 0,
            language: None,
        }],
        next_page_token: PageToken::new(Some("next")),
    };
    let json = serde_json::to_string(&paginated)?;
    let parsed: rust_http_client::Paginated<Repository> = serde_json::from_str(&json)?;
    assert_eq!(paginated, parsed);
    Ok(())
}

proptest! {
    #[test]
    fn repository_json_roundtrip(
        id in 0u64..,
        name in "[a-zA-Z0-9_-]{1,20}",
        full_name in "[a-zA-Z0-9_/-]{1,40}",
        html_url in "https://github.com/[a-zA-Z0-9_/-]{1,40}",
        stars in 0u64..,
        lang in proptest::option::of("[a-zA-Z+]{1,10}")
    ) {
        let repo = Repository {
            id,
            name: name.clone(),
            full_name: full_name.clone(),
            html_url: html_url.clone(),
            description: None,
            stargazers_count: stars,
            language: lang.clone(),
        };
        let json_result = serde_json::to_string(&repo);
        let is_ok = json_result.is_ok();
        prop_assert!(is_ok, "serialization failed");
        let json = json_result.unwrap_or_default();

        let parsed_result = serde_json::from_str::<Repository>(&json);
        let is_ok = parsed_result.is_ok();
        prop_assert!(is_ok, "deserialization failed");
        let parsed = parsed_result.unwrap_or_default();

        prop_assert_eq!(repo, parsed);
    }

    #[test]
    fn issue_json_roundtrip(
        id in 0u64..,
        number in 0u32..,
        title in "[a-zA-Z0-9 ]{1,30}",
        state in "(open|closed)",
        body in proptest::option::of("[a-zA-Z0-9 ]{0,100}")
    ) {
        let issue = Issue {
            id,
            number,
            title: title.clone(),
            state: state.clone(),
            body: body.clone(),
        };
        let json_result = serde_json::to_string(&issue);
        let is_ok = json_result.is_ok();
        prop_assert!(is_ok, "serialization failed");
        let json = json_result.unwrap_or_default();

        let parsed_result = serde_json::from_str::<Issue>(&json);
        let is_ok = parsed_result.is_ok();
        prop_assert!(is_ok, "deserialization failed");
        let parsed = parsed_result.unwrap_or_default();

        prop_assert_eq!(issue, parsed);
    }

    #[test]
    fn api_key_preserves_valid_input(s in "[a-zA-Z0-9_]{1,100}") {
        match ApiKey::new(&s) {
            Ok(key) => prop_assert_eq!(key.as_str(), s),
            Err(_) => prop_assert!(false, "ApiKey::new failed for valid input"),
        }
    }

    #[test]
    fn page_preserves_nonzero(n in 1u32..) {
        match Page::new(n) {
            Ok(page) => prop_assert_eq!(page.get(), n),
            Err(_) => prop_assert!(false, "Page::new failed for valid input"),
        }
    }

    #[test]
    fn per_page_preserves_valid(n in 1u8..=100) {
        match PerPage::new(n) {
            Ok(pp) => prop_assert_eq!(pp.get(), n),
            Err(_) => prop_assert!(false, "PerPage::new failed for valid input"),
        }
    }
}
