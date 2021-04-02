# GDB: The GNU Project Debugger

> [GDB, the GNU Project debugger, allows you to see what is going on `inside' another program while it executes -- or what another program was doing at the moment it crashed.](https://www.gnu.org/software/gdb/)

- Connects to Cargo Embed or OpenOCD server
- Communicates with ST-Link, which talks to our chip

To enable it, edit `.cargo/config.toml`, and uncomment the `gdb-multiarch` runner:

```toml
# Uncomment the line below to use probe-rs
# runner = "probe-run --chip STM32L476RG"
# Uncomment the line below to GDB with Cargo Embed
runner = "gdb-multiarch -x cargo-embed.cfg"
```
Then after setting up your preferred GDB server, just run
```bash
$ cargo run -p assignments --bin blinky
```

*Only one of the runners can be enabled at a time*

## Common commands

- `c` (`continue`): continue running until next breakpoint
- `s` (`step`): step to next line of code
- `si` (`stepi`): step to next instruction
- `n` (`next`): step over to next line of code (does not enter functions)
- `fin` (`finish`): continue current function until it returns
- `bt` (`backtrace`): prints a stack backtrace
- `i lo` (`info locals`): lists local variables and their values
- `i va` (`info variables`): lists info about all variables
- `p` (`print`): print value of local variable. Rust-style pointer derefencing and casting is supported
- `l` (`list`): show code around the current instruction

## Example

```rust
struct Data {
    x: u32,
    y: &'static str,
    z: Option<u8>,
}
// -snip-
let d = Data {
    x: 123,
    y: "Hello, world!",
    z: Some(123),
};

let f = &d;
```

```gdb
(gdb) print f
$2 = (*mut gdb_demo::Data) 0x20017fd0
(gdb) print *f
$3 = gdb_demo::Data {
  x: 123,
  y: "Hello, world!",
  z: <error reading variable>
}
(gdb) print f.z
$4 = core::option::Option<u8>::Some(123)
```
