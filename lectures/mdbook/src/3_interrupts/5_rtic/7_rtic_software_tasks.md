# Software tasks
```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/rtic_example.rs:set_led_2_state}}
```

- Piece of work that can be spawned from other contexts
- Message queue for passing parameters
    - default capacity 1
- Uses selected number of resources
- User-defined priority (higher value = higher priority)

```rust,noplaypen
{{#rustdoc_include ../../../../src/bin/rtic_example.rs:extern_interrupts}}
```
- RTIC uses unused interrupts to spawn software tasks
- 'Sacrifice' them using `extern` block.