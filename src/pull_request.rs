use crate::util::from_env;
use crate::client;
use serde::Deserialize;
use anyhow::Result;

#[derive(Deserialize, Debug)]
pub struct PullRequest {
    html_url: String,
}

pub fn list() -> Result<Vec<PullRequest>> {
    let response = client::get(&request_url_constructor()).unwrap();
    Ok(response.json::<Vec<PullRequest>>()?)
}

fn request_url_constructor() -> String {
    let owner = from_env("GITHUB_REPO_OWNER");
    let repository = from_env("GITHUB_REPO_NAME");
    format!("https://api.github.com/repos/{owner}/{repo}/pulls",
            owner = owner,
            repo = repository
    )
}
