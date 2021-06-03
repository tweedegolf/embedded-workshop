# Global state
**Step 2: Initialize globals used in ISR**

```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/interrupt_example.rs:statics}}
```

- Atomics for flags
- `Mutex<RefCell<Option<T>>>` for non-`Sync + Send` data

