# Example use

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
