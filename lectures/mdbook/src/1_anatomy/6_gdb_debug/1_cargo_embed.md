# Cargo Embed

- [Based](https://github.com/probe-rs/cargo-embed) on [`probe-rs`](https://github.com/probe-rs/probe-rs)

- Configuration in [`Embed.toml`](../../includes/Embed.toml.md)

- Works like `cargo run`
```bash
$ cargo embed --chip STM32L476RG -p examples --bin hal_pac_structure --release
```
- Acts as a GDB server