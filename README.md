# pas-co2-rs
Inofficial Rust driver for Infineon XENSIV (TM) PAS CO2 sensor.

```rust
use pas_co2_rs::{
    regs::{measurement_mode::OperatingMode, MeasurementMode, PressureCompensation},
    PasCo2,
};

let i2c = embassy_stm32::i2c::I2c::new(
    p.I2C1,
    p.PB8,
    p.PB9,
    Irqs,
    p.DMA1_CH6,
    p.DMA1_CH0,
    Hertz(100_000),
    config,
);

// Obtain an instance of the driver
let pas_co2 = PasCo2::new(i2c);

info!("Status: {}", pas_co2.get_status());

/// Set to idle mode (default)
let mut mode = MeasurementMode::default();
mode.operating_mode = OperatingMode::Idle;
pas_co2.set_measurement_mode(mode).unwrap();

let pressure: u16 = 950; //hPa

pas_co2.set_pressure_compensation(PressureCompensation(pressure)).unwrap();

let status = pas_co2.get_status().unwrap();
info!("Status: {}", status);

pas_co2.clear_status().unwrap();

pas_co2.start_measurement().unwrap();

embassy_time::Delay {}.delay_ms(1150);

let co2_ppm = pas_co2.get_co2_ppm().unwrap();
```