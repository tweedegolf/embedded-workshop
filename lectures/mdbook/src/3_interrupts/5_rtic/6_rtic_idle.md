# #[idle] The idle task
```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/rtic_example.rs:idle_task}}
```

- Defines what happens when there is nothing to do
- Go to sleep
- Interrupted by any other task