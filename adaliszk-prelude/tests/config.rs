// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use adaliszk_prelude::config::{LoggingConfig, ParseError, parse_files};
use rstest::rstest;
use serde::Deserialize;
use std::path::PathBuf;

fn fixtures(names: &[&str]) -> Vec<PathBuf>
{
    names.iter().map(|name| PathBuf::from(format!("tests/fixtures/{name}"))).collect()
}

#[derive(Deserialize, Debug, PartialEq, Eq)]
struct ExampleConfig
{
    foo: String,
    bar: String,
}

#[test]
fn parses_example_config()
{
    let config: ExampleConfig =
        parse_files(&fixtures(&["sample.yaml"])).expect("parse sample");

    assert_eq!(
        config,
        ExampleConfig {
            foo: String::from("one"),
            bar: String::from("two"),
        }
    );
}

#[test]
fn missing_file_throws_filesystem_error()
{
    let result: Result<ExampleConfig, ParseError> =
        parse_files(&fixtures(&["does-not-exist.yaml"]));

    assert!(matches!(result, Err(ParseError::Filesystem(_))));
}

#[test]
fn invalid_yaml_throws_yaml_error()
{
    let result: Result<ExampleConfig, ParseError> =
        parse_files(&fixtures(&["invalid.yaml"]));

    assert!(matches!(result, Err(ParseError::Yaml(_))));
}

#[derive(Deserialize, Debug)]
struct AppConfig
{
    #[serde(default)]
    logging: LoggingConfig,
}

#[rstest]
#[case("tests/fixtures/with_logging.yaml", "/custom/logs")]
#[case("tests/fixtures/without_logging.yaml", "storage/logs")]
fn logging_falls_back_to_default_when_absent(#[case] path: &str, #[case] expected_dir: &str)
{
    let config: AppConfig = parse_files(&[PathBuf::from(path)]).expect("parse config");

    assert_eq!(config.logging.dir, PathBuf::from(expected_dir));
}

#[rstest]
#[case("sample.yaml")]
#[case("sample.json")]
#[case("sample.toml")]
fn parses_each_supported_format(#[case] name: &str)
{
    let config: ExampleConfig = parse_files(&fixtures(&[name])).expect("parse single file");

    assert_eq!(
        config,
        ExampleConfig {
            foo: String::from("one"),
            bar: String::from("two"),
        }
    );
}

#[test]
fn later_files_override_earlier_values()
{
    let config: ExampleConfig =
        parse_files(&fixtures(&["layer_base.yaml", "layer_override.yaml"])).expect("parse layers");

    assert_eq!(
        config,
        ExampleConfig {
            foo: String::from("one"),
            bar: String::from("overridden"),
        }
    );
}

#[test]
fn layers_may_mix_formats()
{
    // YAML base overridden by a JSON layer.
    let config: ExampleConfig =
        parse_files(&fixtures(&["layer_base.yaml", "override.json"])).expect("parse mixed layers");

    assert_eq!(
        config,
        ExampleConfig {
            foo: String::from("one"),
            bar: String::from("overridden"),
        }
    );
}

#[test]
fn merges_nested_mappings_key_by_key()
{
    let merged: AppConfig =
        parse_files(&fixtures(&["logging_base.yaml", "logging_override.yaml"]))
            .expect("parse layers");

    // Base contributes level + format, override contributes dir; together they
    // should equal the same shape expressed as one file.
    let combined: AppConfig =
        parse_files(&fixtures(&["with_logging.yaml"])).expect("parse combined");

    assert_eq!(merged.logging, combined.logging);
}

#[test]
fn unsupported_extension_throws_error()
{
    let result: Result<ExampleConfig, ParseError> = parse_files(&fixtures(&["sample.ini"]));

    assert!(matches!(result, Err(ParseError::UnsupportedFormat(_))));
}
