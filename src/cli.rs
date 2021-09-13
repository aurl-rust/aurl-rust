use log::{debug, info};
use options::Opts;
use reqwest::Client;

use crate::options;
use crate::profile::{read_profiles, InvalidConfig as InvalidConfigError, Profile};
use crate::request::{Dispatcher, RequestError};

pub enum AppError {
    ProfileNotFound(String),
    InvalidConfig(InvalidConfigError),
    RequestError(RequestError),
}

pub async fn execute(opts: Opts) -> Result<(), AppError> {
    debug!("{:?}", opts);
    let profile = Profile::new(&opts.profile.as_str());

    let profiles = read_profiles().map_err(|e| AppError::InvalidConfig(e))?;
    let config = profiles
        .get(&profile.name)
        .ok_or(AppError::ProfileNotFound(profile.name))?;
    let client = Client::new();
    let dispatcher = Dispatcher { client };

    let res = dispatcher
        .send(&opts, config)
        .await
        .map_err(|e| AppError::RequestError(e))?;
    let body = res
        .text()
        .await
        .map_err(|e| AppError::RequestError(RequestError::HttpError(e)))?;
    info!("{:}", body);
    Ok(())
}
