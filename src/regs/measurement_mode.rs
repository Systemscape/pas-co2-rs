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
        let bitmask = (value.pwm_out_enable as u8) << 5
            | (value.pwm_mode as u8) << 4
            | (value.baseline_offset_comp as u8) << 2
            | value.operating_mode as u8;

        bitmask
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

impl super::Reg for MeasurementMode {
    fn address() -> u8 {
        0x04
    }
}

#[cfg(test)]
#[test]
fn test_bitmask() {
    let mode = MeasurementMode {
        pwm_out_enable: true,                                     // 0b1
        pwm_mode: PwmMode::SinglePulse,                           //0b0
        baseline_offset_comp: BaselineOffsetCompensation::Forced, //0b10
        operating_mode: OperatingMode::Continuous,                // 0b10
    };

    let bitmask: u8 = mode.into();

    assert_eq!(bitmask, 0b0010_1010)
}
