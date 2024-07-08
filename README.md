# pas-co2-rs
Inofficial Rust driver for Infineon XENSIV (TM) PAS CO2 sensor.

## Non-async I2C
Currently, there is no support for synchronous I2C, i.e., embedded-hal. Only embedded-hal-async is supported.
It is straightforward to add this, but would result in a lot of code duplication.

## Examples
You can find an example for the STM32F469 in the examples folder inside the repository.
This should be easy to adapt to any other platform thanks to embedded-hal.

## Contributing
Feel free to contribute to this project by opening a pull-request or creating an issue for bugs, questions or suggestions.
