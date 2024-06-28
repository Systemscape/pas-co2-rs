pub trait Reg {
    fn address() -> u8;
}

mod status;
pub use status::Status;

mod measurement_mode;
pub use measurement_mode::*;

mod measurement_status;
pub use measurement_status::*;

mod interrupt_config;
pub use interrupt_config::*;

pub struct MeasurementPeriod;
impl Reg for MeasurementPeriod {
    fn address() -> u8 {
        0x02
    }
}

pub struct Co2Ppm;
impl Reg for Co2Ppm {
    fn address() -> u8 {
        0x05
    }
}

pub struct AlarmThreshold(pub i16);
impl Reg for AlarmThreshold {
    fn address() -> u8 {
        0x09
    }
}
impl Default for AlarmThreshold {
    fn default() -> Self {
        Self(0)
    }
}

pub struct PressureCompensation(pub u16);
impl Reg for PressureCompensation {
    fn address() -> u8 {
        0x0B
    }
}
impl Default for PressureCompensation {
    fn default() -> Self {
        Self(0x03F7) // high byte 0x03, low byte 0xF7 according to register map
    }
}

/// Automatic Baseline Offset Compensation
pub struct ABOC(pub i16);
impl Reg for ABOC {
    fn address() -> u8 {
        0x0D
    }
}
impl Default for ABOC {
    fn default() -> Self {
        Self(0x0190) // high byte 0x01, low byte 0x90 according to register map
    }
}

pub struct Scratchpad;
impl Reg for Scratchpad {
    fn address() -> u8 {
        0x0F
    }
}

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

impl Reg for SoftReset {
    fn address() -> u8 {
        0x10
    }
}
