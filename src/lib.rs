#![no_std]
use core::panic;

use defmt::info;
use embedded_hal::{
    delay::DelayNs,
    i2c::{I2c, SevenBitAddress},
};

/// Sensor registers (addresses, struct representations etc.)
pub mod regs;
use crate::regs::*;

/// I2C Address of the Sensor
pub const ADDRESS: u8 = 0x28;

#[derive(Debug, Copy, Clone)]
pub enum Error<T> {
    /// Error on the I2C interface
    Interface(T),

    /// Error in response of the sensor
    Response(ResponseError),
}

impl<T> From<T> for Error<T> {
    fn from(e: T) -> Self {
        Self::Interface(e)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ResponseError {
    InvalidRegisterValue,
}

/// Driver for the Infineon XENSIV PAS CO2 sensor
pub struct PasCo2<I2C: I2c<SevenBitAddress>> {
    i2c: I2C,
}

impl<T> PasCo2<T>
where
    T: I2c<SevenBitAddress>,
{
    /// Create a new instance of this driver
    pub fn new(i2c: T) -> Self {
        Self { i2c }
    }

    /// Obtain the sensor's [Status]
    pub fn get_status(&mut self) -> Result<Status, Error<T::Error>> {
        self.read_reg_u8(Register::SensorStatus).map(|x| x.into())
    }

    /// Clear temperature, voltage and communication errors from the sensor status
    pub fn clear_status(&mut self) -> Result<(), Error<T::Error>> {
        self.clear_temperature_error()?;
        self.clear_voltage_error()?;
        self.clear_communication_error()
    }

    /// Write bitmask to clear the temperature error bit
    pub fn clear_temperature_error(&mut self) -> Result<(), Error<T::Error>> {
        self.write_reg(Register::SensorStatus, &[0b0000_0100])
    }

    /// Write bitmask to clear the voltage error bit
    pub fn clear_voltage_error(&mut self) -> Result<(), Error<T::Error>> {
        self.write_reg(Register::SensorStatus, &[0b0000_0010])
    }

    /// Write bitmask to clear the communication error bit
    pub fn clear_communication_error(&mut self) -> Result<(), Error<T::Error>> {
        self.write_reg(Register::SensorStatus, &[0b0000_0001])
    }

    /// Time between two measurements in continuous mode
    ///
    /// Must be between 5 and 4095 seconds
    /// Values below 5s are treated as 5s by the sensor and generate a communication error.
    pub fn set_measurement_period(&mut self, period: i16) -> Result<(), Error<T::Error>> {
        debug_assert!(period <= 4096);
        debug_assert!(period >= 5);

        // Registermap says bits 7:0 shall be written with 0.
        let period = (period & 0x0FFF).to_be_bytes();

        self.write_reg(Register::MeasurementRate, &period)
    }

    /// Configure the [MeasurementMode]
    pub fn set_measurement_mode(&mut self, mode: MeasurementMode) -> Result<(), Error<T::Error>> {
        self.write_reg(Register::MeasurementMode, &[mode.into()])
    }

    /// Read the sensor's [MeasurementMode]
    pub fn get_measurement_mode(&mut self) -> Result<MeasurementMode, Error<T::Error>> {
        self.read_reg_u8(Register::MeasurementMode)
            .map(|x| x.into())
    }

    /// Start a single measurement.
    ///
    /// This function reads the current [MeasurementMode] and sets it
    /// operating mode to [measurement_mode::OperatingMode::SingleShot].
    pub fn start_measurement(&mut self) -> Result<(), Error<T::Error>> {
        let mut mode = self.get_measurement_mode()?;
        mode.operating_mode = regs::OperatingMode::SingleShot;
        self.set_measurement_mode(mode)
    }

    /// Get the current CO2 reading in PPM
    ///
    /// **Caution**: The user is responsible for starting a measurement and checking whether
    /// measured data is available. See [Self::get_measurement_status()].
    pub fn get_co2_ppm(&mut self) -> Result<i16, Error<T::Error>> {
        self.read_reg_i16(Register::Co2Ppm)
    }

    /// Get the current sensor [MeasurementStatus]
    pub fn get_measurement_status(&mut self) -> Result<MeasurementStatus, Error<T::Error>> {
        self.read_reg_u8(Register::MeasurementStatus)
            .map(|x| x.into())
    }

    /// Clear the int active bit and the alarm bit of the sensor's [MeasurementStatus] register
    pub fn clear_measurement_status(&mut self) -> Result<(), Error<T::Error>> {
        self.clear_int_active()?;
        self.clear_alarm()
    }

    /// Clear the int active bit of the sensor's [MeasurementStatus] register
    pub fn clear_int_active(&mut self) -> Result<(), Error<T::Error>> {
        // Write bitmask to clear the int pin active bit
        self.write_reg(Register::MeasurementStatus, &[0b0000_0100])
    }

    /// Clear the the alarm bit of the sensor's [MeasurementStatus] register
    pub fn clear_alarm(&mut self) -> Result<(), Error<T::Error>> {
        // Write bitmask to clear the alarm bit
        self.write_reg(Register::MeasurementStatus, &[0b0000_0010])
    }

    /// Configure when the interrupt pin is activated
    pub fn set_interrupt_config(&mut self, config: InterruptConfig) -> Result<(), Error<T::Error>> {
        let config: u8 = config.into();

        #[cfg(feature = "defmt")]
        defmt::info!("Setting interrupt config: {:b}", config);

        self.write_reg(Register::InterruptConfig, &[config])
    }

    /// Get the current sensor [InterruptConfig]
    pub fn get_interrupt_config(&mut self) -> Result<InterruptConfig, Error<T::Error>> {
        let val = self.read_reg_u8(Register::InterruptConfig)?;
        val.try_into()
            .map_err(|_| Error::Response(ResponseError::InvalidRegisterValue))
    }

    /// Set the threshold for the alarm status bit or interrupt (if enabled)
    pub fn set_alarm_threshold(&mut self, threshold_ppm: i16) -> Result<(), Error<T::Error>> {
        // Although it is a signed integer (why?), it makes no sense to have a negative value here.
        debug_assert!(threshold_ppm > 0);

        let threshold_ppm = threshold_ppm.to_be_bytes();

        self.write_reg(Register::AlarmThreshold, &threshold_ppm)
    }

    /// Set the [PressureCompensation] in hPa.
    ///
    /// Valid range: 750 hPa to 1150 hPa.
    ///
    /// Setting to invalid values clips the pressure and causes a communication error
    pub fn set_pressure_compensation(&mut self, pressure: u16) -> Result<(), Error<T::Error>> {
        debug_assert!(pressure <= 1150);
        debug_assert!(pressure >= 750);

        let pressure = pressure.to_be_bytes();

        self.write_reg(Register::PressureReference, &pressure)
    }

    /// Get the [PressureCompensation] in hPa.
    pub fn get_pressure_compensation(&mut self) -> Result<u16, Error<T::Error>> {
        self.read_reg_u16(Register::PressureReference)
    }

    /// Set the Automatic Baseline Offset Compensation Reference in PPM.
    ///
    /// Valid range: 350 ppm to 900 ppm.
    ///
    /// Setting to invalid values clips the value and causes a communication error
    pub fn set_aboc(&mut self, aboc: i16) -> Result<(), Error<T::Error>> {
        debug_assert!(aboc <= 900);
        debug_assert!(aboc >= 350);

        let aboc = aboc.to_be_bytes();

        self.write_reg(Register::PressureReference, &aboc)
    }

    pub fn do_forced_compensation(
        &mut self,
        calibration_value: i16,
        mut delay: impl DelayNs,
    ) -> Result<(), Error<T::Error>> {
        // 1. set idle mode
        let mut mode = self.get_measurement_mode()?;
        mode.operating_mode = OperatingMode::Idle;
        self.set_measurement_mode(mode)?;

        // 2. Configure measurement rate to 10s
        self.set_measurement_period(10)?;

        // 3. Set calibration register according to the reference value
        self.set_aboc(calibration_value)?;

        // 4. Enable forced calibration at continuous mode
        mode.baseline_offset_comp = BaselineOffsetCompensation::Forced;
        mode.operating_mode = OperatingMode::Continuous;
        self.set_measurement_mode(mode)?;

        // 5. run loop for 3 times
        for _ in 0..3 {
            if self.get_measurement_status()?.data_ready {
                let co2_ppm = self.get_co2_ppm()?;
                #[cfg(feature = "defmt")]
                info!("Read CO2 PPM: {}", co2_ppm);
            }
            delay.delay_ms(100);
        }

        // 6. Set to Idle Mode
        mode.operating_mode = OperatingMode::Idle;
        self.set_measurement_mode(mode)?;

        // 7. Save the calibration
        //self.soft_reset(SoftReset::SaveForceCalibNvm);

        Ok(())
    }

    /// Perform a write-then-read to the scratch pad register and return the read back value.
    pub fn test_write_read(&mut self, val: u8) -> Result<u8, Error<T::Error>> {
        self.write_reg(Register::ScratchPad, &[val])?;

        self.read_reg_u8(Register::ScratchPad)
    }

    /// Send a [SoftReset] event to the sensor
    pub fn soft_reset(&mut self, reset: SoftReset) -> Result<(), Error<T::Error>> {
        self.write_reg(Register::SensorReset, &[reset.into()])
    }

    /// Length of val must be 1 or 2. The sensor only has 1 or 2 byte registers
    fn write_reg(&mut self, reg: Register, val: &[u8]) -> Result<(), Error<T::Error>> {
        assert!(val.len() <= 2);
        assert!(!val.is_empty());
        match val.len() {
            1 => self.i2c.write(ADDRESS, &[reg.into(), val[0]])?,
            2 => self.i2c.write(ADDRESS, &[reg.into(), val[0], val[1]])?,
            _ => panic!("Invalid length for write_reg"),
        }

        Ok(())
    }

    fn read_reg_u8(&mut self, register: Register) -> Result<u8, Error<T::Error>> {
        let mut result = [0u8; 1];
        self.i2c
            .write_read(ADDRESS, &[register.into()], &mut result[..])?;
        Ok(result[0])
    }

    fn read_reg_u16(&mut self, register: Register) -> Result<u16, Error<T::Error>> {
        let mut bytes = [0u8; 2];
        self.i2c
            .write_read(ADDRESS, &[register.into()], &mut bytes[..])?;
        Ok(u16::from_be_bytes(bytes))
    }

    fn read_reg_i16(&mut self, register: Register) -> Result<i16, Error<T::Error>> {
        let mut bytes = [0u8; 2];
        self.i2c
            .write_read(ADDRESS, &[register.into()], &mut bytes[..])?;
        Ok(i16::from_be_bytes(bytes))
    }
}
