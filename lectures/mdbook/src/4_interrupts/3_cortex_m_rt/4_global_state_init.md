# Global state initialization
**Step 2: Initialize globals used in ISR**

```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/interrupt_example.rs:init_globals}}
```

- `interrupt_free` needed to use mutex, disables interrupts
