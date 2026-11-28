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
    let logfile = format!("{}.log", app_name()?);
    let rotation = tracing_appender::rolling::daily(&config.dir, logfile);
    let (writer, guard) = tracing_appender::non_blocking(rotation);

    tracing_subscriber::registry()
        .with(EnvFilter::new(filter_directives(namespaces, detail_level(config))))
        .with(fmt::layer().with_writer(std::io::stderr)) // console
        .with(fmt::layer().json().with_writer(writer).with_ansi(false)) // logfile
        .try_init()?;

    Ok(guard)
}

fn detail_level(config: &LoggingConfig) -> LogLevel
{
    // Development builds always get the full detail; the config only tunes releases:
    if cfg!(debug_assertions) { LogLevel::Debug } else { config.level }
}

fn filter_directives(namespaces: &[String], level: LogLevel) -> String
{
    // Dependencies outside the workspace only surface hazards; the given
    // namespaces (normally `workspace_crates!()`) get the detailed level:
    let mut filters: Vec<String> = vec![LogLevel::Warn.to_string()];
    for package in namespaces {
        filters.push(format!("{package}={level}"))
    }
    filters.join(",")
}

fn app_name() -> std::io::Result<String>
{
    // The running binary, so the logfile carries the application's name, not this crate's:
    let exe = std::env::current_exe()?;
    let stem = exe.file_stem().unwrap_or(exe.as_os_str());
    Ok(stem.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests
{
    use super::*;
    use googletest::prelude::*;

    #[test]
    fn everything_warns_and_the_namespaces_get_the_detail_level()
    {
        let namespaces = vec!["app".to_owned(), "app-core".to_owned()];

        assert_that!(
            filter_directives(&namespaces, LogLevel::Debug).as_str(),
            eq("warn,app=debug,app-core=debug")
        );
        assert_that!(filter_directives(&[], LogLevel::Trace).as_str(), eq("warn"));
    }

    #[cfg(debug_assertions)]
    #[test]
    fn development_builds_force_the_debug_detail()
    {
        let config = LoggingConfig { level: LogLevel::Error, ..LoggingConfig::default() };
        assert_that!(detail_level(&config), eq(LogLevel::Debug));
    }

    #[test]
    fn the_app_name_is_the_running_binary_without_extension()
    {
        let name = app_name().expect("the current exe is known");

        assert_that!(name.is_empty(), eq(false));
        assert_that!(name.contains(".exe"), eq(false));
        assert_that!(name.as_str(), not(eq("adaliszk-prelude")));
    }
}
