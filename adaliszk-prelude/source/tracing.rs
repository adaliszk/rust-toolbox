// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

pub use crate::types::{LogLevel, LoggingConfig};
pub use adaliszk_prelude_macros::workspace_crates;

#[derive(Debug, thiserror::Error)]
pub enum TracingError
{
    #[error(transparent)]
    Filesystem(#[from] std::io::Error),
    #[error(transparent)]
    Initialization(#[from] tracing_subscriber::util::TryInitError),
}

pub fn init_tracing(
    namespaces: &[String],
    config: &LoggingConfig,
) -> Result<WorkerGuard, TracingError>
{
    #[cfg(not(debug_assertions))]
    const FILTER_LEVEL: LogLevel = config.logging.level;
    #[cfg(debug_assertions)]
    const FILTER_LEVEL: LogLevel = LogLevel::Debug;

    let mut filters: Vec<String> = vec![FILTER_LEVEL.to_string()];
    for package in namespaces {
        filters.push(format!("{package}={FILTER_LEVEL}"))
    }

    const APP_NAME: &str = env!("CARGO_CRATE_NAME");

    let logfile = format!("{APP_NAME}.log");
    let rotation = tracing_appender::rolling::daily(&config.dir, logfile);
    let (writer, guard) = tracing_appender::non_blocking(rotation);

    tracing_subscriber::registry()
        .with(EnvFilter::new(filters.join(",")))
        .with(fmt::layer().with_writer(std::io::stderr)) // console
        .with(fmt::layer().with_writer(writer).with_ansi(true))
        .try_init()?;

    Ok(guard)
}
