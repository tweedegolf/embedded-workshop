# Peripheral Access Crates (PACs)

- Generated using [`svd2rust`](https://docs.rs/svd2rust/0.17.0/svd2rust/)

- Typically re-exported by HAL
```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/pac_init_example.rs:pac_import}}
```

- Provides a type safe API to access device configuration registers
- Models device's peripherals as singletons
```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/pac_init_example.rs:peripheral_init}}
```
- Defines read-only, write-only, and read-write proxies to registers
```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/pac_init_example.rs:pin_init_pac}}
```

Using the PAC looks scary, though!