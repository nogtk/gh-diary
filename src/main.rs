mod util;
mod client;
mod pull_request;

use reqwest::Error;

fn main() -> Result<(), Error> {
    println!("{:?}", pull_request::list().unwrap());
    Ok(())
}
