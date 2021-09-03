use reqwest::Client;
use tokio;
use crate::profile::Profile;

mod oauth2;
mod profile;
mod access_token_cache;
mod request;
mod options;

#[tokio::main]
async fn main() {
    let profile = Profile::new("");
    let profile = profile::read_profiles(profile).unwrap_or_else(|_| panic!());
    let config = profile.get("").unwrap();
    let token = config.grant_type.get_access_token(config, &Client::new()).await;
    match token {
        Ok(t) => print!("{:?}", t),
        Err(e) => panic!("{:?}", e),
    }
}
