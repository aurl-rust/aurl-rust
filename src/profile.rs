use std::collections::HashMap;
use std::path::PathBuf;

use tini::Ini;

use crate::oauth2::OAuth2Config;

pub enum InvalidConfig {
    MissingFields(String),
    IniFileError(tini::Error),
    InvalidGrantType(String),
}

pub struct Profile {
    name: String,
}

impl Profile {
    pub fn new(name: &str) -> Profile {
        Profile { name: name.to_string() }
    }

    fn basedir() -> PathBuf {
        let mut home = dirs::home_dir().unwrap();
        home.push(".aurl/");
        home
    }

    pub fn config_file() -> PathBuf {
        let mut file = Profile::basedir();
        file.push("profile.json");
        file
    }

    pub fn token_file(&self) -> PathBuf {
        let mut file = Profile::basedir();
        file.push(format!("token/{}.json", self.name));
        file
    }
}

pub fn read_profiles(profile: Profile) -> Result<HashMap<String, OAuth2Config>, InvalidConfig> {
    let config = Ini::from_file(&Profile::config_file()).map_err(|e| InvalidConfig::IniFileError(e))?;
    let mut profiles: HashMap<String, OAuth2Config> = HashMap::new();

    for (name, section) in config.iter() {
        let profile = OAuth2Config {
            auth_server_auth_endpoint: section.get("auth_server_auth_endpoint"),
            auth_server_token_endpoint: section.get("auth_server_token_endpoint"),
            client_id: section.get("client_id"),
            client_secret: section.get("client_secret"),
            username: section.get("username"),
            password: section.get("password"),
            grant_type: section.get("grant_type")
                .ok_or(InvalidConfig::MissingFields("grant_type".to_string()))?,
            scopes: section.get("scopes"),
        };
        profiles.insert(name.to_string(), profile);
    }
    Ok(profiles)
}