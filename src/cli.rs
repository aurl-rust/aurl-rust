use std::fmt::Display;

use log::{debug, info};
use reqwest::Client;

use options::Opts;

use crate::options;
use crate::profile::{read_profiles, InvalidConfig as InvalidConfigError, Profile};
use crate::request::{same_origin_redirect_policy, Dispatcher, RequestError, Response};

pub enum AppError {
    ProfileNotFound(String),
    InvalidConfig(InvalidConfigError),
    RequestError(RequestError),
}

impl Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AppError::ProfileNotFound(s) => write!(f, "Not Found Profile. {}", s),
            AppError::InvalidConfig(e) => write!(f, "Invalid Configuration. {}", e),
            AppError::RequestError(e) => write!(f, "{}", e),
        }
    }
}

pub async fn execute(opts: Opts) -> Result<(), AppError> {
    debug!("{:?}", opts);
    let profile = Profile::new(opts.profile.as_str());

    let profiles = read_profiles().map_err(AppError::InvalidConfig)?;
    let config = profiles
        .get(&profile.name)
        .ok_or(AppError::ProfileNotFound(profile.name))?;
    let client = Client::builder()
        .redirect(same_origin_redirect_policy())
        .build()
        .unwrap();
    let dispatcher = Dispatcher { client };

    let res = dispatcher
        .send(&opts, config)
        .await
        .map_err(AppError::RequestError)?;

    let body = match res {
        Response::Dispatched(r) => r.text().await,
        Response::SnippetGenerated(snippet) => Ok(snippet),
    }
    .map_err(|e| AppError::RequestError(RequestError::Http(e)))?;
    info!("{:}", body);
    Ok(())
}
