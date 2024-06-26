pub trait Reg {
    fn address() -> u8;
}

mod status;
pub use status::Status;

mod measurement_mode;
pub use measurement_mode::*;

pub struct MeasurementPeriod {}
impl Reg for MeasurementPeriod {
    fn address() -> u8 {
        0x02
    }
}

pub struct Co2Ppm {}
impl Reg for Co2Ppm {
    fn address() -> u8 {
        0x05
    }
}

pub struct PressureCompensation {}
impl Reg for PressureCompensation {
    fn address() -> u8 {
        0x0B
    }
}

