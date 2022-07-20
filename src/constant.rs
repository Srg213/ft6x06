

/// Maximum border values of the touchscreen pad///
pub const  FT_6X06_MAX_WIDTH              :u16 = 800;   /// Touchscreen pad max width  ///
pub const  FT_6X06_MAX_HEIGHT             :u16 = 480;   /// Touchscreen pad max height ///

/// Touchscreen pad max width and height values for FT6x36 Touch
pub const  FT_6X06_MAX_WIDTH_HEIGHT       :u8 = 240;     

/// Possible values of driver functions return status///
pub const FT6X06_STATUS_OK                :u8 = 0;
pub const FT6X06_STATUS_NOT_OK            :u8 = 1;

/// Possible values of global variable 'TS_I2C_Initialized'///
pub const FT6X06_I2C_NOT_INITIALIZED      :u8 = 0;
pub const FT6X06_I2C_INITIALIZED          :u8 = 1;

/// Max detectable simultaneous touches///
pub const FT6X06_MAX_DETECTABLE_TOUCH     :u8 = 2;

/////
  /// @brief : Definitions for FT6X06 I2C register addresses on 8 bit
  //////

/// Current mode register of the FT6X06 (R/W)///
pub const FT6X06_DEV_MODE_REG             : u8 = 0x00;

/// Possible values of FT6X06_DEV_MODE_REG///
pub const FT6X06_DEV_MODE_WORKING         : u8 = 0x00;
pub const FT6X06_DEV_MODE_FACTORY         : u8 = 0x04;

pub const FT6X06_DEV_MODE_MASK            : u8 = 0x7;
pub const FT6X06_DEV_MODE_SHIFT           :u8 = 4;

/// Gesture ID register///
pub const FT6X06_GEST_ID_REG              : u8 = 0x01;

/// Possible values of FT6X06_GEST_ID_REG///
pub const FT6X06_GEST_ID_NO_GESTURE       : u8 = 0x00;
pub const FT6X06_GEST_ID_MOVE_UP          : u8 = 0x10;
pub const FT6X06_GEST_ID_MOVE_RIGHT       : u8 = 0x14;
pub const FT6X06_GEST_ID_MOVE_DOWN        : u8 = 0x18;
pub const FT6X06_GEST_ID_MOVE_LEFT        : u8 = 0x1C;
pub const FT6X06_GEST_ID_ZOOM_IN          : u8 = 0x48;
pub const FT6X06_GEST_ID_ZOOM_OUT         : u8 = 0x49;

/// Touch Data Status register : gives number of active touch points (0..2)///
pub const FT6X06_TD_STAT_REG              : u8 = 0x02;

/// Values related to FT6X06_TD_STAT_REG///
pub const FT6X06_TD_STAT_MASK             : u8 = 0x0F;
pub const FT6X06_TD_STAT_SHIFT            : u8 = 0x00;

/// Values Pn_XH and Pn_YH related///
pub const FT6X06_TOUCH_EVT_FLAG_PRESS_DOWN : u8 = 0x00;
pub const FT6X06_TOUCH_EVT_FLAG_LIFT_UP    : u8 = 0x01;
pub const FT6X06_TOUCH_EVT_FLAG_CONTACT    : u8 = 0x02;
pub const FT6X06_TOUCH_EVT_FLAG_NO_EVENT   : u8 = 0x03;

pub const FT6X06_TOUCH_EVT_FLAG_SHIFT      :u8 = 6;
pub const FT6X06_TOUCH_EVT_FLAG_MASK       :u8 = 3 << FT6X06_TOUCH_EVT_FLAG_SHIFT;

pub const FT6X06_MSB_MASK                 : u8 = 0x0F;
pub const FT6X06_MSB_SHIFT                :u8 = 0;

/// Values Pn_XL and Pn_YL related///
pub const FT6X06_LSB_MASK                 : u8 = 0xFF;
pub const FT6X06_LSB_SHIFT                :u8 = 0;

pub const FT6X06_P1_XH_REG                : u8 = 0x03;
pub const FT6X06_P1_XL_REG                : u8 = 0x04;
pub const FT6X06_P1_YH_REG                : u8 = 0x05;
pub const FT6X06_P1_YL_REG                : u8 = 0x06;

/// Touch Pressure register value (R)///
pub const FT6X06_P1_WEIGHT_REG            : u8 = 0x07;

/// Values Pn_WEIGHT related ///
pub const FT6X06_TOUCH_WEIGHT_MASK        : u8 = 0xFF;
pub const FT6X06_TOUCH_WEIGHT_SHIFT       : u8 = 0;

/// Touch area register///
pub const FT6X06_P1_MISC_REG              : u8 = 0x08;

/// Values related to FT6X06_Pn_MISC_REG///
pub const FT6X06_TOUCH_AREA_MASK         : u8 = 0x04 << 4;
pub const FT6X06_TOUCH_AREA_SHIFT        : u8 = 0x04;

pub const FT6X06_P2_XH_REG               : u8 = 0x09;
pub const FT6X06_P2_XL_REG               : u8 = 0x0A;
pub const FT6X06_P2_YH_REG               : u8 = 0x0B;
pub const FT6X06_P2_YL_REG               : u8 = 0x0C;
pub const FT6X06_P2_WEIGHT_REG           : u8 = 0x0D;
pub const FT6X06_P2_MISC_REG             : u8 = 0x0E;

/// Threshold for touch detection///
pub const FT6X06_TH_GROUP_REG            : u8 = 0x80;

/// Values FT6X06_TH_GROUP_REG : threshold related ///
pub const FT6X06_THRESHOLD_MASK          : u8 = 0xFF;
pub const FT6X06_THRESHOLD_SHIFT         :u8 = 0;

/// Filter function coefficients///
pub const FT6X06_TH_DIFF_REG             : u8 = 0x85;

/// Control register///
pub const FT6X06_CTRL_REG                : u8 = 0x86;

/// Values related to FT6X06_CTRL_REG///

/// Will keep the Active mode when there is no touching///
pub const FT6X06_CTRL_KEEP_ACTIVE_MODE    : u8 = 0x00;

/// Switching from Active mode to Monitor mode automatically when there is no touching///
pub const FT6X06_CTRL_KEEP_AUTO_SWITCH_MONITOR_MODE  : u8 = 0x01;

/// The time period of switching from Active mode to Monitor mode when there is no touching///
pub const FT6X06_TIMEENTERMONITOR_REG     : u8 = 0x87;

/// Report rate in Active mode///
pub const FT6X06_PERIODACTIVE_REG         : u8 = 0x88;

/// Report rate in Monitor mode///
pub const FT6X06_PERIODMONITOR_REG        : u8 = 0x89;

/// The value of the minimum allowed angle while Rotating gesture mode///
pub const FT6X06_RADIAN_VALUE_REG         : u8 = 0x91;

/// Maximum offset while Moving Left and Moving Right gesture///
pub const FT6X06_OFFSET_LEFT_RIGHT_REG    : u8 = 0x92;

/// Maximum offset while Moving Up and Moving Down gesture///
pub const FT6X06_OFFSET_UP_DOWN_REG       : u8 = 0x93;

/// Minimum distance while Moving Left and Moving Right gesture///
pub const FT6X06_DISTANCE_LEFT_RIGHT_REG  : u8 = 0x94;

/// Minimum distance while Moving Up and Moving Down gesture///
pub const FT6X06_DISTANCE_UP_DOWN_REG     : u8 = 0x95;

/// Maximum distance while Zoom In and Zoom Out gesture///
pub const FT6X06_DISTANCE_ZOOM_REG        : u8 = 0x96;

/// High 8-bit of LIB Version info///
pub const FT6X06_LIB_VER_H_REG            : u8 = 0xA1;

/// Low 8-bit of LIB Version info///
pub const FT6X06_LIB_VER_L_REG            : u8 = 0xA2;

/// Chip Selecting///
pub const FT6X06_CIPHER_REG               : u8 = 0xA3;

/// Interrupt mode register (used when in interrupt mode)///
pub const FT6X06_GMODE_REG                : u8 = 0xA4;

pub const FT6X06_G_MODE_INTERRUPT_MASK    : u8 = 0x03;
pub const FT6X06_G_MODE_INTERRUPT_SHIFT   : u8 = 0x00;

/// Possible values of FT6X06_GMODE_REG///
pub const FT6X06_G_MODE_INTERRUPT_POLLING : u8 = 0x00;
pub const FT6X06_G_MODE_INTERRUPT_TRIGGER : u8 = 0x01;

/// Current power mode the FT6X06 system is in (R)///
pub const FT6X06_PWR_MODE_REG             : u8 = 0xA5;

/// FT6X06 firmware version///
pub const FT6X06_FIRMID_REG               : u8 = 0xA6;

/// FT6X06 Chip identification register///
pub const FT6X06_CHIP_ID_REG              : u8 = 0xA8;

///  Possible values of FT6X06_CHIP_ID_REG///
pub const FT6X06_ID_VALUE                 : u8 = 0x11;
pub const FT6X36_ID_VALUE                 : u8 = 0xCD;

/// Release code version///
pub const FT6X06_RELEASE_CODE_ID_REG      : u8 = 0xAF;

/// Current operating mode the FT6X06 system is in (R)///
pub const FT6X06_STATE_REG                : u8 = 0xBC;

pub const FT6X06_OK                      : u8 = 0;
pub const FT6X06_ERROR                   : i8 = -1;

/// Max detectable simultaneous touches 
pub const FT6X06_MAX_NB_TOUCH            : u8 =  2;
  
/// Touch FT6XX6 IDs 
pub const FT6X06_ID                      : u8 = 0x11;
pub const FT6X36_ID                      : u8 = 0xCD;


pub const FT6X06_MAX_X_LENGTH: u16 = 800_u16;
pub const FT6X06_MAX_Y_LENGTH: u16 = 480_u16;
pub const FT6X06_P1_XH_TP_BIT_MASK: u8 = 0x0F;
pub const FT6X06_P1_YH_TP_BIT_MASK: u8 = 0x0F;
pub const FT6X06_DEV_MODE_BIT_MASK: u8 = 0x70;
pub const FT6X06_DEV_MODE_BIT_POSITION: u8 = 4;
pub const FT6X06_AUTO_CALIBRATION_ENABLED: bool = false;
