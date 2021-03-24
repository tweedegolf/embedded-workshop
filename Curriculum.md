# Curriculum

## 1 Anatomy of a Rust Embedded application (0,5d)
### Subjects
- no_std
- HAL/PAC
- cortex-m(-rt)
- dev env (probe-rs e.d.)
- GDB usage
- optimisation

### Assignment
Blinky with busy waiting

### Learning outcomes
- Constraints of no_std programming
- Structure of embedded rust applicaties
- Relationship HAL and PAC
- Purpose of cortex-m and cortex-m-rt
- Setting up a dev env with probe-rs
- Debugging with GDB
- First intuitions on the implications of optimization

### Preparation
- Slides + story - (6h)
- Project set up (2h)
- Prepare GDB configuration (2h)

## 2 Peripherals (1d)
### Subjects
- SPI
- I2C
- USART
- RTC
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

### Preparation
- Read up on peripherals L476 (4h)
- Slides + story (6h)
- Project set up (2h)

## 3 Debugging and analysis (0.5d)
- Saleae Logic analyzer
- Power optimization With Otii arc

### Assignment
Debug and optimize an SPI/I2C driver

### Learning outcomes
- How to analyze usung a Seleae Logic?
- How to analyze usung an Otii Arc?
- How to optimize for power usage?

### Preparation
- Project set up (2h)

## 4 Interrupts (1d)
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

### Voorbereiden
- Read up on L476 interrupts (4h)
- Slides + story (4h)
- RTIC-based project set up (2h)
- Bare-metal project set up (2h)

## 5 Memory layout (1d)
- Yypes of memory
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

## 6 Exceptions (bonus) (0.5d)
- HardFault/UsageFault/MemManage/BusFault

### Assignment
Debug a couple of HardFaults using GDB

### Learning outcomes
- When are exceptions thrown?
- Which exception is thrown when?
- How to debug an exception

### Voorbereiden
- Read up on Cortex-M exceptions (4h)
- Slides + story (4h)
- Project set up (2h)



