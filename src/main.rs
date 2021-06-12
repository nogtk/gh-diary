mod util;
mod client;
mod pull_request;

use reqwest::Error;

fn main() -> Result<(), Error> {
    let nogtk_prs = pull_request::list_by_author("nogtk".to_string()).unwrap();
    println!("{:?}", nogtk_prs);

    Ok(())
}
