#![no_std]
use embedded_hal::i2c::{I2c, SevenBitAddress};

pub mod regs;
use crate::regs::*;

pub const ADDRESS: u8 = 0x28;
pub struct PasCo2<I2C: I2c<SevenBitAddress>> {
    i2c: I2C,
}

impl<I2C: I2c<SevenBitAddress>> PasCo2<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn get_status(&mut self) -> Result<Status, I2C::Error> {
        let mut temp: [u8; 1] = [0];
        self.i2c
            .write_read(ADDRESS, &[Status::address()], &mut temp)?;
        Ok(temp[0].into())
    }

    pub fn clear_status(&mut self) -> Result<(), I2C::Error> {
        let bitmask = Status::clear_temperature_error()
            & Status::clear_voltage_error()
            & Status::clear_communication_error();
        self.i2c.write(ADDRESS, &[Status::address(), bitmask])?;
        Ok(())
    }

    /// Must be between 5 and 4095 seconds
    /// Values below 5s are treated as 5s by the sensor and generate a communication error.
    pub fn set_measurement_period(&mut self, period: u16) -> Result<(), I2C::Error> {
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

    pub fn set_measurement_mode(&mut self, mode: MeasurementMode) -> Result<(), I2C::Error> {
        let mode: u8 = mode.into();
        #[cfg(feature = "defmt")]
        defmt::info!("Setting measurement mode: {:b}", mode);

        self.i2c
            .write(ADDRESS, &[MeasurementMode::address(), mode.into()])?;
        Ok(())
    }

    pub fn get_measurement_mode(&mut self) -> Result<MeasurementMode, I2C::Error> {
        let mut temp = [0];
        self.i2c
            .write_read(ADDRESS, &[MeasurementMode::address()], &mut temp)?;
        Ok(temp[0].into())
    }
    /// Start a single measurement
    ///
    /// *Caution:*
    pub fn start_measurement(&mut self) -> Result<(), I2C::Error> {
        let mut mode = self.get_measurement_mode()?;
        mode.operating_mode = OperatingMode::SingleShot;
        self.i2c
            .write(ADDRESS, &[MeasurementMode::address(), mode.into()])?;
        Ok(())
    }

    pub fn get_co2_ppm(&mut self) -> Result<u16, I2C::Error> {
        let mut temp = [0, 0];
        // Actually two bytes to be read, but sensor supports bulk read and write
        // i.e., automatically increments the register address
        self.i2c
            .write_read(ADDRESS, &[Co2Ppm::address()], &mut temp)?;

        let co2_ppm = u16::from_be_bytes([temp[0], temp[1]]);

        Ok(co2_ppm)
    }

    /// Valid range: 750 hPa to 1150 hPa.
    ///
    /// Setting to invalid values clips the pressure and causes a communication error
    pub fn set_pressure_compensation(&mut self, pressure: u16) -> Result<(), I2C::Error> {
        debug_assert!(pressure <= 1150);
        debug_assert!(pressure >= 750);

        let pressure = pressure.to_be_bytes();

        self.i2c.write(
            ADDRESS,
            &[MeasurementMode::address(), pressure[0], pressure[1]],
        )?;
        Ok(())
    }
}
