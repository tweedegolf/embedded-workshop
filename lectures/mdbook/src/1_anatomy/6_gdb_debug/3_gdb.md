# GDB: The GNU Project Debugger

> [GDB, the GNU Project debugger, allows you to see what is going on `inside' another program while it executes -- or what another program was doing at the moment it crashed.](https://www.gnu.org/software/gdb/)

- Connects to Cargo Embed or OpenOCD server
- Communicates with ST-Link, which talks to our chip

To enable it, edit `.cargo/config.toml`, and uncomment one of the runners:

```toml
# Uncomment the line below to use probe-rs
runner = "probe-run --chip STM32L476RG"
# Uncomment the line below to GDB with OpenOCD
runner = "gdb-multiarch -x openocd.gdb"
# Uncomment the line below to GDB with Cargo Embed
runner = "gdb-multiarch -x cargo-embed.cfg"
```
Then after setting up your preferred GDB server, just run
```bash
$ cargo run -p assignments --bin blinky
```

*Only one of the runners can be enabled at a time*

## Common commands

- `bt` 