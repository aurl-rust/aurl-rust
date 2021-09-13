use std::collections::HashMap;
use std::convert::TryInto;

use reqwest::header::HeaderMap;
use reqwest::{Client, Response, StatusCode};

use crate::oauth2::{AccessTokenError, OAuth2Config};
use crate::options::Opts;

#[derive(Debug)]
pub enum RequestError {
    OAuthError(AccessTokenError),
    HttpError(reqwest::Error),
    InvalidHeaderError(String),
}

pub struct Dispatcher {
    pub client: Client,
}

impl Dispatcher {
    pub async fn send(&self, opts: &Opts, oauth2: &OAuth2Config) -> Result<Response, RequestError> {
        let mut hm: HashMap<String, String> = HashMap::with_capacity(opts.header.len());
        for h in opts.header.clone() {
            let kv = h.split(',').collect::<Vec<_>>();
            if kv.len() == 2 {
                match (kv.get(0), kv.get(1)) {
                    (Some(k), Some(v)) => {
                        hm.insert((*k).to_string(), (*v).to_string());
                    }
                    _ => (),
                }
            }
        }
        let headers: HeaderMap = (&hm)
            .try_into()
            .map_err(|e| RequestError::InvalidHeaderError(format!("{:?}", e)))?;

        loop {
            let token = oauth2
                .grant_type
                .get_access_token(&oauth2, &self.client)
                .await
                .map_err(|e| RequestError::OAuthError(e))?;
            let res = self
                .client
                .request(opts.request.clone(), opts.url.clone())
                .bearer_auth(token.access_token)
                .headers(headers.clone())
                .send()
                .await;
            match res {
                Ok(ok) => return Ok(ok),
                Err(e) if e.status().map_or(false, |s| s == StatusCode::UNAUTHORIZED) => (),
                Err(e) => return Err(RequestError::HttpError(e)),
            }
        }
    }
}
