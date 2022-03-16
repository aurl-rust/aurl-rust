use reqwest::RequestBuilder;

use crate::oauth2::OAuth2Config;
use crate::options::Opts;
use crate::request::error::RequestError;
use crate::request::modifier::RequestModifier;
pub struct Body {}

impl Body {
  pub fn new()-> Body {
    Body{}
  }
}

impl RequestModifier for Body {
    fn modify(
        self,
        request: RequestBuilder,
        opts: &Opts,
        _oauth2: &OAuth2Config,
    ) -> Result<RequestBuilder, RequestError> {
      if let Some(b) = &opts.data {
        let body = b.clone();
        Ok(request.body(body))
      } else {
        Ok(request)
      }
    }
}
