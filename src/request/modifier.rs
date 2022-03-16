use crate::oauth2::{AccessToken, OAuth2Config};
use crate::options::Opts;
use crate::request::error::RequestError;

use crate::request::auth_header::CustomAuthHeader;
use crate::request::custom_headers::CustomHeaders;
use crate::request::timeout::Timeout;
use reqwest::RequestBuilder;

pub trait RequestModifier {
    fn modify(
        self,
        request: RequestBuilder,
        opts: &Opts,
        oauth2: &OAuth2Config,
    ) -> Result<RequestBuilder, RequestError>;
}

pub fn custom_headers() -> impl RequestModifier {
    CustomHeaders {}
}

pub fn auth_header(token: AccessToken) -> impl RequestModifier {
    CustomAuthHeader::from(token)
}

pub fn timeout() -> impl RequestModifier {
    Timeout::new()
}
