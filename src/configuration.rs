use crate::util::from_env;
use std::fs::File;
use std::io::prelude::*;

pub fn github_pat() -> String {
    let mut f = File::open(build_token_file_path()).expect("file not found");
    let mut token = String::new();
    f.read_to_string(&mut token).expect("something went wrong reading the file");

    token.trim().to_string()
}

pub fn repo_owner() -> String {
    let mut f = File::open(build_repo_owner_file_path()).expect("file not found");
    let mut owner = String::new();
    f.read_to_string(&mut owner).expect("something went wrong reading the file");

    owner.trim().to_string()
}

pub fn repo_name() -> String {
    let mut f = File::open(build_repo_name_file_path()).expect("file not found");
    let mut name = String::new();
    f.read_to_string(&mut name).expect("something went wrong reading the file");

    name.trim().to_string()
}

fn build_token_file_path() -> String {
    let mut path = from_env("HOME");
    path.push_str("/.config/gh_diary");
    path
}

fn build_repo_owner_file_path() -> String {
    let mut path = from_env("HOME");
    path.push_str("/.config/gh_diary_repo_owner");
    path
}

fn build_repo_name_file_path() -> String {
    let mut path = from_env("HOME");
    path.push_str("/.config/gh_diary_repo_name");
    path
}
