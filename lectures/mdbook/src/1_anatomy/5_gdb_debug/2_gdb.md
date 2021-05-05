# GDB: The GNU Project Debugger

> [GDB, the GNU Project debugger, allows you to see what is going on 'inside' another program while it executes](https://www.gnu.org/software/gdb/)

- Connects to a GDB server (cargo-embed)
- Communicates with J-Link, which talks to our chip

To enable it, edit `.cargo/config.toml`, and uncomment the `gdb-multiarch` runner:

```toml
# Uncomment the line below to use probe-rs
# runner = "probe-run --chip nRF52840"
# Uncomment the line below to GDB with Cargo Embed
runner = "gdb-multiarch -x cargo-embed.cfg"
```
Then after setting up your preferred GDB server, just run
```bash
$ cargo run -p examples --bin gdb_demo
```
