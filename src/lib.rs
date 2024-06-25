use embedded_hal::i2c::I2c;

pub struct PasCo2<I2C: I2c> {
    i2c: I2C,
}

impl<I2C: I2c> PasCo2<I2C> {
    pub fn new(i2c: I2C) -> Self {
        Self { i2c }
    }

    pub fn get_status() -> Status {

    }
}
