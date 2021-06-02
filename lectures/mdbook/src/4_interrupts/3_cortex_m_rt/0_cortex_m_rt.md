# Interrupts with cortex-m-rt and PAC

**Remember?**
```rust,noplaypen
use cortex_m_rt::entry;
#[entry]
```

- Defines where ISRs are located in flash
- Uses PAC to generate ISR code

## Basic steps
  1. Configure peripherals for interrupt source 
  1. Initialize globals used in ISR
  1. Define interrupt handlers 
  1. Set interrupt priority
  1. Unmask interrupt in NVIC