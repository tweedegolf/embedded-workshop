# Example use
Run cargo-embed in background:
```bash
cargo embed --chip nRF52840 -p examples --bin gdb_demo
```

Run gdb:
```bash
cargo run -p examples --bin gdb_demo
```

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

**Questions so far?**