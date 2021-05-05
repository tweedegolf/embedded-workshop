# Common GDB commands

- `c` (`continue`): continue running until next breakpoint
- `s` (`step`): step to next line of code
- `si` (`stepi`): step to next instruction
- `n` (`next`): step over to next line of code (does not enter functions)
- `fin` (`finish`): continue current function until it returns
- `bt` (`backtrace`): prints a stack backtrace
- `i lo` (`info locals`): lists local variables and their values
- `i va` (`info variables`): lists info about all variables
- `p` (`print`): print value of local variable. Rust-style pointer derefencing and casting is supported
- `l` (`list`): show code around the current instruction
