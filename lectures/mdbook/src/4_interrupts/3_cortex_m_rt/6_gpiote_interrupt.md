# GPIOTE interrupt definition

**Step 3: Define interrupt handlers**

```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/interrupt_example.rs:gpiote_isr}}
```

- Get `GPIOTE` reference
- Check what happened
- Set flags accordingly
- **Dont forget: Clear event**