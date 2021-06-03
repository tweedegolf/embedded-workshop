# Interrupt priority and unmasking interrupt

**Step 4: Set interrupt priority**

**Step 5: Set interrupt priority**

```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/interrupt_example.rs:nvic_unmask}}
```

- Lower value = higher priority
- `GPIOTE` takes precedence over `TIMER0`

**Questions so far?**