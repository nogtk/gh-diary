use anyhow::Result;
use crate::pull_request::User;
use chrono::{DateTime, Local};
use serde::Deserialize;
use crate::client;
use crate::util::from_env;

#[derive(Deserialize, Debug, Clone)]
struct Issue {
    html_url: String,
    title: String,
    state: String,
    created_at: String,
    closed_at: Option<String>,
    user: User
}

impl Issue {
    fn is_closed_on_today(&self) -> bool {
        match &self.closed_at {
            Some(closed_at) => self.state == "closed" &&
                DateTime::parse_from_rfc3339(closed_at.as_str()).unwrap().with_timezone(&Local).date() == Local::today(),
            None => false
        }
    }
}

pub fn created_list_by_author_md(author: String) -> Result<Vec<String>> {
    let mut md_result = Vec::new();
    let issues = list_by_author(author).unwrap();
    for issue in issues.iter() {
        let buf = format!("- [{}]({})", &issue.title, &issue.html_url);
        md_result.push(buf);
    }

    Ok(md_result)
}

pub fn closed_list_by_author_md(author: String) -> Result<Vec<String>> {
    let mut md_result = Vec::new();
    let issues = list_closed_by_author(author).unwrap();
    for issue in issues.iter() {
        let buf = format!("- [{}]({})", &issue.title, &issue.html_url);
        md_result.push(buf);
    }

    Ok(md_result)
}

fn list_by_author(author: String) -> Result<Vec<Issue>> {
    let issues = list(author)
        .unwrap()
        .iter()
        .filter(|issue|
            DateTime::parse_from_rfc3339(&issue.created_at).unwrap().with_timezone(&Local).date() == Local::today()
        )
        .cloned()
        .collect();

    Ok(issues)
}

fn list_closed_by_author(author: String) -> Result<Vec<Issue>> {
    let issues = list(author)
        .unwrap()
        .iter()
        .filter(|issue| issue.is_closed_on_today())
        .cloned()
        .collect();

    Ok(issues)
}

fn list(author: String) -> Result<Vec<Issue>> {
    let response = client::get(request_url_constructor(author).as_str()).unwrap();
    Ok(response.json::<Vec<Issue>>()?)
}

fn request_url_constructor(author: String) -> reqwest::Url {
    let owner = from_env("GITHUB_REPO_OWNER");
    let repository = from_env("GITHUB_REPO_NAME");
    let url = format!("https://api.github.com/repos/{owner}/{repo}/issues",
        owner = owner,
        repo = repository
    );
    reqwest::Url::parse_with_params(&url, &query_strings_array(author)).unwrap()
}

fn query_strings_array(author: String) -> [(&'static str, String); 2] {
    [("creator", author), ("state", "closed".to_string())]
}
