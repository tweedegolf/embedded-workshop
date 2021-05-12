# HAL abstractions

```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/uart_config.rs:uarte_init}}
```

```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/spi_config.rs:spim_init}}
```

```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/i2c_config.rs:twim_init}}
```

- HAL abstractions take peripheral handles and pins by value
    - Only a single instance at a time
- No conflicting configurations

**Questions so far?**
