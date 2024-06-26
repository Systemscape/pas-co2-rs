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
#[derive(Clone, Copy)]
pub enum PwmMode {
    SinglePulse = 0,
    PulseTrain = 1,
}

#[derive(Clone, Copy)]
pub enum BaselineOffsetCompensation {
    Disabled = 0b00,
    Enabled = 0b01,
    Forced = 0b10,
}

#[derive(Clone, Copy)]
pub enum OperatingMode {
    Idle = 0b00,
    SingleShot = 0b01,
    Continuous = 0b10,
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
