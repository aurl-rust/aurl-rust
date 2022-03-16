use crate::oauth2::OAuth2Config;
use crate::options::Opts;
use crate::request::error::RequestError;
use crate::request::modifier::RequestModifier;
use reqwest::RequestBuilder;
use std::time::Duration;

pub struct Timeout {}

impl Timeout {
    pub fn new() -> Timeout {
        Timeout {}
    }
}

impl RequestModifier for Timeout {
    fn modify(
        self,
        request: RequestBuilder,
        opts: &Opts,
        _oauth2: &OAuth2Config,
    ) -> Result<reqwest::RequestBuilder, RequestError> {
        Ok(request.timeout(Duration::from_secs(opts.timeout)))
    }
}
