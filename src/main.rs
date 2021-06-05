use serde::Deserialize;
use reqwest::Error;
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT, AUTHORIZATION};

#[derive(Deserialize, Debug)]
struct PullRequest {
    html_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let owner = from_env("GITHUB_REPO_OWNER");
    let repository = from_env("GITHUB_REPO_NAME");
    let request_url = format!("https://api.github.com/repos/{owner}/{repo}/pulls",
                              owner = owner,
                              repo = repository);
    let mut header_map = HeaderMap::new();
    header_map.insert(
        USER_AGENT, HeaderValue::from_static("foo")
    );
    let base = "Token ".to_string();
    let token = from_env("GITHUB_TOKEN");
    let authorization_header = format!("{}{}", base, token);
    header_map.insert(
        AUTHORIZATION, HeaderValue::from_str(&authorization_header).unwrap()
    );
    let client = reqwest::Client::new();
    let res = client.get(&request_url).headers(header_map).send().await?;
    let prs: Vec<PullRequest> = res.json().await?;
    println!("{:?}", prs);
    Ok(())
}

fn from_env(name: &str) -> String {
    match std::env::var(name) {
        Ok(v) => v,
        Err(err) => {
            println!("{}: {}", err, name);
            std::process::exit(1);
        }
    }
}
