# Interrupts in Rust

**`cortex-m` crate**

- Defines where ISRs are located in flash
- Uses PAC to generate ISR code
- Defines `#[interrupt]` macro for ISRs

## Basic steps
  1. Configure peripherals for interrupt source 
  1. Initialize globals used in ISR
  1. Define interrupt handlers 
  1. Set interrupt priority
  1. Unmask interrupt in NVIC