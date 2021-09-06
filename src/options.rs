use clap::{AppSettings, Clap};
use reqwest::Method;

#[derive(Clap, Debug)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {
    #[clap(short, long, default_value = "default")]
    pub profile: String,
    #[clap(short = 'X', long, default_value = "GET")]
    pub request: Method,
    // -H HEADER:VALUE
    #[clap(short = 'H', long, multiple = true)]
    pub header: Vec<String>,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long)]
    pub verbose: bool,
    pub url:String,
}

pub fn parse_opts() -> Opts {
    Opts::parse()
}