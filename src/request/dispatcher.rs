use log::{debug, warn};
use reqwest::{Client, StatusCode};

use crate::oauth2::{AccessToken, OAuth2Config};
use crate::options::Opts;
use crate::output::{Curl, Type, Output};
use crate::request::error::RequestError;
use crate::request::modifier::{auth_header, custom_headers, timeout, RequestModifier};
use crate::request::response::Response;
pub struct Dispatcher {
    pub client: Client,
}

impl Dispatcher {
    pub async fn send(&self, opts: &Opts, oauth2: &OAuth2Config) -> Result<Response, RequestError> {
        loop {
            let mut token = match AccessToken::load_cache(&opts.profile) {
                Some(t) => t,
                None => oauth2
                    .grant_type
                    .get_access_token(oauth2, opts.timeout, &self.client)
                    .await
                    .map_err(RequestError::OAuth)?,
            };
            debug!("Get Token: {}", token);

            // save token in the cache
            token
                .save_cache(&opts.profile)
                .unwrap_or_else(|err| warn!("can not save cache. {:?}", err));

            let mut req = self.client.request(opts.request.clone(), opts.url.clone());
            req = custom_headers().modify(req, opts, oauth2)?;
            req = auth_header(token).modify(req, opts, oauth2)?;
            req = timeout().modify(req, opts, oauth2)?;

            // output 指定があったら send 実行せずに return
            match &opts.output {
                Type::Curl => {
                    return Ok(Response::SnippetGenerated(Curl::output(
                        &req.build().unwrap(),
                    )))
                }
                Type::None => {
                    // output 指定が未指定 or 無効な場合
                    let res = req.send().await;
                    debug!("{:?}", res);
                    match res {
                        Err(e) if e.status() == Some(StatusCode::UNAUTHORIZED) => {
                            AccessToken::remove_cache(&opts.profile);
                        }
                        Err(e) => return Err(RequestError::Http(e)),
                        Ok(ok) => return Ok(Response::Dispatched(ok)),
                    }
                }
            }
        }
    }
}
