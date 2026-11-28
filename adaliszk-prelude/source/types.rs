// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum::{AsRefStr, Display, IntoStaticStr};

/// A boxed, thread-safe error for async results.
///
/// Any [`Error`](std::error::Error) that is [`Send`] + [`Sync`] converts into it with `?`.
pub type AsyncError = Box<dyn std::error::Error + Send + Sync>;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct LoggingConfig
{
    pub level: LogLevel,
    pub format: LogFormat,
    pub dir: PathBuf,
}

impl Default for LoggingConfig
{
    fn default() -> Self
    {
        Self {
            level: LogLevel::default(),
            format: LogFormat::default(),
            dir: PathBuf::from("storage/logs"),
        }
    }
}

#[cfg(test)]
mod logging_config_test
{
    use super::*;
    use googletest::prelude::*;

    #[test]
    fn logging_config_default_uses_system_log_dir()
    {
        let config = LoggingConfig::default();

        assert_that!(config.level, eq(LogLevel::Info));
        assert_that!(config.format, eq(LogFormat::Compact));
        assert_that!(config.dir, eq(&PathBuf::from("storage/logs")));
    }
}

#[derive(
    Serialize,
    Deserialize,
    Display,
    AsRefStr,
    IntoStaticStr,
    Default,
    Clone,
    Copy,
    Debug,
    PartialEq,
)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LogLevel
{
    /// Extremely verbose implementation debugging.
    Trace,
    /// Extra context for logic debugging.
    Debug,
    /// General information, everything works as expected.
    #[default]
    Info,
    /// Hazardous situations, but not actual errors.
    Warn,
    /// Errors that needs to be looked at.
    Error,
}

#[cfg(test)]
mod log_level_test
{
    use super::*;
    use googletest::prelude::*;
    use rstest::rstest;

    #[rstest]
    #[case("trace", LogLevel::Trace)]
    #[case("debug", LogLevel::Debug)]
    #[case("info", LogLevel::Info)]
    #[case("warn", LogLevel::Warn)]
    #[case("error", LogLevel::Error)]
    fn log_level_deserializes_from_lowercase(#[case] input: &str, #[case] expected: LogLevel)
    {
        let parsed: LogLevel = serde_norway::from_str(input).expect("valid level");
        assert_that!(parsed, eq(expected));
    }

    #[test]
    fn log_level_renders_lowercase()
    {
        assert_that!(LogLevel::Warn.to_string().as_str(), eq("warn"));
        assert_that!(<&str>::from(LogLevel::Error), eq("error"));
    }
}

#[derive(
    Serialize,
    Deserialize,
    Display,
    AsRefStr,
    IntoStaticStr,
    Default,
    Clone,
    Copy,
    Debug,
    PartialEq,
)]
#[strum(ascii_case_insensitive, serialize_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum LogFormat
{
    /// Terse single-line notes.
    #[default]
    Compact,
    /// Parsable JSON-lines.
    Structured,
}

#[cfg(test)]
mod log_format_test
{
    use super::*;
    use googletest::prelude::*;
    use rstest::rstest;
    #[rstest]
    #[case("compact", LogFormat::Compact)]
    #[case("structured", LogFormat::Structured)]
    fn log_format_deserializes_from_lowercase(#[case] input: &str, #[case] expected: LogFormat)
    {
        let parsed: LogFormat = serde_norway::from_str(input).expect("valid format");
        assert_that!(parsed, eq(expected));
    }
}
