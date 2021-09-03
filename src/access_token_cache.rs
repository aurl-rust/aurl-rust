use std::fs::File;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, BufWriter, Seek, SeekFrom, Write};

use serde_json;

use crate::oauth2::AccessToken;
use crate::profile::Profile;

pub enum AccessTokenCacheError {
    IOErr(std::io::Error),
    SerdeErr(serde_json::Error),
}

pub struct AccessTokenCache {
    file: File,
}

impl AccessTokenCache {
    pub fn new(profile: Profile) -> AccessTokenCache {
        //create config dir
        let cache_file = profile.token_file();
        std::fs::create_dir_all(cache_file.parent().unwrap()).unwrap();

        let file = OpenOptions::new()
            .create(true)
            .write(true)
            .read(true)
            .open(cache_file)
            .unwrap();
        return AccessTokenCache { file };
    }

    pub fn restore_cache(&self) -> Result<Option<AccessToken>, AccessTokenCacheError> {
        let mut buf = Ok(BufReader::new(&self.file)).map_err(|e| AccessTokenCacheError::IOErr(e))?;
        match serde_json::from_reader(buf) {
            Ok(token) => Ok(token),
            Err(e) => Err(AccessTokenCacheError::SerdeErr(e)),
        }
    }

    pub fn save_cache(mut self, token: &AccessToken) -> Result<(), AccessTokenCacheError> {
        self.file.set_len(0).map_err(|e| AccessTokenCacheError::IOErr(e))?;
        self.file.seek(SeekFrom::Start(0)).map_err(|e| AccessTokenCacheError::IOErr(e))?;
        let writer = BufWriter::new(self.file);
        serde_json::to_writer(writer, token).map_err(|e| AccessTokenCacheError::SerdeErr(e))
    }
}