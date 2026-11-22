// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use adaliszk_prelude::config::{LoggingConfig, ParseError, parse_config};
use rstest::rstest;
use serde::Deserialize;
use std::path::{Path, PathBuf};

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
        parse_config(Path::new("tests/fixtures/sample.yaml")).expect("parse sample");

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
        parse_config(Path::new("tests/fixtures/does-not-exist.yaml"));

    assert!(matches!(result, Err(ParseError::Filesystem(_))));
}

#[test]
fn invalid_yaml_throws_yaml_error()
{
    let result: Result<ExampleConfig, ParseError> =
        parse_config(Path::new("tests/fixtures/invalid.yaml"));

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
    let config: AppConfig = parse_config(Path::new(path)).expect("parse config");

    assert_eq!(config.logging.dir, PathBuf::from(expected_dir));
}
