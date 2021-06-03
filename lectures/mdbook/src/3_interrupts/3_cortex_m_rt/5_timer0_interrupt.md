# TIMER0 interrupt definition

**Step 3: Define interrupt handlers**

```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/interrupt_example.rs:timer0_isr}}
```

- Get `TIMER0` reference
- Check what happened
- Set flags accordingly
- **Dont forget: Clear event**