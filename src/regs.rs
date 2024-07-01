use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(IntoPrimitive)]
#[repr(u8)]
pub enum Register {
    ProdId = 0x0,
    SensorStatus = 0x01,
    MeasurementRate = 0x02,
    MeasurementMode = 0x04,
    Co2Ppm = 0x05,
    MeasurementStatus = 0x07,
    InterruptConfig = 0x08,
    AlarmThreshold = 0x09,
    PressureReference = 0x0B,
    CalibrationReference = 0x0D,
    ScratchPad = 0x0F,
    SensorReset = 0x10,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Status {
    /// Sensor ready bit
    pub ready: bool,
    /// PWM_DIS pin status
    pub pwm_dis: bool,
    /// Out-of-range temperature error bit
    pub temperature_error: bool,
    /// Out-of-range VDD12V/5V error bit
    pub voltage_error: bool,
    /// Communication error notification bit
    pub communication_error: bool,
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        Self {
            ready: (value & 0b1000_0000) != 0,
            pwm_dis: (value & 0b0100_0000) != 0,
            temperature_error: (value & 0b0010_0000) != 0,
            voltage_error: (value & 0b0001_0000) != 0,
            communication_error: (value & 0b0000_1000) != 0,
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MeasurementStatus {
    /// New data available in CO2PPM Register
    pub data_ready: bool,
    /// Pin INT has been latched to active state
    pub int_active: bool,
    /// Alarm notification (threshold violation occured)
    pub alarm: bool,
}

impl From<u8> for MeasurementStatus {
    fn from(value: u8) -> Self {
        Self {
            data_ready: (value & 0b0001_0000) != 0,
            int_active: (value & 0b0000_1000) != 0,
            alarm: (value & 0b0000_0100) != 0,
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy)]
pub struct MeasurementMode {
    /// PWM output software enable bit
    pub pwm_out_enable: bool,
    /// PWM mode configuration
    pub pwm_mode: PwmMode,
    /// Baseline offset compensation config
    pub baseline_offset_comp: BaselineOffsetCompensation,
    /// Sensor operating mode
    pub operating_mode: OperatingMode,
}
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy)]
pub enum PwmMode {
    SinglePulse = 0,
    PulseTrain = 1,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy)]
pub enum BaselineOffsetCompensation {
    Disabled = 0b00,
    Enabled = 0b01,
    Forced = 0b10,
    _Reserved = 0b11,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy)]
pub enum OperatingMode {
    Idle = 0b00,
    SingleShot = 0b01,
    Continuous = 0b10,
    _Reserved = 0b11,
}

impl Default for MeasurementMode {
    fn default() -> Self {
        Self::from(0x24)
    }
}

impl From<MeasurementMode> for u8 {
    fn from(value: MeasurementMode) -> Self {
        (value.pwm_out_enable as u8) << 5
            | (value.pwm_mode as u8) << 4
            | (value.baseline_offset_comp as u8) << 2
            | value.operating_mode as u8
    }
}

impl From<u8> for MeasurementMode {
    fn from(value: u8) -> Self {
        Self {
            pwm_out_enable: (value & 0b0010_0000) != 0,
            pwm_mode: match (value & 0b0001_0000) >> 4 {
                0 => PwmMode::SinglePulse,
                _ => PwmMode::PulseTrain,
            },
            baseline_offset_comp: match (value & 0b0000_1100) >> 2 {
                0b00 => BaselineOffsetCompensation::Disabled,
                0b01 => BaselineOffsetCompensation::Enabled,
                0b10 => BaselineOffsetCompensation::Forced,
                _ => BaselineOffsetCompensation::_Reserved,
            },
            operating_mode: match value & 0b0000_0011 {
                0b00 => OperatingMode::Idle,
                0b01 => OperatingMode::SingleShot,
                0b10 => OperatingMode::Continuous,
                _ => OperatingMode::_Reserved,
            },
        }
    }
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct InterruptConfig {
    /// Pin INT electrical config: false = active low, true = active high
    pub int_pin_active_high: bool,
    /// Pin INT function config
    pub int_function_config: IntFunctionConfig,
    /// Alarm type: false = crossing down, true = crossing up
    pub alarm_crossing_up: bool,
}

#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum IntFunctionConfig {
    /// Pin INT is inactive
    Inactive = 0x0,
    /// Alarm threshold violation notification
    Alarm = 0x1,
    /// Data ready notification
    DataReady = 0x2,
    /// Busy notification
    Busy = 0x3,
    /// Early measurement start notification (only continuous mode)
    EarlyMeasurementStart = 0x4,
}

impl Default for InterruptConfig {
    fn default() -> Self {
        Self {
            int_pin_active_high: true,
            int_function_config: IntFunctionConfig::Inactive,
            alarm_crossing_up: true,
        }
    }
}

impl From<InterruptConfig> for u8 {
    fn from(value: InterruptConfig) -> Self {
        (value.int_pin_active_high as u8) << 4
            | (value.int_function_config as u8) << 1
            | value.alarm_crossing_up as u8
    }
}

impl TryFrom<u8> for InterruptConfig {
    type Error = crate::ResponseError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self {
            int_pin_active_high: (value & 0b0001_0000) != 0,
            int_function_config: ((value & 0b0000_1110) >> 1)
                .try_into()
                .map_err(|_| Self::Error::InvalidRegisterValue)?,
            alarm_crossing_up: (value & 0b0000_0001) != 0,
        })
    }
}

#[derive(IntoPrimitive)]
#[repr(u8)]
/// Soft reset register
pub enum SoftReset {
    /// Trigger a soft reset event
    SoftReset = 0xA3,
    /// Reset the ABOC context
    AbocReset = 0xBC,
    /// Save the force calibration offset to internal NVM immediately
    SaveForceCalibNvm = 0xCF,
    /// Disable the stepwise reactive IIR filter
    DisableStepwiseReractiveIirFilter = 0xDF,
    /// Reset the forced calibration correction factor
    ResetForcedCalibCorrectionFactor = 0xFC,
    /// Enable the stepwise reactive IIR filter (default enabled)
    EnableStepwiseReaciveIirFilter = 0xFE,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_status_bitmask() {
        let status = Status {
            ready: true,
            pwm_dis: false,
            temperature_error: false,
            voltage_error: true,
            communication_error: true,
        };

        let bitmask: u8 = 0b1001_1000;

        assert_eq!(status, Status::from(bitmask))
    }

    #[test]
    fn test_measurement_status_bitmask() {
        let status = MeasurementStatus {
            data_ready: true,
            int_active: false,
            alarm: true,
        };

        let bitmask: u8 = 0b0001_0100;

        assert_eq!(status, MeasurementStatus::from(bitmask));

        // Check that not equal if alarm bit is flipped
        assert_ne!(status, MeasurementStatus::from(bitmask ^ 0b0000_0100));
    }

    #[test]
    fn test_measurement_mode_bitmask() {
        let mode = MeasurementMode {
            pwm_out_enable: true,                                     // 0b1
            pwm_mode: PwmMode::SinglePulse,                           //0b0
            baseline_offset_comp: BaselineOffsetCompensation::Forced, //0b10
            operating_mode: OperatingMode::Continuous,                // 0b10
        };

        let bitmask: u8 = mode.into();

        assert_eq!(bitmask, 0b0010_1010)
    }

    #[test]
    fn test_interrupt_config_bitmask() {
        let config = InterruptConfig {
            int_pin_active_high: true,
            int_function_config: IntFunctionConfig::DataReady,
            alarm_crossing_up: true,
        };

        let bitmask: u8 = config.into();

        assert_eq!(bitmask, 0b0001_0101);

        let config_from = InterruptConfig::try_from(bitmask).unwrap();
        assert_eq!(config, config_from);
    }
}
