use std::process::abort;

use log::error;

use crate::cli::AppError;

mod cli;
mod logger;
mod oauth2;
mod options;
mod output;
mod profile;
mod request;
mod version;

#[tokio::main]
async fn main() {
    let opts = options::parse_opts();
    logger::init_logger(opts.verbose);
    match cli::execute(opts).await {
        Err(AppError::RequestError(e)) => {
            error!("RequestError: {:?}", e);
            abort();
        }
        Err(AppError::ProfileNotFound(profile)) => {
            error!("Profile not found: {:?}", profile);
            abort();
        }
        Err(AppError::InvalidConfig(e)) => {
            error!("Invalid .aurl/profiles: {:?}", e);
            abort();
        }
        Ok(_) => (),
    }
}
