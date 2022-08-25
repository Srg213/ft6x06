
#![no_main]
#![no_std]

/// An example to use access Ft6x06 driver and get coordinates for
/// multiple touch points.

use cortex_m;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
#[cfg(feature = "stm32f413")]
use stm32f4xx_hal::fmpi2c::FMPI2c;
#[cfg(feature = "stm32f412")]
use stm32f4xx_hal::i2c::I2c;
use stm32f4xx_hal::{pac, prelude::*, rcc::Rcc};

#[allow(unused_imports)]
use panic_semihosting;

extern crate ft6x06;

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
            400.kHz(),
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
            400.kHz(),
        )
    };

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
            let t = touch.get_multi_touch(&mut i2c, 1);
            match t {
                Err(_e) => rprintln!("Error fetching touch data"),
                Ok(n) => {
                    rprintln!(
                        "Touch: {:>3}x{:>3} - weight: {:>3} misc: {}",
                        n.touch_x[0],
                        n.touch_y[0],
                        n.touch_weight[0],
                        n.touch_area[0],
                    );
                    rprintln!(
                        "Touch: {:>3}x{:>3} - weight: {:>3} misc: {}",
                        n.touch_x[0],
                        n.touch_y[1],
                        n.touch_weight[1],
                        n.touch_area[1],
                    )
                }
            }
        }
    }
}
