use chrono::Utc;
use log::{self, LevelFilter, Metadata, Record, SetLoggerError};

struct ConsoleLogger {
    level: LevelFilter,
}

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let target = record.target();
            let target = target.split("::").next().unwrap_or(target);
            let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S%.3f");

            println!(
                "{} [{}] {}: {}",
                timestamp,
                record.level(),
                target,
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

pub fn initialize_logging(verbosity: u8) -> Result<(), SetLoggerError> {
    let level = match verbosity {
        0 => LevelFilter::Error,
        1 => LevelFilter::Warn,
        2 => LevelFilter::Info,
        _ => LevelFilter::Debug,
    };

    let logger = Box::new(ConsoleLogger { level });

    log::set_boxed_logger(logger)?;
    log::set_max_level(level);

    Ok(())
}
