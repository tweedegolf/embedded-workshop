# What to take into account

**Like multithreading**
- ... but higher priority pre-empts lower priority code
- Avoid non-reentrant function calls
- Make ISR as short as possible
    - Use flags
    - Avoid locks
- Don't forget to clear event