#![no_std]
use embedded_hal::i2c::{I2c, SevenBitAddress};

/// Sensor registers (addresses, struct representations etc.)
pub mod regs;
use crate::regs::*;

/// I2C Address of the Sensor
pub const ADDRESS: u8 = 0x28;

/// Driver for the Infineon XENSIV PAS CO2 sensor
pub struct PasCo2<I2C: I2c<SevenBitAddress>> {
    i2c: I2C,
}

impl<I2C: I2c<SevenBitAddress>> PasCo2<I2C> {
    /// Create a new instance of this driver
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    /// Obtain the sensor's [Status]
    pub fn get_status(&mut self) -> Result<Status, I2C::Error> {
        let mut temp: [u8; 1] = [0];
        self.i2c
            .write_read(ADDRESS, &[Status::address()], &mut temp)?;
        Ok(temp[0].into())
    }

    /// Clear temperature, voltage and communication errors from the sensor status
    pub fn clear_status(&mut self) -> Result<(), I2C::Error> {
        let bitmask = Status::clear_temperature_error()
            & Status::clear_voltage_error()
            & Status::clear_communication_error();
        self.i2c.write(ADDRESS, &[Status::address(), bitmask])?;
        Ok(())
    }

    /// Time between two measurements in continuous mode
    ///
    /// Must be between 5 and 4095 seconds
    /// Values below 5s are treated as 5s by the sensor and generate a communication error.
    pub fn set_measurement_period(&mut self, period: i16) -> Result<(), I2C::Error> {
        debug_assert!(period <= 4096);
        debug_assert!(period >= 5);

        // Registermap says bits 7:0 shall be written with 0.
        let period = (period & 0x0FFF).to_be_bytes();
        self.i2c.write(
            ADDRESS,
            &[MeasurementPeriod::address(), period[0], period[1]],
        )?;
        Ok(())
    }

    /// Configure the [MeasurementMode]
    pub fn set_measurement_mode(&mut self, mode: MeasurementMode) -> Result<(), I2C::Error> {
        let mode: u8 = mode.into();
        #[cfg(feature = "defmt")]
        defmt::info!("Setting measurement mode: {:b}", mode);

        self.i2c
            .write(ADDRESS, &[MeasurementMode::address(), mode.into()])?;
        Ok(())
    }

    /// Read the sensor's [MeasurementMode]
    pub fn get_measurement_mode(&mut self) -> Result<MeasurementMode, I2C::Error> {
        let mut temp = [0];
        self.i2c
            .write_read(ADDRESS, &[MeasurementMode::address()], &mut temp)?;
        Ok(temp[0].into())
    }

    /// Start a single measurement.
    ///
    /// This function reads the current [MeasurementMode] and sets it
    /// operating mode to [measurement_mode::OperatingMode::SingleShot].
    pub fn start_measurement(&mut self) -> Result<(), I2C::Error> {
        let mut mode = self.get_measurement_mode()?;
        mode.operating_mode = measurement_mode::OperatingMode::SingleShot;
        self.i2c
            .write(ADDRESS, &[MeasurementMode::address(), mode.into()])?;
        Ok(())
    }

    /// Get the current CO2 reading in PPM
    ///
    /// **Caution**: The user is responsible for starting a measurement and checking whether
    /// measured data is available. See [Self::get_measurement_status()].
    pub fn get_co2_ppm(&mut self) -> Result<i16, I2C::Error> {
        let mut temp = [0, 0];
        // Actually two bytes to be read, but sensor supports bulk read and write
        // i.e., automatically increments the register address
        self.i2c
            .write_read(ADDRESS, &[Co2Ppm::address()], &mut temp)?;

        let co2_ppm = i16::from_be_bytes([temp[0], temp[1]]);

        Ok(co2_ppm)
    }

    /// Get the current sensor [MeasurementStatus]
    pub fn get_measurement_status(&mut self) -> Result<MeasurementStatus, I2C::Error> {
        let mut temp = [0];
        self.i2c
            .write_read(ADDRESS, &[MeasurementStatus::address()], &mut temp)?;
        Ok(temp[0].into())
    }

    /// Clear the int active bit and the alarm bit of the sensor's [MeasurementStatus] register
    pub fn clear_measurement_status(&mut self) -> Result<(), I2C::Error> {
        let bitmask = &MeasurementStatus::clear_int_active() & MeasurementStatus::clear_alarm();
        self.i2c
            .write(ADDRESS, &[MeasurementStatus::address(), bitmask])?;
        Ok(())
    }

    /// Configure when the interrupt pin is activated
    pub fn set_interrupt_config(&mut self, config: InterruptConfig) -> Result<(), I2C::Error> {
        let config: u8 = config.into();

        #[cfg(feature = "defmt")]
        defmt::info!("Setting interrupt config: {:b}", config);

        self.i2c
            .write(ADDRESS, &[InterruptConfig::address(), config.into()])?;
        Ok(())
    }

    /// Get the current sensor [InterruptConfig]
    pub fn get_interrupt_config(&mut self) -> Result<InterruptConfig, I2C::Error> {
        let mut temp = [0];
        self.i2c
            .write_read(ADDRESS, &[InterruptConfig::address()], &mut temp)?;
        Ok(temp[0].into())
    }

    /// Set the threshold for the alarm status bit or interrupt (if enabled)
    pub fn set_alarm_threshold(&mut self, threshold_ppm: AlarmThreshold) -> Result<(), I2C::Error> {
        let threshold_ppm = threshold_ppm.0;
        // Although it is a signed integer (why?), it makes no sense to have a negative value here.
        debug_assert!(threshold_ppm > 0);

        let threshold_ppm = threshold_ppm.to_be_bytes();

        self.i2c.write(
            ADDRESS,
            &[
                AlarmThreshold::address(),
                threshold_ppm[0],
                threshold_ppm[1],
            ],
        )?;
        Ok(())
    }

    /// Set the [PressureCompensation] in hPa.
    ///
    /// Valid range: 750 hPa to 1150 hPa.
    ///
    /// Setting to invalid values clips the pressure and causes a communication error
    pub fn set_pressure_compensation(
        &mut self,
        pressure: PressureCompensation,
    ) -> Result<(), I2C::Error> {
        let pressure = pressure.0;

        debug_assert!(pressure <= 1150);
        debug_assert!(pressure >= 750);

        let pressure = pressure.to_be_bytes();

        self.i2c.write(
            ADDRESS,
            &[PressureCompensation::address(), pressure[0], pressure[1]],
        )?;
        Ok(())
    }

    /// Set the Automatic Baseline Offset Compensation Reference in PPM.
    ///
    /// Valid range: 350 ppm to 900 ppm.
    ///
    /// Setting to invalid values clips the value and causes a communication error
    pub fn set_aboc(&mut self, aboc: ABOC) -> Result<(), I2C::Error> {
        let aboc = aboc.0;

        debug_assert!(aboc <= 900);
        debug_assert!(aboc >= 350);

        let aboc = aboc.to_be_bytes();

        self.i2c.write(
            ADDRESS,
            &[PressureCompensation::address(), aboc[0], aboc[1]],
        )?;
        Ok(())
    }

    /// Perform a write-then-read to the scratch pad register and return the read back value.
    pub fn test_write_read(&mut self, val: u8) -> Result<u8, I2C::Error> {
        let mut tmp = [0u8; 1];

        self.i2c.write(ADDRESS, &[Scratchpad::address(), val])?;

        self.i2c
            .write_read(ADDRESS, &[Scratchpad::address()], &mut tmp)?;

        Ok(tmp[0])
    }

    /// Send a [SoftReset] event to the sensor
    pub fn soft_reset(&mut self, reset: SoftReset) -> Result<(), I2C::Error> {
        self.i2c
            .write(ADDRESS, &[SoftReset::address(), reset as u8])?;

        Ok(())
    }
}
