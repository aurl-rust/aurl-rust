use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Handle;

pub fn init_logger(verbose: bool) -> Handle {
    let (level, pattern) = if verbose {
        (LevelFilter::Debug, "**** {m}{n}")
    } else {
        (LevelFilter::Info, "{m}{n}")
    };

    let stdout = Box::new(ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build());
    let requests = Box::new(ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(pattern)))
        .build());
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", stdout))
        .appender(Appender::builder().build("requests", requests))
        .logger(Logger::builder()
            .appender("requests")
            .additive(false)
            .build("requests", level))
        .build(Root::builder().appender("stdout").build(level)).unwrap();

    log4rs::init_config(config).unwrap()
}