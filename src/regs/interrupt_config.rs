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
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum IntFunctionConfig {
    /// Pin INT is inactive
    Inactive = 0,
    /// Alarm threshold violation notification
    Alarm = 1,
    /// Data ready notification
    DataReady = 2,
    /// Busy notification
    Busy = 3,
    /// Early measurement start notification (only continuous mode)
    EarlyMeasurementStart = 4,
    /// Reserved / Invalid
    _Reserved = 5,
}

impl Default for InterruptConfig {
    fn default() -> Self {
        Self::from(0x11)
    }
}

impl From<InterruptConfig> for u8 {
    fn from(value: InterruptConfig) -> Self {
        (value.int_pin_active_high as u8) << 4
            | (value.int_function_config as u8) << 1
            | value.alarm_crossing_up as u8
    }
}

impl From<u8> for InterruptConfig {
    fn from(value: u8) -> Self {
        Self {
            int_pin_active_high: (value & 0b0001_0000) != 0,
            int_function_config: match (value & 0b0000_1110) >> 1 {
                0 => IntFunctionConfig::Inactive,
                1 => IntFunctionConfig::Alarm,
                2 => IntFunctionConfig::DataReady,
                3 => IntFunctionConfig::Busy,
                4 => IntFunctionConfig::EarlyMeasurementStart,
                _ => IntFunctionConfig::_Reserved,
            },
            alarm_crossing_up: (value & 0b0000_0001) != 0,
        }
    }
}

impl super::Reg for InterruptConfig {
    fn address() -> u8 {
        0x08
    }
}

#[cfg(test)]
#[test]
fn test_bitmask() {
    let config = InterruptConfig {
        int_pin_active_high: true,
        int_function_config: IntFunctionConfig::DataReady,
        alarm_crossing_up: true,
    };

    let bitmask: u8 = config.into();

    assert_eq!(bitmask, 0b0001_0101);

    let config_from = InterruptConfig::from(bitmask);
    assert_eq!(config, config_from);
}
