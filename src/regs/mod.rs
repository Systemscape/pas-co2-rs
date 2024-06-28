/// Shared trait for all registers
pub trait Reg {
    /// Register address
    fn address() -> u8;
}

/// Status register representation
pub mod status;
pub use status::Status;

/// Measurement mode register representation
pub mod measurement_mode;
pub use measurement_mode::MeasurementMode;

/// Measurement status register representation
pub mod measurement_status;
pub use measurement_status::MeasurementStatus;

/// Interrupt configuration register representation
pub mod interrupt_config;
pub use interrupt_config::InterruptConfig;

/// Measurement period (or rate) register
pub struct MeasurementPeriod;
impl Reg for MeasurementPeriod {
    fn address() -> u8 {
        0x02
    }
}

/// CO2 PPM measurement register
pub struct Co2Ppm;
impl Reg for Co2Ppm {
    fn address() -> u8 {
        0x05
    }
}

/// Alarm threshold PPM register
pub struct AlarmThreshold(pub i16);
impl Reg for AlarmThreshold {
    fn address() -> u8 {
        0x09
    }
}
// Explicitly state it here that the threshold is zero
#[allow(clippy::derivable_impls)]
impl Default for AlarmThreshold {
    fn default() -> Self {
        Self(0x0)
    }
}

/// Pressure compensation register
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

/// Scratchpad register (free to write and read by the user)
pub struct Scratchpad;
impl Reg for Scratchpad {
    fn address() -> u8 {
        0x0F
    }
}

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
impl Reg for SoftReset {
    fn address() -> u8 {
        0x10
    }
}
