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

impl Status {
    /// Bitmask to clear the temperature error bit
    pub(crate) fn clear_temperature_error() -> u8 {
        0b0000_0100
    }

    /// Bitmask to clear the voltage error bit
    pub(crate) fn clear_voltage_error() -> u8 {
        0b0000_0010
    }

    /// Bitmask to clear the communication error bit
    pub(crate) fn clear_communication_error() -> u8 {
        0b0000_0001
    }
}

impl From<u8> for Status {
    fn from(value: u8) -> Self {
        Self {
            ready: (value & 0b1000_0000) != 0,
            pwm_dis: (value & 0b0100_0000) != 0,
            temperature_error:(value & 0b0010_0000) != 0,
            voltage_error: (value & 0b0001_0000) != 0,
            communication_error: (value & 0b0000_1000) != 0,
        }
    }
}

impl super::Reg for Status {
    fn address() -> u8 {
        0x01
    }
}


#[cfg(test)]
#[test]
fn test_bitmask() {
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
