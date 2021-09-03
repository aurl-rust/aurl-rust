use clap::{AppSettings, Clap};


#[derive(Clap)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long, default_value = "default")]
    profile: String,
    #[clap(short = 'X', long, default_value = "GET")]
    request: String,
    // -H HEADER:VALUE
    #[clap(short = 'H', long, multiple = true)]
    header: Vec<String>,
    /// A level of verbosity, and can be used multiple times
    #[clap(short, long)]
    verbose: bool,
}