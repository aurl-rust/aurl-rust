use std::fs::File;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;
use std::time::SystemTime;

use log::{info, warn};
use rand::Rng;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::oauth2::GrantType::{AuthorizationCode, ClientCredentials, Password};
use crate::profile::InvalidConfig;
use crate::version;
use reqwest::header::USER_AGENT;
use std::io::Write;

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
        ok_or(
            self.auth_server_auth_endpoint.clone(),
            "auth_server_auth_endpoint",
        )
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

    fn redirect(&self) -> Result<String, AccessTokenError> {
        ok_or(self.redirect.clone(), "redirect")
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

impl AccessToken {
    // Save AccessToken Cache
    pub fn save_cache(&self, profile: &str) -> AccessToken {
        // open cache file
        // let path = AccessToken::cache_file(profile);
        // info!("{:?}", path.as_path());
        // let mut cache_file = File::create(path).unwrap();

        todo!()
    }

    // calculate ttl with expires_in in AccessToken
    fn calc_ttl(expires_in: u64) -> u64 {
        // Epoch Sec に expires_in を加えた秒を TTL
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        now.as_secs() + expires_in
    }

    fn basedir() -> PathBuf {
        let mut home = dirs::home_dir().unwrap();
        home.push(".aurl/");
        home
    }

    // create Token Cache File path
    fn cache_file(profile: &str) -> PathBuf {
        let mut file = AccessToken::basedir();
        file.push("token");
        file.push(profile);
        file.set_extension("json");

        file
    }
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
                let req = http.get(config.auth_server_auth_endpoint()?).query(&[
                    ("response_type", "code"),
                    ("client_id", &config.client_id()?),
                    ("scope", &config.scopes()?),
                    ("state", random().as_str()),
                    ("redirect_uri", config.redirect()?.as_str()),
                ]);

                // 2. 認可リクエストのURLをブラウザで開く
                let req = req.build().unwrap();
                let url = req.url().as_str();
                info!("{:?}", url);

                webbrowser::open(url).unwrap();

                // 3. Dummy URL で停止するので URL から認可コードを取得して入力
                let mut auth_code = String::new();

                loop {
                    print!("\nEnter authorization code:");
                    io::stdout().flush().unwrap();
                    match io::stdin().read_line(&mut auth_code) {
                        Ok(size) if size > 1 => break,
                        Err(e) => warn!("{:?}", e),
                        _ => (),
                    }
                }
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
                    .header("content-type", "application/x-www-form-urlencoded")
                    .form(&[
                        ("code", auth_code.trim()),
                        ("grant_type", "authorization_code"),
                        ("redirect_uri", config.redirect()?.as_str()),
                    ])
            }
        }
        .send()
        .await?;
        res.json()
            .await
            // TODO: save cache here
            .map_err(AccessTokenError::HttpError)
    }
}

// Generate Random State String
fn random() -> String {
    let mut rng = rand::thread_rng();
    let val: i32 = rng.gen();

    // TODO: なんかアレなのでどうにかする
    base64::encode(&val.to_be_bytes())
}
