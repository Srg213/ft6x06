// #![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
#[cfg(feature = "stm32f413")]
use stm32f4xx_hal::fmpi2c::FMPI2c;
#[cfg(feature = "stm32f412")]
use stm32f4xx_hal::i2c::I2c;
use stm32f4xx_hal::{pac, prelude::*, rcc::Rcc};

use panic_semihosting as _;

extern crate ft6x06;

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

    let perif = pac::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let rcc: Rcc = perif.RCC.constrain();

    let clocks = rcc.cfgr.sysclk(100.MHz()).freeze();
    let mut delay = cp.SYST.delay(&clocks);

    rprintln!("Connecting to I2c");

    #[cfg(feature = "stm32f412")]
    let mut i2c = {
        let gpiob = perif.GPIOB.split();
        I2c::new(
            perif.I2C1,
            (
                gpiob.pb6.into_alternate().set_open_drain(),
                gpiob.pb7.into_alternate().set_open_drain(),
            ),
            80.kHz(),
            &clocks,
        )
    };

    #[cfg(feature = "stm32f413")]
    let mut i2c = {
        let gpioc = perif.GPIOC.split();
        FMPI2c::new(
            perif.FMPI2C1,
            (
                gpioc.pc6.into_alternate().set_open_drain(),
                gpioc.pc7.into_alternate().set_open_drain(),
            ),
            80.kHz(),
        )
    };

    let mut touch = ft6x06::Ft6X06::new(&i2c, 0x38).unwrap();

    // for _i in 0..3000 {
    loop {
        let t = touch.gest_logic(&mut i2c);

        match t {
            Err(e) => rprintln!("Error {}", e),
            Ok(a) => rprintln!("{}", a as str),
        }
    }
}
