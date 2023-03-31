use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

const LOG_FMT: &str = "{d} [{t}] {h({l})} - {m}{n}";

pub fn log_init(level: LevelFilter) -> log4rs::Handle {
    let console_out = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_FMT)))
        .build();
    let config = Config::builder()
        .appender(Appender::builder().build("console_out", Box::new(console_out)))
        .logger(Logger::builder().build("app::backend::db", level))
        .build(Root::builder().appender("console_out").build(level))
        .unwrap();

    log4rs::init_config(config).unwrap()
}

pub fn log_init_with_file(level: LevelFilter, filename: &str) -> log4rs::Handle {
    let console_out = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_FMT)))
        .build();

    let fout = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(LOG_FMT)))
        .build(filename)
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("console_out", Box::new(console_out)))
        .appender(Appender::builder().build("fout", Box::new(fout)))
        .logger(Logger::builder().build("app::backend::db", level))
        .logger(
            Logger::builder()
                .appender("fout")
                .additive(false)
                .build("app::fout", level),
        )
        .build(
            Root::builder()
                .appender("console_out")
                .appender("fout")
                .build(level),
        )
        .unwrap();

    log4rs::init_config(config).unwrap()
}
