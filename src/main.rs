mod util;
mod client;
mod pull_request;

use reqwest::Error;

fn main() -> Result<(), Error> {
    println!("### Today's created PRs");
    let nogtk_prs = pull_request::created_list_by_author_md("nogtk".to_string()).unwrap();
    nogtk_prs.iter().for_each(|pr| println!("{}", pr));

    println!("### Today's reviewed PRs");
    let reviewed_by_nogtk_prs = pull_request::reviewed_list_by_reviewer_md("nogtk".to_string()).unwrap();
    reviewed_by_nogtk_prs.iter().for_each(|pr| println!("{}", pr));

    Ok(())
}
