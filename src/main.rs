mod util;
mod client;
mod pull_request;
mod issue;
mod configuration;

use reqwest::Error;

fn main() -> Result<(), Error> {
    println!("### Today's created PRs");
    let nogtk_prs = pull_request::created_list_by_author_md("nogtk".to_string()).unwrap();
    nogtk_prs.iter().for_each(|pr| println!("{}", pr));

    println!("### Today's reviewed PRs");
    let reviewed_by_nogtk_prs = pull_request::reviewed_list_by_reviewer_md("nogtk".to_string()).unwrap();
    reviewed_by_nogtk_prs.iter().for_each(|pr| println!("{}", pr));

    println!("### Today's created issues");
    let nogtk_issues = issue::created_list_by_author_md("nogtk".to_string()).unwrap();
    nogtk_issues.iter().for_each(|issue| println!("{}", issue));

    println!("### Today's closed issues");
    let nogtk_closed_issues = issue::closed_list_by_author_md("nogtk".to_string()).unwrap();
    nogtk_closed_issues.iter().for_each(|issue| println!("{}", issue));

    Ok(())
}
