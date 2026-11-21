> A drop-in `rustfmt` shim that bundles a personal style and initializes nightly requirements.

Wraps **rustfmt** and injects a bundled configuration on execution, with initialization also
dispatches to the nightly toolchain through **rustup**.

No per-project `rustfmt.toml` is required and applies the common style of:

```toml
edition = "2024"
brace_style = "AlwaysNextLine"  # <- Adds spacing between signature and body for easier reading
max_width = 96 # <- Preference for my own flow where I have mutliple vertical panels
```

## Requirements

- RustUp: which the shim uses to provision and select the nightly `rustfmt`.
- Network access: on first run `rustup` needs to download the nightly toolchain.

The **init** automation currently targets Windows. On other platforms, set `RUSTFMT` to the
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

## Why do this?

The same formatting style tends to repeat across projects. A shared **rustfmt.toml** has no
import or inheritance mechanism, so the file gets copied and drifts. A custom ship instead
embeds the style in the binary instead. One installation carries the style everywhere, and
nightly-only options stay available with the expected initialization.

This is purely a tool for my own projects, but anyone is welcome to use it!

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
NO SOURCE LINES where generated or changed without direct human decisions and work.

Contributions expected to follow the same **No-Automated-LLM Contributions** policy with strict
limitations to retain the quality and understanding!
