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
cd embeddded-workshop
```
Or, if you like to use HTTPS instead:
```bash
git clone https://github.com/tweedegolf/embedded-workshop.git
cd embeddded-workshop
```

Then, we'll install some tools needed to flash the mcu and inspect the code:
```bash
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
To test the hardware, please connect the nucleo board to your pc and run
```bash
cargo run --release -p examples --bin test
```

If everything works correctly, you should now see the board LEDs flashing, as well as some console output. If not, don't worry and contact us.

Press button 1 to complete the test.

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
