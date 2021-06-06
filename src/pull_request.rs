use crate::util::from_env;
use crate::client;
use serde::Deserialize;
use anyhow::Result;

#[derive(Deserialize, Debug)]
pub struct PullRequest {
    html_url: String,
}

pub fn list() -> Result<Vec<PullRequest>> {
    let owner = from_env("GITHUB_REPO_OWNER");
    let repository = from_env("GITHUB_REPO_NAME");
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/pulls",
                              owner = owner,
                              repo = repository);
    let response = client::get(&request_url).unwrap();
    Ok(response.json::<Vec<PullRequest>>()?)
}
