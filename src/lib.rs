
//! A platform agnostic driver for FT6X06 touchscreen .
//! 
//! Built using 'embedded-hal' traits and 'stm32-base' as reference

#![no_std]
#![no_main]

pub mod constant;
use embedded_hal as hal;
use hal::blocking::{delay::DelayMs, i2c};

use core::marker::PhantomData;
use crate::constant::*;


#[derive(Copy, Clone, Debug)]
pub struct Ft6x06Capabilities {
    #[allow(dead_code)]
    multi_touch: bool,
    #[allow(dead_code)]
    gesture: bool,
    #[allow(dead_code)]
    max_touch: u8,
    #[allow(dead_code)]
    max_x_length: u16,
    #[allow(dead_code)]
    may_y_length: u16,
}

const TRUE: bool = true;
const FALSE: bool = false;

const FT6X06_CAPABILITIES: Ft6x06Capabilities = Ft6x06Capabilities {
    multi_touch: TRUE,
    gesture: FALSE,
    max_touch: FT6X06_MAX_NB_TOUCH,
    max_x_length: FT6X06_MAX_X_LENGTH,
    may_y_length: FT6X06_MAX_Y_LENGTH,
};

/// Touch structure - derived from the available I2C registers
/// There are ten available touch registers on the chip, but also
/// a defined maximum of 5 in FT6X06_MAX_NB_TOUCH above.
/// The touch registers occur in banks of 6, for each of the ten
/// potential touches, defined as follows, and the registers are
/// contiguous. That means that a single read can get all of the
/// data for one touch, or all of the data for all the touches.
/// In the absence of documentation on the MISC register, it is being
/// ignored.
// #define FT6X06_P1_XH_REG            0x03U
// #define FT6X06_P1_XL_REG            0x04U
// #define FT6X06_P1_YH_REG            0x05U
// #define FT6X06_P1_YL_REG            0X06U
// #define FT6X06_P1_WEIGHT_REG        0x07U
// #define FT6X06_P1_MISC_REG          0x08U
//   followed by:
// #define FT6X06_P2_XH_REG            0x09U
// etc
#[derive(Copy, Clone, Debug)]
pub struct TouchState {
    /// Was a touch detected:
    pub detected: bool,
    /// X postion
    pub x: u16,
    /// Y position
    pub y: u16,
    /// Weight of touch
    pub weight: u8,
    /// Misc (contents not known)
    pub misc: u8,
}

/// When a gesture is polled it could be one of these:
pub enum GestureKind {
    /// No gesture detected
    None,
    /// Up gesture
    Up,
    /// Right gesture
    Right,
    /// Down gesture
    Down,
    /// Left gesture
    Left,
    /// ZoomIn gesture
    ZoomIn,
    /// ZoomOut gesture
    ZoomOut,
    /// Fault gesture
    Fault,
}

/// Structure that holds the values for a gesture
/// The name is what's in the c code.
/// The register definitions are:
/// pub const FT6X06_RADIAN_VALUE_REG: u8 = 0x91;
/// pub const FT6X06_OFFSET_LR_REG: u8 = 0x92;
/// pub const FT6X06_OFFSET_UD_REG: u8 = 0x93;
/// pub const FT6X06_DISTANCE_LR_REG: u8 = 0x94;
/// pub const FT6X06_DISTANCE_UD_REG: u8 = 0x95;
/// pub const FT6X06_DISTANCE_ZOOM_REG: u8 = 0x96;
pub struct GestureInit<I2C> {
    addr: u8,
    i2c: PhantomData<I2C>,

    /// radians required to sense a circle (probably not used)
    pub radian: u8,
    /// Offset distance left right
    pub offset_left_right: u8,
    /// Offset distance up down
    pub offset_up_down: u8,
    /// Distance for swipes left right
    pub distance_left_right: u8,
    /// Distance for swipes up down
    pub distance_up_down: u8,
    /// Distance for zoom
    pub distance_zoom: u8,
}

/// I wasn't able to get gestures to work. I suspect something is required in
/// the control register, but I don't know what. Also, this STM page (for nominally the same device):
/// <https://github.com/ryankurte/stm32-base/blob/master/drivers/BSP/Components/ft6X06/ft6X06.c>
/// has a different set of gestures available to the list above.
impl<'b, I2C, E> GestureInit<I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E> + i2c::Read<Error = E>,
{
    /// Initialise. Takes the I2C address just to avoid transferring it all the time.
    /// It turns out the gesture init registers are contiguous, see comment above
    /// or definitions of FT6X06_RADIAN_VALUE_REG and what follow, so they're also
    /// in the initialiser.
    ///
    /// This code didn't work in the STM32F7 Discovery - It wouldn't read parameters set.
    pub fn new(addr: u8) -> GestureInit<I2C> {
        GestureInit {
            i2c: PhantomData,
            addr,
            radian: 0,
            offset_left_right: 0,
            offset_up_down: 0,
            distance_left_right: 0,
            distance_up_down: 0,
            distance_zoom: 0,
        }
    }

    /// Fill the gesture struct with the values held for it on the
    /// touchscreen
    pub fn read(&mut self, i2c: &mut I2C) -> Result<&str, &str> {
        let mut buf: [u8; 6] = [4; 6];
        let result = i2c.write_read(self.addr, &[FT6X06_RADIAN_VALUE_REG], &mut buf);

        match result {
            Err(_e) => Err("Error reading gesture init registers"),
            Ok(_d) => {
                self.radian = buf[0];
                self.offset_left_right = buf[1];
                self.offset_up_down = buf[2];
                self.distance_left_right = buf[3];
                self.distance_up_down = buf[4];
                self.distance_zoom = buf[5];
                Ok("Success reading gesture init")
            }
        }
    }

    /// Write the six parameters of the gesture_init type into the FT5663
    pub fn write(
        &mut self,
        i2c: &mut I2C,
        radian: u8,
        offset_lr: u8,
        offset_ud: u8,
        dist_lr: u8,
        dist_up: u8,
        zoom: u8,
    ) -> Result<&str, &str> {
        let mut entries: [u8; 6] = [radian, offset_lr, offset_ud, dist_lr, dist_up, zoom];

        let result = i2c.write_read(self.addr, &mut [FT6X06_RADIAN_VALUE_REG], &mut entries);
        if let Err(_g) = result {
            Err("Error setting address in GestureInit")
        } else {
            // let result = i2c.write(self.addr, &mut entries);
            // match result {
            // Err(_e) => Err("Error writing GestureInit"),
            Ok("Okay writing GestureInit")
            // }
        }
    }
}

/// FT5883 driver
pub struct Ft6X06<'a, I2C> {
    i2c: PhantomData<I2C>,
    addr: u8,
    delay: &'a mut dyn DelayMs<u32>,
}

impl<'a, I2C, E> Ft6X06<'a, I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Creates a new sensor associated with an I2C peripheral.
    ///
    /// Phantom I2C ensures that whatever I2C bus the device was created on is the one that is used for all future interations.
    pub fn new(_i2c: &I2C, addr: u8, delay_source: &'a mut impl DelayMs<u32>) -> Result<Self, E> {
        let ft6x06 = Ft6X06 {
            i2c: PhantomData,
            addr: addr,
            delay: delay_source,
        };
        Ok(ft6x06)
    }

    /// Initialise device and disable interupt mode.
    /// FT6X06 should be calibrated once after each power up.
    pub fn init(&mut self, i2c: &mut I2C) {
        // -> Result<Self, E> {
        if FT6X06_AUTO_CALIBRATION_ENABLED {
            self.ts_calibration(i2c).unwrap();
        }
        // FT6X06_DisableIT(i2c)?;
        // Ok(*self)
    }

    //pub fn DisableIT(&self, i2c: &mut I2C) -> Result<u8, E> {}
    /// Future test code
    pub fn test(&self, _i2c: &mut I2C) {}

    ///As the ft6X06 library owns the delay, the simplest way to
    /// deliver it to the callign code seems to be to return a function call.
    pub fn delay_ms(&mut self, delay: u32) {
        self.delay.delay_ms(delay);
    }

    /// Returns the structure that contains all the preset capabilities
    /// of the FT6X06
    pub fn get_capabilities(&self) -> Ft6x06Capabilities {
        FT6X06_CAPABILITIES
    }

    /// Read whether the FT5663 is in dev mode or not
    pub fn dev_mode_r(&self, i2c: &mut I2C) -> Result<u8, E> {
        let mut buf: [u8; 1] = [0];

        i2c.write_read(self.addr, &[FT6X06_DEV_MODE_REG], &mut buf)?;

        let mut value = buf[0];
        value &= FT6X06_DEV_MODE_BIT_MASK;
        value &= FT6X06_DEV_MODE_BIT_POSITION;

        Ok(value)
    }

    /// Put the FT5663 into dev mode
    pub fn dev_mode_w(&self, i2c: &mut I2C, value: u8) -> Result<bool, E> {
        let mut buf: [u8; 1] = [0];

        i2c.write_read(self.addr, &[FT6X06_DEV_MODE_REG], &mut buf)?;

        let mut tmp = buf[0];

        tmp &= !FT6X06_DEV_MODE_BIT_MASK;
        tmp |= value << FT6X06_DEV_MODE_BIT_POSITION;

        i2c.write(self.addr, &[tmp])?;

        Ok(value == 0)
    }

    /// Run an internal calibration on the FT6X06
    pub fn ts_calibration(&mut self, i2c: &mut I2C) -> Result<bool, &str> {
        //} -> Result<Self, E> {
        let mut _ret = FT6X06_OK;
        let mut _nbr_attempt: u32;
        let mut _read_data: u8;
        let mut _end_calibration: u8;

        let _result = self.dev_mode_w(i2c, FT6X06_DEV_MODE_FACTORY);

        self.delay.delay_ms(300);

        for _attempt in 0..100 {
            match self.dev_mode_r(i2c) {
                Err(_e) => return Err("Bad comms in ts_calibration"),
                Ok(n) => {
                    if n == FT6X06_DEV_MODE_WORKING {
                        return Ok(true);
                    }
                }
            }
            self.delay.delay_ms(200);
        }
        Err("Calibration does not return")
    }

    /// Read the touch device status
    pub fn td_status(&self, i2c: &mut I2C) -> Result<u8, E> {
        let mut buf: [u8; 1] = [0];
        i2c.write_read(self.addr, &[FT6X06_TD_STAT_REG], &mut buf)?;
        Ok(buf[0])
    }

    /// Read the touch device chip ID. It should be 0x51 if it is the FT6X06 on the
    /// stm32f746 Discovery board
    pub fn chip_id(&self, i2c: &mut I2C) -> Result<u8, &str> {
        let mut buf: [u8; 1] = [0];
        match i2c.write_read(self.addr, &[FT6X06_CHIP_ID_REG], &mut buf) {
            Err(_e) => Err("Chip ID call failed"),
            Ok(_a) => {
                if buf[0] != FT6X06_ID {
                    Err("error in chip ID")
                } else {
                    Ok(buf[0])
                }
            }
        }
    }

    /// Is the device being touched? If so, how many fingers?
    pub fn detect_touch(&mut self, i2c: &mut I2C) -> Result<u8, &str> {
        match self.td_status(i2c) {
            Err(_e) => Err("Error getting touch data"),
            Ok(n) => {
                if n < FT6X06_MAX_NB_TOUCH {
                    Ok(n)
                } else {
                    Ok(0)
                }
            }
        }
    }

    /// Retrieve the FT6X06 firmware id
    pub fn firmware_id(&mut self, i2c: &mut I2C) -> Result<u8, &str> {
        let mut buf: [u8; 1] = [0];
        match i2c.write_read(self.addr, &[FT6X06_FIRMID_REG], &mut buf) {
            Err(_e) => Err("Error getting firmware ID"),
            Ok(_d) => Ok(buf[0]),
        }
    }

    /// Retrieve the Gesture Init variable
    pub fn gesture_radian_read(&mut self, i2c: &mut I2C) -> Result<u8, &str> {
        let mut buf: [u8; 1] = [0];
        match i2c.write_read(self.addr, &[FT6X06_RADIAN_VALUE_REG], &mut buf) {
            Err(_e) => Err("Error getting Gesture Init: RADIAN VALUE REG"),
            Ok(_d) => Ok(buf[0]),
        }
    }

    /// Write the Gesture Init variable
    pub fn gesture_radian_write(&self, i2c: &mut I2C, value: u8) -> Result<bool, E> {
        let mut buf: [u8; 1] = [value];

        i2c.write_read(self.addr, &[FT6X06_RADIAN_VALUE_REG], &mut buf)?;

        Ok(value == 0)
    }

    /// Fetch the touch data specified by touch_i
    /// touch_i should go from 1 to FT6X06_MAX_NB_TOUCH
    pub fn get_touch(&mut self, i2c: &mut I2C, touch_i: u8) -> Result<TouchState, E> {
        let mut buf: [u8; 6] = [0; 6];
        i2c.write_read(self.addr, &[FT6X06_P1_XH_REG + 6 * (touch_i - 1)], &mut buf)?;

        // Tried copying the c code literally here. It makes no difference though
        let x: u16 = (FT6X06_P1_XH_TP_BIT_MASK & buf[0]) as u16 * 256 + buf[1] as u16;
        let y: u16 = (FT6X06_P1_YH_TP_BIT_MASK & buf[2]) as u16 * 256 + buf[3] as u16;

        Ok(TouchState {
            detected: true,
            x,
            y,
            weight: buf[4],
            misc: buf[5],
        })
    }

    /// Get gestures interpreted by touchscreen
    pub fn get_gesture(&mut self, i2c: &mut I2C) -> Result<GestureKind, E> {
        let mut buf: [u8; 1] = [0];
        i2c.write_read(self.addr, &[FT6X06_GEST_ID_REG], &mut buf)?;

        let g: GestureKind = match buf[0] {
            FT6X06_GEST_ID_NO_GESTURE => GestureKind::None,
            FT6X06_GEST_ID_MOVE_UP => GestureKind::Up,
            FT6X06_GEST_ID_MOVE_RIGHT => GestureKind::Right,
            FT6X06_GEST_ID_MOVE_DOWN => GestureKind::Down,
            FT6X06_GEST_ID_MOVE_LEFT => GestureKind::Left,
            FT6X06_GEST_ID_ZOOM_IN => GestureKind::ZoomIn,
            FT6X06_GEST_ID_ZOOM_OUT => GestureKind::ZoomOut,
            _ => GestureKind::Fault,
        };
        Ok(g)
    }
}

