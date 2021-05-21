<div class="read">

# Assignment 2
For this assignment, you can use `assignments/accel_display/main.rs` as an outline.

## Write a message to the screen
- First, let's hook up the SSD1306 OLED screen if you haven't already done so yet. The best way is to do that using your breadboard. Connect it like this:

| SSD1306 pin | nRF52840DK pin | Initial level | SPI pin |
| ----------- | -------------- | ------------- | ------- |
| VCC         | VDD            | -             |         |
| GND         | GND            | -             |         |
| D0 (CLK)    | p1.07          | Low           | SCK     |
| D1 (DIN)    | p1.08          | Low           | MOSI    |
| RES         | p1.04          | High          |         |
| DC          | p1.05          | Low           |         |
| CS          | p1.06          | High          | NCS     |

- Then, you'll need to initialize the pins. All of the pins we're using to control the display, should be put in push-pull output mode, using `into_push_pull_output`. Refer to the previous table for the initial output levels, as well as to the [GPIO example](./2_gpio/1_gpio_configuration.md).

- Next, we'll initialize the SPIM0 peripheral. Refer to the [SPI example](./6_spi/4_example.md) if you need inspiration. Don't forget to check the pins if you are just going to copypasta.

- Initialize the display in Terminal mode with SPI:
```rust,noplaypen
use ssd1306::{prelude::*, Builder};
let interface = SPIInterface::new(spi, dc_pin, not_chip_sel);
let mut disp: TerminalMode<_, _> = Builder::new().connect(interface).into();
disp.init().unwrap(); // Don't forget to initialize!
```

- Write some cool message to the display. Just like in the [UART example](./4_uart/3_example.md), we can use the fact that the display in `TerminalMode` implements `Core::fmt::Write`.

## Read data from the accelerometer

- Connect the LIS3DH accelerometer for I2C usage:

| LIS3DH Pin | nRF52840DK pin |
|------------|----------------|
| VCC        | VDD            |
| 3vo        | -              |
| GND        | GND            |
| SCL        | P0.27          |
| SDA        | P0.26          |
| SDO        | -              |
| CS'        | -              |
| INT        | -              |
| A1         | -              |
| A2         | -              |
| A3         | -              |

- Initialize the SCL and SDA pins. Due to some [peculiarities](https://github.com/nrf-rs/nrf-hal/blob/master/nrf-hal-common/src/twim.rs#L45) of the nRF52840, they need to be in floating input mode.

- Initialize the TWIM1 peripheral, which is I2C compatible. Use the [I2C example](./5_i2c/3_example.md) for reference.

- Set up the `lis3dh` driver:
```rust,noplaypen
let mut lis3dh = Lis3dh::new(i2c, SlaveAddr::Default).unwrap();
```

- Read a sample and log it using `dfmt::info!(..)`. Reading a normalized sample is done though the [`Accelerometer` trait](https://docs.rs/accelerometer/0.12.0/accelerometer/trait.Accelerometer.html), which `Lis3dh` implements.

- Use the SSD1306 driver in `TerminalMode` to write samples to the display. Don't forget to clear the screen before writing new data.

## Show an image on the display
- Use and adapt the [image example](https://github.com/jamwaffles/ssd1306/blob/master/examples/image_i2c.rs) to show the Rust logo on the display. Or, perhaps another image of your liking?

- Update the image position according to the accelerometer orientation in real time.

## Free for all!
- Now that you know how to read accelerometer sample, put text and images on the display, read button states and control LEDS, use your imagination again to create another cool game! Or, of course, if you have a better idea of how to get your knowledge into action, go right ahead! Ask me or your peers if you need any help or ideas.

## Bonus: Writing an SPI driver for the LIS3DH accelerometer


The LIS3DH accelerometer can be communicated to over SPI as well as I2C, it's just that the driver doesn't support that. Using the [`lis3dh` source code](https://github.com/BenBergman/lis3dh-rs) and the lecture slides, write an application that first reads out the device ID, configures the device, and then reads samples. Don't use I2C like the driver does, but use SPI. I'd advise to fork the `lis3dh` repository, so you can use the register definitions in there.

Connect the LIS3DH for SPI usage:

| LIS3DH Pin | nRF52840DK pin |
|------------|----------------|
| VCC        | VDD            |
| 3vo        | -              |
| GND        | GND            |
| SCL (SCK)  | P0.27          |
| SDA (MOSI) | P0.26          |
| SDO (MISO) | p1.15          |
| CS' (NCS)  | p1.14          |
| INT        | -              |
| A1         | -              |
| A2         | -              |
| A3         | -              |

Basically, you need to implement the following functionality:
- Low level function that writes bytes to a given register, like [`write_register`](https://github.com/BenBergman/lis3dh-rs/blob/46c74382d8f87da5a234d932f67d5b2a0166b8b3/src/lib.rs#L305)

- Low level function that reads bytes from a given register, like [`read_register`](https://github.com/BenBergman/lis3dh-rs/blob/46c74382d8f87da5a234d932f67d5b2a0166b8b3/src/lib.rs#L316)

- A way to get the device id, like [`get_device_id`](https://github.com/BenBergman/lis3dh-rs/blob/46c74382d8f87da5a234d932f67d5b2a0166b8b3/src/lib.rs#L91)

- Configuring the device, like [`new`](https://github.com/BenBergman/lis3dh-rs/blob/46c74382d8f87da5a234d932f67d5b2a0166b8b3/src/lib.rs#L65)
    - Setting the device in HighResulution mode
    - Enabling all axes
    - Setting the datarate to 400Hz

*This exercise is quite a challenge. Please do ask any questions in the group chat, so you, your peers, and I can help each other!*

</div>