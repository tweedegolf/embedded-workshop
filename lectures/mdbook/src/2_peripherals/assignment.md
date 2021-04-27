# Assignment 2

## Write a message to the screen
- First, let's hook up the SSD1306 OLED screen. The best way is to do that using your breadboard. Connect it like this:

| SSD1306 pin | nRF52840DK pin | Initial level |
| ----------- | -------------- | ------------- |
| VCC         | VDD            | -             |
| GND         | GND            | -             |
| D0 (SCK)    | p1.10          | Low           |
| D1 (MOSI)   | p1.11          | Low           |
| RES         | p1.12          | High          |
| DC          | p1.13          | Low           |
| CS (NCS)    | p1.14          | High          |

- Then, you'll need to initialize the pins. All of the pins we're using to control the display, should be put in push-pull output mode, using `into_push_pull_output`. Refer to the previous table for the initial output levels, as well as to the [GPIO example](./2_gpio/1_gpio_configuration.md).

- Next, we'll initialize the SPIM0 peripheral. Refer to the [SPI example](./5_spi/3_example.md) if you need inspiration. Don't forget to check the pins if you are just going to copypasta.

- Initialize the display in Terminal mode with SPI:
```rust,noplaypen
use ssd1306::{prelude::*, Builder};
let interface = SPIInterface::new(spi, dc_pin, not_chip_sel);
let mut disp: TerminalMode<_, _> = Builder::new().connect(interface).into();
disp.init().unwrap(); // Don;t forget!
```

- Write some cool message to the display. Just like in the [UART example](./4_uart/3_example.md), we can use the fact that the display in `TerminalMode` implements `Core::fmt::Write`.

## Read data from the accelerometer

- Connect the LIS3DH accelerometer for I2C usage:

| LIS3DH Pin | nRF52840DK pin |
|------------|----------------|
| VCC        | VDD            |
| 3vo        | -              |
| GND        | GND            |
| SCL        | P1.02          |
| SDA        | P1.01          |
| SDO        | -              |
| CS'        | -              |
| INT        | -              |
| A1         | -              |
| A2         | -              |
| A3         | -              |

- Initialize the SCL and SDA pins. Due to some [peculiarities](https://github.com/nrf-rs/nrf-hal/blob/master/nrf-hal-common/src/twim.rs#L45) of the nRF52840, they need to be in floating input mode.

- Initialize the TWIM0 peripheral, which is I2C compatible. Use the [I2C example](./6_i2c/2_example.md) for reference.

- Set up the `lis3dh` driver:
```rust,noplaypen
let mut lis3dh = Lis3dh::new(i2c, SlaveAddr::Default).unwrap();
```

- Read a sample and log it using `dfmt::info!(..)`. Reading a normalized sample is done though the [`Accelerometer` trait](https://docs.rs/accelerometer/0.12.0/accelerometer/trait.Accelerometer.html), which `Lis3dh` implements.

## Show an image on the display
- Use and adapt the [image example](https://github.com/jamwaffles/ssd1306/blob/master/examples/image_i2c.rs) to show the Rust logo on the display. Or, perhaps another image of your liking?

- Update the image position according to the accelerometer orientation in real time.

## Free for all!
- Now that you know how to read accelerometer sample, put text and images on the display, read button states and control LEDS, use your imagination again to create another cool game! Or, of course, if you have a better idea of how to get your knowledge into action, go right ahead! Ask me or your peers if you need any help or ideas.