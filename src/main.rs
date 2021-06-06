mod util;
mod client;

use serde::Deserialize;
use reqwest::Error;
use util::from_env;

#[derive(Deserialize, Debug)]
struct PullRequest {
    html_url: String,
}

fn main() -> Result<(), Error> {
    let owner = from_env("GITHUB_REPO_OWNER");
    let repository = from_env("GITHUB_REPO_NAME");
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/pulls",
                              owner = owner,
                              repo = repository);
    let response = client::get(&request_url).unwrap();
    let prs: Vec<PullRequest> = response.json()?;
    println!("{:?}", prs);
    Ok(())
}
