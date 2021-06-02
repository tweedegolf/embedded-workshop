# What to take into account

- Like multithreading but higher prio pre-empts lower prio.
- Avoid non-reentrant function calls
- Make ISR as short as possible
    - Use flags
    - Avoid locks
- Dont forget to unpend interrupt