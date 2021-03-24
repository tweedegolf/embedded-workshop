# Harware Abstraction Libraries (HALs)

- Hand-written abstraction layer around PAC
```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/hal_pac_structure.rs:hal_pac_import}}
```
- Extension traits for ergonomics in prelude
```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/hal_pac_structure.rs:prelude}}
```

- Repository typically contains [examples](https://github.com/stm32-rs/stm32l4xx-hal/tree/master/examples)
- Implements traits from [`embedded_hal`](https://docs.rs/embedded-hal/0.2.4/embedded_hal/) (more on that later)

- Makes interacting with peripherals a lot easier
```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/hal_pac_structure.rs:pin_init_hal}}
```

- Usually no complete API for all of the peripherals

**Questions so far?**