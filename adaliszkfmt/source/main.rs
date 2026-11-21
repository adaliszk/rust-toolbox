// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0
//!
//! `adaliszkfmt` is a drop-in `rustfmt` and `rustup` shim with a custom configuration.
//!
//! To use it, install and initialize with:
//!
//! ```bash
//! cargo install --path adaliszkfmt
//! adaliszkfmt init
//! ```
//!

use std::path::Path;
use std::{env, fs, process::Command};

// Bake a file into the binary for runtime usage
const CONFIG: &str = include_str!("../resources/rustfmt.toml");

fn main()
{
    match env::args().nth(1).as_deref() {
        Some("init") => init(),
        _ => format(),
    }
}

fn format()
{
    // Save the binary-stored config into a temporary location
    let cfg = env::temp_dir().join("adaliszkfmt.toml");
    fs::write(&cfg, CONFIG).expect("failed to write custom rustfmt config");
    // TODO: Read and fold local "rustfmt.toml" into the custom defaults

    std::process::exit(
        Command::new("rustfmt")
            .env("RUSTUP_TOOLCHAIN", "nightly")
            .arg("--config-path")
            .arg(&cfg)
            .args(env::args_os().skip(1))
            .status()
            .expect("failed to execute rustfmt")
            .code()
            .unwrap_or(1),
    );
}

fn init()
{
    // Install the nightly rustfmt to allow the experimental flags to be used
    let status = Command::new("rustup")
        .args([
            "toolchain",
            "install",
            "nightly",
            "--profile",
            "minimal",
            "--component",
            "rustfmt",
        ])
        .status()
        .expect("failed to execute rustup")
        .code()
        .unwrap_or(1);

    // Overwrite the RUSTFMT path to the `adaliszkfmt` for future shells
    let current_binary = env::current_exe().expect("failed to get current exe");
    setenv(&current_binary);

    eprintln!("Installed `adaliszkfmt`, please set the rust-analyzer in your editor!");
    eprintln!("For the shell session, please restart to see the changes applied!");

    std::process::exit(status);
}

#[cfg(windows)]
fn setenv(bin: &Path)
{
    Command::new("setx")
        .args([
            "RUSTFMT",
            bin.to_str().expect("failed to convert binary path"),
        ])
        .status()
        .map(|status| status.success())
        .unwrap();
}

#[cfg(unix)]
fn setenv(bin: &Path)
{
    eprintln!("adaliszkfmt init: Unix currently not supported");
    std::process::exit(1);
}
