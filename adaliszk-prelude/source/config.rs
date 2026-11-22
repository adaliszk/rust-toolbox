// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use serde::de::DeserializeOwned;
use std::path::Path;

pub use crate::types::LoggingConfig;

#[derive(Debug, thiserror::Error)]
pub enum ParseError
{
    #[error(transparent)]
    Filesystem(#[from] std::io::Error),
    #[error(transparent)]
    Yaml(#[from] serde_norway::Error),
}

pub fn parse_config<CONFIG: DeserializeOwned>(path: &Path) -> Result<CONFIG, ParseError>
{
    let raw = std::fs::read_to_string(path)?;
    let parsed: CONFIG = serde_norway::from_str(&raw)?;
    Ok(parsed)
}
