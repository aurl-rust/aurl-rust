use crate::oauth2::OAuth2Config;
use crate::options::Opts;
use crate::request::error::RequestError;
use crate::request::modifier::RequestModifier;
use reqwest::header::HeaderMap;
use reqwest::RequestBuilder;
use std::collections::HashMap;
use std::convert::TryInto;

pub struct Headers {
    hm: HashMap<String, String>,
}

impl Headers {
    pub fn add(&mut self, name: String, value: String) {
        self.hm.insert(name, value);
    }

    pub fn add_if_absent(&mut self, name: String, default_value: String) {
        if !self.hm.contains_key(&name) {
            self.hm.insert(name, default_value);
        }
    }

    pub fn add_if_absent_opt(&mut self, name: String, default_value: Option<String>) {
        if let Some(val) = default_value {
            self.add_if_absent(name, val);
        }
    }

    pub fn with_capacity(cap: usize) -> Headers {
        Headers {
            hm: HashMap::with_capacity(cap),
        }
    }
}

impl RequestModifier for Headers {
    fn modify(
        self,
        request: RequestBuilder,
        _: &Opts,
        _: &OAuth2Config,
    ) -> Result<RequestBuilder, RequestError> {
        let headers: HeaderMap = self.try_into()?;
        Ok(request.headers(headers))
    }
}

impl TryInto<HeaderMap> for Headers {
    type Error = RequestError;
    fn try_into(self) -> Result<HeaderMap, Self::Error> {
        (&self.hm)
            .try_into()
            .map_err(|e| RequestError::InvalidHeader(format!("{:?}", e)))
    }
}
