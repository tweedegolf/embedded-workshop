# `#[init]`: Initialize late resources
```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/rtic_example.rs:init_start}}

        // Initializing peripherals omitted

{{#rustdoc_include ../../../../src/bin/rtic_example.rs:init_end}}
```

- Initialize the peripherals
- Runs with interrupts disabled