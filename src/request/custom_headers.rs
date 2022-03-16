use reqwest::header::{CONTENT_TYPE, USER_AGENT};
use reqwest::RequestBuilder;

use super::error::RequestError;
use super::headers::Headers;
use super::modifier::RequestModifier;
use crate::oauth2::OAuth2Config;
use crate::options::Opts;
use crate::version;

pub struct CustomHeaders {}

impl RequestModifier for CustomHeaders {
    fn modify(
        self,
        request: RequestBuilder,
        opts: &Opts,
        oauth2: &OAuth2Config,
    ) -> Result<RequestBuilder, RequestError> {
        let mut headers = Headers::with_capacity(opts.header.len());
        for h in opts.header.clone() {
            let kv = h.split(',').collect::<Vec<_>>();
            if kv.len() == 2 {
                if let (Some(k), Some(v)) = (kv.get(0), kv.get(1)) {
                    headers.add((*k).to_string(), (*v).to_string());
                }
            }
        }
        //set user agent
        headers.add_if_absent(
            USER_AGENT.to_string(),
            oauth2
                .default_user_agent
                .clone()
                .unwrap_or_else(version::name),
        );

        // set content-type
        headers.add_if_absent_opt(
            CONTENT_TYPE.to_string(),
            oauth2.default_content_type.clone(),
        );
        headers.modify(request, opts, oauth2)
    }
}
