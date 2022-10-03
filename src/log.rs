use log::{info, LevelFilter, SetLoggerError};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Config, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

use std::fs;

pub fn init_log(
    log_path: &String,
    log_size: f64,
    console_level: log::LevelFilter,
    file_level: log::LevelFilter,
) -> Result<(), SetLoggerError> {
    // if the log file is bigger than log_size in bytes, we replace the log file instead of
    // appending to it.
    let mut append = true;
    match fs::metadata(log_path) {
        Ok(log) => {
            if log.len() as f64 > log_size * 1048576.0 {
                append = false;
            }
        }
        Err(_) => {
            append = true;
        }
    };

    // creates configuration for the log file
    let log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{l} {d(%Y-%m-%d %H:%M:%S)} - {m}\n",
        )))
        .append(append)
        .build(log_path)
        .unwrap();

    // creates configuration for stderr outputs
    let stderr = ConsoleAppender::builder()
        .target(Target::Stderr)
        .encoder(Box::new(PatternEncoder::new(
            "{h({l})} {d(%Y-%m-%d %H:%M:%S)} - {m}\n", // pattern is the same as in the log file,
                                                       // but slightly more colourful
        )))
        .build();

    // creates the full configuration using log_file and stderr
    let config = Config::builder()
        .appender(Appender::builder().build("log_file", Box::new(log_file)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(console_level)))
                .build("stderr", Box::new(stderr)),
        )
        .build(
            Root::builder()
                .appender("log_file")
                .appender("stderr")
                .build(file_level),
        )
        .unwrap();

    // initializes config
    let _handle = log4rs::init_config(config)?;

    // send a lil message announcing that logging has started successfully
    info!("Logging has started :)");
    Ok(()) //Ok
}
