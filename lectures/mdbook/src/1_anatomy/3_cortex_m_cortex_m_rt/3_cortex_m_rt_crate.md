# The `cortex-m-rt` crate

> [Startup code and minimal runtime for Cortex-M microcontrollers](https://docs.rs/cortex-m-rt/0.6.13/cortex_m_rt/)

- Defines the program's vector table
- Initializes `static` variables before program start
- Enables the Floating Point Unit (FPU) if applicable
- Provides the `#[entry]` attribute
- Provides the `#[exception]` attribute to override exception handlers
- Provides the `#[pre-init]` attribute to run code before initializing `static` variables

**Questions so far?**