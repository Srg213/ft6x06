// #![deny(warnings)]
#![no_main]
#![no_std]

use cortex_m;
use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal::{
    i2c::I2c,
    pac,
    prelude::*,
    rcc::{ Rcc},
};

#[allow(unused_imports)]
use panic_semihosting;

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
    let gpioc = perif.GPIOC.split();
    //let scl = gpiob.pb10.into_alternate().set_open_drain(); //LCD_SCL
    //let sda = gpiob.pb11.into_alternate().set_open_drain(); //LSD_SDA

    let gpiob = perif.GPIOB.split();
    let mut i2c = I2c::new(
            perif.I2C1,
            (
                gpiob.pb6.into_alternate().set_open_drain(),
                gpiob.pb7.into_alternate().set_open_drain(),
            ),
            400.kHz(),
            &clocks,
        );
    
    let mut touch = ft6x06::Ft6X06::new(&i2c, 0x38, &mut delay).unwrap();

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
                Ok(n) => rprintln!(
                    "Touch: {:>3}x{:>3} - weight: {:>3} misc: {}",
                    n.x,
                    n.y,
                    n.weight,
                    n.misc
                ),
            }
        }
    }
}
// touch.test(&mut i2c);
// rprintln!("Returned from test");

// rprintln!("{:?}", touch.get_capabilities());

// let dm = touch.dev_mode_r(&mut i2c);
// match dm {
//     Err(_e) => rprintln!("Error {} returned from i2c"),
//     Ok(u) => rprintln!("dev_mode_r returned {}", u),
// }

// let dmw = touch.dev_mode_w(&mut i2c, FT6x06_DEV_MODE_FACTORY);
// match dmw {
//     Err(_e) => rprintln!("Error {} returned from writing dev mode"),
//     Ok(u) => rprintln!("dev_mode_w returned {}", u),
// }

// let tds = touch.td_status(&mut i2c);
// match tds {
//     Err(_e) => rprintln!("Error returned when querying status"),
//     Ok(u) => rprintln!("td_status returned {}", u),
// }

// let tsc = touch.ts_calibration(&mut i2c);
// match tsc {
//     Err(e) => rprintln!("Error {} from ts_calibration", e),
//     Ok(u) => rprintln!("ts_calibration returned {}", u),
// }

// let cid = touch.chip_id(&mut i2c);
// match cid {
//     Err(e) => rprintln!("Error {} from chip_id", e),
//     Ok(u) => rprintln!("Chip ID is: 0x{:02x}", u),
// }

// let fid = touch.firmware_id(&mut i2c);
// match fid {
//     Err(e) => rprintln!("Error {} from chip_id", e),
//     Ok(u) => rprintln!("Firmware ID is: 0x{:02x}", u),
// }

// match touch.gesture_radian_read(&mut i2c) {
//     Err(e) => rprintln!("Error {} from gesture_radian_read", e),
//     Ok(u) => rprintln!("*Gesture_radian_read ID is: 0x{:02x}", u),
// }

// match touch.gesture_radian_write(&mut i2c, 0x20) {
//     Err(_e) => rprintln!("Error from gesture_radian_write"),
//     Ok(u) => rprintln!("*Okay writing radians, result = {}", u),
// }
// match touch.gesture_radian_read(&mut i2c) {
//     Err(e) => rprintln!("Error {} from gesture_radian_read", e),
//     Ok(u) => rprintln!("*Gesture_radian_read ID is: 0x{:02x}", u),
// }
// let mut gesture_init = ft6x06::GestureInit::new(0x38);

// let gesture_entries: [u8; 6] = [0x20, 0x20, 0x20, 0x80, 0x80, 0x80];
// rprintln!(
//     "Gesture write result: {}",
//     gesture_init
//         .write(
//             &mut i2c,
//             gesture_entries[0],
//             gesture_entries[1],
//             gesture_entries[2],
//             gesture_entries[3],
//             gesture_entries[4],
//             gesture_entries[5],
//         )
//         .unwrap()
// );

// gesture_init.read(&mut i2c).ok();
// rprintln!(
//     "Gestures as initialized
//            Radians: {}
//            Offset left/right: {}
//            Offset up/down: {}
//            Distance left/right: {}
//            Distance up/down: {}
//            Distance zoom: {}",
//     gesture_init.radian,
//     gesture_init.offset_left_right,
//     gesture_init.offset_up_down,
//     gesture_init.distance_left_right,
//     gesture_init.distance_up_down,
//     gesture_init.distance_zoom
// );
//     let g = touch.get_gesture(&mut i2c);
//     if let Err(_e) = g {
//         rprintln!("Error in getting gesture");
//     } else if let Ok(a) = g {
//         rprintln!("Returned from get_gesture: {}", print_gesturekind(a));
//     }

//     match touch.gesture_radian_read(&mut i2c) {
//         Err(a) => rprintln!("Error {} reading radians", a),
//         Ok(b) => rprintln!("Value read from radians: {}", b),
//     }

//     touch.delay_ms(10);

//     match touch.gesture_radian_write(&mut i2c, 0x20) {
//         Err(_a) => rprintln!("Error writing radians"),
//         Ok(_b) => rprintln!("Value written to radians okay"),
//     }

//     touch.delay_ms(10);

//     match touch.gesture_radian_read(&mut i2c) {
//         Err(a) => rprintln!("Error {} reading radians", a),
//         Ok(b) => rprintln!("Value read from radians: {}", b),
//     }
//     touch.delay_ms(100);
// }

//     let g = touch.get_gesture(&mut i2c);
//     if let Err(e) = g {
//         rprintln!("Error in getting gesture");
//     } else if let Ok(a) = g {
//         // loop {
//         //     match a {
//         //         ft6x06::GestureKind::Fault => break,
//         //         _ => {
//         //             rprintln!("Trying write and read GestureInit");

//         //             let mut gesture_entries: [u8; 6] = [0x20, 0x20, 0x20, 0x80, 0x80, 0x80];
//         //             rprintln!(
//         //                 "Gesture write result: {}",
//         //                 gesture_init
//         //                     .write(
//         //                         &mut i2c,
//         //                         gesture_entries[0],
//         //                         gesture_entries[1],
//         //                         gesture_entries[2],
//         //                         gesture_entries[3],
//         //                         gesture_entries[4],
//         //                         gesture_entries[5],
//         //                     )
//         //                     .unwrap()
//         //             );

//         //             gesture_init.read(&mut i2c);
//         //             rprintln!(
//         //                 "Gestures as initialized
//         //                Radians: {}
//         //                Offset left/right: {}
//         //                Offset up/down: {}
//         //                Distance left/right: {}
//         //                Distance up/down: {}
//         //                Distance zoom: {}",
//         //                 gesture_init.radian,
//         //                 gesture_init.offset_left_right,
//         //                 gesture_init.offset_up_down,
//         //                 gesture_init.distance_left_right,
//         //                 gesture_init.distance_up_down,
//         //                 gesture_init.distance_zoom
//         //             );
//         //         }
//         //     }
//         //     rprintln!(
//         //         "                                Gesture: {}",
//         //         match a {
//         //             ft6x06::GestureKind::None => "None",
//         //             ft6x06::GestureKind::Up => "Up",
//         //             ft6x06::GestureKind::Right => "Right",
//         //             ft6x06::GestureKind::Down => "Down",
//         //             ft6x06::GestureKind::Left => "Left",
//         //             ft6x06::GestureKind::ZoomIn => "ZoomIn",
//         //             ft6x06::GestureKind::ZoomOut => "ZoomOut",
//         //             ft6x06::GestureKind::Fault => "Fault",
//         //         }
//         //     );
//         // }
//     }
// }

//     loop {}
// }

// fn print_gesturekind(g: ft6x06::GestureKind) -> &'static str {
//     match g {
//         ft6x06::GestureKind::None => "None",
//         ft6x06::GestureKind::Up => "Up",
//         ft6x06::GestureKind::Right => "Right",
//         ft6x06::GestureKind::Down => "Down",
//         ft6x06::GestureKind::Left => "Left",
//         ft6x06::GestureKind::ZoomIn => "ZoomIn",
//         ft6x06::GestureKind::ZoomOut => "ZoomOut",
//         ft6x06::GestureKind::Fault => "Fault",
//     }
// }
