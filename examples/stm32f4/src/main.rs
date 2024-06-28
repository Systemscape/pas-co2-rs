#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embassy_time::Timer;

use pas_co2_rs::regs::*;
use pas_co2_rs::*;

use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    I2C1_EV => i2c::EventInterruptHandler<peripherals::I2C1>;
    I2C1_ER => i2c::ErrorInterruptHandler<peripherals::I2C1>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_stm32::init(Default::default());

    let mut config = embassy_stm32::i2c::Config::default();
    config.sda_pullup = true;
    config.scl_pullup = true;

    let i2c = embassy_stm32::i2c::I2c::new_blocking(p.I2C1, p.PB8, p.PB9, Hertz(100_000), config);

    // Obtain an instance of the driver
    let mut pas_co2 = PasCo2::new(i2c);

    info!("Status: {}", pas_co2.get_status());

    // Set to idle mode (default)
    let mut mode = MeasurementMode::default();
    mode.operating_mode = measurement_mode::OperatingMode::Idle;
    pas_co2.set_measurement_mode(mode).unwrap();

    let pressure: u16 = 950; //hPa

    pas_co2
        .set_pressure_compensation(PressureCompensation(pressure))
        .unwrap();

    let status = pas_co2.get_status().unwrap();
    info!("Status: {}", status);

    pas_co2.clear_status().unwrap();

    loop {
        pas_co2.start_measurement().unwrap();

        Timer::after_millis(1150).await;

        let co2_ppm = pas_co2.get_co2_ppm().unwrap();

        info!("CO2: {} ppm", co2_ppm);

        Timer::after_millis(10000).await;
    }
}