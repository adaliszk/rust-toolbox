// SPDX-FileCopyrightText: 2026, Ádám Liszkai "Kicsivazz" and Contributors
// SPDX-License-Identifier: MPL-2.
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
    run(
        "rustup",
        &[
            "toolchain",
            "install",
            "nightly",
            "--profile",
            "minimal",
            "--component",
            "rustfmt",
        ],
    );

    // Overwrite the RUSTFMT path to the `adaliszkfmt` for future shells
    let current_exe = env::current_exe().expect("failed to get current exe");
    run(
        "setx",
        &["RUSTFMT", current_exe.to_str().expect("bad utf8")],
    );

    eprintln!("Installed `adaliszkfmt`, please set the rust-analyzer in your editor!");
    eprintln!("For the shell session, please restart to see the changes applied!");
}

fn run(cmd: &str, args: &[&str])
{
    let ok = Command::new(cmd)
        .args(args)
        .status()
        .map(|status| status.success())
        .unwrap_or(false);
    if !ok {
        eprintln!("adaliszkfmt init: {cmd} failed");
        std::process::exit(1);
    }
}
