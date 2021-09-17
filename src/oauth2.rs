use std::io;
use std::str::FromStr;

use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::oauth2::GrantType::{AuthorizationCode, ClientCredentials, Password};
use crate::profile::InvalidConfig;
use crate::version;
use reqwest::header::USER_AGENT;

pub struct OAuth2Config {
    pub auth_server_auth_endpoint: Option<String>,
    pub auth_server_token_endpoint: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
    pub scopes: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub grant_type: GrantType,
    pub redirect: Option<String>,
    pub default_content_type: Option<String>,
    pub default_user_agent: Option<String>,
}

impl OAuth2Config {
    #[allow(dead_code)]
    fn auth_server_auth_endpoint(&self) -> Result<String, AccessTokenError> {
        // ok_or(
        //     self.auth_server_auth_endpoint.clone(),
        //     "auth_server_auth_endpoint",
        // )
        todo!("DELETE annotation after implement AzC")
    }

    fn auth_server_token_endpoint(&self) -> Result<String, AccessTokenError> {
        ok_or(
            self.auth_server_token_endpoint.clone(),
            "auth_server_token_endpoint",
        )
    }

    fn client_id(&self) -> Result<String, AccessTokenError> {
        ok_or(self.client_id.clone(), "client_id")
    }

    fn username(&self) -> Result<String, AccessTokenError> {
        ok_or(self.username.clone(), "username")
    }

    fn password(&self) -> Result<String, AccessTokenError> {
        ok_or(self.password.clone(), "password")
    }

    fn scopes(&self) -> Result<String, AccessTokenError> {
        ok_or(self.scopes.clone(), "scopes")
    }
}

fn ok_or<T>(v: Option<T>, fname: &str) -> Result<T, AccessTokenError> {
    v.ok_or_else(|| AccessTokenError::InvalidConfig(fname.to_string()))
}

#[derive(Deserialize, Debug, Serialize)]
pub struct AccessToken {
    pub access_token: String,
    token_type: String,
    refresh_token: Option<String>,
    expires_in: u64,
    scope: Option<String>,
    id_token: Option<String>,
}

#[derive(Debug)]
pub enum AccessTokenError {
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

impl FromStr for GrantType {
    type Err = InvalidConfig;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "password" => Ok(Password),
            "authorization_code" | "auth" => Ok(AuthorizationCode),
            "client_credentials" | "client" => Ok(ClientCredentials),
            _ => Err(InvalidConfig::InvalidGrantType(s.to_string())),
        }
    }
}

impl GrantType {
    pub async fn get_access_token(
        &self,
        config: &OAuth2Config,
        http: &Client,
    ) -> Result<AccessToken, AccessTokenError> {
        let res = match self {
            GrantType::Password => http
                .post(config.auth_server_token_endpoint()?)
                .basic_auth(config.client_id()?, config.client_secret.clone())
                .header(
                    USER_AGENT,
                    config
                        .default_user_agent
                        .clone()
                        .unwrap_or_else(version::name),
                )
                .form(&[
                    ("grant_type", "password"),
                    ("scope", &config.scopes()?),
                    ("username", &config.username()?),
                    ("password", &config.password()?),
                ]),
            GrantType::ClientCredentials => http
                .post(config.auth_server_token_endpoint()?)
                .basic_auth(config.client_id()?, config.client_secret.clone())
                .header(
                    USER_AGENT,
                    config
                        .default_user_agent
                        .clone()
                        .unwrap_or_else(version::name),
                )
                .form(&[
                    ("grant_type", "client_credentials"),
                    ("scope", &config.scopes()?),
                ]),
            GrantType::AuthorizationCode => {
                // 1. 認可リクエストのURLを作成
                let req = http
                    .get(config.auth_server_auth_endpoint()?)
                    .basic_auth(config.client_id()?, config.client_secret.clone())
                    .header(
                        USER_AGENT,
                        config
                            .default_user_agent
                            .clone()
                            .unwrap_or_else(version::name),
                    )
                    .query(&[
                        ("grant_type", "code"),
                        ("scope", &config.scopes()?),
                        ("state", random().as_str()),
                        ("redirect_uri", "http://localhost:8080/callback"), // TODO: ここどうしよう
                    ]);

                // 2. 認可リクエストのURLをブラウザで開く
                let url = req.build().unwrap();
                let url = url.url().as_str();
                webbrowser::open(url).unwrap();

                // 3. Dummy URL で停止するので URL から認可コードを取得して入力
                let mut auth_code = String::new();
                io::stdin()
                    .read_line(&mut auth_code)
                    .expect("input authorization code");

                // 4. 認可コードをトークンエンドポイントへ POST. AccessToken を取得
                http.post(config.auth_server_token_endpoint()?)
                    .basic_auth(config.client_id()?, config.client_secret.clone())
                    .header(
                        USER_AGENT,
                        config
                            .default_user_agent
                            .clone()
                            .unwrap_or_else(version::name),
                    )
                    .form(&[
                        "code",
                        auth_code.as_str(),
                        "grant_type",
                        "authorization_code",
                    ])
            }
        }
        .send()
        .await?;
        res.json().await.map_err(AccessTokenError::HttpError)
    }
}

// Generate Random State String
fn random() -> String {
    let mut rng = rand::thread_rng();
    let val: i32 = rng.gen();

    // TODO: なんかアレなのでどうにかする
    base64::encode(&val.to_be_bytes()).clone()
}
