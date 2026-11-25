// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use serde::de::DeserializeOwned;
use serde_norway::{Mapping, Value};
use std::path::{Path, PathBuf};

pub use crate::types::LoggingConfig;

#[derive(Debug, thiserror::Error)]
pub enum ParseError
{
    #[error(transparent)]
    Filesystem(#[from] std::io::Error),
    #[error(transparent)]
    Yaml(#[from] serde_norway::Error),
    #[error(transparent)]
    Json(#[from] serde_json::Error),
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
    #[error("unsupported config format: {}", .0.display())]
    UnsupportedFormat(PathBuf),
    #[error("invalid config pattern: {0}")]
    Pattern(#[from] glob::PatternError),
    #[error(transparent)]
    Glob(#[from] glob::GlobError),
    #[error("no config files matched pattern: {0}")]
    NoMatches(String),
    #[error(
        "config nesting exceeds the maximum merge depth of {}",
        MAX_MERGE_DEPTH
    )]
    TooDeep,
}

/// Maximum mapping nesting depth merged before bailing out, bounding recursion
/// on deeply nested or hostile config.
const MAX_MERGE_DEPTH: usize = 64;

/// Parse and layer several config files into one config.
///
/// Each file is parsed according to its extension — `.yaml`/`.yml`, `.json`,
/// or `.toml` — into a common value tree, then the trees are merged left to
/// right so a later file overrides an earlier one. Mappings are combined
/// key-by-key (recursing into nested mappings), while scalars and sequences are
/// replaced wholesale.
///
/// The order of `paths` is the layering order; use [`resolve_paths`] to turn a
/// glob into that list. An unknown extension yields
/// [`ParseError::UnsupportedFormat`].
pub fn parse_files<CONFIG: DeserializeOwned>(paths: &[PathBuf]) -> Result<CONFIG, ParseError>
{
    let mut merged = Value::Mapping(Mapping::new());
    for path in paths {
        merge_into(&mut merged, parse_layer(path)?, 0)?;
    }

    let parsed: CONFIG = serde_norway::from_value(merged)?;
    Ok(parsed)
}

fn parse_layer(path: &Path) -> Result<Value, ParseError>
{
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(str::to_ascii_lowercase);

    // Reject unknown extensions before touching the filesystem.
    if !matches!(extension.as_deref(), Some("yaml" | "yml" | "json" | "toml")) {
        return Err(ParseError::UnsupportedFormat(path.to_path_buf()));
    }

    let raw = std::fs::read_to_string(path)?;
    match extension.as_deref() {
        Some("json") => Ok(serde_json::from_str(&raw)?),
        Some("yaml" | "yml") => Ok(serde_norway::from_str(&raw)?),
        _ => Ok(toml::from_str(&raw)?),
    }
}

/// Expand a glob pattern into the sorted list of matching file paths.
///
/// A plain path matches only itself. Results come back in sorted (alphabetical)
/// order so callers get a stable layering order; matching nothing yields
/// [`ParseError::NoMatches`] rather than an empty list.
pub fn resolve_paths(pattern: &str) -> Result<Vec<PathBuf>, ParseError>
{
    let mut paths: Vec<PathBuf> = Vec::new();
    for entry in glob::glob(pattern)? {
        paths.push(entry?);
    }
    if paths.is_empty() {
        return Err(ParseError::NoMatches(pattern.to_owned()));
    }
    paths.sort();
    Ok(paths)
}

/// Deep-merge `source` into `target`.
///
/// When both sides are mappings, merge them key by key and recurse into keys
/// they share so nested mappings combine. For every other shape — scalars,
/// sequences, or a type mismatch — the source replaces the target value.
///
/// Recursion is capped at [`MAX_MERGE_DEPTH`] to guard against deeply nested or
/// hostile config; exceeding it yields [`ParseError::TooDeep`].
fn merge_into(target: &mut Value, source: Value, depth: usize) -> Result<(), ParseError>
{
    if depth > MAX_MERGE_DEPTH {
        return Err(ParseError::TooDeep);
    }

    match (target, source) {
        // merge each source key into the target key by overwriting
        (Value::Mapping(target), Value::Mapping(source)) => {
            for (key, value) in source {
                let slot = target.entry(key).or_insert(Value::Null);
                merge_into(slot, value, depth + 1)?;
            }
        }
        // mismatches ignored and the source wins
        (slot, value) => *slot = value,
    }

    Ok(())
}

#[cfg(test)]
mod tests
{
    // `resolve_paths` is private, so its tests live beside it rather than in the
    // integration suite under `tests/`.
    use super::*;

    #[test]
    fn resolve_paths_expands_glob_in_sorted_order()
    {
        let paths = resolve_paths("tests/fixtures/layer_*.yaml").expect("resolve glob");

        assert_eq!(
            paths,
            vec![
                PathBuf::from("tests/fixtures/layer_base.yaml"),
                PathBuf::from("tests/fixtures/layer_override.yaml"),
            ]
        );
    }

    #[test]
    fn resolve_paths_matching_nothing_throws_no_matches_error()
    {
        let result = resolve_paths("tests/fixtures/does-not-exist-*.yaml");

        assert!(matches!(result, Err(ParseError::NoMatches(_))));
    }

    /// Build a mapping nested `depth` levels deep: `{k: {k: {k: ...}}}`.
    fn nested_mapping(depth: usize) -> Value
    {
        let mut value = Value::Null;
        for _ in 0..depth {
            let mut map = Mapping::new();
            map.insert(Value::String("k".to_owned()), value);
            value = Value::Mapping(map);
        }
        value
    }

    #[test]
    fn merge_into_rejects_nesting_beyond_the_depth_cap()
    {
        let mut target = nested_mapping(MAX_MERGE_DEPTH + 2);
        let source = nested_mapping(MAX_MERGE_DEPTH + 2);

        assert!(matches!(
            merge_into(&mut target, source, 0),
            Err(ParseError::TooDeep)
        ));
    }

    #[test]
    fn merge_into_accepts_nesting_within_the_depth_cap()
    {
        let mut target = nested_mapping(MAX_MERGE_DEPTH);
        let source = nested_mapping(MAX_MERGE_DEPTH);

        assert!(merge_into(&mut target, source, 0).is_ok());
    }
}
