pub trait Reg {
    fn address() -> u8;
}

mod status;
pub use status::Status; 

mod measurement_mode;
pub use measurement_mode::MeasurementMode;

pub struct MeasurementPeriod{}
impl Reg for MeasurementPeriod {
    fn address() -> u8 {
        0x02
    }
}