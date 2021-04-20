# Assignment 1

## Get the LEDS working
- Initialize the LEDS, and switch them on. To do so, use the table below to find out which pin is connected to which LEDs, put the pin in push-pull output mode, and set its state to low. The LEDs are active low, meaning that they are switched on with a low pin state, and switched off by setting a pin to high.

- Use busy waiting as a means of delaying the code, and have the LEDs flash in some cool fashion.

## Read button state
- Initialize the buttons, and read their state. Find out which pin is connected to which button, put the pin in pull-up input mode, and read its state. The buttons are active low, meaning they pull the signal to low state when pressed.

- Update the LED pin output states based on whether a button is being pressed.

## Create a simple game using the buttons and the LEDs
- Use your imagination! If you need any inspiration, just ask me or your peers.


**LED and Button connections**

| Item | GPIO Pin |
| ---- | -------- |
| LED1 | P0.13    |
| LED2 | P0.14    |
| LED3 | P0.15    |
| LED4 | P0.16    |
| BTN1 | P0.11    |
| BTN2 | P0.12    |
| BTN3 | P0.24    |
| BTN4 | P0.20    |

*Tip: look up the nrf-hal examples*
