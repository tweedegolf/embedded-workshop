# Schedule a task somewhere in the future

- Requires declaring a monotonic timer in app attribute
- Uses the SysTick interrupt handler
- Cycle counter is easy to use
```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/rtic_example.rs:app_attr}}
```

- Enable the cycle counter
```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/rtic_example.rs:cyccnt_enable}}
```

- Takes up space in task message queue
