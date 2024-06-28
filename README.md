# pas-co2-rs
Inofficial Rust driver for Infineon XENSIV (TM) PAS CO2 sensor.

```rust
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
```