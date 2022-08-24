//! 
//! A platform agnostic driver for FT6X06 touchscreen .
//! 
//! Built using 'embedded-hal' traits.
//! 
//! The Touchscreen driver for FT6X06
//! 
//! 
//!
//!
//!  ##Example 
//!		
//!  ### Initializing the Ft6x06 driver struct
//! let mut touch = ft6x06::Ft6X06::new(&i2c, 0x38, &mut delay).unwrap();
//! 
//! 


#![no_std]
#![no_main]

pub mod constant;
#[cfg(feature = "gesture")]
pub mod gesture;

#[cfg(feature = "gesture")]
use heapless::Vec;

[#allow(unused_imports)]
use panic_semihosting as _;

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
    max_touch: FT6X06_MAX_NB_TOUCH as u8,
    max_x_length: FT6X06_MAX_X_LENGTH,
    may_y_length: FT6X06_MAX_Y_LENGTH,
};



/// Touch structure - derived from the available I2C registers
/// There are ten available touch registers on the chip, but also
/// a defined maximum of 2 in FT6X06_MAX_NB_TOUCH above.
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
#[derive(Copy, Clone, Debug, PartialOrd, Ord, Eq, PartialEq)]
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



/// For storing multi-touch data
pub struct MultiTouch{
	pub detected: bool,
    /// X postion
    pub touch_x: [u16; 2],
    /// Y position
    pub touch_y: [u16; 2],
    /// Weight of touch
    pub touch_weight: [u16; 2],
    /// Misc (contents not known)
	pub touch_area: [u16; 2]
}


#[derive(Debug)]
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
		
		rprintln!(
		 "Gesture read buf values: {}  {} {} {} {} {} ",
		         buf[0],
		         buf[1],
		     	 buf[2],
		     	 buf[3],
		     	 buf[4],
		     	 buf[5],
	 );
		
		
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

    /// Write the six parameters of the gesture_init type into the FT6206
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

        rprintln!(
		 "Gesture write entries values before: {}  {} {} {} {} {} ",
		         entries[0],
		         entries[1],
		     	 entries[2],
		     	 entries[3],
		     	 entries[4],
		     	 entries[5],
	 );
		
        
        let result = i2c.write_read(self.addr, &mut [FT6X06_RADIAN_VALUE_REG], &mut entries);
        
        rprintln!(
		 "Gesture write entries values: {}  {} {} {} {} {} ",
		         entries[0],
		         entries[1],
		     	 entries[2],
		     	 entries[3],
		     	 entries[4],
		     	 entries[5],
	 );
		
        
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

/// FT6x06 driver
pub struct Ft6X06<I2C> {
    i2c: PhantomData<I2C>,
    addr: u8,
}

impl<I2C, E> Ft6X06<I2C>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Creates a new sensor associated with an I2C peripheral.
    ///
    /// Phantom I2C ensures that whatever I2C bus the device was created on is the one that is used for all future interations.
    pub fn new(_i2c: &I2C, addr: u8) -> Result<Self, E> {
        let ft6x06 = Ft6X06 {
            i2c: PhantomData,
            addr: addr,
        };
        Ok(ft6x06)
    }

    /// Initialise device and disable interupt mode.
    /// FT6X06 should be calibrated once after each power up.
    pub fn init(&mut self, i2c: &mut I2C, delay_source: &mut impl DelayMs<u32>) {
        // -> Result<Self, E> {
        if FT6X06_AUTO_CALIBRATION_ENABLED {
            self.ts_calibration(i2c, delay_source).unwrap();
        }
        // FT6X06_DisableIT(i2c)?;
        // Ok(*self)
    }

    //pub fn DisableIT(&self, i2c: &mut I2C) -> Result<u8, E> {}
    /// Future test code
    pub fn test(&self, _i2c: &mut I2C) {}

    ///As the ft6X06 library owns the delay, the simplest way to
    /// deliver it to the callign code seems to be to return a function call.
    pub fn delay_ms(&mut self, delay_source: &mut impl DelayMs<u32>, delay: u32) {
        delay_source.delay_ms(delay);
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
    pub fn ts_calibration(&mut self, i2c: &mut I2C, delay_source: &mut impl DelayMs<u32>) -> Result<bool, &str> {
        //} -> Result<Self, E> {
        let mut _ret = FT6X06_OK;
        let mut _nbr_attempt: u32;
        let mut _read_data: u8;
        let mut _end_calibration: u8;

        let _result = self.dev_mode_w(i2c, FT6X06_DEV_MODE_FACTORY);

        delay_source.delay_ms(300);

        for _attempt in 0..100 {
            match self.dev_mode_r(i2c) {
                Err(_e) => return Err("Bad comms in ts_calibration"),
                Ok(n) => {
                    if n == FT6X06_DEV_MODE_WORKING {
                        return Ok(true);
                    }
                }
            }
            delay_source.delay_ms(200);
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
                if n <= FT6X06_MAX_NB_TOUCH as u8 {
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
    
    /// Fetch the touch data specified by touch_i
    /// touch_i should go from 1 to FT6X06_MAX_NB_TOUCH
    pub fn get_multi_touch(&mut self, i2c: &mut I2C, touch_i: u8) -> Result<MultiTouch, E> {
        let mut buf: [u8; 12] = [0; 12];
        i2c.write_read(self.addr, &[FT6X06_P1_XH_REG + 6 * (touch_i - 1)], &mut buf)?;
		
        let mut x: [u16; FT6X06_MAX_NB_TOUCH] = [0; FT6X06_MAX_NB_TOUCH];
        let mut y: [u16; FT6X06_MAX_NB_TOUCH] = [0; FT6X06_MAX_NB_TOUCH];
        let mut weight: [u16; FT6X06_MAX_NB_TOUCH] = [0; FT6X06_MAX_NB_TOUCH];
        let mut misc: [u16; FT6X06_MAX_NB_TOUCH] = [0; FT6X06_MAX_NB_TOUCH];
        
        let mut it: usize = 0;
        for i in 0..FT6X06_MAX_NB_TOUCH{
        	 x[i] = (FT6X06_P1_XH_TP_BIT_MASK & buf[0 + it]) as u16 * 256 + buf[1 + it] as u16;
        	 y[i] = (FT6X06_P1_YH_TP_BIT_MASK & buf[2 + it]) as u16 * 256 + buf[3 + it] as u16;
        	 weight[i] = buf[4 + it] as u16;
        	 misc[i] = buf[5 + it] as u16;
        	 it = it + 6;
        }

        Ok(MultiTouch {
            detected: true,
            touch_x: x,
            touch_y: y,
            touch_weight: weight,
            touch_area: misc,
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


    pub fn get_coordinates(&mut self, i2c: &mut I2C) -> Result<(u16,u16), &str>
	{
		    let mut t = self.detect_touch(i2c);
			while t.unwrap() == 0 || t == Err("Error getting touch data") {
				t = self.detect_touch(i2c);
			}
			
		    let mut num: u8 = 0;
		    match t {
		        Err(_e) => return Err("Error {} from fetching number of touches"),
		        Ok(n) => {
		            num = n;
		            if num != 0 {
		                rprintln!("Number of touches in get_coordinates: {}", num);
		            };
		        	if num > 0 {
						let t = self.get_touch(i2c, 1);
						return match t {
						    Err(_e) => Err("Error fetching touch data"),
						    Ok(n) => Ok((n.x,n.y)),
							}
						}
					else{
						 return Err("no");
		      	}       	
			}
		}
	}

	#[cfg(feature="gesture")]
	pub fn gest_logic(&mut self, i2c:&mut I2C, sec: u16) -> Result<GestureKind,&str>{
		
		let mut vec1: Vec<u16, 100> = Vec::new();
		let mut vec2: Vec<u16, 100> = Vec::new();
		
		for _i in 1..20{
			let a = self.get_coordinates(i2c);
			
			match a{
				Err(_e) => {
					rprintln!("err");
					continue;
				}
				Ok((x,y)) => {
					vec1.push(x).expect("errrrrr");
					vec2.push(y).expect("errrrr");
				},
			};			
		}
			let itr1 = vec1.iter();
			let itr2 = vec2.iter();
			
			let maxX: u16 = *itr1.max().expect("errrrrr");
			let maxY: u16 = *itr2.max().expect("errrrrr");
			 
			let itr1 = vec1.iter();
			let itr2 = vec2.iter();
			 
			let minX: u16 = *itr1.min().expect("errrrrr");
			let minY: u16 = *itr2.min().expect("errrrrr");
			
			let startX: u16 = vec1[0];
			let startY: u16 = vec2[0];
			
			let endX: u16 = vec1[19];
			let endY: u16 = vec2[19];
			
			let diffX = endX-startX;
			let diffY = endY-startY;
			
			if diffX > 100 || diffY > 100 {
				return Err("wrong gestures.")
			}
			else if diffX > diffY{
				if diffX > 0  { return Ok(GestureKind::Right)}
				else   { return Ok(GestureKind::Left)}
			}
			else if diffX < diffY{				
				if diffY > 0  { return Ok(GestureKind::Up)}
				else  { return Ok(GestureKind::Left)}
			}
			else{
				return Err("error gesture")
			}
		}
}
