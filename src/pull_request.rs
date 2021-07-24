use crate::{client, configuration};
use serde::Deserialize;
use anyhow::Result;
use chrono::{DateTime, Local};

#[derive(Deserialize, Debug, Clone)]
pub struct PullRequest {
    html_url: String,
    created_at: String,
    title: String,
    number: u64,
    user: User
}

#[derive(Deserialize, Debug, Clone)]
pub struct ReviewedPullRequest {
    state: String,
    submitted_at: String,
    user: User
}

#[derive(Deserialize, Debug, Clone)]
pub struct User {
    pub(crate) login: String
}

pub fn created_list_by_author_md(author: String) -> Result<Vec<String>> {
    let mut md_result: Vec<String> = Vec::new();
    let prs = list_by_author(author).unwrap();
    for pr in prs.iter() {
        let buf = format!("- [{}]({})", &pr.title, &pr.html_url);
        md_result.push(buf);
    }

    Ok(md_result)
}

pub fn reviewed_list_by_reviewer_md(reviewer: String) -> Result<Vec<String>> {
    let mut md_result: Vec<String> = Vec::new();
    let prs = reviewed_list(reviewer).unwrap();
    for pr in prs.iter() {
        let buf = format!("- [{}]({})", &pr.title, &pr.html_url);
        md_result.push(buf);
    }
    Ok(md_result)
}

fn list() -> Result<Vec<PullRequest>> {
    let response = client::get(request_url_constructor().as_str()).unwrap();
    Ok(response.json::<Vec<PullRequest>>()?)
}

fn list_by_author(author: String) -> Result<Vec<PullRequest>> {
    let prs = list()
        .unwrap()
        .iter()
        .filter(|pr|
            pr.user.login == author
            && DateTime::parse_from_rfc3339(&pr.created_at).unwrap().with_timezone(&Local).date() == Local::today()
        )
        .cloned()
        .collect();
    Ok(prs)
}

fn reviewed_list(reviewer: String) -> Result<Vec<PullRequest>> {
    let prs = list()
        .unwrap()
        .iter()
        .filter(|pr| is_reviewed(pr.number, &reviewer).unwrap())
        .cloned()
        .collect();
    Ok(prs)
}

fn is_reviewed(pr_number: u64, reviewer: &String) -> Result<bool> {
    let reviews_json = reviews(pr_number).unwrap();
    let reviewed = reviews_json
        .iter()
        .any(|pr|
            &pr.user.login == reviewer
                && DateTime::parse_from_rfc3339(&pr.submitted_at).unwrap().with_timezone(&Local).date() == Local::today()
        );
    Ok(reviewed)
}

pub fn reviews(pr_number: u64) -> Result<Vec<ReviewedPullRequest>> {
    let response = client::get(reviews_url_constructor(pr_number).as_str()).unwrap();
    Ok(response.json::<Vec<ReviewedPullRequest>>()?)
}

fn request_url_constructor() -> reqwest::Url {
    let owner = configuration::repo_owner();
    let repository = configuration::repo_name();
    let host = format!("https://api.github.com/repos/{owner}/{repo}/pulls",
            owner = owner,
            repo = repository
    );
    reqwest::Url::parse_with_params(&host, &query_strings_array()).unwrap()
}

fn reviews_url_constructor(pr_number: u64) -> reqwest::Url {
    let owner = configuration::repo_owner();
    let repository = configuration::repo_name();
    let url = format!("https://api.github.com/repos/{owner}/{repo}/pulls/{pr_number}/reviews",
                       owner = owner,
                       repo = repository,
                       pr_number = pr_number,
    );
    reqwest::Url::parse(&url).unwrap()
}

fn query_strings_array() -> [(&'static str, &'static str); 2] {
    [("per_page", "50"), ("state", "all")]
}
