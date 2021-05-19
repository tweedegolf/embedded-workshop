# Summary

[Introduction](./preface/introduction.md)
[Course overview](./preface/curriculum.md)
[Setting up](./preface/setup.md)

- [Anatomy of a Rust embedded application](./1_anatomy/0_summary.md)
    - [Setting up a project](./1_anatomy/1_setup.md)
    - [Contents of a Rust embedded development environment](./1_anatomy/2_dev_env.md)
    - [Structure](./1_anatomy/3_structure/0_structure.md)
        - [#!\[no_std\] and #!\[no_main\]](./1_anatomy/3_structure/1_attrs.md)
        - [Entry](./1_anatomy/3_structure/2_entry.md)
        - [Panic handler](./1_anatomy/3_structure/3_panic_handler.md)
        - [Minimal Rust embedded program](./1_anatomy/3_structure/4_structure_all.md)
    - [HALs and PACs](./1_anatomy/4_hal_pac/0_hal_pac.md)
        - [Peripherals](./1_anatomy/4_hal_pac/1_peripherals.md)
        - [Peripheral Access Crates (PACs)](./1_anatomy/4_hal_pac/2_pac.md)
        - [Hardware Abstraction Libraries (HALs)](./1_anatomy/4_hal_pac/3_hal.md)
        - [A typical embedded Rust program](./1_anatomy/4_hal_pac/4_typical_program.md)

    - [Debugging with GDB](./1_anatomy/5_gdb_debug/0_gdb_debug.md)
        - [GDB](./1_anatomy/5_gdb_debug/2_gdb.md)
        - [GDB commands](./1_anatomy/5_gdb_debug/3_gdb_commands.md)
        - [GDB example](./1_anatomy/5_gdb_debug/4_gdb_example.md)
    - [Questions](./1_anatomy/6_questions.md)
    - [Assignment](./1_anatomy/assignment.md)

- [Peripherals](./2_peripherals/0_summary.md)
    - [Contents](./2_peripherals/contents.md)
    - [Recap](./2_peripherals/recap.md)

    - [Core and peripherals](./2_peripherals/1_peripherals/0_cortex_m_family.md)
        - [nRF52840 block diagram](./2_peripherals/1_peripherals/1_peripherals.md)

    - [General Purpose Input/Output (GPIO)](./2_peripherals/2_gpio/0_gpio.md)
        - [GPIO configuration](./2_peripherals/2_gpio/1_gpio_configuration.md)
        - [GPIO applications](./2_peripherals/2_gpio/2_gpio_applications.md)

    - [Timers/counters (TIMER)](./2_peripherals/3_timer/0_timer.md)
        - [Soft timer](./2_peripherals/3_timer/1_soft_timer.md)
        - [Timer mode](./2_peripherals/3_timer/2_timer_mode.md)
        - [Counter mode](./2_peripherals/3_timer/3_counter_mode.md)
        - [Timer example](./2_peripherals/3_timer/4_example.md)

    - [Break](./2_peripherals/break_1.md)

    - [Universal asynchronous receiver/transmitter (UART)](./2_peripherals/4_uart/0_uart.md)
        - [UART timing](./2_peripherals/4_uart/1_uart_timing.md)
        - [UART configuration](./2_peripherals/4_uart/2_uart_configuration.md)
        - [UART example](./2_peripherals/4_uart/3_example.md)

    - [Inter-integrated circuit (I2C)](./2_peripherals/5_i2c/0_i2c.md)
        - [I2C timing](./2_peripherals/5_i2c/1_i2c_timing.md)
        - [I2C exercise](./2_peripherals/5_i2c/2_exercise.md)
        - [I2C example](./2_peripherals/5_i2c/3_example.md)

    - [Serial Peripheral Interface (SPI)](./2_peripherals/6_spi/0_spi.md)
        - [SPI timing](./2_peripherals/6_spi/1_spi_timing.md)
        - [SPI configuration](./2_peripherals/6_spi/2_spi_configuration.md)
        - [SPI exercise](./2_peripherals/6_spi/3_exercise.md)
        - [SPI example](./2_peripherals/6_spi/4_example.md)

    - [Break](./2_peripherals/break_2.md)

    - [Peripheral ownership](./2_peripherals/7_ownership/0_ownership.md)
        - [`Peripherals::take()`](./2_peripherals/7_ownership/1_peripherals_take.md)
        - [HAL abstractions](./2_peripherals/7_ownership/2_hal_abstractions.md)

    - [`embedded_hal`](./2_peripherals/8_embedded_hal/0_embedded_hal.md)
        - [Example driver](./2_peripherals/8_embedded_hal/1_example_driver.md)
        - [Example driver usage](./2_peripherals/8_embedded_hal/2_driver_usage.md)

    - [Reading a datasheet](./2_peripherals/9_datasheets/0_datasheets.md)

    - [Questions](./2_peripherals/questions.md)
    - [Assignment](./2_peripherals/assignment.md)
- [Debugging and analysis]()
- [Interrupts]()
- [Memory layout]()
- [Exceptions (bonus)]()

[Embed.toml](./includes/Embed.toml.md)
[cargo-embed.gdb](./includes/cargo-embed.gdb.md)
[memory.x](./includes/memory.x.md)
