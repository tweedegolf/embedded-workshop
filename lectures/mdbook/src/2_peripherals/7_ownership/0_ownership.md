# Peripheral ownership

- From code standpoint, peripherals are just registers
- Can be read and written to from anywhere

## Problems

- conflicting configuration
- data races

*`svd2rust` solves this using Rusts ownership model!*