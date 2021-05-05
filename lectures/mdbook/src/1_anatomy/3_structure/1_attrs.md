# #![no_std] and #![no_main]

```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/minimal_structure.rs:attributes}}
```

- **`#![no_std]`** Disables linking to `std`. We can use `core`, a subset of `std`

*Question: why can't we use `std`?*

- **`#![no_main]`** Disables linking to standard main entry point.

*Question: why can't we use `main`?*