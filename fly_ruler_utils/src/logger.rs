use log::{debug, error, info, trace, warn, Level};

pub fn test_logger_init() {
    let _ = env_logger::builder().is_test(true).try_init();
}

pub fn log_output(level: Level, msg: &str) {
    match level {
        Level::Trace => trace!("{}", msg),
        Level::Debug => debug!("{}", msg),
        Level::Info => info!("{}", msg),
        Level::Warn => warn!("{}", msg),
        Level::Error => error!("{}", msg),
    }
}
