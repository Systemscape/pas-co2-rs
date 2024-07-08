#![no_std]
#![no_main]

use defmt::*;
use embassy_executor::Spawner;
use embassy_stm32::time::Hertz;
use embassy_stm32::{bind_interrupts, i2c, peripherals};
use embassy_time::{Duration, Timer};

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
    config.timeout = Duration::from_secs(1);

    info!("Configuring I2C");

    let i2c = embassy_stm32::i2c::I2c::new(
        p.I2C1,
        p.PB8,
        p.PB9,
        Irqs,
        p.DMA1_CH6,
        p.DMA1_CH0,
        Hertz(400_000),
        config,
    );

    info!("Obtaining driver instance");

    // Obtain an instance of the driver
    let mut pas_co2 = PasCo2::new(i2c);

    info!("Status: {}", pas_co2.get_status().await.unwrap());

    // Set to idle mode (default)
    let mode = MeasurementMode {
        operating_mode: OperatingMode::Idle,
        ..Default::default()
    };

    pas_co2.set_measurement_mode(mode).await.unwrap();

    let pressure: u16 = 950; //hPa
    pas_co2.set_pressure_compensation(pressure).await.unwrap();

    defmt::info!("Testing write -> read");
    let test_val = 0b1010_0101;
    let read_val = pas_co2.test_write_read(test_val).await.unwrap();
    defmt::assert_eq!(test_val, read_val);

    pas_co2.clear_status().await.unwrap();

    // Perform a forced calibration/compensation at 490 ppm reference

    /*
        pas_co2
            .do_forced_compensation(490, embassy_time::Delay)
            .await
            .unwrap();
    */

    loop {
        defmt::info!("Starting measurement");
        pas_co2.start_measurement().await.unwrap();

        Timer::after_millis(1150).await;

        let co2_ppm = pas_co2.get_co2_ppm().await.unwrap();

        info!("CO2: {} ppm", co2_ppm);

        Timer::after_secs(10).await;
    }
}
