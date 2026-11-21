# adaliszkfmt

> A drop-in `rustfmt` shim that bundles a personal style and provisions nightly automatically.

`adaliszkfmt` wraps `rustfmt`. It injects a bundled configuration and dispatches to the nightly
toolchain through `rustup`. You install it once, run `adaliszkfmt init`, and every `cargo fmt`
uses your style across all projects. No per-project `rustfmt.toml` is required.

## Why

The same formatting style tends to repeat across projects. A shared `rustfmt.toml` has no import
or inheritance mechanism, so the file gets copied and drifts. `adaliszkfmt` embeds the style in
the binary instead. One installation carries the style everywhere, and nightly-only options stay
available because the shim always targets the nightly `rustfmt`.

This is purely a tool for my own projects, but anyone is welcome to use it!

## Requirements

- `rustup`, which the shim uses to provision and select the nightly `rustfmt`.
- Network access on first run, so `rustup` can download the nightly toolchain.

The `init` automation currently targets Windows. On other platforms, set `RUSTFMT` to the
installed binary path in your shell profile manually.

## Install

```sh
cargo install adaliszkfmt
adaliszkfmt init
```

`init` performs the global setup:

- Installs the nightly toolchain with the `rustfmt` component through `rustup`.
- Registers `adaliszkfmt` as the `RUSTFMT` binary for future shell sessions.

Restart your shell after `init` so the environment variable takes effect.

## Usage

Run `cargo fmt` as usual. Cargo reads the `RUSTFMT` variable and calls `adaliszkfmt`, which
formats with the bundled style.

```sh
cargo fmt
```

### JetBrains RustRover

The `RUSTFMT` variable covers `cargo fmt`, not the editor. RustRover runs `rustfmt` directly
from the toolchain and exposes no setting to swap the binary. Run the shim through a File
Watcher instead.

Open `Settings | Tools | File Watchers`, add a custom watcher, and set these fields:

- File type: `Rust`
- Program: `adaliszkfmt`
- Arguments: `$FilePath$`
- Output paths to refresh: `$FilePath$`

Then open `Settings | Rust | Rustfmt` and clear "Use Rustfmt instead of built-in formatter". The
watcher becomes the only formatter, so the IDE and the command line share one binary.

## License & Disclosure

Licensed under the Mozilla Public License 2.0 (MPL-2.0).

Development was done with LLM as a search engine and debugger (meaning it is for learning only),
NO LINES where generated or changed without direct human decisions and work.

Contributions expected to follow the same **No-Automated-LLM Contributions** policy with strict
limitations to retain the quality and understanding!
