// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use adaliszk_prelude::tracing::{init_tracing, LoggingConfig};

/// One test owns the process-global subscriber: the logfile must be JSON lines named after
/// the running binary.
#[test]
fn the_logfile_is_json_lines_named_after_the_binary()
{
    let dir = std::env::temp_dir().join(format!("adaliszk-prelude-test-{}", std::process::id()));
    let config = LoggingConfig { dir: dir.clone(), ..LoggingConfig::default() };

    let guard = init_tracing(&["tracing".to_owned()], &config).expect("tracing initializes");
    tracing::warn!(marker = "logfile-test", "hello");
    drop(guard); // flushes the non-blocking writer

    let logfile = std::fs::read_dir(&dir)
        .expect("the log directory was created")
        .filter_map(Result::ok)
        .find(|entry| entry.file_name().to_string_lossy().contains(".log"))
        .expect("a logfile was written");

    let name = logfile.file_name().to_string_lossy().into_owned();
    assert!(!name.contains("adaliszk_prelude."), "named after the binary, not the crate: {name}");

    let content = std::fs::read_to_string(logfile.path()).expect("the logfile reads");
    let line = content.lines().find(|line| line.contains("logfile-test")).expect("the event was written");
    let parsed: serde_json::Value = serde_json::from_str(line).expect("the line is valid JSON");
    assert_eq!(parsed["fields"]["marker"], "logfile-test");

    std::fs::remove_dir_all(&dir).ok();
}
