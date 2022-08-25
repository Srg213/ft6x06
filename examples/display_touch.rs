// #![deny(warnings)]
#![no_main]
#![no_std]
#![allow(unused_variables)]

use cortex_m;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
#[cfg(feature = "stm32f412")]
use stm32f4xx_hal::fsmc_lcd::ChipSelect1;
#[cfg(feature = "stm32f413")]
use stm32f4xx_hal::fsmc_lcd::ChipSelect3;
use stm32f4xx_hal::{
    fsmc_lcd::{FsmcLcd, LcdPins, Timing},
    pac,
    prelude::*,
    rcc::Rcc,
};

use embedded_graphics::{
    pixelcolor::Rgb565,
    prelude::*,
    primitives::{Circle, PrimitiveStyle},
};

#[cfg(feature = "stm32f413")]
use stm32f4xx_hal::fmpi2c::FMPI2c;
#[cfg(feature = "stm32f412")]
use stm32f4xx_hal::i2c::I2c;

#[allow(unused_imports)]
use panic_semihosting;

use ft6x06;
use st7789::*;

/// A simple example to connect to the FT6x06 crate and access it for
/// x and y positions of touch points. There are a lot of commented-out
/// calls to items in the library, but they're a bit pointless. I couldn't
/// get the gesture stuff to work - I couldn't even get an I2C register change
/// to take place. I didn't try for the other functions like Events.
///
/// It works for me - if you get more working, please send a PR.
/// My approach to Results is also a bit ad-hoc.
#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Started");

    let p = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc: Rcc = p.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(100.MHz()).freeze();
    let mut delay = cp.SYST.delay(&clocks);

    let gpiob = p.GPIOB.split();
    let gpioc = p.GPIOC.split();
    let gpiod = p.GPIOD.split();
    let gpioe = p.GPIOE.split();
    let gpiof = p.GPIOF.split();
    let gpiog = p.GPIOG.split();

    let lcd_pins = LcdPins {
        data: (
            gpiod.pd14.into_alternate(),
            gpiod.pd15.into_alternate(),
            gpiod.pd0.into_alternate(),
            gpiod.pd1.into_alternate(),
            gpioe.pe7.into_alternate(),
            gpioe.pe8.into_alternate(),
            gpioe.pe9.into_alternate(),
            gpioe.pe10.into_alternate(),
            gpioe.pe11.into_alternate(),
            gpioe.pe12.into_alternate(),
            gpioe.pe13.into_alternate(),
            gpioe.pe14.into_alternate(),
            gpioe.pe15.into_alternate(),
            gpiod.pd8.into_alternate(),
            gpiod.pd9.into_alternate(),
            gpiod.pd10.into_alternate(),
        ),
        address: gpiof.pf0.into_alternate(),
        read_enable: gpiod.pd4.into_alternate(),
        write_enable: gpiod.pd5.into_alternate(),
        #[cfg(feature = "stm32f413")]
        chip_select: ChipSelect3(gpiog.pg10.into_alternate()),
        #[cfg(feature = "stm32f412")]
        chip_select: ChipSelect1(gpiod.pd7.into_alternate()),
    };

    // Setup the RESET pin
    #[cfg(feature = "stm32f413")]
    let rst = gpiob.pb13.into_push_pull_output();
    // Enable backlight
    #[cfg(feature = "stm32f413")]
    let mut backlight_control = gpioe.pe5.into_push_pull_output();

    #[cfg(feature = "stm32f412")]
    let rst = gpiod.pd11.into_push_pull_output();
    #[cfg(feature = "stm32f412")]
    let mut backlight_control = gpiof.pf5.into_push_pull_output();

    backlight_control.set_high();
    // We're not using the "tearing" signal from the display
    let mut _te = gpiob.pb14.into_floating_input();

    // Set up timing
    let write_timing = Timing::default().data(3).address_setup(3).bus_turnaround(0);
    let read_timing = Timing::default().data(8).address_setup(8).bus_turnaround(0);

    // Initialise FSMC memory provider
    let (_fsmc, interface) = FsmcLcd::new(p.FSMC, lcd_pins, &read_timing, &write_timing);

    // Pass display-interface instance ST7789 driver to setup a new display
    let mut disp = ST7789::new(interface, rst, 240, 240);

    // Initialise the display and clear the screen
    disp.init(&mut delay).unwrap();
    rprintln!("{}", disp.orientation() as u8);
    disp.clear(Rgb565::BLACK).unwrap();

    //Intializing the i2c bus for touchscreen

    rprintln!("Connecting to I2c");

    #[cfg(feature = "stm32f412")]
    let mut i2c = {
        I2c::new(
            p.I2C1,
            (
                gpiob.pb6.into_alternate().set_open_drain(),
                gpiob.pb7.into_alternate().set_open_drain(),
            ),
            400.kHz(),
            &clocks,
        )
    };

    #[cfg(feature = "stm32f413")]
    let mut i2c = {
        FMPI2c::new(
            p.FMPI2C1,
            (
                gpioc.pc6.into_alternate().set_open_drain(),
                gpioc.pc7.into_alternate().set_open_drain(),
            ),
            400.kHz(),
        )
    };

    //ft6x06 driver

    let mut touch = ft6x06::Ft6X06::new(&i2c, 0x38).unwrap();

    let tsc = touch.ts_calibration(&mut i2c, &mut delay);
    match tsc {
        Err(e) => rprintln!("Error {} from ts_calibration", e),
        Ok(u) => rprintln!("ts_calibration returned {}", u),
    }
    rprintln!("If nothing happens - touch the screen!");
    // for _i in 0..3000 {
    loop {
        let t = touch.detect_touch(&mut i2c);
        let mut num: u8 = 0;
        match t {
            Err(e) => rprintln!("Error {} from fetching number of touches", e),
            Ok(n) => {
                num = n;
                if num != 0 {
                    rprintln!("Number of touches: {}", num)
                };
            }
        }

        if num > 0 {
            let t = touch.get_touch(&mut i2c, 1);

            match t {
                Err(_e) => rprintln!("Error fetching touch data"),
                Ok(n) => {
                    rprintln!(
                        "Touch: {:>3}x{:>3} - weight: {:>3} misc: {}",
                        n.x,
                        n.y,
                        n.weight,
                        n.misc
                    );
                    // Circle with 1 pixel wide white stroke with top-left point at (10, 20) with a diameter of 3
                    Circle::new(
                        Point::new(
                            <u16 as Into<i32>>::into(n.y),
                            240 - <u16 as Into<i32>>::into(n.x),
                        ),
                        20,
                    )
                    .into_styled(PrimitiveStyle::with_stroke(Rgb565::RED, 1))
                    .draw(&mut disp)
                    .unwrap();
                }
            }
        }
    }
}
