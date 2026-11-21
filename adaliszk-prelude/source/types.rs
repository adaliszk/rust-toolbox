// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use strum::{AsRefStr, Display, IntoStaticStr};

#[derive(Serialize, Deserialize, Clone, Default, Debug, PartialEq)]
pub struct LoggingConfig
{
    pub level: LogLevel,
    pub format: LogFormat,
    pub dir: PathBuf,
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
    /// Nicely readable YAML-like output.
    Pretty,
    /// Terse single-line notes.
    #[default]
    Compact,
    /// Parsable JSON-lines.
    Structured,
}
