<div class="read">

# Setting up your environment
Before we begin, you'll need to check your parts, install some software, and verify that everything works the way it should.
## Hardware
You should have received the following parts:

- nRF52840-DK
- Breadboard
- LIS3DH Breakout board
- SSD1306 0.96" OLED display
- Male-to-male breadboard wires

You'll also need a Micro-USB cable, but I'm sure you've got one to spare.

Please check that everything is complete. If not, please contact us.

## Software
Next, we'll need the software needed to build and flash the firware we're going to create.

Please install Rustup, using [the instructions here](https://rustup.rs/).

Make sure you've got the latest stable rust version:
```bash
rustup default stable
rustup update
```

Install the `thumbv7em-none-eabi` toolchain with the following command:
```bash
rustup target add thumbv7em-none-eabi
```

For the rest of the steps, you'll need the [source code of this workshop](https://github.com/tweedegolf/embedded-workshop).

```bash
git clone git@github.com:tweedegolf/embedded-workshop.git
cd embedded-workshop
```
Or, if you like to use HTTPS instead:
```bash
git clone https://github.com/tweedegolf/embedded-workshop.git
cd embedded-workshop
```

Then, we'll install some tools needed to flash the mcu and inspect the code:
```bash
sudo apt install -y libusb-1.0-0-dev libftdi1-dev
cargo install --force flip-link cargo-binutils cargo-flash cargo-embed probe-run
```

If you're on linux, you'll need to update your udev rules.
On ubuntu, run the following

```bash
sudo cp 99-jlink-nrf52840dk.rules /etc/udev/rules.d
sudo udevadm control --reload-rules
```

Then, switch the nRF52840DK on and off or remove the cable and plug it in again.

For debugging, you can use GDB. On ubuntu, it can be installed with
```bash
sudo apt update
sudo apt install gdb-multiarch
```

## Testing
Before we begin, we need to test our hardware. We'll be testing the LIS3DH accelerometer, the SSD1306 display, as well as the nRF52840DK board. Make sure you have checked out the latest version of the workshop source.

### LIS3DH accelerometer connection
First, let's wire up the LIS3DH accelerometer for I2C usage. Place it on your breadboard, with each row of pins on a separate side of the gutter.
As you can see in the image below, the rows on the breadboard labeled by a number are connected. E.g. hole a1 is connected to hole b1 and c1, but not to a2 or a60. Also, both columns on the sides of the breadboards (with the blue (-) and red (+) lines) are connected. Please keep this in mind when wiring up the display and the accelerometer, as shorting the contacts might break your components.

<img src="https://upload.wikimedia.org/wikipedia/commons/e/e8/Breadboard.png" width="40%"/>

An example of a correct placement of the components on the breadboard:
<img src="/assets/images/breadboard-components.jpeg" width="20%"/>

**Turn off your nRF52840DK**. Then, wire up the accelerometer, referring to the table below.

| LIS3DH Pin | nRF52840DK pin |
|------------|----------------|
| VIN (+)    | VDD            |
| 3vo        | -              |
| GND (-)    | GND            |
| SCL        | P0.27          |
| SDA        | P0.26          |
| SDO        | -              |
| CS'        | -              |
| INT        | -              |
| A1         | -              |
| A2         | -              |
| A3         | -              |

*We'll be using the other pins later on, but they're not needed to test the hardware*

### SSD1306 Display connection
**Turn off your nRF52840DK**. Place the SSD1306 on your breadboard, taking care that the pins are not connected to each other. Then, wire it up using the table below:

| SSD1306 pin | nRF52840DK pin |
| ----------- | -------------- |
| VCC (+)     | VDD            |
| GND (-)     | GND            |
| NC          | -              |
| D0 (CLK)    | p1.07          |
| D1 (DIN)    | p1.08          |
| RES         | p1.04          |
| DC (D/C)    | p1.05          |
| CS          | p1.06          |

***Verify once more that all pins have been correctly connected, and that they are not connected to each other***

### Running the test

To test the hardware, please connect the nrf board to your pc, turn it on, and run
```bash
cargo run --release -p examples --bin test
```

If everything works correctly, you should now see the accelerometer samples being printed on the display. If not, don't worry and contact us.

## Docs
Datasheets, manuals, and schematics of the parts we are using in this workshop.
### nRF52840
- [nRF52840DK documentation](https://infocenter.nordicsemi.com/topic/ug_nrf52840_dk/UG/dk/intro.html)
- [nRF52840 product specification](https://infocenter.nordicsemi.com/pdf/nRF52840_PS_v1.2.pdf)

### SSD1306
- [Datasheet](https://cdn-shop.adafruit.com/datasheets/SSD1306.pdf)
- [Schematic](https://cdn-learn.adafruit.com/assets/assets/000/093/884/original/adafruit_products_0-96in_OLED_sch.png?1596746114) Pin labels differ a bit, but the same set of pins is available.

### LIS3DH
- [Datsheet](https://cdn-learn.adafruit.com/assets/assets/000/085/846/original/lis3dh.pdf?1576396666)
- [Schematic](https://cdn-learn.adafruit.com/assets/assets/000/028/587/original/sensors_sch.png?1447888851)
</div>
