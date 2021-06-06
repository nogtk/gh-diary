use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, AUTHORIZATION};
use anyhow::Result;
use crate::util::from_env;

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
    let token = from_env("GITHUB_PAT");
    format!("{}{}", base, token)
}
