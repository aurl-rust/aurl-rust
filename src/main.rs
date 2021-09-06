use reqwest::Client;
use tokio;

use crate::profile::Profile;

mod oauth2;
mod profile;
mod access_token_cache;
mod request;
mod options;


#[tokio::main]
async fn main() -> Result<(), ()> {
    let opts = options::parse_opts();
    print!("{:?}", opts);
    let profile = Profile::new(&opts.profile.as_str());
    let profiles = profile::read_profiles().unwrap_or_else(|e| std::panic::panic_any(format!("{:?}", e)));
    let config = profiles.get(&profile.name).unwrap();
    let client = Client::new();
    let dispatcher = request::Dispatcher { client };

    //TODO better handling Err(...)
    let res = dispatcher.send(&opts, config).await.unwrap();
    let body = res.text().await.unwrap();
    println!("{:?}", body);
    Ok(())
}
