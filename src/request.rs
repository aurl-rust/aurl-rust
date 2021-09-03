use reqwest::{RequestBuilder, ClientBuilder, Result, Response};
use crate::oauth2::AccessToken;

pub async fn send(req:RequestBuilder, access_token:AccessToken) -> Result<Response> {
    reqwest::ClientBuilder::default().build()?.execute(
        req
            .bearer_auth(access_token.access_token.clone())
            .build()?).await
}