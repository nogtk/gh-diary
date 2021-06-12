mod util;
mod client;
mod pull_request;

use reqwest::Error;

fn main() -> Result<(), Error> {
    let nogtk_prs = pull_request::list_by_author_md("nogtk".to_string()).unwrap();
    nogtk_prs.iter().for_each(|pr| println!("{}", pr));

    Ok(())
}
