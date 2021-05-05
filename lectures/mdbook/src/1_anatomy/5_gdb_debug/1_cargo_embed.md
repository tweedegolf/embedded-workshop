# Cargo Embed

- [Based](https://github.com/probe-rs/cargo-embed) on [`probe-rs`](https://github.com/probe-rs/probe-rs)

- Configuration in [`Embed.toml`](../../includes/Embed.toml.md)

- Works like `cargo run`, but runs in background
```bash
$ cargo embed --chip nRF52840 -p examples --bin hal_pac_structure --release
```

- Can act as a GDB server
- Prints a whole load of warnings, but works fine
