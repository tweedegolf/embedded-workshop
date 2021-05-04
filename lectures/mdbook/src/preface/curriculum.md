<div class="read">

# Curriculum
Below you'll find an overview of the learning outcomes of each of the workshops.
## 1 Anatomy of a Rust Embedded application (0,5d)
### Subjects
- Constraints of no_std environment.
- Structure of a typical Rust Embedded application 
- Purpose of HAL/PAC crates
- Purposes of cortex-m(-rt) crates
- Setting up a development environment (probe-rs e.d.)
- GDB usage

### Assignment
Blinky with button

### Learning outcomes
- Constraints of no_std programming
- Structure of embedded rust applications
- Relationship HAL and PAC
- Purpose of cortex-m and cortex-m-rt
- Setting up a dev env with probe-rs
- Debugging with GDB

## 2 Peripherals
### Subjects
- SPI
- I2C
- UART
- Timers
- Rust ownership and peripherals
- embedded-hal
- drivers

### Assignment
Read out lis3dh using I2C, show data on SSD1306 using SPI

### Learning outcomes
- Relationship between core - busses - peripherals
- Properties of the protocols
- Why the Rust ownership model and peripherals are great together
- How to write platform independent device drivers using `embedded-hal`

## 3 Debugging and analysis (live only)
- Saleae Logic analyzer
- Power optimization With Otii arc

### Assignment
Debug and optimize an SPI/I2C driver

### Learning outcomes
- How to analyze usung a Seleae Logic?
- How to analyze usung an Otii Arc?
- How to optimize for power usage?

## 4 Interrupts
### Subjects
- cortex-m-rt
- NVIC
- EXTI
- TIMER
- RTIC vs Bare Metal

### Assignment 
Configure fall detection IRQ on lis3dh, catch it using EXTI, read out data

### Learning outcomes
- What are interrupts?
- How are interrupts implemented in Cortex M?
- Interrupt priority
- What to take into account when enabling interrupts
- How to configure interrupts using RTIC
- How to configure interrupts using the HAL

## 5 Memory layout
- Types of memory
- Memory map
- MPU
- Linker scripts
- Bootloaders

### Assignment
Write a simple bootloader

### Learning outcomes
- Which types of memory are available
- What is the memory layout
- How to read a memory map
- What's the purpose of an MPU
- How to configure where your code is flashed in memory and how to manipulate stack position and such using a linker script

## 6 Exceptions (bonus)
- HardFault/UsageFault/MemManage/BusFault

### Assignment
Debug a couple of HardFaults using GDB

### Learning outcomes
- When are exceptions thrown?
- Which exception is thrown when?
- How to debug an exception

</div>