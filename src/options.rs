use crate::output::Type;
use clap::{AppSettings, Parser};
use reqwest::Method;

#[derive(Parser, Debug)]
#[clap(setting = AppSettings::ArgRequiredElseHelp)]
pub struct Opts {
    #[clap(short, long, default_value = "default")]
    pub profile: String,
    #[clap(short = 'X', long, default_value = "GET")]
    pub request: Method,
    // -H HEADER:VALUE
    #[clap(short = 'H', long, multiple_values = true)]
    pub header: Vec<String>,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long)]
    pub verbose: bool,
    #[clap(long, default_value = "")]
    pub auth_header_template: String,
    #[clap(long, default_value = "none")]
    pub output: Type,
    pub url: String,
}

pub fn parse_opts() -> Opts {
    Opts::parse()
}
