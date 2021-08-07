use serde::Deserialize;
use reqwest::{Client};

struct OAuth2Config {
    auth_server_auth_endpoint: Option<String>,
    auth_server_token_endpoint: Option<String>,
    client_id: Option<String>,
    client_secret: Option<String>,
    scopes: Option<Vec<String>>,
    username: Option<String>,
    password: Option<String>,
    grant_type: GrantType,
}

impl OAuth2Config {
    fn auth_server_auth_endpoint(&self) -> Result<String, AccessTokenError> {
        ok_or(self.auth_server_auth_endpoint.clone(), "auth_server_auth_endpoint")
    }

    fn auth_server_token_endpoint(&self) -> Result<String, AccessTokenError> {
        ok_or(self.auth_server_token_endpoint.clone(), "auth_server_token_endpoint")
    }

    fn client_id(&self) -> Result<String, AccessTokenError> {
        ok_or(self.client_id.clone(), "client_id")
    }

    fn client_secret(&self) -> Result<String, AccessTokenError> {
        ok_or(self.client_secret.clone(), "client_secret")
    }

    fn username(&self) -> Result<String, AccessTokenError> {
        ok_or(self.username.clone(), "username")
    }

    fn password(&self) -> Result<String, AccessTokenError> {
        ok_or(self.password.clone(), "password")
    }
}


fn ok_or<T>(v: Option<T>, fname: &str) -> Result<T, AccessTokenError> {
    v.ok_or(AccessTokenError::InvalidConfig(fname.to_string()))
}


#[derive(Deserialize)]
pub struct AccessToken {
    access_token: String,
    token_type: String,
    refresh_token: Option<String>,
    expires_in: u64,
    scope: Option<Vec<String>>,
    id_token: Option<String>,
}

enum AccessTokenError {
    InvalidConfig(String),
    HttpError(reqwest::Error),
}

impl From<reqwest::Error> for AccessTokenError {
    fn from(e: reqwest::Error) -> Self {
        AccessTokenError::HttpError(e)
    }
}

pub enum GrantType {
    Password,
    AuthorizationCode,
    ClientCredentials,
}


impl GrantType {

    pub async fn get_access_token(&self, config: &OAuth2Config, http: &Client) -> Result<AccessToken, AccessTokenError> {
        let scopes: &str = &ok_or(config.scopes.clone(), "scopes").map(|s| s.join(" "))?;
        let res = match self {
            GrantType::Password =>
                http
                    .post(config.auth_server_token_endpoint()?)
                    .basic_auth(config.client_id()?, config.client_secret.clone())
                    .form(&[
                        ("grant_type", "password"),
                        ("scope", scopes),
                        ("username", &config.username()?),
                        ("password", &config.password()?),
                    ]),
            GrantType::ClientCredentials =>
                http
                    .post(config.auth_server_token_endpoint()?)
                    .basic_auth(config.client_id()?, config.client_secret.clone())
                    .form(&[
                        ("grant_type", "client_credentials"),
                        ("scope", scopes)
                    ]),
            _ => todo!()
        }.send().await?;
        res.json().await.map_err(|e| AccessTokenError::HttpError(e))
    }
}

