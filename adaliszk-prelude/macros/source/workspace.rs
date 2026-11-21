// SPDX-FileCopyrightText: 2026, Ádám Liszkai "Kicsivazz" and Contributors
// SPDX-License-Identifier: MPL-2.0

use cargo_metadata::MetadataCommand;
use std::env;

/// Resolves the workspace members into package names (`-` replaced with `_`).
pub fn collect_members() -> Vec<String>
{
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is set during build");

    let metadata = MetadataCommand::new()
        .current_dir(manifest_dir)
        .exec()
        .expect("run `cargo metadata`");

    metadata
        .workspace_packages()
        .into_iter()
        .map(|package| package.name.to_string().replace('-', "_"))
        .collect()
}
