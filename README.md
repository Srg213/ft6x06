
# FT6x06 driver

FT6x06 is Self-Capacitive Touch Panel Controller used on many boards manufactured by ST.
The FT6x06 Series ICs are single-chip capacitive touch panel controller ICs with a built-in 8 bit enhanced Micro-controller unit
(MCU).They adopt the self-capacitance technology, which supports single point and gesture touch. In conjunction with a
self-capacitive touch panel. This is built on top of embedded-hal and implements blocking::I2C module. The examples are built using  

The FT6x06 series ICs include FT6206 /FT6306.

This repo also contains examples for FT6206 IC built on top of abstraction layer for STM32F4 devices-stm32f4xx-hal for the boards
STM32F412 and STM32F413 boards. Documentation of touchscreen controllers on this boards is not well written. I initially forked ft5336 
repo written in rust and modified it to fit to ft6x06.
Other parts of this driver have been reverse-engineered from the code written in C by STMicroelectronics,
