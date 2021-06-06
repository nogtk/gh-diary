mod util;
mod client;
mod pull_request;

use reqwest::Error;
use crate::pull_request::User;

fn main() -> Result<(), Error> {
    let prs = pull_request::list().unwrap();
    let users = prs
        .iter()
        .map(|pr| pr.user.clone())
        .collect::<Vec<User>>();
    println!("{:?}", users);
    println!("{:?}", prs);
    Ok(())
}
