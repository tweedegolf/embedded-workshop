# `Peripherals::take()`

```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/hal_pac_structure.rs:peripheral_init}}
```
- All devices modelled as singletons
- `Peripherals::take()` can be called safely only once
- Only one safe method of getting handle to peripherals
