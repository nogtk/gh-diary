use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, AUTHORIZATION};
use anyhow::Result;
use crate::configuration;

pub fn get(request_url: &str) -> Result<reqwest::blocking::Response> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .get(request_url)
        .headers(request_headers_constructor())
        .send()?;

    Ok(response)
}

fn request_headers_constructor() -> HeaderMap {
    let mut header = HeaderMap::new();

    header.insert(
        USER_AGENT, HeaderValue::from_static("foo")
    );
    header.insert(
        AUTHORIZATION, HeaderValue::from_str(&tokenize_for_auth()).unwrap()
    );

    header
}

fn tokenize_for_auth() -> String {
    let base = "Token ".to_string();
    let token = configuration::github_pat();
    format!("{}{}", base, token)
}

#[cfg(test)]
mod test {
    use crate::client::*;

    #[test]
    fn contains_key_request_headers_constructor() {
        let result = request_headers_constructor();
        assert!(result.contains_key(USER_AGENT));
        assert!(result.contains_key(AUTHORIZATION));
    }

    #[test]
    fn user_agent_value_request_headers_constructor() {
        let result = &request_headers_constructor()[USER_AGENT];
        assert_eq!(result, "foo");
    }

    #[test]
    fn success_tokenize_for_auth() {
        std::env::remove_var("GITHUB_PAT");
        std::env::set_var("GITHUB_PAT", "TestToken");

        let result = tokenize_for_auth();
        assert_eq!(result, "Token TestToken");
    }
}
