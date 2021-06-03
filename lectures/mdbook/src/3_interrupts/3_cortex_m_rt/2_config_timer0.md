# Configure TIMER0

**Step 1: configure relevant peripherals to listen to events**

- Set frequency
- Enable interrupt

```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/interrupt_example.rs:init_timer0}}
```