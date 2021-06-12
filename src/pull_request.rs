use crate::util::from_env;
use crate::client;
use serde::Deserialize;
use anyhow::Result;

#[derive(Deserialize, Debug, Clone)]
pub struct PullRequest {
    html_url: String,
    created_at: String,
    user: User
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    login: String
}

pub fn list() -> Result<Vec<PullRequest>> {
    let response = client::get(request_url_constructor().as_str()).unwrap();
    Ok(response.json::<Vec<PullRequest>>()?)
}

pub fn list_by_author(author: String) -> Result<Vec<PullRequest>> {
    let prs = list()
        .unwrap()
        .iter()
        .filter(|pr| pr.user.login == author)
        .cloned()
        .collect();
    Ok(prs)
}

fn request_url_constructor() -> reqwest::Url {
    let owner = from_env("GITHUB_REPO_OWNER");
    let repository = from_env("GITHUB_REPO_NAME");
    let host = format!("https://api.github.com/repos/{owner}/{repo}/pulls",
            owner = owner,
            repo = repository
    );
    reqwest::Url::parse_with_params(&host, &query_strings_array()).unwrap()
}

fn query_strings_array() -> [(&'static str, &'static str); 1] {
    [("per_page", "100")]
}
