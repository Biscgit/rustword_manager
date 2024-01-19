use std::path::PathBuf;
// logger.rs
use log::LevelFilter;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;

pub fn init_logger(log_file_path: PathBuf) {
    // Set up the file appender for log4rs
    let file_appender = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%Y-%m-%d %H:%M:%S%.3f)} [{l}] {m}\n")))
        .build(log_file_path.as_path())
        .expect("Failed to create file appender");

    // Set up the log4rs configuration
    let config = Config::builder()
        .appender(Appender::builder().build("file", Box::new(file_appender)))
        .build(Root::builder().appender("file").build(LevelFilter::Info)).expect("");

    // Try to set the log4rs configuration
    if let Err(err) = log4rs::init_config(config) {
        eprintln!("Failed to initialize log4rs: {}", err);
    }
}
