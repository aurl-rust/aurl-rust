use crate::oauth2::{AccessToken, AccessTokenError};
use log::debug;
use reqwest::header::{HeaderName, HeaderValue};
use reqwest::RequestBuilder;

use super::modifier::RequestModifier;
use super::RequestError;
use crate::oauth2::OAuth2Config;
use crate::options::Opts;
use std::str::FromStr;

pub struct CustomAuthHeader {
    token: AccessToken,
}

impl CustomAuthHeader {
    pub fn from(token: AccessToken) -> CustomAuthHeader {
        CustomAuthHeader { token }
    }
}

impl RequestModifier for CustomAuthHeader {
    fn modify(
        self,
        request: RequestBuilder,
        _opts: &Opts,
        oauth2: &OAuth2Config,
    ) -> Result<reqwest::RequestBuilder, RequestError> {
        if let Some(auth_custom_header) = &oauth2.default_auth_header_template {
            debug!("use custom auth header name({})", auth_custom_header);
            let (header, value) = split_custom_header(auth_custom_header, &self.token.access_token)
                .expect("Invalid custom header configuration");
            Ok(request.header(
                HeaderName::from_str(header).expect("Failed set header"),
                HeaderValue::from_str(&value).expect("Failed set header value"),
            ))
        } else {
            Ok(request.bearer_auth(self.token.access_token))
        }
    }
}

fn split_custom_header<'a>(
    template: &'a str,
    access_token: &'a str,
) -> Result<(&'a str, String), AccessTokenError> {
    let split: Vec<&str> = template.split('=').collect();
    if split.len() != 2 {
        debug!("Failed parse custom_header_template, {}", template);
        Err(AccessTokenError::InvalidConfig(
            "invalid custom_header_template".to_string(),
        ))
    } else if !split[1].to_lowercase().contains("$token") {
        Err(AccessTokenError::InvalidConfig(
            "can't find '$token' placeholder".to_string(),
        ))
    } else {
        let value = split[1]
            .trim()
            .to_lowercase()
            .replace("$token", access_token);
        Ok((split[0], value))
    }
}
