# Panic handler

```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/minimal_structure.rs:panic_handler}}
```

*Question: What is the behaviour of panicking in `std` Rust applications?*

- `#[panic_handler]` attribute
- [PanicInfo](https://doc.rust-lang.org/core/panic/struct.PanicInfo.html) type
