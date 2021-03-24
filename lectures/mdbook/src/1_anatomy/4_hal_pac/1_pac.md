# Peripheral Access Crates (PACs)

- Generated using [`svd2rust`](https://docs.rs/svd2rust/0.17.0/svd2rust/)

- Typically re-exported by HAL
```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/hal_pac_structure.rs:hal_pac_import}}
```

- Provides type safe API to access device configuration registers
- Models device's peripherals as singletons
```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/hal_pac_structure.rs:peripheral_init}}
```
- Defines read-only, write-only, and read-write proxies to registers
```rust,noplaypen
// Set GPIO Pin PA1 to high
peripherals.GPIOA.odr.write(|w| w.bits(1));
```

Using the PAC looks scary, though!