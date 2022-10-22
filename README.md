

# FT6x06 driver

## TouchPanel Controller
FT6x06 is Self-Capacitive Touch Panel Controller used on many boards manufactured by ST.
The FT6x06 Series ICs are single-chip capacitive touch panel controller ICs with a built-in 8 bit enhanced Micro-controller unit (MCU). They adopt the self-capacitance technology, which supports single point and gesture touch. This is built on top of embedded-hal and implements blocking I2C module.

The FT6x06 series ICs include FT6206 /FT6306.

## Description 
This repository is a device driver for FT6x06 written in Rust which was reverse engineered from C code in ST's component driver.
This repository, that is built upon embedded-hal, provides tools to access the touchpanel controller in order to retrieve touch coordinate and gesture data via the microcontroller's I2C bus.
This repo also contains examples for FT6206 IC built on top of abstraction layer for STM32F4 devices-stm32f4xx-hal for the boards
`STM32F412/413` boards. Documentation of touchscreen controllers on this boards is not well written.
This is mainly based on the STMicroelectronics github page- https://github.com/STMicroelectronics/stm32-ft6x06. 

Many boards manufactured by STMicroelectronics use the Touch Panel Controller, model number FT6x06. The single-chip FT6x06 Series ICs are capacitive touch panel controllers with an integrated 8 bit improved micro-controller unit.


## Example
More examples of how to use the touch panel component of the *STM32F412/13* boards are included.
`example/interface` demonstrates how the display and touch panel could be used to create a User Interface for an embedded board.
To run an example, 

-   connect to an STM32F413 Discovery board via the ST_Link port (the USB- mini type B port)
-   haves some Rust tools installed and switch to nightly channel, 
-   run the command:  `cargo run --features stm32f413,fsmc_lcd --example interface`


### Version 0.1.1
Issue- Sometimes, STM32F413 would not respond while initializing I2C bus.
Perform a long hard reset, the FT66206 needs at least 5mS ...

 - On the STM32F413 the touchscreen shares the reset GPIO pin w/ the LCD.
 - The ST7789 driver uses a fast (10uS) reset.
 - The touchscreen controller needs 5mS.
 
### Version 0.1.2
Issue- The touchscreen hangs after waiting for longer than 30 second for user input. The touchscreen controller was going to sleep after 30 seconds. 
A solid fix appears to be to poll the touchscreen interrupt output to the stm32 and only attempt to read the registers when the touchscreen controller indicates there are touches waiting.

The ft6x06 indicates when touches are ready by manipulating an interrupt line.
By wait-polling this interrupt before reading the touch status registers we avoid spurious errors when the touchscreen controller becomes dormant (after not being touched for a while).

 - Added function to Wait for the touchscreen interrupt to indicate touches
