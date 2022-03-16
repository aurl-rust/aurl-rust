use crate::oauth2::AccessTokenError;
use std::fmt::Display;

#[derive(Debug)]
pub enum RequestError {
    OAuth(AccessTokenError),
    Http(reqwest::Error),
    InvalidHeader(String),
}

impl std::error::Error for RequestError {}

impl Display for RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RequestError::OAuth(e) => write!(f, "OAuth Process Error. {}", e),
            RequestError::Http(e) => write!(f, "Http Process Error. {}", e),
            RequestError::InvalidHeader(s) => write!(f, "Invalid Request Header. {}", s),
        }
    }
}
