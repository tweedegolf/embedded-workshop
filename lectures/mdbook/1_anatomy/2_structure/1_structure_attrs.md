# #![no_std] and #![no_main]

```rust,noplaypen
{{#rustdoc_include ../../../src/minimal_structure.rs:attributes}}
```

- **`#![no_std]`** Disables linking to `std`. We can use `core`, a subset of `std`
- **`#![no_main]`** Disables injecting a 'main shim', for loading environment