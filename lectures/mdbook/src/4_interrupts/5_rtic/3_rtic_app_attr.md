# `#[rtic::app]` attribute

```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/rtic_example.rs:app_attr}}
```

- Application code defined in const block
- PAC selected in attribute
- Monotonic for task scheduling *(more later)*
