#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct MeasurementStatus {
    /// New data available in CO2PPM Register
    pub data_ready: bool,
    /// Pin INT has been latched to active state 
    pub int_active: bool,
    /// Alarm notification (threshold violation occured)
    pub alarm: bool
}


impl MeasurementStatus {
    /// Bitmask to clear the int pin active bit
    pub(crate) fn clear_int_active() -> u8 {
        0b0000_0100
    }

    /// Bitmask to clear alarm bit
    pub(crate) fn clear_alarm() -> u8 {
        0b0000_0010
    }
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

impl super::Reg for MeasurementStatus {
    fn address() -> u8 {
        0x07
    }
}

#[cfg(test)]
#[test]
fn test_bitmask() {
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
