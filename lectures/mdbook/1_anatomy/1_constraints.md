# Constraints

- No heap
- No thread abstractions, no sockets, no filesystem etc.
- No command line argunents etc.

Rusts std crate assumes existence all of these, so we can't use it.
