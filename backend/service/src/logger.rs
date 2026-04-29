use tracing::Level;

use crate::{
    resources::ResourceRegistry,
    settings::{LogFormat, LogLevel, Settings},
};

pub fn init() -> anyhow::Result<()> {
    let settings = ResourceRegistry::get::<Settings>()?;
    let level = to_tracing_level(&settings.log_level);
    match settings.log_format {
        LogFormat::HumanReadable => {
            tracing_subscriber::fmt().with_max_level(level).init();
        },
        LogFormat::Json => {
            tracing_subscriber::fmt()
                .json()
                .with_max_level(level)
                .init();
        },
    }
    Ok(())
}

fn to_tracing_level(level: &LogLevel) -> Level {
    match level {
        LogLevel::Trace => Level::TRACE,
        LogLevel::Debug => Level::DEBUG,
        LogLevel::Info => Level::INFO,
        LogLevel::Warn => Level::WARN,
        LogLevel::Error => Level::ERROR,
    }
}
